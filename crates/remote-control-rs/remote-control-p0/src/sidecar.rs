//! Sidecar `app-server` process client ported from `sidecar.mjs`.

use anyhow::{Context, Result, bail};
use serde_json::{Value, json};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::{Arc, Mutex, mpsc};
use std::time::{Duration, Instant};

pub struct SidecarClient {
    child: Mutex<Option<Child>>,
    stdin: Arc<Mutex<ChildStdin>>,
    pending: Arc<Mutex<HashMap<i64, mpsc::Sender<Value>>>>,
    next_id: Mutex<i64>,
}

impl SidecarClient {
    pub fn start(codex_bin: &str, codex_home: &str) -> Result<Self> {
        let args = ["-c", "thread_unload_delay_secs=0", "app-server"];
        let mut cmd = Command::new(codex_bin);
        cmd.args(args)
            .env("CODEX_HOME", codex_home)
            .env("RUST_LOG", "error")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        let mut child = cmd
            .spawn()
            .with_context(|| format!("failed to spawn {codex_bin}"))?;
        let stdout = child.stdout.take().context("missing stdout")?;
        let stderr = child.stderr.take().context("missing stderr")?;
        let stdin = Arc::new(Mutex::new(child.stdin.take().context("missing stdin")?));

        let pending: Arc<Mutex<HashMap<i64, mpsc::Sender<Value>>>> =
            Arc::new(Mutex::new(HashMap::new()));
        let (srv_req_tx, srv_req_rx) = mpsc::channel::<Value>();
        let p_clone = Arc::clone(&pending);

        std::thread::spawn(move || {
            let reader = BufReader::new(stdout);
            for line in reader.lines().map_while(Result::ok) {
                let t = line.trim();
                if t.is_empty() {
                    continue;
                }
                let Ok(msg) = serde_json::from_str::<Value>(t) else {
                    continue;
                };
                let id = msg.get("id");
                let method = msg.get("method").and_then(Value::as_str);
                if let Some(id_num) = id.and_then(Value::as_i64)
                    && method.is_none()
                    && let Some(tx) = p_clone.lock().ok().and_then(|mut m| m.remove(&id_num))
                {
                    let _ = tx.send(msg);
                } else if method.is_some() && id.is_some_and(|v| !v.is_null()) {
                    let _ = srv_req_tx.send(json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "error": { "code": -32601, "message": "sidecar does not handle server requests" }
                    }));
                }
            }
        });
        let stdin_for_srv = Arc::clone(&stdin);
        std::thread::spawn(move || {
            for reply in srv_req_rx {
                if let Ok(mut w) = stdin_for_srv.lock() {
                    if serde_json::to_writer(&mut *w, &reply).is_err()
                        || w.write_all(b"\n").is_err()
                        || w.flush().is_err()
                    {
                        break;
                    }
                } else {
                    break;
                }
            }
        });
        std::thread::spawn(
            move || {
                for _ in BufReader::new(stderr).lines().map_while(Result::ok) {}
            },
        );

        let client = Self {
            child: Mutex::new(Some(child)),
            stdin,
            pending,
            next_id: Mutex::new(1),
        };

        if let Err(e) = client
            .call(
                "initialize",
                json!({ "clientInfo": { "name": "codex-plus-remote-sidecar", "version": "0" }, "capabilities": { "experimentalApi": true } }),
            )
            .and_then(|_| {
                client.write_raw(&json!({ "jsonrpc": "2.0", "method": "initialized", "params": {} }))
            })
        {
            client.close();
            return Err(e);
        }
        Ok(client)
    }

    fn write_raw(&self, val: &Value) -> Result<()> {
        let mut stdin = self.stdin.lock().map_err(|e| anyhow::anyhow!("{e}"))?;
        serde_json::to_writer(&mut *stdin, val)?;
        stdin.write_all(b"\n")?;
        stdin.flush()?;
        Ok(())
    }

    pub fn call(&self, method: &str, params: Value) -> Result<Value> {
        let id = {
            let mut g = self.next_id.lock().map_err(|e| anyhow::anyhow!("{e}"))?;
            let cur = *g;
            *g += 1;
            cur
        };
        let (tx, rx) = mpsc::channel();
        if let Ok(mut g) = self.pending.lock() {
            g.insert(id, tx);
        }
        self.write_raw(&json!({ "jsonrpc": "2.0", "id": id, "method": method, "params": params }))?;
        rx.recv_timeout(Duration::from_secs(30))
            .context("sidecar call timeout")
    }

    pub fn is_loaded(&self, thread_id: &str) -> Result<bool> {
        let loaded = self.call("thread/loaded/list", json!({}))?;
        Ok(loaded
            .get("result")
            .and_then(|r| r.get("data"))
            .and_then(Value::as_array)
            .is_some_and(|arr| arr.iter().any(|x| x.as_str() == Some(thread_id))))
    }

    pub fn release(&self, thread_id: &str, timeout_ms: u64) -> Result<()> {
        self.call("thread/unsubscribe", json!({ "threadId": thread_id }))?;
        let deadline = Instant::now() + Duration::from_millis(timeout_ms);
        while Instant::now() < deadline {
            if !self.is_loaded(thread_id)? {
                return Ok(());
            }
            std::thread::sleep(Duration::from_millis(100));
        }
        bail!("侧车 {timeout_ms} ms 内没有卸载会话 {thread_id}")
    }

    pub fn create_thread(&self, cwd: &str, name: &str) -> Result<String> {
        let started = self.call("thread/start", json!({ "cwd": cwd }))?;
        if let Some(err) = started.get("error") {
            bail!(
                "{}",
                err.get("message")
                    .and_then(Value::as_str)
                    .unwrap_or("thread/start failed")
            );
        }
        let thread_id = started
            .get("result")
            .and_then(|r| r.get("thread"))
            .and_then(|t| t.get("id"))
            .and_then(Value::as_str)
            .context("missing thread.id")?
            .to_string();
        let named = self.call(
            "thread/name/set",
            json!({ "threadId": thread_id, "name": name }),
        )?;
        if let Some(err) = named.get("error") {
            bail!(
                "{}",
                err.get("message")
                    .and_then(Value::as_str)
                    .unwrap_or("thread/name/set failed")
            );
        }
        self.release(&thread_id, 5000)?;
        Ok(thread_id)
    }

    pub fn close(&self) {
        if let Ok(mut g) = self.child.lock()
            && let Some(mut c) = g.take()
        {
            let _ = c.kill();
            let _ = c.wait();
        }
    }
}

impl Drop for SidecarClient {
    fn drop(&mut self) {
        self.close();
    }
}
