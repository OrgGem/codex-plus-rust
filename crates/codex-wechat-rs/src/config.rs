//! Configuration and sync-cursor state persistence ported from `codex_wechat.py`.

use anyhow::{Context, Result, bail};
use serde_json::Value;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

pub const ILINK_BASE_URL: &str = "https://ilinkai.weixin.qq.com";
pub const DEFAULT_PROMPT_PREFIX: &str = "回复使用中文，格式适合微信阅读。";

#[derive(Debug, Clone)]
pub struct CodexConfig {
    pub backend: String,
    pub command: String,
    pub app_server_command: String,
    pub workspace: String,
    pub model: String,
    pub profile: String,
    pub sandbox: String,
    pub approval_policy: String,
    pub timeout_seconds: u64,
    pub extra_args: Vec<String>,
    pub prompt_prefix: String,
}

impl Default for CodexConfig {
    fn default() -> Self {
        Self {
            backend: "app-server".into(),
            command: "codex".into(),
            app_server_command: String::new(),
            workspace: ".".into(),
            model: String::new(),
            profile: String::new(),
            sandbox: "read-only".into(),
            approval_policy: "never".into(),
            timeout_seconds: 300,
            extra_args: Vec::new(),
            prompt_prefix: DEFAULT_PROMPT_PREFIX.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BridgeConfig {
    pub base_url: String,
    pub token: String,
    pub account_id: String,
    pub allow_all: bool,
    pub allow_user_ids: HashSet<String>,
    pub require_prefix: bool,
    pub prefix: String,
    pub allow_id_command_for_unauthorized: bool,
    pub poll_timeout_seconds: u64,
    pub retry_delay_seconds: u64,
    pub backoff_delay_seconds: u64,
    pub max_consecutive_failures: u32,
    pub max_reply_chars: usize,
    pub enable_typing: bool,
    pub state_dir: PathBuf,
    pub codex: CodexConfig,
}

impl Default for BridgeConfig {
    fn default() -> Self {
        Self {
            base_url: ILINK_BASE_URL.into(),
            token: String::new(),
            account_id: String::new(),
            allow_all: false,
            allow_user_ids: HashSet::new(),
            require_prefix: true,
            prefix: "/codex".into(),
            allow_id_command_for_unauthorized: true,
            poll_timeout_seconds: 35,
            retry_delay_seconds: 2,
            backoff_delay_seconds: 30,
            max_consecutive_failures: 3,
            max_reply_chars: 1800,
            enable_typing: true,
            state_dir: default_state_dir(),
            codex: CodexConfig::default(),
        }
    }
}

pub fn default_state_dir() -> PathBuf {
    home_dir().join(".codex-plus-plus").join("wechat-claw")
}

fn home_dir() -> PathBuf {
    std::env::var_os("USERPROFILE")
        .or_else(|| std::env::var_os("HOME"))
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."))
}

fn non_empty_str(v: Option<&Value>, default: &str) -> String {
    match v {
        Some(Value::String(s)) if !s.is_empty() => s.clone(),
        Some(Value::Number(n)) => n.to_string(),
        Some(Value::Bool(b)) => b.to_string(),
        _ => default.to_string(),
    }
}

fn bool_or(v: Option<&Value>, default: bool) -> bool {
    match v {
        None => default,
        Some(Value::Bool(b)) => *b,
        Some(Value::Null) => false,
        Some(Value::Number(n)) => n.as_f64().is_some_and(|x| x != 0.0 && !x.is_nan()),
        Some(Value::String(s)) => !s.is_empty(),
        Some(Value::Array(a)) => !a.is_empty(),
        Some(Value::Object(o)) => !o.is_empty(),
    }
}

pub fn load_json_object(path: &Path) -> Result<serde_json::Map<String, Value>> {
    let raw_text =
        fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;
    let val: Value = serde_json::from_str(&raw_text)
        .with_context(|| format!("invalid JSON in {}", path.display()))?;
    match val {
        Value::Object(m) => Ok(m),
        _ => bail!("{} must contain a JSON object", path.display()),
    }
}

pub fn load_config(path: &Path) -> Result<BridgeConfig> {
    let raw = load_json_object(path)?;
    let codex_raw = match raw.get("codex") {
        None | Some(Value::Null) => serde_json::Map::new(),
        Some(Value::Object(o)) => o.clone(),
        _ => bail!("codex config must be an object"),
    };

    let allow_user_ids = raw
        .get("allow_user_ids")
        .and_then(Value::as_array)
        .map(|arr| {
            arr.iter()
                .map(|x| match x {
                    Value::String(s) => s.clone(),
                    other => other.to_string(),
                })
                .collect()
        })
        .unwrap_or_default();

    let extra_args = codex_raw
        .get("extra_args")
        .and_then(Value::as_array)
        .map(|arr| {
            arr.iter()
                .map(|x| match x {
                    Value::String(s) => s.clone(),
                    other => other.to_string(),
                })
                .collect()
        })
        .unwrap_or_default();

    let state_dir = match raw.get("state_dir") {
        Some(Value::String(s)) if !s.is_empty() => PathBuf::from(s),
        _ => default_state_dir(),
    };

    let mut cfg = BridgeConfig {
        base_url: non_empty_str(raw.get("base_url"), ILINK_BASE_URL),
        token: non_empty_str(raw.get("token"), ""),
        account_id: non_empty_str(raw.get("account_id"), ""),
        allow_all: bool_or(raw.get("allow_all"), false),
        allow_user_ids,
        require_prefix: bool_or(raw.get("require_prefix"), true),
        prefix: non_empty_str(raw.get("prefix"), "/codex"),
        allow_id_command_for_unauthorized: bool_or(
            raw.get("allow_id_command_for_unauthorized"),
            true,
        ),
        poll_timeout_seconds: raw
            .get("poll_timeout_seconds")
            .and_then(Value::as_u64)
            .unwrap_or(35),
        retry_delay_seconds: raw
            .get("retry_delay_seconds")
            .and_then(Value::as_u64)
            .unwrap_or(2),
        backoff_delay_seconds: raw
            .get("backoff_delay_seconds")
            .and_then(Value::as_u64)
            .unwrap_or(30),
        max_consecutive_failures: raw
            .get("max_consecutive_failures")
            .and_then(Value::as_u64)
            .unwrap_or(3) as u32,
        max_reply_chars: raw
            .get("max_reply_chars")
            .and_then(Value::as_u64)
            .unwrap_or(1800) as usize,
        enable_typing: bool_or(raw.get("enable_typing"), true),
        state_dir,
        codex: CodexConfig {
            backend: non_empty_str(codex_raw.get("backend"), "app-server"),
            command: non_empty_str(codex_raw.get("command"), "codex"),
            app_server_command: non_empty_str(codex_raw.get("app_server_command"), ""),
            workspace: non_empty_str(codex_raw.get("workspace"), "."),
            model: non_empty_str(codex_raw.get("model"), ""),
            profile: non_empty_str(codex_raw.get("profile"), ""),
            sandbox: non_empty_str(codex_raw.get("sandbox"), "read-only"),
            approval_policy: non_empty_str(codex_raw.get("approval_policy"), "never"),
            timeout_seconds: codex_raw
                .get("timeout_seconds")
                .and_then(Value::as_u64)
                .unwrap_or(300),
            extra_args,
            prompt_prefix: non_empty_str(codex_raw.get("prompt_prefix"), DEFAULT_PROMPT_PREFIX),
        },
    };

    if let Some(t) = env_non_empty("WEIXIN_TOKEN").or_else(|| env_non_empty("ILINK_TOKEN")) {
        cfg.token = t;
    }
    if let Some(u) = env_non_empty("WEIXIN_BASE_URL").or_else(|| env_non_empty("ILINK_BASE_URL")) {
        cfg.base_url = u;
    }
    if let Some(a) = env_non_empty("WEIXIN_ACCOUNT_ID") {
        cfg.account_id = a;
    }
    Ok(cfg)
}

fn env_non_empty(key: &str) -> Option<String> {
    std::env::var(key).ok().filter(|v| !v.is_empty())
}

pub fn load_sync_buf(cfg: &BridgeConfig) -> String {
    fs::read_to_string(cfg.state_dir.join("get_updates_buf.txt")).unwrap_or_default()
}

pub fn save_sync_buf(cfg: &BridgeConfig, buf: &str) -> Result<()> {
    fs::create_dir_all(&cfg.state_dir)?;
    fs::write(cfg.state_dir.join("get_updates_buf.txt"), buf)?;
    Ok(())
}
