use claude_patcher_rs::asar::{extract_blob, get_node, pack_files, read_asar};
use claude_patcher_rs::build_copy::{
    LANG_NEW, LANG_OLD, build_copy, check_copy, install_ion_dir, localize_ion_asar, patch_app_asar,
    verify_ion_asar,
};
use claude_patcher_rs::fuses::{
    WIRE, check_fuses_unlocked, find_fuse_sites_in_bytes, patch_electron_fuses, root_for,
};
use claude_patcher_rs::inject_cn::{
    InjectConfig, build_bootstrap_script, build_hook_script, inject_into_ws, run_inject,
    wait_for_node_target,
};
use claude_patcher_rs::sign_copy::{
    MACHO_MAGICS, collect_sign_targets, is_macho_magic, sanitize_entitlements_xml, verify_succeeded,
};
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;
use tempfile::TempDir;
use tungstenite::Message;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(3)
        .expect("repo root")
        .to_path_buf()
}

fn write_catalogs(dir: &Path) {
    fs::create_dir_all(dir.join("dynamic")).unwrap();
    fs::write(dir.join("zh-Hans.json"), r#"{"hello":"你好"}"#.as_bytes()).unwrap();
    fs::write(
        dir.join("dynamic/zh-Hans.json"),
        r#"{"dyn":"动态"}"#.as_bytes(),
    )
    .unwrap();
    fs::write(dir.join("zh-Hans.overrides.json"), br#"{}"#).unwrap();
}

#[test]
fn differential_build_copy_matches_python_script() {
    let root = repo_root();
    let py_script = root.join("assets/claude-localization/build-copy.py");
    let tmp = TempDir::new().unwrap();

    // Create identical synthetic Electron app dirs for Python and Rust.
    let catalogs = tmp.path().join("catalogs");
    write_catalogs(&catalogs);

    let mut fake_exe = vec![0u8; 64];
    fake_exe.extend_from_slice(WIRE);
    fake_exe.push(0x00);
    fake_exe.push(6); // version = 6 flags
    fake_exe.extend_from_slice(b"000010");
    fake_exe.extend_from_slice(&[0u8; 32]);

    let mut ion_entries = BTreeMap::new();
    ion_entries.insert(
        "assets/main.js".to_string(),
        format!("const langs = {LANG_OLD};").into_bytes(),
    );
    ion_entries.insert(
        "i18n/en-US.json".to_string(),
        br#"{"hello":"hello"}"#.to_vec(),
    );
    ion_entries.insert(
        "index.html".to_string(),
        b"<!doctype html><html><head><title>Claude</title></head><body></body></html>".to_vec(),
    );
    let ion_asar_bytes = pack_files(&ion_entries).unwrap();

    let mut app_entries = BTreeMap::new();
    app_entries.insert(
        ".vite/build/index.js".to_string(),
        br#"const CU_CFG="1291166712";function check(c){return checkGate("chicagoEnabled")&&gate(CU_CFG,"enabled",false)&&gate(CU_CFG,"appScopedEnabled",!1);}"#.to_vec(),
    );
    let app_asar_bytes = pack_files(&app_entries).unwrap();

    for sub in ["py_app", "rs_app"] {
        let app = tmp.path().join(sub);
        fs::create_dir_all(app.join("resources")).unwrap();
        fs::write(app.join("Claude.exe"), &fake_exe).unwrap();
        fs::write(app.join("resources/ion-dist.asar"), &ion_asar_bytes).unwrap();
        fs::write(app.join("resources/app.asar"), &app_asar_bytes).unwrap();
    }

    let py_app = tmp.path().join("py_app");
    let rs_app = tmp.path().join("rs_app");

    // 1. Run real Python `build-copy.py` functions on `py_app`
    let py_code = r#"
import importlib.util, pathlib, sys
spec = importlib.util.spec_from_file_location("bc", sys.argv[1])
mod = importlib.util.module_from_spec(spec)
spec.loader.exec_module(mod)
app = pathlib.Path(sys.argv[2])
cats = pathlib.Path(sys.argv[3])
for b, off, _ in mod.fuses(app):
    with b.open("r+b") as f:
        f.seek(off + 3); f.write(b"1"); f.seek(off + 4); f.write(b"0")
mod.localize_ion_asar(app / "resources/ion-dist.asar", cats)
mod.verify_ion_asar(app / "resources/ion-dist.asar")
mod.patch_asar(app)
"#;
    let py_out = Command::new("python")
        .args([
            "-c",
            py_code,
            py_script.to_str().unwrap(),
            py_app.to_str().unwrap(),
            catalogs.to_str().unwrap(),
        ])
        .output()
        .expect("run python build-copy.py");
    assert!(
        py_out.status.success(),
        "python failed:\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&py_out.stdout),
        String::from_utf8_lossy(&py_out.stderr)
    );

    // 2. Run Rust equivalents on `rs_app`
    let _ = patch_electron_fuses(&rs_app, true).unwrap();
    check_fuses_unlocked(&rs_app, true).unwrap();
    localize_ion_asar(&rs_app.join("resources/ion-dist.asar"), &catalogs).unwrap();
    verify_ion_asar(&rs_app.join("resources/ion-dist.asar")).unwrap();
    let changed = patch_app_asar(&rs_app).unwrap();
    assert!(!changed.is_empty());

    // 3. Compare patched Electron exe byte-for-byte
    assert_eq!(
        fs::read(py_app.join("Claude.exe")).unwrap(),
        fs::read(rs_app.join("Claude.exe")).unwrap(),
        "Electron fuse binary bytes must match Python byte-for-byte"
    );

    // 4. Compare packed ion-dist.asar contents & integrity hashes
    let py_ion = read_asar(&py_app.join("resources/ion-dist.asar")).unwrap();
    let rs_ion = read_asar(&rs_app.join("resources/ion-dist.asar")).unwrap();
    for rel in [
        "assets/main.js",
        "index.html",
        "i18n/zh-Hans.json",
        "i18n/dynamic/zh-Hans.json",
        "i18n/zh-Hans.overrides.json",
        "i18n/en-US.json",
    ] {
        let pm = get_node(&py_ion.header, rel).unwrap();
        let rm = get_node(&rs_ion.header, rel).unwrap();
        assert_eq!(
            extract_blob(&py_ion.data, py_ion.payload_start, pm).unwrap(),
            extract_blob(&rs_ion.data, rs_ion.payload_start, rm).unwrap(),
            "ASAR blob mismatch on {rel}"
        );
        assert_eq!(
            pm["integrity"], rm["integrity"],
            "ASAR integrity mismatch on {rel}"
        );
    }

    // 5. Cross-verify Rust's ion-dist.asar with Python's verify_ion_asar!
    let py_verify = Command::new("python")
        .args([
            "-c",
            "import importlib.util, pathlib, sys; spec = importlib.util.spec_from_file_location('bc', sys.argv[1]); mod = importlib.util.module_from_spec(spec); spec.loader.exec_module(mod); mod.verify_ion_asar(pathlib.Path(sys.argv[2]))",
            py_script.to_str().unwrap(),
            rs_app.join("resources/ion-dist.asar").to_str().unwrap(),
        ])
        .status()
        .unwrap();
    assert!(
        py_verify.success(),
        "Python verify_ion_asar must accept Rust-packed ASAR"
    );

    // 6. Compare CU-patched app.asar blob byte-for-byte
    let py_cu = read_asar(&py_app.join("resources/app.asar")).unwrap();
    let rs_cu = read_asar(&rs_app.join("resources/app.asar")).unwrap();
    let pm = get_node(&py_cu.header, ".vite/build/index.js").unwrap();
    let rm = get_node(&rs_cu.header, ".vite/build/index.js").unwrap();
    let py_js = extract_blob(&py_cu.data, py_cu.payload_start, pm).unwrap();
    let rs_js = extract_blob(&rs_cu.data, rs_cu.payload_start, rm).unwrap();
    assert_eq!(py_js, rs_js);
    assert_eq!(
        std::str::from_utf8(rs_js).unwrap(),
        r#"const CU_CFG="1291166712";function check(c){return !0&&!0&&!0;}"#
    );

    // 7. Test check_copy mode passes on CU-patched copy
    check_copy(&py_app, &rs_app, true).unwrap();

    // 8. Test directory-mode ion-dist writes UTF-8 without BOM (known-diffs KNOWN-DIFF-002)
    let dir_ion = tmp.path().join("ion_dir");
    fs::create_dir_all(dir_ion.join("assets")).unwrap();
    fs::create_dir_all(dir_ion.join("i18n")).unwrap();
    fs::write(
        dir_ion.join("assets/app.js"),
        format!("const l={LANG_OLD};"),
    )
    .unwrap();
    fs::write(dir_ion.join("i18n/en-US.json"), "{}").unwrap();
    fs::write(dir_ion.join("index.html"), "<html><head></head></html>").unwrap();
    install_ion_dir(&dir_ion, &catalogs).unwrap();
    let js_bytes = fs::read(dir_ion.join("assets/app.js")).unwrap();
    let html_bytes = fs::read(dir_ion.join("index.html")).unwrap();
    assert!(!js_bytes.starts_with(&[0xEF, 0xBB, 0xBF]));
    assert!(!html_bytes.starts_with(&[0xEF, 0xBB, 0xBF]));
    assert!(std::str::from_utf8(&js_bytes).unwrap().contains(LANG_NEW));
}

#[test]
fn sign_copy_macho_entitlements_and_untrusted_only_fix() {
    for m in MACHO_MAGICS {
        assert!(is_macho_magic(m));
    }
    assert!(!is_macho_magic(b"MZ\x90\x00"));

    // Known-diff KNOWN-DIFF-001: self-signed untrusted_only must succeed when ok == total.
    assert!(verify_succeeded(5, 5, 1, "CSSMERR_TP_NOT_TRUSTED"));
    assert!(!verify_succeeded(
        5,
        5,
        1,
        "a sealed resource is missing or invalid"
    ));
    assert!(!verify_succeeded(4, 5, 0, ""));

    // Entitlement stripping
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<plist version="1.0">
<dict>
	<key>com.apple.application-identifier</key>
	<string>TEAM.com.anthropic.claudefordesktop</string>
	<key>com.apple.developer.team-identifier</key>
	<string>TEAM123</string>
	<key>keychain-access-groups</key>
	<array><string>group</string></array>
	<key>com.apple.security.cs.allow-jit</key>
	<true/>
</dict>
</plist>"#;
    let cleaned = sanitize_entitlements_xml(xml);
    assert!(!cleaned.contains("com.apple.application-identifier"));
    assert!(!cleaned.contains("com.apple.developer.team-identifier"));
    assert!(!cleaned.contains("keychain-access-groups"));
    assert!(cleaned.contains("com.apple.security.cs.allow-jit"));
    assert!(cleaned.contains("com.apple.security.cs.disable-library-validation"));

    // Mach-O walk depth ordering
    let tmp = TempDir::new().unwrap();
    let contents = tmp.path().join("Claude.app/Contents");
    let nested_fw = contents.join("Frameworks/A.framework/Versions/A");
    fs::create_dir_all(&nested_fw).unwrap();
    fs::write(
        contents.join("MacOS_bin"),
        [0xcf, 0xfa, 0xed, 0xfe, 0, 0, 0, 0],
    )
    .unwrap();
    fs::write(nested_fw.join("A"), [0xfe, 0xed, 0xfa, 0xcf, 0, 0, 0, 0]).unwrap();
    fs::write(contents.join("readme.txt"), b"not macho").unwrap();

    let targets = collect_sign_targets(&contents).unwrap();
    assert_eq!(targets.frameworks.len(), 1);
    assert_eq!(targets.loose.len(), 2);
    assert!(targets.loose[0].ends_with("A"));
}

#[test]
fn mock_cdp_websocket_server_e2e_for_inject_cn() {
    let tmp = TempDir::new().unwrap();
    let fake_exe = tmp.path().join(if cfg!(windows) {
        "Claude.exe"
    } else {
        "Claude"
    });
    fs::write(&fake_exe, b"fake").unwrap();
    let wrong_exe = tmp
        .path()
        .join(if cfg!(windows) { "Other.exe" } else { "Other" });
    fs::write(&wrong_exe, b"other").unwrap();
    let hook = tmp.path().join("hook.js");
    fs::write(&hook, b"// hook").unwrap();

    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let served_exec = fake_exe.canonicalize().unwrap();

    let server = thread::spawn(move || {
        // Handle 1 HTTP `/json/list` request, then 1 WebSocket session (matching exec),
        // then 1 WebSocket session (mismatched exec test).
        let mut saw_hook = false;
        let mut saw_close = false;
        for round in 0..3 {
            let (mut stream, _) = listener.accept().unwrap();
            let mut peek = [0u8; 512];
            let n = stream.peek(&mut peek).unwrap();
            let head = String::from_utf8_lossy(&peek[..n]);
            if head.starts_with("GET /json/list ") {
                let mut sink = [0u8; 1024];
                let _ = stream.read(&mut sink);
                let body = json!([{
                    "type": "node",
                    "webSocketDebuggerUrl": format!("ws://127.0.0.1:{port}/devtools/node")
                }])
                .to_string();
                let resp = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    body.len(),
                    body
                );
                stream.write_all(resp.as_bytes()).unwrap();
            } else {
                let mut ws = tungstenite::accept(stream).unwrap();
                while let Ok(msg) = ws.read() {
                    let Message::Text(txt) = msg else { continue };
                    let req: Value = serde_json::from_str(&txt).unwrap();
                    let id = req["id"].as_u64().unwrap();
                    let expr = req["params"]["expression"].as_str().unwrap_or("");
                    let val = if expr == "process.execPath" {
                        if round < 2 {
                            Value::String(served_exec.to_string_lossy().into_owned())
                        } else {
                            Value::String(wrong_exe.to_string_lossy().into_owned())
                        }
                    } else if expr.contains("beginTimeoutOverrides") {
                        json!(true)
                    } else if expr.contains("app.getVersion()") {
                        json!("1.0.0")
                    } else if expr == "process.pid" {
                        json!(4242)
                    } else if expr.contains("__claudeCnHookInstalled === true") {
                        saw_hook = true;
                        json!(true)
                    } else if expr == "globalThis.__claudeCnBashTimeoutReady" {
                        json!({"enabled": false, "prepared": true})
                    } else if expr.contains("node:inspector") {
                        saw_close = true;
                        json!("closing")
                    } else {
                        Value::Null
                    };
                    let reply = json!({ "id": id, "result": { "result": { "value": val } } });
                    let _ = ws.send(Message::Text(reply.to_string()));
                    if expr.contains("node:inspector") || (round == 2 && expr == "process.execPath")
                    {
                        break;
                    }
                }
            }
        }
        (saw_hook, saw_close)
    });

    let cfg = InjectConfig {
        app: fake_exe.clone(),
        hook,
        timeout_secs: 5,
        no_launch: true,
        attach: true,
        port: Some(port),
        user_data_dir: None,
        locale: "zh-Hans".to_string(),
        voice_input: true,
        localization: true,
        bash_timeout_json: "null".to_string(),
        bash_report_file: None,
        bash_launch_id: None,
    };

    // Round 0 + 1: HTTP discovery + matching `process.execPath` -> succeeds
    run_inject(&cfg).unwrap();

    // Round 2: Mismatched `process.execPath` -> refuses to evaluate hook
    let err = inject_into_ws(
        &format!("ws://127.0.0.1:{port}/devtools/node"),
        &cfg,
        &fake_exe,
    )
    .unwrap_err();
    assert!(err.to_string().contains("目标进程不是这个副本"));

    let (saw_hook, saw_close) = server.join().unwrap();
    assert!(saw_hook);
    assert!(saw_close);
}

#[test]
fn sanitize_entitlements_xml_preserves_keys_without_arrays_and_handles_nested_tags() {
    // Regression: when no <array> follows <string>, must NOT match root </dict> and wipe remaining keys!
    let xml_no_array = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
	<key>com.apple.application-identifier</key>
	<string>TEAM.com.anthropic.claudefordesktop</string>
	<key>com.apple.developer.team-identifier</key>
	<string>TEAM123</string>
	<key>com.apple.security.cs.allow-jit</key>
	<true/>
	<key>com.apple.security.network.client</key>
	<true/>
	<key>com.apple.security.cs.disable-library-validation</key>
	<false/>
	<key>custom-int</key>
	<integer>7</integer>
	<key>custom-data</key>
	<data>QUJD</data>
	<key>nested-dict</key>
	<dict>
		<key>keep-inner</key>
		<string>value</string>
	</dict>
</dict>
</plist>"#;
    let out = sanitize_entitlements_xml(xml_no_array);
    assert!(!out.contains("com.apple.application-identifier"));
    assert!(!out.contains("com.apple.developer.team-identifier"));
    assert!(out.contains("<key>com.apple.security.cs.allow-jit</key>"));
    assert!(out.contains("<key>com.apple.security.network.client</key>"));
    assert!(out.contains("<key>custom-int</key>\n\t<integer>7</integer>"));
    assert!(out.contains("<key>custom-data</key>\n\t<data>QUJD</data>"));
    assert!(out.contains("<key>nested-dict</key>"));
    assert!(out.contains("<key>keep-inner</key>"));
    assert!(out.contains("<key>com.apple.security.cs.disable-library-validation</key>\n\t<true/>"));
    assert!(!out.contains("<false/>"));

    // Balanced nested <array> inside keychain-access-groups must not leave trailing </array>
    let xml_nested_arr = r#"<plist version="1.0">
<dict>
	<key>keychain-access-groups</key>
	<array>
		<string>g1</string>
		<array>
			<string>g2</string>
		</array>
	</array>
	<key>after-array</key>
	<true/>
</dict>
</plist>"#;
    let out_arr = sanitize_entitlements_xml(xml_nested_arr);
    assert!(!out_arr.contains("keychain-access-groups"));
    assert!(!out_arr.contains("g2"));
    assert!(!out_arr.contains("</array>"));
    assert!(out_arr.contains("<key>after-array</key>"));
}

#[test]
fn windows_exe_root_for_readonly_msix_build_copy_and_extended_fuses() {
    // Non-existent Windows .exe target must resolve to parent directory, never treat .exe as dir!
    assert_eq!(
        root_for(Path::new(r"D:\stage\Claude.exe")),
        PathBuf::from(r"D:\stage")
    );
    assert_eq!(
        root_for(Path::new(r"D:\stage\CLAUDE.EXE")),
        PathBuf::from(r"D:\stage")
    );

    // 24-fuse wire with 'r' (removed) slots must be accepted by find_fuse_sites_in_bytes
    let mut exe_24 = vec![0u8; 32];
    exe_24.extend_from_slice(WIRE);
    exe_24.push(1);
    exe_24.push(24);
    exe_24.extend_from_slice(b"01r0100r11001100r1001100");
    exe_24.extend_from_slice(&[0u8; 16]);
    let sites = find_fuse_sites_in_bytes(Path::new("Claude.exe"), &exe_24);
    assert_eq!(sites.len(), 1);
    assert_eq!(sites[0].flags.len(), 24);

    // Missing source must report missing source before target check
    let tmp = TempDir::new().unwrap();
    let catalogs = tmp.path().join("catalogs");
    write_catalogs(&catalogs);
    let missing_err = build_copy(
        &tmp.path().join("does-not-exist/Claude.exe"),
        &tmp.path().join("out/Claude.exe"),
        &catalogs,
        false,
    )
    .unwrap_err();
    assert!(missing_err.to_string().contains("找不到官方 Claude 客户端"));

    // Simulate MSIX WindowsApps read-only files + `--copy <stage>/Claude.exe` where <stage> does not exist yet
    #[cfg(windows)]
    {
        let msix = tmp.path().join("msix_src");
        fs::create_dir_all(msix.join("resources")).unwrap();
        let src_exe = msix.join("Claude.exe");
        fs::write(&src_exe, &exe_24).unwrap();

        let mut ion_entries = BTreeMap::new();
        ion_entries.insert(
            "assets/main.js".to_string(),
            format!("const langs = {LANG_OLD};").into_bytes(),
        );
        ion_entries.insert("i18n/en-US.json".to_string(), br#"{"a":"b"}"#.to_vec());
        ion_entries.insert(
            "index.html".to_string(),
            b"<html><head></head><body></body></html>".to_vec(),
        );
        let src_ion = msix.join("resources/ion-dist.asar");
        fs::write(&src_ion, pack_files(&ion_entries).unwrap()).unwrap();

        let mut app_entries = BTreeMap::new();
        app_entries.insert(
            ".vite/build/index.js".to_string(),
            br#"const CU_CFG="1291166712";checkGate("chicagoEnabled");"#.to_vec(),
        );
        let src_app = msix.join("resources/app.asar");
        fs::write(&src_app, pack_files(&app_entries).unwrap()).unwrap();

        // Mark all MSIX files read-only
        for p in [&src_exe, &src_ion, &src_app] {
            let mut perm = fs::metadata(p).unwrap().permissions();
            perm.set_readonly(true);
            fs::set_permissions(p, perm).unwrap();
        }

        // Existing copy_root must be rejected even if Claude.exe inside it doesn't exist
        let pre_existing_dir = tmp.path().join("already_exists");
        fs::create_dir_all(&pre_existing_dir).unwrap();
        let dup_err = build_copy(
            &src_exe,
            &pre_existing_dir.join("Claude.exe"),
            &catalogs,
            true,
        )
        .unwrap_err();
        assert!(dup_err.to_string().contains("构建目标必须是新的暂存目录"));

        // Fresh `--copy <tmp>/fresh_stage/Claude.exe` succeeds and clears readonly
        let dst_exe = tmp.path().join("fresh_stage").join("Claude.exe");
        build_copy(&src_exe, &dst_exe, &catalogs, true).unwrap();
        assert!(dst_exe.is_file());
        assert!(!dst_exe.join("Claude.exe").exists());
        assert!(!fs::metadata(&dst_exe).unwrap().permissions().readonly());

        // Cleanup read-only on tmp source files so TempDir drops cleanly
        for p in [&src_exe, &src_ion, &src_app] {
            let mut perm = fs::metadata(p).unwrap().permissions();
            #[allow(clippy::permissions_set_readonly_false)]
            perm.set_readonly(false);
            fs::set_permissions(p, perm).unwrap();
        }
    }
}

#[test]
fn inject_scripts_resolve_absolute_paths_empty_timeout_and_early_child_exit() {
    let cfg = InjectConfig {
        app: PathBuf::from("Claude.exe"),
        hook: PathBuf::from("assets/claude-localization/hook.js"),
        timeout_secs: 5,
        no_launch: true,
        attach: true,
        port: Some(9411),
        user_data_dir: None,
        locale: "zh-Hans".into(),
        voice_input: true,
        localization: true,
        bash_timeout_json: String::new(),
        bash_report_file: None,
        bash_launch_id: None,
    };
    let boot = build_bootstrap_script(&cfg).unwrap();
    assert!(boot.contains(r#"JSON.parse("null")"#));
    assert!(!boot.contains(r"\\?\"));
    let hook_js = build_hook_script(&cfg).unwrap();
    assert!(!hook_js.contains(r#""assets/claude-localization/hook.js""#));
    assert!(!hook_js.contains(r"\\?\"));

    // Spawn a short-lived process that exits immediately -> wait_for_node_target must fail fast (< 2s)
    let port = claude_patcher_rs::inject_cn::free_port().unwrap();
    let mut dead_child = if cfg!(windows) {
        Command::new("cmd")
            .args(["/C", "exit", "0"])
            .spawn()
            .unwrap()
    } else {
        Command::new("sh").args(["-c", "exit 0"]).spawn().unwrap()
    };
    let start = std::time::Instant::now();
    let err = wait_for_node_target(
        port,
        std::time::Duration::from_secs(10),
        Some(&mut dead_child),
    )
    .unwrap_err();
    assert!(err.to_string().contains("提前退出"));
    assert!(start.elapsed() < std::time::Duration::from_secs(4));
}
