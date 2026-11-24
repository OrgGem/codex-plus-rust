//! Security hardening implementations addressing Critical & High findings from code reviews:
//! - C-1: Strict WebView CSP (`frame-src`/`form-action` locked), strict navigation guard,
//!   scoped GitHub repository HTTPS allowlist + constant-time SHA-256 updater check
//! - C-2: Explicit allowlist DTO projection (`project_public_settings`) with URL credential/query
//!   stripping, `apiKeys` inspection, and strict header value redaction
//! - C-3: Renderer bridge route allowlist & extension capability risk badges
//! - C-4: Canonical session DB & SQLite-verified `threads.rollout_path` containment checks
//! - H-4.1.6 / H-4.2.3: Symlink-preserving, retry-hardened `atomic_write_unique`
//! - H-4.2.2: Non-creating read-only SQLite open + `busy_timeout(5s)` + `BEGIN IMMEDIATE`

#![forbid(unsafe_code)]

use anyhow::{Context, Result, bail};
use rusqlite::{Connection, OpenFlags, OptionalExtension, Transaction, TransactionBehavior};
use serde_json::{Map, Value, json};
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use std::fs::{self, OpenOptions};
use std::io::{ErrorKind, Write};
use std::path::{Component, Path, PathBuf};
use std::thread;
use std::time::Duration;
use subtle::ConstantTimeEq;
use url::Url;
use uuid::Uuid;

/// Hardened Content-Security-Policy injected into Dioxus Desktop WebView (`C-1`).
pub const STRICT_WEBVIEW_CSP: &str = "default-src 'self' dioxus: http://dioxus.index.html https://dioxus.index.html http://ipc.localhost https://ipc.localhost data: blob:; style-src 'self' 'unsafe-inline' dioxus:; img-src 'self' dioxus: data: blob:; font-src 'self' dioxus: data:; script-src 'self' 'unsafe-inline' 'unsafe-eval' dioxus:; connect-src 'self' dioxus: http://ipc.localhost https://ipc.localhost http://127.0.0.1:* ws://127.0.0.1:*; frame-src 'none'; child-src 'none'; form-action 'none'; object-src 'none'; frame-ancestors 'none'; base-uri 'none'";

/// Upper bound for release installer download buffered into verification (`250 MiB`).
pub const MAX_INSTALLER_BYTES: usize = 250 * 1024 * 1024;

const ALLOWED_UPDATE_HOSTS: &[&str] = &[
    "github.com",
    "objects.githubusercontent.com",
    "release-assets.githubusercontent.com",
];

const ALLOWED_GITHUB_RELEASE_PREFIXES: &[&str] = &[
    "/bigpizzav3/codexplusplus/releases/download/",
    "/gemvn90/codexplusplus/releases/download/",
];

const SENSITIVE_HEADER_KEYWORDS: &[&str] = &[
    "auth",
    "token",
    "secret",
    "key",
    "cookie",
    "password",
    "passwd",
    "pwd",
    "credential",
    "bearer",
    "session",
    "jwt",
    "private",
    "signature",
];

/// Blocks unexpected top-level WebView navigations (`C-1`).
/// Only `about:blank` and Dioxus internal document origins may be loaded as top-level pages.
pub fn is_safe_webview_navigation(target: &str) -> bool {
    let trimmed = target.trim();
    if trimmed == "about:blank" || trimmed.starts_with("dioxus:") {
        return true;
    }
    let Ok(url) = Url::parse(trimmed) else {
        return false;
    };
    url.username().is_empty()
        && url.password().is_none()
        && matches!(url.scheme(), "http" | "https")
        && url.host_str() == Some("dioxus.index.html")
}

/// Validates release installer download URLs strictly against official GitHub HTTPS origins & repo (`C-1`).
pub fn verify_release_asset_url(raw_url: &str) -> Result<Url> {
    let trimmed = raw_url.trim();
    if trimmed.chars().any(char::is_control) {
        bail!("Release URL chứa ký tự điều khiển không hợp lệ");
    }
    let parsed = Url::parse(trimmed).context("Release URL không hợp lệ")?;
    if parsed.scheme() != "https" {
        bail!(
            "Từ chối tải bản cập nhật qua giao thức không phải HTTPS: {}",
            parsed.scheme()
        );
    }
    if !parsed.username().is_empty() || parsed.password().is_some() || parsed.port().is_some() {
        bail!("Từ chối Release URL chứa thông tin xác thực hoặc cổng tùy chỉnh");
    }
    let host = parsed
        .host_str()
        .map(str::to_ascii_lowercase)
        .unwrap_or_default();
    if !ALLOWED_UPDATE_HOSTS.contains(&host.as_str()) {
        bail!("Từ chối host tải bản cập nhật không tin cậy: {host}");
    }
    if host == "github.com" {
        let lower_path = parsed.path().to_ascii_lowercase();
        if !ALLOWED_GITHUB_RELEASE_PREFIXES
            .iter()
            .any(|prefix| lower_path.starts_with(prefix))
        {
            bail!(
                "Từ chối đường dẫn GitHub Release ngoài kho chính thức: {}",
                parsed.path()
            );
        }
    }
    Ok(parsed)
}

/// Parses `SHA256SUMS.txt` lines (`<64-hex>  <filename>`) into `<filename -> lowercase hex>` (`C-1`).
pub fn parse_sha256sums(text: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let Some((digest, rest)) = trimmed.split_once(char::is_whitespace) else {
            continue;
        };
        let hex = digest.trim().to_ascii_lowercase();
        if hex.len() != 64 || !hex.bytes().all(|b| b.is_ascii_hexdigit()) {
            continue;
        }
        let name = rest.trim().trim_start_matches('*');
        let file_name = Path::new(name)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(name);
        if !file_name.is_empty() {
            map.insert(file_name.to_string(), hex);
        }
    }
    map
}

/// Computes SHA-256 and verifies size + constant-time hash equality against expected digest (`C-1`).
pub fn verify_installer_sha256(bytes: &[u8], expected_hex: &str) -> Result<()> {
    if bytes.is_empty() {
        bail!("Tệp cài đặt rỗng");
    }
    if bytes.len() > MAX_INSTALLER_BYTES {
        bail!(
            "Kích thước bộ cài vượt quá giới hạn {} bytes",
            MAX_INSTALLER_BYTES
        );
    }
    let expected_norm = expected_hex.trim().to_ascii_lowercase();
    if expected_norm.len() != 64 || !expected_norm.bytes().all(|b| b.is_ascii_hexdigit()) {
        bail!("Chuỗi SHA-256 kỳ vọng không hợp lệ");
    }
    let digest = Sha256::digest(bytes);
    let actual_hex = digest
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect::<String>();
    if bool::from(actual_hex.as_bytes().ct_eq(expected_norm.as_bytes())) {
        Ok(())
    } else {
        bail!("Kiểm tra SHA-256 thất bại (kỳ vọng {expected_norm}, thực tế {actual_hex})")
    }
}

fn sanitize_public_url(raw: &str) -> String {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return String::new();
    }
    if let Ok(mut parsed) = Url::parse(trimmed) {
        let _ = parsed.set_username("");
        let _ = parsed.set_password(None);
        parsed.set_query(None);
        parsed.set_fragment(None);
        return parsed.to_string();
    }
    let without_query = trimmed.split(['?', '#']).next().unwrap_or(trimmed);
    if let Some((_, host_rest)) = without_query.rsplit_once('@') {
        host_rest.to_string()
    } else {
        without_query.to_string()
    }
}

fn header_entry_is_sensitive(name: &str, value: &str) -> bool {
    let lower_name = name.trim().to_ascii_lowercase();
    let lower_val = value.trim_start().to_ascii_lowercase();
    SENSITIVE_HEADER_KEYWORDS
        .iter()
        .any(|kw| lower_name.contains(kw))
        || lower_val.starts_with("bearer ")
        || lower_val.starts_with("basic ")
}

fn profile_has_any_api_key(obj: &Map<String, Value>) -> bool {
    if obj
        .get("apiKey")
        .and_then(Value::as_str)
        .is_some_and(|s| !s.trim().is_empty())
    {
        return true;
    }
    obj.get("apiKeys")
        .and_then(Value::as_array)
        .is_some_and(|arr| {
            arr.iter().any(|item| {
                item.get("apiKey")
                    .and_then(Value::as_str)
                    .is_some_and(|s| !s.trim().is_empty())
            })
        })
}

fn project_public_relay_profile(profile: &Value) -> Value {
    let Some(obj) = profile.as_object() else {
        return Value::Null;
    };
    let mut out = Map::new();
    for key in [
        "id",
        "name",
        "model",
        "protocol",
        "relayMode",
        "sessionProvider",
        "hideOfficialUsageAlert",
        "useCommonConfig",
        "contextWindow",
        "autoCompactLimit",
        "modelList",
        "modelWindows",
        "modelAutoCompact",
        "modelMetadata",
        "modelRoutes",
        "sub2apiEnabled",
        "noAuth",
        "sub2apiMultiplier",
        "standardOpenaiProtocol",
        "webSearchHistoryCompat",
        "aggregate",
    ] {
        if let Some(val) = obj.get(key) {
            out.insert(key.to_string(), val.clone());
        }
    }
    for url_key in ["baseUrl", "upstreamBaseUrl"] {
        if let Some(raw_url) = obj.get(url_key).and_then(Value::as_str) {
            out.insert(
                url_key.to_string(),
                Value::String(sanitize_public_url(raw_url)),
            );
        }
    }

    let has_api_key = profile_has_any_api_key(obj)
        || obj
            .get("officialMixApiKey")
            .and_then(Value::as_str)
            .is_some_and(|s| !s.trim().is_empty());
    let has_vlm_key = obj
        .get("vlmApiKey")
        .and_then(Value::as_str)
        .is_some_and(|s| !s.trim().is_empty());
    let has_auth = obj
        .get("authContents")
        .and_then(Value::as_str)
        .is_some_and(|s| !s.trim().is_empty());
    out.insert("hasApiKey".into(), Value::Bool(has_api_key));
    out.insert("hasVlmApiKey".into(), Value::Bool(has_vlm_key));
    out.insert("hasAuthContents".into(), Value::Bool(has_auth));

    if let Some(headers) = obj.get("customHeaders").and_then(Value::as_array) {
        let filtered_headers: Vec<Value> = headers
            .iter()
            .filter_map(|entry| {
                let name = entry
                    .get("key")
                    .or_else(|| entry.get("name"))
                    .and_then(Value::as_str)?;
                let value = entry
                    .get("value")
                    .and_then(Value::as_str)
                    .unwrap_or_default();
                if header_entry_is_sensitive(name, value) {
                    Some(json!({ "key": name, "name": name, "redacted": true }))
                } else {
                    Some(json!({ "key": name, "name": name, "value": value }))
                }
            })
            .collect();
        out.insert("customHeaders".into(), Value::Array(filtered_headers));
    }

    Value::Object(out)
}

/// Projects strictly non-secret configuration fields (`C-2` allowlist DTO).
pub fn project_public_settings(settings_json: &Value) -> Value {
    let Some(src) = settings_json.as_object() else {
        return json!({});
    };
    let mut out = Map::new();
    for key in [
        "revision",
        "codexAppPath",
        "codexAppAutoTakeover",
        "codexAppAutoTakeoverWhenActive",
        "providerSyncEnabled",
        "sessionIndexRepairEnabled",
        "threadModelFollowSwitchEnabled",
        "keepProjectsWithoutAccount",
        "relayProfilesEnabled",
        "enhancementsEnabled",
        "updateAutoCheck",
        "updateAutoDownload",
        "updateSkippedVersion",
        "activeRelayId",
        "activeAggregateRelayId",
        "codexAppStandaloneQuotaUnlock",
        "codexAppDisableWorkView",
        "codexAppPluginMarketplaceUnlock",
        "codexAppModelWhitelistUnlock",
        "codexAppSessionDelete",
        "codexAppMarkdownExport",
        "codexAppUsageStatusLine",
        "usageHistoryEnabled",
        "codexAppProxyRequestStatus",
        "codexAppStatsigOverrides",
        "codexAppStatsigGateOverrides",
        "codexAppPromptArgs",
        "claudeAppVoiceInput",
        "claudeAppLocalization",
        "claudeAppExtensions",
        "claudeAppComputerUse",
    ] {
        if let Some(val) = src.get(key) {
            out.insert(key.to_string(), val.clone());
        }
    }

    if let Some(dictation) = src.get("dictation").and_then(Value::as_object) {
        let mut dict_out = Map::new();
        for k in ["enabled", "provider", "model", "language", "hotkey"] {
            if let Some(v) = dictation.get(k) {
                dict_out.insert(k.to_string(), v.clone());
            }
        }
        if let Some(raw_url) = dictation.get("baseUrl").and_then(Value::as_str) {
            dict_out.insert(
                "baseUrl".into(),
                Value::String(sanitize_public_url(raw_url)),
            );
        }
        let has_key = dictation
            .get("apiKey")
            .and_then(Value::as_str)
            .is_some_and(|s| !s.trim().is_empty());
        dict_out.insert("hasApiKey".into(), Value::Bool(has_key));
        out.insert("dictation".into(), Value::Object(dict_out));
    }

    if let Some(profiles) = src.get("relayProfiles").and_then(Value::as_array) {
        let projected: Vec<Value> = profiles.iter().map(project_public_relay_profile).collect();
        out.insert("relayProfiles".into(), Value::Array(projected));
    }

    if let Some(tools) = src.get("tools").and_then(Value::as_object) {
        let mut tools_out = Map::new();
        for (tool_id, tool_val) in tools {
            if let Some(tool_obj) = tool_val.as_object() {
                let mut clean_tool = Map::new();
                for k in ["activeRelayId", "enabled", "gatewayEnabled"] {
                    if let Some(v) = tool_obj.get(k) {
                        clean_tool.insert(k.to_string(), v.clone());
                    }
                }
                if let Some(tp) = tool_obj.get("relayProfiles").and_then(Value::as_array) {
                    clean_tool.insert(
                        "relayProfiles".into(),
                        Value::Array(tp.iter().map(project_public_relay_profile).collect()),
                    );
                }
                tools_out.insert(tool_id.clone(), Value::Object(clean_tool));
            }
        }
        out.insert("tools".into(), Value::Object(tools_out));
    }

    out.insert(
        "hasWeixinConnectToken".into(),
        Value::Bool(
            src.get("weixinConnectToken")
                .and_then(Value::as_str)
                .is_some_and(|s| !s.trim().is_empty()),
        ),
    );
    out.insert(
        "hasRelayApiKey".into(),
        Value::Bool(
            src.get("relayApiKey")
                .and_then(Value::as_str)
                .is_some_and(|s| !s.trim().is_empty()),
        ),
    );

    Value::Object(out)
}

/// Checks whether a route may be called over the renderer bridge (`C-3`).
pub fn is_renderer_bridge_route_permitted(route: &str) -> bool {
    renderer_inject_rs::is_extension_route_allowed(route)
}

/// Computes explicit extension risk tags including `main-world-renderer` warnings (`C-3`).
pub fn classify_extension_risks(manifest: &Value) -> Vec<String> {
    let mut risks = Vec::new();
    let has_renderer = manifest.get("renderer").is_some_and(|v| !v.is_null())
        || manifest
            .get("entry")
            .is_some_and(|v| v.as_str().is_some_and(|s| !s.is_empty()));
    if has_renderer {
        risks.push("main-world-renderer (chạy JS trong ngữ cảnh trang Codex)".to_string());
    }
    if manifest.get("hooks").is_some_and(|v| !v.is_null()) {
        risks.push("hooks (chạy lệnh shell ngoài tiến trình)".to_string());
    }
    if manifest.get("backend").is_some_and(|v| !v.is_null()) {
        risks.push("backend (dịch vụ nền nội bộ)".to_string());
    }
    if manifest.get("gateway").is_some_and(|v| !v.is_null()) {
        risks.push("gateway (can thiệp lưu lượng API)".to_string());
    }
    if let Some(perms) = manifest.get("routes").and_then(Value::as_array) {
        let disallowed: Vec<&str> = perms
            .iter()
            .filter_map(Value::as_str)
            .filter(|r| !is_renderer_bridge_route_permitted(r))
            .collect();
        if !disallowed.is_empty() {
            risks.push(format!("route-bị-chặn: {}", disallowed.join(", ")));
        }
    }
    risks
}

fn validate_rollout_candidate(
    codex_home: &Path,
    allowed_roots: &[PathBuf],
    rp: &Path,
) -> Result<Option<PathBuf>> {
    if rp.components().any(|c| matches!(c, Component::ParentDir)) {
        bail!(
            "Từ chối rollout_path chứa thành phần leo thư mục `..`: {}",
            rp.display()
        );
    }
    match fs::symlink_metadata(rp) {
        Ok(meta) => {
            if meta.file_type().is_symlink() {
                bail!("Từ chối xóa rollout_path là symlink: {}", rp.display());
            }
            if !meta.file_type().is_file() {
                bail!(
                    "Từ chối xóa rollout_path không phải tệp thường: {}",
                    rp.display()
                );
            }
            let can_rp = dunce::canonicalize(rp)
                .with_context(|| format!("Không thể chuẩn hóa rollout_path: {}", rp.display()))?;
            let is_jsonl = can_rp
                .extension()
                .and_then(|e| e.to_str())
                .is_some_and(|e| e.eq_ignore_ascii_case("jsonl"));
            if !is_jsonl {
                bail!(
                    "Từ chối xóa tệp rollout không có phần mở rộng .jsonl: {}",
                    can_rp.display()
                );
            }
            if !allowed_roots.iter().any(|root| can_rp.starts_with(root)) {
                bail!(
                    "Từ chối xóa/sao lưu rollout_path ngoài sessions/archived_sessions: {}",
                    can_rp.display()
                );
            }
            Ok(Some(can_rp))
        }
        Err(e) if e.kind() == ErrorKind::NotFound => {
            // Allow cleanup of orphaned DB rows whose `.jsonl` rollout file was already removed,
            // provided the path lexically targets a `.jsonl` inside sessions / archived_sessions.
            let is_jsonl = rp
                .extension()
                .and_then(|ext| ext.to_str())
                .is_some_and(|ext| ext.eq_ignore_ascii_case("jsonl"));
            let norm = rp.to_string_lossy().replace('\\', "/").to_ascii_lowercase();
            let home_norm = codex_home
                .to_string_lossy()
                .replace('\\', "/")
                .trim_end_matches('/')
                .to_ascii_lowercase();
            let lexically_inside = norm.starts_with(&format!("{home_norm}/sessions/"))
                || norm.starts_with(&format!("{home_norm}/archived_sessions/"))
                || norm.contains("/.codex/sessions/")
                || norm.contains("/.codex/archived_sessions/");
            if !is_jsonl || !lexically_inside {
                bail!(
                    "Từ chối rollout_path không tồn tại nhưng nằm ngoài phạm vi hợp lệ: {}",
                    rp.display()
                );
            }
            Ok(None)
        }
        Err(e) => Err(e).with_context(|| format!("Lỗi kiểm tra rollout_path: {}", rp.display())),
    }
}

/// Enforces canonical `db_path` and `rollout_path` containment within `codex_home` (`C-4`).
pub fn validate_session_deletion_paths(
    codex_home: &Path,
    db_path: &Path,
    rollout_path: Option<&Path>,
) -> Result<(PathBuf, Option<PathBuf>)> {
    let db_meta = fs::symlink_metadata(db_path)
        .with_context(|| format!("Không tìm thấy tệp CSDL phiên: {}", db_path.display()))?;
    if db_meta.file_type().is_symlink() {
        bail!("Từ chối CSDL phiên là symlink: {}", db_path.display());
    }
    let canonical_db = dunce::canonicalize(db_path)
        .with_context(|| format!("Không tìm thấy tệp CSDL phiên: {}", db_path.display()))?;
    let allowed_dbs: HashSet<PathBuf> =
        codex_plus_core::codex_sqlite::codex_session_db_paths_from_home(codex_home)
            .into_iter()
            .filter_map(|p| dunce::canonicalize(&p).ok())
            .collect();

    if !allowed_dbs.contains(&canonical_db) {
        bail!(
            "Từ chối thao tác xóa trên CSDL ngoài phạm vi cho phép: {}",
            canonical_db.display()
        );
    }

    let allowed_roots: Vec<PathBuf> = ["sessions", "archived_sessions"]
        .into_iter()
        .map(|sub| codex_home.join(sub))
        .filter_map(|p| dunce::canonicalize(&p).ok())
        .collect();

    let canonical_rollout = match rollout_path {
        None => None,
        Some(rp) => validate_rollout_candidate(codex_home, &allowed_roots, rp)?,
    };

    Ok((canonical_db, canonical_rollout))
}

/// Validates BOTH caller-provided `rollout_path` AND the authoritative `threads.rollout_path`
/// stored inside `canonical_db` before `delete_local_from_paths` executes (`C-4` complete guard).
pub fn validate_session_deletion_with_db_records(
    codex_home: &Path,
    db_path: &Path,
    session_id: &str,
    caller_rollout: Option<&Path>,
) -> Result<(PathBuf, Option<PathBuf>)> {
    let (canonical_db, can_caller_rp) =
        validate_session_deletion_paths(codex_home, db_path, caller_rollout)?;

    let allowed_roots: Vec<PathBuf> = ["sessions", "archived_sessions"]
        .into_iter()
        .map(|sub| codex_home.join(sub))
        .filter_map(|p| dunce::canonicalize(&p).ok())
        .collect();

    let conn = open_sqlite_safe(&canonical_db, true)?;
    let has_threads_table: bool = conn
        .query_row(
            "SELECT 1 FROM sqlite_master WHERE type = 'table' AND name = 'threads' LIMIT 1",
            [],
            |_| Ok(true),
        )
        .optional()?
        .unwrap_or(false);

    if has_threads_table {
        let db_rollout: Option<String> = conn
            .query_row(
                "SELECT rollout_path FROM threads WHERE id = ?1 LIMIT 1",
                [session_id],
                |row| row.get(0),
            )
            .optional()?
            .flatten();
        if let Some(raw_db_rp) = db_rollout {
            let trimmed = raw_db_rp.trim();
            if !trimmed.is_empty() {
                let _ = validate_rollout_candidate(codex_home, &allowed_roots, Path::new(trimmed))?;
            }
        }
    }

    Ok((canonical_db, can_caller_rp))
}

/// Symlink-preserving, Windows-retry-hardened atomic file write using `<filename>.<pid>.<uuid>.tmp`
/// (`H-4.2.3` + `H-4.1.6`).
pub fn atomic_write_unique(target: &Path, contents: &[u8]) -> Result<()> {
    let resolved_target = if target
        .symlink_metadata()
        .is_ok_and(|m| m.file_type().is_symlink())
    {
        dunce::canonicalize(target).unwrap_or_else(|_| target.to_path_buf())
    } else {
        target.to_path_buf()
    };

    let parent = resolved_target
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent)?;
    let name = resolved_target
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("file");
    let tmp = parent.join(format!(
        ".{name}.{}.{}.tmp",
        std::process::id(),
        Uuid::new_v4()
    ));

    let write_res = (|| -> Result<()> {
        let mut opts = OpenOptions::new();
        opts.create_new(true).write(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            opts.mode(0o600);
        }
        let mut file = opts.open(&tmp)?;
        file.write_all(contents)?;
        file.sync_all()?;
        drop(file);

        let mut last_err = None;
        for attempt in 0..8u8 {
            match fs::rename(&tmp, &resolved_target) {
                Ok(()) => return Ok(()),
                Err(e) => {
                    let retryable = e.kind() == ErrorKind::PermissionDenied
                        || matches!(e.raw_os_error(), Some(5 | 32));
                    if !retryable || attempt == 7 {
                        last_err = Some(e);
                        break;
                    }
                    thread::sleep(Duration::from_millis(15 * u64::from(attempt + 1)));
                }
            }
        }
        Err(last_err.expect("rename loop exited with error").into())
    })();

    if write_res.is_err() {
        let _ = fs::remove_file(&tmp);
    }
    write_res
}

/// Opens SQLite safely with explicit `busy_timeout(5s)` and no accidental empty-DB creation in read-only mode (`H-4.2.2`).
pub fn open_sqlite_safe(path: &Path, read_only: bool) -> Result<Connection> {
    if read_only && !path.exists() {
        bail!("Tệp SQLite không tồn tại: {}", path.display());
    }
    let flags = if read_only {
        OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX
    } else {
        OpenFlags::SQLITE_OPEN_READ_WRITE
            | OpenFlags::SQLITE_OPEN_CREATE
            | OpenFlags::SQLITE_OPEN_NO_MUTEX
    };
    let conn = Connection::open_with_flags(path, flags)
        .with_context(|| format!("Mở SQLite thất bại: {}", path.display()))?;
    conn.busy_timeout(Duration::from_secs(5))?;
    Ok(conn)
}

/// Executes a WAL-safe `BEGIN IMMEDIATE` write transaction (`H-4.2.2`).
pub fn with_immediate_transaction<T>(
    conn: &mut Connection,
    f: impl FnOnce(&Transaction<'_>) -> Result<T>,
) -> Result<T> {
    let tx = conn.transaction_with_behavior(TransactionBehavior::Immediate)?;
    let val = f(&tx)?;
    tx.commit()?;
    Ok(val)
}
