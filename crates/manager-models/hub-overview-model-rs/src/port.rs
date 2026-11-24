//! The Rust port of `src/hub-overview-model.ts`.

#![forbid(unsafe_code)]

use manager_common_rs::gateway_shared::{
    glance_agents, mask_account_name, member_status, model_display, provider_enabled,
};
use manager_common_rs::{is_nil, is_truthy, js_number, js_sort, js_str, to_number, undefined};
use serde_json::{Value, json};

fn js_max(a: f64, b: f64) -> f64 {
    if a.is_nan() || b.is_nan() {
        f64::NAN
    } else if a > b {
        a
    } else {
        b
    }
}

fn require_arr<'a>(v: &'a Value, ctx: &str) -> Result<&'a [Value], String> {
    v.as_array()
        .map(Vec::as_slice)
        .ok_or_else(|| format!("TypeError: {ctx} is not iterable"))
}

fn activity_agents(payload: &Value) -> Result<&[Value], String> {
    if is_nil(payload) {
        return Ok(&[]);
    }
    let Some(act) = payload.get("activity").filter(|v| !is_nil(v)) else {
        return Ok(&[]);
    };
    match act.get("agents") {
        None => Ok(&[]),
        Some(v) if is_nil(v) => Ok(&[]),
        Some(v) => require_arr(v, "activity.agents"),
    }
}

fn map_lookup(calls_by_provider: &Value, key: &Value) -> Result<Option<Value>, String> {
    let Some(entries) = calls_by_provider
        .as_object()
        .filter(|o| o.len() == 1)
        .and_then(|o| o.get("$map"))
        .and_then(Value::as_array)
    else {
        return Err("TypeError: callsByProvider.get is not a function".into());
    };
    for pair in entries.iter().rev() {
        if let Some(arr) = pair.as_array()
            && arr.first() == Some(key)
        {
            return Ok(Some(arr.get(1).cloned().unwrap_or_else(undefined)));
        }
    }
    Ok(None)
}

/// Port of `hubAgents` (`src/hub-overview-model.ts:52`).
pub fn hub_agents(payload: Value, codex: Value) -> Result<Vec<Value>, String> {
    let activity = activity_agents(&payload)?;
    let raw_agents = if is_nil(&payload) {
        Value::Array(Vec::new())
    } else {
        match payload.get("agents") {
            None => Value::Array(Vec::new()),
            Some(v) if is_nil(v) => Value::Array(Vec::new()),
            Some(v) => {
                let _ = require_arr(v, "payload?.agents")?;
                v.clone()
            }
        }
    };
    let mut codex_via_gateway = false;
    if let Some(arr) = raw_agents.as_array() {
        for agent in arr {
            if is_nil(agent) {
                return Err(
                    "TypeError: Cannot read properties of null/undefined (reading 'agent')".into(),
                );
            }
            if agent.get("agent").and_then(Value::as_str) == Some("codex")
                && agent.get("enabled").is_some_and(is_truthy)
            {
                codex_via_gateway = true;
                break;
            }
        }
    }

    let mut nodes = if codex_via_gateway {
        Vec::new()
    } else {
        if is_nil(&codex) {
            return Err(
                "TypeError: Cannot read properties of null/undefined (reading 'providerName')"
                    .into(),
            );
        }
        let pname = match codex.get("providerName") {
            None => String::new(),
            Some(v) if is_nil(v) => String::new(),
            Some(v) => js_str(v),
        };
        vec![json!({
            "id": "codex",
            "name": "Codex",
            "enabled": false,
            "direct": true,
            "detail": mask_account_name(pname)?,
            "requests": 0,
            "live": false,
        })]
    };

    for agent in glance_agents(raw_agents)? {
        let agent_id_val = agent.get("agent").cloned().unwrap_or_else(undefined);
        if agent_id_val.as_str() == Some("codex") && !codex_via_gateway {
            continue;
        }
        let binding = if is_nil(&payload) {
            None
        } else {
            let gw = payload
                .get("gateway")
                .filter(|v| !is_nil(v))
                .ok_or_else(|| {
                    "TypeError: Cannot read properties of undefined (reading 'bindings')"
                        .to_string()
                })?;
            let bindings = gw.get("bindings").filter(|v| !is_nil(v)).ok_or_else(|| {
                "TypeError: Cannot read properties of undefined (reading 'agent')".to_string()
            })?;
            let key = js_str(&agent_id_val);
            bindings.get(&key)
        };
        let stats = activity
            .iter()
            .find(|item| item.get("agent") == Some(&agent_id_val));
        let model = if let Some(b) = binding.filter(|v| is_truthy(v))
            && b.get("enabled").is_some_and(is_truthy)
        {
            let raw_model = match b.get("model") {
                None => {
                    return Err(
                        "TypeError: Cannot read properties of undefined (reading 'trim')".into(),
                    );
                }
                Some(v) if is_nil(v) => {
                    return Err(
                        "TypeError: Cannot read properties of null/undefined (reading 'trim')"
                            .into(),
                    );
                }
                Some(Value::String(s)) => s.clone(),
                Some(_) => return Err("TypeError: binding.model.trim is not a function".into()),
            };
            let exclude = b
                .get("poolExclude")
                .filter(|v| !is_nil(v))
                .cloned()
                .unwrap_or_else(|| Value::Array(Vec::new()));
            let view = model_display(payload.clone(), raw_model, exclude, undefined())?;
            view.get("title").cloned().unwrap_or_else(undefined)
        } else {
            Value::String(String::new())
        };
        let enabled_val = agent.get("enabled").cloned().unwrap_or_else(undefined);
        let requests_val = stats
            .and_then(|s| s.get("requests"))
            .filter(|v| !is_nil(v))
            .cloned()
            .unwrap_or_else(|| json!(0));
        let live_val = if is_truthy(&enabled_val) {
            let active_stats = if let Some(s) = stats.filter(|v| is_truthy(v)) {
                let in_flight = s.get("inFlight").ok_or_else(|| {
                    "TypeError: Cannot read properties of undefined (reading 'length')".to_string()
                })?;
                if is_nil(in_flight) {
                    return Err(
                        "TypeError: Cannot read properties of null/undefined (reading 'length')"
                            .into(),
                    );
                }
                let in_flight_len = match in_flight {
                    Value::Array(arr) => Some(arr.len()),
                    Value::String(st) => Some(st.encode_utf16().count()),
                    _ => None,
                };
                if in_flight_len.is_some_and(|n| n > 0) {
                    true
                } else {
                    s.get("requests").map(to_number).unwrap_or(f64::NAN) > 0.0
                }
            } else {
                false
            };
            Value::Bool(active_stats)
        } else if enabled_val == Value::Bool(false) {
            Value::Bool(false)
        } else {
            enabled_val.clone()
        };
        nodes.push(json!({
            "id": agent_id_val,
            "name": agent.get("name").cloned().unwrap_or_else(undefined),
            "enabled": enabled_val,
            "direct": false,
            "detail": model,
            "requests": requests_val,
            "live": live_val,
        }));
    }
    Ok(nodes)
}

/// Port of `hubProviders` (`src/hub-overview-model.ts:77`).
pub fn hub_providers(payload: Value, calls_by_provider: Value) -> Result<Vec<Value>, String> {
    let pool = if is_nil(&payload) {
        &[][..]
    } else {
        match payload.get("pool") {
            None => &[][..],
            Some(v) if is_nil(v) => &[][..],
            Some(v) => require_arr(v, "payload?.pool")?,
        }
    };
    let has_map = is_truthy(&calls_by_provider);
    let mut out = Vec::new();
    for entry in pool {
        if !provider_enabled(entry.clone())? {
            continue;
        }
        let id_val = entry.get("id").cloned().unwrap_or_else(undefined);
        let id_str = js_str(&id_val);
        let raw_name = entry.get("name").map(js_str).unwrap_or_default();
        let calls = if has_map {
            match map_lookup(&calls_by_provider, &id_val)? {
                Some(v) if !is_nil(&v) => v,
                _ => json!(0),
            }
        } else {
            Value::Null
        };
        out.push(json!({
            "id": id_val,
            "name": mask_account_name(raw_name)?,
            "baseUrl": entry.get("baseUrl").cloned().unwrap_or_else(undefined),
            "keyCount": entry.get("keyCount").cloned().unwrap_or_else(undefined),
            "status": member_status(payload.clone(), id_str, undefined())?,
            "calls": calls,
        }));
    }
    Ok(out)
}

/// Port of `hubPoolState` (`src/hub-overview-model.ts:89`).
pub fn hub_pool_state(payload: Value) -> Result<String, String> {
    if !is_truthy(&payload) {
        return Ok("loading".into());
    }
    let pool = payload
        .get("pool")
        .filter(|v| !is_nil(v))
        .and_then(Value::as_array)
        .ok_or_else(|| {
            "TypeError: Cannot read properties of null/undefined (reading 'length')".to_string()
        })?;
    if pool.is_empty() {
        return Ok("empty".into());
    }
    for entry in pool {
        if provider_enabled(entry.clone())? {
            return Ok("ready".into());
        }
    }
    Ok("allOff".into())
}

/// Port of `hubLinks` (`src/hub-overview-model.ts:96`).
pub fn hub_links(agents: Value, providers: Value) -> Result<Vec<Value>, String> {
    let agent_list = require_arr(&agents, "agents")?;
    let provider_list = require_arr(&providers, "providers")?;
    let mut max_requests = 1.0_f64;
    for agent in agent_list {
        if is_nil(agent) {
            return Err(
                "TypeError: Cannot read properties of null/undefined (reading 'requests')".into(),
            );
        }
        let reqs = agent.get("requests").map(to_number).unwrap_or(f64::NAN);
        max_requests = js_max(max_requests, reqs);
    }
    let mut max_calls = 1.0_f64;
    for provider in provider_list {
        if is_nil(provider) {
            return Err(
                "TypeError: Cannot read properties of null/undefined (reading 'calls')".into(),
            );
        }
        let calls = provider
            .get("calls")
            .filter(|v| !is_nil(v))
            .map(to_number)
            .unwrap_or(0.0);
        max_calls = js_max(max_calls, calls);
    }
    let mut links = Vec::with_capacity(agent_list.len() + provider_list.len());
    for agent in agent_list {
        let id = js_str(agent.get("id").unwrap_or(&undefined()));
        let direct = agent.get("direct").is_some_and(is_truthy);
        let enabled = agent.get("enabled").is_some_and(is_truthy);
        let live = agent.get("live").is_some_and(is_truthy);
        let tone = if direct {
            "direct"
        } else if !enabled {
            "off"
        } else if live {
            "live"
        } else {
            "idle"
        };
        let reqs = agent.get("requests").map(to_number).unwrap_or(f64::NAN);
        links.push(json!({
            "from": format!("agent:{id}"),
            "to": "hub",
            "tone": tone,
            "weight": js_number(reqs / max_requests),
        }));
    }
    for provider in provider_list {
        let id = js_str(provider.get("id").unwrap_or(&undefined()));
        let state = provider
            .get("status")
            .filter(|v| !is_nil(v))
            .ok_or_else(|| {
                "TypeError: Cannot read properties of null/undefined (reading 'state')".to_string()
            })?
            .get("state")
            .and_then(Value::as_str);
        let tone = match state {
            Some("busy" | "answered") => "live",
            Some("cooling") => "warn",
            _ => "idle",
        };
        let calls = provider
            .get("calls")
            .filter(|v| !is_nil(v))
            .map(to_number)
            .unwrap_or(0.0);
        links.push(json!({
            "from": "hub",
            "to": format!("provider:{id}"),
            "tone": tone,
            "weight": js_number(calls / max_calls),
        }));
    }
    Ok(links)
}

fn build_feed_item(agent: Value, record: &Value) -> Result<Value, String> {
    if is_nil(record) {
        return Err("TypeError: Cannot read properties of null/undefined (reading 'steps')".into());
    }
    let steps = record
        .get("steps")
        .filter(|v| !is_nil(v))
        .and_then(Value::as_array)
        .ok_or_else(|| "TypeError: record.steps is not iterable".to_string())?;
    let mut answered = None;
    for step in steps.iter().rev() {
        if is_nil(step) {
            return Err(
                "TypeError: Cannot read properties of null/undefined (reading 'outcome')".into(),
            );
        }
        if step.get("outcome").and_then(Value::as_str) == Some("answered") {
            answered = Some(step);
            break;
        }
    }
    let target = match answered.or_else(|| steps.last()) {
        Some(t) => Some(t),
        None => record
            .get("planned")
            .filter(|v| !is_nil(v))
            .and_then(Value::as_array)
            .ok_or_else(|| {
                "TypeError: Cannot read properties of undefined (reading '0')".to_string()
            })?
            .first(),
    };
    let rerouted =
        record.get("result").and_then(Value::as_str) == Some("answered") && steps.len() > 1;
    let model_val = record.get("model").cloned().unwrap_or_else(undefined);
    let requested = record
        .get("requestedModel")
        .filter(|v| is_truthy(v))
        .cloned()
        .unwrap_or_else(|| model_val.clone());
    let raw_provider = target
        .and_then(|t| t.get("relayName"))
        .filter(|v| !is_nil(v))
        .map(js_str)
        .unwrap_or_default();
    let model_out = target
        .and_then(|t| t.get("model"))
        .filter(|v| !is_nil(v))
        .cloned()
        .unwrap_or(model_val);
    let result_out = if rerouted {
        json!("rerouted")
    } else {
        record.get("result").cloned().unwrap_or_else(undefined)
    };
    Ok(json!({
        "agent": agent,
        "requested": requested,
        "provider": mask_account_name(raw_provider)?,
        "model": model_out,
        "elapsedMs": record.get("elapsedMs").cloned().unwrap_or_else(undefined),
        "result": result_out,
        "finishedAtMs": record.get("finishedAtMs").cloned().unwrap_or_else(undefined),
    }))
}

/// Port of `hubFeed` (`src/hub-overview-model.ts:118`).
pub fn hub_feed(payload: Value) -> Result<Vec<Value>, String> {
    let mut items = Vec::new();
    for agent in activity_agents(&payload)? {
        if is_nil(agent) {
            return Err(
                "TypeError: Cannot read properties of null/undefined (reading 'last')".into(),
            );
        }
        if let Some(last) = agent.get("last").filter(|v| is_truthy(v)) {
            let agent_id = agent.get("agent").cloned().unwrap_or_else(undefined);
            items.push(build_feed_item(agent_id, last)?);
        }
    }
    js_sort(&mut items, |left, right| {
        let r = right.get("finishedAtMs").map(to_number).unwrap_or(f64::NAN);
        let l = left.get("finishedAtMs").map(to_number).unwrap_or(f64::NAN);
        r - l
    });
    Ok(items)
}

/// Port of `hubTotals` (`src/hub-overview-model.ts:163`).
pub fn hub_totals(payload: Value) -> Result<Value, String> {
    let activity = activity_agents(&payload)?;
    let mut requests = 0.0_f64;
    let mut rerouted = 0.0_f64;
    for item in activity {
        if is_nil(item) {
            return Err(
                "TypeError: Cannot read properties of null/undefined (reading 'requests')".into(),
            );
        }
        requests += item.get("requests").map(to_number).unwrap_or(f64::NAN);
        rerouted += item.get("rerouted").map(to_number).unwrap_or(f64::NAN);
    }
    let (cooling_keys, open_breakers) = if is_nil(&payload) {
        (0usize, 0usize)
    } else {
        let health = payload
            .get("health")
            .filter(|v| !is_nil(v))
            .ok_or_else(|| {
                "TypeError: Cannot read properties of undefined (reading 'keys')".to_string()
            })?;
        let keys = match health.get("keys") {
            None => &[][..],
            Some(v) if is_nil(v) => &[][..],
            Some(v) => require_arr(v, "health.keys")?,
        };
        let mut cooling = 0usize;
        for key in keys {
            if is_nil(key) {
                return Err(
                    "TypeError: Cannot read properties of null/undefined (reading 'state')".into(),
                );
            }
            if key.get("state").and_then(Value::as_str) != Some("ok") {
                cooling += 1;
            }
        }
        let providers = match health.get("providers") {
            None => &[][..],
            Some(v) if is_nil(v) => &[][..],
            Some(v) => require_arr(v, "health.providers")?,
        };
        let mut breakers = 0usize;
        for prov in providers {
            if is_nil(prov) {
                return Err(
                    "TypeError: Cannot read properties of null/undefined (reading 'breakerOpen')"
                        .into(),
                );
            }
            if prov.get("breakerOpen").is_some_and(is_truthy) {
                breakers += 1;
            }
        }
        (cooling, breakers)
    };
    Ok(json!({
        "requests": js_number(requests),
        "rerouted": js_number(rerouted),
        "coolingKeys": cooling_keys,
        "openBreakers": open_breakers,
    }))
}

/// Port of `successRate` (`src/hub-overview-model.ts:173`).
pub fn success_rate(counts: Value) -> Result<Value, String> {
    if is_nil(&counts) {
        return Err("TypeError: Cannot read properties of null/undefined (reading 'calls')".into());
    }
    let calls = counts.get("calls").map(to_number).unwrap_or(f64::NAN);
    if calls > 0.0 {
        let errors = counts.get("errors").map(to_number).unwrap_or(f64::NAN);
        Ok(js_number((calls - errors) / calls))
    } else {
        Ok(Value::Null)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn computes_hub_overview_models_and_success_rate() {
        assert_eq!(hub_pool_state(Value::Null).ok(), Some("loading".into()));
        assert_eq!(
            hub_pool_state(json!({"pool": []})).ok(),
            Some("empty".into())
        );
        assert_eq!(
            hub_pool_state(json!({"pool": [{"id": "a", "enabled": false}]})).ok(),
            Some("allOff".into())
        );
        assert_eq!(
            hub_pool_state(json!({"pool": [{"id": "a"}]})).ok(),
            Some("ready".into())
        );
        let agents = hub_agents(
            json!({
                "gateway": {"version": 1, "bindings": {}, "keyPolicy": {"mode": "sticky"}, "restoreOnExit": true},
                "pool": [],
                "groups": [],
                "agents": [{"agent": "grok", "name": "Grok", "enabled": false}],
                "health": {"keys": [], "providers": []}
            }),
            json!({"running": true, "providerName": "ChatGPT · user@example.com"}),
        )
        .ok()
        .unwrap_or_default();
        assert_eq!(agents.len(), 2);
        assert_eq!(agents[0]["id"], "codex");
        assert_eq!(agents[0]["direct"], true);
        let provs = hub_providers(
            json!({
                "pool": [
                    {"id": "ds", "name": "DeepSeek", "baseUrl": "https://api.deepseek.com", "keyCount": 2},
                    {"id": "off", "name": "Off", "baseUrl": "https://x", "keyCount": 1, "enabled": false}
                ],
                "health": {"keys": [], "providers": []},
                "activity": {"agents": []}
            }),
            json!({"$map": [["ds", 40]]}),
        )
        .ok()
        .unwrap_or_default();
        assert_eq!(provs.len(), 1);
        assert_eq!(provs[0]["calls"], 40);
        assert_eq!(
            success_rate(json!({"calls": 10, "errors": 1})).ok(),
            Some(json!(0.9))
        );
        assert_eq!(
            success_rate(json!({"calls": 0, "errors": 0})).ok(),
            Some(Value::Null)
        );
        assert_eq!(
            hub_totals(Value::Null).ok(),
            Some(json!({"requests": 0, "rerouted": 0, "coolingKeys": 0, "openBreakers": 0}))
        );
        assert!(success_rate(Value::Null).is_err());
    }
}
