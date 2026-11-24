//! 1:1 port of pure model catalog, metadata, reasoning, and picker-group logic from
//! `assets/inject/renderer-inject/70-model-catalog.js` and `71-model-picker-groups.js`.

#![forbid(unsafe_code)]

use fancy_regex::Regex as FancyRegex;
use indexmap::IndexSet;
use regex::Regex;
use serde_json::{Map, Value, json};
use std::sync::OnceLock;

const GROUP_NAME_MAX_UTF16_LEN: usize = 24;

fn model_ultra_pattern() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"(?i-u)sol|terra|gpt-5\.6|gpt-5\.5|gpt-5\.4").expect("ultra regex must compile")
    })
}

fn picker_suffix_pattern() -> &'static FancyRegex {
    static RE: OnceLock<FancyRegex> = OnceLock::new();
    RE.get_or_init(|| {
        FancyRegex::new(r"^(.*\S)\s*[（(]([^（()）]{1,64})[)）]$")
            .expect("picker suffix regex must compile")
    })
}

fn whitespace_run_pattern() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"\s+").expect("whitespace regex must compile"))
}

fn value_as_nonempty_str(v: &Value) -> Option<&str> {
    v.as_str().filter(|s| !s.trim().is_empty())
}

/// Deduplicates non-empty strings while preserving first-seen order (`uniqueValues`).
pub fn unique_model_values(values: &[Value]) -> Vec<String> {
    let mut seen = IndexSet::new();
    for item in values {
        if let Some(s) = value_as_nonempty_str(item) {
            seen.insert(s.to_string());
        }
    }
    seen.into_iter().collect()
}

/// Computes `codexModelSettingsSource(settings)` JSON key.
pub fn model_settings_source(settings: &Value) -> String {
    let profiles = settings
        .get("relayProfiles")
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .unwrap_or(&[]);
    let active_id = settings.get("activeRelayId");
    let active_profiles: Vec<Value> = profiles
        .iter()
        .filter(|entry| entry.get("id") == active_id)
        .map(|entry| {
            json!([
                entry.get("relayMode").cloned().unwrap_or(Value::Null),
                entry.get("protocol").cloned().unwrap_or(Value::Null),
            ])
        })
        .collect();

    json!([
        settings
            .get("relayProfilesEnabled")
            .cloned()
            .unwrap_or(Value::Null),
        settings
            .get("activeRelayId")
            .cloned()
            .unwrap_or(Value::Null),
        settings
            .get("activeAggregateRelayId")
            .cloned()
            .unwrap_or(Value::Null),
        settings
            .get("activeRelayCodexProvider")
            .cloned()
            .unwrap_or(Value::Null),
        settings
            .get("modelCatalogSourceIdentity")
            .cloned()
            .unwrap_or(Value::Null),
        active_profiles,
    ])
    .to_string()
}

fn profile_catalog_fields(entry: &Value) -> Value {
    if entry.is_null() || entry == &Value::Bool(false) {
        return entry.clone();
    }
    if !entry.is_object() {
        return Value::Null;
    }
    json!([
        entry.get("id").cloned().unwrap_or(Value::Null),
        entry.get("name").cloned().unwrap_or(Value::Null),
        entry.get("relayMode").cloned().unwrap_or(Value::Null),
        entry.get("model").cloned().unwrap_or(Value::Null),
        entry.get("modelList").cloned().unwrap_or(Value::Null),
        entry.get("modelMetadata").cloned().unwrap_or(Value::Null),
        entry.get("modelWindows").cloned().unwrap_or(Value::Null),
        entry
            .get("modelAutoCompact")
            .cloned()
            .unwrap_or(Value::Null),
        entry.get("contextWindow").cloned().unwrap_or(Value::Null),
        entry
            .get("autoCompactLimit")
            .cloned()
            .unwrap_or(Value::Null),
    ])
}

/// Computes `codexModelSettingsKey(settings)` matching `70-model-catalog.js:143`.
pub fn model_settings_key(settings: &Value) -> String {
    if let Some(fp) = settings.get("modelCatalogFingerprint").filter(|v| {
        !matches!(v, Value::Null | Value::Bool(false)) && v.as_str().is_none_or(|s| !s.is_empty())
    }) {
        return match fp {
            Value::String(s) => s.clone(),
            other => other.to_string(),
        };
    }

    let profiles = settings
        .get("relayProfiles")
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .unwrap_or(&[]);
    let active_relay_id = settings.get("activeRelayId");
    let profile = profiles
        .iter()
        .find(|entry| entry.get("id") == active_relay_id)
        .cloned()
        .unwrap_or(Value::Null);

    let aggregates = settings
        .get("aggregateRelayProfiles")
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .unwrap_or(&[]);
    let agg_target_id = settings
        .get("activeAggregateRelayId")
        .filter(|v| {
            !matches!(v, Value::Null | Value::Bool(false))
                && v.as_str().is_none_or(|s| !s.is_empty())
        })
        .or_else(|| profile.get("id"));
    let aggregate = aggregates
        .iter()
        .find(|entry| entry.get("id") == agg_target_id)
        .cloned()
        .unwrap_or(Value::Null);

    let member_fields: Vec<Value> = aggregate
        .get("members")
        .and_then(Value::as_array)
        .map(|members| {
            members
                .iter()
                .map(|member| {
                    let mid = member.get("relayId");
                    let found = profiles
                        .iter()
                        .find(|entry| entry.get("id") == mid)
                        .cloned()
                        .unwrap_or(Value::Null);
                    profile_catalog_fields(&found)
                })
                .collect()
        })
        .unwrap_or_default();

    json!([profile_catalog_fields(&profile), aggregate, member_fields]).to_string()
}

/// Computes `codexModelCatalogIdentity(settings)`.
pub fn model_catalog_identity(settings: &Value) -> String {
    json!([
        model_settings_source(settings),
        model_settings_key(settings)
    ])
    .to_string()
}

/// Returns true if `catalog` matches `profile.id` via `model_provider` or `sources`.
pub fn catalog_belongs_to_profile(catalog: &Value, profile: &Value) -> bool {
    let profile_id = profile
        .get("id")
        .and_then(Value::as_str)
        .map(str::trim)
        .unwrap_or("");
    if profile_id.is_empty() {
        return false;
    }
    if catalog.get("model_provider").and_then(Value::as_str) == Some(profile_id) {
        return true;
    }
    let expected_src = format!("relay-profile:{profile_id}");
    catalog
        .get("sources")
        .and_then(Value::as_array)
        .is_some_and(|sources| {
            sources
                .iter()
                .any(|s| s.get("id").and_then(Value::as_str) == Some(expected_src.as_str()))
        })
}

/// Resolves active effective model names (`codexPlusModelNames`) given catalog & settings state.
pub fn effective_model_names(
    catalog: &Value,
    settings: &Value,
    settings_loaded: bool,
) -> Vec<String> {
    if settings_loaded
        && settings.get("codexMultiInstance") == Some(&Value::Bool(true))
        && settings.get("relayProfilesEnabled") == Some(&Value::Bool(false))
    {
        let profiles = settings
            .get("relayProfiles")
            .and_then(Value::as_array)
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        let active_id = settings.get("activeRelayId");
        let active: Vec<&Value> = profiles
            .iter()
            .filter(|entry| entry.get("id") == active_id)
            .collect();
        let official = active.len() == 1
            && active[0].get("relayMode").and_then(Value::as_str) == Some("official");
        let status_ok = catalog.get("status").and_then(Value::as_str) == Some("ok");
        let active_profile = if active.len() == 1
            && active[0].get("relayMode").and_then(Value::as_str) == Some("pureApi")
        {
            active[0]
        } else {
            &Value::Null
        };
        if !official && (!status_ok || !catalog_belongs_to_profile(catalog, active_profile)) {
            return Vec::new();
        }
    }

    let mut inputs = Vec::new();
    inputs.push(catalog.get("default_model").cloned().unwrap_or(Value::Null));
    inputs.push(catalog.get("model").cloned().unwrap_or(Value::Null));
    if let Some(arr) = catalog.get("models").and_then(Value::as_array) {
        inputs.extend(arr.iter().cloned());
    }
    unique_model_values(&inputs)
}

/// Looks up model metadata by exact match first, then ASCII-case-insensitive match.
pub fn lookup_model_metadata(catalog: &Value, model_name: &str) -> Option<Value> {
    let metadata = catalog
        .get("modelMetadata")
        .or_else(|| catalog.get("model_metadata"))
        .and_then(Value::as_object)?;
    if let Some(exact) = metadata.get(model_name).filter(|v| v.is_object()) {
        return Some(exact.clone());
    }
    let needle = model_name.to_lowercase();
    for (k, v) in metadata {
        if k.to_lowercase() == needle && v.is_object() {
            return Some(v.clone());
        }
    }
    None
}

/// Computes `supportedReasoningEfforts` array matching `70-model-catalog.js:363-379`.
pub fn model_reasoning_efforts(catalog: &Value, model_name: &str) -> Vec<Value> {
    let meta = lookup_model_metadata(catalog, model_name);
    if let Some(supported) = meta
        .as_ref()
        .and_then(|m| m.get("supportedReasoningEfforts"))
        .and_then(Value::as_array)
        .filter(|arr| !arr.is_empty())
    {
        let mut efforts: Vec<Value> = supported.to_vec();
        let has_max = efforts
            .iter()
            .any(|e| e.get("reasoningEffort").and_then(Value::as_str) == Some("max"));
        let has_ultra = efforts
            .iter()
            .any(|e| e.get("reasoningEffort").and_then(Value::as_str) == Some("ultra"));
        if !has_max {
            efforts.push(json!({
                "reasoningEffort": "max",
                "description": "Maximum reasoning depth for the hardest problems",
            }));
        }
        if !has_ultra {
            let should_add_ultra = model_ultra_pattern().is_match(model_name);
            if should_add_ultra || efforts.len() >= 4 {
                efforts.push(json!({
                    "reasoningEffort": "ultra",
                    "description": "Maximum reasoning with automatic task delegation",
                }));
            }
        }
        return efforts;
    }

    ["low", "medium", "high", "xhigh", "max", "ultra"]
        .into_iter()
        .map(|effort| {
            json!({
                "reasoningEffort": effort,
                "description": format!("{effort} effort"),
            })
        })
        .collect()
}

/// Extracts `inputModalities` / `input_modalities` trimmed list if declared.
pub fn model_input_modalities(metadata: &Value) -> Option<Vec<String>> {
    let declared = metadata
        .get("inputModalities")
        .and_then(Value::as_array)
        .or_else(|| metadata.get("input_modalities").and_then(Value::as_array))?;
    let list: Vec<String> = declared
        .iter()
        .filter_map(Value::as_str)
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .collect();
    Some(list)
}

/// Extracts `supportsImageDetailOriginal` / `supports_image_detail_original` if boolean.
pub fn model_image_detail_original(metadata: &Value) -> Option<bool> {
    metadata
        .get("supportsImageDetailOriginal")
        .and_then(Value::as_bool)
        .or_else(|| {
            metadata
                .get("supports_image_detail_original")
                .and_then(Value::as_bool)
        })
}

/// Builds a Codex model descriptor matching `codexPlusModelDescriptor(modelName)`.
pub fn build_model_descriptor(catalog: &Value, model_name: &str) -> Value {
    let metadata = lookup_model_metadata(catalog, model_name);
    let meta_val = metadata.as_ref().unwrap_or(&Value::Null);
    let modalities = model_input_modalities(meta_val);
    let original = model_image_detail_original(meta_val);

    let display_name = meta_val
        .get("displayName")
        .and_then(value_as_nonempty_str)
        .unwrap_or(model_name);
    let description = meta_val
        .get("description")
        .and_then(value_as_nonempty_str)
        .or_else(|| catalog.get("provider_name").and_then(value_as_nonempty_str))
        .or_else(|| {
            catalog
                .get("model_provider")
                .and_then(value_as_nonempty_str)
        })
        .unwrap_or(model_name);
    let hidden = meta_val.get("hidden") == Some(&Value::Bool(true));
    let default_effort = meta_val
        .get("defaultReasoningEffort")
        .and_then(value_as_nonempty_str)
        .unwrap_or("medium");

    let mut obj = Map::new();
    obj.insert("model".into(), Value::String(model_name.to_string()));
    obj.insert("id".into(), Value::String(model_name.to_string()));
    obj.insert("slug".into(), Value::String(model_name.to_string()));
    obj.insert("name".into(), Value::String(model_name.to_string()));
    obj.insert(
        "displayName".into(),
        Value::String(display_name.to_string()),
    );
    obj.insert("description".into(), Value::String(description.to_string()));
    obj.insert("hidden".into(), Value::Bool(hidden));
    obj.insert("isDefault".into(), Value::Bool(false));
    obj.insert(
        "defaultReasoningEffort".into(),
        Value::String(default_effort.to_string()),
    );
    obj.insert(
        "supportedReasoningEfforts".into(),
        Value::Array(model_reasoning_efforts(catalog, model_name)),
    );
    if let Some(mods) = modalities {
        obj.insert(
            "inputModalities".into(),
            Value::Array(mods.into_iter().map(Value::String).collect()),
        );
    }
    if let Some(orig) = original {
        obj.insert("supportsImageDetailOriginal".into(), Value::Bool(orig));
    }
    Value::Object(obj)
}

/// Applies catalog metadata fields onto an existing descriptor (`applyCodexPlusModelMetadata`).
pub fn apply_model_metadata_to_descriptor(
    descriptor: &mut Value,
    catalog: &Value,
    model_name: &str,
) -> bool {
    let Some(metadata) = lookup_model_metadata(catalog, model_name) else {
        return false;
    };
    let Some(desc_obj) = descriptor.as_object_mut() else {
        return false;
    };
    let mut changed = false;
    for key in [
        "displayName",
        "description",
        "defaultReasoningEffort",
        "contextWindow",
        "maxContextWindow",
        "context_window",
        "max_context_window",
    ] {
        let Some(val) = metadata.get(key) else {
            continue;
        };
        let valid = match val {
            Value::String(s) => !s.is_empty(),
            Value::Number(n) => n.as_f64().is_some_and(|f| f.is_finite() && f > 0.0),
            _ => false,
        };
        if valid && desc_obj.get(key) != Some(val) {
            desc_obj.insert(key.to_string(), val.clone());
            changed = true;
        }
    }

    if let Some(modalities) = model_input_modalities(&metadata) {
        let next_mods = Value::Array(modalities.into_iter().map(Value::String).collect());
        if desc_obj.get("inputModalities") != Some(&next_mods) {
            desc_obj.insert("inputModalities".into(), next_mods);
            changed = true;
        }
    }
    if let Some(original) = model_image_detail_original(&metadata)
        && desc_obj.get("supportsImageDetailOriginal") != Some(&Value::Bool(original))
    {
        desc_obj.insert("supportsImageDetailOriginal".into(), Value::Bool(original));
        changed = true;
    }
    if metadata
        .get("supportedReasoningEfforts")
        .and_then(Value::as_array)
        .is_some_and(|arr| !arr.is_empty())
    {
        let next_efforts = Value::Array(model_reasoning_efforts(catalog, model_name));
        let cur = desc_obj
            .get("supportedReasoningEfforts")
            .cloned()
            .unwrap_or_else(|| Value::Array(Vec::new()));
        if cur != next_efforts {
            desc_obj.insert("supportedReasoningEfforts".into(), next_efforts);
            changed = true;
        }
    }

    changed
}

/// Resolves `modelResponseHostId` across candidate strings or `{ hostId / host_id }` objects.
pub fn model_response_host_id(candidates: &[Value]) -> String {
    let mut local = false;
    for value in candidates {
        let opts: Vec<Option<&str>> = match value {
            Value::String(s) => vec![Some(s.as_str())],
            Value::Object(map) => vec![
                map.get("hostId").and_then(Value::as_str),
                map.get("host_id").and_then(Value::as_str),
            ],
            _ => Vec::new(),
        };
        for cand in opts.into_iter().flatten() {
            let trimmed = cand.trim();
            if trimmed.is_empty() {
                continue;
            }
            if trimmed != "local" {
                return trimmed.to_string();
            }
            local = true;
        }
    }
    if local {
        "local".to_string()
    } else {
        String::new()
    }
}

/// Pure stateless equivalent of `patchModelDefaults` for container objects.
pub fn patch_model_defaults_pure(
    value: &mut Value,
    catalog: &Value,
    allowed: &IndexSet<String>,
) -> bool {
    let Some(obj) = value.as_object_mut() else {
        return false;
    };
    if allowed.is_empty() {
        return false;
    }
    let cat_model = catalog.get("model").and_then(Value::as_str);
    let cat_default = catalog.get("default_model").and_then(Value::as_str);
    let fallback = cat_model
        .filter(|s| allowed.contains(*s))
        .or_else(|| cat_default.filter(|s| allowed.contains(*s)))
        .or_else(|| allowed.iter().next().map(String::as_str));
    let Some(configured_default) = cat_default
        .filter(|s| allowed.contains(*s))
        .or_else(|| cat_model.filter(|s| allowed.contains(*s)))
        .or(fallback)
    else {
        return false;
    };

    let mut changed = false;
    let mut replaced = IndexSet::new();
    for key in [
        "defaultModel",
        "default_model",
        "defaultModelSlug",
        "default_model_slug",
    ] {
        let Some(original) = obj.get(key).cloned() else {
            continue;
        };
        let name = match &original {
            Value::String(s) => s.clone(),
            Value::Object(m) => m
                .get("model")
                .or_else(|| m.get("id"))
                .or_else(|| m.get("slug"))
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_string(),
            _ => String::new(),
        };
        if name.is_empty() || name == configured_default {
            continue;
        }
        replaced.insert(name);
        let patched = if original.is_string() {
            Value::String(configured_default.to_string())
        } else {
            build_model_descriptor(catalog, configured_default)
        };
        obj.insert(key.to_string(), patched);
        changed = true;
    }

    if let Some(Value::Array(versions)) = obj.get_mut("versionOptions") {
        for version in versions.iter_mut() {
            let Some(v_obj) = version.as_object_mut() else {
                continue;
            };
            for key in ["defaultModelSlug", "default_model_slug"] {
                if let Some(Value::String(cur)) = v_obj.get(key)
                    && !cur.is_empty()
                    && cur != configured_default
                {
                    v_obj.insert(
                        key.to_string(),
                        Value::String(configured_default.to_string()),
                    );
                    changed = true;
                }
            }
            if let Some(Value::Array(options)) = v_obj.get_mut("options") {
                for option in options.iter_mut() {
                    let opt_name = option
                        .as_object()
                        .and_then(|m| {
                            m.get("model")
                                .or_else(|| m.get("id"))
                                .or_else(|| m.get("slug"))
                                .and_then(Value::as_str)
                        })
                        .unwrap_or("");
                    if !opt_name.is_empty()
                        && (!allowed.contains(opt_name) || replaced.contains(opt_name))
                    {
                        *option = build_model_descriptor(catalog, configured_default);
                        changed = true;
                    }
                }
            }
        }
    }

    changed
}

/// Splits trailing `（供应商）` or `(Provider)` suffix from a picker label
/// (`71-model-picker-groups.js::codexPlusModelPickerSplitLabel`).
pub fn parse_picker_group_suffix(label: &str) -> (String, String) {
    let normalized = whitespace_run_pattern()
        .replace_all(label.trim(), " ")
        .trim()
        .to_string();
    if normalized.is_empty() {
        return (String::new(), String::new());
    }
    let Some(caps) = picker_suffix_pattern().captures(&normalized).ok().flatten() else {
        return (normalized, String::new());
    };
    let name = caps.get(1).map_or("", |m| m.as_str().trim()).to_string();
    let provider = caps.get(2).map_or("", |m| m.as_str().trim()).to_string();
    if name.is_empty()
        || provider.is_empty()
        || provider.encode_utf16().count() > GROUP_NAME_MAX_UTF16_LEN
    {
        return (normalized, String::new());
    }
    (name, provider)
}
