//! The Rust port of `src/provider-sync-target.ts`.

#![forbid(unsafe_code)]

use manager_common_rs::{is_nil, is_truthy, js_str};
use serde_json::Value;

fn check_target(target: &Value, prop: &str) -> Result<(), String> {
    if is_nil(target) {
        return Err(format!(
            "TypeError: Cannot read properties of null/undefined (reading '{prop}')"
        ));
    }
    Ok(())
}

/// Port of `isProviderSyncTargetSelectable` (`src/provider-sync-target.ts:7`).
pub fn is_provider_sync_target_selectable(target: Value) -> Result<bool, String> {
    check_target(&target, "isResolvable")?;
    Ok(target.get("isResolvable") == Some(&Value::Bool(true)))
}

fn find_target<F>(targets: &[Value], first_prop: &str, pred: F) -> Result<Option<String>, String>
where
    F: Fn(&Value) -> bool,
{
    for target in targets {
        check_target(target, first_prop)?;
        if pred(target) {
            return Ok(target.get("id").filter(|v| !is_nil(v)).map(js_str));
        }
    }
    Ok(None)
}

/// Port of `preferredProviderSyncTarget` (`src/provider-sync-target.ts:11`).
pub fn preferred_provider_sync_target(
    targets: Vec<Value>,
    current_provider: String,
    saved_provider: String,
) -> Result<String, String> {
    let resolvable = |t: &Value| t.get("isResolvable") == Some(&Value::Bool(true));
    if let Some(id) = find_target(&targets, "id", |t| {
        t.get("id").and_then(Value::as_str) == Some(current_provider.as_str()) && resolvable(t)
    })? {
        return Ok(id);
    }
    if let Some(id) = find_target(&targets, "isCurrentProvider", |t| {
        t.get("isCurrentProvider").is_some_and(is_truthy) && resolvable(t)
    })? {
        return Ok(id);
    }
    if let Some(id) = find_target(&targets, "id", |t| {
        t.get("id").and_then(Value::as_str) == Some(saved_provider.as_str()) && resolvable(t)
    })? {
        return Ok(id);
    }
    if let Some(id) = find_target(&targets, "isResolvable", resolvable)? {
        return Ok(id);
    }
    Ok(String::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn selects_preferred_target_and_fails_closed() {
        let targets = vec![
            json!({ "id": "relay-alpha", "isCurrentProvider": true, "isResolvable": true }),
            json!({ "id": "relay-beta", "isCurrentProvider": false, "isResolvable": true }),
        ];
        assert_eq!(
            preferred_provider_sync_target(targets, "relay-alpha".into(), "relay-beta".into())
                .unwrap(),
            "relay-alpha"
        );
        let fallback = vec![
            json!({ "id": "relay-alpha", "isCurrentProvider": true, "isResolvable": false }),
            json!({ "id": "relay-beta", "isCurrentProvider": false, "isResolvable": true }),
        ];
        assert_eq!(
            preferred_provider_sync_target(fallback, "relay-alpha".into(), "relay-beta".into())
                .unwrap(),
            "relay-beta"
        );
        assert!(
            !is_provider_sync_target_selectable(
                json!({ "id": "legacy", "isCurrentProvider": false })
            )
            .unwrap()
        );
        assert!(is_provider_sync_target_selectable(Value::Null).is_err());
    }
}
