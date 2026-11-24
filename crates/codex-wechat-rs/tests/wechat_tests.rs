use codex_wechat_rs::bridge::{poll_loop_with, qr_login_with};
use codex_wechat_rs::codex_client::{
    AppServerCodexClient, CodexRunner, resolve_app_server_command, run_codex_exec,
};
use codex_wechat_rs::config::{BridgeConfig, CodexConfig};
use codex_wechat_rs::protocol::{
    build_wechat_prompt, clean_codex_output, describe_non_text_items, extract_text, handle_command,
    ilink_headers, normalize_prompt, split_reply, strip_prefix,
};
use serde_json::{Value, json};
use std::collections::HashSet;
use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tempfile::tempdir;

#[test]
fn differential_protocol_and_helpers_vs_python() {
    let cases_items = json!([
        [{"type": 1, "text_item": {"text": "你好"}, "ref_msg": {"message_item": {"type": 1, "text_item": {"text": "上下文"}, "ref_msg": {"message_item": {"type": 1, "text_item": {"text": "更早"}}}}}}],
        [{"type": 6, "voice_item": {"text": "语音转文字"}}, {"type": 2}, {"type": 3}, {"type": 5, "file_item": {"file_name": "report.pdf"}}, {"type": 6, "voice_item": {}}]
    ]);
    let split_inputs = vec![
        ("a\nb\nc\nd".to_string(), 5_usize),
        (
            format!("{}\n{}", "中文".repeat(60), "第二段🚀".repeat(80)),
            180_usize,
        ),
        ("x".repeat(250), 100_usize),
    ];
    let py = format!(
        r#"
import json, sys
sys.path.insert(0, r'd:\Git\codex-plus\tools\codex-wechat')
import codex_wechat as cw

items = json.loads({items_json})
splits = json.loads({splits_json})
cfg = cw.BridgeConfig(require_prefix=True, prefix='/codex', max_reply_chars=1800)
cfg.codex.prompt_prefix = '请简短回答'

out = {{
  'extract': [cw.extract_text(x) for x in items],
  'describe': [cw.describe_non_text_items(x) for x in items],
  'norm': [cw.normalize_prompt('/codex 帮我看下', x) for x in items],
  'strip': [list(cw.strip_prefix(cfg, s)) for s in ['/codex', '/codex hello', '/codex\nhi', '/codexxx no', 'plain']],
  'clean': cw.clean_codex_output('\x1b[32mok\x1b[0m\n  \nline2  \n\n', 'err'),
  'prompt': cw.build_wechat_prompt(cfg.codex, '测试'),
  'splits': [cw.split_reply(t, m) for t, m in splits],
  'cmds': [cw.handle_command(cfg, 'u123', s) for s in ['/id', '/whoami', '/codex /id', '/ping', '/help', 'other']],
  'headers_keys': sorted(list(cw.ilink_headers('tok-1', b'abc').keys())),
  'client_ver': cw.ILINK_APP_CLIENT_VERSION,
}}
print(json.dumps(out, ensure_ascii=False))
"#,
        items_json = serde_json::to_string(&cases_items.to_string()).unwrap(),
        splits_json =
            serde_json::to_string(&serde_json::to_string(&split_inputs).unwrap()).unwrap(),
    );

    let out = Command::new("python")
        .arg("-c")
        .arg(&py)
        .output()
        .expect("run python");
    assert!(
        out.status.success(),
        "python failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let py_v: Value = serde_json::from_slice(&out.stdout).unwrap();

    let cfg = BridgeConfig {
        require_prefix: true,
        prefix: "/codex".into(),
        codex: CodexConfig {
            prompt_prefix: "请简短回答".into(),
            ..CodexConfig::default()
        },
        ..BridgeConfig::default()
    };

    for (i, group) in cases_items.as_array().unwrap().iter().enumerate() {
        assert_eq!(extract_text(group.as_array().unwrap()), py_v["extract"][i]);
        assert_eq!(
            json!(describe_non_text_items(group.as_array().unwrap())),
            py_v["describe"][i]
        );
        assert_eq!(
            normalize_prompt("/codex 帮我看下", group.as_array().unwrap()),
            py_v["norm"][i]
        );
    }
    for (i, s) in [
        "/codex",
        "/codex hello",
        "/codex\nhi",
        "/codexxx no",
        "plain",
    ]
    .iter()
    .enumerate()
    {
        let (acc, st) = strip_prefix(&cfg, s);
        assert_eq!(json!([acc, st]), py_v["strip"][i]);
    }
    assert_eq!(
        clean_codex_output("\x1b[32mok\x1b[0m\n  \nline2  \n\n", "err"),
        py_v["clean"]
    );
    assert_eq!(build_wechat_prompt(&cfg.codex, "测试"), py_v["prompt"]);
    for (i, (text, max_c)) in split_inputs.iter().enumerate() {
        assert_eq!(json!(split_reply(text, *max_c)), py_v["splits"][i]);
    }
    for (i, s) in ["/id", "/whoami", "/codex /id", "/ping", "/help", "other"]
        .iter()
        .enumerate()
    {
        assert_eq!(handle_command(&cfg, "u123", s), py_v["cmds"][i]);
    }
    let hdr_keys: Vec<String> = ilink_headers("tok-1", Some(3)).keys().cloned().collect();
    assert_eq!(json!(hdr_keys), py_v["headers_keys"]);
    assert_eq!(
        codex_wechat_rs::protocol::ILINK_APP_CLIENT_VERSION,
        py_v["client_ver"]
    );
}

#[test]
fn resolve_cmd_finds_vendor_codex_exe() {
    let dir = tempdir().unwrap();
    let npm = dir.path().join("npm");
    let vendor = npm.join("node_modules/@openai/codex/node_modules/@openai/codex-win32-x64/vendor/x86_64-pc-windows-msvc/bin");
    fs::create_dir_all(&vendor).unwrap();
    let exe = vendor.join("codex.exe");
    fs::write(&exe, b"stub").unwrap();
    let cfg = CodexConfig {
        command: npm.join("codex.cmd").to_string_lossy().into(),
        ..CodexConfig::default()
    };
    assert_eq!(
        std::path::Path::new(&resolve_app_server_command(&cfg)),
        exe.as_path()
    );
}

fn spawn_mock_http(
    handler: impl Fn(&str, &str, &str) -> (u16, String) + Send + Sync + 'static,
) -> (u16, std::thread::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let handler = Arc::new(handler);
    let handle = std::thread::spawn(move || {
        for mut stream in listener.incoming().flatten() {
            let mut buf = Vec::new();
            let mut tmp = [0_u8; 4096];
            let mut header_end = None;
            while let Ok(n) = stream.read(&mut tmp) {
                if n == 0 {
                    break;
                }
                buf.extend_from_slice(&tmp[..n]);
                if let Some(pos) = buf.windows(4).position(|w| w == b"\r\n\r\n") {
                    header_end = Some(pos + 4);
                    break;
                }
            }
            let Some(hend) = header_end else { continue };
            let head_str = String::from_utf8_lossy(&buf[..hend]).into_owned();
            let content_len = head_str
                .lines()
                .find_map(|l| {
                    l.split_once(':')
                        .filter(|(k, _)| k.eq_ignore_ascii_case("content-length"))
                        .and_then(|(_, v)| v.trim().parse::<usize>().ok())
                })
                .unwrap_or(0);
            while buf.len() < hend + content_len {
                let Ok(n) = stream.read(&mut tmp) else { break };
                if n == 0 {
                    break;
                }
                buf.extend_from_slice(&tmp[..n]);
            }
            let body =
                String::from_utf8_lossy(&buf[hend..buf.len().min(hend + content_len)]).into_owned();
            let mut first = head_str.lines().next().unwrap_or("").split_whitespace();
            let method = first.next().unwrap_or("");
            let path = first.next().unwrap_or("");
            if path == "/__stop" {
                let _ = stream.write_all(
                    b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\n{}",
                );
                break;
            }
            let (status, resp_body) = handler(method, path, &body);
            let msg = format!(
                "HTTP/1.1 {status} OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{resp_body}",
                resp_body.len()
            );
            let _ = stream.write_all(msg.as_bytes());
        }
    });
    (port, handle)
}

#[test]
fn e2e_qr_login_redirect_and_poll_loop_with_mock_app_server() {
    let dir = tempdir().unwrap();
    // 1. Test QR login with redirect host -> confirmed -> writes JSON config
    let (p2, h2) = spawn_mock_http(|_, path, _| {
        if path.starts_with("/ilink/bot/get_qrcode_status") {
            (200, json!({"status":"confirmed","bot_token":"tok-live","ilink_bot_id":"bot-1","ilink_user_id":"user-9"}).to_string())
        } else {
            (404, "{}".into())
        }
    });
    let calls_qr = Arc::new(Mutex::new(0_usize));
    let cq = Arc::clone(&calls_qr);
    let (p1, h1) = spawn_mock_http(move |_, path, _| {
        if path.starts_with("/ilink/bot/get_bot_qrcode") {
            (
                200,
                json!({"qrcode":"qr123","qrcode_img_content":"https://qr.example/123"}).to_string(),
            )
        } else {
            let mut g = cq.lock().unwrap();
            *g += 1;
            if *g == 1 {
                (200, json!({"status":"scaned"}).to_string())
            } else {
                (200, json!({"status":"scaned_but_redirect","redirect_host":format!("127.0.0.1:{p2}")}).to_string())
            }
        }
    });
    let login_out = dir.path().join("wechat-cfg.json");
    qr_login_with(
        &format!("http://127.0.0.1:{p1}"),
        &login_out,
        "3",
        10,
        "http",
        Duration::from_millis(10),
    )
    .expect("qr login");
    let _ = ureq::get(&format!("http://127.0.0.1:{p1}/__stop")).call();
    let _ = ureq::get(&format!("http://127.0.0.1:{p2}/__stop")).call();
    let _ = h1.join();
    let _ = h2.join();
    let saved: Value = serde_json::from_str(&fs::read_to_string(&login_out).unwrap()).unwrap();
    assert_eq!(saved["token"], "tok-live");
    assert_eq!(saved["account_id"], "bot-1");
    assert_eq!(saved["login_user_id"], "user-9");

    // 2. Test mock app-server + poll_loop_with end-to-end
    let mock_js = dir.path().join("mock_app_server.cjs");
    fs::write(
        &mock_js,
        r#"
const rl = require('node:readline').createInterface({ input: process.stdin });
rl.on('line', (line) => {
  if (!line.trim()) return;
  const m = JSON.parse(line);
  if (m.method === 'initialize') console.log(JSON.stringify({ id: m.id, result: { ok: true } }));
  else if (m.method === 'thread/start') console.log(JSON.stringify({ id: m.id, result: { thread: { id: 'th_1' } } }));
  else if (m.method === 'turn/start') {
    console.log(JSON.stringify({ id: m.id, result: { turn: { id: 'tu_1' } } }));
    console.log(JSON.stringify({ method: 'item/started', params: { threadId: 'th_1', item: { id: 'it_1', type: 'agentMessage', phase: 'final_answer' } } }));
    console.log(JSON.stringify({ method: 'item/agentMessage/delta', params: { threadId: 'th_1', turnId: 'tu_1', itemId: 'it_1', delta: '来自 mock-app: ' } }));
    console.log(JSON.stringify({ method: 'item/completed', params: { threadId: 'th_1', item: { id: 'it_1', type: 'agentMessage', phase: 'final_answer', text: '来自 mock-app 的完整中文回复 ✅' } } }));
    console.log(JSON.stringify({ method: 'turn/completed', params: { threadId: 'th_1', turn: { id: 'tu_1', status: 'completed' } } }));
  }
});
"#,
    )
    .unwrap();
    let wrapper = dir.path().join("mock_codex.cmd");
    fs::write(
        &wrapper,
        format!("@echo off\r\nnode \"{}\" %*\r\n", mock_js.display()),
    )
    .unwrap();

    let sent_msgs: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let sm = Arc::clone(&sent_msgs);
    let (pm, hm) = spawn_mock_http(move |_, path, body| {
        if path.ends_with("getupdates") {
            let v = json!({
                "ret": 0,
                "get_updates_buf": "buf-2",
                "msgs": [{
                    "message_id": "m1",
                    "message_type": 1,
                    "from_user_id": "u_ok",
                    "context_token": "ctx-1",
                    "item_list": [{"type": 1, "text_item": {"text": "/codex 你好吗"}}]
                }]
            });
            (200, v.to_string())
        } else if path.ends_with("getconfig") {
            (200, json!({"typing_ticket": "tick-1"}).to_string())
        } else if path.ends_with("sendmessage") {
            let parsed: Value = serde_json::from_str(body).unwrap();
            let txt = parsed["msg"]["item_list"][0]["text_item"]["text"]
                .as_str()
                .unwrap()
                .to_string();
            sm.lock().unwrap().push(txt);
            (200, json!({"ret": 0}).to_string())
        } else {
            (200, json!({"ret": 0}).to_string())
        }
    });

    let cfg = BridgeConfig {
        base_url: format!("http://127.0.0.1:{pm}"),
        token: "tok-live".into(),
        account_id: "bot-1".into(),
        allow_user_ids: HashSet::from(["u_ok".into()]),
        state_dir: dir.path().join("state"),
        codex: CodexConfig {
            backend: "app-server".into(),
            command: wrapper.to_string_lossy().into(),
            app_server_command: wrapper.to_string_lossy().into(),
            ..CodexConfig::default()
        },
        ..BridgeConfig::default()
    };
    poll_loop_with(&cfg, Some(1)).expect("poll loop");
    let _ = ureq::get(&format!("http://127.0.0.1:{pm}/__stop")).call();
    let _ = hm.join();

    let got = sent_msgs.lock().unwrap();
    assert_eq!(got.as_slice(), &["来自 mock-app 的完整中文回复 ✅"]);
    assert_eq!(
        fs::read_to_string(dir.path().join("state/get_updates_buf.txt")).unwrap(),
        "buf-2"
    );

    // Also test CodexRunner exec fallback explicitly
    let exec_cmd = dir.path().join("exec_stub.cmd");
    fs::write(&exec_cmd, "@echo off\r\necho \x1b[32mexec-ok\x1b[0m\r\n").unwrap();
    let exec_cfg = BridgeConfig {
        codex: CodexConfig {
            backend: "exec".into(),
            command: exec_cmd.to_string_lossy().into(),
            ..CodexConfig::default()
        },
        ..cfg
    };
    let runner = CodexRunner::new(exec_cfg);
    assert_eq!(runner.run("u_ok", "hi"), "exec-ok");
}

#[test]
fn exec_stdin_prompt_avoids_cmd_injection_and_drains_large_pipes() {
    let dir = tempdir().unwrap();
    let pwned_marker = dir.path().join("pwned.txt");
    let script = dir.path().join("mock_exec_large.cjs");
    fs::write(
        &script,
        r#"
let input = '';
process.stdin.setEncoding('utf8');
process.stdin.on('data', (c) => { input += c; });
process.stdin.on('end', () => {
  const lastArg = process.argv[process.argv.length - 1];
  process.stderr.write('E'.repeat(100000) + '\n');
  const pad = 'A'.repeat(100000);
  process.stdout.write(pad + '\nLAST_ARG=' + lastArg + '\nHAS_INJECTION_TEXT=' + input.includes('& echo PWNED') + '\n');
});
"#,
    )
    .unwrap();
    let cmd_wrapper = dir.path().join("mock_exec_large.cmd");
    fs::write(
        &cmd_wrapper,
        format!("@echo off\r\nnode \"{}\" %*\r\n", script.display()),
    )
    .unwrap();

    let cfg = BridgeConfig {
        codex: CodexConfig {
            backend: "exec".into(),
            command: cmd_wrapper.to_string_lossy().into(),
            timeout_seconds: 15,
            ..CodexConfig::default()
        },
        ..BridgeConfig::default()
    };
    let tricky_prompt = format!(
        "第一行\n第二行 \"quoted\" %PATH% & echo PWNED > \"{}\"",
        pwned_marker.display()
    );
    let out = run_codex_exec(&cfg, &tricky_prompt);
    assert!(
        !pwned_marker.exists(),
        "cmd.exe shell injection must not execute"
    );
    assert!(out.contains("LAST_ARG=-"));
    assert!(out.contains("HAS_INJECTION_TEXT=true"));
    assert!(out.len() > 100_000);
}

#[test]
fn app_server_restart_clears_threads_and_failed_init_closes_child() {
    let dir = tempdir().unwrap();
    let state_file = dir.path().join("srv_state.txt");
    let mock_js = dir.path().join("mock_restart_srv.cjs");
    fs::write(
        &mock_js,
        format!(
            r#"
const fs = require('node:fs');
const stateFile = {};
let runNum = 0;
try {{ runNum = Number(fs.readFileSync(stateFile, 'utf8')) || 0; }} catch {{}}
runNum += 1;
fs.writeFileSync(stateFile, String(runNum));

const rl = require('node:readline').createInterface({{ input: process.stdin }});
rl.on('line', (line) => {{
  if (!line.trim()) return;
  const m = JSON.parse(line);
  if (m.method === 'initialize') {{
    if (runNum === 1) {{
      console.log(JSON.stringify({{ id: m.id, error: {{ code: -32000, message: 'init boom' }} }}));
      return;
    }}
    console.log(JSON.stringify({{ id: m.id, result: {{ ok: true }} }}));
  }} else if (m.method === 'thread/start') {{
    console.log(JSON.stringify({{ id: m.id, result: {{ thread: {{ id: 'th_run_' + runNum }} }} }}));
  }} else if (m.method === 'turn/start') {{
    const tid = m.params.threadId;
    console.log(JSON.stringify({{ id: m.id, result: {{ turn: {{ id: 'tu_' + runNum }} }} }}));
    console.log(JSON.stringify({{ method: 'item/completed', params: {{ threadId: tid, item: {{ id: 'i1', type: 'agentMessage', phase: 'final_answer', text: 'thread=' + tid }} }} }}));
    console.log(JSON.stringify({{ method: 'turn/completed', params: {{ threadId: tid, turn: {{ id: 'tu_' + runNum, status: 'completed' }} }} }}));
    if (runNum === 2) {{
      setTimeout(() => process.exit(0), 20);
    }}
  }}
}});
"#,
            serde_json::to_string(&state_file.to_string_lossy()).unwrap()
        ),
    )
    .unwrap();
    let wrapper = dir.path().join("mock_restart.cmd");
    fs::write(
        &wrapper,
        format!("@echo off\r\nnode \"{}\" %*\r\n", mock_js.display()),
    )
    .unwrap();

    let cfg = BridgeConfig {
        codex: CodexConfig {
            backend: "app-server".into(),
            command: wrapper.to_string_lossy().into(),
            app_server_command: wrapper.to_string_lossy().into(),
            timeout_seconds: 10,
            ..CodexConfig::default()
        },
        ..BridgeConfig::default()
    };
    let client = AppServerCodexClient::new(cfg);

    // Run 1: initialize fails -> client.close() cleans up proc so Run 2 spawns fresh
    assert!(client.run("u1", "hello").is_err());

    // Run 2: fresh spawn succeeds with th_run_2, then server exits
    let r2 = client.run("u1", "hello").unwrap();
    assert_eq!(r2, "thread=th_run_2");
    std::thread::sleep(Duration::from_millis(80));

    // Run 3: server exited -> ensure_started clears stale th_run_2 and starts th_run_3
    let r3 = client.run("u1", "hello again").unwrap();
    assert_eq!(r3, "thread=th_run_3");
    client.close();
}
