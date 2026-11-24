//! The Rust port of `src/model-reasoning.ts`.

#![forbid(unsafe_code)]

use manager_common_rs::{
    js_rt, model_metadata_key, parse_model_metadata_map, serialize_model_metadata_map,
};
use serde_json::{Value, json};
use std::collections::HashSet;

pub const REASONING_LEVELS: [&str; 8] = [
    "none", "minimal", "low", "medium", "high", "xhigh", "max", "ultra",
];

fn reasoning_level_description(level: &str) -> Option<&'static str> {
    match level {
        "none" => Some("Fast responses without reasoning"),
        "minimal" => Some("Minimal reasoning, fastest responses"),
        "low" => Some("Fast responses with lighter reasoning"),
        "medium" => Some("Balances speed and reasoning depth for everyday tasks"),
        "high" => Some("Greater reasoning depth for complex problems"),
        "xhigh" => Some("Extra high reasoning depth for complex problems"),
        "max" => Some("Maximum reasoning depth for the hardest problems"),
        "ultra" => Some("Maximum reasoning with automatic task delegation"),
        _ => None,
    }
}

fn plain_object(value: &Value) -> Option<&serde_json::Map<String, Value>> {
    value.as_object().filter(|_| js_rt::is_plain_object(value))
}

fn normalize_levels_inner(value: &Value) -> Vec<String> {
    let Some(items) = value.as_array() else {
        return Vec::new();
    };
    let mut picked: HashSet<&str> = HashSet::new();
    for item in items {
        let effort = match item {
            Value::String(s) => Some(s.as_str()),
            _ => plain_object(item)
                .and_then(|obj| obj.get("effort"))
                .and_then(Value::as_str),
        };
        if let Some(level) = effort.filter(|lvl| reasoning_level_description(lvl).is_some()) {
            picked.insert(level);
        }
    }
    REASONING_LEVELS
        .iter()
        .filter(|level| picked.contains(*level))
        .map(|level| (*level).to_string())
        .collect()
}

/// Port of `reasoningParameterMode` (`src/model-reasoning.ts:25`).
pub fn reasoning_parameter_mode(metadata: Option<Value>) -> Result<String, String> {
    let mode = metadata
        .as_ref()
        .and_then(plain_object)
        .and_then(|obj| obj.get("codex_plus_reasoning_parameters"))
        .and_then(Value::as_str)
        .filter(|value| matches!(*value, "omit" | "passthrough"))
        .unwrap_or("auto");
    Ok(mode.to_string())
}

/// Port of `setModelReasoningParameterMode` (`src/model-reasoning.ts:30`).
pub fn set_model_reasoning_parameter_mode(
    value: String,
    slug: String,
    mode: String,
) -> Result<String, String> {
    let mut map = parse_model_metadata_map(&value);
    let key = model_metadata_key(&slug);
    if key.is_empty() {
        return Ok(value);
    }
    let mut entry = map
        .get(&key)
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();
    if mode == "auto" {
        entry.shift_remove("codex_plus_reasoning_parameters");
    } else {
        entry.insert(
            "codex_plus_reasoning_parameters".to_string(),
            Value::String(mode),
        );
    }
    if entry.is_empty() {
        map.shift_remove(&key);
    } else {
        map.insert(key, Value::Object(entry));
    }
    Ok(serialize_model_metadata_map(&map))
}

/// Port of `normalizeReasoningLevels` (`src/model-reasoning.ts:58`).
pub fn normalize_reasoning_levels(value: Value) -> Result<Vec<String>, String> {
    Ok(normalize_levels_inner(&value))
}

/// Port of `reasoningLevelsFromMetadata` (`src/model-reasoning.ts:73`).
pub fn reasoning_levels_from_metadata(metadata: Option<Value>) -> Result<Value, String> {
    let Some(meta) = metadata.filter(js_rt::is_truthy) else {
        return Ok(Value::Null);
    };
    let Some(declared) = plain_object(&meta).and_then(|obj| obj.get("supported_reasoning_levels"))
    else {
        return Ok(Value::Null);
    };
    let levels = normalize_levels_inner(declared);
    Ok(Value::Array(
        levels.into_iter().map(Value::String).collect(),
    ))
}

/// Port of `defaultReasoningLevelFromMetadata` (`src/model-reasoning.ts:78`).
pub fn default_reasoning_level_from_metadata(metadata: Option<Value>) -> Result<Value, String> {
    let level = metadata
        .as_ref()
        .and_then(plain_object)
        .and_then(|obj| obj.get("default_reasoning_level"))
        .and_then(Value::as_str)
        .filter(|value| reasoning_level_description(value).is_some());
    Ok(level.map_or(Value::Null, |s| Value::String(s.to_string())))
}

/// Port of `formatReasoningLevelsLabel` (`src/model-reasoning.ts:87`).
pub fn format_reasoning_levels_label(levels: Vec<String>) -> Result<String, String> {
    let mut positions = Vec::with_capacity(levels.len());
    let mut ordered = true;
    for level in &levels {
        let Some(pos) = REASONING_LEVELS.iter().position(|&item| item == level) else {
            ordered = false;
            break;
        };
        if positions.last().is_some_and(|&prev| pos <= prev) {
            ordered = false;
            break;
        }
        positions.push(pos);
    }
    if !ordered {
        return Ok(levels.join(", "));
    }
    let mut parts: Vec<String> = Vec::new();
    let mut run_start = 0;
    for index in 1..=levels.len() {
        if index < levels.len() && positions[index] == positions[index - 1] + 1 {
            continue;
        }
        let run = &levels[run_start..index];
        if run.len() >= 3
            && let (Some(first), Some(last)) = (run.first(), run.last())
        {
            parts.push(format!("{first} → {last}"));
        } else {
            parts.extend(run.iter().cloned());
        }
        run_start = index;
    }
    Ok(parts.join(", "))
}

/// Port of `resolveModelReasoningLevels` (`src/model-reasoning.ts:114`).
pub fn resolve_model_reasoning_levels(
    r#override: Option<Value>,
    builtin_levels: Value,
) -> Result<Value, String> {
    let declared = reasoning_levels_from_metadata(r#override.clone())?;
    if !declared.is_null() {
        let default_level = default_reasoning_level_from_metadata(r#override)?;
        return Ok(json!({
            "levels": declared,
            "overridden": true,
            "defaultLevel": default_level,
        }));
    }
    let levels = normalize_levels_inner(&builtin_levels);
    Ok(json!({
        "levels": levels,
        "overridden": false,
        "defaultLevel": Value::Null,
    }))
}

/// Port of `setModelReasoningLevelsForSlug` (`src/model-reasoning.ts:138`).
pub fn set_model_reasoning_levels_for_slug(
    value: String,
    slug: String,
    levels: Value,
    default_level: Option<String>,
) -> Result<String, String> {
    let mut map = parse_model_metadata_map(&value);
    let key = model_metadata_key(&slug);
    if key.is_empty() {
        return Ok(serialize_model_metadata_map(&map));
    }
    let normalized = normalize_levels_inner(&levels);
    let mut entry = map
        .get(&key)
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();
    if !normalized.is_empty() {
        let items: Vec<Value> = normalized
            .iter()
            .filter_map(|effort| {
                reasoning_level_description(effort).map(|description| {
                    json!({
                        "effort": effort,
                        "description": description,
                    })
                })
            })
            .collect();
        entry.insert(
            "supported_reasoning_levels".to_string(),
            Value::Array(items),
        );
    } else {
        entry.shift_remove("supported_reasoning_levels");
    }
    if let Some(default_lvl) =
        default_level.filter(|lvl| !lvl.is_empty() && normalized.iter().any(|item| item == lvl))
    {
        entry.insert(
            "default_reasoning_level".to_string(),
            Value::String(default_lvl),
        );
    } else {
        entry.shift_remove("default_reasoning_level");
    }
    if entry.is_empty() {
        map.shift_remove(&key);
    } else {
        map.insert(key, Value::Object(entry));
    }
    Ok(serialize_model_metadata_map(&map))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn label_collapses_contiguous_runs() {
        assert_eq!(
            format_reasoning_levels_label(vec![
                "low".into(),
                "medium".into(),
                "high".into(),
                "xhigh".into(),
                "max".into(),
            ]),
            Ok("low → max".into())
        );
        assert_eq!(
            format_reasoning_levels_label(vec![
                "none".into(),
                "low".into(),
                "medium".into(),
                "high".into(),
                "xhigh".into(),
                "max".into(),
            ]),
            Ok("none, low → max".into())
        );
        assert_eq!(
            format_reasoning_levels_label(vec!["none".into(), "high".into()]),
            Ok("none, high".into())
        );
        assert_eq!(
            format_reasoning_levels_label(vec![
                "low".into(),
                "medium".into(),
                "high".into(),
                "minimal".into(),
            ]),
            Ok("low, medium, high, minimal".into())
        );
        assert_eq!(format_reasoning_levels_label(vec![]), Ok(String::new()));
    }

    #[test]
    fn normalization_and_metadata_readers() {
        assert_eq!(
            normalize_reasoning_levels(json!(["max", "low", "bogus", "low", "", "high"])),
            Ok(vec!["low".into(), "high".into(), "max".into()])
        );
        assert_eq!(
            normalize_reasoning_levels(json!([{ "effort": "xhigh" }, { "effort": "nope" }, 3])),
            Ok(vec!["xhigh".into()])
        );
        assert_eq!(normalize_reasoning_levels(js_rt::undefined()), Ok(vec![]));

        assert_eq!(
            reasoning_levels_from_metadata(Some(json!({}))),
            Ok(Value::Null)
        );
        assert_eq!(
            reasoning_levels_from_metadata(Some(js_rt::undefined())),
            Ok(Value::Null)
        );
        assert_eq!(
            reasoning_levels_from_metadata(Some(json!({ "supported_reasoning_levels": [] }))),
            Ok(json!([]))
        );
        assert_eq!(
            reasoning_levels_from_metadata(Some(json!({
                "supported_reasoning_levels": [{ "effort": "high" }, "none"]
            }))),
            Ok(json!(["none", "high"]))
        );
        assert_eq!(
            default_reasoning_level_from_metadata(Some(
                json!({ "default_reasoning_level": "high" })
            )),
            Ok(json!("high"))
        );
        assert_eq!(
            default_reasoning_level_from_metadata(Some(
                json!({ "default_reasoning_level": "bogus" })
            )),
            Ok(Value::Null)
        );
    }

    #[test]
    fn resolve_and_write_levels_and_parameter_mode() {
        let state = resolve_model_reasoning_levels(
            Some(json!({
                "supported_reasoning_levels": [{ "effort": "none" }, { "effort": "high" }],
                "default_reasoning_level": "high"
            })),
            json!(["low", "medium", "high"]),
        );
        assert_eq!(
            state,
            Ok(json!({
                "levels": ["none", "high"],
                "overridden": true,
                "defaultLevel": "high"
            }))
        );
        assert_eq!(
            resolve_model_reasoning_levels(None, json!(["low", "medium", "high"])),
            Ok(json!({
                "levels": ["low", "medium", "high"],
                "overridden": false,
                "defaultLevel": null
            }))
        );

        let written = set_model_reasoning_levels_for_slug(
            String::new(),
            "deepseek-v4-pro".into(),
            json!(["max", "low"]),
            Some("max".into()),
        )
        .unwrap_or_default();
        let parsed = parse_model_metadata_map(&written);
        assert_eq!(
            parsed.get("deepseek-v4-pro"),
            Some(&json!({
                "supported_reasoning_levels": [
                    { "effort": "low", "description": "Fast responses with lighter reasoning" },
                    { "effort": "max", "description": "Maximum reasoning depth for the hardest problems" }
                ],
                "default_reasoning_level": "max"
            }))
        );

        let cleared =
            set_model_reasoning_levels_for_slug(written, "deepseek-v4-pro".into(), json!([]), None);
        assert_eq!(cleared, Ok(String::new()));

        let omitted = set_model_reasoning_parameter_mode(
            r#"{"model":{"custom":"kept"}}"#.into(),
            "model".into(),
            "omit".into(),
        )
        .unwrap_or_default();
        assert_eq!(
            reasoning_parameter_mode(parse_model_metadata_map(&omitted).get("model").cloned()),
            Ok("omit".into())
        );
        assert_eq!(
            set_model_reasoning_parameter_mode(omitted, "model".into(), "auto".into()),
            Ok(r#"{"model":{"custom":"kept"}}"#.into())
        );
    }
}
