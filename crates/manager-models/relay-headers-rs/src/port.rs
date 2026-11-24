//! The Rust port of `src/relay-headers.ts`.

#![forbid(unsafe_code)]

use crate::js;
use manager_common_rs::{is_nil, is_truthy, js_str};
use serde_json::{Map, Value, json};
use std::collections::HashSet;

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

fn is_js_ws(c: char) -> bool {
    c != '\u{0085}' && (c.is_whitespace() || c == '\u{FEFF}')
}

fn js_trim(s: &str) -> &str {
    s.trim_matches(is_js_ws)
}

fn empty_map() -> &'static Map<String, Value> {
    static EMPTY: std::sync::LazyLock<Map<String, Value>> = std::sync::LazyLock::new(Map::new);
    &EMPTY
}

fn require_obj<'a>(v: &'a Value, prop: &str) -> Result<&'a Map<String, Value>, String> {
    if is_nil(v) {
        return Err(format!(
            "TypeError: Cannot read properties of null/undefined (reading '{prop}')"
        ));
    }
    Ok(v.as_object().unwrap_or(empty_map()))
}

fn trim_field(obj: &Map<String, Value>, prop: &str) -> Result<String, String> {
    match obj.get(prop) {
        Some(Value::String(s)) => Ok(js_trim(s).to_string()),
        Some(v) if is_nil(v) => {
            Err("TypeError: Cannot read properties of null/undefined (reading 'trim')".into())
        }
        None => Err("TypeError: Cannot read properties of undefined (reading 'trim')".into()),
        _ => Err(format!("TypeError: {prop}.trim is not a function")),
    }
}

fn is_token(name: &str) -> bool {
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

fn keeps_saved(obj: &Map<String, Value>, key: &str, val: &str) -> Result<bool, String> {
    if !obj.get("masked").is_some_and(is_truthy) || !val.is_empty() {
        return Ok(false);
    }
    let Some(saved) = obj.get("savedKey").filter(|v| is_truthy(v)) else {
        return Ok(false);
    };
    let Value::String(s) = saved else {
        return Err("TypeError: savedKey.trim is not a function".into());
    };
    Ok(key.to_lowercase() == js_trim(s).to_lowercase())
}

/// Port of `isForbiddenRelayHeader` (`src/relay-headers.ts:24`).
pub fn is_forbidden_relay_header(name: String) -> Result<bool, String> {
    let lowered = js_trim(&name).to_lowercase();
    Ok(FORBIDDEN_RELAY_HEADERS.contains(&lowered.as_str()))
}

/// Port of `normalizeRelayHeaders` (`src/relay-headers.ts:29`).
pub fn normalize_relay_headers(rows: Vec<Value>) -> Result<Vec<Value>, String> {
    let mut out = Vec::with_capacity(rows.len());
    for row in &rows {
        let obj = require_obj(row, "key")?;
        let key = trim_field(obj, "key")?;
        let value = trim_field(obj, "value")?;
        if !key.is_empty() || !value.is_empty() {
            out.push(json!({ "key": key, "value": value }));
        }
    }
    Ok(out)
}

/// Port of `relayHeadersValidationMessage` (`src/relay-headers.ts:44`).
pub fn relay_headers_validation_message(rows: Vec<Value>) -> Result<Option<String>, String> {
    let mut seen = HashSet::new();
    for row in normalize_relay_headers(rows)? {
        let name = row.get("key").and_then(Value::as_str).unwrap_or("");
        let value = row.get("value").and_then(Value::as_str).unwrap_or("");
        if name.is_empty() {
            return Ok(Some("自定义请求头存在空的名称".into()));
        }
        if !is_token(name) {
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

/// Port of `serializeRelayHeaders` (`src/relay-headers.ts:70`).
pub fn serialize_relay_headers(rows: Vec<Value>) -> Result<Vec<Value>, String> {
    normalize_relay_headers(rows)
}

/// Port of `hasCustomAuthorization` (`src/relay-headers.ts:75`).
pub fn has_custom_authorization(rows: Vec<Value>) -> Result<bool, String> {
    Ok(normalize_relay_headers(rows)?.into_iter().any(|row| {
        row.get("key")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_lowercase()
            == "authorization"
            && !row
                .get("value")
                .and_then(Value::as_str)
                .unwrap_or("")
                .is_empty()
    }))
}

/// Port of `providerHeaderRows` (`src/relay-headers.ts:92`).
pub fn provider_header_rows(headers: Option<Vec<Value>>) -> Result<Vec<Value>, String> {
    let Some(headers) = headers else {
        return Ok(Vec::new());
    };
    let mut out = Vec::with_capacity(headers.len());
    for header in &headers {
        let obj = require_obj(header, "masked")?;
        let key = obj.get("key").cloned().unwrap_or_else(js::undefined);
        if let Some(masked) = obj.get("masked").filter(|v| is_truthy(v)) {
            out.push(json!({ "key": key, "value": "", "masked": masked, "savedKey": key }));
        } else {
            let value = obj.get("value").cloned().unwrap_or_else(js::undefined);
            out.push(json!({ "key": key, "value": value }));
        }
    }
    Ok(out)
}

/// Port of `providerHeaderInputs` (`src/relay-headers.ts:103`).
pub fn provider_header_inputs(rows: Vec<Value>) -> Result<Vec<Value>, String> {
    let mut out = Vec::with_capacity(rows.len());
    for row in &rows {
        let obj = require_obj(row, "key")?;
        let key = trim_field(obj, "key")?;
        let value = trim_field(obj, "value")?;
        let keep = keeps_saved(obj, &key, &value)?;
        if !key.is_empty() || !value.is_empty() {
            out.push(json!({ "key": key, "value": value, "keep": keep }));
        }
    }
    Ok(out)
}

/// Port of `providerHeadersValidationMessage` (`src/relay-headers.ts:110`).
pub fn provider_headers_validation_message(rows: Vec<Value>) -> Result<Option<String>, String> {
    for row in &rows {
        let obj = require_obj(row, "masked")?;
        if !obj.get("masked").is_some_and(is_truthy) || !obj.get("savedKey").is_some_and(is_truthy)
        {
            continue;
        }
        let val = trim_field(obj, "value")?;
        if !val.is_empty() {
            continue;
        }
        let key = trim_field(obj, "key")?;
        if !key.is_empty() && !keeps_saved(obj, &key, &val)? {
            return Ok(Some(format!("改了请求头「{key}」的名称，请重新填写它的值")));
        }
    }
    relay_headers_validation_message(rows)
}

/// Port of `providerHeadersSummary` (`src/relay-headers.ts:117`).
pub fn provider_headers_summary(headers: Option<Vec<Value>>) -> Result<String, String> {
    let Some(headers) = headers else {
        return Ok(String::new());
    };
    let mut keys = Vec::new();
    for header in &headers {
        if let Some(key) = require_obj(header, "key")?
            .get("key")
            .filter(|v| is_truthy(v))
        {
            keys.push(js_str(key));
        }
    }
    Ok(keys.join(" · "))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_and_formats_headers() {
        assert_eq!(is_forbidden_relay_header("Host".into()).ok(), Some(true));
        assert!(
            relay_headers_validation_message(vec![json!({"key": "Host", "value": "x"})])
                .ok()
                .flatten()
                .is_some()
        );
        assert_eq!(
            has_custom_authorization(vec![json!({"key": "authorization", "value": "Bearer x"})])
                .ok(),
            Some(true)
        );
        let rows = provider_header_rows(Some(vec![
            json!({"key": "Authorization", "value": "", "masked": "Bear…"}),
        ]))
        .ok()
        .unwrap_or_default();
        assert_eq!(
            provider_header_inputs(rows).ok(),
            Some(vec![
                json!({"key": "Authorization", "value": "", "keep": true})
            ])
        );
        assert_eq!(
            provider_headers_summary(Some(vec![json!({"key": "X-A"})]))
                .ok()
                .as_deref(),
            Some("X-A")
        );
    }
}
