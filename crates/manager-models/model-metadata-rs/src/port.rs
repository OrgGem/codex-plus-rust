//! The Rust port of `src/model-metadata.ts`.

#![forbid(unsafe_code)]

use manager_common_rs::{DEFAULT_AUTO_COMPACT_PERCENT, js_rt, normalize_auto_compact_percent};
use serde_json::{Map, Value, json};
use std::collections::HashSet;

const MAX_SAFE_INT_U128: u128 = 9_007_199_254_740_991;
const MAX_SAFE_INT_F64: f64 = 9_007_199_254_740_991.0;
const PERCENT_SCALE: u128 = 1_000_000;
const SCALED_PERCENT_DENOM: u128 = 100 * PERCENT_SCALE;

fn is_es_ws(c: char) -> bool {
    c != '\u{0085}' && js_rt::is_js_ws(c)
}

fn es_trim(s: &str) -> &str {
    s.trim_matches(is_es_ws)
}

fn is_line_terminator(c: char) -> bool {
    matches!(c, '\n' | '\r' | '\u{2028}' | '\u{2029}')
}

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
            || o.contains_key("$buffer"))
}

fn is_plain_record(v: &Value) -> bool {
    v.as_object().is_some_and(|o| !is_special_non_plain(o))
}

fn as_array_index(key: &str) -> Option<u32> {
    if key == "0" {
        return Some(0);
    }
    if key.is_empty() || key.starts_with('0') || !key.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let n: u64 = key.parse().ok()?;
    if n < u64::from(u32::MAX) {
        u32::try_from(n).ok()
    } else {
        None
    }
}

fn js_own_keys_order<'a>(map: &'a Map<String, Value>) -> Vec<&'a String> {
    let mut indices: Vec<(u32, &'a String)> = Vec::new();
    let mut rest: Vec<&'a String> = Vec::new();
    for k in map.keys() {
        if let Some(idx) = as_array_index(k) {
            indices.push((idx, k));
        } else {
            rest.push(k);
        }
    }
    indices.sort_by_key(|(idx, _)| *idx);
    let mut out = Vec::with_capacity(indices.len() + rest.len());
    out.extend(indices.into_iter().map(|(_, k)| k));
    out.extend(rest);
    out
}

fn js_entries(v: &Value, context_prop: &str) -> Result<Vec<(String, Value)>, String> {
    if js_rt::is_nil(v) {
        return Err(format!(
            "TypeError: Cannot convert undefined or null to object ({context_prop})"
        ));
    }
    if let Some(units) = js_rt::string_units(v) {
        return Ok(units
            .iter()
            .enumerate()
            .map(|(i, &u)| (i.to_string(), js_rt::utf16_string(&[u])))
            .collect());
    }
    match v {
        Value::Array(arr) => Ok(arr
            .iter()
            .enumerate()
            .map(|(i, item)| (i.to_string(), item.clone()))
            .collect()),
        Value::Object(o)
            if o.len() == 1 && (o.contains_key("$bytes") || o.contains_key("$buffer")) =>
        {
            let key = if o.contains_key("$bytes") {
                "$bytes"
            } else {
                "$buffer"
            };
            let arr = o
                .get(key)
                .and_then(Value::as_array)
                .map(Vec::as_slice)
                .unwrap_or(&[]);
            Ok(arr
                .iter()
                .enumerate()
                .map(|(i, item)| (i.to_string(), item.clone()))
                .collect())
        }
        Value::Object(o) if !is_special_non_plain(o) => Ok(js_own_keys_order(o)
            .into_iter()
            .filter_map(|k| o.get(k).map(|val| (k.clone(), val.clone())))
            .collect()),
        _ => Ok(Vec::new()),
    }
}

fn js_object_keys_len(v: &Value) -> Result<usize, String> {
    if js_rt::is_nil(v) {
        return Err("TypeError: Cannot convert undefined or null to object".into());
    }
    if let Some(units) = js_rt::string_units(v) {
        return Ok(units.len());
    }
    Ok(match v {
        Value::Array(arr) => arr.len(),
        Value::Object(o)
            if o.len() == 1 && (o.contains_key("$bytes") || o.contains_key("$buffer")) =>
        {
            let key = if o.contains_key("$bytes") {
                "$bytes"
            } else {
                "$buffer"
            };
            o.get(key).and_then(Value::as_array).map_or(0, Vec::len)
        }
        Value::Object(o) if !is_special_non_plain(o) => o.len(),
        _ => 0,
    })
}

fn js_spread_into(dst: &mut Map<String, Value>, src: &Value) {
    if js_rt::is_nil(src) {
        return;
    }
    if let Ok(entries) = js_entries(src, "spread") {
        for (k, val) in entries {
            dst.insert(k, val);
        }
    }
}

fn js_strict_eq(a: &Value, b: &Value) -> bool {
    if js_rt::is_undefined(a) || js_rt::is_undefined(b) {
        return js_rt::is_undefined(a) && js_rt::is_undefined(b);
    }
    if let (Some(na), Some(nb)) = (js_rt::as_number(a), js_rt::as_number(b)) {
        return na == nb;
    }
    if js_rt::as_number(a).is_some() || js_rt::as_number(b).is_some() {
        return false;
    }
    if let (Some(ua), Some(ub)) = (js_rt::string_units(a), js_rt::string_units(b)) {
        return ua == ub;
    }
    a == b
}

fn js_array_from(v: &Value) -> Result<Vec<Value>, String> {
    if js_rt::is_nil(v) {
        return Err("TypeError: undefined or null is not iterable".into());
    }
    if let Some(units) = js_rt::string_units(v) {
        let mut out = Vec::new();
        let mut i = 0;
        while i < units.len() {
            let u = units[i];
            if (0xD800..=0xDBFF).contains(&u)
                && i + 1 < units.len()
                && (0xDC00..=0xDFFF).contains(&units[i + 1])
            {
                out.push(js_rt::utf16_string(&units[i..i + 2]));
                i += 2;
            } else {
                out.push(js_rt::utf16_string(&[u]));
                i += 1;
            }
        }
        return Ok(out);
    }
    match v {
        Value::Array(arr) => Ok(arr.clone()),
        Value::Object(o) if o.len() == 1 && o.contains_key("$set") => Ok(o
            .get("$set")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default()),
        Value::Object(o) if o.len() == 1 && o.contains_key("$map") => Ok(o
            .get("$map")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default()),
        Value::Object(o)
            if o.len() == 1 && (o.contains_key("$bytes") || o.contains_key("$buffer")) =>
        {
            let key = if o.contains_key("$bytes") {
                "$bytes"
            } else {
                "$buffer"
            };
            Ok(o.get(key)
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default())
        }
        _ => Ok(Vec::new()),
    }
}

fn require_obj<'a>(v: &'a Value, prop: &str) -> Result<Option<&'a Map<String, Value>>, String> {
    if js_rt::is_nil(v) {
        return Err(format!(
            "TypeError: Cannot read properties of null/undefined (reading '{prop}')"
        ));
    }
    Ok(v.as_object().filter(|o| !is_special_non_plain(o)))
}

fn require_str(v: Option<&Value>, method: &str) -> Result<String, String> {
    match v {
        Some(Value::String(s)) => Ok(s.clone()),
        Some(v) if js_rt::is_nil(v) => Err(format!(
            "TypeError: Cannot read properties of null/undefined (reading '{method}')"
        )),
        None => Err(format!(
            "TypeError: Cannot read properties of undefined (reading '{method}')"
        )),
        Some(_) => Err(format!("TypeError: value.{method} is not a function")),
    }
}

fn get_prop<'a>(v: &'a Value, key: &str) -> Option<&'a Value> {
    match v {
        Value::Object(o) if !is_special_non_plain(o) => o.get(key),
        Value::Array(arr) => key.parse::<usize>().ok().and_then(|i| arr.get(i)),
        _ => None,
    }
}

fn write_json_str_units(out: &mut String, units: &[u16]) {
    out.push('"');
    let mut i = 0;
    while i < units.len() {
        let u = units[i];
        match u {
            0x0022 => out.push_str("\\\""),
            0x005C => out.push_str("\\\\"),
            0x0008 => out.push_str("\\b"),
            0x000C => out.push_str("\\f"),
            0x000A => out.push_str("\\n"),
            0x000D => out.push_str("\\r"),
            0x0009 => out.push_str("\\t"),
            0x0000..=0x001F => {
                use std::fmt::Write as _;
                let _ = write!(out, "\\u{u:04x}");
            }
            0xD800..=0xDBFF if i + 1 < units.len() && (0xDC00..=0xDFFF).contains(&units[i + 1]) => {
                let hi = u32::from(u) - 0xD800;
                let lo = u32::from(units[i + 1]) - 0xDC00;
                if let Some(ch) = char::from_u32(0x10000 + ((hi << 10) | lo)) {
                    out.push(ch);
                }
                i += 1;
            }
            0xD800..=0xDFFF => {
                use std::fmt::Write as _;
                let _ = write!(out, "\\u{u:04x}");
            }
            _ => {
                if let Some(ch) = char::from_u32(u32::from(u)) {
                    out.push(ch);
                }
            }
        }
        i += 1;
    }
    out.push('"');
}

fn write_json_value(out: &mut String, v: &Value, pretty: bool, depth: usize, sort_keys: bool) {
    if js_rt::is_undefined(v) || v.is_null() {
        out.push_str("null");
        return;
    }
    if let Some(b) = v.as_bool() {
        out.push_str(if b { "true" } else { "false" });
        return;
    }
    if let Some(n) = js_rt::as_number(v) {
        if n.is_finite() {
            out.push_str(&js_rt::number_to_string(n));
        } else {
            out.push_str("null");
        }
        return;
    }
    if let Some(units) = js_rt::string_units(v) {
        write_json_str_units(out, &units);
        return;
    }
    if let Some(arr) = v.as_array() {
        if arr.is_empty() {
            out.push_str("[]");
            return;
        }
        out.push('[');
        for (i, item) in arr.iter().enumerate() {
            if i > 0 {
                out.push(',');
            }
            if pretty {
                out.push('\n');
                for _ in 0..(depth + 1) * 2 {
                    out.push(' ');
                }
            }
            write_json_value(out, item, pretty, depth + 1, sort_keys);
        }
        if pretty {
            out.push('\n');
            for _ in 0..depth * 2 {
                out.push(' ');
            }
        }
        out.push(']');
        return;
    }
    if let Some(o) = v.as_object() {
        if o.len() == 1 {
            if let Some(date) = o.get("$date").and_then(Value::as_str) {
                if date == "Invalid" {
                    out.push_str("null");
                } else {
                    write_json_str_units(out, &date.encode_utf16().collect::<Vec<_>>());
                }
                return;
            }
            if o.contains_key("$set") || o.contains_key("$map") {
                out.push_str("{}");
                return;
            }
            if let Some(bytes) = o
                .get("$bytes")
                .or_else(|| o.get("$buffer"))
                .and_then(Value::as_array)
            {
                if bytes.is_empty() {
                    out.push_str("{}");
                    return;
                }
                let mut tmp = Map::new();
                for (i, b) in bytes.iter().enumerate() {
                    tmp.insert(i.to_string(), b.clone());
                }
                write_json_object(out, &tmp, pretty, depth, sort_keys);
                return;
            }
        }
        write_json_object(out, o, pretty, depth, sort_keys);
    }
}

fn write_json_object(
    out: &mut String,
    o: &Map<String, Value>,
    pretty: bool,
    depth: usize,
    sort_keys: bool,
) {
    let mut keys: Vec<&String> = if sort_keys {
        let mut indices: Vec<(u32, &String)> = Vec::new();
        let mut rest: Vec<&String> = Vec::new();
        for k in o.keys() {
            if let Some(idx) = as_array_index(k) {
                indices.push((idx, k));
            } else {
                rest.push(k);
            }
        }
        indices.sort_by_key(|(idx, _)| *idx);
        rest.sort_by(|a, b| a.encode_utf16().cmp(b.encode_utf16()));
        let mut merged = Vec::with_capacity(indices.len() + rest.len());
        merged.extend(indices.into_iter().map(|(_, k)| k));
        merged.extend(rest);
        merged
    } else {
        js_own_keys_order(o)
    };
    keys.retain(|k| o.get(*k).is_some_and(|val| !js_rt::is_undefined(val)));
    if keys.is_empty() {
        out.push_str("{}");
        return;
    }
    out.push('{');
    for (i, k) in keys.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        if pretty {
            out.push('\n');
            for _ in 0..(depth + 1) * 2 {
                out.push(' ');
            }
        }
        let units: Vec<u16> = k.encode_utf16().collect();
        write_json_str_units(out, &units);
        out.push(':');
        if pretty {
            out.push(' ');
        }
        if let Some(val) = o.get(*k) {
            write_json_value(out, val, pretty, depth + 1, sort_keys);
        }
    }
    if pretty {
        out.push('\n');
        for _ in 0..depth * 2 {
            out.push(' ');
        }
    }
    out.push('}');
}

fn js_stringify(v: &Value, pretty: bool) -> String {
    let mut out = String::new();
    write_json_value(&mut out, v, pretty, 0, false);
    out
}

fn stable_metadata_key(v: &Value) -> String {
    let mut out = String::new();
    write_json_value(&mut out, v, false, 0, true);
    out
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

fn parse_json_value(text: &str) -> Result<Value, ()> {
    let mut de = serde_json::Deserializer::from_str(text);
    de.disable_recursion_limit();
    let val = <Value as serde::Deserialize>::deserialize(&mut de).map_err(|_| ())?;
    de.end().map_err(|_| ())?;
    Ok(normalize_parsed_json(val))
}

fn is_imported_metadata_field(key: &str) -> bool {
    !matches!(
        key,
        "slug" | "context_window" | "max_context_window" | "auto_compact_token_limit"
    )
}

fn filtered_metadata_map(metadata: &Value) -> Result<Map<String, Value>, String> {
    let entries = js_entries(metadata, "filteredMetadata")?;
    let mut out = Map::new();
    for (k, v) in entries {
        if is_imported_metadata_field(&k) {
            out.insert(k, v);
        }
    }
    Ok(out)
}

fn canonicalize_model_metadata_map(map: Map<String, Value>) -> Map<String, Value> {
    let mut canonical = Map::new();
    for (slug, metadata) in map {
        let key = parse_model_row_identity(&slug).key;
        if key.is_empty() {
            continue;
        }
        canonical.insert(key, metadata);
    }
    canonical
}

fn parse_map_inner(value: &str) -> Map<String, Value> {
    if es_trim(value).is_empty() {
        return Map::new();
    }
    let Ok(parsed) = parse_json_value(value) else {
        return Map::new();
    };
    if !is_plain_record(&parsed) {
        return Map::new();
    }
    let entries = js_entries(&parsed, "parseModelMetadataMap").unwrap_or_default();
    let mut map = Map::new();
    for (slug, metadata) in entries {
        if is_plain_record(&metadata)
            && let Ok(filtered) = filtered_metadata_map(&metadata)
        {
            map.insert(slug, Value::Object(filtered));
        }
    }
    canonicalize_model_metadata_map(map)
}

fn serialize_map_inner(map: &Map<String, Value>) -> String {
    if map.is_empty() {
        String::new()
    } else {
        js_stringify(&Value::Object(map.clone()), false)
    }
}

fn parse_uint_and_unit(raw: &str) -> Option<(u128, u128)> {
    let s = es_trim(raw);
    if s.is_empty() {
        return None;
    }
    let (digits, mult) = match s.as_bytes().last().copied() {
        Some(b'k' | b'K') => (&s[..s.len() - 1], 1_000u128),
        Some(b'm' | b'M') => (&s[..s.len() - 1], 1_000_000u128),
        _ => (s, 1u128),
    };
    if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let n: u128 = digits.parse().ok()?;
    n.checked_mul(mult).map(|v| (v, mult))
}

fn context_window_to_u64(value: &str) -> Option<u64> {
    let (tokens, _) = parse_uint_and_unit(value)?;
    if tokens > 0 && tokens <= u128::from(u64::MAX) {
        u64::try_from(tokens).ok()
    } else {
        None
    }
}

fn context_window_to_tokens(value: &str) -> Option<u64> {
    let tokens = context_window_to_u64(value)?;
    if u128::from(tokens) <= MAX_SAFE_INT_U128 {
        Some(tokens)
    } else {
        None
    }
}

fn auto_compact_percent_to_scaled(value: &str) -> Option<u128> {
    let trimmed = es_trim(value);
    let stripped = trimmed.strip_suffix('%').unwrap_or(trimmed);
    let normalized = es_trim(stripped);
    if normalized.is_empty() {
        return None;
    }
    let (int_str, frac_str) = match normalized.split_once('.') {
        Some((i, f)) => {
            if f.is_empty() || f.len() > 6 || !f.bytes().all(|b| b.is_ascii_digit()) {
                return None;
            }
            (i, f)
        }
        None => (normalized, ""),
    };
    if int_str.is_empty() || !int_str.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let int_part: u128 = int_str.parse().ok()?;
    let mut padded = frac_str.to_string();
    while padded.len() < 6 {
        padded.push('0');
    }
    let frac_part: u128 = padded.parse().ok()?;
    let scaled = int_part
        .checked_mul(PERCENT_SCALE)?
        .checked_add(frac_part)?;
    if scaled > 0 && scaled <= SCALED_PERCENT_DENOM {
        Some(scaled)
    } else {
        None
    }
}

fn auto_compact_percent_to_token_limit(
    context_window: &str,
    auto_compact_percent: &str,
) -> Option<u64> {
    let cw = u128::from(context_window_to_u64(context_window)?);
    let sp = auto_compact_percent_to_scaled(auto_compact_percent)?;
    let rounded = (cw * sp + SCALED_PERCENT_DENOM / 2) / SCALED_PERCENT_DENOM;
    let compact = rounded.max(1);
    if compact <= MAX_SAFE_INT_U128 {
        u64::try_from(compact).ok()
    } else {
        None
    }
}

fn auto_compact_token_limit_to_percent(context_window: &str, token_limit: &str) -> Option<String> {
    let cw = u128::from(context_window_to_u64(context_window)?);
    if token_limit.is_empty() || !token_limit.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let ct: u128 = token_limit.parse().ok()?;
    if ct == 0 || ct > cw {
        return None;
    }
    let scaled = (ct * SCALED_PERCENT_DENOM + cw / 2) / cw;
    if scaled == 0 || scaled > SCALED_PERCENT_DENOM {
        return None;
    }
    let whole = scaled / PERCENT_SCALE;
    let rem = scaled % PERCENT_SCALE;
    if rem == 0 {
        Some(format!("{whole}%"))
    } else {
        let frac = format!("{rem:06}");
        let frac = frac.trim_end_matches('0');
        Some(format!("{whole}.{frac}%"))
    }
}

fn display_auto_compact_percent(value: Option<String>) -> Option<String> {
    let s = value?;
    if s.is_empty() {
        return Some(s);
    }
    let Some(scaled) = auto_compact_percent_to_scaled(&s) else {
        return Some(s);
    };
    let rounded = (scaled + PERCENT_SCALE / 2) / PERCENT_SCALE;
    Some(format!("{rounded}%"))
}

struct RowIdentity {
    raw_name: String,
    canonical_slug: String,
    suffix_window: Option<String>,
    key: String,
}

fn suffix_window_tokens_u64(suffix: &str) -> Option<u64> {
    let (tokens, _) = parse_uint_and_unit(suffix)?;
    if tokens > 0 && tokens <= MAX_SAFE_INT_U128 {
        u64::try_from(tokens).ok()
    } else {
        None
    }
}

fn parse_model_row_identity(row_name: &str) -> RowIdentity {
    let trimmed = es_trim(row_name);
    let mut suffix_window: Option<u64> = None;
    let mut canonical = trimmed;
    if let Some(before_close) = trimmed.strip_suffix(']')
        && let Some(open_idx) = before_close.rfind('[')
    {
        let prefix = &before_close[..open_idx];
        let inner = &before_close[open_idx + 1..];
        if !prefix.chars().any(is_line_terminator)
            && !es_trim(prefix).is_empty()
            && let Some(tokens) = suffix_window_tokens_u64(es_trim(inner))
        {
            suffix_window = Some(tokens);
            canonical = es_trim(prefix);
        }
    }
    RowIdentity {
        raw_name: row_name.to_string(),
        canonical_slug: canonical.to_string(),
        suffix_window: suffix_window.map(|t| t.to_string()),
        key: canonical.to_lowercase(),
    }
}

fn serialize_document_val(
    slug: Value,
    metadata: &Value,
    context_window: &str,
    auto_compact_percent: &str,
) -> Result<String, String> {
    let filtered = filtered_metadata_map(metadata)?;
    let cw_tokens = context_window_to_tokens(context_window);
    let ac_limit = auto_compact_percent_to_token_limit(context_window, auto_compact_percent);
    let mut entry = Map::new();
    entry.insert("slug".into(), slug);
    if let Some(tokens) = cw_tokens.filter(|&t| t > 0) {
        entry.insert("context_window".into(), Value::from(tokens));
    }
    if let Some(limit) = ac_limit.filter(|&l| l > 0) {
        entry.insert("auto_compact_token_limit".into(), Value::from(limit));
    }
    for (k, v) in filtered {
        entry.insert(k, v);
    }
    let root = json!({ "models": [Value::Object(entry)] });
    Ok(js_stringify(&root, true))
}

fn strip_ascii_prefix_ci<'a>(s: &'a str, prefix: &str) -> Option<&'a str> {
    s.get(..prefix.len())
        .filter(|p| p.eq_ignore_ascii_case(prefix))
        .map(|_| &s[prefix.len()..])
}

fn strip_markdown_fence(text: &str) -> Option<&str> {
    let body = text.strip_prefix("```")?.strip_suffix("```")?;
    let after_lang = strip_ascii_prefix_ci(body, "javascript")
        .or_else(|| strip_ascii_prefix_ci(body, "json"))
        .or_else(|| strip_ascii_prefix_ci(body, "js"))
        .unwrap_or(body);
    Some(es_trim(after_lang))
}

fn strip_export_default(s: &str) -> &str {
    let Some(after_export) = strip_ascii_prefix_ci(s, "export") else {
        return s;
    };
    let rest = after_export.trim_start_matches(is_es_ws);
    if rest.len() == after_export.len() {
        return s;
    }
    let Some(after_default) = strip_ascii_prefix_ci(rest, "default") else {
        return s;
    };
    let out = after_default.trim_start_matches(is_es_ws);
    if out.len() == after_default.len() {
        s
    } else {
        out
    }
}

fn strip_module_exports(s: &str) -> &str {
    let Some(after) = strip_ascii_prefix_ci(s, "module.exports") else {
        return s;
    };
    let Some(after_eq) = after.trim_start_matches(is_es_ws).strip_prefix('=') else {
        return s;
    };
    after_eq.trim_start_matches(is_es_ws)
}

fn strip_var_decl(s: &str) -> &str {
    let Some(after_kw) = strip_ascii_prefix_ci(s, "const")
        .or_else(|| strip_ascii_prefix_ci(s, "let"))
        .or_else(|| strip_ascii_prefix_ci(s, "var"))
    else {
        return s;
    };
    let trimmed = after_kw.trim_start_matches(is_es_ws);
    if trimmed.len() == after_kw.len() {
        return s;
    }
    let mut chars = trimmed.char_indices();
    let Some((_, first)) = chars.next() else {
        return s;
    };
    if !(first.is_ascii_alphabetic() || first == '_' || first == '$') {
        return s;
    }
    let mut end = first.len_utf8();
    for (idx, c) in chars {
        if c.is_ascii_alphanumeric() || c == '_' || c == '$' {
            end = idx + c.len_utf8();
        } else {
            break;
        }
    }
    let after_ident = trimmed[end..].trim_start_matches(is_es_ws);
    let Some(after_eq) = after_ident.strip_prefix('=') else {
        return s;
    };
    after_eq.trim_start_matches(is_es_ws)
}

fn unwrap_json_compatible_document(source: &str) -> String {
    let mut text = es_trim(source);
    text = text.strip_prefix('\u{FEFF}').unwrap_or(text);
    if let Some(inner) = strip_markdown_fence(text) {
        text = es_trim(inner);
    }
    let s1 = strip_export_default(text);
    let s2 = strip_module_exports(s1);
    let s3 = strip_var_decl(s2);
    let trimmed = es_trim(s3);
    let without_ws = trimmed.trim_end_matches(is_es_ws);
    let without_semi = without_ws.strip_suffix(';').unwrap_or(trimmed);
    es_trim(without_semi).to_string()
}

fn slug_matches_ignore_case(candidate_slug: Option<&Value>, target_slug: &str) -> bool {
    let Some(Value::String(s)) = candidate_slug else {
        return false;
    };
    s.to_lowercase()
        == parse_model_row_identity(target_slug)
            .canonical_slug
            .to_lowercase()
}

fn document_candidates(root: &Value) -> Option<Vec<&Map<String, Value>>> {
    if let Some(arr) = root.as_array() {
        return Some(
            arr.iter()
                .filter(|v| is_plain_record(v))
                .filter_map(Value::as_object)
                .collect(),
        );
    }
    if let Some(obj) = root.as_object().filter(|_| is_plain_record(root)) {
        if let Some(models) = obj.get("models").and_then(Value::as_array) {
            return Some(
                models
                    .iter()
                    .filter(|v| is_plain_record(v))
                    .filter_map(Value::as_object)
                    .collect(),
            );
        }
        if obj.get("slug").is_some_and(Value::is_string) {
            return Some(vec![obj]);
        }
    }
    None
}

fn single_matching_in_slice<'a>(
    items: &'a mut [Value],
    target_slug: &str,
) -> Option<&'a mut Map<String, Value>> {
    let mut hit: Option<usize> = None;
    for (i, item) in items.iter().enumerate() {
        if is_plain_record(item) && slug_matches_ignore_case(item.get("slug"), target_slug) {
            if hit.is_some() {
                return None;
            }
            hit = Some(i);
        }
    }
    hit.and_then(|i| items.get_mut(i)?.as_object_mut())
}

fn mut_single_candidate<'a>(
    root: &'a mut Value,
    target_slug: &str,
) -> Option<&'a mut Map<String, Value>> {
    match root {
        Value::Array(arr) => single_matching_in_slice(arr, target_slug),
        Value::Object(obj) if !is_special_non_plain(obj) => {
            if obj.get("models").is_some_and(Value::is_array) {
                let models = obj.get_mut("models")?.as_array_mut()?;
                single_matching_in_slice(models, target_slug)
            } else if obj.get("slug").is_some_and(Value::is_string)
                && slug_matches_ignore_case(obj.get("slug"), target_slug)
            {
                Some(obj)
            } else {
                None
            }
        }
        _ => None,
    }
}

fn reorder_managed_model_fields(model: &mut Map<String, Value>) {
    let mut ordered = Map::new();
    for key in [
        "slug",
        "context_window",
        "max_context_window",
        "auto_compact_token_limit",
    ] {
        if let Some(val) = model.get(key) {
            ordered.insert(key.into(), val.clone());
        }
    }
    let old = std::mem::take(model);
    for (k, v) in old {
        if !ordered.contains_key(&k) {
            ordered.insert(k, v);
        }
    }
    *model = ordered;
}

fn speed_tier_is(value: &Value, id: &str) -> bool {
    let tier = if value.is_string() {
        Some(value)
    } else if is_plain_record(value) {
        value.get("id")
    } else {
        None
    };
    tier.and_then(Value::as_str)
        .is_some_and(|s| es_trim(s).to_lowercase() == id)
}

fn sync_document_and_parse(
    source: &str,
    target_slug: &str,
    mutate: impl FnOnce(&mut Map<String, Value>),
) -> Result<Option<Value>, String> {
    let mut root = if es_trim(source).is_empty() {
        json!({ "models": [{ "slug": parse_model_row_identity(target_slug).canonical_slug }] })
    } else {
        let Ok(parsed) = parse_json_value(&unwrap_json_compatible_document(source)) else {
            return Ok(None);
        };
        parsed
    };
    let Some(candidate) = mut_single_candidate(&mut root, target_slug) else {
        return Ok(None);
    };
    mutate(candidate);
    let document = js_stringify(&root, true);
    let parsed = parse_document_inner(&document, target_slug);
    if parsed.get("ok").and_then(Value::as_bool) == Some(true) {
        Ok(Some(json!({
            "document": document,
            "preview": parsed.get("value").cloned().unwrap_or(Value::Null),
        })))
    } else {
        Ok(None)
    }
}

fn is_safe_positive_int(n: f64) -> bool {
    n.is_finite() && n.fract() == 0.0 && n > 0.0 && n <= MAX_SAFE_INT_F64
}

fn positive_integer_string(value: &Value) -> Option<String> {
    if let Some(n) = js_rt::as_number(value) {
        return is_safe_positive_int(n).then(|| js_rt::number_to_string(n));
    }
    if let Some(s) = value.as_str() {
        let trimmed = es_trim(s);
        if !trimmed.is_empty() && trimmed.bytes().all(|b| b.is_ascii_digit()) {
            let parsed: f64 = trimmed.parse().ok()?;
            return is_safe_positive_int(parsed).then(|| js_rt::number_to_string(parsed));
        }
    }
    None
}

fn save_decision_inner(
    parse_ok: bool,
    document_blank: bool,
    imported: bool,
    matches_builtin: bool,
) -> Value {
    let label = "保存此模型";
    if !parse_ok {
        return json!({
            "needsSave": false,
            "effect": "none",
            "label": label,
            "title": "JSON 无法解析，修复后即可保存",
        });
    }
    if document_blank {
        return if imported {
            json!({
                "needsSave": true,
                "effect": "builtin",
                "label": "保存此模型",
                "title": "保存后清除该模型的自定义配置，生成时回退默认模板",
            })
        } else {
            json!({
                "needsSave": false,
                "effect": "none",
                "label": label,
                "title": "没有可保存的内容",
            })
        };
    }
    if matches_builtin {
        return if imported {
            json!({
                "needsSave": true,
                "effect": "builtin",
                "label": "保存此模型",
                "title": "内容与内置元数据一致，保存后使用内置元数据",
            })
        } else {
            json!({
                "needsSave": false,
                "effect": "none",
                "label": label,
                "title": "内容与内置元数据一致，保存后继续使用内置元数据",
            })
        };
    }
    json!({
        "needsSave": true,
        "effect": "custom",
        "label": if imported { "更新此模型配置" } else { "保存为自定义配置" },
        "title": if imported {
            "保存当前内容为该模型的自定义配置"
        } else {
            "当前为内置元数据预览的修改版；保存后将成为该模型的自定义配置，生成时覆盖内置"
        },
    })
}

fn validate_capabilities_inner(metadata: &Value) -> Result<Option<String>, String> {
    if js_rt::is_nil(metadata) {
        return Err("TypeError: Cannot convert undefined or null to object".into());
    }
    let Some(obj) = metadata.as_object().filter(|o| !is_special_non_plain(o)) else {
        return Ok(None);
    };
    if let Some(v) = obj.get("base_instructions")
        && !v.is_string()
    {
        return Ok(Some(
            "base_instructions 必须是字符串（允许空字符串）。".into(),
        ));
    }
    if let Some(v) = obj.get("supports_parallel_tool_calls")
        && !v.is_boolean()
    {
        return Ok(Some(
            "supports_parallel_tool_calls 必须是 true 或 false。".into(),
        ));
    }
    if let Some(v) = obj.get("supported_reasoning_levels") {
        let Some(arr) = v.as_array() else {
            return Ok(Some("supported_reasoning_levels 必须是数组。".into()));
        };
        for item in arr {
            let valid = is_plain_record(item)
                && item
                    .get("effort")
                    .and_then(Value::as_str)
                    .is_some_and(|s| !es_trim(s).is_empty())
                && item.get("description").is_some_and(Value::is_string);
            if !valid {
                return Ok(Some(
                    "supported_reasoning_levels 的每一项都必须包含 effort 和 description 字符串。"
                        .into(),
                ));
            }
        }
    }
    if let Some(v) = obj.get("multi_agent_version")
        && !matches!(v.as_str(), Some("v1" | "v2" | "disabled"))
    {
        return Ok(Some(
            "multi_agent_version 只支持 v1、v2 或 disabled（留空/删除该键表示自动）。".into(),
        ));
    }
    if let Some(v) = obj.get("default_reasoning_level")
        && !v.is_null()
        && !v.as_str().is_some_and(|s| !es_trim(s).is_empty())
    {
        return Ok(Some("default_reasoning_level 必须是非空字符串。".into()));
    }
    if let Some(v) = obj.get("support_verbosity")
        && !v.is_boolean()
    {
        return Ok(Some("support_verbosity 必须是 true 或 false。".into()));
    }
    if let Some(v) = obj.get("default_verbosity")
        && !v.is_null()
        && !v.as_str().is_some_and(|s| !es_trim(s).is_empty())
    {
        return Ok(Some("default_verbosity 必须是非空字符串。".into()));
    }
    Ok(None)
}

fn parse_document_inner(source: &str, target_slug: &str) -> Value {
    if es_trim(source).is_empty() {
        return json!({ "ok": false, "error": "请先粘贴 model.js 或 JSON 配置。" });
    }
    if es_trim(target_slug).is_empty() {
        return json!({ "ok": false, "error": "当前模型名称为空，无法匹配 slug。" });
    }
    let Ok(root) = parse_json_value(&unwrap_json_compatible_document(source)) else {
        return json!({
            "ok": false,
            "error": "无法解析配置。仅支持 JSON，或 export default / module.exports 包裹的 JSON；不会执行 JavaScript。",
        });
    };
    let Some(candidates) = document_candidates(&root) else {
        return json!({
            "ok": false,
            "error": "配置中没有找到 models 数组或带 slug 的模型对象。",
        });
    };
    let matches: Vec<&Map<String, Value>> = candidates
        .iter()
        .copied()
        .filter(|m| slug_matches_ignore_case(m.get("slug"), target_slug))
        .collect();
    if matches.is_empty() {
        let available: Vec<&str> = candidates
            .iter()
            .filter_map(|m| m.get("slug").and_then(Value::as_str))
            .filter(|s| !s.is_empty())
            .collect();
        let suffix = if available.is_empty() {
            String::new()
        } else {
            format!(" 文档包含：{}。", available.join("、"))
        };
        return json!({
            "ok": false,
            "error": format!("文档中没有找到当前模型 slug：{target_slug}。{suffix}"),
        });
    }
    if matches.len() > 1 {
        return json!({
            "ok": false,
            "error": format!("文档中存在多个 slug 为 {target_slug} 的模型，无法确定要导入哪一个。"),
        });
    }

    let model = matches[0];
    let window_field = if model
        .get("max_context_window")
        .is_some_and(|v| !v.is_null())
    {
        "max_context_window"
    } else {
        "context_window"
    };
    let mut context_window: Option<String> = None;
    if let Some(raw_win) = model.get(window_field)
        && !raw_win.is_null()
    {
        let Some(cw) = positive_integer_string(raw_win) else {
            return json!({ "ok": false, "error": format!("{window_field} 必须是正整数。") });
        };
        context_window = Some(cw);
    }
    let mut auto_compact_percent: Option<String> = None;
    if let Some(raw_limit) = model.get("auto_compact_token_limit")
        && !raw_limit.is_null()
    {
        let Some(limit) = positive_integer_string(raw_limit) else {
            return json!({ "ok": false, "error": "auto_compact_token_limit 必须是正整数或 null。" });
        };
        let Some(ref cw) = context_window else {
            return json!({
                "ok": false,
                "error": "存在 auto_compact_token_limit 时必须同时提供 context_window 或 max_context_window。",
            });
        };
        let Some(pct) = auto_compact_token_limit_to_percent(cw, &limit) else {
            return json!({ "ok": false, "error": "auto_compact_token_limit 必须小于或等于 context_window。" });
        };
        auto_compact_percent = Some(pct);
    }

    let model_val = Value::Object(model.clone());
    let mut metadata = filtered_metadata_map(&model_val).unwrap_or_default();
    if let Some(b) = metadata
        .get("supports_reasoning_summaries")
        .and_then(Value::as_bool)
        && !metadata.contains_key("supports_reasoning_summary_parameter")
    {
        metadata.insert(
            "supports_reasoning_summary_parameter".into(),
            Value::Bool(b),
        );
    }
    let meta_val = Value::Object(metadata);
    if let Ok(Some(err)) = validate_capabilities_inner(&meta_val) {
        return json!({ "ok": false, "error": err });
    }
    let display_pct = display_auto_compact_percent(auto_compact_percent.clone());
    json!({
        "ok": true,
        "value": {
            "slug": target_slug,
            "metadata": meta_val,
            "documentEntry": model_val,
            "contextWindow": context_window,
            "autoCompactPercent": display_pct,
            "autoCompactCalculationPercent": auto_compact_percent,
            "ignoredFields": [],
        },
    })
}

/// Port of `modelMetadataKey` (`src/model-metadata.ts:49`).
pub fn model_metadata_key(row_name: String) -> Result<String, String> {
    Ok(parse_model_row_identity(&row_name).key)
}

/// Port of `parseModelMetadataMap` (`src/model-metadata.ts:63`).
pub fn parse_model_metadata_map(value: String) -> Result<Value, String> {
    Ok(Value::Object(parse_map_inner(&value)))
}

/// Port of `serializeModelMetadataMap` (`src/model-metadata.ts:80`).
pub fn serialize_model_metadata_map(map: Value) -> Result<String, String> {
    let len = js_object_keys_len(&map)?;
    if len > 0 {
        Ok(js_stringify(&map, false))
    } else {
        Ok(String::new())
    }
}

/// Port of `serializeModelMetadataDocument` (`src/model-metadata.ts:151`).
pub fn serialize_model_metadata_document(
    slug: String,
    metadata: Value,
    context_window: String,
    auto_compact_percent: Option<String>,
) -> Result<String, String> {
    serialize_document_val(
        Value::String(slug),
        &metadata,
        &context_window,
        auto_compact_percent.as_deref().unwrap_or(""),
    )
}

/// Port of `parseModelRowName` (`src/model-metadata.ts:196`).
pub fn parse_model_row_name(row_name: String) -> Result<Value, String> {
    let id = parse_model_row_identity(&row_name);
    Ok(json!({
        "rawName": id.raw_name,
        "canonicalSlug": id.canonical_slug,
        "suffixWindow": id.suffix_window,
        "key": id.key,
    }))
}

/// Port of `modelSlugFromRowName` (`src/model-metadata.ts:211`).
pub fn model_slug_from_row_name(row_name: String) -> Result<String, String> {
    Ok(parse_model_row_identity(&row_name).canonical_slug)
}

/// Port of `suffixWindowTokens` (`src/model-metadata.ts:217`).
pub fn suffix_window_tokens(suffix: String) -> Result<Value, String> {
    Ok(suffix_window_tokens_u64(&suffix).map_or(Value::Null, |t| js_rt::js_number(t as f64)))
}

/// Port of `suffixWindowString` (`src/model-metadata.ts:229`).
pub fn suffix_window_string(row_name: String) -> Result<Value, String> {
    Ok(parse_model_row_identity(&row_name)
        .suffix_window
        .map_or(Value::Null, Value::String))
}

/// Port of `normalizeTokenCountInput` (`src/model-metadata.ts:244`).
pub fn normalize_token_count_input(value: String) -> Result<String, String> {
    let trimmed = es_trim(&value);
    if trimmed.is_empty() {
        return Ok(String::new());
    }
    if let Some(tokens) = suffix_window_tokens_u64(trimmed) {
        return Ok(tokens.to_string());
    }
    let digits: String = trimmed.chars().filter(char::is_ascii_digit).collect();
    Ok(if digits == "0" { String::new() } else { digits })
}

/// Port of `builtinRowBackfillValue` (`src/model-metadata.ts:262`).
pub fn builtin_row_backfill_value(
    slug: String,
    row_window: String,
    row_auto_compact: String,
    builtin: Option<Value>,
) -> Result<Value, String> {
    let Some(b) = builtin.filter(js_rt::is_truthy) else {
        return Ok(json!({ "window": null, "autoCompact": null }));
    };
    if parse_model_row_identity(&slug).suffix_window.is_some() {
        return Ok(json!({ "window": null, "autoCompact": null }));
    }
    let cw = get_prop(&b, "context_window").and_then(js_rt::as_number);
    let window = if es_trim(&row_window).is_empty() {
        cw.filter(|n| n.is_finite() && *n > 0.0)
            .map(js_rt::number_to_string)
    } else {
        None
    };
    let limit = get_prop(&b, "auto_compact_token_limit").and_then(js_rt::as_number);
    let derived = match (limit.filter(|n| n.is_finite() && *n > 0.0), &window) {
        (Some(l), Some(w)) => display_auto_compact_percent(auto_compact_token_limit_to_percent(
            w,
            &js_rt::number_to_string(l),
        )),
        _ => None,
    };
    let auto_compact = if !es_trim(&row_auto_compact).is_empty() {
        None
    } else {
        Some(derived.unwrap_or_else(|| DEFAULT_AUTO_COMPACT_PERCENT.to_string()))
    };
    Ok(json!({ "window": window, "autoCompact": auto_compact }))
}

/// Port of `createActiveImportDraft` (`src/model-metadata.ts:321`).
pub fn create_active_import_draft(input: Value) -> Result<Value, String> {
    let _ = require_obj(&input, "index")?;
    let row_name = require_str(get_prop(&input, "rowName"), "trim")?;
    let index = get_prop(&input, "index")
        .cloned()
        .unwrap_or_else(js_rt::undefined);
    let orig_window = get_prop(&input, "window")
        .cloned()
        .unwrap_or_else(js_rt::undefined);
    let orig_compact = get_prop(&input, "autoCompact")
        .cloned()
        .unwrap_or_else(js_rt::undefined);
    let document = match get_prop(&input, "document") {
        Some(v) if !js_rt::is_nil(v) => v.clone(),
        _ => Value::String(String::new()),
    };
    let preview = match get_prop(&input, "preview") {
        Some(v) if !js_rt::is_nil(v) => v.clone(),
        _ => Value::Null,
    };
    Ok(json!({
        "index": index,
        "originalSlug": es_trim(&row_name),
        "originalWindow": orig_window,
        "originalAutoCompact": orig_compact,
        "document": document,
        "preview": preview,
    }))
}

/// Port of `updateActiveImportDraft` (`src/model-metadata.ts:339`).
pub fn update_active_import_draft(draft: Value, patch: Value) -> Result<Value, String> {
    let mut out = Map::new();
    js_spread_into(&mut out, &draft);
    js_spread_into(&mut out, &patch);
    Ok(Value::Object(out))
}

/// Port of `cancelActiveImportDraft` (`src/model-metadata.ts:346`).
pub fn cancel_active_import_draft(draft: Value) -> Result<Value, String> {
    let _ = require_obj(&draft, "originalWindow")?;
    let window = get_prop(&draft, "originalWindow")
        .cloned()
        .unwrap_or_else(js_rt::undefined);
    let auto_compact = get_prop(&draft, "originalAutoCompact")
        .cloned()
        .unwrap_or_else(js_rt::undefined);
    Ok(json!({
        "draft": null,
        "rowPatch": {
            "window": window,
            "autoCompact": auto_compact,
        },
    }))
}

/// Port of `rematchActiveImportDraft` (`src/model-metadata.ts:355`).
pub fn rematch_active_import_draft(
    draft: Value,
    document: String,
    preview: Option<Value>,
) -> Result<Value, String> {
    let mut out = Map::new();
    js_spread_into(&mut out, &draft);
    out.insert("document".into(), Value::String(document));
    out.insert("preview".into(), preview.unwrap_or(Value::Null));
    Ok(Value::Object(out))
}

/// Port of `builtinEntryToImportDocument` (`src/model-metadata.ts:370`).
pub fn builtin_entry_to_import_document(
    entry: Value,
    row_name: Option<String>,
) -> Result<String, String> {
    let _ = require_obj(&entry, "context_window")?;
    let cw = match get_prop(&entry, "context_window") {
        Some(v) if !js_rt::is_nil(v) => Some(v),
        _ => match get_prop(&entry, "max_context_window") {
            Some(v) if !js_rt::is_nil(v) => Some(v),
            _ => None,
        },
    };
    let cw_str = cw
        .filter(|v| js_rt::is_truthy(v))
        .map(js_rt::js_str)
        .unwrap_or_default();
    let row_slug = row_name
        .as_deref()
        .filter(|s| !es_trim(s).is_empty())
        .map(|s| parse_model_row_identity(s).canonical_slug)
        .unwrap_or_default();
    let slug = if !row_slug.is_empty() {
        Value::String(row_slug)
    } else {
        get_prop(&entry, "slug")
            .cloned()
            .unwrap_or_else(js_rt::undefined)
    };
    serialize_document_val(slug, &entry, &cw_str, "")
}

/// Port of `metadataSourceTags` (`src/model-metadata.ts:397`).
pub fn metadata_source_tags(options: Value) -> Result<Vec<Value>, String> {
    let _ = require_obj(&options, "imported")?;
    let imported = get_prop(&options, "imported").is_some_and(js_rt::is_truthy);
    let builtin_match = get_prop(&options, "builtinMatch");
    let builtin_index_slug = get_prop(&options, "builtinIndexSlug");

    let fallback_slug =
        if let Some(v) = get_prop(&options, "fallbackSlug").filter(|v| js_rt::is_truthy(v)) {
            v.clone()
        } else if let Some(v) = builtin_match
            .filter(|v| !js_rt::is_nil(v))
            .and_then(|bm| get_prop(bm, "fallback"))
            .filter(|v| !js_rt::is_nil(v))
            .and_then(|fb| get_prop(fb, "slug"))
            .filter(|v| js_rt::is_truthy(v))
        {
            v.clone()
        } else {
            Value::String("gpt-5.5".into())
        };

    let matched_hit = builtin_match
        .filter(|v| !js_rt::is_nil(v))
        .is_some_and(|bm| {
            get_prop(bm, "matched").is_some_and(js_rt::is_truthy)
                && get_prop(bm, "entry").is_some_and(js_rt::is_truthy)
        });
    let matched_source = if matched_hit {
        match builtin_match.and_then(|bm| get_prop(bm, "source")) {
            Some(v) if !js_rt::is_nil(v) => v.clone(),
            _ => Value::String(String::new()),
        }
    } else {
        builtin_index_slug
            .filter(|v| !js_rt::is_nil(v))
            .and_then(|bis| get_prop(bis, "source"))
            .cloned()
            .unwrap_or_else(js_rt::undefined)
    };

    let mut tags = Vec::new();
    if js_rt::is_truthy(&matched_source) {
        tags.push(json!({ "kind": "match", "source": matched_source, "tone": "builtin" }));
    } else if !imported {
        tags.push(json!({ "kind": "fallback", "source": fallback_slug, "tone": "fallback" }));
    }
    if imported {
        let custom_src = if js_rt::is_truthy(&matched_source) {
            matched_source
        } else {
            js_rt::undefined()
        };
        tags.push(json!({ "kind": "custom", "source": custom_src, "tone": "custom" }));
    }
    Ok(tags)
}

/// Port of `importDocumentSyncPatch` (`src/model-metadata.ts:444`).
pub fn import_document_sync_patch(row: Value, preview: Value) -> Result<Value, String> {
    let _ = require_obj(&preview, "contextWindow")?;
    let mut patch = Map::new();
    if let Some(cw) = get_prop(&preview, "contextWindow").filter(|v| js_rt::is_truthy(v)) {
        let _ = require_obj(&row, "window")?;
        let row_win = require_str(get_prop(&row, "window"), "trim")?;
        if es_trim(&row_win).is_empty() {
            patch.insert("window".into(), cw.clone());
        }
    }
    if let Some(ac) = get_prop(&preview, "autoCompactPercent").filter(|v| js_rt::is_truthy(v)) {
        let _ = require_obj(&row, "autoCompact")?;
        let row_ac = get_prop(&row, "autoCompact").unwrap_or(&Value::Null);
        let row_ac_is_undef = get_prop(&row, "autoCompact").is_none();
        let diff = if row_ac_is_undef {
            !js_rt::is_undefined(ac)
        } else {
            !js_strict_eq(ac, row_ac)
        };
        if diff {
            patch.insert("autoCompact".into(), ac.clone());
        }
    }
    Ok(Value::Object(patch))
}

/// Port of `metadataMatchesBuiltin` (`src/model-metadata.ts:487`).
pub fn metadata_matches_builtin(
    metadata: Option<Value>,
    builtin_metadata: Option<Value>,
) -> Result<bool, String> {
    let (Some(m), Some(b)) = (
        metadata.filter(js_rt::is_truthy),
        builtin_metadata.filter(js_rt::is_truthy),
    ) else {
        return Ok(false);
    };
    if js_object_keys_len(&m)? == 0 || js_object_keys_len(&b)? == 0 {
        return Ok(false);
    }
    Ok(stable_metadata_key(&m) == stable_metadata_key(&b))
}

/// Port of `importSaveDecision` (`src/model-metadata.ts:500`).
pub fn import_save_decision(options: Value) -> Result<Value, String> {
    let _ = require_obj(&options, "parseOk")?;
    let parse_ok = get_prop(&options, "parseOk").is_some_and(js_rt::is_truthy);
    let doc_blank = get_prop(&options, "documentBlank").is_some_and(js_rt::is_truthy);
    let imported = get_prop(&options, "imported").is_some_and(js_rt::is_truthy);
    let matches_builtin = get_prop(&options, "matchesBuiltin").is_some_and(js_rt::is_truthy);
    Ok(save_decision_inner(
        parse_ok,
        doc_blank,
        imported,
        matches_builtin,
    ))
}

/// Port of `importPanelControls` (`src/model-metadata.ts:548`).
pub fn import_panel_controls(options: Value) -> Result<Value, String> {
    let _ = require_obj(&options, "slug")?;
    let slug = require_str(get_prop(&options, "slug"), "trim")?;
    let doc = require_str(get_prop(&options, "document"), "trim")?;
    let slug_blank = es_trim(&slug).is_empty();
    let doc_blank = es_trim(&doc).is_empty();
    let imported = get_prop(&options, "imported").is_some_and(js_rt::is_truthy);
    let parse_ok = get_prop(&options, "parseOk").is_some_and(js_rt::is_truthy);
    let matched = get_prop(&options, "matched").is_some_and(js_rt::is_truthy);
    let matches_builtin = get_prop(&options, "matchesBuiltin").is_some_and(js_rt::is_truthy);
    let decision = save_decision_inner(parse_ok, doc_blank, imported, matches_builtin);
    let rematch_title = if slug_blank {
        "请先填写模型名称"
    } else if matched {
        "按当前模型名重新匹配内置元数据并重填下方内容"
    } else {
        "当前模型名没有内置元数据可匹配"
    };
    Ok(json!({
        "rematch": { "disabled": slug_blank || !matched, "title": rematch_title },
        "clear": { "disabled": false, "title": "清空面板内容（不影响已保存的配置）" },
        "cancel": { "disabled": false, "title": "放弃本次在面板里的改动，不写入任何配置" },
        "save": {
            "disabled": !parse_ok,
            "label": decision.get("label").cloned().unwrap_or(Value::Null),
            "title": decision.get("title").cloned().unwrap_or(Value::Null),
        },
    }))
}

/// Port of `replaceModelMetadataForSlug` (`src/model-metadata.ts:592`).
pub fn replace_model_metadata_for_slug(
    value: String,
    slug: String,
    metadata: Value,
) -> Result<String, String> {
    let mut map = parse_map_inner(&value);
    let key = parse_model_row_identity(&slug).key;
    if key.is_empty() {
        return Ok(serialize_map_inner(&map));
    }
    let mut imported = filtered_metadata_map(&metadata)?;
    if let Some(existing_dn) = map
        .get(&key)
        .and_then(|v| get_prop(v, "display_name"))
        .and_then(Value::as_str)
        .filter(|s| !es_trim(s).is_empty())
    {
        imported.insert(
            "display_name".into(),
            Value::String(existing_dn.to_string()),
        );
    }
    map.insert(key, Value::Object(imported));
    Ok(serialize_map_inner(&map))
}

/// Port of `resolveModelMetadataRowKey` (`src/model-metadata.ts:616`).
pub fn resolve_model_metadata_row_key(map: Value, row_names: Value) -> Result<Value, String> {
    let _ = require_obj(&row_names, "current")?;
    let current_str = require_str(get_prop(&row_names, "current"), "trim")?;
    let current = parse_model_row_identity(&current_str).key;
    if !current.is_empty() {
        let _ = require_obj(&map, &current)?;
        if get_prop(&map, &current).is_some_and(js_rt::is_truthy) {
            return Ok(Value::String(current));
        }
    }
    let origin_str = match get_prop(&row_names, "origin") {
        Some(v) if !js_rt::is_nil(v) => require_str(Some(v), "trim")?,
        _ => String::new(),
    };
    let origin = parse_model_row_identity(&origin_str).key;
    if !origin.is_empty() {
        let _ = require_obj(&map, &origin)?;
        if get_prop(&map, &origin).is_some_and(js_rt::is_truthy) {
            return Ok(Value::String(origin));
        }
    }
    Ok(Value::Null)
}

/// Port of `clearModelMetadataForSlug` (`src/model-metadata.ts:627`).
pub fn clear_model_metadata_for_slug(value: String, slug: String) -> Result<String, String> {
    let mut map = parse_map_inner(&value);
    let key = parse_model_row_identity(&slug).key;
    if !key.is_empty() {
        map.shift_remove(&key);
    }
    Ok(serialize_map_inner(&map))
}

/// Port of `remapModelMetadataSlugs` (`src/model-metadata.ts:634`).
pub fn remap_model_metadata_slugs(value: String, mappings: Value) -> Result<String, String> {
    let map = parse_map_inner(&value);
    let items = js_array_from(&mappings)?;
    let mut normalized = Vec::with_capacity(items.len());
    for item in items {
        let _ = require_obj(&item, "previousSlug")?;
        let prev = require_str(get_prop(&item, "previousSlug"), "trim")?;
        let next = require_str(get_prop(&item, "nextSlug"), "trim")?;
        normalized.push((
            parse_model_row_identity(&prev).key,
            parse_model_row_identity(&next).key,
        ));
    }
    let retained: HashSet<&str> = normalized
        .iter()
        .filter(|(p, n)| !p.is_empty() && p == n)
        .map(|(p, _)| p.as_str())
        .collect();
    let moves: Vec<(&str, &str)> = normalized
        .iter()
        .filter(|(p, n)| {
            !p.is_empty() && !n.is_empty() && p != n && map.get(p).is_some_and(js_rt::is_truthy)
        })
        .map(|(p, n)| (p.as_str(), n.as_str()))
        .collect();
    if moves.is_empty() {
        return Ok(serialize_map_inner(&map));
    }
    let mut moved_keys: HashSet<&str> = moves.iter().map(|(_, n)| *n).collect();
    for (p, _) in &moves {
        if !retained.contains(p) {
            moved_keys.insert(p);
        }
    }
    let mut next_map = Map::new();
    for (k, v) in &map {
        if !moved_keys.contains(k.as_str()) {
            next_map.insert(k.clone(), v.clone());
        }
    }
    for (p, n) in &moves {
        if let Some(v) = map.get(*p) {
            next_map.insert((*n).to_string(), v.clone());
        }
    }
    Ok(serialize_map_inner(&next_map))
}

/// Port of `retainModelMetadataForSlugs` (`src/model-metadata.ts:664`).
pub fn retain_model_metadata_for_slugs(value: String, slugs: Value) -> Result<String, String> {
    let items = js_array_from(&slugs)?;
    let mut allowed = HashSet::new();
    for item in items {
        let s = require_str(Some(&item), "trim")?;
        let key = parse_model_row_identity(&s).key;
        if !key.is_empty() {
            allowed.insert(key);
        }
    }
    let map = parse_map_inner(&value);
    let mut next = Map::new();
    for (k, v) in map {
        if allowed.contains(&k) {
            next.insert(k, v);
        }
    }
    Ok(serialize_map_inner(&next))
}

/// Port of `modelFastSupportMode` (`src/model-metadata.ts:706`).
pub fn model_fast_support_mode(metadata: Option<Value>) -> Result<String, String> {
    let Some(meta) = metadata.filter(|v| !js_rt::is_nil(v)) else {
        return Ok("inherit".into());
    };
    if let Some(mode) = get_prop(&meta, "codex_plus_fast_support").and_then(Value::as_str)
        && matches!(mode, "inherit" | "supported" | "unsupported")
    {
        return Ok(mode.into());
    }
    if let Some(tiers) = get_prop(&meta, "service_tiers").and_then(Value::as_array) {
        return Ok(if tiers.iter().any(|t| speed_tier_is(t, "priority")) {
            "supported"
        } else {
            "unsupported"
        }
        .into());
    }
    if let Some(tiers) = get_prop(&meta, "additional_speed_tiers").and_then(Value::as_array) {
        return Ok(if tiers.iter().any(|t| speed_tier_is(t, "fast")) {
            "supported"
        } else {
            "unsupported"
        }
        .into());
    }
    Ok("inherit".into())
}

/// Port of `setModelFastSupportMode` (`src/model-metadata.ts:719`).
pub fn set_model_fast_support_mode(metadata: Value, mode: String) -> Result<Value, String> {
    let mut out = Map::new();
    js_spread_into(&mut out, &metadata);
    out.insert("codex_plus_fast_support".into(), Value::String(mode));
    Ok(Value::Object(out))
}

/// Port of `synchronizeModelMetadataFastSupport` (`src/model-metadata.ts:724`).
pub fn synchronize_model_metadata_fast_support(
    source: String,
    target_slug: String,
    mode: String,
) -> Result<Option<Value>, String> {
    sync_document_and_parse(&source, &target_slug, |entry| {
        entry.insert("codex_plus_fast_support".into(), Value::String(mode));
    })
}

/// Port of `synchronizeModelMetadataLiteSupport` (`src/model-metadata.ts:744`).
pub fn synchronize_model_metadata_lite_support(
    source: String,
    target_slug: String,
    mode: String,
) -> Result<Option<Value>, String> {
    sync_document_and_parse(&source, &target_slug, |entry| {
        if mode == "inherit" {
            entry.shift_remove("use_responses_lite");
        } else {
            entry.insert(
                "use_responses_lite".into(),
                Value::Bool(mode == "supported"),
            );
        }
    })
}

/// Port of `modelMultiAgentMode` (`src/model-metadata.ts:764`).
pub fn model_multi_agent_mode(metadata: Option<Value>) -> Result<String, String> {
    let Some(meta) = metadata.filter(|v| !js_rt::is_nil(v)) else {
        return Ok("inherit".into());
    };
    Ok(
        match get_prop(&meta, "multi_agent_version").and_then(Value::as_str) {
            Some(v @ ("v1" | "v2" | "disabled")) => v.to_string(),
            _ => "inherit".to_string(),
        },
    )
}

/// Port of `synchronizeModelMetadataMultiAgentVersion` (`src/model-metadata.ts:775`).
pub fn synchronize_model_metadata_multi_agent_version(
    source: String,
    target_slug: String,
    mode: String,
) -> Result<Option<Value>, String> {
    sync_document_and_parse(&source, &target_slug, |entry| {
        if mode == "inherit" {
            entry.shift_remove("multi_agent_version");
        } else {
            entry.insert("multi_agent_version".into(), Value::String(mode));
        }
    })
}

fn sync_doc_context_window_opt(
    source: &str,
    target_slug: &str,
    context_window: &str,
) -> Option<String> {
    let mut root = parse_json_value(&unwrap_json_compatible_document(source)).ok()?;
    let entry = mut_single_candidate(&mut root, target_slug)?;
    let trimmed = es_trim(context_window);
    let tokens = context_window_to_tokens(trimmed);
    if !trimmed.is_empty() && tokens.is_none_or(|t| t == 0) {
        return None;
    }
    if let Some(t) = tokens.filter(|&t| t > 0) {
        entry.insert("context_window".into(), Value::from(t));
        if entry.contains_key("max_context_window") {
            entry.insert("max_context_window".into(), Value::from(t));
        }
    } else if entry.contains_key("context_window") {
        entry.insert("context_window".into(), Value::Null);
        if entry.contains_key("max_context_window") {
            entry.insert("max_context_window".into(), Value::Null);
        }
    }
    reorder_managed_model_fields(entry);
    Some(js_stringify(&root, true))
}

fn sync_doc_limits_opt(
    source: &str,
    target_slug: &str,
    context_window: &str,
    auto_compact_percent: &str,
) -> Option<String> {
    let synced = sync_doc_context_window_opt(source, target_slug, context_window)?;
    let mut root = parse_json_value(&synced).ok()?;
    let entry = mut_single_candidate(&mut root, target_slug)?;
    if let Some(limit) =
        auto_compact_percent_to_token_limit(context_window, auto_compact_percent).filter(|&l| l > 0)
    {
        entry.insert("auto_compact_token_limit".into(), Value::from(limit));
    } else if entry.contains_key("auto_compact_token_limit") {
        entry.insert("auto_compact_token_limit".into(), Value::Null);
    }
    reorder_managed_model_fields(entry);
    Some(js_stringify(&root, true))
}

/// Port of `synchronizeModelMetadataDocumentContextWindow` (`src/model-metadata.ts:805`).
pub fn synchronize_model_metadata_document_context_window(
    source: String,
    target_slug: String,
    context_window: String,
) -> Result<Value, String> {
    Ok(
        sync_doc_context_window_opt(&source, &target_slug, &context_window)
            .map_or(Value::Null, Value::String),
    )
}

/// Port of `synchronizeModelMetadataDocumentLimits` (`src/model-metadata.ts:841`).
pub fn synchronize_model_metadata_document_limits(
    source: String,
    target_slug: String,
    context_window: String,
    auto_compact_percent: String,
) -> Result<Value, String> {
    Ok(sync_doc_limits_opt(
        &source,
        &target_slug,
        &context_window,
        &auto_compact_percent,
    )
    .map_or(Value::Null, Value::String))
}

/// Port of `synchronizeModelMetadataDocumentLimitsPreview` (`src/model-metadata.ts:869`).
pub fn synchronize_model_metadata_document_limits_preview(
    source: String,
    target_slug: String,
    context_window: String,
    auto_compact_percent: String,
) -> Result<Option<Value>, String> {
    let Some(document) = sync_doc_limits_opt(
        &source,
        &target_slug,
        &context_window,
        &auto_compact_percent,
    ) else {
        return Ok(None);
    };
    let parsed = parse_document_inner(&document, &target_slug);
    if parsed.get("ok").and_then(Value::as_bool) != Some(true) {
        return Ok(None);
    }
    let Some(mut preview) = parsed.get("value").and_then(Value::as_object).cloned() else {
        return Ok(None);
    };
    if !es_trim(&auto_compact_percent).is_empty() {
        let raw_pct = preview
            .get("autoCompactPercent")
            .and_then(Value::as_str)
            .map(ToString::to_string);
        preview.insert(
            "autoCompactPercent".into(),
            display_auto_compact_percent(raw_pct)
                .map(Value::String)
                .unwrap_or(Value::Null),
        );
        preview.insert(
            "autoCompactCalculationPercent".into(),
            Value::String(normalize_auto_compact_percent(&auto_compact_percent)),
        );
    } else {
        preview.insert("autoCompactPercent".into(), Value::String(String::new()));
        preview.insert(
            "autoCompactCalculationPercent".into(),
            Value::String(String::new()),
        );
    }
    Ok(Some(
        json!({ "document": document, "preview": Value::Object(preview) }),
    ))
}

/// Port of `validateModelCapabilities` (`src/model-metadata.ts:902`).
pub fn validate_model_capabilities(metadata: Value) -> Result<Value, String> {
    Ok(validate_capabilities_inner(&metadata)?.map_or(Value::Null, Value::String))
}

/// Port of `parseModelMetadataDocument` (`src/model-metadata.ts:939`).
pub fn parse_model_metadata_document(source: String, target_slug: String) -> Result<Value, String> {
    Ok(parse_document_inner(&source, &target_slug))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suffix_and_row_name_normalization() {
        assert_eq!(suffix_window_tokens("1M".into()), Ok(json!(1_000_000)));
        assert_eq!(suffix_window_tokens("256K".into()), Ok(json!(256_000)));
        assert_eq!(suffix_window_tokens("0K".into()), Ok(Value::Null));
        assert_eq!(
            suffix_window_tokens("9007199254740992".into()),
            Ok(Value::Null)
        );
        assert_eq!(
            model_slug_from_row_name("  DeepSeek-V4-Pro[1M] ".into()),
            Ok("DeepSeek-V4-Pro".into())
        );
        assert_eq!(
            model_metadata_key(" DeepSeek-V4-Pro[1M] ".into()),
            Ok("deepseek-v4-pro".into())
        );
        assert_eq!(
            normalize_token_count_input(" 1m ".into()),
            Ok("1000000".into())
        );
        assert_eq!(normalize_token_count_input("1M2".into()), Ok("12".into()));
        assert_eq!(normalize_token_count_input("0".into()), Ok(String::new()));
    }

    #[test]
    fn bigint_half_up_rounding_and_document_sync() {
        let Ok(Value::String(doc)) = synchronize_model_metadata_document_limits(
            r#"{"slug":"tiny","context_window":3}"#.into(),
            "tiny".into(),
            "3".into(),
            "50%".into(),
        ) else {
            panic!("expected document");
        };
        let Ok(parsed) = parse_json_value(&doc) else {
            panic!("valid json");
        };
        assert_eq!(parsed.get("auto_compact_token_limit"), Some(&json!(2)));

        let Ok(Some(preview)) = synchronize_model_metadata_document_limits_preview(
            r#"{"slug":"model-a","context_window":272000,"auto_compact_token_limit":229376}"#
                .into(),
            "model-a".into(),
            "800000".into(),
            "84.329412%".into(),
        ) else {
            panic!("expected preview");
        };
        assert_eq!(preview["preview"]["autoCompactPercent"], json!("84%"));
        assert_eq!(
            preview["preview"]["autoCompactCalculationPercent"],
            json!("84.329412%")
        );
    }

    #[test]
    fn map_mutations_and_builtin_backfill() {
        let initial = r#"{"DeepSeek-V4-Pro[1M]":{"temperature":0.2}}"#.to_string();
        let replaced = replace_model_metadata_for_slug(
            initial.clone(),
            "deepseek-v4-pro".into(),
            json!({"top_p": 0.8}),
        );
        assert_eq!(replaced, Ok(r#"{"deepseek-v4-pro":{"top_p":0.8}}"#.into()));
        assert_eq!(
            clear_model_metadata_for_slug(
                replaced.unwrap_or_default(),
                "DEEPSEEK-V4-PRO[512K]".into()
            ),
            Ok(String::new())
        );

        let backfill = builtin_row_backfill_value(
            "kimi-k3".into(),
            String::new(),
            String::new(),
            Some(json!({"context_window": 1_048_576, "auto_compact_token_limit": 943_718})),
        );
        assert_eq!(
            backfill,
            Ok(json!({"window": "1048576", "autoCompact": "90%"}))
        );
    }

    #[test]
    fn fast_lite_multi_agent_and_capabilities() {
        let src = r#"{"models":[{"slug":"one","custom":42},{"slug":"two"}]}"#.to_string();
        let Ok(Some(synced)) =
            synchronize_model_metadata_multi_agent_version(src, "one".into(), "v1".into())
        else {
            panic!("expected sync");
        };
        assert_eq!(
            synced["preview"]["metadata"]["multi_agent_version"],
            json!("v1")
        );
        assert_eq!(
            model_fast_support_mode(Some(json!({"service_tiers": []}))),
            Ok("unsupported".into())
        );
        assert_eq!(
            validate_model_capabilities(json!({"multi_agent_version": "auto"})),
            Ok(json!(
                "multi_agent_version 只支持 v1、v2 或 disabled（留空/删除该键表示自动）。"
            ))
        );
        assert!(validate_model_capabilities(json!(null)).is_err());
    }
}
