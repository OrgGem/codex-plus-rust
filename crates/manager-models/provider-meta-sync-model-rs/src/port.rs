//! The Rust port of `src/provider-meta-sync-model.ts`.

#![forbid(unsafe_code)]

use crate::js;
use manager_common_rs::gateway_shared::format_tokens;
use manager_common_rs::{is_nil, is_plain_object, is_truthy, is_undefined, js_str, to_number};
use serde_json::{Value, json};

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

fn req_arr<'a>(v: &'a Value, method: &str) -> Result<&'a [Value], String> {
    if is_nil(v) {
        return Err(nil_err(method));
    }
    v.as_array()
        .map(Vec::as_slice)
        .ok_or_else(|| format!("TypeError: value.{method} is not a function"))
}

fn nullish_or(v: Option<&Value>, fallback: Value) -> Value {
    match v {
        Some(x) if !is_nil(x) => x.clone(),
        _ => fallback,
    }
}

fn format_levels_values(
    levels: &[Value],
    default_level: &Value,
    default_label: &Value,
) -> Result<String, String> {
    if levels.is_empty() {
        return Ok(String::new());
    }
    let joined = levels.iter().map(js_str).collect::<Vec<_>>().join(", ");
    if is_truthy(default_level) {
        let label = if is_undefined(default_label) {
            "默认".to_string()
        } else {
            js_str(default_label)
        };
        Ok(format!("{joined} · {label} {}", js_str(default_level)))
    } else {
        Ok(joined)
    }
}

fn set_has_value(picked: &Value, needle: &Value) -> Result<bool, String> {
    if is_nil(picked) {
        return Err(nil_err("has"));
    }
    let items = picked
        .as_object()
        .filter(|o| o.len() == 1)
        .and_then(|o| o.get("$set"))
        .and_then(Value::as_array)
        .ok_or("TypeError: picked.has is not a function")?;
    Ok(items
        .iter()
        .any(|x| match (js::as_number(x), js::as_number(needle)) {
            (Some(a), Some(b)) => (a.is_nan() && b.is_nan()) || a == b,
            _ => (is_undefined(x) && is_undefined(needle)) || x == needle,
        }))
}

/// Port of `normalizePreview` (`src/provider-meta-sync-model.ts:65`).
pub fn normalize_preview(raw: Value) -> Result<Value, String> {
    let map = raw.as_object().filter(|_| is_plain_object(&raw));
    let field = |k: &str, def: Value| nullish_or(map.and_then(|m| m.get(k)), def);
    Ok(json!({
        "upstreamModels": field("upstreamModels", json!(0)),
        "matched": field("matched", json!(0)),
        "withWindow": field("withWindow", json!(0)),
        "withReasoning": field("withReasoning", json!(0)),
        "windows": field("windows", json!([])),
        "reasoning": field("reasoning", json!([])),
        "ignored": field("ignored", json!([])),
    }))
}

/// Port of `formatLevels` (`src/provider-meta-sync-model.ts:78`).
pub fn format_levels(
    levels: Vec<String>,
    default_level: Value,
    default_label: Value,
) -> Result<String, String> {
    let vals: Vec<Value> = levels.into_iter().map(Value::String).collect();
    format_levels_values(&vals, &default_level, &default_label)
}

/// Port of `changeRows` (`src/provider-meta-sync-model.ts:84`).
pub fn change_rows(
    preview: Value,
    kind: String,
    default_label: Value,
) -> Result<Vec<Value>, String> {
    if kind == "windows" {
        let windows = req_arr(prop(&preview, "windows")?, "map")?;
        let mut out = Vec::with_capacity(windows.len());
        for change in windows {
            if is_nil(change) {
                return Err(nil_err("model"));
            }
            let next = format_tokens(to_number(prop(change, "window")?))?;
            let current_v = prop(change, "current")?;
            let current = if is_truthy(current_v) {
                format_tokens(to_number(current_v))?
            } else {
                String::new()
            };
            let opts = req_arr(prop(change, "options")?, "map")?;
            let options = if opts.len() > 1 {
                opts.iter()
                    .map(|o| format_tokens(to_number(o)))
                    .collect::<Result<Vec<_>, _>>()?
                    .join(" / ")
            } else {
                String::new()
            };
            out.push(json!({
                "model": prop(change, "model")?,
                "next": next,
                "current": current,
                "currentSource": prop(change, "currentSource")?,
                "source": prop(change, "source")?,
                "manual": prop(change, "manual")?,
                "options": options,
            }));
        }
        return Ok(out);
    }

    let reasoning = req_arr(prop(&preview, "reasoning")?, "map")?;
    let mut out = Vec::with_capacity(reasoning.len());
    for change in reasoning {
        if is_nil(change) {
            return Err(nil_err("model"));
        }
        let levels = req_arr(prop(change, "levels")?, "join")?;
        let next = format_levels_values(levels, prop(change, "defaultLevel")?, &default_label)?;
        let curr_levels = req_arr(prop(change, "currentLevels")?, "join")?;
        let current =
            format_levels_values(curr_levels, prop(change, "currentDefault")?, &default_label)?;
        let src = prop(change, "source")?;
        let def_src = prop(change, "defaultSource")?;
        let source = if is_truthy(def_src) {
            Value::String(format!("{} · {}", js_str(src), js_str(def_src)))
        } else {
            src.clone()
        };
        out.push(json!({
            "model": prop(change, "model")?,
            "next": next,
            "current": current,
            "currentSource": prop(change, "currentSource")?,
            "source": source,
            "manual": prop(change, "manual")?,
            "options": "",
        }));
    }
    Ok(out)
}

/// Port of `windowPicks` (`src/provider-meta-sync-model.ts:114`).
pub fn window_picks(preview: Value, picked: Value) -> Result<Vec<Value>, String> {
    let windows = req_arr(prop(&preview, "windows")?, "filter")?;
    let mut out = Vec::new();
    for change in windows {
        if is_nil(change) {
            return Err(nil_err("model"));
        }
        let model = prop(change, "model")?;
        if set_has_value(&picked, model)? {
            out.push(json!({
                "model": model,
                "window": prop(change, "window")?,
            }));
        }
    }
    Ok(out)
}

/// Port of `reasoningPicks` (`src/provider-meta-sync-model.ts:119`).
pub fn reasoning_picks(preview: Value, picked: Value) -> Result<Vec<Value>, String> {
    let reasoning = req_arr(prop(&preview, "reasoning")?, "filter")?;
    let mut out = Vec::new();
    for change in reasoning {
        if is_nil(change) {
            return Err(nil_err("model"));
        }
        let model = prop(change, "model")?;
        if set_has_value(&picked, model)? {
            out.push(json!({
                "model": model,
                "levels": prop(change, "levels")?,
                "defaultLevel": prop(change, "defaultLevel")?,
            }));
        }
    }
    Ok(out)
}

/// Port of `ignoredFor` (`src/provider-meta-sync-model.ts:129`).
pub fn ignored_for(preview: Value, kind: String) -> Result<Vec<Value>, String> {
    let ignored = req_arr(prop(&preview, "ignored")?, "filter")?;
    let mut out = Vec::new();
    for note in ignored {
        if is_nil(note) {
            return Err(nil_err("reason"));
        }
        let reason = prop(note, "reason")?.as_str();
        let is_reasoning = matches!(reason, Some("unknownLevel" | "defaultNotInLevels"))
            || (reason == Some("invalidType") && {
                let field_lo = js_str(prop(note, "field")?).to_ascii_lowercase();
                field_lo.contains("reason") || field_lo.contains("effort")
            });
        if (kind == "reasoning") == is_reasoning {
            out.push(note.clone());
        }
    }
    Ok(out)
}

/// Port of `currentSourceLabel` (`src/provider-meta-sync-model.ts:138`).
pub fn current_source_label(source: String) -> Result<String, String> {
    Ok(match source.as_str() {
        "custom" => "手动设置",
        "synced" => "上次同步",
        "providerDefault" => "供应商默认",
        "builtin" => "内置资料",
        _ => "未设置",
    }
    .into())
}

/// Port of `ignoredReasonLabel` (`src/provider-meta-sync-model.ts:154`).
pub fn ignored_reason_label(reason: String) -> Result<String, String> {
    Ok(match reason.as_str() {
        "invalidWindow" => "窗口数值不合理",
        "unknownLevel" => "不是 Codex 支持的档位",
        "defaultNotInLevels" => "默认档不在支持的档位里",
        _ => "字段类型不对",
    }
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_preview() -> Value {
        normalize_preview(json!({
            "upstreamModels": 3,
            "matched": 3,
            "withWindow": 2,
            "withReasoning": 2,
            "windows": [
                { "model": "gpt-x", "window": 1_000_000, "source": "context_length", "options": [300_000, 600_000, 1_000_000], "current": 272_000, "currentSource": "builtin", "manual": false },
                { "model": "mine", "window": 400_000, "source": "max_input_tokens", "options": [], "current": 128_000, "currentSource": "custom", "manual": true }
            ],
            "reasoning": [
                { "model": "gpt-x", "levels": ["low", "high", "max"], "defaultLevel": "high", "source": "reasoning_efforts", "defaultSource": "reasoning_default_effort", "currentLevels": ["low", "medium", "high"], "currentDefault": null, "currentSource": "builtin", "manual": false },
                { "model": "mine", "levels": ["low", "high"], "defaultLevel": null, "source": "reasoning_efforts", "defaultSource": "", "currentLevels": ["high"], "currentDefault": "high", "currentSource": "custom", "manual": true }
            ],
            "ignored": [
                { "model": "gpt-x", "field": "reasoning_efforts", "value": "turbo", "reason": "unknownLevel" },
                { "model": "gpt-x", "field": "max_context_tokens", "value": "0", "reason": "invalidWindow" },
                { "model": "mine", "field": "reasoning.efforts", "value": "{}", "reason": "invalidType" }
            ]
        }))
        .unwrap()
    }

    #[test]
    fn meta_sync_rows_picks_and_labels() {
        let preview = sample_preview();
        let win_rows = change_rows(preview.clone(), "windows".into(), js::undefined()).unwrap();
        assert_eq!(win_rows[0]["next"], json!("1M"));
        assert_eq!(win_rows[0]["current"], json!("272K"));
        assert_eq!(win_rows[0]["options"], json!("300K / 600K / 1M"));

        let reas_rows = change_rows(preview.clone(), "reasoning".into(), js::undefined()).unwrap();
        assert_eq!(reas_rows[0]["next"], json!("low, high, max · 默认 high"));
        assert_eq!(reas_rows[0]["current"], json!("low, medium, high"));
        assert_eq!(
            reas_rows[0]["source"],
            json!("reasoning_efforts · reasoning_default_effort")
        );
        assert_eq!(reas_rows[1]["current"], json!("high · 默认 high"));

        assert_eq!(
            window_picks(preview.clone(), json!({ "$set": ["gpt-x"] })).unwrap(),
            vec![json!({ "model": "gpt-x", "window": 1_000_000 })]
        );
        assert_eq!(
            reasoning_picks(preview.clone(), json!({ "$set": ["mine"] })).unwrap(),
            vec![json!({ "model": "mine", "levels": ["low", "high"], "defaultLevel": null })]
        );
        assert_eq!(
            ignored_for(preview.clone(), "windows".into()).unwrap()[0]["field"],
            json!("max_context_tokens")
        );
        assert_eq!(ignored_for(preview, "reasoning".into()).unwrap().len(), 2);
        assert_eq!(
            normalize_preview(js::undefined()).unwrap()["windows"],
            json!([])
        );
        assert_eq!(current_source_label("custom".into()).unwrap(), "手动设置");
        assert_eq!(
            ignored_reason_label("invalidWindow".into()).unwrap(),
            "窗口数值不合理"
        );
    }
}
