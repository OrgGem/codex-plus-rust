use remote_control_p0::asar::{extract_ipc_versions, read_asar_file};
use remote_control_p0::follower::{ThreadFollower, apply_patches};
use remote_control_p0::ipc_client::{IpcClient, MAX_FRAME_BYTES, frame_message, read_frame};
use remote_control_p0::mock_responses::serve_mock_responses;
use remote_control_p0::runners::{ensure_isolated_home, home_dir};
use remote_control_p0::sidecar::SidecarClient;
use serde_json::{Value, json};
use std::fs;
use std::io::Write;
use std::net::TcpListener;
use std::process::Command;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tempfile::tempdir;

fn build_synthetic_asar(file_contents: &str) -> Vec<u8> {
    let header_json = json!({
        "files": {
            ".vite": { "files": { "build": { "files": { "early-bootstrap.js": { "size": file_contents.len(), "offset": "0" } } } } }
        }
    })
    .to_string();
    let json_bytes = header_json.as_bytes();
    let padded_json_len = (json_bytes.len() + 3) & !3;
    let header_size = padded_json_len + 8;
    let mut out = Vec::new();
    out.extend_from_slice(&4_u32.to_le_bytes());
    out.extend_from_slice(&(header_size as u32).to_le_bytes());
    out.extend_from_slice(&((padded_json_len + 4) as u32).to_le_bytes());
    out.extend_from_slice(&(json_bytes.len() as u32).to_le_bytes());
    out.extend_from_slice(json_bytes);
    out.resize(16 + padded_json_len, 0);
    out.extend_from_slice(file_contents.as_bytes());
    out
}

#[test]
fn differential_asar_ipc_versions_vs_node() {
    let dir = tempdir().unwrap();
    let asar_path = dir.path().join("app.asar");
    let bootstrap = r#"/* banner */ const IPC={"thread-stream-state-changed":13,"thread-follower-start-turn":2,"thread-follower-interrupt-turn":4}; export {};"#;
    fs::write(&asar_path, build_synthetic_asar(bootstrap)).unwrap();

    let rust_src = read_asar_file(&asar_path, ".vite/build/early-bootstrap.js").unwrap();
    let rust_ver = extract_ipc_versions(&rust_src).unwrap();

    let node_script = format!(
        r#"import {{ readAsarFile, extractIpcVersions }} from './ipc-versions.mjs';
           const s = readAsarFile({path}, '.vite/build/early-bootstrap.js');
           console.log(JSON.stringify(extractIpcVersions(s)));"#,
        path = serde_json::to_string(&asar_path.to_string_lossy()).unwrap()
    );
    let out = Command::new("node")
        .current_dir(r"d:\Git\codex-plus\tools\remote-control-p0")
        .args(["--input-type=module", "-e", &node_script])
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let node_ver: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(rust_ver, node_ver);
}

#[test]
fn mock_responses_server_handles_models_runs_asks_and_logs() {
    let dir = tempdir().unwrap();
    let log_path = dir.path().join("mock.jsonl");
    let port = TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port();
    let stop = Arc::new(AtomicBool::new(false));
    let stop_srv = Arc::clone(&stop);
    let lp = log_path.clone();
    let work = dir.path().to_string_lossy().into_owned();
    let handle = std::thread::spawn(move || {
        let _ = serve_mock_responses(port, work, Some(lp), Some(stop_srv));
    });
    std::thread::sleep(Duration::from_millis(80));

    let base = format!("http://127.0.0.1:{port}");
    let models: Value = ureq::get(&format!("{base}/v1/models"))
        .call()
        .unwrap()
        .into_json()
        .unwrap();
    assert_eq!(models["data"][0]["id"], "mock-model");

    let sse_ok = ureq::post(&format!("{base}/v1/responses"))
        .send_json(json!({
            "model": "gpt-test",
            "reasoning": { "effort": "high" },
            "input": [
                { "role": "developer", "content": "Collaboration Mode: Plan\ndanger-full-access" },
                { "role": "user", "content": [{ "type": "text", "text": "hello remote-uploads/doc.txt" }, { "type": "input_image" }] }
            ]
        }))
        .unwrap()
        .into_string()
        .unwrap();
    assert!(sse_ok.contains("收到：hello remote-uploads/doc.txt"));

    let sse_run = ureq::post(&format!("{base}/v1/responses"))
        .send_json(json!({ "input": [{ "role": "user", "content": "RUN test" }] }))
        .unwrap()
        .into_string()
        .unwrap();
    assert!(sse_run.contains("exec_command"));

    let sse_ask = ureq::post(&format!("{base}/v1/responses"))
        .send_json(json!({ "input": [{ "role": "user", "content": "ASK choice" }] }))
        .unwrap()
        .into_string()
        .unwrap();
    assert!(sse_ask.contains("request_user_input"));

    let sse_tool = ureq::post(&format!("{base}/v1/responses"))
        .send_json(json!({ "input": [{ "type": "function_call_output", "output": "ok-out" }] }))
        .unwrap()
        .into_string()
        .unwrap();
    assert!(sse_tool.contains("工具结果：ok-out"));

    stop.store(true, Ordering::Relaxed);
    let _ = ureq::get(&format!("{base}/__stop")).call();
    let _ = handle.join();

    let logged = fs::read_to_string(&log_path).unwrap();
    let first: Value = serde_json::from_str(logged.lines().next().unwrap()).unwrap();
    assert_eq!(first["model"], "gpt-test");
    assert_eq!(first["effort"], "high");
    assert_eq!(first["mode"], "plan");
    assert_eq!(first["sandbox"], "full");
    assert_eq!(first["images"], 1);
    assert_eq!(first["files"][0], "remote-uploads/doc.txt");
}

#[test]
fn ipc_client_follower_and_sidecar_e2e() {
    // 1. Mock IPC router over TCP loopback
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let srv_handle = std::thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        // Read initialize
        let init = read_frame(&mut stream).unwrap();
        assert_eq!(init["method"], "initialize");
        stream
            .write_all(&frame_message(&json!({ "type": "response", "requestId": init["requestId"], "resultType": "success", "result": { "clientId": "cli-1" } })).unwrap())
            .unwrap();

        // Send discovery probe -> client must respond canHandle:false
        stream
            .write_all(
                &frame_message(
                    &json!({ "type": "client-discovery-request", "requestId": "disc-1" }),
                )
                .unwrap(),
            )
            .unwrap();
        let disc_resp = read_frame(&mut stream).unwrap();
        assert_eq!(disc_resp["type"], "client-discovery-response");
        assert_eq!(disc_resp["response"]["canHandle"], false);

        // Read thread-owner-discovery
        let owner_req = read_frame(&mut stream).unwrap();
        assert_eq!(owner_req["method"], "thread-owner-discovery");
        stream
            .write_all(&frame_message(&json!({ "type": "response", "requestId": owner_req["requestId"], "resultType": "success", "handledByClientId": "owner-9" })).unwrap())
            .unwrap();

        // Read following-changed broadcast & complete-history resync
        let bcast = read_frame(&mut stream).unwrap();
        assert_eq!(bcast["type"], "broadcast");
        let hist_req = read_frame(&mut stream).unwrap();
        assert_eq!(hist_req["method"], "thread-follower-load-complete-history");
        stream
            .write_all(&frame_message(&json!({ "type": "response", "requestId": hist_req["requestId"], "resultType": "success" })).unwrap())
            .unwrap();

        // Broadcast snapshot then Immer patch
        stream
            .write_all(
                &frame_message(&json!({
                    "type": "broadcast",
                    "sourceClientId": "owner-9",
                    "method": "thread-stream-state-changed",
                    "params": {
                        "conversationId": "th-1",
                        "change": { "type": "snapshot", "revision": 1, "conversationState": { "turns": [{ "id": "t0", "items": [] }] } }
                    }
                }))
                .unwrap(),
            )
            .unwrap();
        stream
            .write_all(
                &frame_message(&json!({
                    "type": "broadcast",
                    "sourceClientId": "owner-9",
                    "method": "thread-stream-state-changed",
                    "params": {
                        "conversationId": "th-1",
                        "change": { "type": "patches", "baseRevision": 1, "revision": 2, "patches": [{ "op": "add", "path": ["turns", 0, "items", 0], "value": { "type": "agentMessage", "text": "patched" } }] }
                    }
                }))
                .unwrap(),
            )
            .unwrap();
        std::thread::sleep(Duration::from_millis(120));
    });

    let ipc = Arc::new(IpcClient::new(
        format!("tcp://127.0.0.1:{port}"),
        "test-remote",
    ));
    assert_eq!(ipc.connect().unwrap(), "cli-1");
    let follower = ThreadFollower::attach(Arc::clone(&ipc), "th-1", "local");
    follower.start().unwrap();
    std::thread::sleep(Duration::from_millis(80));
    let turns = follower.turns();
    assert_eq!(turns.len(), 1);
    assert_eq!(turns[0]["items"][0]["text"], "patched");
    follower.stop().unwrap();
    ipc.close();
    let _ = srv_handle.join();

    // 2. Patch application preserves key order on object delete (shift_remove)
    let patched = apply_patches(
        json!({ "a": 1, "b": 2, "c": 3 }),
        &[json!({ "op": "remove", "path": ["b"] })],
    )
    .unwrap();
    assert_eq!(patched.to_string(), r#"{"a":1,"c":3}"#);

    // 3. Sidecar create_thread + release + server-initiated request -32601 reply against mock app-server
    let dir = tempdir().unwrap();
    let mock_js = dir.path().join("mock_sidecar.cjs");
    fs::write(
        &mock_js,
        r#"
let loaded = false;
let pendingServerApproval = false;
require('node:readline').createInterface({ input: process.stdin }).on('line', (l) => {
  if (!l.trim()) return;
  const m = JSON.parse(l);
  if (m.id === 999 && m.error && m.error.code === -32601) {
    pendingServerApproval = false;
    console.log(JSON.stringify({ jsonrpc: '2.0', id: pendingCallId, result: { thread: { id: 'sc-1' } } }));
    return;
  }
  if (m.method === 'initialize') console.log(JSON.stringify({ jsonrpc: '2.0', id: m.id, result: {} }));
  else if (m.method === 'thread/start') {
    loaded = true;
    global.pendingCallId = m.id;
    pendingServerApproval = true;
    // Emit a server-initiated JSON-RPC request; sidecar must immediately reject with -32601!
    console.log(JSON.stringify({ jsonrpc: '2.0', id: 999, method: 'item/commandExecution/requestApproval', params: {} }));
  }
  else if (m.method === 'thread/name/set') console.log(JSON.stringify({ jsonrpc: '2.0', id: m.id, result: {} }));
  else if (m.method === 'thread/unsubscribe') { loaded = false; console.log(JSON.stringify({ jsonrpc: '2.0', id: m.id, result: {} })); }
  else if (m.method === 'thread/loaded/list') console.log(JSON.stringify({ jsonrpc: '2.0', id: m.id, result: { data: loaded ? ['sc-1'] : [] } }));
});
"#,
    )
    .unwrap();
    let cmd_stub = dir.path().join("sidecar_stub.cmd");
    fs::write(
        &cmd_stub,
        format!("@echo off\r\nnode \"{}\" %*\r\n", mock_js.display()),
    )
    .unwrap();
    let sc =
        SidecarClient::start(cmd_stub.to_str().unwrap(), dir.path().to_str().unwrap()).unwrap();
    let tid = sc.create_thread(".", "Test Sidecar").unwrap();
    assert_eq!(tid, "sc-1");
    assert!(!sc.is_loaded(&tid).unwrap());
    sc.close();
}

#[test]
fn bounds_checks_isolated_home_and_cli_nonzero_exit_code() {
    // 1. Oversized IPC frame (> 64 MiB) rejected before allocation
    let huge_len = ((MAX_FRAME_BYTES as u32) + 1).to_le_bytes();
    let mut cursor = std::io::Cursor::new(huge_len);
    let err = read_frame(&mut cursor).unwrap_err();
    assert!(err.to_string().contains("exceeds"));

    // 2. ASAR corrupted size/offset bounds checked against file length
    let dir = tempdir().unwrap();
    let bad_asar = dir.path().join("bad.asar");
    let mut bytes = build_synthetic_asar("hello");
    // Overwrite header JSON entry size with huge value via fresh header
    let bad_header = json!({
        "files": {
            "a.js": { "size": 999_999_999_u64, "offset": "0" }
        }
    })
    .to_string();
    let jb = bad_header.as_bytes();
    let pad = (jb.len() + 3) & !3;
    bytes.clear();
    bytes.extend_from_slice(&4_u32.to_le_bytes());
    bytes.extend_from_slice(&((pad + 8) as u32).to_le_bytes());
    bytes.extend_from_slice(&((pad + 4) as u32).to_le_bytes());
    bytes.extend_from_slice(&(jb.len() as u32).to_le_bytes());
    bytes.extend_from_slice(jb);
    bytes.resize(16 + pad, 0);
    bytes.extend_from_slice(b"tiny");
    fs::write(&bad_asar, &bytes).unwrap();
    assert!(read_asar_file(&bad_asar, "a.js").is_err());

    // 3. ensure_isolated_home rejects case/slash/dotdot variants of ~/.codex
    let real = home_dir().join(".codex");
    assert!(ensure_isolated_home(&real.to_string_lossy()).is_err());
    assert!(ensure_isolated_home(&format!("{}/", real.to_string_lossy())).is_err());
    assert!(ensure_isolated_home(&real.to_string_lossy().to_uppercase()).is_err());
    let dotdot = home_dir().join("sub").join("..").join(".codex");
    assert!(ensure_isolated_home(&dotdot.to_string_lossy()).is_err());
    assert!(ensure_isolated_home(&dir.path().join("isolated-codex").to_string_lossy()).is_ok());

    // 4. `run-suite` exits with status code 1 when HTTP check fails (!r.ok)
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let srv = std::thread::spawn(move || {
        for _ in 0..2 {
            if let Ok((mut s, _)) = listener.accept() {
                let mut sink = [0_u8; 512];
                let _ = std::io::Read::read(&mut s, &mut sink);
                // Return 200 even to unauthenticated request so first check (expects 401) fails!
                let _ = s.write_all(
                    b"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: 2\r\nConnection: close\r\n\r\n{}",
                );
            }
        }
    });
    let bin = env!("CARGO_BIN_EXE_remote-control-p0");
    let out = Command::new(bin)
        .args([
            "run-suite",
            "p1",
            "--port",
            &port.to_string(),
            "--token",
            "tok",
        ])
        .output()
        .unwrap();
    let _ = srv.join();
    assert_eq!(out.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&out.stdout).contains("✗"));
}
