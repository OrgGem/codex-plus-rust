//! macOS bottom-up code signing with restricted entitlement stripping & fixed `untrusted_only` check.

use anyhow::{Context, Result, bail};
use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::{Component, Path, PathBuf};
use std::process::Command;

pub const DEFAULT_SRC_APP: &str = "/Applications/Claude.app";
pub const DR: &str =
    r#"designated => (identifier "com.anthropic.claudefordesktop" and ! anchor apple generic)"#;
pub const SIGN_IDENTITY_NAME: &str = "Claude Enhanced Local";

pub const STRIP_ENTITLEMENT_KEYS: &[&str] = &[
    "com.apple.application-identifier",
    "com.apple.developer.team-identifier",
    "keychain-access-groups",
];

pub const MACHO_MAGICS: &[[u8; 4]] = &[
    [0xcf, 0xfa, 0xed, 0xfe],
    [0xfe, 0xed, 0xfa, 0xcf],
    [0xce, 0xfa, 0xed, 0xfe],
    [0xfe, 0xed, 0xfa, 0xce],
    [0xca, 0xfe, 0xba, 0xbe],
    [0xbe, 0xba, 0xfe, 0xca],
    [0xca, 0xfe, 0xba, 0xbf],
    [0xbf, 0xba, 0xfe, 0xca],
];

pub fn is_macho_magic(bytes: &[u8]) -> bool {
    if bytes.len() < 4 {
        return false;
    }
    MACHO_MAGICS.iter().any(|m| bytes[..4] == *m)
}

pub fn is_macho(path: &Path) -> bool {
    let Ok(meta) = fs::symlink_metadata(path) else {
        return false;
    };
    if meta.file_type().is_symlink() || !meta.is_file() {
        return false;
    }
    let Ok(mut f) = fs::File::open(path) else {
        return false;
    };
    let mut magic = [0u8; 4];
    if f.read_exact(&mut magic).is_err() {
        return false;
    }
    is_macho_magic(&magic)
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SignTargets {
    pub loose: Vec<PathBuf>,
    pub frameworks: Vec<PathBuf>,
    pub apps: Vec<PathBuf>,
}

pub fn path_depth(p: &Path) -> usize {
    p.components()
        .filter(|c| !matches!(c, Component::CurDir))
        .count()
}

/// Walks `contents_dir` skipping symlinks and returns signing targets ordered deepest-first per tier.
pub fn collect_sign_targets(contents_dir: &Path) -> Result<SignTargets> {
    let mut targets = SignTargets::default();
    if !contents_dir.is_dir() {
        return Ok(targets);
    }

    fn walk_dir(dir: &Path, targets: &mut SignTargets) -> Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let meta = fs::symlink_metadata(&path)?;
            if meta.file_type().is_symlink() {
                continue;
            }
            if meta.is_dir() {
                let name = entry.file_name();
                let name_str = name.to_string_lossy();
                if name_str.ends_with(".framework") {
                    targets.frameworks.push(path.clone());
                } else if name_str.ends_with(".app") {
                    targets.apps.push(path.clone());
                }
                walk_dir(&path, targets)?;
            } else if meta.is_file() && is_macho(&path) {
                targets.loose.push(path);
            }
        }
        Ok(())
    }

    walk_dir(contents_dir, &mut targets)?;
    targets
        .loose
        .sort_by_key(|p| std::cmp::Reverse(path_depth(p)));
    targets
        .frameworks
        .sort_by_key(|p| std::cmp::Reverse(path_depth(p)));
    targets
        .apps
        .sort_by_key(|p| std::cmp::Reverse(path_depth(p)));
    Ok(targets)
}

/// Strips restricted team/keychain keys from XML entitlements plist and ensures
/// `com.apple.security.cs.disable-library-validation` is `<true/>`.
pub fn sanitize_entitlements_xml(xml: &str) -> String {
    let mut cleaned = if xml.trim().is_empty() {
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">\n<plist version=\"1.0\">\n<dict>\n</dict>\n</plist>\n".to_string()
    } else {
        xml.to_string()
    };

    for key in STRIP_ENTITLEMENT_KEYS {
        let needle = format!("<key>{key}</key>");
        while let Some(kpos) = cleaned.find(&needle) {
            let after_key = kpos + needle.len();
            // Locate line start before <key> if preceded only by horizontal whitespace.
            let line_start = cleaned[..kpos].rfind('\n').map_or(0, |i| i + 1);
            let indent_only = cleaned[line_start..kpos]
                .chars()
                .all(|c| c == ' ' || c == '\t');
            let start = if indent_only { line_start } else { kpos };
            let tail = &cleaned[after_key..];
            let Some(end_rel) = next_plist_value_end(tail) else {
                break;
            };
            let mut end = after_key + end_rel;
            if indent_only {
                let ws_len: usize = cleaned[end..]
                    .chars()
                    .take_while(|&c| c == ' ' || c == '\t')
                    .map(char::len_utf8)
                    .sum();
                if cleaned[end + ws_len..].starts_with("\r\n") {
                    end += ws_len + 2;
                } else if cleaned[end + ws_len..].starts_with('\n') {
                    end += ws_len + 1;
                }
            }
            cleaned.replace_range(start..end, "");
        }
    }

    let lib_val_key = "<key>com.apple.security.cs.disable-library-validation</key>";
    if !cleaned.contains(lib_val_key) {
        if let Some(close_dict) = cleaned.rfind("</dict>") {
            cleaned.insert_str(
                close_dict,
                "\t<key>com.apple.security.cs.disable-library-validation</key>\n\t<true/>\n",
            );
        }
    } else {
        // Ensure existing value is <true/>
        if let Some(kpos) = cleaned.find(lib_val_key) {
            let after = kpos + lib_val_key.len();
            if let Some(end_rel) = next_plist_value_end(&cleaned[after..]) {
                cleaned.replace_range(after..after + end_rel, "\n\t<true/>");
            }
        }
    }
    cleaned
}

fn skip_ws_and_xml_comments(s: &str, mut pos: usize) -> Option<usize> {
    loop {
        let trimmed = s[pos..].trim_start();
        pos = s.len() - trimmed.len();
        if s[pos..].starts_with("<!--") {
            let close = s[pos + 4..].find("-->")?;
            pos += 4 + close + 3;
        } else {
            return Some(pos);
        }
    }
}

fn next_plist_value_end(s: &str) -> Option<usize> {
    let pos = skip_ws_and_xml_comments(s, 0)?;
    let rest = &s[pos..];
    if !rest.starts_with('<') || rest.starts_with("</") || rest.starts_with("<?") {
        return None;
    }
    let close_angle_rel = rest.find('>')?;
    let open_end = pos + close_angle_rel;
    let inner = s[pos + 1..open_end].trim();

    if let Some(without_slash) = inner.strip_suffix('/') {
        let tag_name = without_slash.split_whitespace().next().unwrap_or("");
        return matches!(
            tag_name,
            "true" | "false" | "string" | "data" | "integer" | "real" | "date" | "array" | "dict"
        )
        .then_some(open_end + 1);
    }

    let tag_name = inner.split_whitespace().next().unwrap_or("");
    match tag_name {
        "string" | "data" | "integer" | "real" | "date" | "array" | "dict" => {
            let mut depth = 1usize;
            let mut cur = open_end + 1;
            while cur < s.len() {
                let lt_rel = s[cur..].find('<')?;
                let lt = cur + lt_rel;
                if s[lt..].starts_with("<!--") {
                    let c = s[lt + 4..].find("-->")?;
                    cur = lt + 4 + c + 3;
                    continue;
                }
                if s[lt..].starts_with("<![CDATA[") {
                    let c = s[lt + 9..].find("]]>")?;
                    cur = lt + 9 + c + 3;
                    continue;
                }
                let gt_rel = s[lt..].find('>')?;
                let gt = lt + gt_rel;
                let tag_inner = s[lt + 1..gt].trim();
                if tag_inner.ends_with('/') {
                    // Self-closing element (`<true/>`, `<array/>`, `<dict/>`, etc.) does not change depth.
                } else if let Some(close_name) = tag_inner.strip_prefix('/') {
                    if close_name.trim() == tag_name {
                        depth -= 1;
                        if depth == 0 {
                            return Some(gt + 1);
                        }
                    }
                } else {
                    let open_name = tag_inner.split_whitespace().next().unwrap_or("");
                    if open_name == tag_name {
                        depth += 1;
                    }
                }
                cur = gt + 1;
            }
            None
        }
        _ => None,
    }
}

/// Evaluates macOS `codesign --verify` outcome matching `build-copy.py:371`
/// and fixing the intentional bug at `sign-copy.py:179` (`known-diffs.jsonl`).
pub fn verify_succeeded(ok: usize, total: usize, exit_code: i32, combined_output: &str) -> bool {
    let untrusted_only = combined_output.contains("CSSMERR_TP_NOT_TRUSTED");
    ok == total && (exit_code == 0 || untrusted_only)
}

fn original_entitlements(src_app: &Path, rel: &Path) -> String {
    let p = src_app.join(rel);
    if !p.exists() {
        return String::new();
    }
    Command::new("codesign")
        .args(["-d", "--entitlements", ":-", "--xml"])
        .arg(&p)
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).into_owned())
        .unwrap_or_default()
}

fn resolve_identity() -> String {
    if let Ok(ov) = std::env::var("CODEX_PLUS_CLAUDE_SIGN_IDENTITY")
        && !ov.is_empty()
    {
        return ov;
    }
    let out = Command::new("security")
        .args(["find-identity", "-p", "codesigning"])
        .output();
    if let Ok(o) = out {
        let text = format!(
            "{}{}",
            String::from_utf8_lossy(&o.stdout),
            String::from_utf8_lossy(&o.stderr)
        );
        if text.contains(SIGN_IDENTITY_NAME) {
            return SIGN_IDENTITY_NAME.to_string();
        }
    }
    "-".to_string()
}

fn run_codesign(identity: &str, path: &Path, ent_path: &Path, extra: &[&str]) -> bool {
    let mut cmd = Command::new("codesign");
    cmd.args([
        "--force",
        "--sign",
        identity,
        "--options",
        "runtime",
        "--timestamp=none",
        "--entitlements",
    ])
    .arg(ent_path);
    cmd.args(extra);
    cmd.arg(path);
    match cmd.output() {
        Ok(r) if r.status.success() => true,
        Ok(r) => {
            let name = path.file_name().unwrap_or_default().to_string_lossy();
            let err = String::from_utf8_lossy(&r.stderr);
            eprintln!(
                "  ✗ {name}: {}",
                err.trim().chars().take(300).collect::<String>()
            );
            false
        }
        Err(e) => {
            eprintln!("  ✗ codesign error: {e}");
            false
        }
    }
}

pub fn sign_copy(src_app: Option<&Path>, copy_app: Option<&Path>) -> Result<()> {
    let src = src_app
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(DEFAULT_SRC_APP));
    let copy = match copy_app {
        Some(p) => p.to_path_buf(),
        None => {
            let home = std::env::var_os("HOME")
                .map(PathBuf::from)
                .context("HOME not set")?;
            home.join(".codex-session-delete/claude-localization/Claude Enhanced.app")
        }
    };
    if !copy.is_dir() {
        bail!("副本不存在: {}", copy.display());
    }

    let targets = collect_sign_targets(&copy.join("Contents"))?;
    let identity = resolve_identity();
    let tmpdir = tempfile::Builder::new()
        .prefix("cn-sign-")
        .tempdir()
        .context("failed to create temporary signing directory")?;

    let mut cache: HashMap<PathBuf, PathBuf> = HashMap::new();
    let mut ent_file = |rel: &Path| -> Result<PathBuf> {
        if let Some(cached) = cache.get(rel) {
            return Ok(cached.clone());
        }
        let xml = sanitize_entitlements_xml(&original_entitlements(&src, rel));
        let dest = tmpdir.path().join(format!("{}.plist", cache.len()));
        fs::write(&dest, xml.as_bytes())?;
        cache.insert(rel.to_path_buf(), dest.clone());
        Ok(dest)
    };

    let mut total = 0usize;
    let mut ok = 0usize;
    for (group, label) in [
        (&targets.loose, "散装 Mach-O"),
        (&targets.frameworks, "framework"),
        (&targets.apps, "嵌套 .app"),
    ] {
        println!("--- 签名{label}：{} 个", group.len());
        for p in group {
            let rel = p.strip_prefix(&copy).unwrap_or(p);
            let ent = ent_file(rel)?;
            total += 1;
            if run_codesign(&identity, p, &ent, &[]) {
                ok += 1;
            }
        }
    }

    println!("--- 签名外层 app（写入自定义 DR）");
    let outer_ent = tmpdir.path().join("outer.plist");
    fs::write(
        &outer_ent,
        sanitize_entitlements_xml(&original_entitlements(&src, Path::new("."))).as_bytes(),
    )?;
    total += 1;
    let dr_arg = format!("-r={DR}");
    if run_codesign(&identity, &copy, &outer_ent, &[&dr_arg]) {
        ok += 1;
    }

    println!("\n完成：{ok}/{total}");
    let verify = Command::new("codesign")
        .args(["--verify", "--deep", "--strict", "--verbose=2"])
        .arg(&copy)
        .output()?;
    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&verify.stderr),
        String::from_utf8_lossy(&verify.stdout)
    );
    let code = verify.status.code().unwrap_or(1);
    if !verify_succeeded(ok, total, code, &combined) {
        eprintln!("verify 输出：\n{}", combined.trim());
        bail!("签名校验失败");
    }
    println!("✓ codesign 校验通过（签名身份：{identity}）");
    Ok(())
}
