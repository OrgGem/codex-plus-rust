//! The Rust port of `src/extension-host-model.ts`.

#![forbid(unsafe_code)]

use manager_common_rs::{is_nil, is_plain_object, is_truthy, local_script_state_for, undefined};
use serde_json::Value;

fn prop(obj: &Value, key: &str) -> Value {
    obj.as_object()
        .filter(|_| is_plain_object(obj))
        .and_then(|o| o.get(key))
        .cloned()
        .unwrap_or_else(undefined)
}

/// Port of `extensionHostForTool` (`src/extension-host-model.ts:12`).
pub fn extension_host_for_tool(tool: String) -> Result<String, String> {
    Ok(if tool == "claude" { "claude" } else { "codex" }.into())
}

/// Port of `extensionsForHost` (`src/extension-host-model.ts:16`).
pub fn extensions_for_host(scripts: Vec<Value>, host: String) -> Result<Vec<Value>, String> {
    if host == "codex" {
        return Ok(scripts);
    }
    let mut out = Vec::with_capacity(scripts.len());
    for script in scripts {
        if is_nil(&script) {
            return Err(
                "TypeError: Cannot read properties of null/undefined (reading 'package')".into(),
            );
        }
        let has_claude = script
            .as_object()
            .filter(|_| is_plain_object(&script))
            .and_then(|o| o.get("package"))
            .filter(|p| is_plain_object(p))
            .and_then(|p| p.get("claudeDesktop"))
            .is_some_and(is_truthy);
        if has_claude {
            out.push(script);
        }
    }
    Ok(out)
}

/// Port of `extensionEnabledForHost` (`src/extension-host-model.ts:20`).
pub fn extension_enabled_for_host(script: Value, host: String) -> Result<Value, String> {
    if is_nil(&script) {
        return Err(
            "TypeError: Cannot read properties of null/undefined (reading 'enabled')".into(),
        );
    }
    if host == "claude" {
        let claude = prop(&script, "claude_enabled");
        if !is_nil(&claude) {
            return Ok(claude);
        }
    }
    Ok(prop(&script, "enabled"))
}

/// Port of `extensionStateForHost` (`src/extension-host-model.ts:24`).
pub fn extension_state_for_host(
    script: Value,
    host: String,
    global_enabled: bool,
) -> Result<String, String> {
    if is_nil(&script) {
        return Err(
            "TypeError: Cannot read properties of null/undefined (reading 'status')".into(),
        );
    }
    if host == "codex" {
        let enabled = script.get("enabled").is_some_and(is_truthy);
        let status = script.get("status").and_then(Value::as_str).unwrap_or("");
        return Ok(local_script_state_for(enabled, status, global_enabled).into());
    }
    if !is_truthy(&extension_enabled_for_host(script, host)?) {
        return Ok("off".into());
    }
    if !global_enabled {
        return Ok("globalOff".into());
    }
    Ok("notLoaded".into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn host_selection_filtering_enabled_and_state() {
        assert_eq!(
            extension_host_for_tool("claude".into()),
            Ok("claude".into())
        );
        for tool in ["codex", "grok", "opencode", ""] {
            assert_eq!(extension_host_for_tool(tool.into()), Ok("codex".into()));
        }

        let entries = vec![
            json!({"key": "user:legacy.js", "enabled": true, "status": "loaded"}),
            json!({"key": "builtin:codex", "enabled": true, "status": "loaded", "package": {}}),
            json!({"key": "user:null", "enabled": true, "status": "failed", "package": {"claudeDesktop": null}}),
            json!({"key": "user:claude", "enabled": false, "status": "not_loaded", "package": {"claudeDesktop": {"tools": ["status"]}}}),
            json!({"key": "user:both", "enabled": true, "status": "loaded", "package": {"claudeDesktop": {"buttons": ["status"]}}}),
        ];
        assert_eq!(
            extensions_for_host(entries.clone(), "codex".into()).map(|v| v.len()),
            Ok(5)
        );
        assert_eq!(
            extensions_for_host(entries, "claude".into())
                .map(|v| v.into_iter().map(|e| e["key"].clone()).collect::<Vec<_>>()),
            Ok(vec![json!("user:claude"), json!("user:both")])
        );
        assert!(extensions_for_host(vec![Value::Null], "claude".into()).is_err());

        let claude_only = json!({"enabled": false, "claude_enabled": true, "status": "loaded"});
        assert_eq!(
            extension_enabled_for_host(claude_only.clone(), "codex".into()),
            Ok(json!(false))
        );
        assert_eq!(
            extension_enabled_for_host(claude_only.clone(), "claude".into()),
            Ok(json!(true))
        );
        assert_eq!(
            extension_enabled_for_host(json!({}), "codex".into()),
            Ok(undefined())
        );
        assert!(extension_enabled_for_host(Value::Null, "claude".into()).is_err());

        assert_eq!(
            extension_state_for_host(claude_only.clone(), "claude".into(), true),
            Ok("notLoaded".into())
        );
        assert_eq!(
            extension_state_for_host(claude_only, "claude".into(), false),
            Ok("globalOff".into())
        );
        assert_eq!(
            extension_state_for_host(
                json!({"enabled": true, "claude_enabled": false, "status": "loaded"}),
                "codex".into(),
                true
            ),
            Ok("loaded".into())
        );
        assert!(extension_state_for_host(Value::Null, "codex".into(), true).is_err());
    }
}
