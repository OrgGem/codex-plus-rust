//! Reusable cross-wave domain helpers shared across `manager-models/` crates.

#![forbid(unsafe_code)]

use crate::js_rt::{
    as_number, is_js_ws, is_nil, is_plain_object, is_truthy, is_undefined, js_str, js_trim,
    number_to_string, string_units, undefined,
};
use fancy_regex::Regex as FancyRegex;
use serde_json::{Map, Value, json};
use std::collections::HashSet;
use url::Url;

pub const DEFAULT_AUTO_COMPACT_PERCENT: &str = "90%";

pub fn is_valid_percent_pattern(s: &str) -> bool {
    let core = s.strip_suffix('%').unwrap_or(s);
    if core.is_empty() {
        return false;
    }
    match core.split_once('.') {
        Some((int_part, frac_part)) => {
            !int_part.is_empty()
                && int_part.bytes().all(|b| b.is_ascii_digit())
                && !frac_part.is_empty()
                && frac_part.len() <= 6
                && frac_part.bytes().all(|b| b.is_ascii_digit())
        }
        None => core.bytes().all(|b| b.is_ascii_digit()),
    }
}

pub fn is_valid_auto_compact_percent(value: &str) -> bool {
    let trimmed = js_trim(value);
    if trimmed.is_empty() {
        return true;
    }
    if !is_valid_percent_pattern(trimmed) {
        return false;
    }
    let body = trimmed.strip_suffix('%').unwrap_or(trimmed);
    let Ok(numeric) = body.parse::<f64>() else {
        return false;
    };
    numeric.is_finite() && numeric > 0.0 && numeric <= 100.0
}

pub fn normalize_auto_compact_percent(value: &str) -> String {
    let trimmed = js_trim(value);
    if trimmed.is_empty() {
        return String::new();
    }
    if !is_valid_auto_compact_percent(trimmed) {
        return trimmed.to_string();
    }
    let body = trimmed.strip_suffix('%').unwrap_or(trimmed);
    format!("{}%", js_trim(body))
}

pub fn split_crlf_lines(s: &str) -> Vec<&str> {
    s.split('\n')
        .map(|line| line.strip_suffix('\r').unwrap_or(line))
        .collect()
}

pub fn toml_section_string_value(
    contents: &str,
    section_name: &str,
    key: &str,
) -> Result<String, String> {
    let escaped_key = fancy_regex::escape(key);
    let pattern = format!(
        r#"^[\s\u{{FEFF}}]*{escaped_key}[\s\u{{FEFF}}]*=[\s\u{{FEFF}}]*(['"])(.*)\1[\s\u{{FEFF}}]*(?:#.*)?$"#
    );
    let re = FancyRegex::new(&pattern)
        .map_err(|e| format!("SyntaxError: Invalid regular expression: {e}"))?;
    let mut in_section = false;
    for line in split_crlf_lines(contents) {
        let trimmed = js_trim(line);
        if trimmed.starts_with('[') && trimmed.ends_with(']') && trimmed.len() >= 3 {
            let inner = &trimmed[1..trimmed.len() - 1];
            if !inner.contains(']') {
                in_section = js_trim(inner) == section_name;
                continue;
            }
        }
        if !in_section {
            continue;
        }
        if let Some(caps) = re
            .captures(trimmed)
            .map_err(|e| format!("RegexError: {e}"))?
            && let Some(m) = caps.get(2)
        {
            let mut out = String::with_capacity(m.as_str().len());
            let mut chars = m.as_str().chars().peekable();
            while let Some(c) = chars.next() {
                if c == '\\'
                    && let Some(&next) = chars.peek()
                    && matches!(next, '"' | '\'' | '\\')
                {
                    out.push(next);
                    chars.next();
                    continue;
                }
                out.push(c);
            }
            return Ok(out);
        }
    }
    Ok(String::new())
}

pub fn resolve_provider_name(contents: &str, provider: &str) -> Result<String, String> {
    let raw = toml_section_string_value(contents, &format!("model_providers.{provider}"), "name")?;
    let trimmed = raw.trim_matches(is_js_ws);
    if trimmed.is_empty() {
        Ok(provider.to_string())
    } else {
        Ok(trimmed.to_string())
    }
}

pub fn normalize_subscription_policy(value: Option<&Value>) -> Value {
    let obj = value.and_then(|v| v.as_object());
    let pick = |key: &str, fallback: Value| -> Value {
        match obj.and_then(|o| o.get(key)) {
            Some(v) if !v.is_null() && !crate::js_rt::is_undefined(v) => v.clone(),
            _ => fallback,
        }
    };
    let limits = obj
        .and_then(|o| o.get("windowUsedPercentLimits"))
        .map(crate::js_rt::spread_object)
        .unwrap_or_default();
    json!({
        "defaultUsedPercentLimit": pick("defaultUsedPercentLimit", Value::Null),
        "windowUsedPercentLimits": Value::Object(limits),
        "onLimit": pick("onLimit", json!("stop")),
        "unknownUsage": pick("unknownUsage", json!("allow")),
        "maxAgeSeconds": pick("maxAgeSeconds", json!(600)),
    })
}

pub fn local_script_state_for(enabled: bool, status: &str, global_enabled: bool) -> &'static str {
    if status == "unknown" {
        return "unknown";
    }
    if !enabled {
        return "off";
    }
    if !global_enabled {
        return "globalOff";
    }
    match status {
        "loaded" => "loaded",
        "loading" => "loading",
        "failed" => "failed",
        _ => "notLoaded",
    }
}

const GITHUB_NON_OWNER_ROUTES: &[&str] = &[
    "about",
    "account",
    "advisories",
    "apps",
    "business",
    "codespaces",
    "collections",
    "contact",
    "copilot",
    "customer-stories",
    "dashboard",
    "edu",
    "enterprise",
    "events",
    "explore",
    "features",
    "git-guides",
    "github-copilot",
    "issues",
    "login",
    "marketplace",
    "mcp",
    "mobile",
    "new",
    "newsletter",
    "newsroom",
    "notifications",
    "open-source",
    "organizations",
    "orgs",
    "pricing",
    "pulls",
    "readme",
    "resources",
    "search",
    "security",
    "settings",
    "site",
    "sitemap",
    "social-impact",
    "solutions",
    "sponsors",
    "stars",
    "team",
    "topics",
    "trending",
    "users",
    "why-github",
];

pub fn is_valid_github_owner(owner: &str) -> bool {
    !owner.is_empty()
        && owner.len() <= 39
        && !GITHUB_NON_OWNER_ROUTES.contains(&owner.to_ascii_lowercase().as_str())
        && owner
            .split('-')
            .all(|seg| !seg.is_empty() && seg.bytes().all(|b| b.is_ascii_alphanumeric()))
}

pub fn is_valid_github_repo(repo: &str) -> bool {
    !repo.is_empty()
        && repo.len() <= 100
        && repo != "."
        && repo != ".."
        && repo
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'_' | b'-'))
}

pub fn is_git_hub_repository_homepage(value: &str) -> bool {
    let Ok(url) = Url::parse(value) else {
        return false;
    };
    if url.scheme() != "https" || !matches!(url.host_str(), Some("github.com" | "www.github.com")) {
        return false;
    }
    if !url.username().is_empty()
        || url.password().is_some_and(|p| !p.is_empty())
        || url.port().is_some()
    {
        return false;
    }
    let path = url.path();
    let Some(after_lead) = path.strip_prefix('/') else {
        return false;
    };
    let trimmed = after_lead.strip_suffix('/').unwrap_or(after_lead);
    let Some((owner, raw_repo)) = trimmed.split_once('/') else {
        return false;
    };
    if owner.is_empty() || raw_repo.is_empty() || raw_repo.contains('/') {
        return false;
    }
    let repository = raw_repo.strip_suffix(".git").unwrap_or(raw_repo);
    is_valid_github_owner(owner) && is_valid_github_repo(repository)
}

// ── JS OwnKeys / JSON stringify + Model Metadata shared helpers ─────────────

pub fn is_es_ws(c: char) -> bool {
    c != '\u{0085}' && is_js_ws(c)
}

pub fn es_trim(s: &str) -> &str {
    s.trim_matches(is_es_ws)
}

fn as_array_index(key: &str) -> Option<u32> {
    if key == "0" {
        return Some(0);
    }
    if key.is_empty() || key.starts_with('0') || !key.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let n: u64 = key.parse().ok()?;
    if n < u64::from(u32::MAX) {
        u32::try_from(n).ok()
    } else {
        None
    }
}

pub fn js_own_keys_order<'a>(map: &'a Map<String, Value>) -> Vec<&'a String> {
    let mut indices: Vec<(u32, &'a String)> = Vec::new();
    let mut rest: Vec<&'a String> = Vec::new();
    for k in map.keys() {
        if let Some(idx) = as_array_index(k) {
            indices.push((idx, k));
        } else {
            rest.push(k);
        }
    }
    indices.sort_by_key(|(idx, _)| *idx);
    let mut out = Vec::with_capacity(indices.len() + rest.len());
    out.extend(indices.into_iter().map(|(_, k)| k));
    out.extend(rest);
    out
}

fn write_json_str_units(out: &mut String, units: &[u16]) {
    out.push('"');
    let mut i = 0;
    while i < units.len() {
        let u = units[i];
        match u {
            0x0022 => out.push_str("\\\""),
            0x005C => out.push_str("\\\\"),
            0x0008 => out.push_str("\\b"),
            0x000C => out.push_str("\\f"),
            0x000A => out.push_str("\\n"),
            0x000D => out.push_str("\\r"),
            0x0009 => out.push_str("\\t"),
            0x0000..=0x001F => {
                use std::fmt::Write as _;
                let _ = write!(out, "\\u{u:04x}");
            }
            0xD800..=0xDBFF if i + 1 < units.len() && (0xDC00..=0xDFFF).contains(&units[i + 1]) => {
                let hi = u32::from(u) - 0xD800;
                let lo = u32::from(units[i + 1]) - 0xDC00;
                if let Some(ch) = char::from_u32(0x10000 + ((hi << 10) | lo)) {
                    out.push(ch);
                }
                i += 1;
            }
            0xD800..=0xDFFF => {
                use std::fmt::Write as _;
                let _ = write!(out, "\\u{u:04x}");
            }
            _ => {
                if let Some(ch) = char::from_u32(u32::from(u)) {
                    out.push(ch);
                }
            }
        }
        i += 1;
    }
    out.push('"');
}

fn write_json_value_impl(out: &mut String, v: &Value) {
    if is_undefined(v) || v.is_null() {
        out.push_str("null");
        return;
    }
    if let Some(b) = v.as_bool() {
        out.push_str(if b { "true" } else { "false" });
        return;
    }
    if let Some(n) = as_number(v) {
        if n.is_finite() {
            out.push_str(&number_to_string(n));
        } else {
            out.push_str("null");
        }
        return;
    }
    if let Some(units) = string_units(v) {
        write_json_str_units(out, &units);
        return;
    }
    if let Some(arr) = v.as_array() {
        out.push('[');
        for (i, item) in arr.iter().enumerate() {
            if i > 0 {
                out.push(',');
            }
            write_json_value_impl(out, item);
        }
        out.push(']');
        return;
    }
    if let Some(o) = v.as_object() {
        if o.len() == 1 && (o.contains_key("$set") || o.contains_key("$map")) {
            out.push_str("{}");
            return;
        }
        let keys: Vec<&String> = js_own_keys_order(o)
            .into_iter()
            .filter(|k| o.get(*k).is_some_and(|val| !is_undefined(val)))
            .collect();
        out.push('{');
        for (i, k) in keys.iter().enumerate() {
            if i > 0 {
                out.push(',');
            }
            let units: Vec<u16> = k.encode_utf16().collect();
            write_json_str_units(out, &units);
            out.push(':');
            if let Some(val) = o.get(*k) {
                write_json_value_impl(out, val);
            }
        }
        out.push('}');
    }
}

pub fn js_stringify_compact(v: &Value) -> String {
    let mut out = String::new();
    write_json_value_impl(&mut out, v);
    out
}

pub struct ModelRowIdentity {
    pub raw_name: String,
    pub canonical_slug: String,
    pub suffix_window: Option<String>,
    pub key: String,
}

pub fn suffix_window_tokens_u64(suffix: &str) -> Option<u64> {
    let s = es_trim(suffix);
    if s.is_empty() {
        return None;
    }
    let (digits, mult) = match s.as_bytes().last().copied() {
        Some(b'k' | b'K') => (&s[..s.len() - 1], 1_000u128),
        Some(b'm' | b'M') => (&s[..s.len() - 1], 1_000_000u128),
        _ => (s, 1u128),
    };
    if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let n: u128 = digits.parse().ok()?;
    let tokens = n.checked_mul(mult)?;
    if tokens > 0 && tokens <= 9_007_199_254_740_991u128 {
        u64::try_from(tokens).ok()
    } else {
        None
    }
}

pub fn parse_model_row_identity(row_name: &str) -> ModelRowIdentity {
    let trimmed = es_trim(row_name);
    let mut suffix_window: Option<u64> = None;
    let mut canonical = trimmed;
    if let Some(before_close) = trimmed.strip_suffix(']')
        && let Some(open_idx) = before_close.rfind('[')
    {
        let prefix = &before_close[..open_idx];
        let inner = &before_close[open_idx + 1..];
        if !prefix
            .chars()
            .any(|c| matches!(c, '\n' | '\r' | '\u{2028}' | '\u{2029}'))
            && !es_trim(prefix).is_empty()
            && let Some(tokens) = suffix_window_tokens_u64(es_trim(inner))
        {
            suffix_window = Some(tokens);
            canonical = es_trim(prefix);
        }
    }
    ModelRowIdentity {
        raw_name: row_name.to_string(),
        canonical_slug: canonical.to_string(),
        suffix_window: suffix_window.map(|t| t.to_string()),
        key: canonical.to_lowercase(),
    }
}

pub fn model_slug_from_row_name(row_name: &str) -> String {
    parse_model_row_identity(row_name).canonical_slug
}

pub fn suffix_window_string(row_name: &str) -> Option<String> {
    parse_model_row_identity(row_name).suffix_window
}

pub fn model_metadata_key(row_name: &str) -> String {
    parse_model_row_identity(row_name).key
}

fn is_imported_metadata_field(key: &str) -> bool {
    !matches!(
        key,
        "slug" | "context_window" | "max_context_window" | "auto_compact_token_limit"
    )
}

pub fn parse_model_metadata_map(value: &str) -> Map<String, Value> {
    if es_trim(value).is_empty() {
        return Map::new();
    }
    let Ok(parsed) = serde_json::from_str::<Value>(value) else {
        return Map::new();
    };
    if !is_plain_object(&parsed) {
        return Map::new();
    }
    let Some(obj) = parsed.as_object() else {
        return Map::new();
    };
    let mut canonical = Map::new();
    for slug in js_own_keys_order(obj) {
        let Some(entry) = obj.get(slug) else {
            continue;
        };
        if !is_plain_object(entry) {
            continue;
        }
        let Some(entry_obj) = entry.as_object() else {
            continue;
        };
        let key = model_metadata_key(slug);
        if key.is_empty() {
            continue;
        }
        let mut filtered = Map::new();
        for ek in js_own_keys_order(entry_obj) {
            if is_imported_metadata_field(ek)
                && let Some(ev) = entry_obj.get(ek)
            {
                filtered.insert(ek.clone(), ev.clone());
            }
        }
        canonical.insert(key, Value::Object(filtered));
    }
    canonical
}

pub fn serialize_model_metadata_map(map: &Map<String, Value>) -> String {
    if map.is_empty() {
        String::new()
    } else {
        js_stringify_compact(&Value::Object(map.clone()))
    }
}

// ── Relay Headers shared helpers ────────────────────────────────────────────

const FORBIDDEN_RELAY_HEADERS: &[&str] = &[
    "host",
    "content-length",
    "transfer-encoding",
    "connection",
    "keep-alive",
    "proxy-connection",
    "te",
    "trailer",
    "upgrade",
    "expect",
];

fn hdr_empty_map() -> &'static Map<String, Value> {
    static EMPTY: std::sync::LazyLock<Map<String, Value>> = std::sync::LazyLock::new(Map::new);
    &EMPTY
}

fn hdr_require_obj<'a>(v: &'a Value, prop: &str) -> Result<&'a Map<String, Value>, String> {
    if is_nil(v) {
        return Err(format!(
            "TypeError: Cannot read properties of null/undefined (reading '{prop}')"
        ));
    }
    Ok(v.as_object().unwrap_or(hdr_empty_map()))
}

fn hdr_trim_field(obj: &Map<String, Value>, prop: &str) -> Result<String, String> {
    match obj.get(prop) {
        Some(Value::String(s)) => Ok(es_trim(s).to_string()),
        Some(v) if is_nil(v) => {
            Err("TypeError: Cannot read properties of null/undefined (reading 'trim')".into())
        }
        None => Err("TypeError: Cannot read properties of undefined (reading 'trim')".into()),
        _ => Err(format!("TypeError: {prop}.trim is not a function")),
    }
}

fn hdr_is_token(name: &str) -> bool {
    !name.is_empty()
        && name.bytes().all(|b| {
            b.is_ascii_alphanumeric()
                || matches!(
                    b,
                    b'!' | b'#'
                        | b'$'
                        | b'%'
                        | b'&'
                        | b'\''
                        | b'*'
                        | b'+'
                        | b'-'
                        | b'.'
                        | b'^'
                        | b'_'
                        | b'`'
                        | b'|'
                        | b'~'
                )
        })
}

fn hdr_keeps_saved(obj: &Map<String, Value>, key: &str, val: &str) -> Result<bool, String> {
    if !obj.get("masked").is_some_and(is_truthy) || !val.is_empty() {
        return Ok(false);
    }
    let Some(saved) = obj.get("savedKey").filter(|v| is_truthy(v)) else {
        return Ok(false);
    };
    let Value::String(s) = saved else {
        return Err("TypeError: savedKey.trim is not a function".into());
    };
    Ok(key.to_lowercase() == es_trim(s).to_lowercase())
}

pub fn normalize_relay_headers(rows: &[Value]) -> Result<Vec<Value>, String> {
    let mut out = Vec::with_capacity(rows.len());
    for row in rows {
        let obj = hdr_require_obj(row, "key")?;
        let key = hdr_trim_field(obj, "key")?;
        let value = hdr_trim_field(obj, "value")?;
        if !key.is_empty() || !value.is_empty() {
            out.push(json!({ "key": key, "value": value }));
        }
    }
    Ok(out)
}

pub fn relay_headers_validation_message(rows: &[Value]) -> Result<Option<String>, String> {
    let mut seen = HashSet::new();
    for row in normalize_relay_headers(rows)? {
        let name = row.get("key").and_then(Value::as_str).unwrap_or("");
        let value = row.get("value").and_then(Value::as_str).unwrap_or("");
        if name.is_empty() {
            return Ok(Some("自定义请求头存在空的名称".into()));
        }
        if !hdr_is_token(name) {
            return Ok(Some(format!(
                "自定义请求头「{name}」不是合法的 HTTP 头名称"
            )));
        }
        let lowered = name.to_lowercase();
        if FORBIDDEN_RELAY_HEADERS.contains(&lowered.as_str()) {
            return Ok(Some(format!(
                "自定义请求头「{name}」由协议层掌控，不允许覆盖（Host、Content-Length 等传输头）"
            )));
        }
        if !seen.insert(lowered) {
            return Ok(Some(format!("自定义请求头「{name}」重复配置")));
        }
        if value.contains('\r') || value.contains('\n') {
            return Ok(Some(format!("自定义请求头「{name}」的值不能包含换行")));
        }
    }
    Ok(None)
}

pub fn provider_header_rows(headers: Option<&[Value]>) -> Result<Vec<Value>, String> {
    let Some(headers) = headers else {
        return Ok(Vec::new());
    };
    let mut out = Vec::with_capacity(headers.len());
    for header in headers {
        let obj = hdr_require_obj(header, "masked")?;
        let key = obj.get("key").cloned().unwrap_or_else(undefined);
        if let Some(masked) = obj.get("masked").filter(|v| is_truthy(v)) {
            out.push(json!({ "key": key, "value": "", "masked": masked, "savedKey": key }));
        } else {
            let value = obj.get("value").cloned().unwrap_or_else(undefined);
            out.push(json!({ "key": key, "value": value }));
        }
    }
    Ok(out)
}

pub fn provider_header_inputs(rows: &[Value]) -> Result<Vec<Value>, String> {
    let mut out = Vec::with_capacity(rows.len());
    for row in rows {
        let obj = hdr_require_obj(row, "key")?;
        let key = hdr_trim_field(obj, "key")?;
        let value = hdr_trim_field(obj, "value")?;
        let keep = hdr_keeps_saved(obj, &key, &value)?;
        if !key.is_empty() || !value.is_empty() {
            out.push(json!({ "key": key, "value": value, "keep": keep }));
        }
    }
    Ok(out)
}

pub fn provider_headers_validation_message(rows: &[Value]) -> Result<Option<String>, String> {
    for row in rows {
        let obj = hdr_require_obj(row, "masked")?;
        if !obj.get("masked").is_some_and(is_truthy) || !obj.get("savedKey").is_some_and(is_truthy)
        {
            continue;
        }
        let val = hdr_trim_field(obj, "value")?;
        if !val.is_empty() {
            continue;
        }
        let key = hdr_trim_field(obj, "key")?;
        if !key.is_empty() && !hdr_keeps_saved(obj, &key, &val)? {
            return Ok(Some(format!("改了请求头「{key}」的名称，请重新填写它的值")));
        }
    }
    relay_headers_validation_message(rows)
}

pub fn provider_headers_summary(headers: Option<&[Value]>) -> Result<String, String> {
    let Some(headers) = headers else {
        return Ok(String::new());
    };
    let mut keys = Vec::new();
    for header in headers {
        if let Some(key) = hdr_require_obj(header, "key")?
            .get("key")
            .filter(|v| is_truthy(v))
        {
            keys.push(js_str(key));
        }
    }
    Ok(keys.join(" · "))
}
