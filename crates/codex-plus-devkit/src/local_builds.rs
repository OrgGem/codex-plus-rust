//! Safe pruning of old `dist/local-build*` directories (`scripts/prune-local-builds.mjs`).

use anyhow::Result;
use chrono::Utc;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug)]
pub struct PruneOptions {
    pub apply: bool,
    pub keep: usize,
    pub min_age_minutes: u64,
}

impl Default for PruneOptions {
    fn default() -> Self {
        Self {
            apply: false,
            keep: 0,
            min_age_minutes: 30,
        }
    }
}

#[derive(Clone, Debug)]
struct BuildEntry {
    dir: PathBuf,
    name: String,
    canonical: bool,
    in_use: bool,
    fresh: bool,
    keep_marker: bool,
    mtime_ms: u128,
}

fn list_build_dirs(dist_dir: &Path) -> Vec<PathBuf> {
    if !dist_dir.is_dir() {
        return Vec::new();
    }
    let Ok(rd) = fs::read_dir(dist_dir) else {
        return Vec::new();
    };
    let mut dirs = Vec::new();
    for entry in rd.flatten() {
        let path = entry.path();
        let name = entry.file_name();
        let Some(name_str) = name.to_str() else {
            continue;
        };
        let is_match = name_str == "local-build" || name_str.starts_with("local-build-");
        if is_match && entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
            dirs.push(path);
        }
    }
    dirs
}

fn mtime_ms(path: &Path) -> Option<u128> {
    let m = fs::metadata(path).ok()?.modified().ok()?;
    Some(m.duration_since(UNIX_EPOCH).ok()?.as_millis())
}

fn newest_mtime_ms(dir: &Path) -> u128 {
    let mut newest = mtime_ms(dir).unwrap_or(0);
    let Ok(children) = fs::read_dir(dir) else {
        return newest;
    };
    for child in children.flatten() {
        let p = child.path();
        if let Some(ms) = mtime_ms(&p) {
            newest = newest.max(ms);
        }
        if child.file_type().map(|t| t.is_dir()).unwrap_or(false)
            && let Ok(grand) = fs::read_dir(&p)
        {
            for g in grand.flatten() {
                if let Some(ms) = mtime_ms(&g.path()) {
                    newest = newest.max(ms);
                }
            }
        }
    }
    newest
}

pub fn filter_paths_in_use(dirs: &[PathBuf], commands: &str) -> HashSet<PathBuf> {
    if commands.is_empty() {
        return HashSet::new();
    }
    let norm_commands = commands.replace('\\', "/").to_lowercase();
    dirs.iter()
        .filter(|dir| {
            let raw = dir.to_string_lossy();
            let stripped = raw.trim_start_matches(r"\\?\");
            let slashed = stripped.replace('\\', "/");
            let key = slashed.trim_end_matches('/').to_lowercase();
            if key.is_empty() {
                return false;
            }
            norm_commands.contains(&format!("{key}/"))
                || norm_commands.contains(&format!("{key} "))
                || norm_commands.contains(&format!("{key}\""))
                || norm_commands.lines().any(|l| l.trim() == key)
        })
        .cloned()
        .collect()
}

fn paths_in_use(dirs: &[PathBuf]) -> HashSet<PathBuf> {
    let commands = running_process_commands();
    filter_paths_in_use(dirs, &commands)
}

fn running_process_commands() -> String {
    if cfg!(windows) {
        Command::new("powershell")
            .args([
                "-NoProfile",
                "-Command",
                "Get-CimInstance Win32_Process | ForEach-Object { ($_.ExecutablePath + ' ' + $_.CommandLine) }",
            ])
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).into_owned())
            .unwrap_or_default()
    } else {
        Command::new("ps")
            .args(["-axo", "command="])
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).into_owned())
            .unwrap_or_default()
    }
}

fn dir_size_bytes(dir: &Path) -> u64 {
    let mut total = 0u64;
    let Ok(rd) = fs::read_dir(dir) else {
        return 0;
    };
    for entry in rd.flatten() {
        let Ok(meta) = fs::symlink_metadata(entry.path()) else {
            continue;
        };
        if meta.file_type().is_symlink() {
            continue;
        }
        if meta.is_dir() {
            total = total.saturating_add(dir_size_bytes(&entry.path()));
        } else {
            total = total.saturating_add(meta.len());
        }
    }
    total
}

fn format_size(kb: u64) -> String {
    if kb >= 1024 * 1024 {
        format!("{:.2} GB", kb as f64 / 1024.0 / 1024.0)
    } else if kb >= 1024 {
        format!("{} MB", (kb as f64 / 1024.0).round() as u64)
    } else {
        format!("{kb} KB")
    }
}

fn unregister_apps_macos(dir: &Path) {
    if !cfg!(target_os = "macos") {
        return;
    }
    let lsregister = "/System/Library/Frameworks/CoreServices.framework/Frameworks/LaunchServices.framework/Support/lsregister";
    if let Ok(rd) = fs::read_dir(dir) {
        for e in rd.flatten() {
            if e.file_name().to_str().is_some_and(|n| n.ends_with(".app")) {
                let _ = Command::new(lsregister).args(["-u"]).arg(e.path()).output();
            }
        }
    }
}

fn default_trash_dir(dist_dir: &Path) -> Result<PathBuf> {
    if cfg!(target_os = "macos")
        && let Ok(home) = std::env::var("HOME")
    {
        return Ok(PathBuf::from(home).join(".Trash"));
    }
    let trash = dist_dir.join(".trash");
    fs::create_dir_all(&trash)?;
    Ok(trash)
}

fn trash_destination(trash_dir: &Path, dir: &Path) -> PathBuf {
    let stamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
    let name = dir
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| "build".into());
    let base = trash_dir.join(format!("{name}-{stamp}"));
    if !base.exists() {
        return base;
    }
    let mut suffix = 1u32;
    loop {
        let cand = trash_dir.join(format!("{name}-{stamp}-{suffix}"));
        if !cand.exists() {
            return cand;
        }
        suffix += 1;
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PruneReport {
    pub protected: Vec<String>,
    pub candidates: Vec<String>,
    pub moved: Vec<PathBuf>,
}

pub fn prune_local_builds(root: &Path, opts: &PruneOptions) -> Result<PruneReport> {
    let abs_root = std::path::absolute(root)?;
    let dist_dir = abs_root.join("dist");
    let canonical_dir = dist_dir.join("local-build");
    let dirs = list_build_dirs(&dist_dir);
    if dirs.is_empty() {
        println!("dist/ 下没有 local-build* 目录，无需清理。");
        return Ok(PruneReport::default());
    }

    let in_use = paths_in_use(&dirs);
    let now_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    let age_limit = u128::from(opts.min_age_minutes) * 60 * 1000;

    let mut entries: Vec<BuildEntry> = dirs
        .into_iter()
        .map(|dir| {
            let mt = newest_mtime_ms(&dir);
            BuildEntry {
                name: dir
                    .file_name()
                    .map(|n| n.to_string_lossy().into_owned())
                    .unwrap_or_default(),
                canonical: dir == canonical_dir,
                in_use: in_use.contains(&dir),
                fresh: now_ms.saturating_sub(mt) < age_limit,
                keep_marker: dir.join(".keep").exists(),
                mtime_ms: mt,
                dir,
            }
        })
        .collect();
    entries.sort_by(|a, b| a.name.cmp(&b.name));

    let (protected_always, mut candidates): (Vec<_>, Vec<_>) = entries
        .into_iter()
        .partition(|e| e.canonical || e.in_use || e.fresh || e.keep_marker);
    candidates.sort_by_key(|a| std::cmp::Reverse(a.mtime_ms));

    let keep_n = opts.keep.min(candidates.len());
    let kept_by_count: Vec<BuildEntry> = candidates.drain(..keep_n).collect();
    let removable = candidates;

    println!("本地构建目录: {}", dist_dir.display());
    let mut protected_names = Vec::new();
    for entry in protected_always.iter().chain(kept_by_count.iter()) {
        let reason = if entry.canonical {
            "固定开发实例".to_string()
        } else if entry.in_use {
            "有进程正在使用".to_string()
        } else if entry.fresh {
            format!("最近 {} 分钟内改动过", opts.min_age_minutes)
        } else if entry.keep_marker {
            "有 .keep 标记".to_string()
        } else {
            format!("--keep {} 保留", opts.keep)
        };
        println!("  保留  {}（{}）", entry.name, reason);
        protected_names.push(entry.name.clone());
    }

    if removable.is_empty() {
        println!("\n没有需要清理的历史构建。");
        return Ok(PruneReport {
            protected: protected_names,
            ..PruneReport::default()
        });
    }

    let kb_of = |dir: &Path| (dir_size_bytes(dir) / 1024).max(1);
    let total_kb: u64 = removable.iter().map(|e| kb_of(&e.dir)).sum();
    let candidate_names: Vec<String> = removable.iter().map(|e| e.name.clone()).collect();

    println!(
        "\n待移入废纸篓: {} 个目录，共约 {}{}",
        removable.len(),
        format_size(total_kb),
        if opts.apply {
            ""
        } else {
            "（预览，加 --apply 执行）"
        }
    );
    for entry in &removable {
        println!("  - {}（{}）", entry.name, format_size(kb_of(&entry.dir)));
    }

    if !opts.apply {
        println!("\n[dry-run] 未做任何改动。确认无误后加 --apply。");
        return Ok(PruneReport {
            protected: protected_names,
            candidates: candidate_names,
            moved: Vec::new(),
        });
    }

    let trash_dir = default_trash_dir(&dist_dir)?;
    let mut moved = Vec::new();
    for entry in &removable {
        let dest = trash_destination(&trash_dir, &entry.dir);
        unregister_apps_macos(&entry.dir);
        fs::rename(&entry.dir, &dest)?;
        println!("  已移入废纸篓: {} -> {}", entry.name, dest.display());
        moved.push(dest);
    }
    println!(
        "\n已移入废纸篓 {} 个历史构建。清空废纸篓后可释放约 {}。",
        moved.len(),
        format_size(total_kb)
    );
    Ok(PruneReport {
        protected: protected_names,
        candidates: candidate_names,
        moved,
    })
}
