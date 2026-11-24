//! The Rust port of `src/dictation-settings.ts`.

#![forbid(unsafe_code)]

use crate::js;
use serde_json::{Value, json};

const DEFAULT_BASE_URL: &str = "https://api.groq.com/openai/v1";
const DEFAULT_MODEL: &str = "whisper-large-v3-turbo";

fn is_js_ws(c: char) -> bool {
    c.is_whitespace() || c == '\u{FEFF}'
}

fn js_trim(s: &str) -> &str {
    s.trim_matches(is_js_ws)
}

fn text_field(entry: Option<&Value>, fallback: &str) -> String {
    match entry {
        Some(Value::String(s)) => js_trim(s).to_string(),
        _ => fallback.to_string(),
    }
}

/// Port of `defaultDictationSettings` (`src/dictation-settings.ts:15`).
pub fn default_dictation_settings() -> Result<Value, String> {
    Ok(json!({
        "enabled": false,
        "provider": "api",
        "baseUrl": DEFAULT_BASE_URL,
        "model": DEFAULT_MODEL,
        "apiKey": "",
        "apiKeyEnv": "",
        "language": "",
        "timeoutSeconds": 120,
    }))
}

/// Port of `normalizeDictationSettings` (`src/dictation-settings.ts:27`).
pub fn normalize_dictation_settings(value: Option<Value>) -> Result<Value, String> {
    let obj = value
        .as_ref()
        .filter(|v| !v.is_null() && !js::is_undefined(v))
        .and_then(Value::as_object);

    let enabled = obj
        .and_then(|o| o.get("enabled"))
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let base_url_raw = text_field(obj.and_then(|o| o.get("baseUrl")), DEFAULT_BASE_URL);
    let base_url = base_url_raw.trim_end_matches('/').to_string();
    let provider_str = obj
        .and_then(|o| o.get("provider"))
        .and_then(Value::as_str)
        .unwrap_or("");
    let provider =
        if provider_str == "sensevoice" || base_url.eq_ignore_ascii_case("local://sensevoice") {
            "sensevoice"
        } else {
            "api"
        };
    let timeout_seconds = obj
        .and_then(|o| o.get("timeoutSeconds"))
        .and_then(js::as_number)
        .filter(|n| n.is_finite())
        .map(|n| js::math_round(n).clamp(1.0, 600.0))
        .unwrap_or(120.0);

    Ok(json!({
        "enabled": enabled,
        "provider": provider,
        "baseUrl": base_url,
        "apiKey": text_field(obj.and_then(|o| o.get("apiKey")), ""),
        "apiKeyEnv": text_field(obj.and_then(|o| o.get("apiKeyEnv")), ""),
        "model": text_field(obj.and_then(|o| o.get("model")), DEFAULT_MODEL),
        "language": text_field(obj.and_then(|o| o.get("language")), ""),
        "timeoutSeconds": js::number(timeout_seconds),
    }))
}
