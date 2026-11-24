//! The Rust port of `src/agent-dashboard-model.ts`.

#![forbid(unsafe_code)]

use manager_common_rs::gateway_shared::mask_account_name;
use manager_common_rs::js_rt::{
    is_nil, is_plain_object, is_truthy, is_undefined, js_number, js_sort, js_str_u16, string_units,
    to_number, undefined, utf16_string,
};
use serde_json::{Value, json};
use std::sync::LazyLock;

static UNDEF: LazyLock<Value> = LazyLock::new(undefined);

fn field_or_undef<'a>(v: &'a Value, key: &str) -> &'a Value {
    v.as_object()
        .filter(|_| is_plain_object(v))
        .and_then(|m| m.get(key))
        .unwrap_or(&UNDEF)
}

fn mask_relay_name(val: &Value) -> Result<Value, String> {
    if is_nil(val) {
        return Err(
            "TypeError: Cannot read properties of null/undefined (reading 'replace')".into(),
        );
    }
    match val {
        Value::String(s) => Ok(Value::String(mask_account_name(s.clone())?)),
        _ => {
            let Some(units) = string_units(val) else {
                return Err("TypeError: text.replace is not a function".into());
            };
            match String::from_utf16(&units) {
                Ok(s) => Ok(Value::String(mask_account_name(s)?)),
                Err(_) => {
                    let lossy = String::from_utf16_lossy(&units);
                    let needs_mask = lossy.contains('@')
                        || ["ChatGPT", "Copilot", "SuperGrok", "GitHub Copilot"]
                            .iter()
                            .any(|p| lossy.starts_with(p));
                    if needs_mask {
                        Ok(Value::String(mask_account_name(lossy)?))
                    } else {
                        Ok(val.clone())
                    }
                }
            }
        }
    }
}

/// Port of `recentAgentRequests` (`src/agent-dashboard-model.ts:27`).
pub fn recent_agent_requests(
    entries: Value,
    agent: String,
    limit: Value,
) -> Result<Vec<Value>, String> {
    if is_nil(&entries) {
        return Err(
            "TypeError: Cannot read properties of null/undefined (reading 'filter')".into(),
        );
    }
    let arr = entries
        .as_array()
        .ok_or_else(|| "TypeError: entries.filter is not a function".to_string())?;
    let agent_u16: Vec<u16> = agent.encode_utf16().collect();
    let running_u16: Vec<u16> = "running".encode_utf16().collect();

    let mut filtered: Vec<&Value> = Vec::new();
    for entry in arr {
        if is_nil(entry) {
            return Err(
                "TypeError: Cannot read properties of null/undefined (reading 'agent')".into(),
            );
        }
        let agent_field = field_or_undef(entry, "agent");
        let matched = if is_truthy(agent_field) {
            string_units(agent_field).is_some_and(|u| u == agent_u16)
        } else {
            agent == "codex"
        };
        if matched {
            filtered.push(entry);
        }
    }

    js_sort(&mut filtered, |left, right| {
        to_number(field_or_undef(right, "timestampMs"))
            - to_number(field_or_undef(left, "timestampMs"))
    });

    let end_val = if is_undefined(&limit) {
        6.0
    } else {
        to_number(&limit)
    };
    let len = filtered.len();
    let end_idx = if end_val.is_nan() {
        0
    } else if end_val.is_infinite() {
        if end_val.is_sign_positive() { len } else { 0 }
    } else {
        let t = end_val.trunc();
        if t < 0.0 {
            ((len as f64) + t).max(0.0) as usize
        } else {
            t.min(len as f64) as usize
        }
    };

    let mut out = Vec::with_capacity(end_idx);
    for entry in &filtered[..end_idx] {
        let model_field = field_or_undef(entry, "model");
        let model = if is_truthy(model_field) {
            model_field.clone()
        } else {
            field_or_undef(entry, "upstreamModel").clone()
        };
        let provider = mask_relay_name(field_or_undef(entry, "relayName"))?;
        let running =
            string_units(field_or_undef(entry, "state")).is_some_and(|u| u == running_u16);
        out.push(json!({
            "id": field_or_undef(entry, "id"),
            "model": model,
            "provider": provider,
            "durationMs": field_or_undef(entry, "durationMs"),
            "success": field_or_undef(entry, "success"),
            "running": running,
            "timestampMs": field_or_undef(entry, "timestampMs"),
        }));
    }
    Ok(out)
}

/// Port of `agentUsageStats` (`src/agent-dashboard-model.ts:47`).
pub fn agent_usage_stats(usage: Value, agent: String) -> Result<Value, String> {
    if !is_truthy(&usage) {
        return Ok(Value::Null);
    }
    let by_agent = field_or_undef(&usage, "byAgent");
    if is_nil(by_agent) {
        return Err("TypeError: Cannot read properties of null/undefined (reading 'find')".into());
    }
    let arr = by_agent
        .as_array()
        .ok_or_else(|| "TypeError: usage.byAgent.find is not a function".to_string())?;
    let agent_u16: Vec<u16> = agent.encode_utf16().collect();

    let mut found: Option<&Value> = None;
    for item in arr {
        if is_nil(item) {
            return Err(
                "TypeError: Cannot read properties of null/undefined (reading 'key')".into(),
            );
        }
        if string_units(field_or_undef(item, "key")).is_some_and(|u| u == agent_u16) {
            found = Some(item);
            break;
        }
    }

    let Some(group) = found else {
        return Ok(json!({ "calls": 0, "successRate": null, "tokens": 0 }));
    };

    let calls = field_or_undef(group, "calls").clone();
    let success_rate = if is_truthy(&calls) {
        let c = to_number(&calls);
        let e = to_number(field_or_undef(group, "errors"));
        js_number((c - e) / c)
    } else {
        Value::Null
    };
    let inp = field_or_undef(group, "inputTokens");
    let outp = field_or_undef(group, "outputTokens");
    let tokens = if string_units(inp).is_some() || string_units(outp).is_some() {
        let mut units = js_str_u16(inp);
        units.extend(js_str_u16(outp));
        utf16_string(&units)
    } else {
        js_number(to_number(inp) + to_number(outp))
    };

    Ok(json!({
        "calls": calls,
        "successRate": success_rate,
        "tokens": tokens,
    }))
}

/// Port of `agentAccessState` (`src/agent-dashboard-model.ts:61`).
pub fn agent_access_state(view: Value) -> Result<String, String> {
    if !is_truthy(&view) {
        return Ok("missing".into());
    }
    let enabled = is_truthy(field_or_undef(&view, "enabled"));
    if enabled && is_truthy(field_or_undef(&view, "drift")) {
        return Ok("drift".into());
    }
    if enabled {
        return Ok("gateway".into());
    }
    Ok("direct".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn log(id: &str, agent: Value, ts: i64, relay: &str, state: Value) -> Value {
        json!({
            "id": id,
            "agent": agent,
            "timestampMs": ts,
            "durationMs": 1800,
            "success": true,
            "model": "claude-sonnet",
            "upstreamModel": "deepseek-v4-pro",
            "relayName": relay,
            "state": state,
        })
    }

    #[test]
    fn filters_sorts_slices_and_masks_recent_requests() {
        let entries = json!([
            log("a", json!("claude"), 1, "DeepSeek", Value::Null),
            log(
                "b",
                json!("grok"),
                5,
                "ChatGPT Pro · me@example.com",
                Value::Null
            ),
            log("c", json!("claude"), 3, "DeepSeek", json!("running")),
            log("d", Value::Null, 9, "CodexDirect", Value::Null),
            log("e", json!("claude"), 2, "DeepSeek", Value::Null),
        ]);
        let claude = recent_agent_requests(entries.clone(), "claude".into(), json!(2)).unwrap();
        assert_eq!(
            claude.iter().map(|v| v["id"].clone()).collect::<Vec<_>>(),
            vec![json!("c"), json!("e")]
        );
        assert_eq!(claude[0]["running"], json!(true));
        assert_eq!(claude[0]["provider"], json!("DeepSeek"));

        let grok = recent_agent_requests(entries.clone(), "grok".into(), undefined()).unwrap();
        assert_eq!(grok[0]["provider"], json!("ChatGPT Pro · m***@example.com"));

        let codex = recent_agent_requests(entries.clone(), "codex".into(), undefined()).unwrap();
        assert_eq!(codex.len(), 1);
        assert_eq!(codex[0]["id"], json!("d"));

        assert!(
            recent_agent_requests(entries, "claude".into(), Value::Null)
                .unwrap()
                .is_empty()
        );
        assert!(recent_agent_requests(Value::Null, "codex".into(), undefined()).is_err());
    }

    #[test]
    fn computes_usage_stats_and_access_states() {
        let usage = json!({
            "byAgent": [
                { "key": "claude", "calls": 20, "errors": 1, "inputTokens": 1000, "outputTokens": 500 },
                { "key": "zero", "calls": 0, "errors": 0, "inputTokens": 10, "outputTokens": 5 }
            ]
        });
        assert_eq!(
            agent_usage_stats(usage.clone(), "claude".into()).unwrap(),
            json!({ "calls": 20, "successRate": 0.95, "tokens": 1500 })
        );
        assert_eq!(
            agent_usage_stats(usage.clone(), "zero".into()).unwrap(),
            json!({ "calls": 0, "successRate": null, "tokens": 15 })
        );
        assert_eq!(
            agent_usage_stats(usage, "grok".into()).unwrap(),
            json!({ "calls": 0, "successRate": null, "tokens": 0 })
        );
        assert_eq!(
            agent_usage_stats(Value::Null, "claude".into()).unwrap(),
            Value::Null
        );

        assert_eq!(
            agent_access_state(json!({ "enabled": true })).unwrap(),
            "gateway"
        );
        assert_eq!(
            agent_access_state(json!({ "enabled": true, "drift": { "endpoint": "x" } })).unwrap(),
            "drift"
        );
        assert_eq!(
            agent_access_state(json!({ "enabled": false })).unwrap(),
            "direct"
        );
        assert_eq!(agent_access_state(undefined()).unwrap(), "missing");
    }
}
