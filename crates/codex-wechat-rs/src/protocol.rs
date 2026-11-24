//! iLink HTTP protocol, headers, message parsing, command handling, and reply splitting.

use crate::config::{BridgeConfig, CodexConfig};
use anyhow::{Context, Result};
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use regex::Regex;
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::sync::LazyLock;
use std::time::Duration;

pub const ILINK_APP_ID: &str = "bot";
pub const CHANNEL_VERSION: &str = "2.1.3";
pub const ILINK_APP_CLIENT_VERSION: &str = "131331"; // (2 << 16) | (1 << 8) | 3

pub const EP_GET_UPDATES: &str = "ilink/bot/getupdates";
pub const EP_SEND_MESSAGE: &str = "ilink/bot/sendmessage";
pub const EP_SEND_TYPING: &str = "ilink/bot/sendtyping";
pub const EP_GET_CONFIG: &str = "ilink/bot/getconfig";
pub const EP_GET_BOT_QR: &str = "ilink/bot/get_bot_qrcode";
pub const EP_GET_QR_STATUS: &str = "ilink/bot/get_qrcode_status";

pub const ITEM_TEXT: i64 = 1;
pub const ITEM_IMAGE: i64 = 2;
pub const ITEM_VIDEO: i64 = 3;
pub const ITEM_FILE: i64 = 5;
pub const ITEM_VOICE: i64 = 6;

pub const MSG_TYPE_BOT: i64 = 2;
pub const MSG_STATE_FINISH: i64 = 2;

static ANSI_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\x1b\[[0-9;?]*[ -/]*[@-~]").expect("valid regex"));

pub fn random_wechat_uin() -> String {
    let mut buf = [0_u8; 4];
    let _ = getrandom::getrandom(&mut buf);
    let val = u32::from_be_bytes(buf);
    STANDARD.encode(val.to_string().as_bytes())
}

pub fn random_hex(bytes_len: usize) -> String {
    let mut buf = vec![0_u8; bytes_len];
    let _ = getrandom::getrandom(&mut buf);
    buf.iter().map(|b| format!("{b:02x}")).collect()
}

pub fn ilink_headers(token: &str, body_len: Option<usize>) -> BTreeMap<String, String> {
    let mut headers = BTreeMap::new();
    headers.insert("Content-Type".into(), "application/json".into());
    headers.insert("AuthorizationType".into(), "ilink_bot_token".into());
    headers.insert("X-WECHAT-UIN".into(), random_wechat_uin());
    headers.insert("iLink-App-Id".into(), ILINK_APP_ID.into());
    headers.insert(
        "iLink-App-ClientVersion".into(),
        ILINK_APP_CLIENT_VERSION.into(),
    );
    if let Some(len) = body_len {
        headers.insert("Content-Length".into(), len.to_string());
    }
    if !token.is_empty() {
        headers.insert("Authorization".into(), format!("Bearer {token}"));
    }
    headers
}

pub fn url_encode_component(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    for b in raw.bytes() {
        if b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.' | b'~') {
            out.push(b as char);
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}

pub fn api_post(
    base_url: &str,
    endpoint: &str,
    mut payload: Value,
    token: &str,
    timeout_secs: u64,
) -> Result<Value> {
    if let Some(obj) = payload.as_object_mut()
        && !obj.contains_key("base_info")
    {
        obj.insert(
            "base_info".into(),
            json!({ "channel_version": CHANNEL_VERSION }),
        );
    }
    let body = serde_json::to_vec(&payload)?;
    let url = format!(
        "{}/{}",
        base_url.trim_end_matches('/'),
        endpoint.trim_start_matches('/')
    );
    let headers = ilink_headers(token, Some(body.len()));
    let agent = ureq::AgentBuilder::new()
        .timeout(Duration::from_secs(timeout_secs))
        .build();
    let mut req = agent.post(&url);
    for (k, v) in headers {
        req = req.set(&k, &v);
    }
    let resp = req
        .send_bytes(&body)
        .with_context(|| format!("POST {url} failed"))?;
    let text = resp.into_string()?;
    serde_json::from_str(&text).with_context(|| format!("invalid JSON from {url}"))
}

pub fn api_get(base_url: &str, endpoint_with_query: &str, timeout_secs: u64) -> Result<Value> {
    let url = format!(
        "{}/{}",
        base_url.trim_end_matches('/'),
        endpoint_with_query.trim_start_matches('/')
    );
    let headers = ilink_headers("", None);
    let agent = ureq::AgentBuilder::new()
        .timeout(Duration::from_secs(timeout_secs))
        .build();
    let mut req = agent.get(&url);
    for (k, v) in headers {
        req = req.set(&k, &v);
    }
    let resp = req.call().with_context(|| format!("GET {url} failed"))?;
    let text = resp.into_string()?;
    serde_json::from_str(&text).with_context(|| format!("invalid JSON from {url}"))
}

pub fn extract_text(item_list: &[Value]) -> String {
    for item in item_list {
        if item.get("type").and_then(Value::as_i64) == Some(ITEM_TEXT) {
            let text = item
                .get("text_item")
                .and_then(|t| t.get("text"))
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_string();
            let ref_item = item.get("ref_msg").and_then(|r| r.get("message_item"));
            let Some(ref_obj) = ref_item.filter(|v| v.as_object().is_some_and(|m| !m.is_empty()))
            else {
                return text;
            };
            let ref_text = extract_text(std::slice::from_ref(ref_obj));
            if !ref_text.is_empty() {
                return format!("[引用: {ref_text}]\n{text}");
            }
            return text;
        }
    }
    for item in item_list {
        if item.get("type").and_then(Value::as_i64) == Some(ITEM_VOICE) {
            return item
                .get("voice_item")
                .and_then(|v| v.get("text"))
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_string();
        }
    }
    String::new()
}

pub fn describe_non_text_items(item_list: &[Value]) -> Vec<String> {
    let mut descriptions = Vec::new();
    for item in item_list {
        match item.get("type").and_then(Value::as_i64) {
            Some(ITEM_IMAGE) => descriptions.push("[图片消息：当前 PoC 暂未下载图片]".into()),
            Some(ITEM_VIDEO) => descriptions.push("[视频消息：当前 PoC 暂未下载视频]".into()),
            Some(ITEM_FILE) => {
                let name = item
                    .get("file_item")
                    .and_then(|f| f.get("file_name"))
                    .and_then(Value::as_str)
                    .filter(|s| !s.is_empty())
                    .unwrap_or("文件");
                descriptions.push(format!("[文件消息：{name}，当前 PoC 暂未下载文件]"));
            }
            Some(ITEM_VOICE) => {
                let has_text = item
                    .get("voice_item")
                    .and_then(|v| v.get("text"))
                    .and_then(Value::as_str)
                    .is_some_and(|s| !s.is_empty());
                if !has_text {
                    descriptions.push("[语音消息：未包含转写文本]".into());
                }
            }
            _ => {}
        }
    }
    descriptions
}

pub fn normalize_prompt(text: &str, item_list: &[Value]) -> String {
    let mut parts = Vec::new();
    let trimmed = text.trim();
    if !trimmed.is_empty() {
        parts.push(trimmed.to_string());
    }
    parts.extend(describe_non_text_items(item_list));
    parts.join("\n").trim().to_string()
}

pub fn is_authorized(cfg: &BridgeConfig, user_id: &str) -> bool {
    cfg.allow_all || cfg.allow_user_ids.contains(user_id)
}

pub fn strip_prefix(cfg: &BridgeConfig, text: &str) -> (bool, String) {
    if !cfg.require_prefix {
        return (true, text.trim().to_string());
    }
    let stripped = text.trim();
    let prefix = cfg.prefix.trim();
    if prefix.is_empty() {
        return (true, stripped.to_string());
    }
    if stripped == prefix {
        return (true, String::new());
    }
    if let Some(rest) = stripped.strip_prefix(prefix)
        && (rest.starts_with(' ') || rest.starts_with('\n'))
    {
        return (true, rest.trim().to_string());
    }
    (false, stripped.to_string())
}

pub fn clean_codex_output(stdout: &str, stderr: &str) -> String {
    let s = stdout.trim();
    let base = if s.is_empty() { stderr.trim() } else { s };
    let no_ansi = ANSI_RE.replace_all(base, "");
    let mut lines: Vec<&str> = no_ansi.lines().map(|l| l.trim_end()).collect();
    while lines.first().is_some_and(|l| l.trim().is_empty()) {
        lines.remove(0);
    }
    while lines.last().is_some_and(|l| l.trim().is_empty()) {
        lines.pop();
    }
    lines.join("\n").trim().to_string()
}

pub fn build_wechat_prompt(codex: &CodexConfig, prompt: &str) -> String {
    let mut full =
        format!("请直接回答这条微信消息，不要确认收到，不要要求用户再发送消息。微信消息：{prompt}");
    if !codex.prompt_prefix.is_empty() {
        full.push('。');
        full.push_str(codex.prompt_prefix.trim());
    }
    full
}

pub fn split_reply(text: &str, max_chars: usize) -> Vec<String> {
    let chars: Vec<char> = text.chars().collect();
    if max_chars == 0 || chars.len() <= max_chars {
        return vec![text.to_string()];
    }
    let mut chunks = Vec::new();
    let mut rest = text.to_string();
    loop {
        let rc: Vec<char> = rest.chars().collect();
        if rc.len() <= max_chars {
            break;
        }
        let max_byte = rc[..max_chars].iter().map(|c| c.len_utf8()).sum::<usize>();
        let cut_char = match rest[..max_byte].rfind('\n') {
            Some(byte_idx) => {
                let c_idx = rest[..byte_idx].chars().count();
                if c_idx < max_chars / 2 {
                    max_chars
                } else {
                    c_idx
                }
            }
            None => max_chars,
        };
        let cut_byte: usize = rc[..cut_char].iter().map(|c| c.len_utf8()).sum();
        chunks.push(rest[..cut_byte].trim().to_string());
        rest = rest[cut_byte..].trim().to_string();
    }
    if !rest.is_empty() {
        chunks.push(rest);
    }
    chunks
}

pub fn handle_command(cfg: &BridgeConfig, user_id: &str, text: &str) -> String {
    let command = text.trim();
    let p = &cfg.prefix;
    if command == "/id"
        || command == "/whoami"
        || command == format!("{p} id")
        || command == format!("{p} /id")
    {
        return format!("你的 iLink user_id：{user_id}");
    }
    if command == "/ping" || command == format!("{p} ping") {
        return "pong".into();
    }
    if command == "/help" || command == format!("{p} help") {
        return format!(
            "Codex 微信助手 PoC\n\
- 发送 `{p} 你的问题` 调用 Codex\n\
- 发送 `/id` 查看你的 iLink user_id\n\
- 默认只响应 allow_user_ids 中的用户"
        );
    }
    String::new()
}

pub fn send_text(
    cfg: &BridgeConfig,
    to_user_id: &str,
    text: &str,
    context_token: &str,
) -> Result<()> {
    for chunk in split_reply(text, cfg.max_reply_chars) {
        let mut msg = json!({
            "from_user_id": "",
            "to_user_id": to_user_id,
            "client_id": format!("codex-claw-{}", random_hex(8)),
            "message_type": MSG_TYPE_BOT,
            "message_state": MSG_STATE_FINISH,
            "item_list": [{ "type": ITEM_TEXT, "text_item": { "text": chunk } }]
        });
        if !context_token.is_empty() {
            msg["context_token"] = json!(context_token);
        }
        api_post(
            &cfg.base_url,
            EP_SEND_MESSAGE,
            json!({ "msg": msg }),
            &cfg.token,
            15,
        )?;
    }
    Ok(())
}

pub fn get_typing_ticket(cfg: &BridgeConfig, user_id: &str, context_token: &str) -> Result<String> {
    let mut payload = json!({ "ilink_user_id": user_id });
    if !context_token.is_empty() {
        payload["context_token"] = json!(context_token);
    }
    let resp = api_post(&cfg.base_url, EP_GET_CONFIG, payload, &cfg.token, 10)?;
    Ok(resp
        .get("typing_ticket")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string())
}

pub fn send_typing(cfg: &BridgeConfig, user_id: &str, ticket: &str, status: i64) -> Result<()> {
    if ticket.is_empty() {
        return Ok(());
    }
    api_post(
        &cfg.base_url,
        EP_SEND_TYPING,
        json!({ "ilink_user_id": user_id, "typing_ticket": ticket, "status": status }),
        &cfg.token,
        10,
    )?;
    Ok(())
}
