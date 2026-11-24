//! The Rust port of `src/update-notification.ts`.

#![forbid(unsafe_code)]

use manager_common_rs::is_nil;
use serde_json::{Value, json};

fn is_js_ws(c: char) -> bool {
    c != '\u{0085}' && (c.is_whitespace() || c == '\u{FEFF}')
}

fn js_trim(s: &str) -> &str {
    s.trim_matches(is_js_ws)
}

/// 与后端 `parse_version_tag` 同规则：去掉前缀 v，取开头的数字段；解析不了返回 None。
fn version_parts(value: &str) -> Option<Vec<f64>> {
    let stripped = js_trim(value).trim_start_matches(['v', 'V']);
    let digits: String = stripped
        .chars()
        .take_while(|c| c.is_ascii_digit() || *c == '.')
        .collect();
    if digits.is_empty() {
        return None;
    }
    let mut parts = Vec::new();
    for part in digits.split('.') {
        if part.is_empty() {
            return None;
        }
        let parsed = part.parse::<f64>().ok()?;
        if !parsed.is_finite() {
            return None;
        }
        parts.push(parsed);
    }
    Some(parts)
}

/// Port of `updatePreferencesFrom` (`src/update-notification.ts:23`).
pub fn update_preferences_from(settings: Option<Value>) -> Result<Value, String> {
    let obj = settings
        .as_ref()
        .filter(|v| !is_nil(v))
        .and_then(Value::as_object);
    let auto_check = obj
        .and_then(|o| o.get("updateAutoCheck"))
        .is_none_or(|v| v != &Value::Bool(false));
    let auto_download = obj
        .and_then(|o| o.get("updateAutoDownload"))
        .is_none_or(|v| v != &Value::Bool(false));
    let skipped_version = match obj.and_then(|o| o.get("updateSkippedVersion")) {
        None => String::new(),
        Some(v) if is_nil(v) => String::new(),
        Some(Value::String(s)) => js_trim(s).to_string(),
        Some(_) => {
            return Err("TypeError: ((intermediate value) ?? \"\").trim is not a function".into());
        }
    };
    Ok(json!({
        "autoCheck": auto_check,
        "skippedVersion": skipped_version,
        "autoDownload": auto_download,
    }))
}

/// Port of `isVersionSkipped` (`src/update-notification.ts:44`).
pub fn is_version_skipped(candidate: String, skipped: String) -> Result<bool, String> {
    let skipped_trimmed = js_trim(&skipped);
    if skipped_trimmed.is_empty() {
        return Ok(false);
    }
    let Some(left) = version_parts(&candidate) else {
        return Ok(js_trim(&candidate) == skipped_trimmed);
    };
    let Some(right) = version_parts(&skipped) else {
        return Ok(js_trim(&candidate) == skipped_trimmed);
    };
    let len = left.len().max(right.len());
    for index in 0..len {
        let a = left.get(index).copied().unwrap_or(0.0);
        let b = right.get(index).copied().unwrap_or(0.0);
        if a != b {
            return Ok(a < b);
        }
    }
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_and_parses_update_preferences() {
        assert_eq!(
            update_preferences_from(None).ok(),
            Some(json!({"autoCheck": true, "skippedVersion": "", "autoDownload": true}))
        );
        assert_eq!(
            update_preferences_from(Some(json!({
                "updateAutoCheck": false,
                "updateSkippedVersion": " v2.0.0 ",
                "updateAutoDownload": false,
            })))
            .ok(),
            Some(json!({"autoCheck": false, "skippedVersion": "v2.0.0", "autoDownload": false}))
        );
        assert!(update_preferences_from(Some(json!({"updateSkippedVersion": 123}))).is_err());
    }

    #[test]
    fn compares_skipped_versions_like_backend() {
        assert_eq!(
            is_version_skipped("v1.11.0".into(), "".into()).ok(),
            Some(false)
        );
        assert_eq!(
            is_version_skipped("v1.11.0".into(), "1.11".into()).ok(),
            Some(true)
        );
        assert_eq!(
            is_version_skipped("V1.11.0-beta".into(), "v1.11.0".into()).ok(),
            Some(true)
        );
        assert_eq!(
            is_version_skipped("v1.11.10".into(), "v1.11.9".into()).ok(),
            Some(false)
        );
        assert_eq!(
            is_version_skipped("nightly".into(), "nightly".into()).ok(),
            Some(true)
        );
        assert_eq!(
            is_version_skipped("nightly-2".into(), "nightly".into()).ok(),
            Some(false)
        );
        assert_eq!(
            is_version_skipped("v1..2".into(), "v1.2".into()).ok(),
            Some(false)
        );
    }
}
