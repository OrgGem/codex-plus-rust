//! The Rust port of `src/native-auth-model.ts`.

#![forbid(unsafe_code)]

use manager_common_rs::{
    as_number, is_nil, is_truthy, is_undefined, js_str, string_units, undefined,
};
use serde_json::Value;

fn js_strict_eq(a: &Value, b: &Value) -> bool {
    if is_undefined(a) || is_undefined(b) {
        return is_undefined(a) && is_undefined(b);
    }
    if a.is_null() || b.is_null() {
        return a.is_null() && b.is_null();
    }
    if let (Some(x), Some(y)) = (as_number(a), as_number(b)) {
        return x == y;
    }
    if let (Some(x), Some(y)) = (string_units(a), string_units(b)) {
        return x == y;
    }
    matches!((a, b), (Value::Bool(x), Value::Bool(y)) if x == y)
}

/// Port of `nativeAuthSelection` (`src/native-auth-model.ts:21`).
pub fn native_auth_selection(state: Value) -> Result<String, String> {
    if is_nil(&state) {
        return Err(
            "TypeError: Cannot read properties of null/undefined (reading 'selection')".into(),
        );
    }
    let sel = state.get("selection").filter(|v| !is_nil(v));
    match sel.and_then(|s| s.get("mode")).and_then(Value::as_str) {
        Some("none") => Ok("none".into()),
        Some("account") => {
            let acct = sel
                .and_then(|s| s.get("accountRef"))
                .map_or_else(|| "undefined".to_string(), js_str);
            Ok(format!("account:{acct}"))
        }
        _ => Ok(String::new()),
    }
}

/// Port of `savedAuthLabel` (`src/native-auth-model.ts:27`).
pub fn saved_auth_label(account: Value) -> Result<String, String> {
    if is_nil(&account) {
        return Err("TypeError: Cannot read properties of null/undefined (reading 'name')".into());
    }
    let undef = undefined();
    let get = |k: &str| account.get(k).unwrap_or(&undef);
    let (name, email, id, ws) = (get("name"), get("email"), get("id"), get("workspaceId"));
    let name_ok = is_truthy(name);
    let label = if name_ok {
        name
    } else if is_truthy(email) {
        email
    } else {
        id
    };
    let same = (!name_ok && is_truthy(email)) || (name_ok && js_strict_eq(email, label));
    let mut out = js_str(label);
    if is_truthy(email) && !same {
        out.push_str(&format!(" · {}", js_str(email)));
    }
    if is_truthy(ws) {
        out.push_str(&format!(" · {}", js_str(ws)));
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn formats_selection_and_saved_label() {
        assert_eq!(
            native_auth_selection(json!({"selection": null})).unwrap(),
            ""
        );
        assert_eq!(
            native_auth_selection(json!({"selection": {"mode": "none"}})).unwrap(),
            "none"
        );
        assert_eq!(
            native_auth_selection(json!({"selection": {"mode": "account", "accountRef": "a"}}))
                .unwrap(),
            "account:a"
        );
        assert_eq!(
            saved_auth_label(json!({"id":"a","name":"A","email":"a@x","workspaceId":"ws"}))
                .unwrap(),
            "A · a@x · ws"
        );
        assert_eq!(
            saved_auth_label(json!({"id":"a","name":"","email":"a@x","workspaceId":"ws"})).unwrap(),
            "a@x · ws"
        );
        assert!(native_auth_selection(Value::Null).is_err());
        assert!(saved_auth_label(Value::Null).is_err());
    }
}
