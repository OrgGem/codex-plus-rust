//! Cross-platform `relaunch-local` supporting macOS `Codex++.app` AND Windows `CodexPlusPlus.exe`.

use crate::local_builds::{PruneOptions, prune_local_builds};
use anyhow::{Context, Result, bail, ensure};
use regex::Regex;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

pub const CLIENT_OWNED_FLAGS: &[&str] = &[
    "--codex-plus-cu-mcp",
    "--codex-plus-browser-mcp",
    "--codex-plus-agent-mcp",
    "--codex-plus-dsh-mcp",
    "--codex-plus-extension-mcp",
    "--codex-plus-plugin-mcp",
    "--codex-plus-claude-extension-mcp",
    "--codex-plus-hook",
    "--codex-plus-browser-turn-end",
    "--codex-plus-cu-cli",
];

pub fn clean_launch_env<'a>(
    vars: impl IntoIterator<Item = (&'a str, &'a str)>,
) -> HashMap<String, String> {
    let stripped = Regex::new(r"^(?i)(CLAUDE_CODE_|CLAUDECODE$|ANTHROPIC_|(HTTPS?|ALL|NO)_PROXY$)")
        .expect("valid strip regex");
    vars.into_iter()
        .filter(|(k, _)| !stripped.is_match(k))
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

pub fn should_stop_process_cmdline(cmdline: &str, exe_marker: &str) -> bool {
    let lower_cmd = cmdline.replace('\\', "/").to_lowercase();
    let lower_marker = exe_marker
        .trim_start_matches(r"\\?\")
        .replace('\\', "/")
        .to_lowercase();
    if lower_marker.is_empty() || !lower_cmd.contains(&lower_marker) {
        return false;
    }
    !cmdline
        .split_whitespace()
        .any(|tok| CLIENT_OWNED_FLAGS.contains(&tok))
}

#[derive(Clone, Debug, Default)]
pub struct RelaunchOptions {
    pub skip_build: bool,
    pub no_open: bool,
    pub dry_run: bool,
    pub pure_rust: bool,
}

fn clean_abs_path(path: &Path) -> Result<PathBuf> {
    let abs = std::path::absolute(path)?;
    let s = abs.to_string_lossy();
    if let Some(rest) = s.strip_prefix(r"\\?\UNC\") {
        Ok(PathBuf::from(format!(r"\\{rest}")))
    } else if let Some(rest) = s.strip_prefix(r"\\?\") {
        Ok(PathBuf::from(rest))
    } else {
        Ok(abs)
    }
}

fn relaunch_pure_rust(root: &Path, opts: &RelaunchOptions) -> Result<()> {
    let root = clean_abs_path(root)?;
    let root = root.as_path();
    let port_dir = if root.join("crates/codex-plus-app/Cargo.toml").is_file() {
        root.to_path_buf()
    } else {
        root.join("rust-port")
    };
    let workspace_manifest = port_dir.join("Cargo.toml");
    let app_manifest = port_dir.join("crates/codex-plus-app/Cargo.toml");
    let manifest = if workspace_manifest.is_file() {
        workspace_manifest.clone()
    } else {
        app_manifest.clone()
    };
    let target_dir = port_dir.join("target");
    let bin_name = if cfg!(windows) {
        "codex-plus-app.exe"
    } else {
        "codex-plus-app"
    };
    let built = target_dir.join("release").join(bin_name);
    let bundle_dir = target_dir.join("local-bundle");
    let exe_dest = bundle_dir.join(bin_name);
    let state_dir = bundle_dir.join("state");

    if opts.dry_run {
        println!("[dry-run] mode=pure-rust");
        println!("[dry-run] manifest={}", manifest.display());
        println!("[dry-run] built={}", built.display());
        println!("[dry-run] dest={}", exe_dest.display());
        println!("[dry-run] state={}", state_dir.display());
        println!("[dry-run] node_npm_required=false");
        return Ok(());
    }

    ensure!(
        app_manifest.is_file(),
        "Pure-Rust manifest not found: {}",
        app_manifest.display()
    );
    if !opts.skip_build {
        let mut cmd = Command::new("cargo");
        if workspace_manifest.is_file() {
            cmd.args(["build", "--release", "-p", "codex-plus-app"])
                .current_dir(&port_dir);
        } else {
            cmd.args([
                "build",
                "--manifest-path",
                &app_manifest.to_string_lossy(),
                "--release",
            ])
            .current_dir(root);
        }
        let status = cmd
            .env("CARGO_TARGET_DIR", &target_dir)
            .status()
            .context("failed to invoke cargo build for codex-plus-app")?;
        ensure!(
            status.success(),
            "cargo build --release for codex-plus-app failed"
        );
    }
    ensure!(
        built.is_file(),
        "Pure-Rust binary not found at {}",
        built.display()
    );

    fs::create_dir_all(&bundle_dir)?;
    fs::create_dir_all(&state_dir)?;
    if cfg!(windows) {
        stop_windows_instances(&exe_dest)?;
        let dll_src = target_dir.join("release").join("WebView2Loader.dll");
        if dll_src.is_file() {
            let _ = fs::copy(&dll_src, bundle_dir.join("WebView2Loader.dll"));
        }
    } else {
        stop_unix_process_pattern(&exe_dest.to_string_lossy())?;
    }
    let staging = bundle_dir.join(format!(".{bin_name}.staging"));
    fs::copy(&built, &staging)?;
    fs::rename(&staging, &exe_dest)?;

    if !opts.no_open {
        let env_map: HashMap<String, String> = clean_launch_env(
            std::env::vars()
                .collect::<Vec<_>>()
                .iter()
                .map(|(k, v)| (k.as_str(), v.as_str())),
        );
        Command::new(&exe_dest)
            .current_dir(&bundle_dir)
            .env_clear()
            .envs(env_map)
            .env("CODEX_PLUS_STATE_DIR", &state_dir)
            .env("CODEX_PLUS_MANAGER_GUARD_PORT", "63132")
            .env("CODEX_PLUS_LAUNCHER_GUARD_PORT", "63133")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .context("failed to launch Pure-Rust codex-plus-app")?;
    }
    Ok(())
}

pub fn relaunch_local(root: &Path, opts: &RelaunchOptions) -> Result<()> {
    let root = clean_abs_path(root)?;
    let root = root.as_path();
    if opts.pure_rust {
        return relaunch_pure_rust(root, opts);
    }
    let dist_dir = root.join("dist");
    let bundle_dir = dist_dir.join("local-build");
    let state_dir = bundle_dir.join("state");

    if cfg!(windows) {
        let exe_dest = bundle_dir.join("CodexPlusPlus.exe");
        let built = root
            .join("target")
            .join("release")
            .join("codex-plus-plus.exe");
        let alt_built = root
            .join("target")
            .join("release")
            .join("CodexPlusPlus.exe");
        if opts.dry_run {
            println!("[dry-run] platform=windows");
            println!("[dry-run] built={}", built.display());
            println!("[dry-run] dest={}", exe_dest.display());
            println!("[dry-run] state={}", state_dir.display());
            println!("[dry-run] exclude_flags={}", CLIENT_OWNED_FLAGS.join(" "));
            let _ = prune_local_builds(
                root,
                &PruneOptions {
                    apply: false,
                    keep: 0,
                    min_age_minutes: 30,
                },
            );
            return Ok(());
        }
        if !opts.skip_build {
            run_frontend_and_cargo(root)?;
        }
        let src_bin = if built.is_file() {
            built
        } else if alt_built.is_file() {
            alt_built
        } else {
            bail!(
                "Built release binary not found at {} or {}",
                built.display(),
                alt_built.display()
            );
        };
        fs::create_dir_all(&bundle_dir)?;
        fs::create_dir_all(&state_dir)?;
        stop_windows_instances(&exe_dest)?;
        let staging = bundle_dir.join(".CodexPlusPlus.exe.staging");
        fs::copy(&src_bin, &staging)?;
        fs::rename(&staging, &exe_dest)?;
        if !opts.no_open {
            let env_map: HashMap<String, String> = clean_launch_env(
                std::env::vars()
                    .collect::<Vec<_>>()
                    .iter()
                    .map(|(k, v)| (k.as_str(), v.as_str())),
            );
            Command::new(&exe_dest)
                .env_clear()
                .envs(env_map)
                .env("CODEX_PLUS_STATE_DIR", &state_dir)
                .env("CODEX_PLUS_MANAGER_GUARD_PORT", "63122")
                .env("CODEX_PLUS_LAUNCHER_GUARD_PORT", "63123")
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .context("failed to start Windows CodexPlusPlus.exe")?;
        }
        let _ = prune_local_builds(
            root,
            &PruneOptions {
                apply: true,
                keep: 0,
                min_age_minutes: 30,
            },
        );
        return Ok(());
    }

    // macOS / Unix local bundle flow
    let bundle = bundle_dir.join("Codex++.app");
    let built = root.join("target").join("release").join("codex-plus-plus");
    if opts.dry_run {
        println!("[dry-run] platform=macos");
        println!("[dry-run] built={}", built.display());
        println!("[dry-run] bundle={}", bundle.display());
        println!("[dry-run] state={}", state_dir.display());
        println!("[dry-run] exclude_flags={}", CLIENT_OWNED_FLAGS.join(" "));
        let _ = prune_local_builds(
            root,
            &PruneOptions {
                apply: false,
                keep: 0,
                min_age_minutes: 30,
            },
        );
        return Ok(());
    }
    ensure!(
        bundle.exists(),
        "Missing fixed local bundle: {}",
        bundle.display()
    );
    if !opts.skip_build {
        run_frontend_and_cargo(root)?;
    }
    ensure!(
        built.is_file(),
        "Built binary not found: {}",
        built.display()
    );

    let staging_dir = dist_dir.join(".local-build-staging");
    let staging_bundle = staging_dir.join("Codex++.app");
    if staging_dir.exists() {
        fs::remove_dir_all(&staging_dir)?;
    }
    fs::create_dir_all(&staging_dir)?;
    copy_tree(&bundle, &staging_bundle)?;
    let macos_bin = staging_bundle.join("Contents/MacOS/CodexPlusPlus");
    fs::copy(&built, &macos_bin)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&macos_bin, fs::Permissions::from_mode(0o755))?;
    }
    let _icon_updated = refresh_macos_icon(root, &staging_dir, &staging_bundle)?;
    let signing_id = find_macos_signing_identity();
    run_inherit(
        "codesign",
        &[
            "--force",
            "--deep",
            "--timestamp=none",
            "--sign",
            &signing_id,
        ],
        Some(&staging_bundle),
        root,
    )?;
    stop_unix_instances(&bundle)?;
    let previous = bundle_dir.join(".Codex++.app.previous");
    if previous.exists() {
        fs::remove_dir_all(&previous)?;
    }
    fs::rename(&bundle, &previous)?;
    fs::rename(&staging_bundle, &bundle)?;
    if previous.exists() {
        fs::remove_dir_all(&previous)?;
    }
    if staging_dir.exists() {
        fs::remove_dir_all(&staging_dir)?;
    }
    if !opts.no_open {
        let env_map: HashMap<String, String> = clean_launch_env(
            std::env::vars()
                .collect::<Vec<_>>()
                .iter()
                .map(|(k, v)| (k.as_str(), v.as_str())),
        );
        Command::new("open")
            .args(["-n"])
            .arg(&bundle)
            .env_clear()
            .envs(env_map)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;
    }
    let _ = prune_local_builds(
        root,
        &PruneOptions {
            apply: true,
            keep: 0,
            min_age_minutes: 30,
        },
    );
    Ok(())
}

fn run_frontend_and_cargo(root: &Path) -> Result<()> {
    let npm = if cfg!(windows) { "npm.cmd" } else { "npm" };
    let status = Command::new(npm)
        .args(["run", "vite:build"])
        .current_dir(root.join("apps/codex-plus-manager"))
        .status()?;
    ensure!(status.success(), "npm run vite:build failed");
    let status = Command::new("cargo")
        .args(["build", "--release", "-p", "codex-plus-manager"])
        .current_dir(root)
        .env("CARGO_PROFILE_RELEASE_INCREMENTAL", "true")
        .status()?;
    ensure!(status.success(), "cargo build --release failed");
    Ok(())
}

fn run_inherit(prog: &str, args: &[&str], target: Option<&Path>, cwd: &Path) -> Result<()> {
    let mut cmd = Command::new(prog);
    cmd.args(args).current_dir(cwd);
    if let Some(t) = target {
        cmd.arg(t);
    }
    let status = cmd.status()?;
    ensure!(status.success(), "{prog} failed");
    Ok(())
}

fn copy_tree(src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let from = entry.path();
        let to = dst.join(entry.file_name());
        let meta = fs::symlink_metadata(&from)?;
        if meta.file_type().is_symlink() {
            #[cfg(unix)]
            {
                let target = fs::read_link(&from)?;
                std::os::unix::fs::symlink(target, &to)?;
            }
        } else if meta.is_dir() {
            copy_tree(&from, &to)?;
        } else {
            fs::copy(&from, &to)?;
        }
    }
    Ok(())
}

fn refresh_macos_icon(root: &Path, staging: &Path, bundle: &Path) -> Result<bool> {
    let src = root.join("apps/codex-plus-manager/src-tauri/icons/icon-macos.png");
    if !src.is_file() {
        return Ok(false);
    }
    let digest = format!("{:x}", Sha256::digest(fs::read(&src)?));
    let icns = bundle.join("Contents/Resources/codex-plus-plus.icns");
    let marker = bundle.join("Contents/Resources/.icon-source.sha256");
    if icns.exists()
        && fs::read_to_string(&marker)
            .map(|s| s.trim() == digest)
            .unwrap_or(false)
    {
        return Ok(false);
    }
    let iconset = staging.join("codex-plus-plus.iconset");
    fs::create_dir_all(&iconset)?;
    for (name, sz) in [
        ("icon_16x16", "16"),
        ("icon_16x16@2x", "32"),
        ("icon_32x32", "32"),
        ("icon_32x32@2x", "64"),
        ("icon_128x128", "128"),
        ("icon_128x128@2x", "256"),
        ("icon_256x256", "256"),
        ("icon_256x256@2x", "512"),
        ("icon_512x512", "512"),
        ("icon_512x512@2x", "1024"),
    ] {
        let _ = Command::new("sips")
            .args(["-z", sz, sz])
            .arg(&src)
            .arg("--out")
            .arg(iconset.join(format!("{name}.png")))
            .output();
    }
    if let Some(p) = icns.parent() {
        fs::create_dir_all(p)?;
    }
    let _ = Command::new("iconutil")
        .args(["-c", "icns"])
        .arg(&iconset)
        .arg("-o")
        .arg(&icns)
        .status();
    fs::write(&marker, format!("{digest}\n"))?;
    Ok(true)
}

fn find_macos_signing_identity() -> String {
    let out = Command::new("security")
        .args(["find-identity", "-v", "-p", "codesigning"])
        .output();
    if let Ok(o) = out {
        let text = String::from_utf8_lossy(&o.stdout);
        let re = Regex::new(r#""(Developer ID Application:[^"]+)""#).ok();
        if let Some(id) = re.and_then(|r| r.captures(&text)).map(|c| c[1].to_string()) {
            return id;
        }
    }
    "-".to_string()
}

fn alive_windows_pids(candidates: &[String]) -> Vec<String> {
    let still = Command::new("powershell")
        .args([
            "-NoProfile",
            "-Command",
            "Get-CimInstance Win32_Process | Select-Object -ExpandProperty ProcessId",
        ])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).into_owned())
        .unwrap_or_default();
    candidates
        .iter()
        .filter(|p| still.lines().any(|l| l.trim() == p.as_str()))
        .cloned()
        .collect()
}

fn stop_windows_instances(exe: &Path) -> Result<()> {
    let abs_exe = clean_abs_path(exe).unwrap_or_else(|_| exe.to_path_buf());
    let marker = abs_exe.to_string_lossy().into_owned();
    let out = Command::new("powershell")
        .args([
            "-NoProfile",
            "-Command",
            "Get-CimInstance Win32_Process | ForEach-Object { \"$($_.ProcessId)\t$($_.ExecutablePath)\t$($_.CommandLine)\" }",
        ])
        .output()?;
    let text = String::from_utf8_lossy(&out.stdout);
    let my_pid = std::process::id().to_string();
    let mut pids = Vec::new();
    for line in text.lines() {
        let Some((pid, rest)) = line.split_once('\t') else {
            continue;
        };
        if pid.trim() == my_pid {
            continue;
        }
        if should_stop_process_cmdline(rest, &marker) {
            pids.push(pid.trim().to_string());
        }
    }
    if pids.is_empty() {
        return Ok(());
    }
    for pid in &pids {
        let _ = Command::new("taskkill").args(["/PID", pid]).output();
    }
    for _ in 0..15 {
        if alive_windows_pids(&pids).is_empty() {
            return Ok(());
        }
        thread::sleep(Duration::from_millis(200));
    }
    let alive = alive_windows_pids(&pids);
    for pid in &alive {
        let _ = Command::new("taskkill").args(["/F", "/PID", pid]).output();
    }
    for _ in 0..15 {
        if alive_windows_pids(&alive).is_empty() {
            return Ok(());
        }
        thread::sleep(Duration::from_millis(200));
    }
    let remaining = alive_windows_pids(&alive);
    if !remaining.is_empty() {
        bail!(
            "Existing local Windows process failed to exit (PIDs: {})",
            remaining.join(", ")
        );
    }
    Ok(())
}

fn stop_unix_instances(bundle: &Path) -> Result<()> {
    stop_unix_process_pattern(&format!("{}/Contents/MacOS/", bundle.display()))
}

fn stop_unix_process_pattern(raw_pattern: &str) -> Result<()> {
    let pattern = regex::escape(raw_pattern);
    let out = Command::new("pgrep").args(["-lf", &pattern]).output();
    let Ok(o) = out else {
        return Ok(());
    };
    let my_pid = std::process::id().to_string();
    let mut pids = Vec::new();
    for line in String::from_utf8_lossy(&o.stdout).lines() {
        let t = line.trim();
        let Some((pid, rest)) = t.split_once(char::is_whitespace) else {
            continue;
        };
        if pid != my_pid
            && !rest
                .split_whitespace()
                .any(|tok| CLIENT_OWNED_FLAGS.contains(&tok))
        {
            pids.push(pid.to_string());
        }
    }
    if pids.is_empty() {
        return Ok(());
    }
    for pid in &pids {
        let _ = Command::new("kill").args(["-TERM", pid]).output();
    }
    for _ in 0..25 {
        let any_alive = pids.iter().any(|p| {
            Command::new("kill")
                .args(["-0", p])
                .status()
                .map(|s| s.success())
                .unwrap_or(false)
        });
        if !any_alive {
            return Ok(());
        }
        thread::sleep(Duration::from_millis(200));
    }
    bail!("Existing local bundle process failed to exit")
}

#[allow(dead_code)]
fn _keep_pathbuf(_: PathBuf) {}
