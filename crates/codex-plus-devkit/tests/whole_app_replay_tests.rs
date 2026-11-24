//! Phase 6 whole-app CLI replay & zero-fallback verification suite.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("workspace root")
        .to_path_buf()
}

fn collect_non_test_rs_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.filter_map(Result::ok) {
        let p = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        if p.is_dir() {
            if name == "tests"
                || name == "target"
                || name == "codex-plus-core"
                || name == "codex-plus-data"
                || name.starts_with('.')
            {
                continue;
            }
            collect_non_test_rs_files(&p, out);
        } else if p.extension().and_then(|e| e.to_str()) == Some("rs") && !name.contains("test") {
            out.push(p);
        }
    }
}

#[test]
fn zero_fallback_kill_switches_and_zero_python_node_in_production_crates() {
    let crates_dir = workspace_root().join("crates");
    let mut rs_files = Vec::new();
    collect_non_test_rs_files(&crates_dir, &mut rs_files);
    assert!(
        rs_files.len() >= 400,
        "expected >= 400 production .rs files, found {}",
        rs_files.len()
    );

    for file in rs_files {
        let content = fs::read_to_string(&file).unwrap_or_default();
        assert!(
            !content.contains("RUST_ANYTHING_UNIT_"),
            "leftover temporary kill switch in {}",
            file.display()
        );
        for banned in [
            "Command::new(\"node\")",
            "Command::new(\"python\")",
            "Command::new(\"python3\")",
        ] {
            assert!(
                !content.contains(banned),
                "unexpected runtime interpreter fallback `{banned}` in {}",
                file.display()
            );
        }
    }
}

#[test]
fn devkit_pure_rust_relaunch_dry_run_and_release_meta_replay_100_percent() {
    let bin = env!("CARGO_BIN_EXE_codex-plus-devkit");
    let root = workspace_root();

    // 1. Pure-Rust relaunch dry-run requires zero npm/Vite
    let out = Command::new(bin)
        .args([
            "relaunch-local",
            "--pure-rust",
            "--dry-run",
            "--root",
            &root.to_string_lossy(),
        ])
        .output()
        .expect("run devkit relaunch-local --pure-rust --dry-run");
    assert!(
        out.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("[dry-run] mode=pure-rust"));
    assert!(stdout.contains("[dry-run] node_npm_required=false"));

    // 2. Release metadata check against live repo v2.8.0
    let meta = Command::new(bin)
        .args([
            "release-meta",
            "check",
            "2.8.0",
            "--root",
            &root.to_string_lossy(),
        ])
        .output()
        .expect("run release-meta check 2.8.0");
    assert!(
        meta.status.success(),
        "release-meta check failed: {}",
        String::from_utf8_lossy(&meta.stderr)
    );

    // 3. Renderer-inject shard manifest assembly check
    let inject = Command::new(bin)
        .args([
            "assemble-renderer-inject",
            "--check",
            "--root",
            &root.to_string_lossy(),
        ])
        .output()
        .expect("run assemble-renderer-inject --check");
    assert!(
        inject.status.success(),
        "assemble-renderer-inject --check failed: {}",
        String::from_utf8_lossy(&inject.stderr)
    );
}

#[test]
fn native_browser_inspector_binary_replays_without_node() {
    let bin = env!("CARGO_BIN_EXE_native-browser-inspector");
    let root = workspace_root();
    let specimen =
        fs::read_to_string(root.join("assets/native-browser/fixtures/service-specimen.mjs"))
            .expect("read service-specimen.mjs");
    let valid_entry = "import * as cua from '@oai/cua-repl';\nexport async function main() { return await cua.launch(); }\n";
    let payload = serde_json::json!({
        "entry": valid_entry,
        "service": specimen,
    })
    .to_string();

    let mut child = Command::new(bin)
        .arg("--stdin")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn native-browser-inspector");
    child
        .stdin
        .as_mut()
        .expect("stdin")
        .write_all(payload.as_bytes())
        .expect("write payload");
    let out = child
        .wait_with_output()
        .expect("wait native-browser-inspector");
    assert!(
        out.status.success(),
        "expected exit 0, stderr={}",
        String::from_utf8_lossy(&out.stderr)
    );
    let parsed: serde_json::Value =
        serde_json::from_slice(&out.stdout).expect("valid inspector JSON output");
    assert!(parsed.is_object(), "expected JSON object contract");

    let mut bad = Command::new(bin)
        .arg("--stdin")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn bad case");
    bad.stdin
        .as_mut()
        .expect("stdin")
        .write_all(b"{\"entry\":\"\",\"service\":\"export const broken = 1;\"}")
        .expect("write invalid payload");
    let bad_out = bad.wait_with_output().expect("wait bad case");
    assert_eq!(bad_out.status.code(), Some(2));
    assert!(
        String::from_utf8_lossy(&bad_out.stderr)
            .contains("Browser service does not match the supported semantic contract.")
    );
}
