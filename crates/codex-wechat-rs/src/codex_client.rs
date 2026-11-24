//! Persistent `codex app-server --stdio` JSON-RPC client and `codex exec` fallback.

use crate::config::{BridgeConfig, CodexConfig};
use crate::protocol::{build_wechat_prompt, clean_codex_output};
use anyhow::{Context, Result, bail};
use serde_json::{Value, json};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::sync::{Mutex, mpsc};
use std::time::{Duration, Instant};

/// Resolve npm `.cmd` wrapper on Windows to the native vendor `codex.exe` when present.
pub fn resolve_app_server_command(codex: &CodexConfig) -> String {
    if !codex.app_server_command.is_empty() {
        return codex.app_server_command.clone();
    }
    let command = &codex.command;
    let lower = command.replace('\\', "/").to_ascii_lowercase();
    if lower.ends_with("/codex.cmd")
        && let Some(npm_dir) = Path::new(command).parent()
    {
        let native = npm_dir
            .join("node_modules")
            .join("@openai")
            .join("codex")
            .join("node_modules")
            .join("@openai")
            .join("codex-win32-x64")
            .join("vendor")
            .join("x86_64-pc-windows-msvc")
            .join("bin")
            .join("codex.exe");
        if native.exists() {
            return native.to_string_lossy().into_owned();
        }
    }
    command.clone()
}

fn spawn_command(program: &str, args: &[&str]) -> std::io::Result<Child> {
    Command::new(program)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
}

pub fn run_codex_exec(cfg: &BridgeConfig, prompt: &str) -> String {
    let codex = &cfg.codex;
    let full_prompt = build_wechat_prompt(codex, prompt);
    let mut args: Vec<String> = vec![
        "--ask-for-approval".into(),
        codex.approval_policy.clone(),
        "exec".into(),
        "-C".into(),
        codex.workspace.clone(),
        "--sandbox".into(),
        codex.sandbox.clone(),
    ];
    if !codex.model.is_empty() {
        args.extend(["--model".into(), codex.model.clone()]);
    }
    if !codex.profile.is_empty() {
        args.extend(["--profile".into(), codex.profile.clone()]);
    }
    args.extend(codex.extra_args.iter().cloned());
    args.push("-".into());

    let arg_refs: Vec<&str> = args.iter().map(String::as_str).collect();
    let mut child = match spawn_command(&codex.command, &arg_refs) {
        Ok(c) => c,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            return format!("找不到 Codex 命令：{}", codex.command);
        }
        Err(e) => return format!("Codex 执行失败：{e}"),
    };

    if let Some(mut stdin) = child.stdin.take() {
        let prompt_bytes = full_prompt.into_bytes();
        std::thread::spawn(move || {
            let _ = stdin.write_all(&prompt_bytes);
            let _ = stdin.flush();
        });
    }
    let stdout_pipe = child.stdout.take();
    let stderr_pipe = child.stderr.take();
    let out_reader = std::thread::spawn(move || {
        let mut buf = Vec::new();
        if let Some(mut s) = stdout_pipe {
            let _ = s.read_to_end(&mut buf);
        }
        buf
    });
    let err_reader = std::thread::spawn(move || {
        let mut buf = Vec::new();
        if let Some(mut s) = stderr_pipe {
            let _ = s.read_to_end(&mut buf);
        }
        buf
    });

    let timeout = Duration::from_secs(codex.timeout_seconds.max(1));
    let start = Instant::now();
    let status = loop {
        match child.try_wait() {
            Ok(Some(st)) => break st,
            Ok(None) if start.elapsed() >= timeout => {
                let _ = child.kill();
                let _ = child.wait();
                let _ = out_reader.join();
                let _ = err_reader.join();
                return format!("Codex 执行超时（>{}s）。", codex.timeout_seconds);
            }
            Ok(None) => std::thread::sleep(Duration::from_millis(50)),
            Err(e) => {
                let _ = child.kill();
                let _ = child.wait();
                let _ = out_reader.join();
                let _ = err_reader.join();
                return format!("Codex 执行失败：{e}");
            }
        }
    };

    let stdout_bytes = out_reader.join().unwrap_or_default();
    let stderr_bytes = err_reader.join().unwrap_or_default();
    let stdout = String::from_utf8_lossy(&stdout_bytes);
    let stderr = String::from_utf8_lossy(&stderr_bytes);
    let cleaned = clean_codex_output(&stdout, &stderr);
    if !status.success() {
        if !cleaned.is_empty() {
            cleaned
        } else {
            format!("Codex 执行失败，退出码 {}。", status.code().unwrap_or(-1))
        }
    } else if !cleaned.is_empty() {
        cleaned
    } else {
        "Codex 没有返回内容。".into()
    }
}

struct ClientInner {
    cfg: BridgeConfig,
    command: String,
    proc: Option<Child>,
    rx: Option<mpsc::Receiver<Value>>,
    next_id: i64,
    threads: HashMap<String, String>,
}

impl ClientInner {
    fn close(&mut self) {
        if let Some(mut p) = self.proc.take() {
            let _ = p.kill();
            let _ = p.wait();
        }
        self.rx = None;
        self.threads.clear();
    }

    fn ensure_started(&mut self) -> Result<()> {
        if let Some(p) = self.proc.as_mut()
            && matches!(p.try_wait(), Ok(None))
        {
            return Ok(());
        }
        self.close();
        self.threads.clear();
        let mut child = spawn_command(&self.command, &["app-server", "--stdio"])
            .with_context(|| format!("failed to launch {} app-server --stdio", self.command))?;
        let stdout = child.stdout.take().context("missing stdout pipe")?;
        let stderr = child.stderr.take().context("missing stderr pipe")?;
        let (tx, rx) = mpsc::channel();
        std::thread::spawn(move || {
            let reader = BufReader::new(stdout);
            for line in reader.lines().map_while(Result::ok) {
                let t = line.trim();
                if t.is_empty() {
                    continue;
                }
                if let Ok(val) = serde_json::from_str::<Value>(t)
                    && tx.send(val).is_err()
                {
                    break;
                }
            }
        });
        std::thread::spawn(move || {
            let reader = BufReader::new(stderr);
            for _ in reader.lines().map_while(Result::ok) {}
        });
        self.proc = Some(child);
        self.rx = Some(rx);
        if let Err(e) = self.request(
            "initialize",
            json!({
                "clientInfo": { "name": "codex-wechat", "version": "0.1.0" },
                "capabilities": Value::Null
            }),
            20,
        ) {
            self.close();
            return Err(e);
        }
        Ok(())
    }

    fn send(&mut self, msg: &Value) -> Result<()> {
        let Some(p) = self.proc.as_mut() else {
            bail!("app-server 未启动");
        };
        let Some(stdin) = p.stdin.as_mut() else {
            bail!("app-server 未启动");
        };
        serde_json::to_writer(&mut *stdin, msg)?;
        stdin.write_all(b"\n")?;
        stdin.flush()?;
        Ok(())
    }

    fn request(&mut self, method: &str, params: Value, timeout_secs: u64) -> Result<Value> {
        let req_id = self.next_id;
        self.next_id += 1;
        self.send(&json!({ "id": req_id, "method": method, "params": params }))?;
        let deadline = Instant::now() + Duration::from_secs(timeout_secs);
        loop {
            if Instant::now() >= deadline {
                bail!("{method} 超时");
            }
            let recv = self
                .rx
                .as_ref()
                .context("missing stdout receiver")?
                .recv_timeout(Duration::from_millis(500));
            match recv {
                Ok(msg) => {
                    if msg.get("id").and_then(Value::as_i64) == Some(req_id) {
                        if let Some(err) = msg.get("error") {
                            bail!("{method} 失败：{err}");
                        }
                        return Ok(msg.get("result").cloned().unwrap_or_else(|| json!({})));
                    }
                }
                Err(mpsc::RecvTimeoutError::Timeout) => {
                    if let Some(p) = self.proc.as_mut()
                        && let Ok(Some(status)) = p.try_wait()
                    {
                        bail!("app-server 已退出，退出码 {}", status.code().unwrap_or(-1));
                    }
                }
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    bail!("app-server stdout disconnected")
                }
            }
        }
    }

    fn ensure_thread(&mut self, user_id: &str) -> Result<String> {
        if let Some(tid) = self.threads.get(user_id) {
            return Ok(tid.clone());
        }
        let mut params = json!({
            "cwd": self.cfg.codex.workspace,
            "approvalPolicy": self.cfg.codex.approval_policy,
            "sandbox": self.cfg.codex.sandbox,
            "ephemeral": true,
            "baseInstructions": "你是运行在微信里的 Codex 助手。全程中文，只把最终答复发给微信用户。"
        });
        if !self.cfg.codex.model.is_empty() {
            params["model"] = json!(self.cfg.codex.model);
        }
        let result = self.request("thread/start", params, 60)?;
        let tid = result
            .get("thread")
            .and_then(|t| t.get("id"))
            .and_then(Value::as_str)
            .context("missing thread.id from thread/start")?
            .to_string();
        self.threads.insert(user_id.to_string(), tid.clone());
        Ok(tid)
    }

    fn run_turn(&mut self, user_id: &str, prompt: &str) -> Result<String> {
        self.ensure_started()?;
        let thread_id = self.ensure_thread(user_id)?;
        let full_prompt = build_wechat_prompt(&self.cfg.codex, prompt);
        let result = self.request(
            "turn/start",
            json!({
                "threadId": thread_id,
                "input": [{ "type": "text", "text": full_prompt, "text_elements": [] }],
                "approvalPolicy": self.cfg.codex.approval_policy,
                "cwd": self.cfg.codex.workspace
            }),
            30,
        )?;
        let turn_id = result
            .get("turn")
            .and_then(|t| t.get("id"))
            .and_then(Value::as_str)
            .context("missing turn.id from turn/start")?
            .to_string();
        self.collect_turn(&thread_id, &turn_id)
    }

    fn collect_turn(&mut self, thread_id: &str, turn_id: &str) -> Result<String> {
        let deadline = Instant::now() + Duration::from_secs(self.cfg.codex.timeout_seconds.max(1));
        let mut final_item_id = String::new();
        let mut final_chunks: Vec<String> = Vec::new();
        let mut completed = false;
        let mut error = String::new();

        while Instant::now() < deadline && !completed {
            let recv = self
                .rx
                .as_ref()
                .context("missing stdout receiver")?
                .recv_timeout(Duration::from_millis(500));
            let msg = match recv {
                Ok(m) => m,
                Err(mpsc::RecvTimeoutError::Timeout) => {
                    if let Some(p) = self.proc.as_mut()
                        && let Ok(Some(status)) = p.try_wait()
                    {
                        bail!("app-server 已退出，退出码 {}", status.code().unwrap_or(-1));
                    }
                    continue;
                }
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    bail!("app-server stdout disconnected")
                }
            };

            let method = msg.get("method").and_then(Value::as_str).unwrap_or("");
            let params = msg.get("params").unwrap_or(&Value::Null);
            if params.get("threadId").and_then(Value::as_str) != Some(thread_id) {
                continue;
            }
            match method {
                "item/started" => {
                    let item = params.get("item").unwrap_or(&Value::Null);
                    if item.get("type").and_then(Value::as_str) == Some("agentMessage")
                        && item.get("phase").and_then(Value::as_str) == Some("final_answer")
                    {
                        final_item_id = item.get("id").and_then(Value::as_str).unwrap_or("").into();
                    }
                }
                "item/agentMessage/delta" => {
                    if !final_item_id.is_empty()
                        && params.get("turnId").and_then(Value::as_str) == Some(turn_id)
                        && params.get("itemId").and_then(Value::as_str)
                            == Some(final_item_id.as_str())
                    {
                        final_chunks.push(
                            params
                                .get("delta")
                                .and_then(Value::as_str)
                                .unwrap_or("")
                                .into(),
                        );
                    }
                }
                "item/completed" => {
                    let item = params.get("item").unwrap_or(&Value::Null);
                    if item.get("type").and_then(Value::as_str) == Some("agentMessage")
                        && item.get("phase").and_then(Value::as_str) == Some("final_answer")
                    {
                        let text = item.get("text").and_then(Value::as_str).unwrap_or("");
                        if !text.is_empty() {
                            final_chunks = vec![text.to_string()];
                        }
                    }
                }
                "turn/completed"
                    if params
                        .get("turn")
                        .and_then(|t| t.get("id"))
                        .and_then(Value::as_str)
                        == Some(turn_id) =>
                {
                    let turn = params.get("turn").unwrap_or(&Value::Null);
                    if turn.get("status").and_then(Value::as_str) == Some("failed") {
                        error = turn
                            .get("error")
                            .and_then(|e| e.get("message"))
                            .and_then(Value::as_str)
                            .unwrap_or("turn failed")
                            .into();
                    }
                    completed = true;
                }
                _ => {}
            }
        }
        if !completed {
            bail!("app-server turn 超时");
        }
        if !error.is_empty() {
            bail!("{error}");
        }
        let reply = final_chunks.concat().trim().to_string();
        Ok(if reply.is_empty() {
            "Codex 没有返回最终答复。".into()
        } else {
            reply
        })
    }
}

pub struct AppServerCodexClient {
    inner: Mutex<ClientInner>,
}

impl AppServerCodexClient {
    pub fn new(cfg: BridgeConfig) -> Self {
        let command = resolve_app_server_command(&cfg.codex);
        Self {
            inner: Mutex::new(ClientInner {
                cfg,
                command,
                proc: None,
                rx: None,
                next_id: 1,
                threads: HashMap::new(),
            }),
        }
    }

    pub fn run(&self, user_id: &str, prompt: &str) -> Result<String> {
        let mut guard = self.inner.lock().map_err(|e| anyhow::anyhow!("{e}"))?;
        guard.run_turn(user_id, prompt)
    }

    pub fn close(&self) {
        if let Ok(mut guard) = self.inner.lock() {
            guard.close();
        }
    }
}

impl Drop for AppServerCodexClient {
    fn drop(&mut self) {
        self.close();
    }
}

pub struct CodexRunner {
    cfg: BridgeConfig,
    app_server: Option<AppServerCodexClient>,
}

impl CodexRunner {
    pub fn new(cfg: BridgeConfig) -> Self {
        let app_server = if cfg.codex.backend == "app-server" {
            Some(AppServerCodexClient::new(cfg.clone()))
        } else {
            None
        };
        Self { cfg, app_server }
    }

    pub fn run(&self, user_id: &str, prompt: &str) -> String {
        if let Some(client) = &self.app_server {
            match client.run(user_id, prompt) {
                Ok(reply) => return reply,
                Err(e) => {
                    eprintln!("[wechat] app-server 调用失败，回退 exec：{e}");
                }
            }
        }
        run_codex_exec(&self.cfg, prompt)
    }
}
