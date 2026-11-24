//! Cloudflare Pages Worker router ported from `services/share-site/_worker.js`.
//! Security hardening (ADR-007): `POST /api/shares` is closed by default (`403`)
//! unless an explicit `create_admin_token` is configured and presented.

use crate::crypto::{EncryptedPayload, random_token_b64u, sha256_b64u, timing_safe_eq_str};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
#[cfg(not(target_arch = "wasm32"))]
use std::time::{Duration, Instant};

pub const MAX_BODY_BYTES: usize = 23_000_000;
pub const MAX_CIPHERTEXT_LENGTH: usize = 22_500_000;
pub const ALLOWED_TTLS: [u64; 3] = [86_400, 604_800, 2_592_000];

#[derive(Debug, Clone, Deserialize)]
pub struct CreateShareRequest {
    pub ttl: u64,
    pub encrypted: EncryptedPayload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareRecord {
    pub encrypted: EncryptedPayload,
    #[serde(rename = "deleteTokenHash")]
    pub delete_token_hash: String,
    #[serde(rename = "createdAt")]
    pub created_at: u64,
    #[serde(rename = "expiresAt")]
    pub expires_at: u64,
}

pub trait ShareStore: Send + Sync {
    fn get(&self, key: &str) -> Result<Option<ShareRecord>, String>;
    fn put(&self, key: &str, record: ShareRecord, expiration_ttl: u64) -> Result<(), String>;
    fn delete(&self, key: &str) -> Result<(), String>;
}

#[derive(Debug, Clone)]
struct StoredShareEntry {
    record: ShareRecord,
    #[cfg(not(target_arch = "wasm32"))]
    deadline: Instant,
}

impl StoredShareEntry {
    fn is_expired_realtime(&self) -> bool {
        #[cfg(not(target_arch = "wasm32"))]
        {
            Instant::now() >= self.deadline
        }
        #[cfg(target_arch = "wasm32")]
        {
            false
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct InMemoryShareStore {
    inner: Arc<Mutex<HashMap<String, StoredShareEntry>>>,
}

impl InMemoryShareStore {
    pub fn evict_expired(&self, now_ms: u64) -> Result<usize, String> {
        let mut guard = self.inner.lock().map_err(|e| e.to_string())?;
        let before = guard.len();
        guard.retain(|_, entry| !entry.is_expired_realtime() && entry.record.expires_at > now_ms);
        Ok(before - guard.len())
    }

    pub fn len(&self) -> Result<usize, String> {
        let guard = self.inner.lock().map_err(|e| e.to_string())?;
        Ok(guard.len())
    }

    pub fn is_empty(&self) -> Result<bool, String> {
        Ok(self.len()? == 0)
    }
}

impl ShareStore for InMemoryShareStore {
    fn get(&self, key: &str) -> Result<Option<ShareRecord>, String> {
        let mut guard = self.inner.lock().map_err(|e| e.to_string())?;
        if guard
            .get(key)
            .is_some_and(StoredShareEntry::is_expired_realtime)
        {
            guard.remove(key);
            return Ok(None);
        }
        Ok(guard.get(key).map(|e| e.record.clone()))
    }

    fn put(&self, key: &str, record: ShareRecord, expiration_ttl: u64) -> Result<(), String> {
        let mut guard = self.inner.lock().map_err(|e| e.to_string())?;
        let ref_now_ms = record.created_at;
        guard.retain(|_, entry| {
            !entry.is_expired_realtime()
                && (ref_now_ms == 0 || entry.record.expires_at > ref_now_ms)
        });
        if expiration_ttl == 0 || (record.created_at > 0 && record.expires_at <= record.created_at)
        {
            guard.remove(key);
            return Ok(());
        }
        #[cfg(not(target_arch = "wasm32"))]
        let entry = StoredShareEntry {
            record,
            deadline: Instant::now()
                .checked_add(Duration::from_secs(expiration_ttl))
                .unwrap_or_else(Instant::now),
        };
        #[cfg(target_arch = "wasm32")]
        let entry = StoredShareEntry { record };
        guard.insert(key.to_string(), entry);
        Ok(())
    }

    fn delete(&self, key: &str) -> Result<(), String> {
        let mut guard = self.inner.lock().map_err(|e| e.to_string())?;
        guard.remove(key);
        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct ShareSiteConfig {
    /// Default `None`: closes the deprecated unauthenticated `POST /api/shares` write endpoint.
    pub create_admin_token: Option<String>,
}

#[derive(Debug, Clone)]
pub struct WorkerRequest {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkerResponse {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
    pub rewritten_asset_path: Option<String>,
}

impl WorkerResponse {
    pub fn json_body(&self) -> Option<Value> {
        serde_json::from_slice(&self.body).ok()
    }
}

fn cors_headers() -> HashMap<String, String> {
    HashMap::from([
        ("Access-Control-Allow-Origin".into(), "*".into()),
        (
            "Access-Control-Allow-Methods".into(),
            "GET, POST, DELETE, OPTIONS".into(),
        ),
        (
            "Access-Control-Allow-Headers".into(),
            "Content-Type, X-Delete-Token".into(),
        ),
    ])
}

fn json_resp(body: Value, status: u16, extra: &[(&str, &str)]) -> WorkerResponse {
    let mut headers = cors_headers();
    headers.insert(
        "Content-Type".into(),
        "application/json; charset=utf-8".into(),
    );
    headers.insert("Cache-Control".into(), "no-store".into());
    headers.insert("X-Content-Type-Options".into(), "nosniff".into());
    for (k, v) in extra {
        headers.insert((*k).into(), (*v).into());
    }
    WorkerResponse {
        status,
        headers,
        body: serde_json::to_vec(&body).unwrap_or_default(),
        rewritten_asset_path: None,
    }
}

fn header_ci<'a>(headers: &'a HashMap<String, String>, name: &str) -> Option<&'a str> {
    headers
        .iter()
        .find(|(k, _)| k.eq_ignore_ascii_case(name))
        .map(|(_, v)| v.as_str())
}

fn is_valid_b64u_token(s: &str, exact_len: Option<usize>) -> bool {
    let ok_len = exact_len.map_or(!s.is_empty(), |n| s.len() == n);
    ok_len
        && s.bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
}

fn match_share_id(path: &str) -> Option<&str> {
    let id = path.strip_prefix("/api/shares/")?;
    ((20..=32).contains(&id.len()) && is_valid_b64u_token(id, None)).then_some(id)
}

fn check_create_authorized(req: &WorkerRequest, cfg: &ShareSiteConfig) -> bool {
    let Some(expected) = cfg.create_admin_token.as_deref().filter(|t| !t.is_empty()) else {
        return false;
    };
    let Some(auth) = header_ci(&req.headers, "authorization") else {
        return false;
    };
    let Some(supplied) = auth
        .strip_prefix("Bearer ")
        .or_else(|| auth.strip_prefix("bearer "))
    else {
        return false;
    };
    timing_safe_eq_str(supplied.trim(), expected)
}

pub fn handle_request(
    req: &WorkerRequest,
    store: &dyn ShareStore,
    cfg: &ShareSiteConfig,
    now_ms: u64,
) -> WorkerResponse {
    if req.method.eq_ignore_ascii_case("OPTIONS") && req.path.starts_with("/api/shares") {
        return WorkerResponse {
            status: 204,
            headers: cors_headers(),
            body: Vec::new(),
            rewritten_asset_path: None,
        };
    }
    if req.path == "/api/shares" {
        if req.method.eq_ignore_ascii_case("POST") {
            return handle_create_share(req, store, cfg, now_ms);
        }
        return json_resp(json!({ "error": "Method not allowed" }), 405, &[]);
    }
    if let Some(id) = match_share_id(&req.path) {
        if req.method.eq_ignore_ascii_case("GET") {
            return handle_get_share(id, store, now_ms);
        }
        if req.method.eq_ignore_ascii_case("DELETE") {
            return handle_delete_share(req, id, store);
        }
        return json_resp(json!({ "error": "Method not allowed" }), 405, &[]);
    }
    if req.path == "/api" || req.path.starts_with("/api/") {
        return json_resp(json!({ "error": "Not found" }), 404, &[]);
    }
    if !req.method.eq_ignore_ascii_case("GET") && !req.method.eq_ignore_ascii_case("HEAD") {
        return json_resp(json!({ "error": "Method not allowed" }), 405, &[]);
    }
    if req.path.starts_with("/s/") {
        return WorkerResponse {
            status: 200,
            headers: HashMap::new(),
            body: Vec::new(),
            rewritten_asset_path: Some("/".into()),
        };
    }
    WorkerResponse {
        status: 200,
        headers: HashMap::new(),
        body: Vec::new(),
        rewritten_asset_path: Some(req.path.clone()),
    }
}

fn handle_create_share(
    req: &WorkerRequest,
    store: &dyn ShareStore,
    cfg: &ShareSiteConfig,
    now_ms: u64,
) -> WorkerResponse {
    if !check_create_authorized(req, cfg) {
        return json_resp(
            json!({ "error": "Session share creation is disabled" }),
            403,
            &[],
        );
    }
    let ct = header_ci(&req.headers, "content-type").unwrap_or("");
    if !ct.to_ascii_lowercase().starts_with("application/json") {
        return json_resp(
            json!({ "error": "Content-Type must be application/json" }),
            415,
            &[],
        );
    }
    let content_len = header_ci(&req.headers, "content-length")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(req.body.len());
    if content_len > MAX_BODY_BYTES || req.body.len() > MAX_BODY_BYTES {
        return json_resp(json!({ "error": "Share is too large" }), 413, &[]);
    }
    let parsed = match serde_json::from_slice::<CreateShareRequest>(&req.body) {
        Ok(req_body) => req_body,
        Err(err) if err.is_data() => {
            return json_resp(json!({ "error": "Invalid encrypted payload" }), 400, &[]);
        }
        Err(_) => {
            return json_resp(json!({ "error": "Invalid JSON" }), 400, &[]);
        }
    };
    let ttl = parsed.ttl;
    let enc = parsed.encrypted;
    if !ALLOWED_TTLS.contains(&ttl)
        || enc.v != 1
        || !is_valid_b64u_token(&enc.iv, Some(16))
        || !(24..=MAX_CIPHERTEXT_LENGTH).contains(&enc.ciphertext.len())
        || !is_valid_b64u_token(&enc.ciphertext, None)
    {
        return json_resp(json!({ "error": "Invalid encrypted payload" }), 400, &[]);
    }
    let (Ok(id), Ok(delete_token)) = (random_token_b64u(16), random_token_b64u(24)) else {
        return json_resp(json!({ "error": "RNG failure" }), 500, &[]);
    };
    let expires_at = now_ms.saturating_add(ttl.saturating_mul(1000));
    let record = ShareRecord {
        encrypted: enc,
        delete_token_hash: sha256_b64u(delete_token.as_bytes()),
        created_at: now_ms,
        expires_at,
    };
    if store.put(&format!("share:{id}"), record, ttl).is_err() {
        return json_resp(json!({ "error": "Storage failure" }), 500, &[]);
    }
    json_resp(
        json!({ "id": id, "deleteToken": delete_token, "expiresAt": expires_at }),
        201,
        &[],
    )
}

fn handle_get_share(id: &str, store: &dyn ShareStore, now_ms: u64) -> WorkerResponse {
    let record = match store.get(&format!("share:{id}")) {
        Ok(Some(record)) => record,
        Ok(None) => return json_resp(json!({ "error": "Share not found or expired" }), 404, &[]),
        Err(_) => return json_resp(json!({ "error": "Storage failure" }), 500, &[]),
    };
    if record.expires_at <= now_ms {
        return json_resp(json!({ "error": "Share not found or expired" }), 404, &[]);
    }
    json_resp(
        json!({ "encrypted": record.encrypted, "createdAt": record.created_at, "expiresAt": record.expires_at }),
        200,
        &[("Cache-Control", "private, no-store")],
    )
}

fn handle_delete_share(req: &WorkerRequest, id: &str, store: &dyn ShareStore) -> WorkerResponse {
    let Some(token) =
        header_ci(&req.headers, "x-delete-token").filter(|t| !t.is_empty() && t.len() <= 64)
    else {
        return json_resp(json!({ "error": "Invalid delete token" }), 401, &[]);
    };
    let key = format!("share:{id}");
    let record = match store.get(&key) {
        Ok(Some(record)) => record,
        Ok(None) => {
            return WorkerResponse {
                status: 204,
                headers: cors_headers(),
                body: Vec::new(),
                rewritten_asset_path: None,
            };
        }
        Err(_) => return json_resp(json!({ "error": "Storage failure" }), 500, &[]),
    };
    let supplied_hash = sha256_b64u(token.as_bytes());
    if !timing_safe_eq_str(&supplied_hash, &record.delete_token_hash) {
        return json_resp(json!({ "error": "Invalid delete token" }), 403, &[]);
    }
    if store.delete(&key).is_err() {
        return json_resp(json!({ "error": "Storage failure" }), 500, &[]);
    }
    WorkerResponse {
        status: 204,
        headers: cors_headers(),
        body: Vec::new(),
        rewritten_asset_path: None,
    }
}
