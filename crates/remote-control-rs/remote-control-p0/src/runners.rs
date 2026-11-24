//! Headless PoC scripts (`poc-headless-*.mjs`) and P0–P5 verification runners (`run-p0..p5.mjs`).

use crate::follower::ThreadFollower;
use crate::ipc_client::{IpcClient, default_socket_path};
use crate::sidecar::SidecarClient;
use anyhow::{Context, Result, bail, ensure};
use serde_json::{Value, json};
use std::io::{BufRead, BufReader, Write};
use std::path::{Component, Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct CheckResult {
    pub name: String,
    pub ok: bool,
    pub detail: String,
}

pub fn home_dir() -> PathBuf {
    std::env::var_os("USERPROFILE")
        .or_else(|| std::env::var_os("HOME"))
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."))
}

fn normalize_path_key(path: &Path) -> Result<String> {
    let resolved = match std::fs::canonicalize(path) {
        Ok(c) => c,
        Err(_) => {
            let abs = std::path::absolute(path)?;
            let mut clean = PathBuf::new();
            for comp in abs.components() {
                match comp {
                    Component::CurDir => {}
                    Component::ParentDir => {
                        let _ = clean.pop();
                    }
                    other => clean.push(other.as_os_str()),
                }
            }
            clean
        }
    };
    let raw = resolved.to_string_lossy();
    let unprefixed = if let Some(rest) = raw.strip_prefix(r"\\?\UNC\") {
        format!("//{rest}")
    } else if let Some(rest) = raw.strip_prefix(r"\\?\") {
        rest.to_string()
    } else {
        raw.into_owned()
    };
    let slashed = unprefixed.replace('\\', "/");
    let trimmed = if slashed.len() > 1 {
        slashed.trim_end_matches('/')
    } else {
        slashed.as_str()
    };
    Ok(trimmed.to_lowercase())
}

pub fn ensure_isolated_home(home: &str) -> Result<()> {
    ensure!(!home.trim().is_empty(), "P0_HOME 不能为空");
    let target = normalize_path_key(Path::new(home))?;
    let mut forbidden = vec![home_dir().join(".codex")];
    if let Some(up) = std::env::var_os("USERPROFILE") {
        forbidden.push(PathBuf::from(up).join(".codex"));
    }
    if let Some(hm) = std::env::var_os("HOME") {
        forbidden.push(PathBuf::from(hm).join(".codex"));
    }
    for real_codex in forbidden {
        if let Ok(real_key) = normalize_path_key(&real_codex)
            && target == real_key
        {
            bail!("P0_HOME 不能是 ~/.codex：只在隔离实例上跑");
        }
    }
    Ok(())
}

pub fn cdp_eval(port: u16, expression: &str) -> Result<Value> {
    let targets: Value = ureq::get(&format!("http://127.0.0.1:{port}/json"))
        .call()?
        .into_json()?;
    let page = targets
        .as_array()
        .and_then(|arr| {
            arr.iter().find(|t| {
                t.get("type").and_then(Value::as_str) == Some("page")
                    && t.get("url")
                        .and_then(Value::as_str)
                        .is_some_and(|u| u.starts_with("app://-/index.html"))
            })
        })
        .context("隔离桌面没有主窗口")?;
    let ws_url = page
        .get("webSocketDebuggerUrl")
        .and_then(Value::as_str)
        .context("missing webSocketDebuggerUrl")?;
    let (mut socket, _) = tungstenite::connect(ws_url)?;
    socket.send(tungstenite::Message::Text(
        json!({ "id": 1, "method": "Runtime.evaluate", "params": { "expression": expression, "returnByValue": true } }).to_string(),
    ))?;
    loop {
        let msg = socket.read()?;
        if let tungstenite::Message::Text(txt) = msg
            && let Ok(parsed) = serde_json::from_str::<Value>(&txt)
            && parsed.get("id").and_then(Value::as_i64) == Some(1)
        {
            let _ = socket.close(None);
            return Ok(parsed
                .get("result")
                .and_then(|r| r.get("result"))
                .and_then(|r| r.get("value"))
                .cloned()
                .unwrap_or(Value::Null));
        }
    }
}

pub fn api_json(
    method: &str,
    base: &str,
    route: &str,
    token: &str,
    body: Option<&Value>,
) -> Result<(u16, Value)> {
    let url = format!("{base}/api{route}");
    let mut req = match method {
        "GET" => ureq::get(&url),
        "POST" => ureq::post(&url),
        "DELETE" => ureq::delete(&url),
        _ => bail!("unsupported method {method}"),
    };
    if !token.is_empty() {
        req = req.set("Authorization", &format!("Bearer {token}"));
    }
    let resp = if let Some(b) = body {
        req.set("Content-Type", "application/json")
            .send_json(b.clone())
    } else {
        req.call()
    };
    match resp {
        Ok(r) => {
            let st = r.status();
            let v = r.into_json::<Value>().unwrap_or_else(|_| json!({}));
            Ok((st, v))
        }
        Err(ureq::Error::Status(st, r)) => {
            let v = r.into_json::<Value>().unwrap_or_else(|_| json!({}));
            Ok((st, v))
        }
        Err(e) => Err(e.into()),
    }
}

fn rpc_call(
    stdin: &mut std::process::ChildStdin,
    reader: &mut BufReader<std::process::ChildStdout>,
    out: &mut std::fs::File,
    next_id: &mut i64,
    method: &str,
    params: Value,
) -> Result<Value> {
    let id = *next_id;
    *next_id += 1;
    writeln!(
        stdin,
        "{}",
        json!({ "jsonrpc": "2.0", "id": id, "method": method, "params": params })
    )?;
    stdin.flush()?;
    let mut line = String::new();
    while reader.read_line(&mut line)? > 0 {
        if let Ok(v) = serde_json::from_str::<Value>(line.trim()) {
            if v.get("id").and_then(Value::as_i64) == Some(id) && v.get("method").is_none() {
                return Ok(v);
            }
            if v.get("method").is_some() {
                writeln!(out, "{v}")?;
            }
        }
        line.clear();
    }
    bail!("EOF waiting for {method}");
}

pub fn run_poc_dump(codex_bin: &str, codex_home: &str, cwd: &str, out_path: &Path) -> Result<()> {
    let mut child = Command::new(codex_bin)
        .args(["-c", "thread_unload_delay_secs=0", "app-server"])
        .env("CODEX_HOME", codex_home)
        .env("RUST_LOG", "error")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    let mut stdin = child.stdin.take().context("stdin")?;
    let stdout = child.stdout.take().context("stdout")?;
    let mut reader = BufReader::new(stdout);
    let mut out = std::fs::File::create(out_path)?;

    let mut next_id = 1_i64;
    rpc_call(
        &mut stdin,
        &mut reader,
        &mut out,
        &mut next_id,
        "initialize",
        json!({ "clientInfo": { "name": "poc-dump", "version": "0" }, "capabilities": { "experimentalApi": true } }),
    )?;
    writeln!(
        stdin,
        "{}",
        json!({ "jsonrpc": "2.0", "method": "initialized", "params": {} })
    )?;
    let started = rpc_call(
        &mut stdin,
        &mut reader,
        &mut out,
        &mut next_id,
        "thread/start",
        json!({ "cwd": cwd, "model": "mock-model", "approvalPolicy": "on-request", "sandbox": "read-only" }),
    )?;
    let tid = started["result"]["thread"]["id"].as_str().unwrap_or("");
    let _ = rpc_call(
        &mut stdin,
        &mut reader,
        &mut out,
        &mut next_id,
        "turn/start",
        json!({ "threadId": tid, "input": [{ "type": "text", "text": "RUN 通知结构" }] }),
    )?;
    let _ = child.kill();
    Ok(())
}

pub fn run_poc_turn(codex_bin: &str, codex_home: &str, cwd: &str) -> Result<String> {
    let sidecar = SidecarClient::start(codex_bin, codex_home)?;
    let started = sidecar.call("thread/start", json!({ "cwd": cwd }))?;
    let tid = started["result"]["thread"]["id"]
        .as_str()
        .context("no threadId")?
        .to_string();
    let _ = sidecar.call(
        "turn/start",
        json!({ "threadId": tid, "input": [{ "type": "text", "text": "RUN 后台执行检查" }] }),
    )?;
    let deadline = Instant::now() + Duration::from_secs(60);
    while Instant::now() < deadline {
        let read = sidecar.call(
            "thread/read",
            json!({ "threadId": tid, "includeTurns": true }),
        )?;
        if let Some(status) = read["result"]["thread"]["turns"]
            .as_array()
            .and_then(|t| t.last())
            .and_then(|t| t.get("status"))
            .and_then(Value::as_str)
            && status != "inProgress"
        {
            break;
        }
        std::thread::sleep(Duration::from_millis(500));
    }
    sidecar.close();
    Ok(tid)
}

pub fn run_p0(home: &str, work: &str, codex_bin: &str, cdp_port: u16) -> Result<Vec<CheckResult>> {
    ensure_isolated_home(home)?;
    let mut results = Vec::new();
    let sidecar = SidecarClient::start(codex_bin, home)?;
    let ipc = Arc::new(IpcClient::new(
        default_socket_path(Some(home)),
        "codex-plus-remote-p0",
    ));
    let cid = ipc.connect()?;
    results.push(CheckResult {
        name: "IPC 握手".into(),
        ok: !cid.is_empty(),
        detail: cid.chars().take(8).collect(),
    });

    let tid = sidecar.create_thread(work, "P0 Rust Check")?;
    let loaded_after_create = sidecar.is_loaded(&tid)?;
    results.push(CheckResult {
        name: "侧车建会话后放掉写锁".into(),
        ok: !loaded_after_create,
        detail: String::new(),
    });

    let _ = sidecar.call(
        "thread/read",
        json!({ "threadId": tid, "includeTurns": true }),
    )?;
    let loaded_after_read = sidecar.is_loaded(&tid)?;
    results.push(CheckResult {
        name: "侧车读会话不占写锁".into(),
        ok: !loaded_after_read,
        detail: String::new(),
    });

    let _ = cdp_eval(
        cdp_port,
        &format!(
            r#"window.dispatchEvent(new MessageEvent("message", {{ data: {{ type: "navigate-to-route", path: "/local/{tid}" }} }}));"#
        ),
    );
    let follower = ThreadFollower::attach(Arc::clone(&ipc), &tid, "local");
    follower.start()?;
    let _ = follower.start_turn("你好，来自 P0 Rust", None)?;
    follower.stop()?;
    sidecar.close();
    ipc.close();
    Ok(results)
}

pub fn run_http_suite_smoke(suite: &str, base: &str, token: &str) -> Result<Vec<CheckResult>> {
    let mut results = Vec::new();
    let unauth = api_json("GET", base, "/status", "", None)?;
    results.push(CheckResult {
        name: format!("{suite}: 未授权请求被拒"),
        ok: unauth.0 == 401,
        detail: format!("HTTP {}", unauth.0),
    });
    let status = api_json("GET", base, "/status", token, None)?;
    results.push(CheckResult {
        name: format!("{suite}: 状态接口可达"),
        ok: status.0 == 200,
        detail: format!("HTTP {}", status.0),
    });
    Ok(results)
}
