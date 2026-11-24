//! Non-blocking direct Rust service layer connecting UI to `codex-plus-core`,
//! `codex-plus-data`, `codex-plus-launcher`, `manager-core-rs`, and `renderer-inject-rs`.

#![forbid(unsafe_code)]

use crate::security::{
    atomic_write_unique, classify_extension_risks, open_sqlite_safe, project_public_settings,
    validate_session_deletion_with_db_records,
};
use anyhow::{Context, Result};
use codex_plus_core::config_doctor::{Finding, run_rules};
use codex_plus_core::models::{DeleteStatus, SessionRef};
use codex_plus_core::relay_config::apply_relay_profile_to_home_with_switch_rules;
use codex_plus_core::settings::{BackendSettings, SettingsStore};
use codex_plus_data::{BackupStore, LocalSession, SQLiteStorageAdapter, delete_local_from_paths};
use manager_core_rs::relay_editing::sync_legacy_relay_fields;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};
use std::time::Duration;

static GATEWAY_HTTP_CLIENT: OnceLock<reqwest::Client> = OnceLock::new();
static CONFIG_TOML_WRITE_LOCK: Mutex<()> = Mutex::new(());

fn gateway_http_client() -> &'static reqwest::Client {
    GATEWAY_HTTP_CLIENT.get_or_init(|| {
        reqwest::Client::builder()
            .timeout(Duration::from_secs(3))
            .no_proxy()
            .build()
            .expect("build loopback reqwest client")
    })
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessSummary {
    pub pid: u32,
    pub exe: String,
    pub path: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionItemView {
    pub id: String,
    pub title: String,
    pub cwd: String,
    pub model_provider: String,
    pub archived: bool,
    pub updated_at_ms: Option<i64>,
    pub rollout_path: String,
    pub db_path: String,
}

impl From<LocalSession> for SessionItemView {
    fn from(s: LocalSession) -> Self {
        Self {
            id: s.id,
            title: s.title,
            cwd: s.cwd,
            model_provider: s.model_provider,
            archived: s.archived,
            updated_at_ms: s.updated_at_ms,
            rollout_path: s.rollout_path,
            db_path: s.db_path,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestLogView {
    pub id: String,
    pub timestamp_ms: u64,
    pub duration_ms: u64,
    pub method: String,
    pub path: String,
    pub status_code: u16,
    pub success: bool,
    pub model: String,
    pub relay_name: String,
    pub wire_api: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DoctorFindingView {
    pub id: String,
    pub level: String,
    pub title: String,
    pub detail: String,
    pub fix: Option<String>,
}

impl From<Finding> for DoctorFindingView {
    fn from(f: Finding) -> Self {
        Self {
            id: f.id,
            level: format!("{:?}", f.level).to_ascii_lowercase(),
            title: f.title,
            detail: f.detail,
            fix: f.fix,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionEntryView {
    pub key: String,
    pub name: String,
    pub enabled: bool,
    pub error: Option<String>,
    pub risks: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSnapshot {
    pub version: String,
    pub state_dir: String,
    pub settings_path: String,
    pub codex_home: String,
    pub config_toml_exists: bool,
    pub config_toml_preview: String,
    pub codex_app_dir: Option<String>,
    pub codex_app_version: Option<String>,
    pub codex_processes: Vec<ProcessSummary>,
    pub helper_port: Option<u16>,
    pub configured_proxy_port: u16,
    pub gateway_http_status: Option<u16>,
    pub gateway_status_json: Option<Value>,
    pub proxy_status_json: Option<Value>,
    pub native_browser_summary: String,
    pub settings: Value,
    pub public_settings: Value,
    pub sessions: Vec<SessionItemView>,
    pub request_logs: Vec<RequestLogView>,
    pub doctor_findings: Vec<DoctorFindingView>,
    pub user_scripts: Vec<ExtensionEntryView>,
    pub launcher_ready: bool,
    pub launcher_status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmokeCheckReport {
    pub ok: bool,
    pub version: String,
    pub state_dir: String,
    pub codex_home: String,
    pub codex_app_dir: Option<String>,
    pub codex_app_version: Option<String>,
    pub codex_running_processes: usize,
    pub gateway_helper_port: u16,
    pub backend_status_http: u16,
    pub backend_status_body: Value,
    pub proxy_status_http: u16,
    pub proxy_status_body: Value,
    pub sqlite_db_count: usize,
    pub local_sessions_count: usize,
    pub preset_count: usize,
    pub c2_public_dto_clean: bool,
    pub prompt_expand_sample: Value,
    pub c3_blocked_dangerous_route: bool,
}

fn find_windows_codex_processes() -> Vec<ProcessSummary> {
    #[cfg(windows)]
    {
        codex_plus_core::windows_enumerate_processes()
            .into_iter()
            .filter(|p| {
                let lower = p.exe_file.to_ascii_lowercase();
                lower.contains("codex") || lower.contains("chatgpt")
            })
            .map(|p| ProcessSummary {
                pid: p.process_id,
                exe: p.exe_file,
                path: p
                    .executable_path
                    .map(|pb| pb.to_string_lossy().into_owned()),
            })
            .collect()
    }
    #[cfg(not(windows))]
    {
        Vec::new()
    }
}

fn query_sessions_readonly(
    conn: &rusqlite::Connection,
    db_path: &Path,
    limit: usize,
) -> Result<Vec<SessionItemView>> {
    let mut stmt = conn.prepare(
        "SELECT id, COALESCE(title, ''), COALESCE(cwd, ''), COALESCE(model_provider, ''), \
         COALESCE(archived, 0), updated_at, COALESCE(rollout_path, '') \
         FROM threads ORDER BY updated_at DESC LIMIT ?1",
    )?;
    let db_str = db_path.to_string_lossy().into_owned();
    let rows = stmt.query_map([limit as i64], |row| {
        let archived_int: i64 = row.get(4).unwrap_or(0);
        let raw_updated: Option<i64> = row.get(5).ok();
        let updated_ms = raw_updated.map(|ts| if ts < 10_000_000_000 { ts * 1000 } else { ts });
        Ok(SessionItemView {
            id: row.get(0)?,
            title: row.get(1)?,
            cwd: row.get(2)?,
            model_provider: row.get(3)?,
            archived: archived_int != 0,
            updated_at_ms: updated_ms,
            rollout_path: row.get(6)?,
            db_path: db_str.clone(),
        })
    })?;
    let mut out = Vec::new();
    for item in rows.flatten() {
        out.push(item);
    }
    Ok(out)
}

fn read_sessions_safely(home: &Path, limit: usize) -> Vec<SessionItemView> {
    let mut out = Vec::new();
    for db_path in codex_plus_core::codex_sqlite::codex_session_db_paths_from_home(home) {
        if !db_path.is_file() {
            continue;
        }
        let remaining = limit.saturating_sub(out.len());
        if remaining == 0 {
            break;
        }
        let Ok(conn) = open_sqlite_safe(&db_path, true) else {
            continue;
        };
        if let Ok(direct) = query_sessions_readonly(&conn, &db_path, remaining) {
            out.extend(direct);
            continue;
        }
        drop(conn);
        let backup =
            BackupStore::new(codex_plus_core::paths::default_app_state_dir().join("backups"));
        let adapter = SQLiteStorageAdapter::new(db_path, backup).with_codex_home(home);
        if let Ok(list) = adapter.list_local_sessions_limited(remaining) {
            out.extend(list.into_iter().map(SessionItemView::from));
        }
    }
    out
}

fn load_extensions_summary() -> Vec<ExtensionEntryView> {
    let Ok(inv) = codex_plus_core::user_scripts::default_user_script_manager().inventory() else {
        return Vec::new();
    };
    let Some(arr) = inv.get("scripts").and_then(Value::as_array) else {
        return Vec::new();
    };
    arr.iter()
        .map(|item| {
            let key = item
                .get("key")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_string();
            let name = item
                .get("name")
                .and_then(Value::as_str)
                .unwrap_or(&key)
                .to_string();
            let enabled = item
                .get("enabled")
                .and_then(Value::as_bool)
                .unwrap_or(false);
            let error = item
                .get("error")
                .and_then(Value::as_str)
                .map(str::to_string);
            let risks = classify_extension_risks(item.get("package").unwrap_or(item));
            ExtensionEntryView {
                key,
                name,
                enabled,
                error,
                risks,
            }
        })
        .collect()
}

fn hydrate_settings_json(settings: &BackendSettings) -> Value {
    let raw = serde_json::to_value(settings).unwrap_or_else(|_| json!({}));
    sync_legacy_relay_fields(raw.clone()).unwrap_or(raw)
}

pub fn load_snapshot_blocking() -> Result<AppSnapshot> {
    let store = SettingsStore::default();
    let settings = store.load().unwrap_or_default();
    let settings_json = hydrate_settings_json(&settings);
    let public_settings = project_public_settings(&settings_json);

    let state_dir = codex_plus_core::paths::default_app_state_dir();
    let settings_path = codex_plus_core::paths::default_settings_path();
    let codex_home = codex_plus_core::codex_home::default_codex_home_dir();
    let cfg_path = codex_home.join("config.toml");
    let config_toml_exists = cfg_path.is_file();
    let config_toml_preview = if config_toml_exists {
        std::fs::read_to_string(&cfg_path)
            .unwrap_or_default()
            .lines()
            .take(80)
            .collect::<Vec<_>>()
            .join("\n")
    } else {
        String::new()
    };

    let app_dir = codex_plus_core::app_paths::resolve_codex_app_dir(
        (!settings.codex_app_path.trim().is_empty()).then(|| Path::new(&settings.codex_app_path)),
    );
    let codex_app_version = app_dir
        .as_deref()
        .and_then(codex_plus_core::app_paths::codex_app_version);

    let findings = run_rules(&settings, &codex_home)
        .into_iter()
        .map(DoctorFindingView::from)
        .collect();

    let logs = codex_plus_core::request_log::read_recent_request_logs(50)
        .unwrap_or_default()
        .into_iter()
        .map(|e| RequestLogView {
            id: e.id,
            timestamp_ms: e.timestamp_ms,
            duration_ms: e.duration_ms,
            method: e.method,
            path: e.path,
            status_code: e.status_code,
            success: e.success,
            model: e.model,
            relay_name: e.relay_name,
            wire_api: e.wire_api,
        })
        .collect();

    let browser_status = codex_plus_core::native_browser::read_status();
    let launcher_snap = codex_plus_launcher::session_snapshot();

    Ok(AppSnapshot {
        version: codex_plus_core::version::VERSION.to_string(),
        state_dir: state_dir.to_string_lossy().into_owned(),
        settings_path: settings_path.to_string_lossy().into_owned(),
        codex_home: codex_home.to_string_lossy().into_owned(),
        config_toml_exists,
        config_toml_preview,
        codex_app_dir: app_dir.map(|p| p.to_string_lossy().into_owned()),
        codex_app_version,
        codex_processes: find_windows_codex_processes(),
        helper_port: codex_plus_core::resident_helper::resident_helper_port(),
        configured_proxy_port: codex_plus_core::protocol_proxy::protocol_proxy_port(),
        gateway_http_status: None,
        gateway_status_json: None,
        proxy_status_json: None,
        native_browser_summary: format!("{browser_status:?}"),
        settings: settings_json,
        public_settings,
        sessions: read_sessions_safely(&codex_home, 40),
        request_logs: logs,
        doctor_findings: findings,
        user_scripts: load_extensions_summary(),
        launcher_ready: launcher_snap.ready,
        launcher_status: launcher_snap.status,
    })
}

/// Non-blocking snapshot loader with concurrent live Gateway HTTP probes (`H-4.2.4`).
pub async fn load_snapshot_with_gateway_probe() -> Result<AppSnapshot> {
    let mut snap = tokio::task::spawn_blocking(load_snapshot_blocking)
        .await
        .context("Snapshot task panicked")??;
    if let Some(port) = snap.helper_port {
        let (backend_res, proxy_res) = tokio::join!(
            probe_gateway_endpoint(port, "/backend/status"),
            probe_gateway_endpoint(port, "/proxy-status")
        );
        if let Ok((code, body)) = backend_res {
            snap.gateway_http_status = Some(code);
            snap.gateway_status_json = Some(body);
        }
        if let Ok((_, proxy_body)) = proxy_res {
            snap.proxy_status_json = Some(proxy_body);
        }
    }
    Ok(snap)
}

pub async fn probe_gateway_endpoint(port: u16, path: &str) -> Result<(u16, Value)> {
    let resp = gateway_http_client()
        .get(format!("http://127.0.0.1:{port}{path}"))
        .send()
        .await?;
    let status = resp.status().as_u16();
    let json = resp.json::<Value>().await.unwrap_or_else(|_| json!({}));
    Ok((status, json))
}

pub async fn ensure_gateway_helper_async() -> Result<u16> {
    tokio::task::spawn_blocking(|| -> Result<u16> {
        let port = codex_plus_core::protocol_proxy::protocol_proxy_port();
        if let Ok(settings) = SettingsStore::default().load() {
            codex_plus_core::launcher::set_http_body_limit_bytes(
                settings.helper_max_request_body_bytes(),
            );
        }
        codex_plus_core::resident_helper::ensure_resident_helper(port)?;
        Ok(port)
    })
    .await
    .context("Gateway task panicked")?
}

pub async fn stop_gateway_helper_async() {
    let _ = tokio::task::spawn_blocking(|| {
        codex_plus_core::resident_helper::stop_resident_helper();
    })
    .await;
}

fn apply_active_relay_if_enabled(settings: &BackendSettings) {
    if !settings.relay_profiles_enabled {
        return;
    }
    if let Some(active) = settings
        .relay_profiles
        .iter()
        .find(|p| p.id == settings.active_relay_id)
    {
        let home = codex_plus_core::codex_home::default_codex_home_dir();
        let _ = apply_relay_profile_to_home_with_switch_rules(
            &home,
            active,
            &settings.relay_common_config_contents,
        );
    }
}

/// Saves full settings snapshot and applies active relay profile to `~/.codex`.
pub async fn save_settings_async(mut next_settings: Value) -> Result<Value> {
    tokio::task::spawn_blocking(move || -> Result<Value> {
        if let Some(obj) = next_settings.as_object_mut() {
            // Drop stale UI-render revision so concurrent clicks merge cleanly via SettingsStore::update.
            obj.remove("revision");
        }
        let store = SettingsStore::default();
        let saved = store
            .update(next_settings)
            .context("Cập nhật cài đặt BackendSettings thất bại")?;
        apply_active_relay_if_enabled(&saved);
        Ok(hydrate_settings_json(&saved))
    })
    .await
    .context("Save settings task panicked")?
}

/// Applies a minimal lock-protected JSON patch (`SettingsStore::update`) without cloning full settings.
pub async fn patch_settings_async(patch: Value) -> Result<Value> {
    tokio::task::spawn_blocking(move || -> Result<Value> {
        let saved = SettingsStore::default()
            .update(patch)
            .context("Cập nhật tùy chọn thất bại")?;
        Ok(hydrate_settings_json(&saved))
    })
    .await
    .context("Patch settings task panicked")?
}

pub async fn launch_codex_async() -> Result<()> {
    tokio::task::spawn_blocking(|| -> Result<()> {
        let port = codex_plus_core::protocol_proxy::protocol_proxy_port();
        let _ = codex_plus_core::resident_helper::ensure_resident_helper(port);
        let opts = codex_plus_launcher::LaunchOptions {
            helper_port: port,
            ..Default::default()
        };
        codex_plus_launcher::start(opts)
    })
    .await
    .context("Launch Codex task panicked")?
}

pub async fn stop_codex_async() -> Result<()> {
    tokio::task::spawn_blocking(codex_plus_launcher::stop)
        .await
        .context("Stop Codex task panicked")?
}

/// Safe session deletion enforcing canonical path guard on BOTH caller & SQLite `threads.rollout_path` (`C-4`).
pub async fn delete_session_guarded_async(
    session_id: String,
    db_path: String,
    rollout_path: Option<String>,
) -> Result<String> {
    tokio::task::spawn_blocking(move || -> Result<String> {
        let home = codex_plus_core::codex_home::default_codex_home_dir();
        let rp = rollout_path
            .as_deref()
            .filter(|s| !s.trim().is_empty())
            .map(PathBuf::from);
        let (canonical_db, _) = validate_session_deletion_with_db_records(
            &home,
            Path::new(&db_path),
            &session_id,
            rp.as_deref(),
        )?;
        let backup_dir = codex_plus_core::paths::default_app_state_dir().join("backups");
        let res = delete_local_from_paths(
            vec![canonical_db],
            BackupStore::new(backup_dir),
            &SessionRef {
                session_id,
                title: String::new(),
                host_id: Some("local".into()),
            },
            Some(&home),
        );
        if matches!(res.status, DeleteStatus::LocalDeleted) {
            Ok(res.message)
        } else {
            anyhow::bail!(res.message)
        }
    })
    .await
    .context("Delete session task panicked")?
}

fn has_mcp_server_table(toml_text: &str, preset_id: &str) -> bool {
    let bare = format!("[mcp_servers.{preset_id}]");
    let quoted_dq = format!("[mcp_servers.\"{preset_id}\"]");
    let quoted_sq = format!("[mcp_servers.'{preset_id}']");
    toml_text.lines().any(|line| {
        let trimmed = line.trim();
        if trimmed.starts_with('#') {
            return false;
        }
        let header_part = trimmed.split('#').next().unwrap_or(trimmed).trim();
        header_part == bare
            || header_part == quoted_dq
            || header_part == quoted_sq
            || header_part.starts_with(&format!("[mcp_servers.{preset_id}."))
            || header_part.starts_with(&format!("[mcp_servers.\"{preset_id}\"."))
    })
}

/// Writes an MCP preset TOML block safely to `~/.codex/config.toml` via `atomic_write_unique` (`H-4.2.3`).
/// Fails closed on any I/O error other than `ErrorKind::NotFound` so existing config is NEVER wiped.
pub async fn append_mcp_preset_async(preset_id: String) -> Result<String> {
    tokio::task::spawn_blocking(move || -> Result<String> {
        let _guard = CONFIG_TOML_WRITE_LOCK
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        let body = manager_core_rs::static_units::mcp_preset_toml_body(&preset_id, cfg!(windows))
            .with_context(|| format!("Không tìm thấy MCP preset: {preset_id}"))?;
        let home = codex_plus_core::codex_home::default_codex_home_dir();
        let cfg_path = home.join("config.toml");
        let existing = match std::fs::read_to_string(&cfg_path) {
            Ok(text) => text,
            Err(e) if e.kind() == ErrorKind::NotFound => String::new(),
            Err(e) => {
                return Err(e).with_context(|| {
                    format!(
                        "Không thể đọc {} (từ chối ghi đè để bảo vệ cấu hình hiện có)",
                        cfg_path.display()
                    )
                });
            }
        };
        if has_mcp_server_table(&existing, &preset_id) {
            return Ok(format!(
                "MCP server `{preset_id}` đã tồn tại trong config.toml"
            ));
        }
        let header = if preset_id
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
        {
            format!("[mcp_servers.{preset_id}]")
        } else {
            format!("[mcp_servers.\"{preset_id}\"]")
        };
        let next = if existing.trim().is_empty() {
            format!("{header}\n{body}")
        } else {
            format!("{}\n\n{header}\n{body}", existing.trim_end())
        };
        atomic_write_unique(&cfg_path, next.as_bytes())?;
        Ok(format!("Đã thêm `{preset_id}` vào {}", cfg_path.display()))
    })
    .await
    .context("MCP append task panicked")?
}

/// Live Gateway + Codex Desktop + Rust crate integration smoke check (`Checkpoint 5/6`).
pub async fn run_smoke_check(stop_helper_after: bool) -> Result<SmokeCheckReport> {
    let was_running = codex_plus_core::resident_helper::resident_helper_port();
    let helper_port = if let Some(p) = was_running {
        p
    } else {
        ensure_gateway_helper_async().await?
    };

    let (backend_res, proxy_res) = tokio::join!(
        probe_gateway_endpoint(helper_port, "/backend/status"),
        probe_gateway_endpoint(helper_port, "/proxy-status")
    );
    let (backend_status_http, backend_status_body) = backend_res?;
    let (proxy_status_http, proxy_status_body) = proxy_res?;

    let snap = tokio::task::spawn_blocking(load_snapshot_blocking)
        .await
        .context("Snapshot task panicked")??;

    // Verify C-2 allowlist redaction strips synthetic secrets (including URL credentials & `key` headers).
    let secret_probe = json!({
        "apiKeys": [{ "id": "k1", "apiKey": "SECRET-MASTER-KEY" }],
        "weixinConnectToken": "SECRET-WEIXIN",
        "relayProfiles": [{
            "id": "r1",
            "name": "Test",
            "baseUrl": "https://user:SECRET-URL-PASS@relay.example.com/v1?api_key=SECRET-QUERY",
            "apiKey": "SECRET-RELAY-KEY",
            "vlmApiKey": "SECRET-VLM-KEY",
            "authContents": "{\"refresh_token\":\"SECRET-REFRESH\"}",
            "customHeaders": [
                { "key": "Authorization", "value": "Bearer SECRET-HDR" },
                { "name": "X-Proxy-Password", "value": "SECRET-PROXY-PASS" }
            ]
        }]
    });
    let dto_str = project_public_settings(&secret_probe).to_string();
    let c2_public_dto_clean = !dto_str.contains("SECRET-");

    let prompt_expand_sample = serde_json::to_value(renderer_inject_rs::prompt_args_expand(
        "demo",
        "Fix $1 in $2",
        r#"bug "src/main.rs""#,
    ))?;
    let c3_blocked_dangerous_route =
        !renderer_inject_rs::is_extension_route_allowed("/hooks/test-run");
    let preset_count = manager_core_rs::static_units::presets()
        .as_array()
        .map_or(0, Vec::len);
    let sqlite_db_count = codex_plus_core::codex_sqlite::codex_session_db_paths_from_home(
        Path::new(&snap.codex_home),
    )
    .into_iter()
    .filter(|p| p.is_file())
    .count();

    if stop_helper_after && was_running.is_none() {
        stop_gateway_helper_async().await;
    }

    let ok = backend_status_http == 200
        && proxy_status_http == 200
        && c2_public_dto_clean
        && c3_blocked_dangerous_route
        && preset_count == 23;

    Ok(SmokeCheckReport {
        ok,
        version: snap.version,
        state_dir: snap.state_dir,
        codex_home: snap.codex_home,
        codex_app_dir: snap.codex_app_dir,
        codex_app_version: snap.codex_app_version,
        codex_running_processes: snap.codex_processes.len(),
        gateway_helper_port: helper_port,
        backend_status_http,
        backend_status_body,
        proxy_status_http,
        proxy_status_body,
        sqlite_db_count,
        local_sessions_count: snap.sessions.len(),
        preset_count,
        c2_public_dto_clean,
        prompt_expand_sample,
        c3_blocked_dangerous_route,
    })
}
