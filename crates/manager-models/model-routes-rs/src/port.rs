//! The Rust port of `src/model-routes.ts`.

#![forbid(unsafe_code)]

use manager_common_rs::{is_nil, is_plain_object, is_truthy, js_trim, string_units};
use serde_json::{Value, json};
use std::cell::RefCell;
use std::collections::HashSet;

pub const PROTOCOL_PROXY_BASE_URL: &str = "http://127.0.0.1:57321/v1";
pub const NO_AUTH_PROXY_BEARER_TOKEN: &str = "codex-plus-no-auth";

thread_local! {
    static INDEPENDENT_PROXY_PROFILES: RefCell<HashSet<String>> = RefCell::new(HashSet::new());
}

fn require_obj<'a>(
    value: &'a Value,
    prop: &str,
) -> Result<&'a serde_json::Map<String, Value>, String> {
    if is_nil(value) {
        return Err(format!(
            "TypeError: Cannot read properties of null/undefined (reading '{prop}')"
        ));
    }
    Ok(value
        .as_object()
        .filter(|_| is_plain_object(value))
        .unwrap_or_else(|| {
            static EMPTY: std::sync::LazyLock<serde_json::Map<String, Value>> =
                std::sync::LazyLock::new(serde_json::Map::new);
            &EMPTY
        }))
}

fn str_prop(obj: &serde_json::Map<String, Value>, key: &str) -> Option<String> {
    obj.get(key)
        .and_then(string_units)
        .map(|u| String::from_utf16_lossy(&u))
}

fn trimmed_required_str(obj: &serde_json::Map<String, Value>, key: &str) -> Result<String, String> {
    let raw = str_prop(obj, key).ok_or_else(|| {
        format!("TypeError: Cannot read properties of undefined (reading 'trim') for {key}")
    })?;
    Ok(js_trim(&raw).to_string())
}

fn js_strict_eq(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Null, Value::Null) => true,
        (Value::Bool(x), Value::Bool(y)) => x == y,
        (Value::String(x), Value::String(y)) => x == y,
        (Value::Number(x), Value::Number(y)) => {
            x.as_f64().zip(y.as_f64()).is_some_and(|(l, r)| l == r)
        }
        (x, y) if crate::js::is_undefined(x) && crate::js::is_undefined(y) => true,
        _ => false,
    }
}

/// Port of `setIndependentProxyConfigured` (`src/model-routes.ts:39`).
pub fn set_independent_proxy_configured(
    profile_id: String,
    configured: bool,
) -> Result<(), String> {
    INDEPENDENT_PROXY_PROFILES.with(|set| {
        let mut set = set.borrow_mut();
        if configured {
            set.insert(profile_id);
        } else {
            set.remove(&profile_id);
        }
    });
    Ok(())
}

/// Port of `independentProxyEnabled` (`src/model-routes.ts:44`).
pub fn independent_proxy_enabled(profile: Value) -> Result<bool, String> {
    let obj = require_obj(&profile, "id")?;
    let Some(id_val) = obj.get("id").filter(|v| is_truthy(v)) else {
        return Ok(false);
    };
    let Some(id) = string_units(id_val).map(|u| String::from_utf16_lossy(&u)) else {
        return Ok(false);
    };
    let configured = INDEPENDENT_PROXY_PROFILES.with(|set| set.borrow().contains(&id));
    if !configured {
        return Ok(false);
    }
    Ok(str_prop(obj, "relayMode").as_deref() != Some("official")
        || obj.get("officialMixApiKey").is_some_and(is_truthy))
}

/// Port of `normalizeRelayModelRoutes` (`src/model-routes.ts:57`).
pub fn normalize_relay_model_routes(routes: Value) -> Result<Vec<Value>, String> {
    let Some(items) = routes.as_array() else {
        return Ok(Vec::new());
    };
    Ok(items
        .iter()
        .map(|route| {
            let obj = route.as_object().filter(|_| is_plain_object(route));
            let pick = |k: &str| {
                obj.and_then(|o| o.get(k))
                    .and_then(string_units)
                    .map(|u| crate::js::utf16_string(&u))
                    .unwrap_or_else(|| Value::String(String::new()))
            };
            json!({
                "model": pick("model"),
                "targetRelayId": pick("targetRelayId"),
                "targetModel": pick("targetModel"),
            })
        })
        .collect())
}

fn issue_json(kind: &str, model: &str, source_relay_id: Value) -> Value {
    json!({
        "kind": kind,
        "model": model,
        "sourceRelayId": source_relay_id,
    })
}

/// Port of `findRelayModelRouteIssue` (`src/model-routes.ts:66`).
pub fn find_relay_model_route_issue(
    sources: Value,
    all_profiles: Value,
) -> Result<Option<Value>, String> {
    let source_items: Vec<Value> = match sources {
        Value::Array(arr) => arr,
        _ if string_units(&sources).is_some() => string_units(&sources)
            .unwrap_or_default()
            .into_iter()
            .map(|u| crate::js::utf16_string(&[u]))
            .collect(),
        _ => return Err("TypeError: sources is not iterable".into()),
    };
    for source in source_items {
        let src_obj = require_obj(&source, "relayMode")?;
        if str_prop(src_obj, "relayMode").as_deref() == Some("aggregate") {
            continue;
        }
        let mut seen_models = HashSet::new();
        let raw_routes = src_obj
            .get("modelRoutes")
            .cloned()
            .unwrap_or_else(crate::js::undefined);
        for route in normalize_relay_model_routes(raw_routes)? {
            let r_obj = route.as_object();
            let model = r_obj
                .and_then(|o| str_prop(o, "model"))
                .map(|s| js_trim(&s).to_string())
                .unwrap_or_default();
            let target_relay_id = r_obj
                .and_then(|o| str_prop(o, "targetRelayId"))
                .map(|s| js_trim(&s).to_string())
                .unwrap_or_default();
            let source_id = src_obj
                .get("id")
                .cloned()
                .unwrap_or_else(crate::js::undefined);

            if model.is_empty() || target_relay_id.is_empty() {
                return Ok(Some(issue_json("incomplete", &model, source_id)));
            }
            if !seen_models.insert(model.clone()) {
                return Ok(Some(issue_json("duplicate", &model, source_id)));
            }
            if str_prop(src_obj, "id").as_deref() == Some(target_relay_id.as_str()) {
                return Ok(Some(issue_json("self", &model, source_id)));
            }

            let profile_items = all_profiles
                .as_array()
                .ok_or("TypeError: allProfiles.find is not a function")?;
            let mut target_obj = None;
            for candidate in profile_items {
                let cand = require_obj(candidate, "id")?;
                if str_prop(cand, "id").as_deref() == Some(target_relay_id.as_str()) {
                    target_obj = Some(cand);
                    break;
                }
            }
            let Some(target) = target_obj else {
                return Ok(Some(issue_json("missingTarget", &model, source_id)));
            };
            let target_mode = str_prop(target, "relayMode");
            if target_mode.as_deref() == Some("aggregate") {
                return Ok(Some(issue_json("aggregateTarget", &model, source_id)));
            }
            if str_prop(target, "protocol").as_deref() != Some("responses") {
                return Ok(Some(issue_json("targetProtocol", &model, source_id)));
            }

            let official_without_mix = target_mode.as_deref() == Some("official")
                && !target.get("officialMixApiKey").is_some_and(is_truthy);
            let has_creds = if official_without_mix {
                false
            } else {
                let base_url = trimmed_required_str(target, "baseUrl")?;
                if base_url.is_empty() {
                    false
                } else {
                    let api_key = trimmed_required_str(target, "apiKey")?;
                    !api_key.is_empty()
                        || (target_mode.as_deref() == Some("pureApi")
                            && target.get("noAuth").is_some_and(is_truthy))
                }
            };
            if !has_creds {
                return Ok(Some(issue_json("targetCredentials", &model, source_id)));
            }
        }
    }
    Ok(None)
}

fn non_official_or_mixed(obj: &serde_json::Map<String, Value>) -> bool {
    str_prop(obj, "relayMode").as_deref() != Some("official")
        || obj.get("officialMixApiKey").is_some_and(is_truthy)
}

/// Port of `webSearchHistoryCompatEnabled` (`src/model-routes.ts:96`).
pub fn web_search_history_compat_enabled(profile: Value) -> Result<bool, String> {
    let obj = require_obj(&profile, "webSearchHistoryCompat")?;
    Ok(
        obj.get("webSearchHistoryCompat") == Some(&Value::Bool(true))
            && str_prop(obj, "protocol").as_deref() == Some("responses")
            && str_prop(obj, "relayMode").as_deref() != Some("aggregate")
            && non_official_or_mixed(obj),
    )
}

/// Port of `localCompactionPromptAtEndEnabled` (`src/model-routes.ts:105`).
pub fn local_compaction_prompt_at_end_enabled(profile: Value) -> Result<bool, String> {
    let obj = require_obj(&profile, "localCompactionPromptAtEnd")?;
    let proto = str_prop(obj, "protocol");
    Ok(
        obj.get("localCompactionPromptAtEnd") == Some(&Value::Bool(true))
            && matches!(proto.as_deref(), Some("responses" | "chatCompletions"))
            && str_prop(obj, "relayMode").as_deref() != Some("aggregate")
            && non_official_or_mixed(obj),
    )
}

/// Port of `stripImageGenerationToolEnabled` (`src/model-routes.ts:115`).
pub fn strip_image_generation_tool_enabled(profile: Value) -> Result<bool, String> {
    let obj = require_obj(&profile, "stripImageGenerationTool")?;
    Ok(
        obj.get("stripImageGenerationTool") == Some(&Value::Bool(true))
            && str_prop(obj, "protocol").as_deref() == Some("responses")
            && str_prop(obj, "relayMode").as_deref() != Some("aggregate")
            && non_official_or_mixed(obj),
    )
}

/// Port of `stripResponsesImageDetailEnabled` (`src/model-routes.ts:125`).
pub fn strip_responses_image_detail_enabled(profile: Value) -> Result<bool, String> {
    let obj = require_obj(&profile, "stripResponsesImageDetail")?;
    Ok(
        obj.get("stripResponsesImageDetail") == Some(&Value::Bool(true))
            && str_prop(obj, "protocol").as_deref() == Some("responses")
            && str_prop(obj, "relayMode").as_deref() != Some("aggregate")
            && non_official_or_mixed(obj),
    )
}

/// Port of `pureApiImageGenerationEnabled` (`src/model-routes.ts:135`).
pub fn pure_api_image_generation_enabled(profile: Value) -> Result<bool, String> {
    let obj = require_obj(&profile, "pureApiImageGeneration")?;
    Ok(
        obj.get("pureApiImageGeneration") == Some(&Value::Bool(true))
            && str_prop(obj, "relayMode").as_deref() == Some("pureApi"),
    )
}

/// Port of `requestCompatibilityEnabled` (`src/model-routes.ts:161`).
pub fn request_compatibility_enabled(profile: Value) -> Result<bool, String> {
    let obj = require_obj(&profile, "relayMode")?;
    if !non_official_or_mixed(obj) {
        return Ok(false);
    }
    if str_prop(obj, "protocol").as_deref() == Some("responses")
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

/// Port of `settingsRequireLocalHelper` (`src/model-routes.ts:141`).
pub fn settings_require_local_helper(settings: Value) -> Result<bool, String> {
    let obj = require_obj(&settings, "enhancementsEnabled")?;
    if obj.get("enhancementsEnabled").is_some_and(is_truthy) {
        return Ok(true);
    }
    let profiles = obj
        .get("relayProfiles")
        .and_then(Value::as_array)
        .ok_or("TypeError: Cannot read properties of undefined (reading 'find')")?;
    let active_id = obj
        .get("activeRelayId")
        .cloned()
        .unwrap_or_else(crate::js::undefined);
    let mut matched = None;
    for candidate in profiles {
        let cand_obj = require_obj(candidate, "id")?;
        let cand_id = cand_obj
            .get("id")
            .cloned()
            .unwrap_or_else(crate::js::undefined);
        if js_strict_eq(&cand_id, &active_id) {
            matched = Some(candidate);
            break;
        }
    }
    let Some(active) = matched.or_else(|| profiles.first()) else {
        return Ok(false);
    };
    let a_obj = require_obj(active, "relayMode")?;
    let mode = str_prop(a_obj, "relayMode");
    let proto = str_prop(a_obj, "protocol");
    if mode.as_deref() == Some("aggregate")
        || proto.as_deref() == Some("chatCompletions")
        || (mode.as_deref() == Some("pureApi") && a_obj.get("noAuth") == Some(&Value::Bool(true)))
        || (mode.as_deref() == Some("official")
            && a_obj.get("officialMixApiKey").is_some_and(is_truthy))
        || independent_proxy_enabled(active.clone())?
        || web_search_history_compat_enabled(active.clone())?
        || local_compaction_prompt_at_end_enabled(active.clone())?
        || request_compatibility_enabled(active.clone())?
        || strip_image_generation_tool_enabled(active.clone())?
        || pure_api_image_generation_enabled(active.clone())?
    {
        return Ok(true);
    }
    let routes = a_obj
        .get("modelRoutes")
        .cloned()
        .unwrap_or_else(crate::js::undefined);
    Ok(normalize_relay_model_routes(routes)?.into_iter().any(|r| {
        let ro = r.as_object();
        let m = ro.and_then(|o| str_prop(o, "model")).unwrap_or_default();
        let t = ro
            .and_then(|o| str_prop(o, "targetRelayId"))
            .unwrap_or_default();
        !js_trim(&m).is_empty() && !js_trim(&t).is_empty()
    }))
}

/// Port of `modelRouteSaveRequiresRestart` (`src/model-routes.ts:172`).
pub fn model_route_save_requires_restart(
    current: Value,
    proposed: Value,
    active_live_base_url: String,
) -> Result<bool, String> {
    let prop_obj = require_obj(&proposed, "relayProfilesEnabled")?;
    if !prop_obj.get("relayProfilesEnabled").is_some_and(is_truthy) {
        return Ok(false);
    }
    if !settings_require_local_helper(proposed)? {
        return Ok(false);
    }
    if !settings_require_local_helper(current)? {
        return Ok(true);
    }
    Ok(js_trim(&active_live_base_url) != PROTOCOL_PROXY_BASE_URL)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mk_profile(id: &str) -> Value {
        json!({
            "id": id,
            "name": id,
            "baseUrl": format!("https://{id}.example/v1"),
            "apiKey": format!("sk-{id}"),
            "protocol": "responses",
            "relayMode": "pureApi",
            "officialMixApiKey": false,
            "modelRoutes": [],
        })
    }

    #[test]
    fn independent_proxy_thread_local_and_route_checks() {
        set_independent_proxy_configured("proxy-test".into(), true).unwrap();
        assert!(independent_proxy_enabled(mk_profile("proxy-test")).unwrap());
        set_independent_proxy_configured("proxy-test".into(), false).unwrap();
        assert!(!independent_proxy_enabled(mk_profile("proxy-test")).unwrap());

        let src = json!({
            "id": "src",
            "name": "SRC",
            "baseUrl": "https://s.example/v1",
            "apiKey": "sk-s",
            "protocol": "responses",
            "relayMode": "pureApi",
            "officialMixApiKey": false,
            "modelRoutes": [{ "model": "gpt-5.6-luna", "targetRelayId": "tgt", "targetModel": "" }],
        });
        let tgt = mk_profile("tgt");
        assert_eq!(
            find_relay_model_route_issue(json!([src.clone()]), json!([src, tgt])).unwrap(),
            None
        );
        assert!(independent_proxy_enabled(Value::Null).is_err());
    }
}
