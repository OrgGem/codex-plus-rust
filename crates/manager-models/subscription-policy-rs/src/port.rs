//! The Rust port of `src/subscription-policy.ts`.

#![forbid(unsafe_code)]

use crate::js;
use serde_json::{Map, Value, json};

fn is_js_ws(c: char) -> bool {
    c.is_whitespace() || c == '\u{FEFF}'
}

fn js_trim(s: &str) -> &str {
    s.trim_matches(is_js_ws)
}

fn parse_js_number(raw: &str) -> f64 {
    let s = js_trim(raw);
    if s.is_empty() {
        return 0.0;
    }
    if s == "Infinity" || s == "+Infinity" {
        return f64::INFINITY;
    }
    if s == "-Infinity" {
        return f64::NEG_INFINITY;
    }
    if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        return u64::from_str_radix(hex, 16)
            .map(|n| n as f64)
            .unwrap_or(f64::NAN);
    }
    if let Some(oct) = s.strip_prefix("0o").or_else(|| s.strip_prefix("0O")) {
        return u64::from_str_radix(oct, 8)
            .map(|n| n as f64)
            .unwrap_or(f64::NAN);
    }
    if let Some(bin) = s.strip_prefix("0b").or_else(|| s.strip_prefix("0B")) {
        return u64::from_str_radix(bin, 2)
            .map(|n| n as f64)
            .unwrap_or(f64::NAN);
    }
    if !s
        .bytes()
        .all(|b| b.is_ascii_digit() || matches!(b, b'.' | b'+' | b'-' | b'e' | b'E'))
    {
        return f64::NAN;
    }
    s.parse::<f64>().unwrap_or(f64::NAN)
}

fn spread_object(v: &Value) -> Map<String, Value> {
    let mut map = Map::new();
    match v {
        Value::Object(o) if !js::is_undefined(v) => {
            for (k, val) in o {
                map.insert(k.clone(), val.clone());
            }
        }
        Value::Array(arr) => {
            for (i, val) in arr.iter().enumerate() {
                map.insert(i.to_string(), val.clone());
            }
        }
        Value::String(s) => {
            for (i, u) in s.encode_utf16().enumerate() {
                map.insert(i.to_string(), js::utf16_string(&[u]));
            }
        }
        _ => {}
    }
    map
}

/// Port of `defaultSubscriptionPolicy` (`src/subscription-policy.ts:14`).
pub fn default_subscription_policy() -> Result<Value, String> {
    Ok(json!({
        "defaultUsedPercentLimit": null,
        "windowUsedPercentLimits": {},
        "onLimit": "stop",
        "unknownUsage": "allow",
        "maxAgeSeconds": 600,
    }))
}

/// Port of `normalizeSubscriptionPolicy` (`src/subscription-policy.ts:24`).
pub fn normalize_subscription_policy(value: Option<Value>) -> Result<Value, String> {
    let obj = value
        .as_ref()
        .filter(|v| !v.is_null() && !js::is_undefined(v))
        .and_then(Value::as_object);
    let pick = |key: &str, fallback: Value| -> Value {
        match obj.and_then(|o| o.get(key)) {
            Some(v) if !v.is_null() && !js::is_undefined(v) => v.clone(),
            _ => fallback,
        }
    };
    let limits = obj
        .and_then(|o| o.get("windowUsedPercentLimits"))
        .map(spread_object)
        .unwrap_or_default();
    Ok(json!({
        "defaultUsedPercentLimit": pick("defaultUsedPercentLimit", Value::Null),
        "windowUsedPercentLimits": Value::Object(limits),
        "onLimit": pick("onLimit", json!("stop")),
        "unknownUsage": pick("unknownUsage", json!("allow")),
        "maxAgeSeconds": pick("maxAgeSeconds", json!(600)),
    }))
}

/// Port of `subscriptionPercentLimit` (`src/subscription-policy.ts:36`).
pub fn subscription_percent_limit(text: String) -> Result<Value, String> {
    if js_trim(&text).is_empty() {
        return Ok(Value::Null);
    }
    let number = parse_js_number(&text);
    if !number.is_finite() || number.fract() != 0.0 || !(0.0..=100.0).contains(&number) {
        return Ok(js::undefined());
    }
    if number == 100.0 {
        Ok(Value::Null)
    } else {
        Ok(js::number(number))
    }
}

/// Port of `withSubscriptionWindowLimit` (`src/subscription-policy.ts:43`).
pub fn with_subscription_window_limit(
    policy: Value,
    minutes: String,
    text: String,
) -> Result<Value, String> {
    if policy.is_null() || js::is_undefined(&policy) {
        return Err("TypeError: Cannot read properties of null/undefined (reading 'windowUsedPercentLimits')".into());
    }
    let mut window_limits = policy
        .as_object()
        .and_then(|o| o.get("windowUsedPercentLimits"))
        .map(spread_object)
        .unwrap_or_default();
    let limit = subscription_percent_limit(text)?;
    if js::is_undefined(&limit) {
        return Ok(policy);
    }
    let next_limits = if limit.is_null() {
        window_limits.shift_remove(&minutes);
        window_limits
    } else {
        window_limits.insert(minutes, limit);
        window_limits
    };
    let mut out = spread_object(&policy);
    out.insert(
        "windowUsedPercentLimits".to_string(),
        Value::Object(next_limits),
    );
    Ok(Value::Object(out))
}
