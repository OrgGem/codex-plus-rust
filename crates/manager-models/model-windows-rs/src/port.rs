//! The Rust port of `src/model-windows.ts`.

#![forbid(unsafe_code)]

use manager_common_rs::{
    DEFAULT_AUTO_COMPACT_PERCENT, es_trim, is_valid_auto_compact_percent, js_own_keys_order, js_rt,
    js_stringify_compact, model_slug_from_row_name, normalize_auto_compact_percent,
};
use serde_json::{Map, Value, json};
use std::collections::HashSet;

fn is_special_non_plain(o: &Map<String, Value>) -> bool {
    o.len() == 1
        && (o.contains_key("$undefined")
            || o.contains_key("$num")
            || o.contains_key("$utf16")
            || o.contains_key("$bigint")
            || o.contains_key("$set")
            || o.contains_key("$map")
            || o.contains_key("$date")
            || o.contains_key("$bytes")
            || o.contains_key("$buffer")
            || o.contains_key("$typed"))
}

fn as_array_index(key: &str) -> Option<usize> {
    if key == "0" {
        return Some(0);
    }
    if key.is_empty() || key.starts_with('0') || !key.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let n: u64 = key.parse().ok()?;
    if n < u64::from(u32::MAX) {
        usize::try_from(n).ok()
    } else {
        None
    }
}

fn is_es_ws_u16(u: u16) -> bool {
    matches!(
        u,
        0x0009..=0x000D
            | 0x0020
            | 0x00A0
            | 0x1680
            | 0x2000..=0x200A
            | 0x2028
            | 0x2029
            | 0x202F
            | 0x205F
            | 0x3000
            | 0xFEFF
    )
}

fn trim_u16(units: &[u16]) -> &[u16] {
    let start = units
        .iter()
        .position(|&u| !is_es_ws_u16(u))
        .unwrap_or(units.len());
    let end = units[start..]
        .iter()
        .rposition(|&u| !is_es_ws_u16(u))
        .map_or(start, |pos| start + pos + 1);
    &units[start..end]
}

fn require_row_obj<'a>(
    row: &'a Value,
    prop: &str,
) -> Result<Option<&'a Map<String, Value>>, String> {
    if js_rt::is_nil(row) {
        return Err(format!(
            "TypeError: Cannot read properties of null/undefined (reading '{prop}')"
        ));
    }
    Ok(row.as_object().filter(|o| !is_special_non_plain(o)))
}

fn require_string_units(val: Option<&Value>, method: &str) -> Result<Vec<u16>, String> {
    let Some(v) = val else {
        return Err(format!(
            "TypeError: Cannot read properties of undefined (reading '{method}')"
        ));
    };
    if js_rt::is_nil(v) {
        return Err(format!(
            "TypeError: Cannot read properties of null/undefined (reading '{method}')"
        ));
    }
    js_rt::string_units(v).ok_or_else(|| format!("TypeError: value.{method} is not a function"))
}

fn require_string(val: Option<&Value>, method: &str) -> Result<String, String> {
    let Some(v) = val else {
        return Err(format!(
            "TypeError: Cannot read properties of undefined (reading '{method}')"
        ));
    };
    if js_rt::is_nil(v) {
        return Err(format!(
            "TypeError: Cannot read properties of null/undefined (reading '{method}')"
        ));
    }
    match v {
        Value::String(s) => Ok(s.clone()),
        _ if js_rt::string_units(v).is_some() => {
            let units = js_rt::string_units(v).unwrap_or_default();
            Ok(String::from_utf16_lossy(&units))
        }
        _ => Err(format!("TypeError: value.{method} is not a function")),
    }
}

fn optional_string_or_empty(val: Option<&Value>, method: &str) -> Result<String, String> {
    match val {
        None => Ok(String::new()),
        Some(v) if js_rt::is_nil(v) => Ok(String::new()),
        Some(v) => require_string(Some(v), method),
    }
}

fn normalize_parsed_json(v: Value) -> Value {
    match v {
        Value::Number(n) => js_rt::js_number(n.as_f64().unwrap_or(f64::NAN)),
        Value::Array(arr) => Value::Array(arr.into_iter().map(normalize_parsed_json).collect()),
        Value::Object(map) => {
            let mut out = Map::with_capacity(map.len());
            for (k, val) in map {
                out.insert(k, normalize_parsed_json(val));
            }
            Value::Object(out)
        }
        other => other,
    }
}

fn parse_json_or_empty_obj(raw: Option<&str>) -> Option<Value> {
    let text = match raw {
        Some(s) if !s.is_empty() => s,
        _ => "{}",
    };
    let mut de = serde_json::Deserializer::from_str(text);
    de.disable_recursion_limit();
    let val = <Value as serde::Deserialize>::deserialize(&mut de).ok()?;
    de.end().ok()?;
    Some(normalize_parsed_json(val))
}

fn as_string_map(value: Option<Value>) -> Map<String, Value> {
    match value {
        Some(Value::Object(obj)) if !is_special_non_plain(&obj) => obj,
        _ => Map::new(),
    }
}

fn get_prop_owned(container: &Value, key: &str) -> Option<Value> {
    match container {
        Value::Object(obj) if !is_special_non_plain(obj) => obj.get(key).cloned(),
        Value::Array(arr) => {
            if key == "length" {
                Some(js_rt::js_number(arr.len() as f64))
            } else {
                as_array_index(key).and_then(|idx| arr.get(idx).cloned())
            }
        }
        _ => js_rt::string_units(container).and_then(|units| {
            if key == "length" {
                Some(js_rt::js_number(units.len() as f64))
            } else {
                as_array_index(key)
                    .and_then(|idx| units.get(idx).map(|&u| js_rt::utf16_string(&[u])))
            }
        }),
    }
}

fn object_entries_owned(container: &Value) -> Vec<(String, Value)> {
    if let Some(units) = js_rt::string_units(container) {
        return units
            .iter()
            .enumerate()
            .map(|(i, &u)| (i.to_string(), js_rt::utf16_string(&[u])))
            .collect();
    }
    match container {
        Value::Object(obj) if !is_special_non_plain(obj) => js_own_keys_order(obj)
            .into_iter()
            .filter_map(|k| obj.get(k).map(|v| (k.clone(), v.clone())))
            .collect(),
        Value::Array(arr) => arr
            .iter()
            .enumerate()
            .map(|(i, v)| (i.to_string(), v.clone()))
            .collect(),
        _ => Vec::new(),
    }
}

fn lookup_entry_in_value(map: &Value, model_name: &str) -> Result<Value, String> {
    let raw = es_trim(model_name);
    if raw.is_empty() {
        return Ok(js_rt::undefined());
    }
    if js_rt::is_nil(map) {
        return Err("TypeError: Cannot read properties of null/undefined".into());
    }
    let slug = model_slug_from_row_name(raw);
    let direct = match get_prop_owned(map, &slug) {
        Some(v) if !js_rt::is_nil(&v) => Some(v),
        _ => get_prop_owned(map, raw),
    };
    if let Some(val) = direct.filter(|v| !js_rt::is_undefined(v)) {
        return Ok(val);
    }
    let lower = slug.to_lowercase();
    for (key, value) in object_entries_owned(map) {
        if key.to_lowercase() == lower {
            return Ok(value);
        }
    }
    Ok(js_rt::undefined())
}

fn is_valid_window_str(value: &str) -> bool {
    let trimmed = es_trim(value);
    if trimmed.is_empty() {
        return true;
    }
    let (digits, mult) = match trimmed.as_bytes().last().copied() {
        Some(b'k' | b'K') => (&trimmed[..trimmed.len() - 1], 1_000u64),
        Some(b'm' | b'M') => (&trimmed[..trimmed.len() - 1], 1_000_000u64),
        _ => (trimmed, 1u64),
    };
    if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return false;
    }
    let Ok(base) = digits.parse::<u64>() else {
        return false;
    };
    base.checked_mul(mult).is_some_and(|tokens| tokens > 0)
}

fn merge_rows_inner(current_rows: &[Value], incoming_rows: &[Value]) -> Result<Vec<Value>, String> {
    let mut rows = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();
    let mut append = |row: &Value| -> Result<(), String> {
        let obj = require_row_obj(row, "model")?;
        let model_units = require_string_units(obj.and_then(|o| o.get("model")), "trim")?;
        let trimmed_model_units = trim_u16(&model_units);
        if trimmed_model_units.is_empty() {
            return Ok(());
        }
        let model_val = js_rt::utf16_string(trimmed_model_units);
        let model_lossy = String::from_utf16_lossy(trimmed_model_units);
        let mapped_key = model_slug_from_row_name(&model_lossy);
        let dedupe_key = if mapped_key.is_empty() {
            model_lossy
        } else {
            mapped_key
        };
        if !seen.insert(dedupe_key) {
            return Ok(());
        }
        let window_units = require_string_units(obj.and_then(|o| o.get("window")), "trim")?;
        let window_val = js_rt::utf16_string(trim_u16(&window_units));
        let auto_compact_raw =
            optional_string_or_empty(obj.and_then(|o| o.get("autoCompact")), "trim")?;
        let auto_compact = normalize_auto_compact_percent(&auto_compact_raw);
        let image_handling = match obj.and_then(|o| o.get("imageHandling")) {
            Some(v) if !js_rt::is_nil(v) => v.clone(),
            _ => Value::String("send-as-is".to_string()),
        };
        rows.push(json!({
            "model": model_val,
            "window": window_val,
            "autoCompact": auto_compact,
            "imageHandling": image_handling,
        }));
        Ok(())
    };
    for row in current_rows {
        append(row)?;
    }
    for row in incoming_rows {
        append(row)?;
    }
    Ok(rows)
}

fn js_splice_index(idx: f64, len: usize) -> usize {
    if idx.is_nan() || idx == 0.0 {
        0
    } else {
        (idx.trunc() as usize).min(len)
    }
}

/// Port of `modelWindowsMapToText` (`src/model-windows.ts:5`).
pub fn model_windows_map_to_text(
    model_list: String,
    model_windows: String,
) -> Result<String, String> {
    let Some(parsed) = parse_json_or_empty_obj(Some(&model_windows)) else {
        return Ok(String::new());
    };
    if parsed.is_null() {
        return Ok(String::new());
    }
    let lines: Vec<String> = model_list
        .split('\n')
        .map(|line| {
            let key = es_trim(line);
            match get_prop_owned(&parsed, key) {
                Some(v) if !js_rt::is_nil(&v) => js_rt::js_str(&v),
                _ => String::new(),
            }
        })
        .collect();
    Ok(lines.join("\n"))
}

/// Port of `modelWindowsTextToMap` (`src/model-windows.ts:18`).
pub fn model_windows_text_to_map(
    model_list: String,
    model_windows_text: String,
) -> Result<String, String> {
    let models: Vec<&str> = model_list
        .split('\n')
        .map(es_trim)
        .filter(|s| !s.is_empty())
        .collect();
    let windows: Vec<&str> = model_windows_text.split('\n').map(es_trim).collect();
    let mut map = Map::new();
    for (index, model) in models.into_iter().enumerate() {
        if let Some(&win) = windows.get(index).filter(|w| !w.is_empty()) {
            map.insert(model.to_string(), Value::String(win.to_string()));
        }
    }
    Ok(js_stringify_compact(&Value::Object(map)))
}

/// Port of `reorderModelWindowRows` (`src/model-windows.ts:48`).
pub fn reorder_model_window_rows(
    rows: Value,
    active_index: f64,
    over_index: f64,
) -> Result<Value, String> {
    if active_index < 0.0 {
        return Ok(rows);
    }
    if js_rt::is_nil(&rows) {
        return Err(
            "TypeError: Cannot read properties of null/undefined (reading 'length')".into(),
        );
    }
    let len_opt = match &rows {
        Value::Array(arr) => Some(arr.len() as f64),
        _ => js_rt::string_units(&rows).map(|u| u.len() as f64),
    };
    if len_opt.is_some_and(|len| active_index >= len)
        || over_index < 0.0
        || len_opt.is_some_and(|len| over_index >= len)
        || active_index == over_index
    {
        return Ok(rows);
    }
    let mut next = match rows {
        Value::Array(arr) => arr,
        _ => {
            let Some(units) = js_rt::string_units(&rows) else {
                return Err("TypeError: rows is not iterable".into());
            };
            let mut chars = Vec::new();
            let mut i = 0;
            while i < units.len() {
                let u = units[i];
                if (0xD800..=0xDBFF).contains(&u)
                    && i + 1 < units.len()
                    && (0xDC00..=0xDFFF).contains(&units[i + 1])
                {
                    chars.push(js_rt::utf16_string(&units[i..i + 2]));
                    i += 2;
                } else {
                    chars.push(js_rt::utf16_string(&[u]));
                    i += 1;
                }
            }
            chars
        }
    };
    let from = js_splice_index(active_index, next.len());
    let moved = next.remove(from);
    let to = js_splice_index(over_index, next.len());
    next.insert(to, moved);
    Ok(Value::Array(next))
}

/// Port of `modelMapKeyFromRowName` (`src/model-windows.ts:80`).
pub fn model_map_key_from_row_name(row_name: String) -> Result<String, String> {
    Ok(model_slug_from_row_name(es_trim(&row_name)))
}

/// Port of `lookupModelMapEntry` (`src/model-windows.ts:88`).
pub fn lookup_model_map_entry(map: Value, model_name: String) -> Result<Value, String> {
    lookup_entry_in_value(&map, &model_name)
}

/// Port of `isValidModelWindow` (`src/model-windows.ts:104`).
pub fn is_valid_model_window(value: String) -> Result<bool, String> {
    Ok(is_valid_window_str(&value))
}

/// Port of `modelWindowRowsValidationError` (`src/model-windows.ts:118`).
pub fn model_window_rows_validation_error(rows: Vec<Value>) -> Result<Value, String> {
    let mut seen: HashSet<String> = HashSet::new();
    for row in &rows {
        let obj = require_row_obj(row, "model")?;
        let model_raw = require_string(obj.and_then(|o| o.get("model")), "trim")?;
        let model = es_trim(&model_raw);
        if model.is_empty() {
            continue;
        }
        let mapped_key = model_slug_from_row_name(model);
        let dedupe_key = if mapped_key.is_empty() {
            model.to_string()
        } else {
            mapped_key
        };
        if !seen.insert(dedupe_key) {
            return Ok(json!({ "code": "duplicateModel", "model": model }));
        }
        let window = require_string(obj.and_then(|o| o.get("window")), "trim")?;
        if !is_valid_window_str(&window) {
            return Ok(json!({ "code": "invalidWindow", "model": model }));
        }
        let auto_compact =
            optional_string_or_empty(obj.and_then(|o| o.get("autoCompact")), "trim")?;
        if !is_valid_auto_compact_percent(&auto_compact) {
            return Ok(json!({ "code": "invalidAutoCompact", "model": model }));
        }
    }
    Ok(Value::Null)
}

/// Port of `mergeModelWindowRows` (`src/model-windows.ts:136`).
pub fn merge_model_window_rows(
    current_rows: Vec<Value>,
    incoming_rows: Vec<Value>,
) -> Result<Vec<Value>, String> {
    merge_rows_inner(&current_rows, &incoming_rows)
}

/// Port of `modelWindowRowsFromProfile` (`src/model-windows.ts:163`).
pub fn model_window_rows_from_profile(
    model_list: String,
    model_windows: String,
    model_vlm: Option<String>,
    model_auto_compact: Option<String>,
) -> Result<Vec<Value>, String> {
    let map = Value::Object(as_string_map(parse_json_or_empty_obj(Some(&model_windows))));
    let auto_compact_map = Value::Object(as_string_map(parse_json_or_empty_obj(
        model_auto_compact.as_deref(),
    )));
    let mut vlm_map = Map::new();
    if let Some(raw_vlm) = parse_json_or_empty_obj(model_vlm.as_deref())
        && !raw_vlm.is_null()
    {
        for (model, value) in object_entries_owned(&raw_vlm) {
            let key = model_slug_from_row_name(es_trim(&model));
            if key.is_empty() {
                continue;
            }
            if value.as_str() == Some("vlm") {
                vlm_map.insert(key, Value::String("vlm".to_string()));
            } else if value.as_str() == Some("strip") {
                vlm_map.insert(key, Value::String("strip".to_string()));
            }
        }
    }
    let vlm_value = Value::Object(vlm_map);

    let mut rows = Vec::new();
    for raw_model in model_list.split('\n') {
        let model = es_trim(raw_model);
        if model.is_empty() {
            continue;
        }
        let window_val = lookup_entry_in_value(&map, model)?;
        let window = match window_val {
            Value::String(s) => s,
            _ if js_rt::string_units(&window_val).is_some() => {
                let units = js_rt::string_units(&window_val).unwrap_or_default();
                String::from_utf16_lossy(&units)
            }
            _ => String::new(),
        };
        let auto_compact_val = lookup_entry_in_value(&auto_compact_map, model)?;
        let auto_compact_src = match &auto_compact_val {
            Value::String(s) => s.as_str(),
            _ => DEFAULT_AUTO_COMPACT_PERCENT,
        };
        let auto_compact = normalize_auto_compact_percent(auto_compact_src);
        let vlm_entry = lookup_entry_in_value(&vlm_value, model)?;
        let image_handling = if js_rt::is_nil(&vlm_entry) {
            Value::String("send-as-is".to_string())
        } else {
            vlm_entry
        };
        rows.push(json!({
            "model": model,
            "window": window,
            "autoCompact": auto_compact,
            "imageHandling": image_handling,
        }));
    }
    Ok(rows)
}

/// Port of `serializeModelWindowRows` (`src/model-windows.ts:220`).
pub fn serialize_model_window_rows(rows: Vec<Value>) -> Result<Value, String> {
    let merged = merge_rows_inner(&rows, &[])?;
    let mut model_list: Vec<Value> = Vec::new();
    let mut model_windows = Map::new();
    let mut model_vlm = Map::new();
    let mut model_auto_compact = Map::new();

    for row in &merged {
        let Some(obj) = row.as_object() else {
            continue;
        };
        let Some(model_val) = obj.get("model") else {
            continue;
        };
        let Some(model_units) = js_rt::string_units(model_val) else {
            continue;
        };
        let trimmed_units = trim_u16(&model_units);
        if trimmed_units.is_empty() {
            continue;
        }
        let model_lossy = String::from_utf16_lossy(trimmed_units);
        let mapped_key = model_slug_from_row_name(&model_lossy);
        let key = if mapped_key.is_empty() {
            model_lossy
        } else {
            mapped_key
        };
        model_list.push(js_rt::utf16_string(trimmed_units));

        if let Some(win_val) = obj.get("window")
            && let Some(win_units) = js_rt::string_units(win_val)
        {
            let trimmed_win = trim_u16(&win_units);
            if !trimmed_win.is_empty() {
                model_windows.insert(key.clone(), js_rt::utf16_string(trimmed_win));
            }
        }
        if let Some(ih) = obj.get("imageHandling").and_then(Value::as_str)
            && (ih == "vlm" || ih == "strip")
        {
            model_vlm.insert(key.clone(), Value::String(ih.to_string()));
        }
        let ac_src = obj
            .get("autoCompact")
            .and_then(Value::as_str)
            .map(es_trim)
            .filter(|s| !s.is_empty())
            .unwrap_or(DEFAULT_AUTO_COMPACT_PERCENT);
        model_auto_compact.insert(key, Value::String(normalize_auto_compact_percent(ac_src)));
    }

    let model_list_joined = model_list
        .iter()
        .map(js_rt::js_str)
        .collect::<Vec<_>>()
        .join("\n");

    Ok(json!({
        "modelList": model_list_joined,
        "modelWindows": js_stringify_compact(&Value::Object(model_windows)),
        "modelVlm": js_stringify_compact(&Value::Object(model_vlm)),
        "modelAutoCompact": js_stringify_compact(&Value::Object(model_auto_compact)),
    }))
}

/// Port of `buildModelWindows` (`src/model-windows.ts:264`).
pub fn build_model_windows(
    model_list: String,
    model_windows_text: String,
) -> Result<Value, String> {
    let models_count = model_list
        .split('\n')
        .map(es_trim)
        .filter(|s| !s.is_empty())
        .count();
    let windows_count = model_windows_text.split('\n').count();
    if models_count != windows_count {
        return Ok(json!({
            "ok": false,
            "error": format!(
                "模型名称有 {models_count} 行，上下文窗口有 {windows_count} 行，请保持行数一致。"
            ),
        }));
    }
    let model_windows = model_windows_text_to_map(model_list, model_windows_text)?;
    Ok(json!({
        "ok": true,
        "modelWindows": model_windows,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_and_text_conversions() {
        assert_eq!(
            model_windows_map_to_text("a\nb\nc".into(), r#"{"a":"1M","c":"200K"}"#.into()),
            Ok("1M\n\n200K".into())
        );
        assert_eq!(
            model_windows_map_to_text("a\nb".into(), "not-json".into()),
            Ok(String::new())
        );
        assert_eq!(
            model_windows_text_to_map("a\nb\nc".into(), "1M\n\n200K".into()),
            Ok(r#"{"a":"1M","c":"200K"}"#.into())
        );
        assert_eq!(
            build_model_windows("deepseek-v4-flash\ndeepseek-v4-pro".into(), "1M\n".into()),
            Ok(json!({
                "ok": true,
                "modelWindows": r#"{"deepseek-v4-flash":"1M"}"#
            }))
        );
        let mismatch = build_model_windows("a\nb".into(), "1M".into()).unwrap_or(Value::Null);
        assert_eq!(mismatch["ok"], json!(false));
        assert!(
            mismatch["error"]
                .as_str()
                .is_some_and(|e| e.contains('2') && e.contains('1'))
        );
    }

    #[test]
    fn rows_profile_validation_merge_and_reorder() {
        assert_eq!(
            model_window_rows_from_profile(
                "a\nb".into(),
                r#"{"a":1048576,"b":"512K"}"#.into(),
                Some(r#"{"a":true,"b":"strip"}"#.into()),
                Some(r#"{"a":90,"b":"80%"}"#.into()),
            ),
            Ok(vec![
                json!({ "model": "a", "window": "", "autoCompact": "90%", "imageHandling": "send-as-is" }),
                json!({ "model": "b", "window": "512K", "autoCompact": "80%", "imageHandling": "strip" }),
            ])
        );
        assert_eq!(
            model_window_rows_from_profile("".into(), "{}".into(), None, None),
            Ok(vec![])
        );

        let rows = json!([
            { "sortId": "1", "model": "a", "window": "1M", "autoCompact": "90%", "imageHandling": "send-as-is" },
            { "sortId": "2", "model": "b", "window": "", "autoCompact": "", "imageHandling": "strip" },
        ]);
        let reordered = reorder_model_window_rows(rows, 0.0, 1.0).unwrap_or(Value::Null);
        assert_eq!(reordered[0]["sortId"], json!("2"));
        assert_eq!(reordered[1]["sortId"], json!("1"));
        assert_eq!(
            reorder_model_window_rows(json!(true), 0.0, 0.0),
            Ok(json!(true))
        );
        assert_eq!(
            reorder_model_window_rows(json!("x"), 100.0, 0.0),
            Ok(json!("x"))
        );

        let saved = serialize_model_window_rows(vec![json!({
            "model": "GLM-5.3[1M]",
            "window": "200K",
            "autoCompact": "80%",
            "imageHandling": "vlm"
        })]);
        assert_eq!(
            saved,
            Ok(json!({
                "modelList": "GLM-5.3[1M]",
                "modelWindows": r#"{"GLM-5.3":"200K"}"#,
                "modelVlm": r#"{"GLM-5.3":"vlm"}"#,
                "modelAutoCompact": r#"{"GLM-5.3":"80%"}"#,
            }))
        );

        assert_eq!(
            model_window_rows_validation_error(vec![
                json!({ "model": "deepseek-v4-pro[1M]", "window": "", "autoCompact": "90%", "imageHandling": "send-as-is" }),
                json!({ "model": "deepseek-v4-pro", "window": "", "autoCompact": "90%", "imageHandling": "send-as-is" }),
            ]),
            Ok(json!({ "code": "duplicateModel", "model": "deepseek-v4-pro" }))
        );
        assert_eq!(
            model_window_rows_validation_error(vec![
                json!({ "model": "m1", "window": "18446744073709551615", "autoCompact": "90%", "imageHandling": "send-as-is" }),
            ]),
            Ok(Value::Null)
        );
        assert_eq!(
            is_valid_model_window("18446744073709551615".into()),
            Ok(true)
        );
        assert_eq!(
            is_valid_model_window("18446744073709551616".into()),
            Ok(false)
        );
        assert_eq!(
            lookup_model_map_entry(json!({ "a": 1 }), "missing".into()),
            Ok(js_rt::undefined())
        );
        assert_eq!(
            lookup_model_map_entry(json!({ "glm-5.3": "vlm" }), "GLM-5.3[1M]".into()),
            Ok(json!("vlm"))
        );
    }
}
