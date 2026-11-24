//! The Rust port of `src/onboarding-setup-model.ts`.

#![forbid(unsafe_code)]

use crate::js;
use manager_common_rs::gateway_shared::{
    GROUP_MODEL_PREFIX, active_auto_group, agent_pool_providers, glob_matches, is_pattern,
    model_routable, qualified_provider, serving_providers,
};
use manager_common_rs::relay_shared::agent_visible_bool;
use manager_common_rs::{
    as_number, es_trim, is_nil, is_plain_object, is_truthy, is_undefined, js_stringify_compact,
    string_units,
};
use serde_json::{Map, Value, json};
use std::collections::HashSet;

const SETUP_VERSION: f64 = 1.0;

static UNDEF: std::sync::LazyLock<Value> = std::sync::LazyLock::new(js::undefined);

fn nil_err(prop: &str) -> String {
    format!("TypeError: Cannot read properties of null/undefined (reading '{prop}')")
}

fn prop<'a>(v: &'a Value, key: &str) -> Result<&'a Value, String> {
    if is_nil(v) {
        return Err(nil_err(key));
    }
    Ok(v.as_object()
        .filter(|_| is_plain_object(v))
        .and_then(|m| m.get(key))
        .unwrap_or(&UNDEF))
}

fn get_opt<'a>(v: &'a Value, key: &str) -> Result<Option<&'a Value>, String> {
    if is_nil(v) {
        return Err(nil_err(key));
    }
    Ok(v.as_object()
        .filter(|_| is_plain_object(v))
        .and_then(|m| m.get(key))
        .filter(|x| !is_undefined(x)))
}

fn req_arr<'a>(v: &'a Value, method: &str) -> Result<&'a [Value], String> {
    if is_nil(v) {
        return Err(nil_err(method));
    }
    v.as_array()
        .map(Vec::as_slice)
        .ok_or_else(|| format!("TypeError: value.{method} is not a function"))
}

fn val_str(v: &Value, method: &str) -> Result<String, String> {
    if is_nil(v) {
        return Err(nil_err(method));
    }
    string_units(v)
        .map(|u| String::from_utf16_lossy(&u))
        .ok_or_else(|| format!("TypeError: value.{method} is not a function"))
}

fn req_str(v: &Value, key: &str, method: &str) -> Result<String, String> {
    val_str(prop(v, key)?, method)
}

fn val_eq_str(v: &Value, s: &str) -> bool {
    string_units(v).is_some_and(|u| u == s.encode_utf16().collect::<Vec<_>>())
}

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

fn chat_model(entry: &Value, model: &str) -> Result<bool, String> {
    let lower = es_trim(model).to_lowercase();
    if lower.is_empty() {
        return Ok(false);
    }
    let image_models = match get_opt(entry, "imageModels")? {
        None | Some(Value::Null) => &[][..],
        Some(v) => req_arr(v, "some")?,
    };
    for img in image_models {
        if val_str(img, "toLowerCase")?.to_lowercase() == lower {
            return Ok(false);
        }
    }
    Ok(true)
}

fn expand_candidates(
    payload: &Value,
    providers: &[Value],
    exclude: &Value,
    value: &str,
    visiting: &[String],
    seen: &mut HashSet<String>,
    out: &mut Vec<Value>,
) -> Result<(), String> {
    let target = es_trim(value);
    let target_lo = target.to_lowercase();
    let group: Option<Value> = if let Some(group_id_lo) = target_lo.strip_prefix(GROUP_MODEL_PREFIX)
    {
        let groups = req_arr(prop(payload, "groups")?, "find")?;
        let mut found = None;
        for item in groups {
            if req_str(item, "id", "toLowerCase")?.to_lowercase() == group_id_lo {
                found = Some(item.clone());
                break;
            }
        }
        found
    } else {
        let auto = active_auto_group(payload.clone(), target.to_string())?;
        if is_nil(&auto) { None } else { Some(auto) }
    };

    if let Some(group) = group {
        if is_truthy(prop(&group, "disabled")?) {
            return Ok(());
        }
        let gid = val_str(prop(&group, "id")?, "includes")?;
        if visiting.len() >= 4 || visiting.iter().any(|v| v == &gid) {
            return Ok(());
        }
        let members_arr = req_arr(prop(&group, "members")?, "find")?;
        let mode = prop(&group, "mode")?.as_str();
        let selected: Vec<Value> = if mode == Some("manual") {
            let pick_lo = match get_opt(&group, "pick")? {
                None | Some(Value::Null) => None,
                Some(v) => Some(es_trim(&val_str(v, "trim")?).to_lowercase()),
            };
            let mut matched = None;
            for member in members_arr {
                let m_lo = req_str(member, "model", "toLowerCase")?.to_lowercase();
                if matched.is_none() && pick_lo.as_deref() == Some(m_lo.as_str()) {
                    matched = Some(member.clone());
                }
            }
            matched
                .or_else(|| members_arr.first().cloned())
                .filter(is_truthy)
                .into_iter()
                .collect()
        } else {
            members_arr.to_vec()
        };
        let mut next_visiting = visiting.to_vec();
        next_visiting.push(gid);
        for member in &selected {
            let m = val_str(prop(member, "model")?, "trim")?;
            expand_candidates(payload, providers, exclude, &m, &next_visiting, seen, out)?;
        }
        return Ok(());
    }

    if target_lo.starts_with(GROUP_MODEL_PREFIX) {
        return Ok(());
    }

    let mut add_cand = |entry: &Value, upstream: &str| -> Result<(), String> {
        if !chat_model(entry, upstream)? {
            return Ok(());
        }
        let entry_id = prop(entry, "id")?;
        let entry_name = prop(entry, "name")?;
        let id = js_stringify_compact(&json!([entry_id, upstream.to_lowercase()]));
        if seen.insert(id.clone()) {
            out.push(json!({
                "id": id,
                "providerId": entry_id,
                "providerName": entry_name,
                "model": upstream,
            }));
        }
        Ok(())
    };

    if is_pattern(target.to_string())? {
        for entry in providers {
            let entry_id = val_str(prop(entry, "id")?, "toString")?;
            for up_val in req_arr(prop(entry, "models")?, "Symbol.iterator")? {
                let upstream = val_str(up_val, "toString")?;
                let qualified = format!("{entry_id}/{upstream}");
                if glob_matches(target.to_string(), upstream.clone())?
                    || (glob_matches(target.to_string(), qualified.clone())?
                        && serving_providers(payload.clone(), qualified, exclude.clone())?
                            .is_empty())
                {
                    add_cand(entry, &upstream)?;
                }
            }
        }
        return Ok(());
    }

    let serving = serving_providers(payload.clone(), target.to_string(), exclude.clone())?;
    if !serving.is_empty() {
        for entry in &serving {
            let models = req_arr(prop(entry, "models")?, "find")?;
            let mut matched = None;
            for known in models {
                let s = val_str(known, "toLowerCase")?;
                if s.to_lowercase() == target_lo {
                    matched = Some(s);
                    break;
                }
            }
            add_cand(entry, matched.as_deref().unwrap_or(target))?;
        }
        return Ok(());
    }

    let qualified = qualified_provider(payload.clone(), target.to_string(), exclude.clone())?;
    if is_truthy(&qualified) {
        let q_entry = prop(&qualified, "entry")?;
        let q_model = val_str(prop(&qualified, "model")?, "trim")?;
        add_cand(q_entry, &q_model)?;
    }
    Ok(())
}

fn probe_candidates(
    payload: &Value,
    providers: &[Value],
    model: &str,
    exclude: &Value,
) -> Result<Vec<Value>, String> {
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    expand_candidates(payload, providers, exclude, model, &[], &mut seen, &mut out)?;
    Ok(out)
}

fn group_configuration(group: &Value) -> Result<Map<String, Value>, String> {
    let members_arr = req_arr(prop(group, "members")?, "map")?;
    let mut members = Vec::with_capacity(members_arr.len());
    for m in members_arr {
        if is_nil(m) {
            return Err(nil_err("model"));
        }
        members.push(json!({
            "model": prop(m, "model")?,
            "weight": prop(m, "weight")?,
        }));
    }
    let sticky_v = prop(group, "sticky")?;
    let sticky = if is_nil(sticky_v) {
        json!("cache")
    } else {
        sticky_v.clone()
    };
    let pick_v = prop(group, "pick")?;
    let pick = if is_nil(pick_v) {
        json!("")
    } else {
        pick_v.clone()
    };
    let mut map = Map::new();
    map.insert("id".into(), prop(group, "id")?.clone());
    map.insert(
        "disabled".into(),
        Value::Bool(prop(group, "disabled")? == &Value::Bool(true)),
    );
    map.insert("mode".into(), prop(group, "mode")?.clone());
    map.insert("sticky".into(), sticky);
    map.insert("pick".into(), pick);
    map.insert("members".into(), Value::Array(members));
    Ok(map)
}

fn configuration_fingerprint(value: &Value) -> String {
    let text = js_stringify_compact(value);
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for unit in text.encode_utf16() {
        hash = (hash ^ u64::from(unit)).wrapping_mul(0x0100_0000_01b3);
    }
    format!("{hash:x}")
}

/// Port of `hasCodexEnhancements` (`src/onboarding-setup-model.ts:44`).
pub fn has_codex_enhancements(config: Value) -> Result<bool, String> {
    if !is_truthy(&config) {
        return Ok(false);
    }
    let settings = prop(&config, "settings")?;
    if is_nil(settings) {
        return Err(nil_err("enhancementsEnabled"));
    }
    Ok(is_truthy(prop(settings, "enhancementsEnabled")?)
        || is_truthy(prop(settings, "codexAppStandaloneQuotaUnlock")?))
}

/// Port of `serializeSetupState` (`src/onboarding-setup-model.ts:52`).
pub fn serialize_setup_state(state: String) -> Result<String, String> {
    serde_json::to_string(&json!({ "version": 1, "state": state })).map_err(|e| e.to_string())
}

/// Port of `shouldShowSetup` (`src/onboarding-setup-model.ts:57`).
pub fn should_show_setup(
    stored_setup: Option<String>,
    stored_tour: Option<String>,
) -> Result<bool, String> {
    if stored_tour
        .as_deref()
        .is_some_and(|s| !es_trim(s).is_empty())
    {
        return Ok(false);
    }
    let raw = stored_setup.as_deref().unwrap_or("null");
    if let Ok(Value::Object(map)) = serde_json::from_str::<Value>(raw)
        && let Some(Value::Number(ver)) = map.get("version")
        && ver.as_f64().is_some_and(|v| v >= SETUP_VERSION)
        && matches!(
            map.get("state").and_then(Value::as_str),
            Some("completed" | "dismissed")
        )
    {
        return Ok(false);
    }
    Ok(true)
}

/// Port of `setupGatewayState` (`src/onboarding-setup-model.ts:139`).
pub fn setup_gateway_state(
    focus: Value,
    payload: Value,
    selected_agent_id: String,
) -> Result<Value, String> {
    let agent_id = if focus.as_str() == Some("codex") {
        "codex"
    } else {
        selected_agent_id.as_str()
    };

    let agent = if is_nil(&payload) {
        Value::Null
    } else {
        let agents = req_arr(prop(&payload, "agents")?, "find")?;
        let mut found = None;
        for item in agents {
            if is_nil(item) {
                return Err(nil_err("agent"));
            }
            if val_eq_str(prop(item, "agent")?, agent_id)
                && agent_visible_bool(Some(&focus), agent_id)
            {
                found = Some(item.clone());
                break;
            }
        }
        found.unwrap_or(Value::Null)
    };

    let binding = if is_nil(&payload) {
        None
    } else {
        let gw = prop(&payload, "gateway")?;
        let bindings = prop(gw, "bindings")?;
        if is_nil(bindings) {
            return Err(nil_err(agent_id));
        }
        get_opt(bindings, agent_id)?.cloned()
    };

    let exclude = match binding.as_ref().filter(|b| !is_nil(b)) {
        Some(b) => match get_opt(b, "poolExclude")? {
            Some(v) if !is_nil(v) => v.clone(),
            _ => json!([]),
        },
        None => json!([]),
    };

    let providers = agent_pool_providers(payload.clone(), exclude.clone())?;
    let model = if is_nil(&agent) {
        String::new()
    } else {
        es_trim(&req_str(&agent, "defaultModel", "trim")?).to_string()
    };

    let candidates = if is_truthy(&payload) && is_truthy(&agent) && !model.is_empty() {
        probe_candidates(&payload, &providers, &model, &exclude)?
    } else {
        Vec::new()
    };

    let mut providers_configured = false;
    for entry in &providers {
        let models = req_arr(prop(entry, "models")?, "some")?;
        for name in models {
            if chat_model(entry, &val_str(name, "trim")?)? {
                providers_configured = true;
                break;
            }
        }
        if providers_configured {
            break;
        }
    }
    if !providers_configured && !candidates.is_empty() {
        providers_configured = true;
    }

    let binding_disabled = binding
        .as_ref()
        .filter(|b| !is_nil(b))
        .and_then(|b| prop(b, "enabled").ok())
        .is_some_and(|v| v == &Value::Bool(false));

    let issue = if !is_truthy(&payload) {
        json!("loading")
    } else if !is_truthy(&agent) {
        json!("agent_missing")
    } else if !providers_configured {
        json!("no_providers")
    } else if !is_truthy(prop(&agent, "enabled")?) || binding_disabled {
        json!("not_connected")
    } else if is_truthy(prop(&agent, "drift")?) {
        json!("drift")
    } else if !is_truthy(prop(&payload, "helperRunning")?) {
        json!("helper_stopped")
    } else if model.is_empty() {
        json!("no_model")
    } else if !model_routable(payload.clone(), model.clone(), exclude.clone())?
        || candidates.is_empty()
    {
        json!("unroutable")
    } else {
        Value::Null
    };

    let agent_fp = if is_truthy(&agent) {
        json!({
            "enabled": prop(&agent, "enabled")?,
            "model": model,
            "baseUrl": prop(&agent, "baseUrl")?,
            "drift": is_truthy(prop(&agent, "drift")?),
        })
    } else {
        Value::Null
    };

    let binding_fp = if let Some(b) = binding.as_ref().filter(|b| is_truthy(b)) {
        json!({
            "enabled": prop(b, "enabled")?,
            "model": prop(b, "model")?,
            "extraModels": prop(b, "extraModels")?,
            "claudeRoles": prop(b, "claudeRoles")?,
            "unknownModel": prop(b, "unknownModel")?,
            "autoReviewModel": prop(b, "autoReviewModel")?,
            "titleModel": prop(b, "titleModel")?,
            "contextWindow": prop(b, "contextWindow")?,
            "claudeEnableEffort": prop(b, "claudeEnableEffort")?,
            "poolExclude": exclude,
        })
    } else {
        Value::Null
    };

    let (port_fp, helper_fp, key_policy_fp, auto_models_fp, groups_fp, auto_groups_fp) =
        if is_nil(&payload) {
            (
                js::undefined(),
                js::undefined(),
                js::undefined(),
                Value::Bool(true),
                js::undefined(),
                js::undefined(),
            )
        } else {
            let gw = prop(&payload, "gateway")?;
            let kp = prop(gw, "keyPolicy")?;
            let key_policy = prop(kp, "mode")?.clone();
            let auto_models = prop(gw, "autoModels")? != &Value::Bool(false);
            let groups_arr = req_arr(prop(&payload, "groups")?, "map")?;
            let groups = Value::Array(
                groups_arr
                    .iter()
                    .map(|g| Ok(Value::Object(group_configuration(g)?)))
                    .collect::<Result<Vec<_>, String>>()?,
            );
            let auto_groups = match get_opt(&payload, "autoGroups")? {
                None | Some(Value::Null) => js::undefined(),
                Some(v) => Value::Array(
                    req_arr(v, "map")?
                        .iter()
                        .map(|g| {
                            let mut map = group_configuration(g)?;
                            map.insert("hidden".into(), prop(g, "hidden")?.clone());
                            Ok(Value::Object(map))
                        })
                        .collect::<Result<Vec<_>, String>>()?,
                ),
            };
            (
                prop(&payload, "port")?.clone(),
                prop(&payload, "helperRunning")?.clone(),
                key_policy,
                Value::Bool(auto_models),
                groups,
                auto_groups,
            )
        };

    let mut providers_fp = Vec::with_capacity(providers.len());
    for entry in &providers {
        let keys_fp = match get_opt(entry, "keys")? {
            None | Some(Value::Null) => js::undefined(),
            Some(v) => Value::Array(
                req_arr(v, "map")?
                    .iter()
                    .map(|k| {
                        if is_nil(k) {
                            return Err(nil_err("id"));
                        }
                        Ok(json!({
                            "id": prop(k, "id")?,
                            "masked": prop(k, "masked")?,
                        }))
                    })
                    .collect::<Result<Vec<_>, String>>()?,
            ),
        };
        providers_fp.push(json!({
            "id": prop(entry, "id")?,
            "baseUrl": prop(entry, "baseUrl")?,
            "protocol": prop(entry, "protocol")?,
            "official": prop(entry, "official")?,
            "models": prop(entry, "models")?,
            "imageModels": prop(entry, "imageModels")?,
            "account": prop(entry, "account")?,
            "keyCount": prop(entry, "keyCount")?,
            "keys": keys_fp,
            "proxy": prop(entry, "proxy")?,
            "keyRpm": prop(entry, "keyRpm")?,
            "keyConcurrency": prop(entry, "keyConcurrency")?,
            "stripImageGenerationTool": prop(entry, "stripImageGenerationTool")?,
            "modelMetadata": prop(entry, "modelMetadata")?,
            "modelWindows": prop(entry, "modelWindows")?,
            "contextWindow": prop(entry, "contextWindow")?,
        }));
    }

    let fingerprint = configuration_fingerprint(&json!({
        "focus": focus,
        "agentId": agent_id,
        "port": port_fp,
        "helperRunning": helper_fp,
        "agent": agent_fp,
        "binding": binding_fp,
        "keyPolicy": key_policy_fp,
        "autoModels": auto_models_fp,
        "providers": providers_fp,
        "groups": groups_fp,
        "autoGroups": auto_groups_fp,
    }));

    let multiple_targets = candidates.len() > 1;
    let binding_ready = issue.is_null();
    Ok(json!({
        "agent": agent,
        "model": model,
        "providersConfigured": providers_configured,
        "bindingReady": binding_ready,
        "issue": issue,
        "candidates": candidates,
        "fingerprint": fingerprint,
        "multipleTargets": multiple_targets,
    }))
}

/// Port of `probeMatches` (`src/onboarding-setup-model.ts:202`).
pub fn probe_matches(state: Value, probe: Value) -> Result<Value, String> {
    if is_nil(&state) {
        return Err(nil_err("bindingReady"));
    }
    let ready = prop(&state, "bindingReady")?;
    if !is_truthy(ready) {
        return Ok(ready.clone());
    }
    if is_nil(&probe) || prop(&probe, "ok")? != &Value::Bool(true) {
        return Ok(Value::Bool(false));
    }
    if !js_strict_eq(prop(&probe, "fingerprint")?, prop(&state, "fingerprint")?) {
        return Ok(Value::Bool(false));
    }
    let probe_cid = prop(&probe, "candidateId")?;
    for cand in req_arr(prop(&state, "candidates")?, "some")? {
        if is_nil(cand) {
            return Err(nil_err("id"));
        }
        if js_strict_eq(prop(cand, "id")?, probe_cid) {
            return Ok(Value::Bool(true));
        }
    }
    Ok(Value::Bool(false))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_payload() -> Value {
        json!({
            "gateway": {
                "version": 1,
                "bindings": {
                    "codex": {
                        "enabled": true,
                        "model": "shared",
                        "extraModels": [],
                        "claudeRoles": { "opus": "", "sonnet": "", "haiku": "", "subagent": "" },
                        "unknownModel": "mapToDefault",
                        "contextWindow": null,
                        "claudeEnableEffort": false,
                        "token": "secret-token"
                    },
                    "claude": {
                        "enabled": true,
                        "model": "backup",
                        "extraModels": [],
                        "claudeRoles": { "opus": "", "sonnet": "", "haiku": "", "subagent": "" },
                        "unknownModel": "mapToDefault",
                        "contextWindow": null,
                        "claudeEnableEffort": false,
                        "token": ""
                    }
                },
                "keyPolicy": { "mode": "sticky" },
                "restoreOnExit": false
            },
            "port": 57321,
            "helperRunning": true,
            "pool": [
                { "id": "first", "name": "First", "baseUrl": "https://first.test/v1", "protocol": "responses", "official": false, "models": ["shared", "image-model"], "imageModels": ["image-model"], "keyCount": 0, "account": "subscription" },
                { "id": "second", "name": "Second", "baseUrl": "https://second.test/v1", "protocol": "chatCompletions", "official": false, "models": ["shared", "backup"], "keyCount": 1, "keys": [{ "id": "key-1", "masked": "sk-...abcd" }] }
            ],
            "groups": [],
            "agents": [
                { "agent": "codex", "name": "Codex", "enabled": true, "baseUrl": "http://127.0.0.1:57321", "defaultModel": "shared", "models": ["shared"] },
                { "agent": "claude", "name": "Claude Code", "enabled": true, "baseUrl": "http://127.0.0.1:57321", "defaultModel": "backup", "models": ["backup"] }
            ],
            "health": { "keys": [], "providers": [] },
            "codexProviders": [],
            "claudeProviders": [],
            "grokProfiles": []
        })
    }

    #[test]
    fn setup_state_and_gateway_diagnostics() {
        assert!(!has_codex_enhancements(Value::Null).unwrap());
        assert!(
            !has_codex_enhancements(
                json!({ "settings": { "enhancementsEnabled": false, "codexAppStandaloneQuotaUnlock": false } })
            )
            .unwrap()
        );
        assert!(
            has_codex_enhancements(
                json!({ "settings": { "enhancementsEnabled": false, "codexAppStandaloneQuotaUnlock": true } })
            )
            .unwrap()
        );

        let saved = serialize_setup_state("completed".into()).unwrap();
        assert!(!should_show_setup(Some(saved), None).unwrap());
        assert!(should_show_setup(Some("broken".into()), None).unwrap());
        assert!(!should_show_setup(Some("broken".into()), Some("1".into())).unwrap());

        let loading = setup_gateway_state(json!("all"), Value::Null, "claude".into()).unwrap();
        assert_eq!(loading["issue"], json!("loading"));

        let mut data = sample_payload();
        let ready = setup_gateway_state(json!("codex"), data.clone(), "claude".into()).unwrap();
        assert_eq!(ready["bindingReady"], json!(true));
        assert_eq!(ready["multipleTargets"], json!(true));
        assert!(!ready["fingerprint"].as_str().unwrap_or("").is_empty());

        let probe = json!({
            "fingerprint": ready["fingerprint"],
            "candidateId": ready["candidates"][0]["id"],
            "ok": true,
        });
        assert_eq!(
            probe_matches(ready.clone(), probe.clone()).unwrap(),
            json!(true)
        );
        assert_eq!(probe_matches(ready, Value::Null).unwrap(), json!(false));

        data["helperRunning"] = json!(false);
        let stopped = setup_gateway_state(json!("codex"), data, "codex".into()).unwrap();
        assert_eq!(stopped["issue"], json!("helper_stopped"));
        assert_eq!(probe_matches(stopped, probe).unwrap(), json!(false));
    }
}
