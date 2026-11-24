//! Shared Wave 0/1/3 domain helpers for Wave 3, Wave 4, and Wave 13 crates.

#![forbid(unsafe_code)]
#![allow(dead_code)]

use crate::domain::{is_valid_auto_compact_percent, normalize_auto_compact_percent};
use crate::js_rt::{
    as_number, is_js_ws, is_nil, is_plain_object, is_truthy, js_number, js_str, js_trim,
    math_round, spread_object, string_units, undefined, utf16_string,
};
use serde_json::{Map, Value, json};
use std::collections::HashSet;

pub fn flow_status_str(entry: &Value) -> Result<&'static str, String> {
    if is_nil(entry) {
        return Err("TypeError: Cannot read properties of null/undefined (reading 'state')".into());
    }
    if entry.get("state").and_then(Value::as_str) == Some("running") {
        return Ok("running");
    }
    let success_not_false = entry.get("success") != Some(&Value::Bool(false));
    let status_code = entry
        .get("statusCode")
        .filter(|v| !is_nil(v))
        .and_then(as_number)
        .unwrap_or(200.0);
    if success_not_false && status_code < 400.0 {
        Ok("ok")
    } else {
        Ok("error")
    }
}

pub fn flow_agent_str(entry: &Value) -> Result<String, String> {
    if is_nil(entry) {
        return Err("TypeError: Cannot read properties of null/undefined (reading 'agent')".into());
    }
    let trimmed = match entry.get("agent") {
        None => "",
        Some(v) if is_nil(v) => "",
        Some(Value::String(s)) => js_trim(s),
        Some(_) => return Err("TypeError: entry.agent?.trim is not a function".into()),
    };
    Ok(if trimmed.is_empty() {
        "codex".into()
    } else {
        trimmed.into()
    })
}

pub fn agent_visible_bool(focus: Option<&Value>, agent: &str) -> bool {
    match focus.and_then(Value::as_str) {
        Some("codex") => agent == "codex",
        Some("enhance") => matches!(agent, "codex" | "claude"),
        _ => true,
    }
}

pub fn session_provider_for_protocol_str(session_provider: &str, protocol: &str) -> String {
    if protocol != "responses" && session_provider == "openai" {
        "custom".into()
    } else {
        session_provider.into()
    }
}

// ── provider-api-keys helpers ───────────────────────────────────────────────

pub fn unique_key_id(base: &str, used: &mut HashSet<String>) -> String {
    if used.insert(base.to_string()) {
        return base.to_string();
    }
    let mut suffix = 2usize;
    loop {
        let candidate = format!("{base}-{suffix}");
        if used.insert(candidate.clone()) {
            return candidate;
        }
        suffix += 1;
    }
}

pub fn normalize_relay_api_keys_value(profile: &Value) -> Result<(Vec<Value>, String), String> {
    if is_nil(profile) {
        return Err(
            "TypeError: Cannot read properties of null/undefined (reading 'apiKeys')".into(),
        );
    }
    let mut used = HashSet::new();
    let mut api_keys = Vec::new();
    if let Some(arr) = profile.get("apiKeys").and_then(Value::as_array) {
        for (idx, entry) in arr.iter().enumerate() {
            let req_id_raw = entry
                .get("id")
                .filter(|v| is_truthy(v))
                .map(js_str)
                .unwrap_or_default();
            let req_id = js_trim(&req_id_raw);
            let base = if req_id.is_empty() {
                format!("key-{}", idx + 1)
            } else {
                req_id.to_string()
            };
            let id = unique_key_id(&base, &mut used);
            let raw_name = entry
                .get("name")
                .filter(|v| is_truthy(v))
                .map(js_str)
                .unwrap_or_default();
            let name = js_trim(&raw_name);
            let name = if name.is_empty() {
                format!("Key {}", idx + 1)
            } else {
                name.to_string()
            };
            let api_key = entry
                .get("apiKey")
                .filter(|v| is_truthy(v))
                .map(js_str)
                .unwrap_or_default();
            api_keys.push(json!({ "id": id, "name": name, "apiKey": api_key }));
        }
    }
    if api_keys.is_empty() {
        let fallback_key = profile
            .get("apiKey")
            .filter(|v| is_truthy(v))
            .map(js_str)
            .unwrap_or_default();
        api_keys.push(json!({ "id": "default", "name": "默认", "apiKey": fallback_key }));
    }
    let req_active_raw = profile
        .get("activeApiKeyId")
        .filter(|v| is_truthy(v))
        .map(js_str)
        .unwrap_or_default();
    let req_active = js_trim(&req_active_raw).to_string();
    let active_id = if api_keys
        .iter()
        .any(|e| e.get("id").and_then(Value::as_str) == Some(req_active.as_str()))
    {
        req_active
    } else {
        api_keys
            .first()
            .and_then(|e| e.get("id"))
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string()
    };
    Ok((api_keys, active_id))
}

pub fn relay_profile_with_normalized_api_keys(profile: &Value) -> Result<Value, String> {
    let (api_keys, active_id) = normalize_relay_api_keys_value(profile)?;
    let selected_key = api_keys
        .iter()
        .find(|e| e.get("id").and_then(Value::as_str) == Some(active_id.as_str()))
        .and_then(|e| e.get("apiKey"))
        .cloned()
        .unwrap_or_else(|| profile.get("apiKey").cloned().unwrap_or_else(undefined));
    let mut obj = spread_object(profile);
    obj.insert("apiKeys".into(), Value::Array(api_keys));
    obj.insert("activeApiKeyId".into(), Value::String(active_id));
    obj.insert("apiKey".into(), selected_key);
    Ok(Value::Object(obj))
}

pub fn relay_profile_with_updated_api_key(profile: &Value, api_key: &str) -> Result<Value, String> {
    if is_nil(profile) {
        return Err(
            "TypeError: Cannot read properties of null/undefined (reading 'noAuth')".into(),
        );
    }
    let no_auth = profile.get("noAuth").is_some_and(is_truthy);
    let mode = profile.get("relayMode").and_then(Value::as_str);
    let mix = profile.get("officialMixApiKey").is_some_and(is_truthy);
    if no_auth || mode == Some("aggregate") || (mode == Some("official") && !mix) {
        return Ok(profile.clone());
    }
    let (api_keys, active_id) = normalize_relay_api_keys_value(profile)?;
    let next_keys: Vec<Value> = api_keys
        .into_iter()
        .map(|mut e| {
            if e.get("id").and_then(Value::as_str) == Some(active_id.as_str())
                && let Some(m) = e.as_object_mut()
            {
                m.insert("apiKey".into(), Value::String(api_key.to_string()));
            }
            e
        })
        .collect();
    let mut obj = spread_object(profile);
    obj.insert("apiKeys".into(), Value::Array(next_keys));
    obj.insert("activeApiKeyId".into(), Value::String(active_id));
    obj.insert("apiKey".into(), Value::String(api_key.to_string()));
    Ok(Value::Object(obj))
}

// ── aggregate-members / aggregate-routes / model-routes helpers ─────────────

fn has_json_key_colon(haystack: &str, key: &str) -> bool {
    let pat = format!("\"{key}\"");
    let mut start = 0;
    while let Some(pos) = haystack[start..].find(&pat) {
        let after = &haystack[start + pos + pat.len()..];
        if after.trim_start_matches(is_js_ws).starts_with(':') {
            return true;
        }
        start += pos + pat.len();
    }
    false
}

pub fn auth_contents_have_chatgpt_tokens(contents: Option<&str>) -> bool {
    let trimmed = contents.map(js_trim).unwrap_or("");
    if trimmed.is_empty() {
        return false;
    }
    if let Ok(parsed) = serde_json::from_str::<Value>(trimmed) {
        return parsed
            .as_object()
            .and_then(|o| o.get("tokens"))
            .and_then(Value::as_object)
            .is_some_and(|t| {
                ["access_token", "refresh_token"].iter().any(|k| {
                    t.get(*k)
                        .and_then(Value::as_str)
                        .is_some_and(|s| !js_trim(s).is_empty())
                })
            });
    }
    has_json_key_colon(trimmed, "tokens")
        && (has_json_key_colon(trimmed, "access_token")
            || has_json_key_colon(trimmed, "refresh_token"))
}

pub fn is_aggregate_profile(profile: &Value) -> Result<bool, String> {
    if is_nil(profile) {
        return Err(
            "TypeError: Cannot read properties of null/undefined (reading 'relayMode')".into(),
        );
    }
    Ok(
        profile.get("relayMode").and_then(Value::as_str) == Some("aggregate")
            || profile.get("aggregate").is_some_and(is_truthy),
    )
}

pub fn is_api_member_profile(profile: &Value) -> Result<bool, String> {
    if is_nil(profile) {
        return Err(
            "TypeError: Cannot read properties of null/undefined (reading 'baseUrl')".into(),
        );
    }
    let trim_field = |k: &str| -> Result<&str, String> {
        match profile.get(k) {
            None => Ok(""),
            Some(v) if is_nil(v) => Ok(""),
            Some(Value::String(s)) => Ok(js_trim(s)),
            Some(_) => Err(format!("TypeError: ({k} ?? \"\").trim is not a function")),
        }
    };
    Ok(!trim_field("baseUrl")?.is_empty() && !trim_field("apiKey")?.is_empty())
}

pub fn is_official_login_member_profile(profile: &Value) -> Result<bool, String> {
    if is_nil(profile) {
        return Err(
            "TypeError: Cannot read properties of null/undefined (reading 'relayMode')".into(),
        );
    }
    if profile.get("relayMode").and_then(Value::as_str) != Some("official")
        || profile.get("officialMixApiKey").is_some_and(is_truthy)
    {
        return Ok(false);
    }
    if profile
        .get("protocol")
        .filter(|v| !is_nil(v))
        .is_some_and(|v| v.as_str() != Some("responses"))
    {
        return Ok(false);
    }
    let auth = match profile.get("authContents") {
        None => None,
        Some(v) if is_nil(v) => None,
        Some(Value::String(s)) => Some(s.as_str()),
        Some(_) => return Err("TypeError: (contents ?? \"\").trim is not a function".into()),
    };
    Ok(auth_contents_have_chatgpt_tokens(auth))
}

pub fn aggregate_member_candidates(
    profiles: &[Value],
    aggregate_id: &str,
) -> Result<Vec<Value>, String> {
    let mut out = Vec::new();
    for p in profiles {
        if is_nil(p) {
            return Err(
                "TypeError: Cannot read properties of null/undefined (reading 'id')".into(),
            );
        }
        if p.get("id").and_then(Value::as_str) == Some(aggregate_id) || is_aggregate_profile(p)? {
            continue;
        }
        if is_api_member_profile(p)? || is_official_login_member_profile(p)? {
            out.push(p.clone());
        }
    }
    Ok(out)
}

pub fn normalize_aggregate_default_relay_id(
    value: Option<&str>,
    member_ids: &HashSet<String>,
) -> String {
    let id = value.map(js_trim).unwrap_or("");
    if id.is_empty() || !member_ids.contains(id) {
        String::new()
    } else {
        id.to_string()
    }
}

pub fn normalize_aggregate_members(
    members: Option<&[Value]>,
    profile_ids: Option<&HashSet<String>>,
) -> Vec<Value> {
    let Some(list) = members else {
        return Vec::new();
    };
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    for m in list {
        let raw_id = m
            .get("profileId")
            .filter(|v| !is_nil(v))
            .map(js_str)
            .unwrap_or_default();
        let pid = js_trim(&raw_id);
        if pid.is_empty() || seen.contains(pid) {
            continue;
        }
        if let Some(ids) = profile_ids
            && !ids.contains(pid)
        {
            continue;
        }
        seen.insert(pid.to_string());
        let mut obj = spread_object(m);
        obj.insert("profileId".into(), Value::String(pid.to_string()));
        out.push(Value::Object(obj));
    }
    out
}

pub fn clamp_aggregate_route_priority(value: f64) -> f64 {
    if !value.is_finite() {
        return 0.0;
    }
    let r = math_round(value).clamp(0.0, 999.0);
    if r == 0.0 { 0.0 } else { r }
}

fn route_parts_vec(route: &Value) -> Result<(&str, Value, Value), String> {
    if is_nil(route) {
        return Err(
            "TypeError: Cannot read properties of null/undefined (reading 'pattern')".into(),
        );
    }
    let pattern = route
        .get("pattern")
        .and_then(Value::as_str)
        .map(js_trim)
        .ok_or("TypeError: route.pattern.trim is not a function")?;
    let profile_id = if route
        .as_object()
        .is_some_and(|o| o.contains_key("profileId"))
    {
        route.get("profileId").cloned().unwrap_or(Value::Null)
    } else {
        route.get("relayId").cloned().unwrap_or_else(undefined)
    };
    let priority = route.get("priority").cloned().unwrap_or_else(undefined);
    Ok((pattern, profile_id, priority))
}

pub fn normalize_aggregate_routes(
    routes: &[Value],
    drop_empty: bool,
    member_ids: Option<&HashSet<String>>,
    clamp: bool,
) -> Result<Vec<Value>, String> {
    let mut out = Vec::new();
    for route in routes {
        let (pattern, profile_id, prio_val) = route_parts_vec(route)?;
        let priority = if clamp {
            let n = as_number(&prio_val).unwrap_or(f64::NAN);
            js_number(clamp_aggregate_route_priority(n))
        } else {
            prio_val
        };
        if drop_empty && pattern.is_empty() {
            continue;
        }
        if let Some(m) = member_ids {
            let Some(pid) = profile_id.as_str() else {
                continue;
            };
            if !m.contains(pid) {
                continue;
            }
        }
        out.push(json!({ "pattern": pattern, "profileId": profile_id, "priority": priority }));
    }
    Ok(out)
}

pub fn validate_aggregate_routes(
    routes: &[Value],
    member_ids: &HashSet<String>,
) -> Result<Option<Vec<Value>>, String> {
    let mut issues = Vec::new();
    for route in routes {
        let (pattern, profile_id, prio_val) = route_parts_vec(route)?;
        if pattern.is_empty() {
            let pid = profile_id
                .as_str()
                .map(js_trim)
                .ok_or("TypeError: profileId.trim is not a function")?;
            if !pid.is_empty() {
                issues.push(json!({ "code": "emptyPattern", "pattern": "" }));
            }
            continue;
        }
        let valid_prio = as_number(&prio_val)
            .is_some_and(|n| n.is_finite() && n.fract() == 0.0 && (0.0..=999.0).contains(&n));
        if !valid_prio {
            issues.push(json!({ "code": "invalidPriority", "pattern": pattern }));
            continue;
        }
        let in_members = profile_id.as_str().is_some_and(|s| member_ids.contains(s));
        if !in_members {
            issues.push(json!({ "code": "notMember", "pattern": pattern }));
        }
    }
    Ok((!issues.is_empty()).then_some(issues))
}

pub const PROTOCOL_PROXY_BASE_URL: &str = "http://127.0.0.1:57321/v1";

fn mr_str_prop(obj: &Map<String, Value>, key: &str) -> Option<String> {
    obj.get(key)
        .and_then(string_units)
        .map(|u| String::from_utf16_lossy(&u))
}

fn mr_non_official_or_mixed(obj: &Map<String, Value>) -> bool {
    mr_str_prop(obj, "relayMode").as_deref() != Some("official")
        || obj.get("officialMixApiKey").is_some_and(is_truthy)
}

pub fn normalize_relay_model_routes(routes: &Value) -> Vec<Value> {
    let Some(items) = routes.as_array() else {
        return Vec::new();
    };
    items
        .iter()
        .map(|route| {
            let obj = route.as_object().filter(|_| is_plain_object(route));
            let pick = |k: &str| {
                obj.and_then(|o| o.get(k))
                    .and_then(string_units)
                    .map(|u| utf16_string(&u))
                    .unwrap_or_else(|| Value::String(String::new()))
            };
            json!({
                "model": pick("model"),
                "targetRelayId": pick("targetRelayId"),
                "targetModel": pick("targetModel"),
            })
        })
        .collect()
}

pub fn find_relay_model_route_issue(
    sources: &[Value],
    all_profiles: &[Value],
) -> Result<Option<Value>, String> {
    let issue_json = |kind: &str, model: &str, source_relay_id: Value| json!({ "kind": kind, "model": model, "sourceRelayId": source_relay_id });
    for source in sources {
        if is_nil(source) {
            return Err(
                "TypeError: Cannot read properties of null/undefined (reading 'relayMode')".into(),
            );
        }
        let src_obj = source
            .as_object()
            .filter(|_| is_plain_object(source))
            .ok_or("TypeError: expected object")?;
        if mr_str_prop(src_obj, "relayMode").as_deref() == Some("aggregate") {
            continue;
        }
        let mut seen_models = HashSet::new();
        let raw_routes = src_obj
            .get("modelRoutes")
            .cloned()
            .unwrap_or_else(undefined);
        for route in normalize_relay_model_routes(&raw_routes) {
            let r_obj = route.as_object();
            let model = r_obj
                .and_then(|o| mr_str_prop(o, "model"))
                .map(|s| js_trim(&s).to_string())
                .unwrap_or_default();
            let target_relay_id = r_obj
                .and_then(|o| mr_str_prop(o, "targetRelayId"))
                .map(|s| js_trim(&s).to_string())
                .unwrap_or_default();
            let source_id = src_obj.get("id").cloned().unwrap_or_else(undefined);
            if model.is_empty() || target_relay_id.is_empty() {
                return Ok(Some(issue_json("incomplete", &model, source_id)));
            }
            if !seen_models.insert(model.clone()) {
                return Ok(Some(issue_json("duplicate", &model, source_id)));
            }
            if mr_str_prop(src_obj, "id").as_deref() == Some(target_relay_id.as_str()) {
                return Ok(Some(issue_json("self", &model, source_id)));
            }
            let target = all_profiles.iter().find(|cand| {
                cand.as_object()
                    .and_then(|o| mr_str_prop(o, "id"))
                    .as_deref()
                    == Some(target_relay_id.as_str())
            });
            let Some(target_obj) = target.and_then(Value::as_object) else {
                return Ok(Some(issue_json("missingTarget", &model, source_id)));
            };
            let target_mode = mr_str_prop(target_obj, "relayMode");
            if target_mode.as_deref() == Some("aggregate") {
                return Ok(Some(issue_json("aggregateTarget", &model, source_id)));
            }
            if mr_str_prop(target_obj, "protocol").as_deref() != Some("responses") {
                return Ok(Some(issue_json("targetProtocol", &model, source_id)));
            }
            let official_without_mix = target_mode.as_deref() == Some("official")
                && !target_obj.get("officialMixApiKey").is_some_and(is_truthy);
            let has_creds = if official_without_mix {
                false
            } else {
                let base_url = mr_str_prop(target_obj, "baseUrl")
                    .map(|s| js_trim(&s).to_string())
                    .ok_or("TypeError: Cannot read properties of undefined (reading 'trim')")?;
                if base_url.is_empty() {
                    false
                } else {
                    let api_key = mr_str_prop(target_obj, "apiKey")
                        .map(|s| js_trim(&s).to_string())
                        .ok_or("TypeError: Cannot read properties of undefined (reading 'trim')")?;
                    !api_key.is_empty()
                        || (target_mode.as_deref() == Some("pureApi")
                            && target_obj.get("noAuth").is_some_and(is_truthy))
                }
            };
            if !has_creds {
                return Ok(Some(issue_json("targetCredentials", &model, source_id)));
            }
        }
    }
    Ok(None)
}

pub fn web_search_history_compat_enabled(profile: &Value) -> bool {
    let Some(obj) = profile.as_object().filter(|_| is_plain_object(profile)) else {
        return false;
    };
    obj.get("webSearchHistoryCompat") == Some(&Value::Bool(true))
        && mr_str_prop(obj, "protocol").as_deref() == Some("responses")
        && mr_str_prop(obj, "relayMode").as_deref() != Some("aggregate")
        && mr_non_official_or_mixed(obj)
}

pub fn local_compaction_prompt_at_end_enabled(profile: &Value) -> bool {
    let Some(obj) = profile.as_object().filter(|_| is_plain_object(profile)) else {
        return false;
    };
    let proto = mr_str_prop(obj, "protocol");
    obj.get("localCompactionPromptAtEnd") == Some(&Value::Bool(true))
        && matches!(proto.as_deref(), Some("responses" | "chatCompletions"))
        && mr_str_prop(obj, "relayMode").as_deref() != Some("aggregate")
        && mr_non_official_or_mixed(obj)
}

pub fn strip_image_generation_tool_enabled(profile: &Value) -> bool {
    let Some(obj) = profile.as_object().filter(|_| is_plain_object(profile)) else {
        return false;
    };
    obj.get("stripImageGenerationTool") == Some(&Value::Bool(true))
        && mr_str_prop(obj, "protocol").as_deref() == Some("responses")
        && mr_str_prop(obj, "relayMode").as_deref() != Some("aggregate")
        && mr_non_official_or_mixed(obj)
}

pub fn strip_responses_image_detail_enabled(profile: &Value) -> bool {
    let Some(obj) = profile.as_object().filter(|_| is_plain_object(profile)) else {
        return false;
    };
    obj.get("stripResponsesImageDetail") == Some(&Value::Bool(true))
        && mr_str_prop(obj, "protocol").as_deref() == Some("responses")
        && mr_str_prop(obj, "relayMode").as_deref() != Some("aggregate")
        && mr_non_official_or_mixed(obj)
}

pub fn pure_api_image_generation_enabled(profile: &Value) -> bool {
    let Some(obj) = profile.as_object().filter(|_| is_plain_object(profile)) else {
        return false;
    };
    obj.get("pureApiImageGeneration") == Some(&Value::Bool(true))
        && mr_str_prop(obj, "relayMode").as_deref() == Some("pureApi")
}

pub fn request_compatibility_enabled(profile: &Value) -> Result<bool, String> {
    if is_nil(profile) {
        return Err(
            "TypeError: Cannot read properties of null/undefined (reading 'relayMode')".into(),
        );
    }
    let Some(obj) = profile.as_object().filter(|_| is_plain_object(profile)) else {
        return Ok(false);
    };
    if !mr_non_official_or_mixed(obj) {
        return Ok(false);
    }
    if mr_str_prop(obj, "protocol").as_deref() == Some("responses")
        && obj.get("strictResponsesReasoning") == Some(&Value::Bool(true))
    {
        return Ok(true);
    }
    let Some(meta_val) = obj.get("modelMetadata").filter(|v| !is_nil(v)) else {
        return Ok(false);
    };
    let Some(units) = string_units(meta_val) else {
        return Err("TypeError: profile.modelMetadata?.includes is not a function".into());
    };
    let meta = String::from_utf16_lossy(&units);
    if !meta.contains("codex_plus_reasoning_parameters") {
        return Ok(false);
    }
    let Ok(Value::Object(map)) = serde_json::from_str::<Value>(&meta) else {
        return Ok(false);
    };
    Ok(map.values().any(|entry| {
        entry
            .as_object()
            .filter(|_| is_plain_object(entry))
            .and_then(|o| o.get("codex_plus_reasoning_parameters"))
            .and_then(Value::as_str)
            .is_some_and(|v| v == "omit" || v == "passthrough")
    }))
}

pub fn keep_unused_auto_compact_helpers() -> (bool, String) {
    let ok = is_valid_auto_compact_percent("90%");
    (ok, normalize_auto_compact_percent("90"))
}
