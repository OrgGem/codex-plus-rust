use serde_json::json;
use share_site_rs::{
    EncryptedPayload, InMemoryShareStore, ShareSiteConfig, ShareStore, WorkerRequest, decrypt_text,
    encrypt_text, handle_request, rollout_to_markdown, session_to_markdown,
};
use std::collections::HashMap;
use std::process::Command;

#[test]
fn cross_verify_aes_gcm_with_node_webcrypto() {
    // 1. Rust encrypts -> Node WebCrypto (using exact app.js functions) decrypts
    let secret = "你好 Codex++ 🚀 secret payload with unicode & newlines\nline2";
    let enc = encrypt_text(secret).expect("rust encrypt");
    let node_script = format!(
        r#"
        const fromB64u = v => {{
          const n = v.replace(/-/g, '+').replace(/_/g, '/');
          return Uint8Array.from(atob(n.padEnd(Math.ceil(n.length / 4) * 4, '=')), c => c.charCodeAt(0));
        }};
        const toB64u = b => btoa(String.fromCharCode(...b)).replace(/\+/g, '-').replace(/\//g, '_').replace(/=+$/, '');
        (async () => {{
          const payload = {payload};
          const key = await crypto.subtle.importKey('raw', fromB64u({key}), {{ name: 'AES-GCM' }}, false, ['decrypt']);
          const pt = await crypto.subtle.decrypt({{ name: 'AES-GCM', iv: fromB64u(payload.iv) }}, key, fromB64u(payload.ciphertext));
          const dec = new TextDecoder().decode(pt);

          const nKey = await crypto.subtle.generateKey({{ name: 'AES-GCM', length: 256 }}, true, ['encrypt']);
          const nIv = crypto.getRandomValues(new Uint8Array(12));
          const nMsg = 'From Node WebCrypto: 加密验证 ✅';
          const nCt = await crypto.subtle.encrypt({{ name: 'AES-GCM', iv: nIv }}, nKey, new TextEncoder().encode(nMsg));
          const nRaw = toB64u(new Uint8Array(await crypto.subtle.exportKey('raw', nKey)));
          console.log(JSON.stringify({{
            decryptedByNode: dec,
            nodeCipher: {{ v: 1, iv: toB64u(nIv), ciphertext: toB64u(new Uint8Array(nCt)) }},
            nodeKey: nRaw,
            nodeMsg: nMsg
          }}));
        }})();
        "#,
        payload = serde_json::to_string(&enc.payload).unwrap(),
        key = serde_json::to_string(&enc.key).unwrap(),
    );
    let out = Command::new("node")
        .arg("-e")
        .arg(&node_script)
        .output()
        .expect("spawn node");
    assert!(
        out.status.success(),
        "node failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let v: serde_json::Value = serde_json::from_slice(&out.stdout).expect("parse node json");
    assert_eq!(v["decryptedByNode"], secret);

    let node_payload: EncryptedPayload = serde_json::from_value(v["nodeCipher"].clone()).unwrap();
    let node_dec =
        decrypt_text(&node_payload, v["nodeKey"].as_str().unwrap()).expect("rust decrypt");
    assert_eq!(node_dec, v["nodeMsg"]);
}

#[test]
fn differential_markdown_vs_app_js() {
    let session = json!({
        "kind": "codex-session",
        "session_id": "sess-001",
        "title": "  测试分享会话  ",
        "messages": [
            { "role": "user", "text": "第一问" },
            { "role": "assistant", "text": "第一答\n第二行" },
            { "role": "system", "text": "系统消息" }
        ]
    });
    let rollout = concat!(
        r#"{"type":"session_meta","payload":{"id":"s1"}}"#,
        "\n",
        "bad-json-line\n",
        r#"{"type":"response_item","payload":{"type":"message","role":"user","content":[{"text":"问题 A"}]}}"#,
        "\n",
        r#"{"type":"response_item","payload":{"type":"message","role":"assistant","content":[{"text":"回答 B"},{"text":"附加"}]}}"#,
        "\n",
        r#"{"type":"response_item","payload":{"type":"message","role":"developer","content":[{"text":"hidden"}]}}"#,
        "\n",
    );
    let js_script = format!(
        r#"
        const fs = require('node:fs');
        const vm = require('node:vm');
        const src = fs.readFileSync(String.raw`d:\Git\codex-plus\services\share-site\app.js`, 'utf8');
        const ctx = vm.createContext({{
          TextEncoder, TextDecoder, URLSearchParams,
          location: {{ search: '', pathname: '/', origin: 'https://x.test', hash: '' }},
          document: {{ querySelector: () => ({{ hidden: false, dataset: {{}}, textContent: '' }}) }},
          setupComposer() {{}},
        }});
        // Replace DOM top-level execution after helper definitions by wrapping only functions
        const fnSrc = src.slice(src.indexOf('function sessionToMarkdown'), src.indexOf('async function importSharedSession'));
        vm.runInContext(fnSrc, ctx);
        console.log(JSON.stringify({{
          sessionMd: ctx.sessionToMarkdown({session}),
          rolloutMd: ctx.rolloutToMarkdown({rollout}, '  滚动标题 '),
          emptyTitleMd: ctx.rolloutToMarkdown({rollout}, ''),
        }}));
        "#,
        session = serde_json::to_string(&session).unwrap(),
        rollout = serde_json::to_string(rollout).unwrap(),
    );
    let out = Command::new("node")
        .arg("-e")
        .arg(&js_script)
        .output()
        .expect("spawn node");
    assert!(
        out.status.success(),
        "node failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let v: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(session_to_markdown(&session), v["sessionMd"]);
    assert_eq!(
        rollout_to_markdown(rollout, Some("  滚动标题 ")),
        v["rolloutMd"]
    );
    assert_eq!(rollout_to_markdown(rollout, Some("")), v["emptyTitleMd"]);
}

#[test]
fn worker_routes_gate_open_post_and_support_authenticated_flow() {
    let store = InMemoryShareStore::default();
    let enc = encrypt_text("hello encrypted world!").unwrap();
    let payload_body =
        serde_json::to_vec(&json!({ "ttl": 86_400, "encrypted": enc.payload })).unwrap();

    // 1. Default config rejects unauthenticated POST /api/shares with 403 (ADR-007)
    let default_cfg = ShareSiteConfig::default();
    let denied = handle_request(
        &WorkerRequest {
            method: "POST".into(),
            path: "/api/shares".into(),
            headers: HashMap::from([("Content-Type".into(), "application/json".into())]),
            body: payload_body.clone(),
        },
        &store,
        &default_cfg,
        1_000_000,
    );
    assert_eq!(denied.status, 403);
    assert_eq!(
        denied.json_body().unwrap()["error"],
        "Session share creation is disabled"
    );

    // 2. OPTIONS CORS check
    let opts = handle_request(
        &WorkerRequest {
            method: "OPTIONS".into(),
            path: "/api/shares".into(),
            headers: HashMap::new(),
            body: vec![],
        },
        &store,
        &default_cfg,
        1_000_000,
    );
    assert_eq!(opts.status, 204);
    assert_eq!(
        opts.headers.get("Access-Control-Allow-Origin").unwrap(),
        "*"
    );

    // 3. Explicit admin token allows POST -> GET -> wrong DELETE (403) -> right DELETE (204) -> GET (404)
    let admin_cfg = ShareSiteConfig {
        create_admin_token: Some("admin-secret-token".into()),
    };
    let created = handle_request(
        &WorkerRequest {
            method: "POST".into(),
            path: "/api/shares".into(),
            headers: HashMap::from([
                ("Content-Type".into(), "application/json".into()),
                ("Authorization".into(), "Bearer admin-secret-token".into()),
            ]),
            body: payload_body,
        },
        &store,
        &admin_cfg,
        1_000_000,
    );
    assert_eq!(created.status, 201);
    let created_json = created.json_body().unwrap();
    let id = created_json["id"].as_str().unwrap().to_string();
    let del_token = created_json["deleteToken"].as_str().unwrap().to_string();

    let fetched = handle_request(
        &WorkerRequest {
            method: "GET".into(),
            path: format!("/api/shares/{id}"),
            headers: HashMap::new(),
            body: vec![],
        },
        &store,
        &default_cfg,
        1_000_500,
    );
    assert_eq!(fetched.status, 200);
    let got_enc: EncryptedPayload =
        serde_json::from_value(fetched.json_body().unwrap()["encrypted"].clone()).unwrap();
    assert_eq!(
        decrypt_text(&got_enc, &enc.key).unwrap(),
        "hello encrypted world!"
    );

    // Expired view -> 404
    let expired = handle_request(
        &WorkerRequest {
            method: "GET".into(),
            path: format!("/api/shares/{id}"),
            headers: HashMap::new(),
            body: vec![],
        },
        &store,
        &default_cfg,
        1_000_000 + 86_400_000,
    );
    assert_eq!(expired.status, 404);

    // Wrong delete token -> 403
    let bad_del = handle_request(
        &WorkerRequest {
            method: "DELETE".into(),
            path: format!("/api/shares/{id}"),
            headers: HashMap::from([("X-Delete-Token".into(), "wrong-token".into())]),
            body: vec![],
        },
        &store,
        &default_cfg,
        1_000_500,
    );
    assert_eq!(bad_del.status, 403);

    // Correct delete token -> 204 & deleted
    let ok_del = handle_request(
        &WorkerRequest {
            method: "DELETE".into(),
            path: format!("/api/shares/{id}"),
            headers: HashMap::from([("X-Delete-Token".into(), del_token)]),
            body: vec![],
        },
        &store,
        &default_cfg,
        1_000_500,
    );
    assert_eq!(ok_del.status, 204);
    assert!(store.get(&format!("share:{id}")).unwrap().is_none());

    // SPA rewrite for /s/:id
    let spa = handle_request(
        &WorkerRequest {
            method: "GET".into(),
            path: "/s/abcdefghijklmnopqrstuv".into(),
            headers: HashMap::new(),
            body: vec![],
        },
        &store,
        &default_cfg,
        1_000_500,
    );
    assert_eq!(spa.status, 200);
    assert_eq!(spa.rewritten_asset_path.as_deref(), Some("/"));

    // Unmatched `/api/*` routes return 404 JSON instead of rewriting to static assets
    for bad_api_path in ["/api", "/api/unknown", "/api/shares/short"] {
        let resp = handle_request(
            &WorkerRequest {
                method: "GET".into(),
                path: bad_api_path.into(),
                headers: HashMap::new(),
                body: vec![],
            },
            &store,
            &default_cfg,
            1_000_500,
        );
        assert_eq!(resp.status, 404, "expected 404 JSON for {bad_api_path}");
        assert!(resp.rewritten_asset_path.is_none());
        assert_eq!(resp.json_body().unwrap()["error"], "Not found");
    }
}

#[test]
fn crypto_padding_ciphertext_cap_store_eviction_and_storage_500_errors() {
    use share_site_rs::ShareRecord;
    use share_site_rs::crypto::{sha256_b64u, timing_safe_eq_str};
    use share_site_rs::worker::MAX_CIPHERTEXT_LENGTH;

    // 1. Constant-time string comparison across differing lengths
    assert!(timing_safe_eq_str("same-secret", "same-secret"));
    assert!(!timing_safe_eq_str("short", "much-longer-secret-token"));

    // 2. Optional `=` padding in `decrypt_text` & `MAX_CIPHERTEXT_LENGTH` guard
    let enc = encrypt_text("padded-check").unwrap();
    let pad_to_4 = |s: &str| {
        let rem = s.len() % 4;
        if rem == 0 {
            s.to_string()
        } else {
            format!("{s}{}", "=".repeat(4 - rem))
        }
    };
    let padded_payload = EncryptedPayload {
        v: 1,
        iv: pad_to_4(&enc.payload.iv),
        ciphertext: pad_to_4(&enc.payload.ciphertext),
    };
    let padded_key = pad_to_4(&enc.key);
    assert_eq!(
        decrypt_text(&padded_payload, &padded_key).unwrap(),
        "padded-check"
    );

    let oversized = EncryptedPayload {
        v: 1,
        iv: enc.payload.iv.clone(),
        ciphertext: "A".repeat(MAX_CIPHERTEXT_LENGTH + 1),
    };
    assert!(decrypt_text(&oversized, &enc.key).is_err());

    // 3. `InMemoryShareStore` evicts expired entries on subsequent writes and `evict_expired`
    let store = InMemoryShareStore::default();
    store
        .put(
            "share:old",
            ShareRecord {
                encrypted: enc.payload.clone(),
                delete_token_hash: "h1".into(),
                created_at: 1_000,
                expires_at: 2_000,
            },
            1,
        )
        .unwrap();
    assert_eq!(store.len().unwrap(), 1);
    store
        .put(
            "share:new",
            ShareRecord {
                encrypted: enc.payload.clone(),
                delete_token_hash: "h2".into(),
                created_at: 5_000,
                expires_at: 9_000,
            },
            4,
        )
        .unwrap();
    assert!(store.get("share:old").unwrap().is_none());
    assert_eq!(store.len().unwrap(), 1);
    assert_eq!(store.evict_expired(10_000).unwrap(), 1);
    assert!(store.is_empty().unwrap());

    // 4. `store.get` and `store.delete` failures return 500 JSON
    struct FailingStore {
        fail_get: bool,
        fail_delete: bool,
        record: ShareRecord,
    }
    impl ShareStore for FailingStore {
        fn get(&self, _key: &str) -> Result<Option<ShareRecord>, String> {
            if self.fail_get {
                Err("db read error".into())
            } else {
                Ok(Some(self.record.clone()))
            }
        }
        fn put(&self, _key: &str, _record: ShareRecord, _ttl: u64) -> Result<(), String> {
            Err("db write error".into())
        }
        fn delete(&self, _key: &str) -> Result<(), String> {
            if self.fail_delete {
                Err("db delete error".into())
            } else {
                Ok(())
            }
        }
    }

    let del_tok = "valid-delete-token";
    let rec = ShareRecord {
        encrypted: enc.payload,
        delete_token_hash: sha256_b64u(del_tok.as_bytes()),
        created_at: 1_000,
        expires_at: 99_000,
    };
    let get_err_store = FailingStore {
        fail_get: true,
        fail_delete: false,
        record: rec.clone(),
    };
    let cfg = ShareSiteConfig::default();
    let r_get = handle_request(
        &WorkerRequest {
            method: "GET".into(),
            path: "/api/shares/abcdefghijklmnopqrstuv".into(),
            headers: HashMap::new(),
            body: vec![],
        },
        &get_err_store,
        &cfg,
        2_000,
    );
    assert_eq!(r_get.status, 500);

    let r_del_get_err = handle_request(
        &WorkerRequest {
            method: "DELETE".into(),
            path: "/api/shares/abcdefghijklmnopqrstuv".into(),
            headers: HashMap::from([("X-Delete-Token".into(), del_tok.into())]),
            body: vec![],
        },
        &get_err_store,
        &cfg,
        2_000,
    );
    assert_eq!(r_del_get_err.status, 500);

    let del_err_store = FailingStore {
        fail_get: false,
        fail_delete: true,
        record: rec,
    };
    let r_del_err = handle_request(
        &WorkerRequest {
            method: "DELETE".into(),
            path: "/api/shares/abcdefghijklmnopqrstuv".into(),
            headers: HashMap::from([("X-Delete-Token".into(), del_tok.into())]),
            body: vec![],
        },
        &del_err_store,
        &cfg,
        2_000,
    );
    assert_eq!(r_del_err.status, 500);
}
