//! The Rust port of `src/relay-profile-editing.ts`.

#![forbid(unsafe_code)]

use crate::js;
use fancy_regex::Regex as FancyRegex;
use manager_common_rs::assets_data::{translate_plain, translate_template};
use manager_common_rs::domain::{
    js_own_keys_order, js_stringify_compact, model_slug_from_row_name, resolve_provider_name,
    suffix_window_string,
};
use manager_common_rs::js_rt::{
    as_number, is_nil, is_plain_object, is_truthy, is_undefined, js_number, js_str, math_round,
    spread_object, string_units, undefined, utf16_string,
};
use manager_common_rs::relay_shared::{
    PROTOCOL_PROXY_BASE_URL, aggregate_member_candidates, clamp_aggregate_route_priority,
    find_relay_model_route_issue, is_aggregate_profile, local_compaction_prompt_at_end_enabled,
    normalize_aggregate_default_relay_id, normalize_aggregate_members, normalize_aggregate_routes,
    normalize_relay_model_routes, pure_api_image_generation_enabled,
    relay_profile_with_normalized_api_keys, relay_profile_with_updated_api_key,
    request_compatibility_enabled, session_provider_for_protocol_str,
    strip_image_generation_tool_enabled, strip_responses_image_detail_enabled,
    validate_aggregate_routes, web_search_history_compat_enabled,
};
use manager_common_rs::{es_trim, is_es_ws};
use serde_json::{Map, Value, json};
use std::collections::HashSet;
use std::sync::LazyLock;

const CHAT_UPSTREAM_BASE_URL_KEY: &str = "codex_plus_chat_base_url";
static UNDEF: LazyLock<Value> = LazyLock::new(undefined);

const AGGREGATE_STRATEGIES: [&str; 4] = [
    "failover",
    "conversationRoundRobin",
    "requestRoundRobin",
    "weightedRoundRobin",
];

// ── JS 运行时与字符串辅助 ────────────────────────────────────────────────────

fn t(zh: &str) -> String {
    translate_plain(zh, "zh")
}

fn tf(key: &str, args: &[Value]) -> String {
    translate_template(key, args, "zh")
}

fn nil_err(prop: &str) -> String {
    format!("TypeError: Cannot read properties of null/undefined (reading '{prop}')")
}

fn es_trim_start(s: &str) -> &str {
    s.trim_start_matches(is_es_ws)
}

fn es_trim_end(s: &str) -> &str {
    s.trim_end_matches(is_es_ws)
}

fn split_lines(s: &str) -> Vec<&str> {
    s.split('\n')
        .map(|l| l.strip_suffix('\r').unwrap_or(l))
        .collect()
}

fn u16_units(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

fn slice_u16(units: &[u16], start: usize, end: usize) -> String {
    let end = end.min(units.len());
    let start = start.min(end);
    String::from_utf16_lossy(&units[start..end])
}

fn prop<'a>(v: &'a Value, key: &str) -> Result<&'a Value, String> {
    if is_nil(v) {
        return Err(nil_err(key));
    }
    Ok(v.as_object()
        .filter(|_| is_plain_object(v))
        .and_then(|o| o.get(key))
        .unwrap_or(&UNDEF))
}

fn has_prop(v: &Value, key: &str) -> Result<bool, String> {
    if is_nil(v) || (!is_plain_object(v) && !v.is_array()) {
        return Err(format!(
            "TypeError: Cannot use 'in' operator to search for '{key}'"
        ));
    }
    Ok(v.as_object().is_some_and(|o| o.contains_key(key)))
}

fn str_prop(v: &Value, key: &str) -> Result<Option<String>, String> {
    let field = prop(v, key)?;
    Ok(string_units(field).map(|u| String::from_utf16_lossy(&u)))
}

fn req_str_field(v: &Value, key: &str, method: &str) -> Result<String, String> {
    let field = prop(v, key)?;
    if let Some(u) = string_units(field) {
        return Ok(String::from_utf16_lossy(&u));
    }
    if is_nil(field) {
        Err(nil_err(method))
    } else {
        Err(format!("TypeError: {key}.{method} is not a function"))
    }
}

fn opt_trim_field(v: &Value, key: &str) -> Result<String, String> {
    let field = prop(v, key)?;
    if is_nil(field) {
        return Ok(String::new());
    }
    let Some(u) = string_units(field) else {
        return Err(format!("TypeError: {key}?.trim is not a function"));
    };
    Ok(es_trim(&String::from_utf16_lossy(&u)).to_string())
}

fn str_or_empty(v: &Value, key: &str) -> Result<String, String> {
    let field = prop(v, key)?;
    Ok(if is_truthy(field) {
        js_str(field)
    } else {
        String::new()
    })
}

fn or_str(v: &Value, fallback: &str) -> String {
    if is_truthy(v) {
        js_str(v)
    } else {
        fallback.to_string()
    }
}

fn req_arr<'a>(v: &'a Value, method: &str) -> Result<&'a [Value], String> {
    if is_nil(v) {
        return Err(nil_err(method));
    }
    v.as_array()
        .map(Vec::as_slice)
        .ok_or_else(|| format!("TypeError: value.{method} is not a function"))
}

fn spread_js_map(base: &Value) -> Map<String, Value> {
    let mut map = spread_object(base);
    if map.is_empty()
        && !is_plain_object(base)
        && !base.is_array()
        && let Some(units) = string_units(base)
    {
        for (i, u) in units.into_iter().enumerate() {
            map.insert(i.to_string(), utf16_string(&[u]));
        }
    }
    map
}

fn spread_with(base: &Value, updates: &[(&str, Value)]) -> Value {
    let mut map = spread_js_map(base);
    for (k, v) in updates {
        map.insert((*k).to_string(), v.clone());
    }
    Value::Object(map)
}

fn spread_merge(base: &Value, patch: &Value) -> Value {
    let mut map = spread_js_map(base);
    for (k, v) in spread_js_map(patch) {
        map.insert(k, v);
    }
    Value::Object(map)
}

/// 生成与 V8 `JSON.stringify(v, null, 2)` 完全一致的缩进 JSON。
fn js_stringify_pretty_2(v: &Value, depth: usize) -> String {
    if let Some(arr) = v.as_array() {
        if arr.is_empty() {
            return "[]".into();
        }
        let pad = "  ".repeat(depth + 1);
        let close = "  ".repeat(depth);
        let items: Vec<String> = arr
            .iter()
            .map(|item| {
                if is_undefined(item) {
                    format!("{pad}null")
                } else {
                    format!("{pad}{}", js_stringify_pretty_2(item, depth + 1))
                }
            })
            .collect();
        return format!("[\n{}\n{close}]", items.join(",\n"));
    }
    if is_plain_object(v)
        && let Some(o) = v.as_object()
    {
        let keys: Vec<&String> = js_own_keys_order(o)
            .into_iter()
            .filter(|k| o.get(*k).is_some_and(|val| !is_undefined(val)))
            .collect();
        if keys.is_empty() {
            return "{}".into();
        }
        let pad = "  ".repeat(depth + 1);
        let close = "  ".repeat(depth);
        let entries: Vec<String> = keys
            .into_iter()
            .filter_map(|k| {
                let val = o.get(k)?;
                let key_json = js_stringify_compact(&Value::String(k.clone()));
                Some(format!(
                    "{pad}{key_json}: {}",
                    js_stringify_pretty_2(val, depth + 1)
                ))
            })
            .collect();
        return format!("{{\n{}\n{close}}}", entries.join(",\n"));
    }
    js_stringify_compact(v)
}

fn has_json_key_colon(haystack: &str, key: &str) -> bool {
    let pat = format!("\"{key}\"");
    let mut start = 0;
    while let Some(pos) = haystack[start..].find(&pat) {
        let after = &haystack[start + pos + pat.len()..];
        if after.trim_start_matches(is_es_ws).starts_with(':') {
            return true;
        }
        start += pos + pat.len();
    }
    false
}

fn has_model_suffix_bracket(trimmed: &str) -> bool {
    let Some(before_close) = trimmed.strip_suffix(']') else {
        return false;
    };
    let tail = before_close
        .rsplit(['\n', '\r', '\u{2028}', '\u{2029}'])
        .next()
        .unwrap_or(before_close);
    tail.find('[').is_some_and(|i| i + 1 < tail.len())
}

fn needs_proxy_base_url(profile: &Value, check_model_routes: bool) -> Result<bool, String> {
    let proto = str_prop(profile, "protocol")?;
    if proto.as_deref() != Some("responses") {
        return Ok(true);
    }
    if check_model_routes && !normalize_relay_model_routes(prop(profile, "modelRoutes")?).is_empty()
    {
        return Ok(true);
    }
    Ok(web_search_history_compat_enabled(profile)
        || local_compaction_prompt_at_end_enabled(profile)
        || request_compatibility_enabled(profile)?
        || strip_image_generation_tool_enabled(profile)
        || strip_responses_image_detail_enabled(profile)
        || pure_api_image_generation_enabled(profile))
}

fn profile_id_set(profiles: &[Value]) -> Result<HashSet<String>, String> {
    let mut set = HashSet::with_capacity(profiles.len());
    for p in profiles {
        let id = prop(p, "id")?;
        if let Some(s) = id.as_str() {
            set.insert(s.to_string());
        } else if !is_nil(id) {
            set.insert(js_str(id));
        }
    }
    Ok(set)
}

fn profile_id_str(p: &Value) -> Result<String, String> {
    let id = prop(p, "id")?;
    Ok(if is_undefined(id) {
        String::new()
    } else {
        js_str(id)
    })
}

fn parse_existing_ids_arg(v: &Value) -> Result<Option<HashSet<String>>, String> {
    if !is_truthy(v) {
        return Ok(None);
    }
    if let Some(arr) = v
        .as_object()
        .filter(|o| o.len() == 1)
        .and_then(|o| o.get("$set"))
        .and_then(Value::as_array)
    {
        let mut set = HashSet::new();
        for item in arr {
            if let Some(s) = item.as_str() {
                set.insert(s.to_string());
            }
        }
        return Ok(Some(set));
    }
    Err("TypeError: existingProfileIds.has is not a function".into())
}

// ── 生成与回读 config.toml / auth.json ──────────────────────────────────────

/// Port of `withGeneratedRelayFiles` (`src/relay-profile-editing.ts:58`).
pub fn with_generated_relay_files(profile: Value) -> Result<Value, String> {
    if is_aggregate_profile(&profile)? {
        let agg = normalize_aggregate_config(
            prop(&profile, "aggregate")?.clone(),
            Value::Array(Vec::new()),
            Value::Null,
        )?;
        return Ok(spread_with(
            &profile,
            &[
                ("configContents", Value::String(String::new())),
                ("authContents", Value::String(String::new())),
                ("aggregate", agg),
            ],
        ));
    }
    if str_prop(&profile, "relayMode")?.as_deref() == Some("official") {
        let mix = is_truthy(prop(&profile, "officialMixApiKey")?);
        let config = if mix {
            build_relay_config_toml(
                profile.clone(),
                json!({ "includeBearerToken": true, "requiresOpenAiAuth": true }),
            )?
        } else {
            String::new()
        };
        let auth = str_or_empty(&profile, "authContents")?;
        return Ok(spread_with(
            &profile,
            &[
                ("configContents", Value::String(config)),
                ("authContents", Value::String(auth)),
            ],
        ));
    }
    let no_auth = is_truthy(prop(&profile, "noAuth")?);
    let config = build_relay_config_toml(
        profile.clone(),
        json!({ "includeBearerToken": !no_auth, "requiresOpenAiAuth": false }),
    )?;
    let auth = build_relay_auth_json(profile.clone())?;
    Ok(spread_with(
        &profile,
        &[
            ("configContents", Value::String(config)),
            ("authContents", Value::String(auth)),
        ],
    ))
}

/// Port of `buildRelayConfigToml` (`src/relay-profile-editing.ts:78`).
pub fn build_relay_config_toml(profile: Value, options: Value) -> Result<String, String> {
    let base_url = if needs_proxy_base_url(&profile, false)? {
        PROTOCOL_PROXY_BASE_URL.to_string()
    } else {
        es_trim(&req_str_field(&profile, "baseUrl", "trim")?).to_string()
    };
    let api_key = es_trim(&req_str_field(&profile, "apiKey", "trim")?).to_string();
    let sp_val = normalize_relay_session_provider(prop(&profile, "sessionProvider")?.clone())?;
    let session_provider = sp_val.as_str().unwrap_or("custom");
    let model_raw = req_str_field(&profile, "model", "trim")?;
    let model = es_trim(&model_raw);

    let mut lines: Vec<String> = Vec::with_capacity(10);
    if !model.is_empty() {
        lines.push(format!("model = \"{}\"", toml_string(model.to_string())?));
    }
    lines.push(format!("model_provider = \"{session_provider}\""));
    if session_provider == "openai" {
        lines.push(format!("openai_base_url = \"{PROTOCOL_PROXY_BASE_URL}\""));
    }
    lines.push(String::new());
    lines.push("[model_providers.custom]".into());
    lines.push("name = \"custom\"".into());
    lines.push("wire_api = \"responses\"".into());
    let req_auth = prop(&options, "requiresOpenAiAuth")?;
    if !is_undefined(req_auth) {
        lines.push(format!(
            "requires_openai_auth = {}",
            if is_truthy(req_auth) { "true" } else { "false" }
        ));
    }
    lines.push(format!("base_url = \"{}\"", toml_string(base_url)?));
    if is_truthy(prop(&options, "includeBearerToken")?) && !api_key.is_empty() {
        lines.push(format!(
            "experimental_bearer_token = \"{}\"",
            toml_string(api_key)?
        ));
    }
    lines.push(String::new());
    Ok(lines.join("\n"))
}

/// Port of `buildRelayAuthJson` (`src/relay-profile-editing.ts:116`).
pub fn build_relay_auth_json(profile: Value) -> Result<String, String> {
    let key = es_trim(&req_str_field(&profile, "apiKey", "trim")?).to_string();
    Ok(format!(
        "{}\n",
        js_stringify_pretty_2(&json!({ "OPENAI_API_KEY": key }), 0)
    ))
}

/// Port of `buildOfficialRelayAuthJson` (`src/relay-profile-editing.ts:120`).
pub fn build_official_relay_auth_json(contents: String) -> Result<String, String> {
    let trimmed = es_trim(&contents);
    if trimmed.is_empty() {
        return Ok(String::new());
    }
    let Ok(mut parsed) = serde_json::from_str::<Value>(trimmed) else {
        return Ok(contents);
    };
    let Some(map) = parsed.as_object_mut() else {
        return Ok(contents);
    };
    let auth_mode = map
        .get("auth_mode")
        .filter(|v| is_truthy(v))
        .map(js_str)
        .unwrap_or_default();
    let known_oauth = auth_mode.to_lowercase() == "chatgpt"
        && map
            .get("tokens")
            .is_some_and(|t| t.is_object() || t.is_array());
    if !known_oauth && map.keys().any(|k| k != "OPENAI_API_KEY") {
        return Ok(contents);
    }
    map.shift_remove("OPENAI_API_KEY");
    Ok(format!("{}\n", js_stringify_pretty_2(&parsed, 0)))
}

/// Port of `deriveRelayProfileFromFiles` (`src/relay-profile-editing.ts:135`).
pub fn derive_relay_profile_from_files(
    profile: Value,
    edited_file: Value,
) -> Result<Value, String> {
    if is_aggregate_profile(&profile)? {
        return normalize_aggregate_relay_profile(profile, Value::Null);
    }
    let edited_str = edited_file.as_str();
    if edited_str == Some("auth") {
        let raw_auth = str_or_empty(&profile, "authContents")?;
        let to_parse = if raw_auth.is_empty() { "{}" } else { &raw_auth };
        if serde_json::from_str::<Value>(to_parse).is_err() {
            return Ok(profile);
        }
    }
    let relay_mode = str_prop(&profile, "relayMode")?;
    let mut config_contents = str_or_empty(&profile, "configContents")?;
    let raw_auth = str_or_empty(&profile, "authContents")?;
    let mut auth_contents = if relay_mode.as_deref() == Some("official") {
        build_official_relay_auth_json(raw_auth)?
    } else {
        raw_auth
    };
    let config_base_url = codex_base_url_from_config(config_contents.clone())?;
    let chat_upstream =
        root_toml_string_value(config_contents.clone(), CHAT_UPSTREAM_BASE_URL_KEY.into())?;
    let is_proxy = config_base_url == PROTOCOL_PROXY_BASE_URL;
    let profile_upstream = prop(&profile, "upstreamBaseUrl")?;
    let upstream_base_url = if is_truthy(profile_upstream) {
        js_str(profile_upstream)
    } else if !chat_upstream.is_empty() {
        chat_upstream
    } else if !config_base_url.is_empty() && !is_proxy {
        config_base_url
    } else {
        str_or_empty(&profile, "baseUrl")?
    };

    let config_api_key = codex_experimental_bearer_token_from_config(config_contents.clone())?;
    let api_key = if edited_str == Some("config") {
        config_api_key
    } else if edited_str == Some("auth") && relay_mode.as_deref() == Some("pureApi") {
        codex_api_key_from_auth(auth_contents.clone())?
    } else if relay_mode.as_deref() == Some("official") {
        if !config_api_key.is_empty() {
            config_api_key
        } else {
            str_or_empty(&profile, "apiKey")?
        }
    } else {
        let from_auth = codex_api_key_from_auth(auth_contents.clone())?;
        if !from_auth.is_empty() {
            from_auth
        } else {
            config_api_key
        }
    };

    if relay_mode.as_deref() == Some("pureApi") {
        let no_auth = is_truthy(prop(&profile, "noAuth")?);
        config_contents = if no_auth {
            remove_codex_experimental_bearer_token(config_contents)?
        } else if !api_key.is_empty() && !es_trim(&config_contents).is_empty() {
            set_codex_experimental_bearer_token(
                config_contents,
                api_key.clone(),
                json!({ "requiresOpenAiAuth": false }),
            )?
        } else {
            config_contents
        };
        if is_truthy(&edited_file) && !no_auth {
            if edited_str == Some("config") {
                auth_contents = set_auth_open_ai_api_key(auth_contents, api_key.clone())?;
            }
            if api_key.is_empty() {
                config_contents = remove_codex_experimental_bearer_token(config_contents)?;
            }
        }
    }

    let config_model = codex_model_from_config(config_contents.clone())?;
    let raw_model = req_str_field(&profile, "model", "trim")?;
    let trimmed_model = es_trim(&raw_model);
    let model = if has_model_suffix_bracket(trimmed_model) {
        trimmed_model.to_string()
    } else {
        config_model
    };
    let sp = relay_session_provider_from_config(config_contents.clone())?;
    let ctx_win =
        codex_top_level_int_from_config(config_contents.clone(), "model_context_window".into())?;
    let auto_compact = codex_top_level_int_from_config(
        config_contents.clone(),
        "model_auto_compact_token_limit".into(),
    )?;
    let derived = spread_with(
        &profile,
        &[
            ("model", Value::String(model)),
            ("sessionProvider", sp),
            ("baseUrl", Value::String(upstream_base_url.clone())),
            ("upstreamBaseUrl", Value::String(upstream_base_url)),
            ("apiKey", Value::String(api_key.clone())),
            ("contextWindow", Value::String(ctx_win)),
            ("autoCompactLimit", Value::String(auto_compact)),
            ("configContents", Value::String(config_contents)),
            ("authContents", Value::String(auth_contents)),
        ],
    );
    if is_truthy(&edited_file) {
        relay_profile_with_updated_api_key(&derived, &api_key)
    } else {
        Ok(derived)
    }
}

/// Port of `applyRelayProfilePatchToFiles` (`src/relay-profile-editing.ts:189`).
pub fn apply_relay_profile_patch_to_files(
    profile: Value,
    patch: Value,
    options: Value,
) -> Result<Value, String> {
    let options = if is_undefined(&options) {
        json!({})
    } else {
        options
    };
    let patch_proto = prop(&patch, "protocol")?;
    let protocol = if !is_nil(patch_proto) {
        patch_proto.clone()
    } else {
        prop(&profile, "protocol")?.clone()
    };
    let sp_val = if has_prop(&patch, "sessionProvider")? {
        normalize_relay_session_provider(prop(&patch, "sessionProvider")?.clone())?
    } else {
        relay_session_provider(profile.clone())?
    };
    let sp_str = sp_val.as_str().unwrap_or("custom");
    let proto_str = protocol.as_str().unwrap_or("");
    let compatible = session_provider_for_protocol_str(sp_str, proto_str);
    let normalized_patch = if compatible == sp_str {
        patch.clone()
    } else {
        spread_with(&patch, &[("sessionProvider", Value::String(compatible))])
    };
    let mut next = spread_merge(&profile, &normalized_patch);
    if is_aggregate_profile(&next)? {
        return normalize_aggregate_relay_profile(next, Value::Null);
    }

    let allow_gen = is_truthy(prop(&options, "allowGenerateFiles")?);
    let should_have_files = str_prop(&next, "relayMode")?.as_deref() != Some("official")
        || is_truthy(prop(&next, "officialMixApiKey")?)
        || !es_trim(&req_str_field(&next, "configContents", "trim")?).is_empty()
        || !es_trim(&req_str_field(&next, "authContents", "trim")?).is_empty();
    let needs_auth_file = str_prop(&next, "relayMode")?.as_deref() == Some("pureApi");
    if allow_gen
        && should_have_files
        && (es_trim(&req_str_field(&next, "configContents", "trim")?).is_empty()
            || (needs_auth_file
                && es_trim(&req_str_field(&next, "authContents", "trim")?).is_empty()))
    {
        next = with_generated_relay_files(next)?;
    }

    if has_prop(&patch, "sessionProvider")? {
        let sp = normalize_relay_session_provider(prop(&patch, "sessionProvider")?.clone())?;
        let sp_s = sp.as_str().unwrap_or("custom").to_string();
        let cfg = set_root_toml_string_key(
            str_or_empty(&next, "configContents")?,
            "model_provider".into(),
            sp_s.clone(),
        )?;
        let cfg = set_managed_open_ai_base_url(cfg, sp_s == "openai")?;
        next = spread_with(
            &next,
            &[
                ("sessionProvider", sp),
                ("configContents", Value::String(cfg)),
            ],
        );
    }
    if has_prop(&patch, "model")? {
        let parsed = parse_model_suffix(str_or_empty(&patch, "model")?)?;
        let slug = str_or_empty(&parsed, "slug")?;
        let cfg =
            set_root_toml_string_key(str_or_empty(&next, "configContents")?, "model".into(), slug)?;
        next = spread_with(&next, &[("configContents", Value::String(cfg))]);
    }
    if has_prop(&patch, "apiKey")? {
        let api_key = str_or_empty(&patch, "apiKey")?;
        next = relay_profile_with_updated_api_key(&next, &api_key)?;
        if str_prop(&next, "relayMode")?.as_deref() == Some("pureApi") {
            let auth =
                set_auth_open_ai_api_key(str_or_empty(&next, "authContents")?, api_key.clone())?;
            let cfg = set_codex_experimental_bearer_token(
                str_or_empty(&next, "configContents")?,
                api_key,
                json!({ "requiresOpenAiAuth": false }),
            )?;
            next = spread_with(
                &next,
                &[
                    ("authContents", Value::String(auth)),
                    ("configContents", Value::String(cfg)),
                ],
            );
        } else {
            let cfg = set_codex_experimental_bearer_token(
                str_or_empty(&next, "configContents")?,
                api_key,
                json!({}),
            )?;
            next = spread_with(&next, &[("configContents", Value::String(cfg))]);
        }
    }
    if has_prop(&patch, "baseUrl")? {
        let v = str_or_empty(&patch, "baseUrl")?;
        next = spread_with(&next, &[("upstreamBaseUrl", Value::String(v))]);
    }
    if has_prop(&patch, "upstreamBaseUrl")? {
        let v = str_or_empty(&patch, "upstreamBaseUrl")?;
        next = spread_with(&next, &[("baseUrl", Value::String(v))]);
    }
    const PROXY_KEYS: [&str; 11] = [
        "baseUrl",
        "upstreamBaseUrl",
        "protocol",
        "modelRoutes",
        "webSearchHistoryCompat",
        "localCompactionPromptAtEnd",
        "strictResponsesReasoning",
        "stripImageGenerationTool",
        "stripResponsesImageDetail",
        "pureApiImageGeneration",
        "modelMetadata",
    ];
    if PROXY_KEYS
        .iter()
        .any(|k| has_prop(&patch, k).unwrap_or(false))
    {
        let base_url_for_config = if needs_proxy_base_url(&next, true)? {
            PROTOCOL_PROXY_BASE_URL.to_string()
        } else {
            let up = prop(&next, "upstreamBaseUrl")?;
            if is_truthy(up) {
                js_str(up)
            } else {
                str_or_empty(&next, "baseUrl")?
            }
        };
        let req_auth = str_prop(&next, "relayMode")?.as_deref() != Some("pureApi");
        let cfg = set_codex_provider_string_key(
            str_or_empty(&next, "configContents")?,
            "base_url".into(),
            base_url_for_config,
            json!({ "requiresOpenAiAuth": req_auth }),
        )?;
        let cfg = remove_root_toml_key(cfg, CHAT_UPSTREAM_BASE_URL_KEY.into())?;
        next = spread_with(&next, &[("configContents", Value::String(cfg))]);
    }
    if has_prop(&patch, "contextWindow")? {
        let cfg = set_root_toml_int_key(
            str_or_empty(&next, "configContents")?,
            "model_context_window".into(),
            str_or_empty(&patch, "contextWindow")?,
        )?;
        next = spread_with(&next, &[("configContents", Value::String(cfg))]);
    }
    if has_prop(&patch, "autoCompactLimit")? {
        let cfg = set_root_toml_int_key(
            str_or_empty(&next, "configContents")?,
            "model_auto_compact_token_limit".into(),
            str_or_empty(&patch, "autoCompactLimit")?,
        )?;
        next = spread_with(&next, &[("configContents", Value::String(cfg))]);
    }
    if has_prop(&patch, "relayMode")? || has_prop(&patch, "officialMixApiKey")? {
        let is_plain_official = str_prop(&next, "relayMode")?.as_deref() == Some("official")
            && !is_truthy(prop(&next, "officialMixApiKey")?);
        if is_plain_official {
            let auth = build_official_relay_auth_json(str_or_empty(&next, "authContents")?)?;
            next = spread_with(
                &next,
                &[
                    ("configContents", Value::String(String::new())),
                    ("authContents", Value::String(auth)),
                ],
            );
        } else if allow_gen
            && (es_trim(&req_str_field(&next, "configContents", "trim")?).is_empty()
                || (str_prop(&next, "relayMode")?.as_deref() == Some("pureApi")
                    && es_trim(&req_str_field(&next, "authContents", "trim")?).is_empty()))
        {
            next = with_generated_relay_files(next)?;
        }
    }
    derive_relay_profile_from_files(next, undefined())
}

// ── TOML 解析与读写 ──────────────────────────────────────────────────────────

fn unescape_toml_basic_quotes(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\\'
            && let Some(&next) = chars.peek()
            && matches!(next, '"' | '\'' | '\\')
        {
            out.push(next);
            chars.next();
            continue;
        }
        out.push(c);
    }
    out
}

fn match_model_assignment_line(trimmed: &str) -> Option<String> {
    let after_key = trimmed.strip_prefix("model")?;
    let after_eq = es_trim_start(after_key).strip_prefix('=')?;
    let rest = es_trim_start(after_eq);
    let quote = rest.chars().next().filter(|&c| c == '"' || c == '\'')?;
    let end_trimmed = es_trim_end(rest);
    if end_trimmed.len() <= quote.len_utf8() || !end_trimmed.ends_with(quote) {
        return None;
    }
    let body = &end_trimmed[quote.len_utf8()..end_trimmed.len() - quote.len_utf8()];
    if body
        .chars()
        .any(|c| matches!(c, '\n' | '\r' | '\u{2028}' | '\u{2029}'))
    {
        return None;
    }
    Some(unescape_toml_basic_quotes(body))
}

/// Port of `codexModelFromConfig` (`src/relay-profile-editing.ts:279`).
pub fn codex_model_from_config(contents: String) -> Result<String, String> {
    let lines = split_lines(&contents);
    let structural = toml_structure_mask_slice(&lines);
    for (i, line) in lines.iter().enumerate() {
        if !structural[i] {
            continue;
        }
        let trimmed = es_trim(line);
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if trimmed.starts_with('[') {
            break;
        }
        if let Some(val) = match_model_assignment_line(trimmed) {
            return Ok(val);
        }
    }
    Ok(String::new())
}

/// Port of `parseModelSuffix` (`src/relay-profile-editing.ts:297`).
pub fn parse_model_suffix(raw: String) -> Result<Value, String> {
    match suffix_window_string(&raw) {
        None => Ok(json!({ "slug": es_trim(&raw) })),
        Some(win) => {
            let num: f64 = win.parse().unwrap_or(f64::NAN);
            Ok(json!({ "slug": model_slug_from_row_name(&raw), "window": js_number(num) }))
        }
    }
}

/// Port of `codexBaseUrlFromConfig` (`src/relay-profile-editing.ts:303`).
pub fn codex_base_url_from_config(contents: String) -> Result<String, String> {
    codex_provider_string_from_config(contents, "base_url".into())
}

/// Port of `codexExperimentalBearerTokenFromConfig` (`src/relay-profile-editing.ts:307`).
pub fn codex_experimental_bearer_token_from_config(contents: String) -> Result<String, String> {
    codex_provider_string_from_config(contents, "experimental_bearer_token".into())
}

/// Port of `codexProviderStringFromConfig` (`src/relay-profile-editing.ts:311`).
pub fn codex_provider_string_from_config(contents: String, key: String) -> Result<String, String> {
    let provider = root_toml_string_value(contents.clone(), "model_provider".into())?;
    let target_section = codex_provider_section_name(if provider.is_empty() {
        "custom".into()
    } else {
        provider.clone()
    })?;
    let fallback_section = if provider == "openai"
        && root_toml_string_value(contents.clone(), "openai_base_url".into())?
            == PROTOCOL_PROXY_BASE_URL
    {
        codex_provider_section_name("custom".into())?
    } else {
        String::new()
    };
    let lines = split_lines(&contents);
    let structural = toml_structure_mask_slice(&lines);
    let mut current_section = String::new();
    let mut root_value = String::new();
    let mut fallback_value = String::new();

    for (i, line) in lines.iter().enumerate() {
        if !structural[i] {
            continue;
        }
        if let Some(sec) = toml_section_name_opt(line) {
            current_section = sec;
            continue;
        }
        let Some(value) = toml_string_assignment_value_opt(line, &key) else {
            continue;
        };
        if current_section == target_section {
            return Ok(value);
        }
        if !fallback_section.is_empty() && current_section == fallback_section {
            fallback_value = value;
        } else if provider.is_empty() && current_section.is_empty() {
            root_value = value;
        }
    }
    Ok(if !fallback_value.is_empty() {
        fallback_value
    } else {
        root_value
    })
}

/// Port of `codexApiKeyFromAuth` (`src/relay-profile-editing.ts:340`).
pub fn codex_api_key_from_auth(contents: String) -> Result<String, String> {
    let src = if contents.is_empty() { "{}" } else { &contents };
    let Ok(parsed) = serde_json::from_str::<Value>(src) else {
        return Ok(String::new());
    };
    Ok(parsed
        .get("OPENAI_API_KEY")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string())
}

/// Port of `codexTopLevelIntFromConfig` (`src/relay-profile-editing.ts:349`).
pub fn codex_top_level_int_from_config(contents: String, key: String) -> Result<String, String> {
    let split = split_toml_root_and_tables(contents)?;
    let top_level = split.get("root").and_then(Value::as_str).unwrap_or("");
    let ws = r"(?:[^\S\u{0085}]|\u{FEFF})";
    let pattern =
        format!(r#"^{ws}*{key}{ws}*={ws}*([0-9]+){ws}*(?:#[^\r\n\u{{2028}}\u{{2029}}]*)?$"#);
    let re = FancyRegex::new(&pattern)
        .map_err(|e| format!("SyntaxError: Invalid regular expression: {e}"))?;
    let lines = split_lines(top_level);
    let structural = toml_structure_mask_slice(&lines);
    for (i, line) in lines.iter().enumerate() {
        if !structural[i] {
            continue;
        }
        if let Some(caps) = re.captures(line).map_err(|e| format!("RegexError: {e}"))?
            && let Some(m) = caps.get(1)
        {
            return Ok(m.as_str().to_string());
        }
    }
    Ok(String::new())
}

/// Port of `rootTomlStringValue` (`src/relay-profile-editing.ts:362`).
pub fn root_toml_string_value(contents: String, key: String) -> Result<String, String> {
    let split = split_toml_root_and_tables(contents)?;
    let top_level = split.get("root").and_then(Value::as_str).unwrap_or("");
    let lines = split_lines(top_level);
    let structural = toml_structure_mask_slice(&lines);
    for (i, line) in lines.iter().enumerate() {
        if !structural[i] {
            continue;
        }
        if let Some(v) = toml_string_assignment_value_opt(line, &key) {
            return Ok(v);
        }
    }
    Ok(String::new())
}

fn toml_structure_mask_slice(lines: &[&str]) -> Vec<bool> {
    let mut multiline: Option<u16> = None;
    let mut out = Vec::with_capacity(lines.len());
    for line in lines {
        let structural = multiline.is_none();
        let units = u16_units(line);
        let mut idx = 0usize;
        while idx < units.len() {
            let u = units[idx];
            if let Some(mq) = multiline {
                if mq == 0x22 && u == 0x5C {
                    idx += 1;
                } else if u == mq {
                    let mut end = idx;
                    while units.get(end).copied() == Some(mq) {
                        end += 1;
                    }
                    if end - idx >= 3 {
                        multiline = None;
                    }
                    idx = end - 1;
                }
            } else if u == 0x23 {
                break;
            } else if u == 0x22 || u == 0x27 {
                if units.get(idx + 1).copied() == Some(u) && units.get(idx + 2).copied() == Some(u)
                {
                    multiline = Some(u);
                    idx += 2;
                } else if let Some((_, end)) = toml_quoted_value_units(&units, idx) {
                    idx = end - 1;
                } else {
                    break;
                }
            }
            idx += 1;
        }
        out.push(structural);
    }
    out
}

/// Port of `tomlStructureMask` (`src/relay-profile-editing.ts:374`).
pub fn toml_structure_mask(lines: Vec<String>) -> Result<Vec<bool>, String> {
    let refs: Vec<&str> = lines.iter().map(String::as_str).collect();
    Ok(toml_structure_mask_slice(&refs))
}

fn toml_section_name_opt(line: &str) -> Option<String> {
    let text = es_trim(line);
    if !text.starts_with('[') {
        return None;
    }
    let array = text.starts_with("[[");
    let start = if array { 2 } else { 1 };
    let units = u16_units(text);
    let mut idx = start;
    while idx < units.len() {
        let u = units[idx];
        if u == 0x22 || u == 0x27 {
            let (_, end) = toml_quoted_value_units(&units, idx)?;
            idx = end - 1;
        } else if u == 0x5D {
            let end = idx + if array { 2 } else { 1 };
            if array && units.get(idx + 1).copied() != Some(0x5D) {
                return None;
            }
            let rest_str = slice_u16(&units, end, units.len());
            let rest = es_trim(&rest_str);
            if !rest.is_empty() && !rest.starts_with('#') {
                return None;
            }
            let inner = slice_u16(&units, start, idx);
            let path = toml_dotted_key_path_opt(&inner)?;
            return Some(format!(
                "{}{}",
                if array { "[]" } else { "" },
                toml_key_path_name_slice(&path)
            ));
        }
        idx += 1;
    }
    None
}

/// Port of `tomlSectionName` (`src/relay-profile-editing.ts:406`).
pub fn toml_section_name(line: String) -> Result<Value, String> {
    Ok(toml_section_name_opt(&line).map_or(Value::Null, Value::String))
}

fn toml_quoted_value_units(units: &[u16], start: usize) -> Option<(Vec<u16>, usize)> {
    let quote = *units.get(start)?;
    if quote != 0x22 && quote != 0x27 {
        return None;
    }
    let mut out = Vec::new();
    let mut idx = start + 1;
    while idx < units.len() {
        let ch = units[idx];
        if ch == quote {
            return Some((out, idx + 1));
        }
        if (ch < 32 && ch != 9) || ch == 127 {
            return None;
        }
        if quote == 0x22 && ch == 0x5C {
            idx += 1;
            let esc = units.get(idx).copied()?;
            match esc {
                0x22 => out.push(0x22),
                0x5C => out.push(0x5C),
                0x62 => out.push(0x08),
                0x74 => out.push(0x09),
                0x6E => out.push(0x0A),
                0x66 => out.push(0x0C),
                0x72 => out.push(0x0D),
                0x75 | 0x55 => {
                    let len = if esc == 0x75 { 4 } else { 8 };
                    let hex_slice = units.get(idx + 1..idx + 1 + len)?;
                    if !hex_slice
                        .iter()
                        .all(|&u| u <= 127 && (u as u8).is_ascii_hexdigit())
                    {
                        return None;
                    }
                    let hex: String = hex_slice.iter().map(|&u| u as u8 as char).collect();
                    let cp = u32::from_str_radix(&hex, 16).ok()?;
                    if cp > 0x10_FFFF || (0xD800..=0xDFFF).contains(&cp) {
                        return None;
                    }
                    let c = char::from_u32(cp)?;
                    let mut buf = [0u16; 2];
                    out.extend_from_slice(c.encode_utf16(&mut buf));
                    idx += len;
                }
                _ => return None,
            }
        } else {
            out.push(ch);
        }
        idx += 1;
    }
    None
}

/// Port of `tomlQuotedValue` (`src/relay-profile-editing.ts:429`).
pub fn toml_quoted_value(text: String, start: f64) -> Result<Value, String> {
    if !start.is_finite() || start < 0.0 || start.fract() != 0.0 {
        return Ok(Value::Null);
    }
    let units = u16_units(&text);
    if start >= units.len() as f64 {
        return Ok(Value::Null);
    }
    match toml_quoted_value_units(&units, start as usize) {
        Some((val, end)) => Ok(json!({ "value": utf16_string(&val), "end": end })),
        None => Ok(Value::Null),
    }
}

fn is_toml_bare_key_unit(u: u16) -> bool {
    u <= 127 && ((u as u8).is_ascii_alphanumeric() || u == b'_' as u16 || u == b'-' as u16)
}

fn toml_dotted_key_path_opt(text: &str) -> Option<Vec<String>> {
    let units = u16_units(text);
    let mut path = Vec::new();
    let mut idx = 0usize;
    while idx < units.len() {
        while matches!(units.get(idx).copied(), Some(0x20 | 0x09)) {
            idx += 1;
        }
        let cur = *units.get(idx)?;
        if cur == 0x22 || cur == 0x27 {
            let (val, end) = toml_quoted_value_units(&units, idx)?;
            path.push(String::from_utf16_lossy(&val));
            idx = end;
        } else {
            let start = idx;
            while units.get(idx).copied().is_some_and(is_toml_bare_key_unit) {
                idx += 1;
            }
            if idx == start {
                return None;
            }
            path.push(slice_u16(&units, start, idx));
        }
        while matches!(units.get(idx).copied(), Some(0x20 | 0x09)) {
            idx += 1;
        }
        if idx == units.len() {
            return Some(path);
        }
        if units[idx] != 0x2E {
            return None;
        }
        idx += 1;
    }
    None
}

/// Port of `tomlDottedKeyPath` (`src/relay-profile-editing.ts:455`).
pub fn toml_dotted_key_path(text: String) -> Result<Value, String> {
    Ok(match toml_dotted_key_path_opt(&text) {
        Some(path) => Value::Array(path.into_iter().map(Value::String).collect()),
        None => Value::Null,
    })
}

fn toml_key_path_name_slice(path: &[String]) -> String {
    path.iter()
        .map(|part| {
            if !part.is_empty()
                && part
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
            {
                part.clone()
            } else {
                js_stringify_compact(&Value::String(part.clone()))
            }
        })
        .collect::<Vec<_>>()
        .join(".")
}

/// Port of `tomlKeyPathName` (`src/relay-profile-editing.ts:479`).
pub fn toml_key_path_name(path: Vec<String>) -> Result<String, String> {
    Ok(toml_key_path_name_slice(&path))
}

/// Port of `codexProviderSectionName` (`src/relay-profile-editing.ts:483`).
pub fn codex_provider_section_name(provider: String) -> Result<String, String> {
    Ok(toml_key_path_name_slice(&[
        "model_providers".to_string(),
        provider,
    ]))
}

fn toml_key_assignment_units(units: &[u16]) -> Option<(Vec<String>, usize)> {
    let mut idx = 0usize;
    while idx < units.len() {
        let u = units[idx];
        if u == 0x22 || u == 0x27 {
            let (_, end) = toml_quoted_value_units(units, idx)?;
            idx = end - 1;
        } else if u == 0x23 {
            return None;
        } else if u == 0x3D {
            let prefix = slice_u16(units, 0, idx);
            let path = toml_dotted_key_path_opt(&prefix)?;
            return Some((path, idx + 1));
        }
        idx += 1;
    }
    None
}

/// Port of `tomlKeyAssignment` (`src/relay-profile-editing.ts:487`).
pub fn toml_key_assignment(line: String) -> Result<Value, String> {
    let units = u16_units(&line);
    Ok(match toml_key_assignment_units(&units) {
        Some((path, value_start)) => json!({ "path": path, "valueStart": value_start }),
        None => Value::Null,
    })
}

fn toml_inline_comment_units(units: &[u16], start: f64) -> String {
    if start.is_nan() || start == f64::INFINITY || start.fract() != 0.0 {
        return String::new();
    }
    let slice_start = if start < 0.0 {
        (units.len() as f64 + start).max(0.0) as usize
    } else {
        (start.min(units.len() as f64)) as usize
    };
    let mut idx = if start < 0.0 { 0usize } else { slice_start };
    while idx < units.len() {
        let u = units[idx];
        if u == 0x22 || u == 0x27 {
            let Some((_, end)) = toml_quoted_value_units(units, idx) else {
                return String::new();
            };
            idx = end - 1;
        } else if u == 0x23 {
            let between = &units[slice_start.min(idx)..idx];
            let ws_count = between
                .iter()
                .rev()
                .take_while(|&&c| c == 0x20 || c == 0x09)
                .count();
            let spacing = String::from_utf16_lossy(&between[between.len() - ws_count..]);
            let rest = String::from_utf16_lossy(&units[idx..]);
            return format!("{spacing}{rest}");
        }
        idx += 1;
    }
    String::new()
}

/// Port of `tomlInlineComment` (`src/relay-profile-editing.ts:502`).
pub fn toml_inline_comment(line: String, start: f64) -> Result<String, String> {
    Ok(toml_inline_comment_units(&u16_units(&line), start))
}

fn toml_string_assignment_value_opt(line: &str, key: &str) -> Option<String> {
    let units = u16_units(line);
    let (path, value_start) = toml_key_assignment_units(&units)?;
    if path.len() != 1 || path[0] != key {
        return None;
    }
    let after = slice_u16(&units, value_start, units.len());
    let text = es_trim_start(&after);
    let text_units = u16_units(text);
    let (val, end) = toml_quoted_value_units(&text_units, 0)?;
    let rest_raw = slice_u16(&text_units, end, text_units.len());
    let rest = es_trim_start(&rest_raw);
    if rest.is_empty() || rest.starts_with('#') {
        Some(String::from_utf16_lossy(&val))
    } else {
        None
    }
}

/// Port of `tomlStringAssignmentValue` (`src/relay-profile-editing.ts:516`).
pub fn toml_string_assignment_value(line: String, key: String) -> Result<Value, String> {
    Ok(toml_string_assignment_value_opt(&line, &key).map_or(Value::Null, Value::String))
}

/// Port of `setAuthOpenAiApiKey` (`src/relay-profile-editing.ts:526`).
pub fn set_auth_open_ai_api_key(contents: String, api_key: String) -> Result<String, String> {
    let src = if contents.is_empty() { "{}" } else { &contents };
    let mut parsed = match serde_json::from_str::<Value>(src) {
        Ok(Value::Object(m)) => m,
        _ => Map::new(),
    };
    parsed.insert(
        "OPENAI_API_KEY".into(),
        Value::String(es_trim(&api_key).to_string()),
    );
    Ok(format!(
        "{}\n",
        js_stringify_pretty_2(&Value::Object(parsed), 0)
    ))
}

/// Port of `setRootTomlStringKey` (`src/relay-profile-editing.ts:538`).
pub fn set_root_toml_string_key(
    contents: String,
    key: String,
    value: String,
) -> Result<String, String> {
    let trimmed = es_trim(&value);
    if trimmed.is_empty() {
        return remove_root_toml_key(contents, key);
    }
    let line = format!("{key} = \"{}\"", toml_string(trimmed.to_string())?);
    set_root_toml_line(contents, key, line)
}

/// Port of `setManagedOpenAiBaseUrl` (`src/relay-profile-editing.ts:544`).
pub fn set_managed_open_ai_base_url(contents: String, enabled: bool) -> Result<String, String> {
    let current = root_toml_string_value(contents.clone(), "openai_base_url".into())?;
    if enabled {
        if current.is_empty() || current == PROTOCOL_PROXY_BASE_URL {
            set_root_toml_string_key(
                contents,
                "openai_base_url".into(),
                PROTOCOL_PROXY_BASE_URL.into(),
            )
        } else {
            Ok(contents)
        }
    } else if current == PROTOCOL_PROXY_BASE_URL {
        remove_root_toml_key(contents, "openai_base_url".into())
    } else {
        Ok(contents)
    }
}

/// Port of `setRootTomlIntKey` (`src/relay-profile-editing.ts:554`).
pub fn set_root_toml_int_key(
    contents: String,
    key: String,
    value: String,
) -> Result<String, String> {
    let digits: String = value.chars().filter(char::is_ascii_digit).collect();
    if digits.is_empty() {
        return remove_root_toml_key(contents, key);
    }
    let line = format!("{key} = {digits}");
    set_root_toml_line(contents, key, line)
}

/// Port of `setRootTomlLine` (`src/relay-profile-editing.ts:560`).
pub fn set_root_toml_line(
    contents: String,
    key: String,
    line_text: String,
) -> Result<String, String> {
    let mut lines: Vec<String> = split_lines(&contents)
        .into_iter()
        .map(str::to_string)
        .collect();
    let refs: Vec<&str> = lines.iter().map(String::as_str).collect();
    let structural = toml_structure_mask_slice(&refs);
    let root_end = lines
        .iter()
        .enumerate()
        .position(|(i, l)| structural[i] && toml_section_name_opt(l).is_some())
        .unwrap_or(lines.len());
    for i in 0..root_end {
        if !structural[i] {
            continue;
        }
        let line_units = u16_units(&lines[i]);
        if let Some((path, val_start)) = toml_key_assignment_units(&line_units)
            && path.len() == 1
            && path[0] == key
        {
            let rep_units = u16_units(&line_text);
            lines[i] = if let Some((_, rep_start)) = toml_key_assignment_units(&rep_units) {
                let prefix = slice_u16(&line_units, 0, val_start);
                let rhs = slice_u16(&rep_units, rep_start, rep_units.len());
                let comment = toml_inline_comment_units(&line_units, val_start as f64);
                format!("{prefix} {}{comment}", es_trim_start(&rhs))
            } else {
                line_text
            };
            return ensure_trailing_newline(es_trim_end(&lines.join("\n")).to_string());
        }
    }
    let insert_at = if key == "model" { 0 } else { root_end };
    lines.insert(insert_at, line_text);
    ensure_trailing_newline(es_trim_end(&lines.join("\n")).to_string())
}

fn match_requires_openai_auth_line(line: &str) -> Option<bool> {
    let trimmed = es_trim_start(line);
    let key_part = trimmed.get(..20)?;
    if !key_part.eq_ignore_ascii_case("requires_openai_auth") {
        return None;
    }
    let after_eq = es_trim_start(&trimmed[20..]).strip_prefix('=')?;
    let val_start = es_trim_start(after_eq);
    let (flag, rest) = if val_start.len() >= 4 && val_start[..4].eq_ignore_ascii_case("true") {
        (true, &val_start[4..])
    } else if val_start.len() >= 5 && val_start[..5].eq_ignore_ascii_case("false") {
        (false, &val_start[5..])
    } else {
        return None;
    };
    let tail = es_trim_start(rest);
    if tail.is_empty()
        || (tail.starts_with('#')
            && !tail
                .chars()
                .any(|c| matches!(c, '\n' | '\r' | '\u{2028}' | '\u{2029}')))
    {
        Some(flag)
    } else {
        None
    }
}

/// Port of `codexRequiresOpenAiAuthFromConfig` (`src/relay-profile-editing.ts:581`).
pub fn codex_requires_open_ai_auth_from_config(contents: String) -> Result<bool, String> {
    let provider = root_toml_string_value(contents.clone(), "model_provider".into())?;
    let target_section = if provider.is_empty() {
        String::new()
    } else {
        codex_provider_section_name(provider)?
    };
    let lines = split_lines(&contents);
    let structural = toml_structure_mask_slice(&lines);
    let mut current_section = String::new();
    let mut saw_provider_section = false;
    let mut root_requires_auth = false;

    for (i, line) in lines.iter().enumerate() {
        if !structural[i] {
            continue;
        }
        if let Some(sec) = toml_section_name_opt(line) {
            if sec.starts_with("model_providers.") {
                saw_provider_section = true;
            }
            current_section = sec;
            continue;
        }
        let Some(flag) = match_requires_openai_auth_line(line) else {
            continue;
        };
        if current_section.is_empty() {
            root_requires_auth = flag;
        }
        if !current_section.starts_with("model_providers.") {
            continue;
        }
        if !target_section.is_empty() {
            if current_section == target_section {
                return Ok(flag);
            }
            continue;
        }
        if flag {
            return Ok(true);
        }
    }
    Ok(!saw_provider_section && root_requires_auth)
}

fn active_table_provider(contents: &str) -> Result<(String, String), String> {
    let root_provider = root_toml_string_value(contents.to_string(), "model_provider".into())?;
    let session_provider = if root_provider.is_empty() {
        "custom".to_string()
    } else {
        root_provider
    };
    let provider = if session_provider == "openai" {
        "custom".to_string()
    } else {
        session_provider.clone()
    };
    Ok((session_provider, provider))
}

/// Port of `setCodexProviderStringKey` (`src/relay-profile-editing.ts:611`).
pub fn set_codex_provider_string_key(
    contents: String,
    key: String,
    value: String,
    options: Value,
) -> Result<String, String> {
    let options = if is_undefined(&options) {
        json!({})
    } else {
        options
    };
    let (session_provider, provider) = active_table_provider(&contents)?;
    let mut next = contents;
    if root_toml_string_value(next.clone(), "model_provider".into())?.is_empty() {
        next = set_root_toml_string_key(next, "model_provider".into(), session_provider)?;
    }
    let req_auth = prop(&options, "requiresOpenAiAuth")? != &Value::Bool(false);
    next = ensure_codex_provider_defaults(
        next,
        provider.clone(),
        json!({ "requiresOpenAiAuth": req_auth }),
    )?;
    set_toml_section_string_key(next, codex_provider_section_name(provider)?, key, value)
}

/// Port of `setCodexExperimentalBearerToken` (`src/relay-profile-editing.ts:627`).
pub fn set_codex_experimental_bearer_token(
    contents: String,
    api_key: String,
    options: Value,
) -> Result<String, String> {
    let options = if is_undefined(&options) {
        json!({})
    } else {
        options
    };
    let trimmed = es_trim(&api_key).to_string();
    if !trimmed.is_empty() && prop(&options, "requiresOpenAiAuth")? == &Value::Bool(false) {
        let (session_provider, provider) = active_table_provider(&contents)?;
        let next = if root_toml_string_value(contents.clone(), "model_provider".into())?.is_empty()
        {
            set_root_toml_string_key(contents, "model_provider".into(), session_provider)?
        } else {
            contents
        };
        return set_toml_section_string_key(
            next,
            codex_provider_section_name(provider)?,
            "experimental_bearer_token".into(),
            trimmed,
        );
    }
    if !trimmed.is_empty() {
        set_codex_provider_string_key(
            contents,
            "experimental_bearer_token".into(),
            trimmed,
            options,
        )
    } else {
        remove_codex_experimental_bearer_token(contents)
    }
}

/// Port of `removeCodexExperimentalBearerToken` (`src/relay-profile-editing.ts:643`).
pub fn remove_codex_experimental_bearer_token(contents: String) -> Result<String, String> {
    let (_, provider) = active_table_provider(&contents)?;
    remove_toml_section_key(
        contents,
        codex_provider_section_name(provider)?,
        "experimental_bearer_token".into(),
    )
}

/// Port of `ensureCodexProviderDefaults` (`src/relay-profile-editing.ts:649`).
pub fn ensure_codex_provider_defaults(
    contents: String,
    provider: String,
    options: Value,
) -> Result<String, String> {
    let options = if is_undefined(&options) {
        json!({})
    } else {
        options
    };
    let section = codex_provider_section_name(provider.clone())?;
    let name = resolve_provider_name(&contents, &provider)?;
    let mut next = set_toml_section_string_key(contents, section.clone(), "name".into(), name)?;
    next =
        set_toml_section_string_key(next, section.clone(), "wire_api".into(), "responses".into())?;
    if prop(&options, "requiresOpenAiAuth")? == &Value::Bool(false) {
        Ok(next)
    } else {
        set_toml_section_bool_key(next, section, "requires_openai_auth".into(), true)
    }
}

/// Port of `setTomlSectionBoolKey` (`src/relay-profile-editing.ts:663`).
pub fn set_toml_section_bool_key(
    contents: String,
    section_name: String,
    key: String,
    value: bool,
) -> Result<String, String> {
    set_toml_section_raw_key(
        contents,
        section_name,
        key,
        if value { "true" } else { "false" }.into(),
    )
}

/// Port of `setTomlSectionStringKey` (`src/relay-profile-editing.ts:667`).
pub fn set_toml_section_string_key(
    contents: String,
    section_name: String,
    key: String,
    value: String,
) -> Result<String, String> {
    let escaped = toml_string(es_trim(&value).to_string())?;
    set_toml_section_raw_key(contents, section_name, key, format!("\"{escaped}\""))
}

fn find_section_bounds(
    lines: &[&str],
    structural: &[bool],
    section_name: &str,
) -> (Option<usize>, usize) {
    let mut start = None;
    let mut end = lines.len();
    for (i, line) in lines.iter().enumerate() {
        if !structural[i] {
            continue;
        }
        let Some(sec) = toml_section_name_opt(line) else {
            continue;
        };
        if start.is_some() {
            end = i;
            break;
        }
        if sec == section_name {
            start = Some(i);
        }
    }
    (start, end)
}

/// Port of `setTomlSectionRawKey` (`src/relay-profile-editing.ts:671`).
pub fn set_toml_section_raw_key(
    contents: String,
    section_name: String,
    key: String,
    value: String,
) -> Result<String, String> {
    let mut lines: Vec<String> = split_lines(&contents)
        .into_iter()
        .map(str::to_string)
        .collect();
    let refs: Vec<&str> = lines.iter().map(String::as_str).collect();
    let structural = toml_structure_mask_slice(&refs);
    let (sec_start, sec_end) = find_section_bounds(&refs, &structural, &section_name);
    let Some(section_start) = sec_start else {
        let prefix = es_trim_end(&ensure_trailing_newline(
            es_trim_end(&lines.join("\n")).to_string(),
        )?)
        .to_string();
        return join_toml_sections(vec![prefix, format!("[{section_name}]\n{key} = {value}")]);
    };
    for i in (section_start + 1)..sec_end {
        if !structural[i] {
            continue;
        }
        let line_units = u16_units(&lines[i]);
        if let Some((path, val_start)) = toml_key_assignment_units(&line_units)
            && path.len() == 1
            && path[0] == key
        {
            let prefix = slice_u16(&line_units, 0, val_start);
            let comment = toml_inline_comment_units(&line_units, val_start as f64);
            lines[i] = format!("{prefix} {value}{comment}");
            return ensure_trailing_newline(es_trim_end(&lines.join("\n")).to_string());
        }
    }
    let mut insert_at = sec_end;
    while insert_at > section_start + 1 && es_trim(&lines[insert_at - 1]).is_empty() {
        insert_at -= 1;
    }
    lines.insert(insert_at, format!("{key} = {value}"));
    ensure_trailing_newline(es_trim_end(&lines.join("\n")).to_string())
}

/// Port of `removeTomlSectionKey` (`src/relay-profile-editing.ts:705`).
pub fn remove_toml_section_key(
    contents: String,
    section_name: String,
    key: String,
) -> Result<String, String> {
    let lines = split_lines(&contents);
    let structural = toml_structure_mask_slice(&lines);
    let (sec_start, sec_end) = find_section_bounds(&lines, &structural, &section_name);
    let Some(section_start) = sec_start else {
        return Ok(contents);
    };
    let next: Vec<&str> = lines
        .into_iter()
        .enumerate()
        .filter(|&(i, line)| {
            if i <= section_start || i >= sec_end || !structural[i] {
                return true;
            }
            let units = u16_units(line);
            match toml_key_assignment_units(&units) {
                Some((path, _)) => path.len() != 1 || path[0] != key,
                None => true,
            }
        })
        .map(|(_, line)| line)
        .collect();
    ensure_trailing_newline(es_trim_end(&next.join("\n")).to_string())
}

// ── 校验与消息 ──────────────────────────────────────────────────────────────

/// Port of `relayProfileSwitchValidation` (`src/relay-profile-editing.ts:729`).
pub fn relay_profile_switch_validation(profile: Value, settings: Value) -> Result<Value, String> {
    let settings = if is_undefined(&settings) {
        Value::Null
    } else {
        settings
    };
    let session_err = relay_session_provider_validation(profile.clone())?;
    if !session_err.is_null() {
        return Ok(session_err);
    }
    if is_aggregate_profile(&profile)? {
        return aggregate_relay_profile_validation(profile);
    }
    let route_err = relay_model_routes_validation(profile.clone(), settings)?;
    if !route_err.is_null() {
        return Ok(route_err);
    }
    let mode = str_prop(&profile, "relayMode")?;
    let mix = is_truthy(prop(&profile, "officialMixApiKey")?);
    if mode.as_deref() == Some("official") && !mix {
        return Ok(Value::Null);
    }
    let cfg = req_str_field(&profile, "configContents", "trim")?;
    if es_trim(&cfg).is_empty() {
        let label = {
            let n = prop(&profile, "name")?;
            if is_truthy(n) {
                n.clone()
            } else {
                prop(&profile, "id")?.clone()
            }
        };
        return Ok(Value::String(tf(
            "供应商「{0}」缺少独立 config.toml，已停止切换，避免继续显示上一套配置文件。请先在该供应商详情里保存 config.toml。",
            &[label],
        )));
    }
    let auth = str_or_empty(&profile, "authContents")?;
    if mode.as_deref() != Some("official") || !auth_json_has_open_ai_api_key(auth)? {
        return Ok(Value::Null);
    }
    Ok(Value::String(t(
        "官方混合 API 不应在 auth.json 中保存 OPENAI_API_KEY。请清理此供应商的 auth.json 后再切换。",
    )))
}

fn route_sources_for_issue(items: &[Value]) -> Vec<Value> {
    items
        .iter()
        .map(|item| {
            if !is_nil(item) && !is_plain_object(item) {
                json!({})
            } else {
                item.clone()
            }
        })
        .collect()
}

/// Port of `relayModelRoutesValidation` (`src/relay-profile-editing.ts:745`).
pub fn relay_model_routes_validation(profile: Value, settings: Value) -> Result<Value, String> {
    let fallback = vec![profile.clone()];
    let all_profiles: Vec<Value> = if is_nil(&settings) {
        fallback
    } else {
        let rp = prop(&settings, "relayProfiles")?;
        if is_nil(rp) {
            fallback
        } else {
            req_arr(rp, "find")?.to_vec()
        }
    };
    let sources = route_sources_for_issue(&[profile]);
    let issue = find_relay_model_route_issue(&sources, &all_profiles)?;
    relay_model_route_issue_message(issue.unwrap_or(Value::Null))
}

/// Port of `relayModelRoutesSettingsValidation` (`src/relay-profile-editing.ts:750`).
pub fn relay_model_routes_settings_validation(settings: Value) -> Result<Value, String> {
    let rp = req_arr(prop(&settings, "relayProfiles")?, "find")?;
    let sources = route_sources_for_issue(rp);
    let issue = find_relay_model_route_issue(&sources, rp)?;
    relay_model_route_issue_message(issue.unwrap_or(Value::Null))
}

/// Port of `relaySettingsValidation` (`src/relay-profile-editing.ts:756`).
pub fn relay_settings_validation(settings: Value) -> Result<Value, String> {
    let profiles = req_arr(prop(&settings, "relayProfiles")?, "Symbol.iterator")?;
    for p in profiles {
        let err = relay_session_provider_validation(p.clone())?;
        if !err.is_null() {
            return Ok(err);
        }
    }
    relay_model_routes_settings_validation(settings)
}

/// Port of `relaySessionProviderValidation` (`src/relay-profile-editing.ts:764`).
pub fn relay_session_provider_validation(profile: Value) -> Result<Value, String> {
    let sp = relay_session_provider(profile.clone())?;
    let proto = str_prop(&profile, "protocol")?;
    if sp.as_str() == Some("openai") && proto.as_deref() != Some("responses") {
        Ok(Value::String(t(
            "OpenAI 会话身份仅支持 Responses API；Chat Completions 不支持 ChatGPT Remote 的远程压缩。请切换协议或改回 Custom。",
        )))
    } else {
        Ok(Value::Null)
    }
}

/// Port of `relayModelRouteIssueMessage` (`src/relay-profile-editing.ts:771`).
pub fn relay_model_route_issue_message(issue: Value) -> Result<Value, String> {
    if !is_truthy(&issue) {
        return Ok(Value::Null);
    }
    let kind = str_prop(&issue, "kind")?;
    let model = prop(&issue, "model")?.clone();
    Ok(match kind.as_deref() {
        Some("incomplete") => Value::String(t("单模型协议转换需要填写模型名称和目标供应商。")),
        Some("duplicate") => Value::String(tf("模型「{0}」存在重复路由。", &[model])),
        Some("self") => Value::String(tf("模型「{0}」不能路由到当前供应商自身。", &[model])),
        Some("missingTarget") => Value::String(tf("模型「{0}」的目标供应商不存在。", &[model])),
        Some("aggregateTarget") => Value::String(tf("模型「{0}」不能路由到聚合供应商。", &[model])),
        Some("targetProtocol") => Value::String(tf(
            "模型「{0}」的目标供应商必须使用 Responses API。",
            &[model],
        )),
        Some("targetCredentials") => Value::String(tf(
            "模型「{0}」的目标供应商缺少 Base URL 或 Key。",
            &[model],
        )),
        _ => js::undefined(),
    })
}

/// Port of `relaySettingsWithDraft` (`src/relay-profile-editing.ts:791`).
pub fn relay_settings_with_draft(
    settings: Value,
    profile_id: String,
    draft: Value,
    is_new: bool,
) -> Result<Value, String> {
    let normalized = if is_aggregate_profile(&draft)? {
        normalize_aggregate_relay_profile(draft, settings.clone())?
    } else {
        derive_relay_profile_from_files(draft, undefined())?
    };
    if is_new {
        add_relay_profile(settings, normalized)
    } else {
        update_relay_profile(settings, profile_id, normalized)
    }
}

/// Port of `relayProfileUsesLiveFiles` (`src/relay-profile-editing.ts:805`).
pub fn relay_profile_uses_live_files(profile: Value) -> Result<bool, String> {
    if str_prop(&profile, "relayMode")?.as_deref() != Some("official")
        || is_truthy(prop(&profile, "officialMixApiKey")?)
    {
        return Ok(true);
    }
    let cfg = req_str_field(&profile, "configContents", "trim")?;
    Ok(!es_trim(&cfg).is_empty())
}

/// Port of `authJsonHasOpenAiApiKey` (`src/relay-profile-editing.ts:809`).
pub fn auth_json_has_open_ai_api_key(contents: String) -> Result<bool, String> {
    let trimmed = es_trim(&contents);
    if trimmed.is_empty() {
        return Ok(false);
    }
    if let Ok(parsed) = serde_json::from_str::<Value>(trimmed) {
        return Ok(parsed
            .as_object()
            .and_then(|o| o.get("OPENAI_API_KEY"))
            .and_then(Value::as_str)
            .is_some_and(|s| !es_trim(s).is_empty()));
    }
    Ok(has_json_key_colon(trimmed, "OPENAI_API_KEY"))
}

/// Port of `tomlString` (`src/relay-profile-editing.ts:820`).
pub fn toml_string(value: String) -> Result<String, String> {
    Ok(value.replace('\\', "\\\\").replace('"', "\\\""))
}

// ── 列表增删改排序与聚合配置 ────────────────────────────────────────────────

/// Port of `syncLegacyRelayFields` (`src/relay-profile-editing.ts:824`).
pub fn sync_legacy_relay_fields(settings: Value) -> Result<Value, String> {
    let raw_profiles = req_arr(prop(&settings, "relayProfiles")?, "filter")?;
    let mut non_agg = Vec::new();
    let mut aggs = Vec::new();
    for p in raw_profiles {
        if is_aggregate_profile(p)? {
            aggs.push(p.clone());
        } else {
            non_agg.push(derive_relay_profile_from_files(p.clone(), undefined())?);
        }
    }
    let mut merged_for_agg = non_agg;
    merged_for_agg.extend(aggs);
    let agg_settings = spread_with(
        &settings,
        &[("relayProfiles", Value::Array(merged_for_agg))],
    );

    let mut relay_profiles = Vec::with_capacity(raw_profiles.len());
    for p in raw_profiles {
        if is_aggregate_profile(p)? {
            relay_profiles.push(normalize_aggregate_relay_profile(
                p.clone(),
                agg_settings.clone(),
            )?);
        } else {
            relay_profiles.push(derive_relay_profile_from_files(p.clone(), undefined())?);
        }
    }

    let next_settings = spread_with(
        &settings,
        &[("relayProfiles", Value::Array(relay_profiles.clone()))],
    );
    let active = active_relay_profile(next_settings)?;
    let agg_profiles =
        normalize_aggregate_profiles_from_relay_profiles(Value::Array(relay_profiles.clone()))?;
    let active_is_agg = is_aggregate_profile(&active)?;
    let active_id = prop(&active, "id")?.clone();
    let active_agg_id = if active_is_agg {
        active_id.clone()
    } else {
        Value::String(String::new())
    };
    let relay_base_url = if active_is_agg {
        Value::String(PROTOCOL_PROXY_BASE_URL.into())
    } else {
        prop(&active, "baseUrl")?.clone()
    };
    let relay_api_key = prop(&active, "apiKey")?.clone();
    Ok(spread_with(
        &settings,
        &[
            ("relayProfiles", Value::Array(relay_profiles)),
            ("activeRelayId", active_id),
            ("relayBaseUrl", relay_base_url),
            ("relayApiKey", relay_api_key),
            ("aggregateRelayProfiles", Value::Array(agg_profiles)),
            ("activeAggregateRelayId", active_agg_id),
        ],
    ))
}

/// Port of `normalizeAggregateProfilesFromRelayProfiles` (`src/relay-profile-editing.ts:849`).
pub fn normalize_aggregate_profiles_from_relay_profiles(
    profiles: Value,
) -> Result<Vec<Value>, String> {
    let list = req_arr(&profiles, "filter")?;
    let mut candidates = Vec::new();
    let mut aggregates = Vec::new();
    for p in list {
        if is_aggregate_profile(p)? {
            aggregates.push(p);
        } else {
            candidates.push(p.clone());
        }
    }
    let cand_val = Value::Array(candidates);
    let mut out = Vec::with_capacity(aggregates.len());
    for p in aggregates {
        let agg = normalize_aggregate_config(
            prop(p, "aggregate")?.clone(),
            cand_val.clone(),
            undefined(),
        )?;
        let members_arr = agg
            .get("members")
            .and_then(Value::as_array)
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        let mut member_ids = HashSet::new();
        let mut out_members = Vec::with_capacity(members_arr.len());
        for m in members_arr {
            let pid = m.get("profileId").cloned().unwrap_or_else(undefined);
            if let Some(s) = pid.as_str() {
                member_ids.insert(s.to_string());
            }
            let w = m.get("weight").and_then(as_number).unwrap_or(f64::NAN);
            out_members.push(json!({
                "relayId": pid,
                "weight": js_number(clamp_aggregate_weight(w)?),
            }));
        }
        let raw_routes = agg
            .get("routes")
            .and_then(Value::as_array)
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        let norm_routes = normalize_aggregate_routes(raw_routes, true, Some(&member_ids), true)?;
        let out_routes: Vec<Value> = norm_routes
            .into_iter()
            .map(|r| {
                json!({
                    "pattern": r.get("pattern").cloned().unwrap_or_else(undefined),
                    "relayId": r.get("profileId").cloned().unwrap_or_else(undefined),
                    "priority": r.get("priority").cloned().unwrap_or_else(undefined),
                })
            })
            .collect();
        let name = or_str(prop(p, "name")?, &t("聚合供应商"));
        let sp = normalize_relay_session_provider(prop(p, "sessionProvider")?.clone())?;
        out.push(json!({
            "id": prop(p, "id")?.clone(),
            "name": name,
            "sessionProvider": sp,
            "modelAggregationEnabled": agg.get("modelAggregationEnabled") == Some(&Value::Bool(true)),
            "strategy": agg.get("strategy").cloned().unwrap_or_else(undefined),
            "members": out_members,
            "routes": out_routes,
            "defaultRelayId": agg.get("defaultRelayId").filter(|v| !is_nil(v)).cloned().unwrap_or_else(|| Value::String(String::new())),
            "syncDefaultMemberLogin": agg.get("syncDefaultMemberLogin") == Some(&Value::Bool(true)),
            "autoReviewRelayId": agg.get("autoReviewRelayId").filter(|v| !is_nil(v)).cloned().unwrap_or_else(|| Value::String(String::new())),
        }));
    }
    Ok(out)
}

/// Port of `updateRelayProfile` (`src/relay-profile-editing.ts:875`).
pub fn update_relay_profile(settings: Value, id: String, patch: Value) -> Result<Value, String> {
    let is_agg_patch = str_prop(&patch, "relayMode")?.as_deref() == Some("aggregate")
        || is_truthy(prop(&patch, "aggregate")?);
    let profiles = req_arr(prop(&settings, "relayProfiles")?, "map")?;
    let mut updated = Vec::with_capacity(profiles.len());
    for p in profiles {
        if profile_id_str(p)? != id {
            updated.push(p.clone());
        } else {
            let merged = spread_merge(p, &patch);
            updated.push(if is_agg_patch {
                normalize_aggregate_relay_profile(merged, settings.clone())?
            } else {
                derive_relay_profile_from_files(merged, undefined())?
            });
        }
    }
    sync_legacy_relay_fields(spread_with(
        &settings,
        &[("relayProfiles", Value::Array(updated))],
    ))
}

/// Port of `addRelayProfile` (`src/relay-profile-editing.ts:988`).
pub fn add_relay_profile(settings: Value, profile: Value) -> Result<Value, String> {
    let next_with_files = if is_aggregate_profile(&profile)? {
        normalize_aggregate_relay_profile(profile, settings.clone())?
    } else {
        let has_cfg = !es_trim(&req_str_field(&profile, "configContents", "trim")?).is_empty();
        let has_auth = !es_trim(&req_str_field(&profile, "authContents", "trim")?).is_empty();
        let prepared = if has_cfg || has_auth {
            profile
        } else {
            with_generated_relay_files(profile)?
        };
        derive_relay_profile_from_files(prepared, undefined())?
    };
    let profiles = req_arr(prop(&settings, "relayProfiles")?, "some")?;
    let cur_active_id = prop(&settings, "activeRelayId")?;
    let has_active = profiles
        .iter()
        .try_fold(false, |acc, item| -> Result<bool, String> {
            Ok(acc || prop(item, "id")? == cur_active_id)
        })?;
    let active_id = if has_active {
        cur_active_id.clone()
    } else {
        let act = active_relay_profile(settings.clone())?;
        prop(&act, "id")?.clone()
    };
    let mut next_profiles = profiles.to_vec();
    next_profiles.push(next_with_files);
    sync_legacy_relay_fields(spread_with(
        &settings,
        &[
            ("relayProfiles", Value::Array(next_profiles)),
            ("activeRelayId", active_id),
        ],
    ))
}

/// Port of `reorderRelayProfiles` (`src/relay-profile-editing.ts:1022`).
pub fn reorder_relay_profiles(
    settings: Value,
    source_id: String,
    target_id: String,
) -> Result<Value, String> {
    if source_id == target_id {
        return Ok(settings);
    }
    let profiles = req_arr(prop(&settings, "relayProfiles")?, "findIndex")?;
    let mut source_idx = None;
    let mut target_idx = None;
    for (i, p) in profiles.iter().enumerate() {
        let pid = profile_id_str(p)?;
        if source_idx.is_none() && pid == source_id {
            source_idx = Some(i);
        }
        if target_idx.is_none() && pid == target_id {
            target_idx = Some(i);
        }
    }
    let (Some(si), Some(ti)) = (source_idx, target_idx) else {
        return Ok(settings);
    };
    let mut next = profiles.to_vec();
    let moved = next.remove(si);
    next.insert(ti, moved);
    sync_legacy_relay_fields(spread_with(
        &settings,
        &[("relayProfiles", Value::Array(next))],
    ))
}

/// Port of `removeRelayProfile` (`src/relay-profile-editing.ts:1036`).
pub fn remove_relay_profile(settings: Value, id: String) -> Result<Value, String> {
    if is_truthy(&relay_profile_delete_issue(settings.clone(), id.clone())?) {
        return Ok(settings);
    }
    let raw_profiles = req_arr(prop(&settings, "relayProfiles")?, "filter")?;
    let mut profiles = Vec::new();
    for p in raw_profiles {
        if profile_id_str(p)? != id {
            profiles.push(p.clone());
        }
    }
    let existing_ids: Vec<Value> = profile_id_set(&profiles)?
        .into_iter()
        .map(Value::String)
        .collect();
    let existing_set = json!({ "$set": existing_ids });
    let agg_settings = spread_with(
        &settings,
        &[("relayProfiles", Value::Array(profiles.clone()))],
    );

    let mut scrubbed = Vec::with_capacity(profiles.len());
    for p in &profiles {
        if is_aggregate_profile(p)? {
            let norm_agg = normalize_aggregate_config(
                prop(p, "aggregate")?.clone(),
                Value::Array(Vec::new()),
                existing_set.clone(),
            )?;
            let members: Vec<Value> = norm_agg
                .get("members")
                .and_then(Value::as_array)
                .map(|arr| {
                    arr.iter()
                        .filter(|m| m.get("profileId").and_then(Value::as_str) != Some(id.as_str()))
                        .cloned()
                        .collect()
                })
                .unwrap_or_default();
            let routes: Vec<Value> = norm_agg
                .get("routes")
                .and_then(Value::as_array)
                .map(|arr| {
                    arr.iter()
                        .filter(|r| r.get("profileId").and_then(Value::as_str) != Some(id.as_str()))
                        .cloned()
                        .collect()
                })
                .unwrap_or_default();
            let next_agg = spread_with(
                &norm_agg,
                &[
                    ("members", Value::Array(members)),
                    ("routes", Value::Array(routes)),
                ],
            );
            scrubbed.push(normalize_aggregate_relay_profile(
                spread_with(p, &[("aggregate", next_agg)]),
                agg_settings.clone(),
            )?);
        } else {
            let routes: Vec<Value> = normalize_relay_model_routes(prop(p, "modelRoutes")?)
                .into_iter()
                .filter(|r| r.get("targetRelayId").and_then(Value::as_str) != Some(id.as_str()))
                .collect();
            scrubbed.push(spread_with(p, &[("modelRoutes", Value::Array(routes))]));
        }
    }

    let cur_active = prop(&settings, "activeRelayId")?;
    let next_active = if cur_active.as_str() == Some(id.as_str()) {
        if let Some(first) = scrubbed.first() {
            let fid = prop(first, "id")?;
            if is_truthy(fid) {
                fid.clone()
            } else {
                Value::String("default".into())
            }
        } else {
            Value::String("default".into())
        }
    } else {
        cur_active.clone()
    };
    sync_legacy_relay_fields(spread_with(
        &settings,
        &[
            ("relayProfiles", Value::Array(scrubbed)),
            ("activeRelayId", next_active),
        ],
    ))
}

/// Port of `relayProfileDeleteIssue` (`src/relay-profile-editing.ts:1068`).
pub fn relay_profile_delete_issue(settings: Value, id: String) -> Result<Value, String> {
    if prop(&settings, "activeRelayId")?.as_str() == Some(id.as_str()) {
        return Ok(Value::String(t(
            "当前正在使用该供应商，请先切换到其他供应商后再删除。",
        )));
    }
    let profiles = req_arr(prop(&settings, "relayProfiles")?, "map")?;
    let existing_ids: Vec<Value> = profile_id_set(profiles)?
        .into_iter()
        .map(Value::String)
        .collect();
    let existing_set = json!({ "$set": existing_ids });
    let mut target_name = None;
    for p in profiles {
        if profile_id_str(p)? == id {
            let n = prop(p, "name")?;
            target_name = Some(if is_truthy(n) {
                n.clone()
            } else {
                Value::String(id.clone())
            });
            break;
        }
    }
    let target_label = target_name.unwrap_or_else(|| Value::String(id.clone()));

    for p in profiles {
        if !is_aggregate_profile(p)? {
            continue;
        }
        let agg = normalize_aggregate_config(
            prop(p, "aggregate")?.clone(),
            Value::Array(Vec::new()),
            existing_set.clone(),
        )?;
        let members = agg
            .get("members")
            .and_then(Value::as_array)
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        if !members
            .iter()
            .any(|m| m.get("profileId").and_then(Value::as_str) == Some(id.as_str()))
        {
            continue;
        }
        let agg_name = {
            let n = prop(p, "name")?;
            if is_truthy(n) {
                n.clone()
            } else {
                prop(p, "id")?.clone()
            }
        };
        if agg.get("defaultRelayId").and_then(Value::as_str) == Some(id.as_str()) {
            return Ok(Value::String(tf(
                "供应商「{0}」是聚合配置「{1}」的默认成员，请先调整聚合配置后再删除。",
                &[target_label, agg_name],
            )));
        }
        if members.len() <= 1 {
            return Ok(Value::String(tf(
                "供应商「{0}」是聚合配置「{1}」的最后成员，请先调整聚合配置后再删除。",
                &[target_label, agg_name],
            )));
        }
    }
    Ok(Value::Null)
}

/// Port of `isAggregateRelayProfile` (`src/relay-profile-editing.ts:1115`).
pub fn is_aggregate_relay_profile(profile: Value) -> Result<bool, String> {
    is_aggregate_profile(&profile)
}

/// Port of `relayProviderView` (`src/relay-profile-editing.ts:1121`).
pub fn relay_provider_view(profile: Value) -> Result<String, String> {
    if is_aggregate_profile(&profile)? {
        let model_agg = prop(&profile, "aggregate")?
            .as_object()
            .filter(|_| is_plain_object(prop(&profile, "aggregate").unwrap_or(&UNDEF)))
            .and_then(|o| o.get("modelAggregationEnabled"))
            == Some(&Value::Bool(true));
        return Ok(if model_agg {
            "modelAggregation"
        } else {
            "failover"
        }
        .into());
    }
    let proto_diff = str_prop(&profile, "protocol")?.as_deref() != Some("responses");
    let has_routes = !normalize_relay_model_routes(prop(&profile, "modelRoutes")?).is_empty();
    Ok(if proto_diff || has_routes {
        "routing"
    } else {
        "direct"
    }
    .into())
}

/// Port of `normalizeAggregateRelayProfile` (`src/relay-profile-editing.ts:1130`).
pub fn normalize_aggregate_relay_profile(profile: Value, settings: Value) -> Result<Value, String> {
    let (candidates, existing_ids) = if is_truthy(&settings) {
        let profiles = req_arr(prop(&settings, "relayProfiles")?, "map")?;
        let pid = profile_id_str(&profile)?;
        let cands = aggregate_member_candidates(profiles, &pid)?;
        let id_vals: Vec<Value> = profile_id_set(profiles)?
            .into_iter()
            .map(Value::String)
            .collect();
        (Value::Array(cands), json!({ "$set": id_vals }))
    } else {
        (Value::Array(Vec::new()), Value::Null)
    };
    let agg = normalize_aggregate_config(
        prop(&profile, "aggregate")?.clone(),
        candidates,
        existing_ids,
    )?;
    let sp = normalize_relay_session_provider(prop(&profile, "sessionProvider")?.clone())?;
    Ok(spread_with(
        &profile,
        &[
            ("baseUrl", Value::String(String::new())),
            ("upstreamBaseUrl", Value::String(String::new())),
            ("apiKey", Value::String(String::new())),
            ("protocol", Value::String("responses".into())),
            ("relayMode", Value::String("aggregate".into())),
            ("sessionProvider", sp),
            ("officialMixApiKey", Value::Bool(false)),
            ("hideOfficialUsageAlert", Value::Bool(false)),
            ("configContents", Value::String(String::new())),
            ("authContents", Value::String(String::new())),
            ("sub2apiEnabled", Value::Bool(false)),
            ("noAuth", Value::Bool(false)),
            ("sub2apiMultiplier", Value::String(String::new())),
            ("standardOpenaiProtocol", Value::Bool(false)),
            ("webSearchHistoryCompat", Value::Bool(false)),
            ("aggregate", agg),
        ],
    ))
}

/// Port of `aggregateProfileForMode` (`src/relay-profile-editing.ts:1157`).
pub fn aggregate_profile_for_mode(
    profile: Value,
    settings: Value,
    mode: Value,
) -> Result<Value, String> {
    let agg_val = prop(&profile, "aggregate")?;
    let raw_members = if is_nil(agg_val) {
        Vec::new()
    } else {
        let m = prop(agg_val, "members")?;
        if is_nil(m) {
            Vec::new()
        } else {
            req_arr(m, "map")?.to_vec()
        }
    };
    let mut members = Vec::with_capacity(raw_members.len());
    let mut member_ids = HashSet::new();
    for m in &raw_members {
        let pid = prop(m, "profileId")?.clone();
        if let Some(s) = pid.as_str() {
            member_ids.insert(s.to_string());
        }
        members.push(json!({ "profileId": pid, "weight": 1 }));
    }
    let default_relay_id = if mode.as_str() == Some("model") {
        let raw_def = if is_nil(agg_val) {
            None
        } else {
            let d = prop(agg_val, "defaultRelayId")?;
            if is_nil(d) {
                None
            } else {
                Some(req_str_field(agg_val, "defaultRelayId", "trim")?)
            }
        };
        let norm = normalize_aggregate_default_relay_id(raw_def.as_deref(), &member_ids);
        if !norm.is_empty() {
            Value::String(norm)
        } else if let Some(first) = members.first() {
            let fp = first.get("profileId").cloned().unwrap_or_else(undefined);
            if is_truthy(&fp) {
                fp
            } else {
                Value::String(String::new())
            }
        } else {
            Value::String(String::new())
        }
    } else {
        Value::String(String::new())
    };
    let next = spread_with(
        &profile,
        &[
            ("testModel", Value::String(String::new())),
            (
                "aggregate",
                json!({
                    "strategy": "failover",
                    "modelAggregationEnabled": mode.as_str() == Some("model"),
                    "members": members,
                    "routes": [],
                    "defaultRelayId": default_relay_id,
                }),
            ),
        ],
    );
    normalize_aggregate_relay_profile(next, settings)
}

/// Port of `normalizeAggregateConfig` (`src/relay-profile-editing.ts:1179`).
pub fn normalize_aggregate_config(
    aggregate: Value,
    candidates: Value,
    existing_profile_ids: Value,
) -> Result<Value, String> {
    let existing_profile_ids = if is_undefined(&existing_profile_ids) {
        Value::Null
    } else {
        existing_profile_ids
    };
    let cand_arr = req_arr(&candidates, "map")?;
    for c in cand_arr {
        let _ = prop(c, "id")?;
    }
    let existing_set = parse_existing_ids_arg(&existing_profile_ids);

    let agg_strategy = if is_nil(&aggregate) {
        None
    } else {
        str_prop(&aggregate, "strategy")?
    };
    let strategy = agg_strategy
        .as_deref()
        .filter(|s| AGGREGATE_STRATEGIES.contains(s))
        .unwrap_or("failover")
        .to_string();
    let model_agg =
        !is_nil(&aggregate) && prop(&aggregate, "modelAggregationEnabled")? == &Value::Bool(true);

    let raw_members_slice: Option<&[Value]> = if is_nil(&aggregate) {
        None
    } else {
        let m = prop(&aggregate, "members")?;
        if is_nil(m) {
            None
        } else {
            Some(req_arr(m, "Symbol.iterator")?)
        }
    };
    let has_nonempty_member = raw_members_slice.is_some_and(|ms| {
        ms.iter().any(|m| {
            let raw = m
                .get("profileId")
                .filter(|v| !is_nil(v))
                .map(js_str)
                .unwrap_or_default();
            !es_trim(&raw).is_empty()
        })
    });
    let resolved_set = match (&existing_set, has_nonempty_member) {
        (Ok(s), _) => s.as_ref(),
        (Err(e), true) => return Err(e.clone()),
        (Err(_), false) => None,
    };
    let norm_members = normalize_aggregate_members(raw_members_slice, resolved_set);
    let mut member_ids = HashSet::with_capacity(norm_members.len());
    let mut members = Vec::with_capacity(norm_members.len());
    for m in norm_members {
        let pid = m
            .get("profileId")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string();
        member_ids.insert(pid.clone());
        let w = if model_agg {
            1.0
        } else {
            let raw_w = m.get("weight").and_then(as_number).unwrap_or(f64::NAN);
            clamp_aggregate_weight(raw_w)?
        };
        members.push(json!({ "profileId": pid, "weight": js_number(w) }));
    }

    let routes = if model_agg {
        Vec::new()
    } else {
        let raw_routes = if is_nil(&aggregate) {
            &[] as &[Value]
        } else {
            let r = prop(&aggregate, "routes")?;
            if is_nil(r) {
                &[]
            } else {
                req_arr(r, "filter")?
            }
        };
        let mut out_routes = Vec::new();
        for route in raw_routes {
            let pat = req_str_field(route, "pattern", "trim")?;
            let pid_raw = req_str_field(route, "profileId", "trim")?;
            let pat_trimmed = es_trim(&pat);
            let pid_trimmed = es_trim(&pid_raw);
            if pat_trimmed.is_empty() && pid_trimmed.is_empty() {
                continue;
            }
            if !pid_trimmed.is_empty() {
                match &existing_set {
                    Ok(Some(set)) if !set.contains(pid_trimmed) => continue,
                    Ok(_) => {}
                    Err(e) => return Err(e.clone()),
                }
            }
            let prio = as_number(prop(route, "priority")?).unwrap_or(f64::NAN);
            out_routes.push(json!({
                "pattern": pat_trimmed,
                "profileId": pid_raw,
                "priority": js_number(clamp_aggregate_route_priority(prio)),
            }));
        }
        out_routes
    };

    let read_opt_str = |k: &str| -> Result<Option<String>, String> {
        if is_nil(&aggregate) {
            return Ok(None);
        }
        let v = prop(&aggregate, k)?;
        if is_nil(v) {
            return Ok(None);
        }
        Ok(Some(req_str_field(&aggregate, k, "trim")?))
    };
    let default_relay_id = normalize_aggregate_default_relay_id(
        read_opt_str("defaultRelayId")?.as_deref(),
        &member_ids,
    );
    let auto_review_relay_id = normalize_aggregate_default_relay_id(
        read_opt_str("autoReviewRelayId")?.as_deref(),
        &member_ids,
    );
    let sync_login =
        !is_nil(&aggregate) && prop(&aggregate, "syncDefaultMemberLogin")? == &Value::Bool(true);

    Ok(json!({
        "strategy": if model_agg { "failover" } else { strategy.as_str() },
        "modelAggregationEnabled": model_agg,
        "members": members,
        "routes": routes,
        "defaultRelayId": default_relay_id,
        "syncDefaultMemberLogin": sync_login,
        "autoReviewRelayId": auto_review_relay_id,
    }))
}

/// Port of `clampAggregateWeight` (`src/relay-profile-editing.ts:1234`).
pub fn clamp_aggregate_weight(value: f64) -> Result<f64, String> {
    if !value.is_finite() {
        return Ok(1.0);
    }
    Ok(math_round(value).clamp(1.0, 999.0))
}

/// Port of `aggregateStrategyLabel` (`src/relay-profile-editing.ts:1239`).
pub fn aggregate_strategy_label(strategy: Value) -> Result<String, String> {
    Ok(match strategy.as_str() {
        Some("conversationRoundRobin") => t("按对话轮转"),
        Some("requestRoundRobin") => t("按请求轮转"),
        Some("weightedRoundRobin") => t("权重轮转"),
        _ => t("失败切换"),
    })
}

/// Port of `aggregateStrategyHelp` (`src/relay-profile-editing.ts:1243`).
pub fn aggregate_strategy_help(strategy: Value) -> Result<String, String> {
    Ok(match strategy.as_str() {
        Some("failover") => t("失败切换会保留成员顺序，优先使用第一个可用供应商。"),
        Some("conversationRoundRobin") => {
            t("按对话轮转会让同一对话尽量保持固定成员，降低上下文漂移。")
        }
        Some("requestRoundRobin") => t("按请求轮转会逐请求切换成员，适合供应商能力接近的场景。"),
        _ => t("权重轮转会读取每个成员的权重值，权重越高的成员获得更多请求。"),
    })
}

/// Port of `aggregateRelayProfileValidation` (`src/relay-profile-editing.ts:1250`).
pub fn aggregate_relay_profile_validation(profile: Value) -> Result<Value, String> {
    let agg = normalize_aggregate_config(
        prop(&profile, "aggregate")?.clone(),
        Value::Array(Vec::new()),
        Value::Null,
    )?;
    let members = agg
        .get("members")
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .unwrap_or(&[]);
    if members.is_empty() {
        return Ok(Value::String(t(
            "聚合供应商至少需要勾选 1 个可用成员（API 供应商或官方登录账号）。",
        )));
    }
    let mut member_ids = HashSet::with_capacity(members.len());
    for m in members {
        if let Some(pid) = m.get("profileId").and_then(Value::as_str) {
            member_ids.insert(pid.to_string());
        }
    }
    if agg.get("modelAggregationEnabled") == Some(&Value::Bool(true)) {
        let def_id = agg
            .get("defaultRelayId")
            .and_then(Value::as_str)
            .map(es_trim)
            .unwrap_or("");
        if def_id.is_empty() || !member_ids.contains(def_id) {
            return Ok(Value::String(t(
                "模型聚合必须指定「默认供应商」：在成员列表里点某一行的「设为默认」。不带 cpx- 前缀的请求会原样交给它，没设就报错。",
            )));
        }
    }
    let routes = agg
        .get("routes")
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .unwrap_or(&[]);
    let Some(issues) = validate_aggregate_routes(routes, &member_ids)? else {
        return Ok(Value::Null);
    };
    let first = &issues[0];
    let code = first.get("code").and_then(Value::as_str).unwrap_or("");
    let pattern = first.get("pattern").cloned().unwrap_or_else(undefined);
    Ok(Value::String(match code {
        "emptyPattern" => t("路由规则的模型匹配模式不能为空。"),
        "invalidPriority" => tf(
            "路由规则「{0}」的优先级必须是大于等于 0 的整数。",
            &[pattern],
        ),
        _ => tf(
            "路由规则「{0}」的目标供应商必须是聚合成员，请先将其勾选为成员。",
            &[pattern],
        ),
    }))
}

// ── 通用 TOML / 规范化小函数 ────────────────────────────────────────────────

/// Port of `ensureTrailingNewline` (`src/relay-profile-editing.ts:1279`).
pub fn ensure_trailing_newline(value: String) -> Result<String, String> {
    Ok(if es_trim(&value).is_empty() {
        String::new()
    } else {
        format!("{value}\n")
    })
}

/// Port of `removeRootTomlKey` (`src/relay-profile-editing.ts:1283`).
pub fn remove_root_toml_key(contents: String, key: String) -> Result<String, String> {
    let source_lines = split_lines(&contents);
    let structural = toml_structure_mask_slice(&source_lines);
    let mut in_root = true;
    let mut out = Vec::with_capacity(source_lines.len());
    for (i, line) in source_lines.into_iter().enumerate() {
        if structural[i] && toml_section_name_opt(line).is_some() {
            in_root = false;
        }
        let units = u16_units(line);
        let skip = structural[i]
            && in_root
            && toml_key_assignment_units(&units)
                .is_some_and(|(path, _)| path.len() == 1 && path[0] == key);
        if !skip {
            out.push(line);
        }
    }
    ensure_trailing_newline(es_trim_end(&out.join("\n")).to_string())
}

/// Port of `joinTomlSections` (`src/relay-profile-editing.ts:1297`).
pub fn join_toml_sections(sections: Vec<String>) -> Result<String, String> {
    let joined = sections
        .iter()
        .map(|s| es_trim(s))
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("\n\n");
    ensure_trailing_newline(joined)
}

/// Port of `splitTomlRootAndTables` (`src/relay-profile-editing.ts:1306`).
pub fn split_toml_root_and_tables(section: String) -> Result<Value, String> {
    let trimmed = es_trim(&section);
    let lines = split_lines(trimmed);
    let structural = toml_structure_mask_slice(&lines);
    let first_table = lines
        .iter()
        .enumerate()
        .position(|(i, l)| structural[i] && toml_section_name_opt(l).is_some());
    Ok(match first_table {
        None => json!({ "root": lines.join("\n"), "tables": "" }),
        Some(idx) => json!({
            "root": lines[..idx].join("\n"),
            "tables": lines[idx..].join("\n"),
        }),
    })
}

/// Port of `normalizeRelaySessionProvider` (`src/relay-profile-editing.ts:1317`).
pub fn normalize_relay_session_provider(value: Value) -> Result<Value, String> {
    Ok(Value::String(
        if value.as_str() == Some("openai") {
            "openai"
        } else {
            "custom"
        }
        .into(),
    ))
}

/// Port of `relaySessionProviderFromConfig` (`src/relay-profile-editing.ts:1321`).
pub fn relay_session_provider_from_config(contents: String) -> Result<Value, String> {
    let mp = root_toml_string_value(contents, "model_provider".into())?;
    normalize_relay_session_provider(Value::String(mp))
}

/// Port of `relaySessionProvider` (`src/relay-profile-editing.ts:1325`).
pub fn relay_session_provider(profile: Value) -> Result<Value, String> {
    let cfg = req_str_field(&profile, "configContents", "trim")?;
    let from_cfg = relay_session_provider_from_config(cfg)?;
    let sp = prop(&profile, "sessionProvider")?;
    Ok(Value::String(
        if from_cfg.as_str() == Some("openai") || sp.as_str() == Some("openai") {
            "openai"
        } else {
            "custom"
        }
        .into(),
    ))
}

/// Port of `activeRelayProfile` (`src/relay-profile-editing.ts:1332`).
pub fn active_relay_profile(settings: Value) -> Result<Value, String> {
    let profiles = req_arr(prop(&settings, "relayProfiles")?, "find")?;
    let active_id = prop(&settings, "activeRelayId")?;
    for p in profiles {
        let pid = prop(p, "id")?;
        if pid == active_id && is_truthy(p) {
            return Ok(p.clone());
        }
    }
    Ok(profiles
        .first()
        .filter(|v| is_truthy(v))
        .cloned()
        .unwrap_or_else(undefined))
}

/// Port of `emptyContextSelection` (`src/relay-profile-editing.ts:1340`).
pub fn empty_context_selection() -> Result<Value, String> {
    Ok(json!({ "mcpServers": [], "skills": [], "plugins": [] }))
}

/// Port of `normalizeContextSelection` (`src/relay-profile-editing.ts:1346`).
pub fn normalize_context_selection(selection: Value, fallback: Value) -> Result<Value, String> {
    let fallback = if is_undefined(&fallback) {
        empty_context_selection()?
    } else {
        fallback
    };
    if !is_truthy(&selection) {
        return Ok(json!({
            "mcpServers": req_arr(prop(&fallback, "mcpServers")?, "Symbol.iterator")?,
            "skills": req_arr(prop(&fallback, "skills")?, "Symbol.iterator")?,
            "plugins": req_arr(prop(&fallback, "plugins")?, "Symbol.iterator")?,
        }));
    }
    let map_arr = |k: &str| -> Result<Vec<Value>, String> {
        let v = prop(&selection, k)?;
        Ok(match v.as_array() {
            Some(arr) => arr.iter().map(|x| Value::String(js_str(x))).collect(),
            None => Vec::new(),
        })
    };
    Ok(json!({
        "mcpServers": map_arr("mcpServers")?,
        "skills": map_arr("skills")?,
        "plugins": map_arr("plugins")?,
    }))
}

/// Port of `normalizeRelayMode` (`src/relay-profile-editing.ts:1364`).
pub fn normalize_relay_mode(mode: Value) -> Result<Value, String> {
    Ok(Value::String(
        match mode.as_str() {
            Some("aggregate") => "aggregate",
            Some("pureApi") => "pureApi",
            _ => "official",
        }
        .into(),
    ))
}

/// Port of `normalizeRelayProtocol` (`src/relay-profile-editing.ts:1371`).
pub fn normalize_relay_protocol(value: Value) -> Result<Value, String> {
    Ok(Value::String(
        match value.as_str() {
            Some("chatCompletions") => "chatCompletions",
            Some("anthropicMessages") => "anthropicMessages",
            _ => "responses",
        }
        .into(),
    ))
}

fn trim_lower_opt(value: Value) -> Result<String, String> {
    if is_nil(&value) {
        return Ok(String::new());
    }
    let Some(u) = string_units(&value) else {
        return Err("TypeError: (value ?? \"\").trim is not a function".into());
    };
    Ok(es_trim(&String::from_utf16_lossy(&u)).to_lowercase())
}

/// Port of `normalizeAnthropicAuthHeader` (`src/relay-profile-editing.ts:1377`).
pub fn normalize_anthropic_auth_header(value: Value) -> Result<String, String> {
    Ok(match trim_lower_opt(value)?.as_str() {
        "authorization" | "bearer" => "authorization",
        "x-api-key" | "x_api_key" | "api_key" => "x-api-key",
        _ => "",
    }
    .into())
}

/// Port of `normalizeAnthropicThinkingMode` (`src/relay-profile-editing.ts:1385`).
pub fn normalize_anthropic_thinking_mode(value: Value) -> Result<String, String> {
    Ok(match trim_lower_opt(value)?.as_str() {
        "adaptive" => "adaptive",
        "budget" => "budget",
        "off" => "off",
        _ => "",
    }
    .into())
}

/// Port of `normalizeAnthropicMaxTokens` (`src/relay-profile-editing.ts:1394`).
pub fn normalize_anthropic_max_tokens(value: Value) -> Result<f64, String> {
    let Some(n) = as_number(&value) else {
        return Ok(0.0);
    };
    if !n.is_finite() || n <= 0.0 {
        return Ok(0.0);
    }
    Ok(math_round(n).min(200_000.0))
}

/// Port of `hydrateAggregateRelayProfile` (`src/relay-profile-editing.ts:1399`).
pub fn hydrate_aggregate_relay_profile(profile: Value, aggregate: Value) -> Result<Value, String> {
    if !is_truthy(&aggregate) {
        return Ok(profile);
    }
    let name = {
        let pn = prop(&profile, "name")?;
        if is_truthy(pn) {
            pn.clone()
        } else {
            prop(&aggregate, "name")?.clone()
        }
    };
    let sp = normalize_relay_session_provider(prop(&aggregate, "sessionProvider")?.clone())?;
    let raw_members = req_arr(prop(&aggregate, "members")?, "map")?;
    let mut members = Vec::with_capacity(raw_members.len());
    for m in raw_members {
        let rid = prop(m, "relayId")?.clone();
        let w = as_number(prop(m, "weight")?).unwrap_or(f64::NAN);
        members.push(json!({
            "profileId": rid,
            "weight": js_number(clamp_aggregate_weight(w)?),
        }));
    }
    let raw_routes = prop(&aggregate, "routes")?;
    let routes = if is_nil(raw_routes) {
        Vec::new()
    } else {
        normalize_aggregate_routes(req_arr(raw_routes, "map")?, false, None, true)?
    };
    let nil_empty = |k: &str| -> Result<Value, String> {
        let v = prop(&aggregate, k)?;
        Ok(if is_nil(v) {
            Value::String(String::new())
        } else {
            v.clone()
        })
    };
    Ok(spread_with(
        &profile,
        &[
            ("name", name),
            ("relayMode", Value::String("aggregate".into())),
            ("sessionProvider", sp),
            (
                "aggregate",
                json!({
                    "strategy": prop(&aggregate, "strategy")?.clone(),
                    "modelAggregationEnabled": prop(&aggregate, "modelAggregationEnabled")? == &Value::Bool(true),
                    "members": members,
                    "routes": routes,
                    "defaultRelayId": nil_empty("defaultRelayId")?,
                    "syncDefaultMemberLogin": prop(&aggregate, "syncDefaultMemberLogin")? == &Value::Bool(true),
                    "autoReviewRelayId": nil_empty("autoReviewRelayId")?,
                }),
            ),
        ],
    ))
}

/// Port of `normalizeRelayProfile` (`src/relay-profile-editing.ts:1421`).
pub fn normalize_relay_profile(
    profile: Value,
    default_context_selection: Value,
) -> Result<Value, String> {
    let default_ctx = if is_undefined(&default_context_selection) {
        empty_context_selection()?
    } else {
        default_context_selection
    };
    let raw_mode = prop(&profile, "relayMode")?;
    let legacy_mixed = raw_mode.as_str() == Some("mixedApi");
    let ctx_selection = if is_truthy(prop(&profile, "contextSelectionInitialized")?) {
        normalize_context_selection(prop(&profile, "contextSelection")?.clone(), undefined())?
    } else {
        normalize_context_selection(undefined(), default_ctx)?
    };
    let use_common = prop(&profile, "useCommonConfig")? != &Value::Bool(false);

    if raw_mode.as_str() == Some("aggregate") || is_truthy(prop(&profile, "aggregate")?) {
        let sp = normalize_relay_session_provider(prop(&profile, "sessionProvider")?.clone())?;
        let pre = spread_with(
            &profile,
            &[
                ("model", Value::String(str_or_empty(&profile, "model")?)),
                ("baseUrl", Value::String(String::new())),
                ("upstreamBaseUrl", Value::String(String::new())),
                ("apiKey", Value::String(String::new())),
                ("protocol", Value::String("responses".into())),
                ("relayMode", Value::String("aggregate".into())),
                ("sessionProvider", sp),
                ("officialMixApiKey", Value::Bool(false)),
                ("hideOfficialUsageAlert", Value::Bool(false)),
                (
                    "testModel",
                    Value::String(str_or_empty(&profile, "testModel")?),
                ),
                ("configContents", Value::String(String::new())),
                ("authContents", Value::String(String::new())),
                ("useCommonConfig", Value::Bool(use_common)),
                ("contextSelection", ctx_selection),
                ("contextSelectionInitialized", Value::Bool(true)),
                ("contextWindow", Value::String(String::new())),
                ("autoCompactLimit", Value::String(String::new())),
                ("modelList", Value::String(String::new())),
                ("modelWindows", Value::String(String::new())),
                ("modelAutoCompact", Value::String(String::new())),
                ("modelMetadata", Value::String(String::new())),
                ("modelRoutes", Value::Array(Vec::new())),
                ("sub2apiEnabled", Value::Bool(false)),
                ("noAuth", Value::Bool(false)),
                ("sub2apiMultiplier", Value::String(String::new())),
                ("standardOpenaiProtocol", Value::Bool(false)),
                ("webSearchHistoryCompat", Value::Bool(false)),
            ],
        );
        return normalize_aggregate_relay_profile(pre, Value::Null);
    }

    let relay_mode = normalize_relay_mode(raw_mode.clone())?;
    let relay_mode_str = relay_mode.as_str().unwrap_or("official");
    let official_mix = prop(&profile, "officialMixApiKey")? == &Value::Bool(true) || legacy_mixed;
    let sp = relay_session_provider(profile.clone())?;
    let base_url = str_or_empty(&profile, "baseUrl")?;
    let up_raw = prop(&profile, "upstreamBaseUrl")?;
    let upstream_base_url = if is_truthy(up_raw) {
        js_str(up_raw)
    } else {
        base_url.clone()
    };
    let raw_auth = str_or_empty(&profile, "authContents")?;
    let auth_contents = if relay_mode_str == "official" && !official_mix {
        build_official_relay_auth_json(raw_auth)?
    } else {
        raw_auth
    };
    let model_routes = if relay_mode_str == "official" && !official_mix {
        Vec::new()
    } else {
        normalize_relay_model_routes(prop(&profile, "modelRoutes")?)
    };
    let multi_agent = match str_prop(&profile, "multiAgent")?.as_deref() {
        Some("v1") => "v1",
        Some("v2") => "v2",
        _ => "",
    };
    let custom_headers = {
        let ch = prop(&profile, "customHeaders")?;
        if is_truthy(ch) {
            ch.clone()
        } else {
            Value::Array(Vec::new())
        }
    };
    let no_auth = is_truthy(prop(&profile, "noAuth")?);
    let sub2api_enabled = !no_auth && prop(&profile, "sub2apiEnabled")? == &Value::Bool(true);
    let sub2api_multiplier = if sub2api_enabled {
        str_or_empty(&profile, "sub2apiMultiplier")?
    } else {
        String::new()
    };
    let bool_eq_true = |k: &str| -> Result<Value, String> {
        Ok(Value::Bool(prop(&profile, k)? == &Value::Bool(true)))
    };

    let normalized = spread_with(
        &profile,
        &[
            ("model", Value::String(str_or_empty(&profile, "model")?)),
            ("baseUrl", Value::String(base_url)),
            ("upstreamBaseUrl", Value::String(upstream_base_url)),
            ("apiKey", Value::String(str_or_empty(&profile, "apiKey")?)),
            (
                "protocol",
                normalize_relay_protocol(prop(&profile, "protocol")?.clone())?,
            ),
            ("relayMode", relay_mode),
            ("sessionProvider", sp),
            ("officialMixApiKey", Value::Bool(official_mix)),
            (
                "hideOfficialUsageAlert",
                bool_eq_true("hideOfficialUsageAlert")?,
            ),
            (
                "testModel",
                Value::String(str_or_empty(&profile, "testModel")?),
            ),
            (
                "configContents",
                Value::String(str_or_empty(&profile, "configContents")?),
            ),
            ("authContents", Value::String(auth_contents)),
            ("useCommonConfig", Value::Bool(use_common)),
            ("contextSelection", ctx_selection),
            ("contextSelectionInitialized", Value::Bool(true)),
            (
                "contextWindow",
                Value::String(str_or_empty(&profile, "contextWindow")?),
            ),
            (
                "autoCompactLimit",
                Value::String(str_or_empty(&profile, "autoCompactLimit")?),
            ),
            (
                "modelList",
                Value::String(str_or_empty(&profile, "modelList")?),
            ),
            (
                "modelWindows",
                Value::String(str_or_empty(&profile, "modelWindows")?),
            ),
            (
                "modelAutoCompact",
                Value::String(str_or_empty(&profile, "modelAutoCompact")?),
            ),
            (
                "modelMetadata",
                Value::String(str_or_empty(&profile, "modelMetadata")?),
            ),
            ("modelRoutes", Value::Array(model_routes)),
            (
                "userAgent",
                Value::String(str_or_empty(&profile, "userAgent")?),
            ),
            (
                "autoReviewModel",
                Value::String(opt_trim_field(&profile, "autoReviewModel")?),
            ),
            ("multiAgent", Value::String(multi_agent.into())),
            ("customHeaders", custom_headers),
            ("sub2apiEnabled", Value::Bool(sub2api_enabled)),
            ("sub2apiMultiplier", Value::String(sub2api_multiplier)),
            (
                "standardOpenaiProtocol",
                bool_eq_true("standardOpenaiProtocol")?,
            ),
            (
                "webSearchHistoryCompat",
                bool_eq_true("webSearchHistoryCompat")?,
            ),
            (
                "localCompactionPromptAtEnd",
                bool_eq_true("localCompactionPromptAtEnd")?,
            ),
            (
                "trustUpstreamCatalogLite",
                bool_eq_true("trustUpstreamCatalogLite")?,
            ),
            (
                "strictResponsesReasoning",
                bool_eq_true("strictResponsesReasoning")?,
            ),
            (
                "stripImageGenerationTool",
                bool_eq_true("stripImageGenerationTool")?,
            ),
            (
                "stripResponsesImageDetail",
                bool_eq_true("stripResponsesImageDetail")?,
            ),
            ("disableToolSearch", bool_eq_true("disableToolSearch")?),
            (
                "pureApiImageGeneration",
                bool_eq_true("pureApiImageGeneration")?,
            ),
            (
                "anthropicAuthHeader",
                Value::String(normalize_anthropic_auth_header(
                    prop(&profile, "anthropicAuthHeader")?.clone(),
                )?),
            ),
            (
                "anthropicMaxTokens",
                js_number(normalize_anthropic_max_tokens(
                    prop(&profile, "anthropicMaxTokens")?.clone(),
                )?),
            ),
            (
                "anthropicPromptCache",
                Value::Bool(prop(&profile, "anthropicPromptCache")? != &Value::Bool(false)),
            ),
            (
                "anthropicImpersonateClaudeCode",
                bool_eq_true("anthropicImpersonateClaudeCode")?,
            ),
            (
                "anthropicThinkingMode",
                Value::String(normalize_anthropic_thinking_mode(
                    prop(&profile, "anthropicThinkingMode")?.clone(),
                )?),
            ),
        ],
    );
    let derived = if relay_profile_uses_live_files(normalized.clone())? {
        derive_relay_profile_from_files(normalized, undefined())?
    } else {
        normalized
    };
    if is_truthy(prop(&derived, "noAuth")?)
        || (str_prop(&derived, "relayMode")?.as_deref() == Some("official")
            && !is_truthy(prop(&derived, "officialMixApiKey")?))
    {
        Ok(derived)
    } else {
        relay_profile_with_normalized_api_keys(&derived)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_pure_profile(id: &str) -> Value {
        json!({
            "id": id,
            "name": id.to_uppercase(),
            "model": "deepseek-v4-pro[1M]",
            "baseUrl": format!("https://{id}.example/v1"),
            "upstreamBaseUrl": format!("https://{id}.example/v1"),
            "apiKey": format!("sk-{id}-1111"),
            "protocol": "responses",
            "relayMode": "pureApi",
            "sessionProvider": "custom",
            "officialMixApiKey": false,
            "noAuth": false,
            "configContents": "",
            "authContents": "",
            "modelRoutes": [],
        })
    }

    #[test]
    fn toml_and_patch_round_trip() {
        let p = with_generated_relay_files(sample_pure_profile("relay-a")).unwrap();
        let cfg = p["configContents"].as_str().unwrap();
        assert!(cfg.contains("model = \"deepseek-v4-pro[1M]\""));
        assert!(cfg.contains("requires_openai_auth = false"));
        assert!(cfg.contains("experimental_bearer_token = \"sk-relay-a-1111\""));

        let patched = apply_relay_profile_patch_to_files(
            p,
            json!({ "model": "deepseek-v4-pro[1M]", "apiKey": "sk-new" }),
            undefined(),
        )
        .unwrap();
        let p_cfg = patched["configContents"].as_str().unwrap();
        assert!(p_cfg.contains("model = \"deepseek-v4-pro\""));
        assert!(p_cfg.contains("experimental_bearer_token = \"sk-new\""));
        assert_eq!(patched["model"], json!("deepseek-v4-pro[1M]"));
        assert_eq!(patched["apiKey"], json!("sk-new"));
    }

    #[test]
    fn dotted_provider_tables_and_validation() {
        let cfg = "model_provider = \"vendor.one\"\n\n[model_providers.vendor.one]\nexperimental_bearer_token = \"bad\"\n\n[model_providers.\"vendor.one\"]\nbase_url = \"https://active.example/v1\"\n";
        let loaded = derive_relay_profile_from_files(
            json!({
                "id": "v",
                "model": "",
                "baseUrl": "https://active.example/v1",
                "upstreamBaseUrl": "",
                "apiKey": "",
                "protocol": "responses",
                "relayMode": "pureApi",
                "noAuth": false,
                "configContents": cfg,
                "authContents": "{\"OPENAI_API_KEY\":\"good\"}",
            }),
            undefined(),
        )
        .unwrap();
        assert_eq!(loaded["apiKey"], json!("good"));
        assert_eq!(
            toml_section_name("[model_providers.\"vendor.one\"]".into()).unwrap(),
            json!("model_providers.\"vendor.one\"")
        );
        assert_eq!(
            toml_section_name("not-a-section".into()).unwrap(),
            Value::Null
        );

        let settings = json!({
            "activeRelayId": "a",
            "relayProfiles": [
                sample_pure_profile("a"),
                sample_pure_profile("b"),
                {
                    "id": "agg",
                    "name": "Agg",
                    "relayMode": "aggregate",
                    "sessionProvider": "custom",
                    "aggregate": {
                        "strategy": "failover",
                        "modelAggregationEnabled": true,
                        "members": [{ "profileId": "b", "weight": 1 }],
                        "defaultRelayId": "b",
                        "routes": []
                    }
                }
            ]
        });
        assert!(
            relay_profile_delete_issue(settings.clone(), "a".into())
                .unwrap()
                .is_string()
        );
        assert!(
            relay_profile_delete_issue(settings, "b".into())
                .unwrap()
                .is_string()
        );
    }
}
