//! The Rust port of `src/provider-api-keys.ts`.

#![forbid(unsafe_code)]

use manager_common_rs::domain::es_trim;
use manager_common_rs::relay_shared::{
    normalize_relay_api_keys_value,
    relay_profile_with_normalized_api_keys as shared_with_normalized,
    relay_profile_with_updated_api_key as shared_with_updated,
};
use manager_common_rs::{string_units, utf16_string};
use serde_json::{Map, Value, json};

const API_KEY_PREVIEW_EDGE: usize = 6;

fn prepend_string_spread(profile: &Value, result: Value) -> Value {
    let Some(units) = string_units(profile) else {
        return result;
    };
    let Some(rest) = result.as_object() else {
        return result;
    };
    let mut out = Map::new();
    for (idx, &u) in units.iter().enumerate() {
        out.insert(idx.to_string(), utf16_string(&[u]));
    }
    for (k, v) in rest {
        out.insert(k.clone(), v.clone());
    }
    Value::Object(out)
}

/// Port of `normalizeRelayApiKeys` (`src/provider-api-keys.ts:17`).
pub fn normalize_relay_api_keys(profile: Value) -> Result<Value, String> {
    let (api_keys, active_api_key_id) = normalize_relay_api_keys_value(&profile)?;
    Ok(json!({
        "apiKeys": api_keys,
        "activeApiKeyId": active_api_key_id,
    }))
}

/// Port of `selectedRelayApiKey` (`src/provider-api-keys.ts:42`).
pub fn selected_relay_api_key(profile: Value) -> Result<Value, String> {
    let (api_keys, active_id) = normalize_relay_api_keys_value(&profile)?;
    Ok(api_keys
        .into_iter()
        .find(|entry| entry.get("id").and_then(Value::as_str) == Some(active_id.as_str()))
        .unwrap_or(Value::Null))
}

/// Port of `relayProfileWithNormalizedApiKeys` (`src/provider-api-keys.ts:47`).
pub fn relay_profile_with_normalized_api_keys(profile: Value) -> Result<Value, String> {
    let out = shared_with_normalized(&profile)?;
    Ok(prepend_string_spread(&profile, out))
}

/// Port of `relayProfileWithUpdatedApiKey` (`src/provider-api-keys.ts:57`).
pub fn relay_profile_with_updated_api_key(
    profile: Value,
    api_key: String,
) -> Result<Value, String> {
    let out = shared_with_updated(&profile, &api_key)?;
    Ok(prepend_string_spread(&profile, out))
}

/// Port of `apiKeyPreview` (`src/provider-api-keys.ts:83`).
pub fn api_key_preview(api_key: String) -> Result<String, String> {
    let value = es_trim(&api_key);
    let units = crate::utf16::units(value);
    if units.len() <= API_KEY_PREVIEW_EDGE * 2 + 4 {
        return Ok(String::new());
    }
    let head = crate::utf16::slice(&units, 0, API_KEY_PREVIEW_EDGE);
    let tail = crate::utf16::slice(&units, units.len() - API_KEY_PREVIEW_EDGE, units.len());
    Ok(format!("{head}…{tail}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_selects_updates_and_previews_api_keys() {
        assert_eq!(
            normalize_relay_api_keys(json!({"apiKey": "sk-legacy"})).ok(),
            Some(json!({
                "apiKeys": [{"id": "default", "name": "默认", "apiKey": "sk-legacy"}],
                "activeApiKeyId": "default"
            }))
        );
        assert_eq!(
            selected_relay_api_key(json!({"apiKey": ""})).ok(),
            Some(json!({"id": "default", "name": "默认", "apiKey": ""}))
        );
        let updated = relay_profile_with_updated_api_key(
            json!({"apiKey": "old", "relayMode": "pureApi", "noAuth": false}),
            "sk-new".into(),
        )
        .ok()
        .unwrap_or(Value::Null);
        assert_eq!(updated["apiKey"], "sk-new");
        assert_eq!(
            api_key_preview("  sk-proj-abcdefghijklmnop-123456  ".into()).ok(),
            Some("sk-pro…123456".into())
        );
        assert_eq!(
            api_key_preview("sk-1234567890ab".into()).ok(),
            Some(String::new())
        );
        assert!(normalize_relay_api_keys(Value::Null).is_err());
    }
}
