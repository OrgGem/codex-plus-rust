//! The Rust port of `src/request-flow-model.ts`.

#![forbid(unsafe_code)]

use chrono::Timelike;
use manager_common_rs::domain::{es_trim, js_stringify_compact};
use manager_common_rs::gateway_shared::mask_account_name;
use manager_common_rs::relay_shared::{flow_agent_str, flow_status_str};
use manager_common_rs::{
    is_nil, is_truthy, is_undefined, js_number, js_str, local_datetime_from_ms, to_number,
};
use serde_json::{Value, json};
use std::collections::HashMap;

fn iterate_entries(entries: &Value) -> Result<Vec<Value>, String> {
    if let Some(arr) = entries.as_array() {
        return Ok(arr.clone());
    }
    if let Some(s) = entries.as_str() {
        return Ok(s.chars().map(|c| Value::String(c.to_string())).collect());
    }
    if let Some(set_arr) = entries
        .as_object()
        .filter(|o| o.len() == 1)
        .and_then(|o| o.get("$set"))
        .and_then(Value::as_array)
    {
        return Ok(set_arr.clone());
    }
    Err("TypeError: entries is not iterable".into())
}

fn js_sort_v8<T>(items: &mut [T], mut compare: impl FnMut(&T, &T) -> f64) {
    let n = items.len();
    if n < 2 {
        return;
    }
    if n >= 64 {
        items.sort_by(|a, b| {
            compare(a, b)
                .partial_cmp(&0.0)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        return;
    }
    let mut run = 2;
    let descending = compare(&items[1], &items[0]) < 0.0;
    while run < n {
        let order = compare(&items[run], &items[run - 1]);
        if descending != (order < 0.0) {
            break;
        }
        run += 1;
    }
    if descending {
        items[..run].reverse();
    }
    for start in run..n {
        let (mut left, mut right) = (0, start);
        while left < right {
            let mid = left + ((right - left) >> 1);
            if compare(&items[start], &items[mid]) < 0.0 {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        items[left..=start].rotate_right(1);
    }
}

fn slice_end(len: usize, end: f64) -> usize {
    if end.is_nan() {
        return 0;
    }
    if end.is_infinite() {
        return if end.is_sign_positive() { len } else { 0 };
    }
    let t = end.trunc();
    if t < 0.0 {
        len.saturating_sub((-t) as usize)
    } else if t >= len as f64 {
        len
    } else {
        t as usize
    }
}

fn js_max_zero(x: f64) -> f64 {
    if x.is_nan() {
        f64::NAN
    } else if x > 0.0 {
        x
    } else {
        0.0
    }
}

fn is_returning_entry(entry: &Value) -> bool {
    let phase = entry.get("phase").and_then(Value::as_str);
    if matches!(phase, Some("retrying" | "waiting" | "requesting")) {
        return false;
    }
    let first_token_nil = entry.get("firstTokenMs").is_none_or(is_nil);
    let est = entry
        .get("estimatedOutputTokens")
        .filter(|v| !is_nil(v))
        .map(to_number)
        .unwrap_or(0.0);
    !(phase != Some("generating") && first_token_nil && est <= 0.0)
}

/// Port of `flowStatus` (`src/request-flow-model.ts:33`).
pub fn flow_status(entry: Value) -> Result<String, String> {
    Ok(flow_status_str(&entry)?.to_string())
}

/// Port of `flowAgent` (`src/request-flow-model.ts:39`).
pub fn flow_agent(entry: Value) -> Result<String, String> {
    flow_agent_str(&entry)
}

/// Port of `agentLinkKey` (`src/request-flow-model.ts:44`).
pub fn agent_link_key(agent: String) -> Result<String, String> {
    Ok(format!("agent:{agent}>hub"))
}

/// Port of `providerLinkKey` (`src/request-flow-model.ts:48`).
pub fn provider_link_key(relay_id: String) -> Result<String, String> {
    Ok(format!("hub>provider:{relay_id}"))
}

/// Port of `pulseHops` (`src/request-flow-model.ts:55`).
pub fn pulse_hops(pulse: Value) -> Result<Vec<Value>, String> {
    if is_nil(&pulse) {
        return Err("TypeError: Cannot read properties of null/undefined (reading 'agent')".into());
    }
    let agent = pulse
        .get("agent")
        .map(js_str)
        .unwrap_or_else(|| "undefined".into());
    let mut out = vec![json!({ "key": agent_link_key(agent)?, "reverse": false })];
    if let Some(relay_id) = pulse.get("relayId").filter(|v| is_truthy(v)) {
        out.push(json!({ "key": provider_link_key(js_str(relay_id))?, "reverse": false }));
    }
    if pulse.get("kind").and_then(Value::as_str) == Some("out") {
        return Ok(out);
    }
    Ok(out
        .into_iter()
        .rev()
        .map(|hop| json!({ "key": hop["key"], "reverse": true }))
        .collect())
}

/// Port of `liveFlowRoutes` (`src/request-flow-model.ts:77`).
pub fn live_flow_routes(entries: Value) -> Result<Vec<Value>, String> {
    let items = iterate_entries(&entries)?;
    let mut routes: Vec<Value> = Vec::new();
    let mut index_by_key: HashMap<String, usize> = HashMap::new();
    for entry in &items {
        if flow_status_str(entry)? != "running" {
            continue;
        }
        let agent = flow_agent_str(entry)?;
        let relay_id = entry
            .get("relayId")
            .filter(|v| !is_nil(v))
            .cloned()
            .unwrap_or_else(|| Value::String(String::new()));
        let reverse = is_returning_entry(entry);
        let key = js_stringify_compact(&json!([agent, relay_id, reverse]));
        if let Some(&idx) = index_by_key.get(&key) {
            let next = routes[idx]["count"].as_u64().unwrap_or(0) + 1;
            routes[idx]["count"] = json!(next);
        } else {
            index_by_key.insert(key.clone(), routes.len());
            routes.push(json!({
                "key": key,
                "agent": agent,
                "relayId": relay_id,
                "reverse": reverse,
                "count": 1,
            }));
        }
    }
    Ok(routes)
}

/// Port of `flowRoutePosition` (`src/request-flow-model.ts:93`).
pub fn flow_route_position(lengths: Value, distance: Value) -> Result<Value, String> {
    if is_nil(&lengths) {
        return Err(
            "TypeError: Cannot read properties of null/undefined (reading 'length')".into(),
        );
    }
    let dist = to_number(&distance);
    if dist < 0.0 {
        return Ok(Value::Null);
    }
    let Some(arr) = lengths.as_array() else {
        return Ok(Value::Null);
    };
    let mut start = 0.0_f64;
    for (segment, len_val) in arr.iter().enumerate() {
        let length = to_number(len_val);
        if length > 0.0 && dist <= start + length {
            return Ok(json!({
                "segment": segment,
                "progress": js_number((dist - start) / length),
            }));
        }
        start += length;
    }
    Ok(Value::Null)
}

/// Port of `flowColumns` (`src/request-flow-model.ts:120`).
pub fn flow_columns(entries: Value, max_per_side: Value) -> Result<Value, String> {
    let items = iterate_entries(&entries)?;
    let limit = if is_undefined(&max_per_side) {
        3.0
    } else {
        to_number(&max_per_side)
    };
    for entry in &items {
        if is_nil(entry) {
            return Err(
                "TypeError: Cannot read properties of null/undefined (reading 'startedAtMs')"
                    .into(),
            );
        }
    }
    let mut ordered: Vec<&Value> = items.iter().collect();
    js_sort_v8(&mut ordered, |left, right| {
        let r = right.get("startedAtMs").map(to_number).unwrap_or(f64::NAN);
        let l = left.get("startedAtMs").map(to_number).unwrap_or(f64::NAN);
        r - l
    });

    let mut agents: Vec<Value> = Vec::new();
    let mut agent_idx: HashMap<String, usize> = HashMap::new();
    let mut providers: Vec<Value> = Vec::new();
    let mut provider_idx: HashMap<String, usize> = HashMap::new();

    for entry in ordered {
        let live = flow_status_str(entry)? == "running";
        let agent = flow_agent_str(entry)?;
        if let Some(&idx) = agent_idx.get(&agent) {
            let cur_live = agents[idx]["live"].as_bool().unwrap_or(false);
            let cur_cnt = agents[idx]["count"].as_u64().unwrap_or(0);
            agents[idx]["live"] = json!(cur_live || live);
            agents[idx]["count"] = json!(cur_cnt + 1);
        } else {
            agent_idx.insert(agent.clone(), agents.len());
            agents.push(json!({ "id": agent, "name": agent, "live": live, "count": 1 }));
        }
        let Some(relay_val) = entry.get("relayId").filter(|v| is_truthy(v)) else {
            continue;
        };
        let relay_id = js_str(relay_val);
        if let Some(&idx) = provider_idx.get(&relay_id) {
            let cur_live = providers[idx]["live"].as_bool().unwrap_or(false);
            let cur_cnt = providers[idx]["count"].as_u64().unwrap_or(0);
            providers[idx]["live"] = json!(cur_live || live);
            providers[idx]["count"] = json!(cur_cnt + 1);
        } else {
            let raw_name = entry
                .get("relayName")
                .filter(|v| is_truthy(v))
                .map(js_str)
                .unwrap_or_else(|| relay_id.clone());
            let masked = mask_account_name(raw_name)?;
            provider_idx.insert(relay_id.clone(), providers.len());
            providers.push(json!({ "id": relay_id, "name": masked, "live": live, "count": 1 }));
        }
    }

    let a_end = slice_end(agents.len(), limit);
    let p_end = slice_end(providers.len(), limit);
    agents.truncate(a_end);
    providers.truncate(p_end);
    Ok(json!({ "agents": agents, "providers": providers }))
}

/// Port of `flowElapsed` (`src/request-flow-model.ts:142`).
pub fn flow_elapsed(entry: Value, now: Value) -> Result<f64, String> {
    let now_num = to_number(&now);
    if flow_status_str(&entry)? == "running" {
        let started = entry
            .get("startedAtMs")
            .filter(|v| is_truthy(v))
            .map(to_number)
            .unwrap_or(now_num);
        Ok(js_max_zero(now_num - started))
    } else {
        let dur = entry.get("durationMs").map(to_number).unwrap_or(f64::NAN);
        Ok(js_max_zero(dur))
    }
}

/// Port of `clockTime` (`src/request-flow-model.ts:148`).
pub fn clock_time(ms: Value) -> Result<String, String> {
    let num = to_number(&ms);
    let Some(dt) = local_datetime_from_ms(num) else {
        return Ok("NaN:NaN:NaN".into());
    };
    Ok(format!(
        "{:02}:{:02}:{:02}",
        dt.hour(),
        dt.minute(),
        dt.second()
    ))
}

#[allow(dead_code)]
fn keep_es_trim_used(s: &str) -> &str {
    es_trim(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flow_status_and_routes_and_positions_match() {
        assert_eq!(
            flow_status(json!({"state": "running", "success": false})).ok(),
            Some("running".into())
        );
        assert_eq!(
            flow_status(json!({"statusCode": 429})).ok(),
            Some("error".into())
        );
        assert_eq!(
            flow_agent(json!({"agent": "  "})).ok(),
            Some("codex".into())
        );
        assert_eq!(
            pulse_hops(json!({"agent": "claude", "relayId": "ds", "kind": "back"}))
                .ok()
                .map(|v| v.len()),
            Some(2)
        );
        let routes = live_flow_routes(json!([
            {"id": "a", "agent": "claude", "startedAtMs": 1, "timestampMs": 2, "durationMs": 0, "model": "m", "state": "running", "phase": "waiting"},
            {"id": "b", "agent": "claude", "startedAtMs": 1, "timestampMs": 2, "durationMs": 0, "model": "m", "state": "running", "phase": "generating"},
            {"id": "c", "agent": "claude", "startedAtMs": 1, "timestampMs": 2, "durationMs": 0, "model": "m", "state": "running", "phase": "generating"}
        ]))
        .ok()
        .unwrap_or_default();
        assert_eq!(routes.len(), 2);
        assert_eq!(routes[0]["reverse"], false);
        assert_eq!(routes[1]["reverse"], true);
        assert_eq!(routes[1]["count"], 2);
        assert_eq!(
            flow_route_position(json!([30, 30, 60]), json!(45)).ok(),
            Some(json!({"segment": 1, "progress": 0.5}))
        );
        assert_eq!(
            flow_route_position(json!([30, 30, 60]), json!(-1)).ok(),
            Some(Value::Null)
        );
        assert_eq!(
            flow_elapsed(
                json!({"state": "running", "startedAtMs": 20000}),
                json!(10000)
            )
            .ok(),
            Some(0.0)
        );
        assert!(flow_status(Value::Null).is_err());
    }
}
