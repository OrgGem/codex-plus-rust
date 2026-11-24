//! Length-prefixed JSON IPC client ported from `ipc-client.mjs`.

use anyhow::{Context, Result, bail, ensure};
use serde_json::{Value, json};
use std::collections::HashMap;
#[cfg(windows)]
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, mpsc};
use std::time::{Duration, Instant};

pub const MAX_FRAME_BYTES: usize = 64 * 1024 * 1024;

pub const DEFAULT_VERSIONS: &[(&str, u64)] = &[
    ("thread-stream-state-changed", 13),
    ("thread-stream-following-changed", 1),
    ("thread-stream-following-status-requested", 1),
    ("ipc-connection-reset", 1),
    ("thread-read-state-changed", 3),
    ("thread-archived", 2),
    ("thread-unarchived", 1),
    ("thread-owner-discovery", 1),
    ("thread-follower-start-turn", 2),
    ("thread-follower-load-complete-history", 1),
    ("thread-follower-compact-thread", 1),
    ("thread-follower-steer-turn", 1),
    ("thread-follower-interrupt-turn", 4),
    ("thread-follower-update-thread-settings", 2),
    ("thread-follower-update-daybreak", 1),
    ("thread-follower-read-model-settings", 1),
    ("thread-follower-edit-last-user-turn", 2),
    ("thread-follower-command-approval-decision", 1),
    ("thread-follower-file-approval-decision", 1),
    ("thread-follower-permissions-request-approval-response", 1),
    ("thread-follower-submit-user-input", 1),
    ("thread-follower-submit-mcp-server-elicitation-response", 1),
    ("thread-follower-set-queued-follow-ups-state", 3),
    ("thread-follower-remove-queued-message", 1),
    ("thread-follower-clear-queued-messages", 1),
    ("thread-queued-followups-changed", 3),
];

pub fn default_versions_map() -> HashMap<String, u64> {
    DEFAULT_VERSIONS
        .iter()
        .map(|(k, v)| ((*k).to_string(), *v))
        .collect()
}

pub fn default_socket_path(codex_home: Option<&str>) -> String {
    if cfg!(windows) {
        return r"\\.\pipe\codex-ipc".into();
    }
    let home = codex_home
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("CODEX_HOME").map(PathBuf::from))
        .unwrap_or_else(|| {
            let base = std::env::var_os("HOME")
                .or_else(|| std::env::var_os("USERPROFILE"))
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("."));
            base.join(".codex")
        });
    home.join("ipc")
        .join("ipc.sock")
        .to_string_lossy()
        .into_owned()
}

pub fn frame_message(msg: &Value) -> Result<Vec<u8>> {
    let body = serde_json::to_vec(msg)?;
    ensure!(
        body.len() <= MAX_FRAME_BYTES,
        "IPC frame size {} exceeds {} byte cap",
        body.len(),
        MAX_FRAME_BYTES
    );
    let len = u32::try_from(body.len()).context("frame too large")?;
    let mut out = Vec::with_capacity(4 + body.len());
    out.extend_from_slice(&len.to_le_bytes());
    out.extend_from_slice(&body);
    Ok(out)
}

pub fn read_frame(r: &mut impl Read) -> Result<Value> {
    let mut head = [0_u8; 4];
    r.read_exact(&mut head)?;
    let size = usize::try_from(u32::from_le_bytes(head)).context("IPC frame size overflow")?;
    ensure!(
        size <= MAX_FRAME_BYTES,
        "IPC frame size {size} exceeds {MAX_FRAME_BYTES} byte cap"
    );
    let mut body = vec![0_u8; size];
    r.read_exact(&mut body)?;
    Ok(serde_json::from_slice(&body)?)
}

pub fn uuid_v4() -> String {
    let mut b = [0_u8; 16];
    let _ = getrandom::getrandom(&mut b);
    b[6] = (b[6] & 0x0f) | 0x40;
    b[8] = (b[8] & 0x3f) | 0x80;
    format!(
        "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        b[0],
        b[1],
        b[2],
        b[3],
        b[4],
        b[5],
        b[6],
        b[7],
        b[8],
        b[9],
        b[10],
        b[11],
        b[12],
        b[13],
        b[14],
        b[15]
    )
}

pub type BroadcastListener = Arc<dyn Fn(&Value) + Send + Sync>;

trait ReadWritePair {
    fn open_streams(target: &str) -> Result<(Box<dyn Read + Send>, Box<dyn Write + Send>)>;
}

struct DefaultTransport;
impl ReadWritePair for DefaultTransport {
    fn open_streams(target: &str) -> Result<(Box<dyn Read + Send>, Box<dyn Write + Send>)> {
        if let Some(addr) = target.strip_prefix("tcp://") {
            let stream = TcpStream::connect(addr)?;
            let writer = stream.try_clone()?;
            return Ok((Box::new(stream), Box::new(writer)));
        }
        #[cfg(windows)]
        {
            let file = OpenOptions::new().read(true).write(true).open(target)?;
            let writer = file.try_clone()?;
            Ok((Box::new(file), Box::new(writer)))
        }
        #[cfg(unix)]
        {
            let stream = std::os::unix::net::UnixStream::connect(target)?;
            let writer = stream.try_clone()?;
            Ok((Box::new(stream), Box::new(writer)))
        }
        #[cfg(not(any(windows, unix)))]
        {
            bail!("IPC socket transport {target} requires windows/unix build or tcp:// endpoint")
        }
    }
}

pub struct IpcClient {
    socket_path: String,
    client_type: String,
    versions: HashMap<String, u64>,
    pub client_id: Mutex<String>,
    writer: Arc<Mutex<Option<Box<dyn Write + Send>>>>,
    pending: Arc<Mutex<HashMap<String, mpsc::Sender<Value>>>>,
    listeners: Arc<Mutex<Vec<(String, BroadcastListener)>>>,
    closed: Arc<AtomicBool>,
}

impl IpcClient {
    pub fn new(socket_path: impl Into<String>, client_type: impl Into<String>) -> Self {
        Self {
            socket_path: socket_path.into(),
            client_type: client_type.into(),
            versions: default_versions_map(),
            client_id: Mutex::new("initializing-client".into()),
            writer: Arc::new(Mutex::new(None)),
            pending: Arc::new(Mutex::new(HashMap::new())),
            listeners: Arc::new(Mutex::new(Vec::new())),
            closed: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn on_method(&self, method: impl Into<String>, cb: BroadcastListener) {
        if let Ok(mut g) = self.listeners.lock() {
            g.push((method.into(), cb));
        }
    }

    pub fn write_msg(&self, obj: &Value) -> Result<()> {
        let bytes = frame_message(obj)?;
        let mut g = self.writer.lock().map_err(|e| anyhow::anyhow!("{e}"))?;
        let Some(w) = g.as_mut() else {
            bail!("IPC socket not connected");
        };
        w.write_all(&bytes)?;
        w.flush()?;
        Ok(())
    }

    pub fn connect(&self) -> Result<String> {
        let (mut reader, writer) = DefaultTransport::open_streams(&self.socket_path)?;
        *self.writer.lock().map_err(|e| anyhow::anyhow!("{e}"))? = Some(writer);
        let pending_clone = Arc::clone(&self.pending);
        let listeners_clone = Arc::clone(&self.listeners);
        let writer_clone = Arc::clone(&self.writer);
        let closed = Arc::clone(&self.closed);
        let (auto_tx, auto_rx) = mpsc::channel::<Value>();

        std::thread::spawn(move || {
            for msg in auto_rx {
                if let Ok(bytes) = frame_message(&msg)
                    && let Ok(mut g) = writer_clone.lock()
                    && let Some(w) = g.as_mut()
                {
                    let _ = w.write_all(&bytes);
                    let _ = w.flush();
                }
            }
        });

        std::thread::spawn(move || {
            while !closed.load(Ordering::Relaxed) {
                let msg = match read_frame(&mut reader) {
                    Ok(m) => m,
                    Err(_) => break,
                };
                let ty = msg.get("type").and_then(Value::as_str).unwrap_or("");
                if ty == "response"
                    && let Some(rid) = msg.get("requestId").and_then(Value::as_str)
                    && let Some(tx) = pending_clone.lock().ok().and_then(|mut m| m.remove(rid))
                {
                    let _ = tx.send(msg);
                    continue;
                }
                if ty == "client-discovery-request"
                    && let Some(rid) = msg.get("requestId").cloned()
                {
                    let _ = auto_tx.send(json!({
                        "type": "client-discovery-response",
                        "requestId": rid,
                        "response": { "canHandle": false }
                    }));
                    continue;
                }
                if ty == "request"
                    && let Some(rid) = msg.get("requestId").cloned()
                {
                    let _ = auto_tx.send(json!({
                        "type": "response",
                        "requestId": rid,
                        "resultType": "error",
                        "error": "unsupported"
                    }));
                    continue;
                }
                if ty == "broadcast"
                    && let Some(method) = msg.get("method").and_then(Value::as_str)
                    && let Ok(g) = listeners_clone.lock()
                {
                    for (pat, cb) in g.iter() {
                        if pat == "*" || pat == method {
                            cb(&msg);
                        }
                    }
                }
            }
        });

        let init = self.request(
            "initialize",
            json!({ "clientType": self.client_type }),
            None,
            None,
            10_000,
        )?;
        if init.get("resultType").and_then(Value::as_str) != Some("success") {
            bail!("initialize failed: {init}");
        }
        let cid = init
            .get("result")
            .and_then(|r| r.get("clientId"))
            .and_then(Value::as_str)
            .context("missing clientId")?
            .to_string();
        *self.client_id.lock().map_err(|e| anyhow::anyhow!("{e}"))? = cid.clone();
        Ok(cid)
    }

    pub fn request(
        &self,
        method: &str,
        params: Value,
        target_client_id: Option<&str>,
        host_id: Option<&str>,
        timeout_ms: u64,
    ) -> Result<Value> {
        let request_id = uuid_v4();
        let (tx, rx) = mpsc::channel();
        if let Ok(mut g) = self.pending.lock() {
            g.insert(request_id.clone(), tx);
        }
        let cid = self.client_id.lock().map(|g| g.clone()).unwrap_or_default();
        let ver = self.versions.get(method).copied().unwrap_or(0);
        let mut msg = json!({
            "type": "request",
            "requestId": request_id,
            "sourceClientId": cid,
            "version": ver,
            "method": method,
            "params": params,
            "timeoutMs": timeout_ms,
        });
        if let Some(t) = target_client_id {
            msg["targetClientId"] = json!(t);
        }
        if let Some(h) = host_id {
            msg["hostId"] = json!(h);
        }
        self.write_msg(&msg)?;
        let deadline = Instant::now() + Duration::from_millis(timeout_ms + 2000);
        match rx.recv_timeout(deadline.saturating_duration_since(Instant::now())) {
            Ok(v) => Ok(v),
            Err(_) => {
                if let Ok(mut g) = self.pending.lock() {
                    g.remove(&request_id);
                }
                Ok(json!({ "resultType": "error", "error": "local-timeout" }))
            }
        }
    }

    pub fn broadcast(
        &self,
        method: &str,
        params: Value,
        target_client_ids: Option<Vec<String>>,
    ) -> Result<()> {
        let cid = self.client_id.lock().map(|g| g.clone()).unwrap_or_default();
        let ver = self.versions.get(method).copied().unwrap_or(0);
        let mut msg = json!({
            "type": "broadcast",
            "method": method,
            "sourceClientId": cid,
            "version": ver,
            "params": params,
        });
        if let Some(t) = target_client_ids {
            msg["targetClientIds"] = json!(t);
        }
        self.write_msg(&msg)
    }

    pub fn close(&self) {
        self.closed.store(true, Ordering::Relaxed);
        if let Ok(mut g) = self.pending.lock() {
            g.clear();
        }
        if let Ok(mut g) = self.writer.lock() {
            let _ = g.take();
        }
    }
}
