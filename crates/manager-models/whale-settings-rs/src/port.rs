//! The Rust port of `src/whale-settings.ts`.

#![forbid(unsafe_code)]

use crate::js;
use serde_json::{Value, json};

fn is_js_ws(c: char) -> bool {
    c.is_whitespace() || c == '\u{FEFF}'
}

fn js_trim(s: &str) -> &str {
    s.trim_matches(is_js_ws)
}

fn js_to_string(v: &Value) -> String {
    match v {
        Value::Null => "null".into(),
        Value::Bool(b) => b.to_string(),
        Value::String(s) => s.clone(),
        Value::Array(arr) => arr
            .iter()
            .map(|item| {
                if item.is_null() || js::is_undefined(item) {
                    String::new()
                } else {
                    js_to_string(item)
                }
            })
            .collect::<Vec<_>>()
            .join(","),
        Value::Number(n) => n.as_f64().map(js::number_to_string).unwrap_or_default(),
        Value::Object(_) => {
            if js::is_undefined(v) {
                "undefined".into()
            } else if let Some(n) = js::as_number(v) {
                js::number_to_string(n)
            } else {
                "[object Object]".into()
            }
        }
    }
}

fn is_valid_balance_field(s: &str) -> bool {
    let mut parts = s.split('.');
    parts.all(|seg| {
        if seg.is_empty() {
            return false;
        }
        let mut rest = seg;
        while let Some(prefix) = rest.strip_suffix(']') {
            let Some((before, digits)) = prefix.rsplit_once('[') else {
                return false;
            };
            if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
                return false;
            }
            rest = before;
        }
        !rest.is_empty()
            && rest
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'_' | b'-'))
    })
}

/// Port of `defaultWhaleBalanceSettings` (`src/whale-settings.ts:17`).
pub fn default_whale_balance_settings() -> Result<Value, String> {
    Ok(json!({
        "codexAppWhaleBalanceProtocol": "auto",
        "codexAppWhaleBalancePath": "",
        "codexAppWhaleBalanceField": "",
        "codexAppWhaleBalanceCurrency": "USD",
        "codexAppWhaleBalanceScale": 1,
    }))
}

/// Port of `normalizeWhaleBalanceSettings` (`src/whale-settings.ts:27`).
pub fn normalize_whale_balance_settings(raw: Value) -> Result<Value, String> {
    if raw.is_null() || js::is_undefined(&raw) {
        return Err(
            "TypeError: Cannot read properties of null/undefined (reading 'codexAppWhaleBalanceProtocol')"
                .into(),
        );
    }
    let obj = raw.as_object();
    let get = |k: &str| obj.and_then(|o| o.get(k));
    let protocol = match get("codexAppWhaleBalanceProtocol").and_then(Value::as_str) {
        Some("custom") => "custom",
        Some("off") => "off",
        _ => "auto",
    };
    let nullish_str = |v: Option<&Value>, def: &str| -> String {
        match v {
            None | Some(Value::Null) => def.to_string(),
            Some(val) if js::is_undefined(val) => def.to_string(),
            Some(val) => js_to_string(val),
        }
    };
    let path = js_trim(&nullish_str(get("codexAppWhaleBalancePath"), "")).to_string();
    let field = js_trim(&nullish_str(get("codexAppWhaleBalanceField"), "")).to_string();
    let currency = js_trim(&nullish_str(get("codexAppWhaleBalanceCurrency"), "USD")).to_uppercase();
    let scale = match get("codexAppWhaleBalanceScale") {
        None | Some(Value::Null) => json!(1),
        Some(v) if js::is_undefined(v) => json!(1),
        Some(v) => v.clone(),
    };
    Ok(json!({
        "codexAppWhaleBalanceProtocol": protocol,
        "codexAppWhaleBalancePath": path,
        "codexAppWhaleBalanceField": field,
        "codexAppWhaleBalanceCurrency": currency,
        "codexAppWhaleBalanceScale": scale,
    }))
}

/// Port of `whaleBalanceSettingsIssue` (`src/whale-settings.ts:41`).
pub fn whale_balance_settings_issue(
    settings: Value,
    whale_active: bool,
) -> Result<Option<String>, String> {
    if !whale_active {
        return Ok(None);
    }
    if settings.is_null() || js::is_undefined(&settings) {
        return Err(
            "TypeError: Cannot read properties of null/undefined (reading 'codexAppWhaleBalanceProtocol')"
                .into(),
        );
    }
    let Some(obj) = settings.as_object() else {
        return Ok(None);
    };
    if obj
        .get("codexAppWhaleBalanceProtocol")
        .and_then(Value::as_str)
        != Some("custom")
    {
        return Ok(None);
    }
    let path = obj
        .get("codexAppWhaleBalancePath")
        .and_then(Value::as_str)
        .unwrap_or("");
    if path.is_empty()
        || path.encode_utf16().count() > 1024
        || path.starts_with("//")
        || path
            .chars()
            .any(|c| matches!(c, '\\' | '?' | '#' | ':') || is_js_ws(c) || (c as u32) <= 0x1F)
    {
        return Ok(Some("path".into()));
    }
    let field = obj
        .get("codexAppWhaleBalanceField")
        .and_then(Value::as_str)
        .unwrap_or("");
    if field.is_empty() || field.encode_utf16().count() > 256 || !is_valid_balance_field(field) {
        return Ok(Some("field".into()));
    }
    let currency = obj
        .get("codexAppWhaleBalanceCurrency")
        .and_then(Value::as_str)
        .unwrap_or("");
    if currency.len() != 3 || !currency.bytes().all(|b| b.is_ascii_uppercase()) {
        return Ok(Some("currency".into()));
    }
    let scale = obj
        .get("codexAppWhaleBalanceScale")
        .and_then(js::as_number)
        .unwrap_or(f64::NAN);
    if !scale.is_finite() || scale <= 0.0 || scale > 1e12 {
        return Ok(Some("scale".into()));
    }
    Ok(None)
}
