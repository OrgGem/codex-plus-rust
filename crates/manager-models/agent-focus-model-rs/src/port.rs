//! The Rust port of `src/agent-focus-model.ts`.

#![forbid(unsafe_code)]

use crate::js;
use manager_common_rs::js_rt::{is_nil, to_number};
use serde_json::{Value, json};

const FOCUS_MODES: [&str; 3] = ["codex", "enhance", "all"];
const AGENT_FOCUS_VERSION: f64 = 1.0;

fn read_stored(stored: Option<&str>) -> Option<Value> {
    let s = stored.filter(|s| !s.is_empty())?;
    serde_json::from_str::<Value>(s)
        .ok()
        .filter(|v| v.is_object() || v.is_array())
}

fn focus_str(focus: &Option<Value>) -> Option<&str> {
    focus.as_ref().and_then(Value::as_str)
}

/// Port of `parseAgentFocus` (`src/agent-focus-model.ts:38`).
pub fn parse_agent_focus(stored: Option<String>) -> Result<Option<Value>, String> {
    let found = read_stored(stored.as_deref())
        .and_then(|v| v.get("mode").and_then(Value::as_str).map(str::to_string))
        .filter(|m| FOCUS_MODES.contains(&m.as_str()));
    Ok(found.map(Value::String))
}

/// Port of `serializeAgentFocus` (`src/agent-focus-model.ts:43`).
pub fn serialize_agent_focus(mode: Value) -> Result<String, String> {
    let val = if js::is_undefined(&mode) {
        json!({ "version": 1 })
    } else if js::as_number(&mode).is_some_and(|n| !n.is_finite()) {
        json!({ "version": 1, "mode": null })
    } else {
        json!({ "version": 1, "mode": mode })
    };
    serde_json::to_string(&val).map_err(|e| e.to_string())
}

/// Port of `shouldAskAgentFocus` (`src/agent-focus-model.ts:58`).
pub fn should_ask_agent_focus(stored: Option<String>) -> Result<bool, String> {
    if parse_agent_focus(stored.clone())?.is_none() {
        return Ok(true);
    }
    let parsed = read_stored(stored.as_deref());
    let ver = parsed
        .as_ref()
        .and_then(|v| v.get("version"))
        .map_or(f64::NAN, to_number);
    Ok(!ver.is_finite() || ver < AGENT_FOCUS_VERSION)
}

/// Port of `agentVisible` (`src/agent-focus-model.ts:68`).
pub fn agent_visible(focus: Option<Value>, agent: String) -> Result<bool, String> {
    Ok(match focus_str(&focus) {
        Some("codex") => agent == "codex",
        Some("enhance") => matches!(agent.as_str(), "codex" | "claude"),
        _ => true,
    })
}

/// Port of `providersHidden` (`src/agent-focus-model.ts:75`).
pub fn providers_hidden(focus: Option<Value>) -> Result<bool, String> {
    Ok(focus_str(&focus) == Some("enhance"))
}

/// Port of `singleAgent` (`src/agent-focus-model.ts:80`).
pub fn single_agent(focus: Option<Value>) -> Result<Value, String> {
    Ok(if focus_str(&focus) == Some("codex") {
        json!("codex")
    } else {
        Value::Null
    })
}

/// Port of `visibleAgents` (`src/agent-focus-model.ts:85`).
pub fn visible_agents(focus: Option<Value>, items: Value) -> Result<Value, String> {
    let Some(mode) = focus_str(&focus).filter(|m| matches!(*m, "codex" | "enhance")) else {
        return Ok(items);
    };
    let arr = items
        .as_array()
        .ok_or("TypeError: items.filter is not a function")?;
    let mut out = Vec::with_capacity(arr.len());
    for item in arr {
        if is_nil(item) {
            return Err(
                "TypeError: Cannot read properties of null/undefined (reading 'agent')".into(),
            );
        }
        let ok = item
            .get("agent")
            .and_then(Value::as_str)
            .is_some_and(|a| agent_visible(Some(json!(mode)), a.into()).unwrap_or(false));
        if ok {
            out.push(item.clone());
        }
    }
    Ok(Value::Array(out))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn focus_modes_roundtrip_and_visibility() {
        let s = serialize_agent_focus(json!("codex")).unwrap();
        assert_eq!(
            parse_agent_focus(Some(s.clone())).unwrap(),
            Some(json!("codex"))
        );
        assert!(!should_ask_agent_focus(Some(s)).unwrap());
        assert!(should_ask_agent_focus(Some("{\"version\":0,\"mode\":\"codex\"}".into())).unwrap());
        assert!(should_ask_agent_focus(None).unwrap());
        assert!(agent_visible(Some(json!("enhance")), "claude".into()).unwrap());
        assert!(!agent_visible(Some(json!("enhance")), "grok".into()).unwrap());
        assert!(providers_hidden(Some(json!("enhance"))).unwrap());
        assert_eq!(single_agent(Some(json!("codex"))).unwrap(), json!("codex"));
        assert_eq!(single_agent(None).unwrap(), Value::Null);
        let items = json!([{"agent":"codex"},{"agent":"claude"},{"agent":"grok"}]);
        assert_eq!(
            visible_agents(Some(json!("enhance")), items).unwrap(),
            json!([{"agent":"codex"},{"agent":"claude"}])
        );
    }
}
