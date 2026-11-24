use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn bundle_manager_css(manifest_dir: &Path, out_dir: &Path) {
    let css_dir = manifest_dir.join("../../../apps/codex-plus-manager/src");
    println!("cargo:rerun-if-changed={}", css_dir.display());
    let mut files: Vec<PathBuf> = fs::read_dir(&css_dir)
        .expect("apps/codex-plus-manager/src must exist")
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().and_then(|e| e.to_str()) == Some("css"))
        .collect();

    files.sort_by(|a, b| {
        let an = a.file_name().and_then(|n| n.to_str()).unwrap_or("");
        let bn = b.file_name().and_then(|n| n.to_str()).unwrap_or("");
        match (an == "styles.css", bn == "styles.css") {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => an.cmp(bn),
        }
    });

    let mut combined = String::with_capacity(700_000);
    combined.push_str("/* Bundled original codex-plus-manager CSS sheets */\n");
    for file in files {
        println!("cargo:rerun-if-changed={}", file.display());
        if let Ok(content) = fs::read_to_string(&file) {
            combined.push_str("\n/* === ");
            combined.push_str(&file.file_name().unwrap_or_default().to_string_lossy());
            combined.push_str(" === */\n");
            // Strip Tailwind v4 @import / @custom-variant directives, and promote `@theme {`
            // into `:root {` so custom CSS design tokens (`--color-*`) resolve in plain WebView2.
            for line in content.lines() {
                let trimmed = line.trim_start();
                if trimmed.starts_with("@import \"tailwindcss\"")
                    || trimmed.starts_with("@import 'tailwindcss'")
                    || trimmed.starts_with("@custom-variant ")
                {
                    continue;
                }
                if trimmed == "@theme {" || trimmed.starts_with("@theme {") {
                    combined.push_str(&line.replacen("@theme", ":root", 1));
                    combined.push('\n');
                    continue;
                }
                combined.push_str(line);
                combined.push('\n');
            }
        }
    }
    fs::write(out_dir.join("combined-manager.css"), combined).expect("write combined-manager.css");
}

fn parse_crate_semver(dir_name: &str, prefix: &str) -> (u64, u64, u64) {
    let ver = dir_name.strip_prefix(prefix).unwrap_or("");
    let core = ver.split(['-', '+']).next().unwrap_or("");
    let mut parts = core.split('.').map(|s| s.parse::<u64>().unwrap_or(0));
    (
        parts.next().unwrap_or(0),
        parts.next().unwrap_or(0),
        parts.next().unwrap_or(0),
    )
}

fn find_webview2_loader_dll() -> Option<PathBuf> {
    let arch_dir = match env::var("CARGO_CFG_TARGET_ARCH").ok().as_deref() {
        Some("x86_64") => "x64",
        Some("aarch64") => "arm64",
        Some("x86") => "x86",
        _ => return None,
    };
    let cargo_home = env::var_os("CARGO_HOME")
        .map(PathBuf::from)
        .or_else(|| env::var_os("USERPROFILE").map(|h| PathBuf::from(h).join(".cargo")))
        .or_else(|| env::var_os("HOME").map(|h| PathBuf::from(h).join(".cargo")))?;

    let registry_src = cargo_home.join("registry").join("src");
    let indices = fs::read_dir(registry_src).ok()?;
    let mut candidates: Vec<((u64, u64, u64), PathBuf)> = Vec::new();
    for idx in indices.filter_map(Result::ok) {
        let Ok(crates) = fs::read_dir(idx.path()) else {
            continue;
        };
        for entry in crates.filter_map(Result::ok) {
            let p = entry.path();
            let Some(name) = p.file_name().and_then(|n| n.to_str()) else {
                continue;
            };
            if !name.starts_with("webview2-com-sys-") {
                continue;
            }
            let dll = p.join(arch_dir).join("WebView2Loader.dll");
            if dll.is_file() {
                candidates.push((parse_crate_semver(name, "webview2-com-sys-"), dll));
            }
        }
    }
    candidates.sort_by_key(|a| a.0);
    candidates.pop().map(|(_, dll)| dll)
}

fn stage_webview2_loader(out_dir: &Path) {
    if env::var("CARGO_CFG_WINDOWS").is_err() {
        return;
    }
    let Some(dll) = find_webview2_loader_dll() else {
        return;
    };
    let profile_dir = out_dir
        .ancestors()
        .find(|p| p.file_name().and_then(|n| n.to_str()) == Some("build"))
        .and_then(|build_dir| build_dir.parent())
        .or_else(|| out_dir.ancestors().nth(3));
    if let Some(profile_dir) = profile_dir {
        let _ = fs::copy(&dll, profile_dir.join("WebView2Loader.dll"));
        let deps_dir = profile_dir.join("deps");
        if deps_dir.is_dir() {
            let _ = fs::copy(&dll, deps_dir.join("WebView2Loader.dll"));
        }
    }
}

fn main() {
    let manifest_dir =
        PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR"));
    bundle_manager_css(&manifest_dir, &out_dir);
    stage_webview2_loader(&out_dir);
}
