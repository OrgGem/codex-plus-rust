use codex_plus_app::security::{
    STRICT_WEBVIEW_CSP, atomic_write_unique, classify_extension_risks,
    is_renderer_bridge_route_permitted, is_safe_webview_navigation, open_sqlite_safe,
    parse_sha256sums, project_public_settings, validate_session_deletion_paths,
    validate_session_deletion_with_db_records, verify_installer_sha256, verify_release_asset_url,
    with_immediate_transaction,
};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::fs;
use std::net::TcpListener;
use std::process::Command;
use tempfile::TempDir;

fn free_loopback_port() -> u16 {
    TcpListener::bind("127.0.0.1:0")
        .and_then(|l| l.local_addr())
        .map(|a| a.port())
        .unwrap_or(63150)
}

#[test]
fn c1_updater_url_allowlist_sha256_and_navigation_csp_are_strict() {
    assert!(
        verify_release_asset_url(
            "https://github.com/BigPizzaV3/CodexPlusPlus/releases/download/v2.8.0/CodexPlusPlus.exe"
        )
        .is_ok()
    );
    assert!(
        verify_release_asset_url(
            "https://github.com/gemvn90/CodexPlusPlus/releases/download/v2.8.0/CodexPlusPlus.exe"
        )
        .is_ok()
    );
    assert!(
        verify_release_asset_url("https://objects.githubusercontent.com/abc/setup.exe").is_ok()
    );
    // Reject arbitrary repositories on github.com, userinfo, custom ports, and HTTP
    assert!(
        verify_release_asset_url(
            "https://github.com/attacker/malware/releases/download/v1/evil.exe"
        )
        .is_err()
    );
    assert!(
        verify_release_asset_url(
            "https://user:pass@github.com/BigPizzaV3/CodexPlusPlus/releases/download/v2.8.0/a.exe"
        )
        .is_err()
    );
    assert!(
        verify_release_asset_url(
            "https://github.com:8443/BigPizzaV3/CodexPlusPlus/releases/download/v2.8.0/a.exe"
        )
        .is_err()
    );
    assert!(verify_release_asset_url("http://github.com/evil.exe").is_err());
    assert!(verify_release_asset_url("https://evil.example.com/CodexPlusPlus.exe").is_err());
    assert!(verify_release_asset_url("file:///C:/Windows/System32/calc.exe").is_err());

    let payload = b"codexplusplus-verified-installer-binary";
    let hex = Sha256::digest(payload)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect::<String>();
    let sums = parse_sha256sums(&format!("{hex}  CodexPlusPlus_2.8.0_x64-setup.exe\n"));
    assert_eq!(
        sums.get("CodexPlusPlus_2.8.0_x64-setup.exe")
            .map(String::as_str),
        Some(hex.as_str())
    );
    assert!(verify_installer_sha256(payload, &hex).is_ok());
    assert!(verify_installer_sha256(b"tampered", &hex).is_err());
    assert!(verify_installer_sha256(b"", &hex).is_err());

    assert!(is_safe_webview_navigation("about:blank"));
    assert!(is_safe_webview_navigation("dioxus://index.html"));
    assert!(is_safe_webview_navigation("http://dioxus.index.html/"));
    // Top-level navigation to arbitrary loopback servers or ipc.localhost must be blocked
    assert!(!is_safe_webview_navigation(
        "http://127.0.0.1:3000/untrusted"
    ));
    assert!(!is_safe_webview_navigation("http://ipc.localhost/"));
    assert!(!is_safe_webview_navigation(
        "https://untrusted.example.com/"
    ));
    assert!(STRICT_WEBVIEW_CSP.contains("frame-src 'none'"));
    assert!(STRICT_WEBVIEW_CSP.contains("form-action 'none'"));
}

#[test]
fn c2_public_settings_dto_redacts_all_credentials() {
    let raw = json!({
        "revision": "rev-1",
        "activeRelayId": "r1",
        "apiKeys": [{ "id": "k1", "apiKey": "SECRET-KEY-LIST" }],
        "relayApiKey": "SECRET-GATEWAY-KEY",
        "codexAppStepwiseApiKey": "SECRET-STEPWISE",
        "weixinConnectToken": "SECRET-WEIXIN-TOKEN",
        "dictation": {
            "enabled": true,
            "provider": "openai",
            "model": "whisper-1",
            "baseUrl": "https://user:SECRET-DICT-URL@api.example.com/v1?api_key=SECRET-DICT-Q",
            "apiKey": "SECRET-DICTATION-KEY"
        },
        "relayProfiles": [{
            "id": "r1",
            "name": "Primary",
            "baseUrl": "https://user:SECRET-URL-PASS@relay.example.com/v1?key=SECRET-URL-KEY",
            "upstreamBaseUrl": "https://relay.example.com/v1?token=SECRET-UPSTREAM-TOKEN",
            "model": "gpt-5.4",
            "apiKeys": [{ "id": "rk1", "name": "main", "apiKey": "SECRET-PROFILE-KEY-ARRAY" }],
            "vlmApiKey": "SECRET-VLM-KEY",
            "authContents": "{\"tokens\":{\"refresh_token\":\"SECRET-OAUTH-REFRESH\"}}",
            "customHeaders": [
                { "key": "Authorization", "value": "Bearer SECRET-BEARER" },
                { "name": "X-Proxy-Password", "value": "SECRET-PROXY-PASS" },
                { "key": "X-Custom", "value": "Bearer SECRET-VALUE-BEARER", "extraLeak": "SECRET-EXTRA" },
                { "key": "X-Client-Tag", "value": "public-safe-tag", "extraLeak": "SECRET-EXTRA-2" }
            ]
        }],
        "tools": {
            "claude": {
                "activeRelayId": "c1",
                "relayProfiles": [{
                    "id": "c1",
                    "name": "Claude Relay",
                    "apiKey": "SECRET-TOOL-RELAY",
                    "authContents": "{\"access_token\":\"SECRET-TOOL-ACCESS\"}"
                }]
            }
        }
    });

    let dto = project_public_settings(&raw);
    let dumped = dto.to_string();
    assert!(
        !dumped.contains("SECRET-"),
        "Secret leaked in DTO: {dumped}"
    );
    assert_eq!(dto["activeRelayId"], json!("r1"));
    assert_eq!(dto["relayProfiles"][0]["hasApiKey"], json!(true));
    assert_eq!(dto["relayProfiles"][0]["hasAuthContents"], json!(true));
    assert_eq!(
        dto["relayProfiles"][0]["baseUrl"],
        json!("https://relay.example.com/v1")
    );
    assert_eq!(
        dto["relayProfiles"][0]["customHeaders"][0]["redacted"],
        json!(true)
    );
    assert_eq!(
        dto["relayProfiles"][0]["customHeaders"][1]["redacted"],
        json!(true)
    );
    assert_eq!(
        dto["relayProfiles"][0]["customHeaders"][2]["redacted"],
        json!(true)
    );
    assert_eq!(
        dto["relayProfiles"][0]["customHeaders"][3]["value"],
        json!("public-safe-tag")
    );
    assert_eq!(dto["hasWeixinConnectToken"], json!(true));
    assert_eq!(dto["dictation"]["hasApiKey"], json!(true));
}

#[test]
fn c3_renderer_bridge_routes_and_extension_risks_enforced() {
    assert!(is_renderer_bridge_route_permitted("/backend/status"));
    assert!(is_renderer_bridge_route_permitted("/settings/public"));
    assert!(!is_renderer_bridge_route_permitted("/hooks/test-run"));
    assert!(!is_renderer_bridge_route_permitted("/hooks/file/write"));
    assert!(!is_renderer_bridge_route_permitted("/settings/set"));
    assert!(!is_renderer_bridge_route_permitted("/delete"));

    let risks = classify_extension_risks(&json!({
        "renderer": "index.js",
        "routes": ["/backend/status", "/hooks/test-run"]
    }));
    assert!(risks.iter().any(|r| r.contains("main-world-renderer")));
    assert!(risks.iter().any(|r| r.contains("/hooks/test-run")));
}

#[test]
fn c4_session_deletion_path_containment_blocks_caller_and_db_escape() {
    let dir = TempDir::new().expect("temp dir");
    let home = dir.path().join("codex-home");
    let sessions_dir = home.join("sessions");
    let archived_dir = home.join("archived_sessions");
    fs::create_dir_all(&sessions_dir).expect("sessions dir");
    fs::create_dir_all(&archived_dir).expect("archived dir");

    let allowed_db = home.join("state_5.sqlite");
    let mut conn = open_sqlite_safe(&allowed_db, false).expect("create state_5.sqlite");
    let valid_rollout = sessions_dir.join("rollout-1.jsonl");
    fs::write(&valid_rollout, b"{}").expect("valid rollout");
    let outside_secret = dir.path().join("secret.txt");
    fs::write(&outside_secret, b"do-not-delete").expect("secret file");
    let outside_jsonl = dir.path().join("outside.jsonl");
    fs::write(&outside_jsonl, b"{}").expect("outside jsonl");
    let orphan_rollout = sessions_dir.join("already-deleted.jsonl");

    with_immediate_transaction(&mut conn, |tx| {
        tx.execute(
            "CREATE TABLE threads (id TEXT PRIMARY KEY, rollout_path TEXT)",
            [],
        )?;
        tx.execute(
            "INSERT INTO threads (id, rollout_path) VALUES ('s-valid', ?1)",
            [valid_rollout.to_string_lossy().as_ref()],
        )?;
        tx.execute(
            "INSERT INTO threads (id, rollout_path) VALUES ('s-evil', ?1)",
            [outside_jsonl.to_string_lossy().as_ref()],
        )?;
        tx.execute(
            "INSERT INTO threads (id, rollout_path) VALUES ('s-orphan', ?1)",
            [orphan_rollout.to_string_lossy().as_ref()],
        )?;
        Ok(())
    })
    .expect("seed threads");
    drop(conn);

    let evil_db = dir.path().join("other.sqlite");
    fs::write(&evil_db, b"").expect("evil db");

    assert!(validate_session_deletion_paths(&home, &allowed_db, Some(&valid_rollout)).is_ok());
    assert!(validate_session_deletion_paths(&home, &evil_db, Some(&valid_rollout)).is_err());
    assert!(validate_session_deletion_paths(&home, &allowed_db, Some(&outside_secret)).is_err());

    // Authoritative SQLite `threads.rollout_path` validation (even when caller passes `None`!):
    assert!(validate_session_deletion_with_db_records(&home, &allowed_db, "s-valid", None).is_ok());
    assert!(
        validate_session_deletion_with_db_records(&home, &allowed_db, "s-evil", None).is_err(),
        "must block unvalidated threads.rollout_path pointing outside sessions/"
    );
    assert!(
        validate_session_deletion_with_db_records(
            &home,
            &allowed_db,
            "s-orphan",
            Some(&orphan_rollout)
        )
        .is_ok(),
        "must allow cleanup of orphaned DB entry whose .jsonl was already deleted"
    );
}

#[test]
fn high_sqlite_and_atomic_write_are_race_and_lock_safe() {
    let dir = TempDir::new().expect("temp dir");
    let missing = dir.path().join("never-created.sqlite");
    assert!(open_sqlite_safe(&missing, true).is_err());
    assert!(!missing.exists());

    let db_path = dir.path().join("test.sqlite");
    let mut conn = open_sqlite_safe(&db_path, false).expect("create rw db");
    with_immediate_transaction(&mut conn, |tx| {
        tx.execute("CREATE TABLE kv (k TEXT PRIMARY KEY, v TEXT NOT NULL)", [])?;
        tx.execute("INSERT INTO kv (k, v) VALUES ('a', '1')", [])?;
        Ok(())
    })
    .expect("immediate tx");

    let target = dir.path().join("atomic.json");
    let mut handles = Vec::new();
    for i in 0..8u32 {
        let p = target.clone();
        handles.push(std::thread::spawn(move || {
            for j in 0..10u32 {
                let data = format!("{{\"thread\":{i},\"iter\":{j}}}");
                atomic_write_unique(&p, data.as_bytes()).expect("atomic write");
            }
        }));
    }
    for h in handles {
        h.join().expect("thread join");
    }
    let final_str = fs::read_to_string(&target).expect("read final");
    let parsed: serde_json::Value = serde_json::from_str(&final_str).expect("valid non-torn JSON");
    assert!(parsed.get("thread").is_some());
}

#[test]
fn smoke_check_cli_starts_live_gateway_and_probes_codex() {
    let dir = TempDir::new().expect("smoke temp");
    let isolated_home = dir.path().join("codex-home");
    fs::create_dir_all(&isolated_home).expect("isolated codex home");
    let mgr_port = free_loopback_port();
    let lch_port = free_loopback_port();
    let prx_port = free_loopback_port();

    let bin = env!("CARGO_BIN_EXE_codex-plus-app");
    let out = Command::new(bin)
        .arg("--smoke-check")
        .env("CODEX_HOME", &isolated_home)
        .env("CODEX_PLUS_STATE_DIR", dir.path())
        .env("CODEX_PLUS_MANAGER_GUARD_PORT", mgr_port.to_string())
        .env("CODEX_PLUS_LAUNCHER_GUARD_PORT", lch_port.to_string())
        .env("CODEX_PLUS_PROXY_PORT", prx_port.to_string())
        .output()
        .expect("run codex-plus-app --smoke-check");
    if !out.status.success() {
        panic!(
            "smoke-check exited {}:\nstdout: {}\nstderr: {}",
            out.status,
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
    }
    let report: serde_json::Value =
        serde_json::from_slice(&out.stdout).expect("smoke-check stdout JSON");
    assert_eq!(report["ok"], json!(true));
    assert_eq!(report["backendStatusHttp"], json!(200));
    assert_eq!(report["proxyStatusHttp"], json!(200));
    assert_eq!(report["c2PublicDtoClean"], json!(true));
    assert_eq!(report["c3BlockedDangerousRoute"], json!(true));
    assert_eq!(report["presetCount"], json!(23));
}
