//! The Rust port of `src/settings-rebase.ts`.

#![forbid(unsafe_code)]

use manager_common_rs::{
    as_number, is_nil, is_plain_object, is_undefined, number_to_string, spread_object,
    string_units, utf16_string,
};
use serde_json::{Map, Value};
use std::collections::HashMap;

fn spread_js(v: &Value) -> Map<String, Value> {
    let mut map = spread_object(v);
    if map.is_empty()
        && !is_plain_object(v)
        && !v.is_array()
        && let Some(units) = string_units(v)
    {
        for (i, u) in units.into_iter().enumerate() {
            map.insert(i.to_string(), utf16_string(&[u]));
        }
    }
    map
}

fn json_quote_u16(units: &[u16]) -> String {
    if let Ok(s) = String::from_utf16(units)
        && let Ok(q) = serde_json::to_string(&s)
    {
        return q;
    }
    let mut out = String::from('"');
    for &u in units {
        if matches!(u, 0x22 | 0x5C | 0x00..=0x1F | 0xD800..=0xDFFF) {
            out.push_str(&format!("\\u{u:04x}"));
        } else if let Some(ch) = char::from_u32(u32::from(u)) {
            out.push(ch);
        }
    }
    out.push('"');
    out
}

fn js_stringify(v: &Value) -> Option<String> {
    if is_undefined(v) {
        return None;
    }
    if let Some(n) = as_number(v) {
        return Some(if n.is_finite() {
            number_to_string(n)
        } else {
            "null".into()
        });
    }
    if let Some(units) = string_units(v) {
        return Some(json_quote_u16(&units));
    }
    match v {
        Value::Null => Some("null".into()),
        Value::Bool(b) => Some(b.to_string()),
        Value::Array(arr) => {
            let items: Vec<String> = arr
                .iter()
                .map(|x| js_stringify(x).unwrap_or_else(|| "null".into()))
                .collect();
            Some(format!("[{}]", items.join(",")))
        }
        Value::Object(map) => {
            let pairs: Vec<String> = map
                .iter()
                .filter_map(|(k, val)| {
                    Some(format!(
                        "{}:{}",
                        json_quote_u16(&k.encode_utf16().collect::<Vec<_>>()),
                        js_stringify(val)?
                    ))
                })
                .collect();
            Some(format!("{{{}}}", pairs.join(",")))
        }
        _ => None,
    }
}

fn js_strict_eq(a: &Value, b: &Value) -> bool {
    (is_undefined(a) && is_undefined(b))
        || as_number(a).zip(as_number(b)).is_some_and(|(l, r)| l == r)
        || string_units(a)
            .zip(string_units(b))
            .is_some_and(|(l, r)| l == r)
        || matches!((a, b), (Value::Null, Value::Null))
        || matches!((a, b), (Value::Bool(x), Value::Bool(y)) if x == y)
}

/// Port of `rebaseSettingsEdits` (`src/settings-rebase.ts:10`).
pub fn rebase_settings_edits(form: Value, previous: Value, latest: Value) -> Result<Value, String> {
    if is_nil(&form) {
        return Err("TypeError: Cannot convert undefined or null to object".into());
    }
    let mut merged = spread_js(&latest);
    for (key, local_val) in spread_js(&form) {
        if key == "revision" || key == "nativeAuthSelection" {
            continue;
        }
        if is_nil(&previous) {
            return Err(format!(
                "TypeError: Cannot read properties of null/undefined (reading '{key}')"
            ));
        }
        let base_val = match &previous {
            Value::Object(o) if is_plain_object(&previous) => o.get(&key).cloned(),
            _ => spread_js(&previous).remove(&key),
        }
        .unwrap_or_else(crate::js::undefined);
        if js_stringify(&local_val) != js_stringify(&base_val) {
            merged.insert(key, local_val);
        }
    }
    Ok(Value::Object(merged))
}

fn profile_str_id(p: &Value) -> Option<String> {
    is_plain_object(p)
        .then(|| p.get("id").and_then(string_units))
        .flatten()
        .map(|u| String::from_utf16_lossy(&u))
}

fn index_profiles(ps: &[Value]) -> HashMap<String, &Value> {
    ps.iter()
        .filter_map(|p| Some((profile_str_id(p)?, p)))
        .collect()
}

/// Port of `mergeRotatedAuthContents` (`src/settings-rebase.ts:31`).
pub fn merge_rotated_auth_contents(
    merged: Value,
    previous: Value,
    latest: Value,
) -> Result<Value, String> {
    if is_nil(&merged) || is_nil(&previous) || is_nil(&latest) {
        return Err(
            "TypeError: Cannot read properties of null/undefined (reading 'relayProfiles')".into(),
        );
    }
    let (Some(merged_p), Some(prev_p), Some(latest_p)) = (
        merged.get("relayProfiles").and_then(Value::as_array),
        previous.get("relayProfiles").and_then(Value::as_array),
        latest.get("relayProfiles").and_then(Value::as_array),
    ) else {
        return Ok(merged);
    };
    let (base_by_id, latest_by_id) = (index_profiles(prev_p), index_profiles(latest_p));
    let auth = |v: &Value| {
        v.get("authContents")
            .cloned()
            .unwrap_or_else(crate::js::undefined)
    };
    let mut changed = false;
    let next_profiles: Vec<Value> = merged_p
        .iter()
        .map(|profile| {
            if let Some(id) = profile_str_id(profile)
                && let (Some(&base), Some(&fresh)) = (base_by_id.get(&id), latest_by_id.get(&id))
                && js_strict_eq(&auth(profile), &auth(base))
                && !js_strict_eq(&auth(fresh), &auth(base))
            {
                changed = true;
                let mut updated = spread_js(profile);
                updated.insert("authContents".into(), auth(fresh));
                return Value::Object(updated);
            }
            profile.clone()
        })
        .collect();
    if !changed {
        return Ok(merged);
    }
    let mut out = spread_js(&merged);
    out.insert("relayProfiles".into(), Value::Array(next_profiles));
    Ok(Value::Object(out))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn rebases_local_edits_and_protects_rotated_auth() {
        let prev = json!({ "revision": "r1", "a": true, "b": false });
        let form = json!({ "revision": "r1", "a": true, "b": true });
        let latest = json!({ "revision": "r2", "a": false, "b": false });
        assert_eq!(
            rebase_settings_edits(form, prev, latest).unwrap(),
            json!({ "revision": "r2", "a": false, "b": true })
        );
        assert!(rebase_settings_edits(Value::Null, json!({}), json!({})).is_err());
        let p = json!({ "id": "p1", "authContents": "old", "apiKey": "sk-1" });
        let m = merge_rotated_auth_contents(
            json!({ "relayProfiles": [{ "id": "p1", "authContents": "old", "apiKey": "sk-2" }] }),
            json!({ "relayProfiles": [p] }),
            json!({ "relayProfiles": [{ "id": "p1", "authContents": "new", "apiKey": "sk-1" }] }),
        )
        .unwrap();
        assert_eq!(m["relayProfiles"][0]["authContents"], "new");
    }
}
