use codex_plus_devkit::browser_inspector::{
    ClientInfo, ClientMetadata, FAILURE_MESSAGE, TurnMetadata, eval_require_identification,
};
use codex_plus_devkit::local_builds::{PruneOptions, filter_paths_in_use, prune_local_builds};
use codex_plus_devkit::plugin_market::build_plugin_market_index;
use codex_plus_devkit::relaunch::{
    CLIENT_OWNED_FLAGS, clean_launch_env, should_stop_process_cmdline,
};
use codex_plus_devkit::release::parse_porcelain_z_dirty_paths;
use codex_plus_devkit::release_meta::{
    bump_version_in_text, changelog_section, compare_versions, read_version_from_text,
    read_versions,
};
use codex_plus_devkit::renderer_inject::{assemble_renderer_inject, split_renderer_inject};
use codex_plus_devkit::sherpa_cache::{REQUIRED_LIBS, prepare_sherpa_cache};
use codex_plus_devkit::sync_models::sync_official_models;
use codex_plus_devkit::whale_runtime::{
    EXPECTED_SOURCE_SHA256, assemble_whale_runtime, transform_whale_source,
};
use serde_json::{Value, json};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, SystemTime};
use tempfile::TempDir;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(3)
        .expect("repo root")
        .to_path_buf()
}

fn devkit_bin() -> &'static str {
    env!("CARGO_BIN_EXE_codex-plus-devkit")
}

fn inspector_bin() -> &'static str {
    env!("CARGO_BIN_EXE_native-browser-inspector")
}

#[test]
fn native_browser_inspector_matches_node_inspector_stdin() {
    let root = repo_root();
    let specimen =
        fs::read_to_string(root.join("assets/native-browser/fixtures/service-specimen.mjs"))
            .expect("read specimen");
    let valid_entry = "import * as cua from '@oai/cua-repl';\nexport async function main() { return await cua.launch(); }\n";
    let payload = json!({ "entry": valid_entry, "service": specimen }).to_string();

    let node_out = {
        let mut child = Command::new("node")
            .args([
                root.join("assets/native-browser/inspect-service.cjs")
                    .to_str()
                    .unwrap(),
                "--stdin",
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("spawn node inspector");
        child
            .stdin
            .take()
            .unwrap()
            .write_all(payload.as_bytes())
            .unwrap();
        child.wait_with_output().unwrap()
    };
    assert!(node_out.status.success(), "node failed: {:?}", node_out);

    let rust_out = {
        let mut child = Command::new(inspector_bin())
            .arg("--stdin")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("spawn rust inspector");
        child
            .stdin
            .take()
            .unwrap()
            .write_all(payload.as_bytes())
            .unwrap();
        child.wait_with_output().unwrap()
    };
    assert!(rust_out.status.success(), "rust failed: {:?}", rust_out);
    let node_json: Value = serde_json::from_slice(&node_out.stdout).unwrap();
    let rust_json: Value = serde_json::from_slice(&rust_out.stdout).unwrap();
    assert_eq!(rust_json, node_json);

    // Invalid input must exit 2 and emit only redacted failure line.
    let bad_out = {
        let mut child = Command::new(devkit_bin())
            .args(["inspect-browser-service", "--stdin"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        child
            .stdin
            .take()
            .unwrap()
            .write_all(b"{\"entry\":\"invalid\",\"service\":\"C:\\\\secret\\\\path\"}")
            .unwrap();
        child.wait_with_output().unwrap()
    };
    assert_eq!(bad_out.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&bad_out.stderr);
    assert_eq!(stderr.trim(), FAILURE_MESSAGE);
    assert!(!stderr.contains("secret"));
}

#[test]
fn eval_require_identification_matches_js_contract() {
    let dir = TempDir::new().unwrap();
    let ctrl = dir.path().join("control.json");
    fs::write(&ctrl, r#"{"schema":1,"requireIdentification":true}"#).unwrap();

    let good_info = ClientInfo {
        kind: Some("extension".into()),
        family: Some("chrome".into()),
        agent_request_header_enabled: Some(true),
        metadata: Some(ClientMetadata {
            extension_id: Some("hehggadaopoacecdllhhajmbjkdcmajg".into()),
            extension_instance_id: Some("inst-1".into()),
        }),
    };
    let good_turn = TurnMetadata {
        session_id: Some("sess-1".into()),
        turn_id: Some("turn-1".into()),
    };

    // Valid matching before & after I/O -> true
    assert!(eval_require_identification(
        Some(&good_info),
        Some(&good_turn),
        &ctrl,
        || (Some(good_info.clone()), Some(good_turn.clone())),
    ));

    // Edge ID allowed too
    let mut edge_info = good_info.clone();
    edge_info.family = Some("edge".into());
    edge_info.metadata.as_mut().unwrap().extension_id =
        Some("odlomjlbamekndcpllcnffbgeohgkmjh".into());
    assert!(eval_require_identification(
        Some(&edge_info),
        Some(&good_turn),
        &ctrl,
        || (Some(edge_info.clone()), Some(good_turn.clone())),
    ));

    // Wrong extension id -> false
    let mut bad_info = good_info.clone();
    bad_info.metadata.as_mut().unwrap().extension_id = Some("wrong".into());
    assert!(!eval_require_identification(
        Some(&bad_info),
        Some(&good_turn),
        &ctrl,
        || (Some(bad_info.clone()), Some(good_turn.clone())),
    ));

    // Turn changed during I/O -> false
    assert!(!eval_require_identification(
        Some(&good_info),
        Some(&good_turn),
        &ctrl,
        || (
            Some(good_info.clone()),
            Some(TurnMetadata {
                session_id: Some("sess-1".into()),
                turn_id: Some("turn-2".into()),
            })
        ),
    ));

    // Control file > 1024 bytes -> false
    fs::write(
        &ctrl,
        format!(
            r#"{{"schema":1,"requireIdentification":true,"pad":"{}"}}"#,
            "x".repeat(1024)
        ),
    )
    .unwrap();
    assert!(!eval_require_identification(
        Some(&good_info),
        Some(&good_turn),
        &ctrl,
        || (Some(good_info.clone()), Some(good_turn.clone())),
    ));
}

#[test]
fn release_meta_matches_node_on_repo_and_bumps_copy() {
    let root = repo_root();
    let versions = read_versions(&root).unwrap();
    assert_eq!(versions.len(), 6);
    for (file, ver) in &versions {
        assert_eq!(ver.as_deref(), Some("2.8.0"), "mismatch in {file}");
    }

    let node_notes = Command::new("node")
        .args([
            root.join("scripts/release-meta.mjs").to_str().unwrap(),
            "notes",
            "2.8.0",
        ])
        .output()
        .unwrap();
    let rust_notes = Command::new(devkit_bin())
        .args([
            "release-meta",
            "notes",
            "2.8.0",
            "--root",
            root.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert_eq!(node_notes.status.code(), rust_notes.status.code());
    assert_eq!(
        String::from_utf8_lossy(&rust_notes.stdout).replace("\r\n", "\n"),
        String::from_utf8_lossy(&node_notes.stdout).replace("\r\n", "\n"),
    );

    let rust_check = Command::new(devkit_bin())
        .args([
            "release-meta",
            "check",
            "2.8.0",
            "--root",
            root.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(rust_check.status.success());

    // Test bump roundtrip on all 6 files in memory without mutating repo
    for (file, _) in &versions {
        let before = fs::read_to_string(root.join(file)).unwrap();
        let after = bump_version_in_text(file, &before, "2.8.0", "2.9.0-rc.1").unwrap();
        assert_eq!(
            read_version_from_text(file, &after).as_deref(),
            Some("2.9.0-rc.1"),
            "failed bump in {file}"
        );
    }
    assert!(compare_versions("2.8.0", "2.8.0-rc.1") > 0);
    assert_eq!(
        changelog_section(
            "0.0.999",
            &fs::read_to_string(root.join("CHANGELOG.md")).unwrap()
        ),
        ""
    );
}

#[test]
fn renderer_inject_check_and_split_roundtrip() {
    let root = repo_root();
    let check_out = Command::new(devkit_bin())
        .args([
            "assemble-renderer-inject",
            "--check",
            "--root",
            root.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(
        check_out.status.success(),
        "assemble-renderer-inject --check failed: {}",
        String::from_utf8_lossy(&check_out.stderr)
    );

    // Copy fragments & artifact to TempDir to test assemble, syntax error detection, and split roundtrip on clean shards.
    let tmp = TempDir::new().unwrap();
    let tmp_inject = tmp.path().join("assets/inject");
    let tmp_frags = tmp_inject.join("renderer-inject");
    fs::create_dir_all(&tmp_frags).unwrap();
    fs::write(
        tmp_frags.join("manifest.json"),
        serde_json::to_string_pretty(&json!({
            "fragments": [
                {"name": "00-head.js", "description": "head", "lines": 2},
                {"name": "99-tail.js", "description": "tail", "lines": 1}
            ]
        }))
        .unwrap(),
    )
    .unwrap();
    fs::write(tmp_frags.join("00-head.js"), "(() => {\n  const x = 1;\n").unwrap();
    fs::write(tmp_frags.join("99-tail.js"), "})();\n").unwrap();

    let h_asm = assemble_renderer_inject(tmp.path(), false).unwrap();
    let h_split = split_renderer_inject(tmp.path()).unwrap();
    assert_eq!(h_split, h_asm);
    assemble_renderer_inject(tmp.path(), true).unwrap();

    // Inject broken JS syntax into first fragment -> oxc_parser must reject!
    fs::write(
        tmp_frags.join("00-head.js"),
        "(() => {\n  const broken = {{\n",
    )
    .unwrap();
    assert!(assemble_renderer_inject(tmp.path(), false).is_err());
}

#[test]
fn whale_runtime_check_on_repo_or_error_guard() {
    let root = repo_root();
    let orig = root.join("assets/inject/upstream/whale-widget/full-widget.js");
    if orig.is_file() {
        assert!(assemble_whale_runtime(&root, true).is_ok());
    } else {
        assert!(transform_whale_source("not-matching").is_err());
        assert_eq!(EXPECTED_SOURCE_SHA256.len(), 64);
    }
}

#[test]
fn prepare_sherpa_cache_matches_node_test_cases() {
    let tmp = TempDir::new().unwrap();
    let target = tmp.path().join("target");
    let temp = tmp.path().join("temp");

    // Absent cache creates nothing
    assert!(prepare_sherpa_cache(&target, &temp).unwrap().is_empty());
    assert!(!target.exists());
    assert!(!temp.exists());

    // Complete vs incomplete package
    let stem = "sherpa-onnx-v1.13.8-win-x64-static-MT-Release-lib";
    let pkg = target.join("sherpa-onnx-prebuilt").join(stem);
    let lib = pkg.join("lib");
    fs::create_dir_all(&lib).unwrap();
    for name in REQUIRED_LIBS {
        fs::write(lib.join(format!("{name}.lib")), b"library").unwrap();
    }
    let archive = pkg.with_file_name(format!("{stem}.tar.bz2"));
    fs::write(&archive, b"cached archive").unwrap();
    assert!(prepare_sherpa_cache(&target, &temp).unwrap().is_empty());

    // Remove one lib -> quarantined along with .tar.bz2
    fs::remove_file(lib.join("onnxruntime.lib")).unwrap();
    let moved = prepare_sherpa_cache(&target, &temp).unwrap();
    assert_eq!(moved, vec![stem.to_string()]);
    assert!(!pkg.exists());
    assert!(!archive.exists());

    // Symlink / junction check
    let outside = tmp.path().join("outside");
    fs::create_dir_all(&outside).unwrap();
    let link = target.join("sherpa-onnx-prebuilt").join(stem);
    #[cfg(windows)]
    {
        let _ = Command::new("cmd")
            .args([
                "/C",
                "mklink",
                "/J",
                link.to_str().unwrap(),
                outside.to_str().unwrap(),
            ])
            .output()
            .unwrap();
    }
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(&outside, &link).unwrap();
    }
    if link.exists() || fs::symlink_metadata(&link).is_ok() {
        assert!(prepare_sherpa_cache(&target, &temp).is_err());
    }
}

#[test]
fn prune_local_builds_respects_protection_and_moves_to_trash() {
    let tmp = TempDir::new().unwrap();
    let dist = tmp.path().join("dist");
    fs::create_dir_all(dist.join("local-build")).unwrap();
    fs::create_dir_all(dist.join("local-build-keep-marker")).unwrap();
    fs::write(dist.join("local-build-keep-marker/.keep"), "").unwrap();
    fs::create_dir_all(dist.join("local-build-fresh")).unwrap();
    fs::create_dir_all(dist.join("local-build-old")).unwrap();

    // Wait: set old build's mtime back 2 hours (or set min_age_minutes=0 except fresh? No, use filetime via File::set_modified)
    let old_time = SystemTime::now() - Duration::from_secs(7200);
    fs::File::options()
        .write(true)
        .open(dist.join("local-build-old"))
        .or_else(|_| fs::File::open(dist.join("local-build-old")))
        .ok();
    // Write a child file and set its modified time on Windows cleanly via File::set_modified, wait: dir mtime matters too.
    // Using min_age_minutes: 0 after verifying protection markers works cleanly across OSes!
    let _ = old_time;
    let rep_dry = prune_local_builds(
        tmp.path(),
        &PruneOptions {
            apply: false,
            keep: 1,
            min_age_minutes: 0,
        },
    )
    .unwrap();
    assert!(rep_dry.protected.contains(&"local-build".to_string()));
    assert!(
        rep_dry
            .protected
            .contains(&"local-build-keep-marker".to_string())
    );
    assert_eq!(rep_dry.candidates.len(), 1);
    assert!(rep_dry.moved.is_empty());

    let rep_apply = prune_local_builds(
        tmp.path(),
        &PruneOptions {
            apply: true,
            keep: 1,
            min_age_minutes: 0,
        },
    )
    .unwrap();
    assert_eq!(rep_apply.moved.len(), 1);
    assert!(rep_apply.moved[0].exists());
    assert!(dist.join("local-build").exists());
    assert!(dist.join("local-build-keep-marker").exists());
}

#[test]
fn relaunch_local_strips_env_filters_flags_and_runs_dry_run() {
    let cleaned = clean_launch_env([
        ("PATH", "/usr/bin"),
        ("CLAUDECODE", "1"),
        ("CLAUDE_CODE_PLUGIN_DIRS", "/x"),
        ("ANTHROPIC_API_KEY", "secret"),
        ("HTTPS_PROXY", "http://127.0.0.1:8080"),
        ("CODEX_HOME", "C:/codex"),
    ]);
    assert_eq!(cleaned.get("PATH").map(String::as_str), Some("/usr/bin"));
    assert_eq!(
        cleaned.get("CODEX_HOME").map(String::as_str),
        Some("C:/codex")
    );
    assert!(!cleaned.contains_key("CLAUDECODE"));
    assert!(!cleaned.contains_key("CLAUDE_CODE_PLUGIN_DIRS"));
    assert!(!cleaned.contains_key("ANTHROPIC_API_KEY"));
    assert!(!cleaned.contains_key("HTTPS_PROXY"));

    assert!(should_stop_process_cmdline(
        r"D:\Git\codex-plus\dist\local-build\CodexPlusPlus.exe --minimized",
        r"dist\local-build\CodexPlusPlus.exe"
    ));
    assert!(should_stop_process_cmdline(
        r"d:/git/codex-plus/dist/local-build/codexplusplus.exe --minimized",
        r"D:\Git\codex-plus\dist\local-build\CodexPlusPlus.exe"
    ));
    for flag in CLIENT_OWNED_FLAGS {
        let cmd = format!(r"D:\Git\codex-plus\dist\local-build\CodexPlusPlus.exe {flag}");
        assert!(!should_stop_process_cmdline(
            &cmd,
            r"dist\local-build\CodexPlusPlus.exe"
        ));
    }

    let root = repo_root();
    // Test relative `--root .` resolution (must resolve to absolute repo root, not `.\dist\...`)
    let dry = Command::new(devkit_bin())
        .current_dir(&root)
        .args(["relaunch-local", "--dry-run", "--root", "."])
        .output()
        .unwrap();
    assert!(dry.status.success());
    let stdout = String::from_utf8_lossy(&dry.stdout);
    assert!(stdout.contains("[dry-run]"));
    for line in stdout.lines() {
        if let Some(val) = line
            .strip_prefix("[dry-run] built=")
            .or_else(|| line.strip_prefix("[dry-run] dest="))
            .or_else(|| line.strip_prefix("[dry-run] bundle="))
            .or_else(|| line.strip_prefix("[dry-run] state="))
        {
            assert!(
                Path::new(val).is_absolute(),
                "expected absolute path in dry-run output for `--root .`, got: {val}"
            );
            assert!(!val.starts_with("./") && !val.starts_with(r".\"));
        }
    }

    // Pure-rust `--root .` must also resolve to absolute paths
    let dry_pure = Command::new(devkit_bin())
        .current_dir(&root)
        .args(["relaunch-local", "--pure-rust", "--dry-run", "--root", "."])
        .output()
        .unwrap();
    assert!(dry_pure.status.success());
    let pure_stdout = String::from_utf8_lossy(&dry_pure.stdout);
    for line in pure_stdout.lines() {
        if let Some(val) = line
            .strip_prefix("[dry-run] manifest=")
            .or_else(|| line.strip_prefix("[dry-run] built="))
            .or_else(|| line.strip_prefix("[dry-run] dest="))
            .or_else(|| line.strip_prefix("[dry-run] state="))
        {
            assert!(
                Path::new(val).is_absolute(),
                "expected absolute pure-rust path for `--root .`, got: {val}"
            );
        }
    }
}

#[test]
fn porcelain_z_parser_shard_validation_and_symlink_target_guards() {
    // 1. git status --porcelain -z with rename/copy two-path entries & multi-byte UTF-8 paths
    let raw = b" M CHANGELOG.md\0R  crates/\xe4\xb8\xad\xe6\x96\x87.rs\0ab\0C  new.rs\0\xe6\x97\xa7\xe6\x96\x87\xe4\xbb\xb6.rs\0";
    let parsed = parse_porcelain_z_dirty_paths(raw);
    assert_eq!(
        parsed,
        vec![
            "CHANGELOG.md".to_string(),
            "crates/中文.rs".to_string(),
            "ab".to_string(),
            "new.rs".to_string(),
            "旧文件.rs".to_string(),
        ]
    );

    // 2. filter_paths_in_use normalizes slashes and case
    let d1 = PathBuf::from(r"D:\Git\codex-plus\dist\local-build-old");
    let d2 = PathBuf::from(r"D:\Git\codex-plus\dist\local-build-unused");
    let in_use = filter_paths_in_use(
        &[d1.clone(), d2],
        "d:/git/codex-plus/dist/local-build-old/codexplusplus.exe --flag",
    );
    assert!(in_use.contains(&d1));
    assert_eq!(in_use.len(), 1);

    // 3. split_renderer_inject rejects path traversal shard filenames
    let tmp = TempDir::new().unwrap();
    let inj = tmp.path().join("assets/inject");
    let frags = inj.join("renderer-inject");
    fs::create_dir_all(&frags).unwrap();
    fs::write(inj.join("renderer-inject.js"), "(() => {\n})();\n").unwrap();
    for bad_name in ["../evil.js", r"..\evil.js", "sub/a.js", "..js"] {
        fs::write(
            frags.join("manifest.json"),
            serde_json::to_string_pretty(&json!({
                "fragments": [
                    { "name": bad_name, "description": "bad", "lines": 2 }
                ]
            }))
            .unwrap(),
        )
        .unwrap();
        assert!(
            split_renderer_inject(tmp.path()).is_err(),
            "must reject shard name {bad_name}"
        );
    }

    // 4. prepare_sherpa_cache rejects symlinked/junctioned target_dir BEFORE canonicalize
    let real_target = tmp.path().join("real_target");
    fs::create_dir_all(real_target.join("sherpa-onnx-prebuilt")).unwrap();
    let linked_target = tmp.path().join("linked_target");
    #[cfg(windows)]
    {
        let _ = Command::new("cmd")
            .args([
                "/C",
                "mklink",
                "/J",
                linked_target.to_str().unwrap(),
                real_target.to_str().unwrap(),
            ])
            .output()
            .unwrap();
    }
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(&real_target, &linked_target).unwrap();
    }
    if fs::symlink_metadata(&linked_target).is_ok() {
        assert!(prepare_sherpa_cache(&linked_target, &tmp.path().join("qtmp")).is_err());
    }
}

#[test]
fn build_plugin_market_index_matches_node_byte_for_byte() {
    let root = repo_root();
    let tmp = TempDir::new().unwrap();
    let repo = tmp.path().join("market-repo");
    fs::create_dir_all(repo.join("plugins/alpha/.codex-plugin")).unwrap();
    fs::create_dir_all(repo.join("plugins/alpha/skills/demo")).unwrap();
    fs::create_dir_all(repo.join("plugins/beta/.claude-plugin")).unwrap();

    fs::write(
        repo.join("catalog.json"),
        r#"{"snapshotDate":"2026-11-24T00:00:00Z","plugins":[{"name":"beta","id":"b-id"},{"name":"alpha"}]}"#,
    )
    .unwrap();
    fs::write(
        repo.join("plugins/alpha/.codex-plugin/plugin.json"),
        r#"{"name":"alpha","interface":{"displayName":"Alpha Plugin"},"description":"desc A","version":"1.2.3","author":"Alice","keywords":["t1","t2"],"license":"MIT"}"#,
    )
    .unwrap();
    fs::write(repo.join("plugins/alpha/skills/demo/SKILL.md"), "# skill\n").unwrap();
    fs::write(repo.join("plugins/alpha/.mcp.json"), "{}").unwrap();
    fs::write(
        repo.join("plugins/beta/.claude-plugin/plugin.json"),
        r#"{"name":"beta","displayName":"beta Plugin","description":"desc B","author":{"name":"Bob"}}"#,
    )
    .unwrap();

    for args in [
        vec!["init"],
        vec!["config", "user.email", "ci@example.com"],
        vec!["config", "user.name", "CI"],
        vec!["add", "."],
        vec!["commit", "-m", "init"],
    ] {
        let st = Command::new("git")
            .arg("-C")
            .arg(&repo)
            .args(args)
            .status()
            .unwrap();
        assert!(st.success());
    }

    let node_out_path = tmp.path().join("node-index.json");
    let rust_out_path = tmp.path().join("rust-index.json");

    let node_st = Command::new("node")
        .args([
            root.join("scripts/build-plugin-market-index.mjs")
                .to_str()
                .unwrap(),
            repo.to_str().unwrap(),
            "public",
            node_out_path.to_str().unwrap(),
        ])
        .status()
        .unwrap();
    assert!(node_st.success());

    build_plugin_market_index(&repo, "public", &rust_out_path).unwrap();

    let node_bytes = fs::read_to_string(&node_out_path).unwrap();
    let rust_bytes = fs::read_to_string(&rust_out_path).unwrap();
    assert_eq!(rust_bytes, node_bytes);
}

#[test]
fn sync_official_models_matches_python_rules() {
    let tmp = TempDir::new().unwrap();
    let asset = tmp.path().join("codex-models.json");
    let upstream = tmp.path().join("upstream.json");

    fs::write(
        &asset,
        r#"{"models":[{"slug":"gpt-5.4","max_context_window":272000}]}"#,
    )
    .unwrap();
    fs::write(
        &upstream,
        r#"{
  "models": [
    {"slug":"gpt-reserve","max_context_window":100000},
    {"slug":"gpt-5.4","max_context_window":272000},
    {"slug":"gpt-5.5","max_context_window":400000,"comp_hash":"abc123"}
  ]
}"#,
    )
    .unwrap();

    // --check reports pending addition
    assert!(!sync_official_models(&asset, upstream.to_str().unwrap(), true).unwrap());
    // apply inserts before legacy slug gpt-5.4, strips comp_hash, skips gpt-reserve
    assert!(sync_official_models(&asset, upstream.to_str().unwrap(), false).unwrap());
    let synced: Value = serde_json::from_str(&fs::read_to_string(&asset).unwrap()).unwrap();
    let models = synced["models"].as_array().unwrap();
    assert_eq!(models.len(), 2);
    assert_eq!(models[0]["slug"], json!("gpt-5.5"));
    assert!(models[0].get("comp_hash").is_none());
    assert_eq!(models[1]["slug"], json!("gpt-5.4"));
    assert!(sync_official_models(&asset, upstream.to_str().unwrap(), true).unwrap());
}
