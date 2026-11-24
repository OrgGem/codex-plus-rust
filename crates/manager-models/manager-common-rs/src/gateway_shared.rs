#![allow(dead_code)]
//! The Rust port of `src/gateway-model.ts`.

#![forbid(unsafe_code)]

use crate::assets_data::presets_value;
use crate::domain::{
    es_trim, is_es_ws, model_slug_from_row_name, normalize_subscription_policy,
    provider_header_inputs, provider_header_rows, provider_headers_validation_message,
};
use crate::js_rt as js;
use crate::js_rt::{
    is_nil, is_plain_object, is_truthy, is_undefined, js_number, js_str, local_datetime_from_ms,
    now_ms, number_to_string, parse_js_number, string_units, to_number, undefined,
};
use chrono::{DateTime, Datelike, Days, Local, NaiveTime};
use serde_json::{Map, Value, json};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use url::Url;

pub const QUEUE_REASON_PREFIX: &str = "local_rate_limit_wait_ms:";
pub const FOREIGN_COMPACTION_DROPPED_REASON: &str = "foreign_compaction_dropped";
pub const COMPACTION_MODEL_REASON_PREFIX: &str = "compaction_model:";
pub const COMPACTION_FALLBACK_REASON_PREFIX: &str = "compaction_fallback:";
pub const GROUP_MODEL_PREFIX: &str = "group/";
pub const MIN_GATEWAY_PORT: f64 = 1024.0;
pub const MAX_GATEWAY_PORT: f64 = 65535.0;
pub const RECENT_USE_MS: f64 = 10.0 * 60.0 * 1000.0;
pub const DIRECT_PROXY: &str = "direct";
pub const GATEWAY_CLAUDE_PROVIDER_ID: &str = "codex-plus-gateway";
pub const BUILTIN_GATEWAY_AGENTS: [&str; 3] = ["codex", "claude", "grok"];

const MONTHS_EN: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

const SPONSORS: [(&str, &str); 5] = [
    ("jojocode-max", "2029-06-15"),
    ("runapi", "2026-12-07"),
    ("apikeyfun", "2026-11-22"),
    ("cccccc-api", "2026-12-01"),
    ("tokenmy", "2026-11-20"),
];

thread_local! {
    static GATEWAY_AGENT_NAMES: RefCell<HashMap<String, String>> = RefCell::new(HashMap::from([
        ("claude".into(), "Claude Code".into()),
        ("grok".into(), "Grok".into()),
        ("codex".into(), "Codex".into()),
    ]));
}

// ── JS interop & numeric helpers ────────────────────────────────────────────

static UNDEF: std::sync::LazyLock<Value> = std::sync::LazyLock::new(undefined);

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

fn const_undef() -> Value {
    UNDEF.clone()
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

fn get_str_opt<'a>(v: &'a Value, key: &str) -> Result<Option<&'a str>, String> {
    match get_opt(v, key)? {
        Some(Value::String(s)) => Ok(Some(s.as_str())),
        Some(_) => Ok(None),
        None => Ok(None),
    }
}

fn req_str_trim(v: &Value, key: &str) -> Result<String, String> {
    let field = get_opt(v, key)?;
    match field {
        Some(Value::String(s)) => Ok(es_trim(s).to_string()),
        Some(x) if !is_nil(x) => Err(format!("TypeError: {key}.trim is not a function")),
        _ => Err(nil_err("trim")),
    }
}

fn req_str<'a>(v: &'a Value, key: &str, method: &str) -> Result<&'a str, String> {
    match get_opt(v, key)? {
        Some(Value::String(s)) => Ok(s.as_str()),
        Some(x) if !is_nil(x) => Err(format!("TypeError: {key}.{method} is not a function")),
        _ => Err(nil_err(method)),
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

fn nil_arr<'a>(v: &'a Value, method: &str) -> Result<&'a [Value], String> {
    if is_nil(v) {
        return Ok(&[]);
    }
    req_arr(v, method)
}

fn val_eq_str(v: &Value, s: &str) -> bool {
    string_units(v).is_some_and(|u| u == s.encode_utf16().collect::<Vec<_>>())
}

fn val_str_lossy(v: &Value) -> Result<String, String> {
    if let Some(u) = string_units(v) {
        return Ok(String::from_utf16_lossy(&u));
    }
    if is_nil(v) {
        return Err(nil_err("toLowerCase"));
    }
    Err("TypeError: value is not a string".into())
}

fn arr_includes_str(arr: &Value, needle: &str) -> Result<bool, String> {
    if is_nil(arr) {
        return Err(nil_err("includes"));
    }
    match arr {
        Value::Array(items) => Ok(items.iter().any(|x| val_eq_str(x, needle))),
        Value::String(s) => Ok(s.contains(needle)),
        _ => Err("TypeError: value.includes is not a function".into()),
    }
}

fn clone_iterable(v: &Value) -> Result<Vec<Value>, String> {
    if is_nil(v) {
        return Err("TypeError: undefined or null is not iterable".into());
    }
    if let Some(arr) = v.as_array() {
        return Ok(arr.clone());
    }
    if let Some(s) = v.as_str() {
        return Ok(s.chars().map(|c| Value::String(c.to_string())).collect());
    }
    if let Some(items) = v
        .as_object()
        .filter(|o| o.len() == 1 && o.contains_key("$set"))
        .and_then(|o| o.get("$set"))
        .and_then(Value::as_array)
    {
        return Ok(items.clone());
    }
    Err("TypeError: value is not iterable".into())
}

fn js_to_fixed(x: f64, digits: u32) -> String {
    if x.is_nan() {
        return "NaN".into();
    }
    if x.is_infinite() {
        return if x.is_sign_negative() {
            "-Infinity"
        } else {
            "Infinity"
        }
        .into();
    }
    if x.abs() >= 1e21 {
        return number_to_string(x);
    }
    if x == 0.0 {
        return match digits {
            0 => "0",
            1 => "0.0",
            _ => "0.00",
        }
        .into();
    }
    let neg = x < 0.0;
    let bits = x.abs().to_bits();
    let exp_bits = ((bits >> 52) & 0x7ff) as i32;
    let frac = bits & ((1u64 << 52) - 1);
    let (mant, exp) = if exp_bits == 0 {
        (u128::from(frac), -1074i32)
    } else {
        (u128::from((1u64 << 52) | frac), exp_bits - 1075)
    };
    let scale = 10u128.pow(digits);
    let q = if exp >= 0 {
        (mant << (exp as u32)) * scale
    } else {
        let shift = (-exp) as u32;
        if shift >= 65 {
            0
        } else {
            let scaled = mant * scale;
            let rem_mask = (1u128 << shift) - 1;
            let half = 1u128 << (shift - 1);
            let mut q = scaled >> shift;
            if (scaled & rem_mask) >= half {
                q += 1;
            }
            q
        }
    };
    let sign = if neg { "-" } else { "" };
    match digits {
        0 => format!("{sign}{q}"),
        1 => format!("{sign}{}.{}", q / 10, q % 10),
        _ => format!("{sign}{}.{:02}", q / 100, q % 100),
    }
}

fn js_to_precision_2(x: f64) -> String {
    if x.is_nan() {
        return "NaN".into();
    }
    if x.is_infinite() {
        return if x.is_sign_negative() {
            "-Infinity"
        } else {
            "Infinity"
        }
        .into();
    }
    if x == 0.0 {
        return "0.0".into();
    }
    let neg = x < 0.0;
    let sci = format!("{:.25e}", x.abs());
    let (mant, exp_str) = sci.split_once('e').unwrap_or((&sci, "0"));
    let mut e = exp_str.parse::<i32>().unwrap_or(0);
    let digits: Vec<u8> = mant
        .bytes()
        .filter(|b| b.is_ascii_digit())
        .map(|b| b - b'0')
        .collect();
    let (mut d0, mut d1) = (*digits.first().unwrap_or(&0), *digits.get(1).unwrap_or(&0));
    if *digits.get(2).unwrap_or(&0) >= 5 {
        d1 += 1;
        if d1 >= 10 {
            d1 = 0;
            d0 += 1;
            if d0 >= 10 {
                d0 = 1;
                e += 1;
            }
        }
    }
    let sign = if neg { "-" } else { "" };
    if !(-6..2).contains(&e) {
        return format!(
            "{sign}{d0}.{d1}e{}{}",
            if e < 0 { "-" } else { "+" },
            e.abs()
        );
    }
    if e == 1 {
        format!("{sign}{d0}{d1}")
    } else if e == 0 {
        format!("{sign}{d0}.{d1}")
    } else {
        let zeros = "0".repeat((-e - 1) as usize);
        format!("{sign}0.{zeros}{d0}{d1}")
    }
}

fn js_min(a: f64, b: f64) -> f64 {
    if a.is_nan() || b.is_nan() {
        f64::NAN
    } else if a < b {
        a
    } else {
        b
    }
}

fn js_max(a: f64, b: f64) -> f64 {
    if a.is_nan() || b.is_nan() {
        f64::NAN
    } else if a > b {
        a
    } else {
        b
    }
}

fn date_marker(v: &Value) -> Option<&str> {
    v.as_object()
        .filter(|o| o.len() == 1)
        .and_then(|o| o.get("$date"))
        .and_then(Value::as_str)
}

fn parse_now_ms_val(v: &Value) -> f64 {
    if is_undefined(v) {
        return now_ms();
    }
    if let Some(iso) = date_marker(v) {
        if iso == "Invalid" {
            return f64::NAN;
        }
        return DateTime::parse_from_rfc3339(iso)
            .map(|dt| dt.timestamp_millis() as f64)
            .unwrap_or(f64::NAN);
    }
    to_number(v)
}

fn parse_date_arg(now: &Value) -> Result<Option<DateTime<Local>>, String> {
    if is_undefined(now) {
        return Ok(Some(Local::now()));
    }
    if now.is_null() {
        return Err("TypeError: Cannot read properties of null (reading 'getFullYear')".into());
    }
    match date_marker(now) {
        Some("Invalid") => Ok(None),
        Some(iso) => Ok(DateTime::parse_from_rfc3339(iso)
            .ok()
            .map(|dt| dt.with_timezone(&Local))),
        None => Err("TypeError: now.getFullYear is not a function".into()),
    }
}

fn local_day_str(dt: Option<DateTime<Local>>) -> String {
    match dt {
        Some(d) => format!("{:04}-{:02}-{:02}", d.year(), d.month(), d.day()),
        None => "NaN-NaN-NaN".into(),
    }
}

fn is_line_term(c: char) -> bool {
    matches!(c, '\n' | '\r' | '\u{2028}' | '\u{2029}')
}

// ── Exports ─────────────────────────────────────────────────────────────────

/// Port of `keyLabel` (`src/gateway-model.ts:146`).
pub fn key_label(keys: Value, key_id: String) -> Result<String, String> {
    let list = nil_arr(&keys, "find")?;
    let mut found: Option<&Value> = None;
    for item in list {
        if is_nil(item) {
            return Err(nil_err("id"));
        }
        if get_str_opt(item, "id")? == Some(key_id.as_str()) {
            found = Some(item);
            break;
        }
    }
    if found.is_none() && key_id == "primary" && list.len() == 1 {
        found = Some(&list[0]);
    }
    match found {
        Some(item) => Ok(get_str_opt(item, "masked")?.unwrap_or("").to_string()),
        None => Ok(key_id),
    }
}

/// Port of `imageModelLabel` (`src/gateway-model.ts:349`).
pub fn image_model_label(model: String, vendor_picks: bool) -> Result<String, String> {
    Ok(if vendor_picks {
        "GPT Image".into()
    } else {
        model
    })
}

/// Port of `blankBinding` (`src/gateway-model.ts:374`).
pub fn blank_binding() -> Result<Value, String> {
    Ok(json!({
        "enabled": false,
        "model": "",
        "extraModels": [],
        "claudeRoles": { "opus": "", "sonnet": "", "haiku": "", "subagent": "" },
        "unknownModel": "mapToDefault",
        "contextWindow": null,
        "claudeEnableEffort": false,
        "token": "",
    }))
}

/// Port of `draftFromBinding` (`src/gateway-model.ts:387`).
pub fn draft_from_binding(binding: Value) -> Result<Value, String> {
    let owned_blank;
    let source = if is_nil(&binding) {
        owned_blank = blank_binding()?;
        &owned_blank
    } else {
        &binding
    };
    let extra = req_arr(prop(source, "extraModels")?, "join")?
        .iter()
        .map(|v| if is_nil(v) { String::new() } else { js_str(v) })
        .collect::<Vec<_>>()
        .join("\n");
    let roles = prop(source, "claudeRoles")?;
    if is_nil(roles) {
        return Err(nil_err("opus"));
    }
    let cw = prop(source, "contextWindow")?;
    let context_window = if is_truthy(cw) {
        js_str(cw)
    } else {
        String::new()
    };
    let pool_exclude = match get_opt(source, "poolExclude")? {
        None | Some(Value::Null) => Vec::new(),
        Some(v) => clone_iterable(v)?,
    };
    let auto_join = get_opt(source, "autoJoinProviders")? != Some(&Value::Bool(false));
    let str_or_empty = |k: &str| -> Result<Value, String> {
        Ok(match get_opt(source, k)? {
            None | Some(Value::Null) => Value::String(String::new()),
            Some(v) => v.clone(),
        })
    };
    Ok(json!({
        "model": prop(source, "model")?,
        "extraModels": extra,
        "opus": prop(roles, "opus")?,
        "sonnet": prop(roles, "sonnet")?,
        "haiku": prop(roles, "haiku")?,
        "subagent": prop(roles, "subagent")?,
        "unknownModel": prop(source, "unknownModel")?,
        "autoReviewModel": str_or_empty("autoReviewModel")?,
        "titleModel": str_or_empty("titleModel")?,
        "compactionModel": str_or_empty("compactionModel")?,
        "compactionReasoningEffort": str_or_empty("compactionReasoningEffort")?,
        "contextWindow": context_window,
        "claudeEnableEffort": prop(source, "claudeEnableEffort")?,
        "poolExclude": pool_exclude,
        "autoJoinProviders": auto_join,
    }))
}

/// Port of `defaultProviderAgents` (`src/gateway-model.ts:409`).
pub fn default_provider_agents(payload: Value) -> Result<Vec<Value>, String> {
    let agents_val = if is_nil(&payload) {
        Value::Null
    } else {
        prop(&payload, "agents")?.clone()
    };
    let glanced = glance_agents(agents_val)?;
    let mut out = Vec::new();
    for item in glanced {
        let agent = prop(&item, "agent")?.clone();
        if is_nil(&payload) {
            out.push(agent);
            continue;
        }
        let gw = prop(&payload, "gateway")?;
        let bindings = prop(gw, "bindings")?;
        if is_nil(bindings) {
            return Err(nil_err("binding"));
        }
        let key = js_str(&agent);
        let auto_join = match get_opt(bindings, &key)? {
            None | Some(Value::Null) => true,
            Some(b) => get_opt(b, "autoJoinProviders")? != Some(&Value::Bool(false)),
        };
        if auto_join {
            out.push(agent);
        }
    }
    Ok(out)
}

/// Port of `skippedAgents` (`src/gateway-model.ts:416`).
pub fn skipped_agents(payload: Value, picked: Value) -> Result<Vec<Value>, String> {
    let agents_val = if is_nil(&payload) {
        Value::Null
    } else {
        prop(&payload, "agents")?.clone()
    };
    let glanced = glance_agents(agents_val)?;
    let mut out = Vec::new();
    for item in glanced {
        let agent = prop(&item, "agent")?.clone();
        let needle = js_str(&agent);
        if !arr_includes_str(&picked, &needle)? {
            out.push(agent);
        }
    }
    Ok(out)
}

/// Port of `poolExcludeToggle` (`src/gateway-model.ts:421`).
pub fn pool_exclude_toggle(
    previous: Value,
    provider_id: String,
    on: bool,
) -> Result<Vec<String>, String> {
    let list = nil_arr(&previous, if on { "filter" } else { "includes" })?;
    let mut strs = Vec::with_capacity(list.len());
    for v in list {
        strs.push(val_str_lossy(v)?);
    }
    if on {
        Ok(strs.into_iter().filter(|id| id != &provider_id).collect())
    } else if strs.iter().any(|id| id == &provider_id) {
        Ok(strs)
    } else {
        strs.push(provider_id);
        Ok(strs)
    }
}

fn parse_context_window_f64(value: &str) -> Option<f64> {
    let text = es_trim(value).to_lowercase();
    if text.is_empty() {
        return None;
    }
    let bytes = text.as_bytes();
    let mut i = 0;
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        i += 1;
    }
    if i == 0 {
        return Some(f64::NAN);
    }
    if i < bytes.len() && bytes[i] == b'.' {
        let dot = i;
        i += 1;
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            i += 1;
        }
        if i == dot + 1 {
            return Some(f64::NAN);
        }
    }
    let num_part = &text[..i];
    let mut rest = &text[i..];
    rest = rest.trim_start_matches(is_es_ws);
    let scale = match rest {
        "" => 1.0,
        "k" => 1_000.0,
        "m" => 1_000_000.0,
        _ => return Some(f64::NAN),
    };
    let base = num_part.parse::<f64>().unwrap_or(f64::NAN);
    Some(js::math_round(base * scale))
}

/// Port of `parseContextWindow` (`src/gateway-model.ts:428`).
pub fn parse_context_window(value: String) -> Result<Value, String> {
    Ok(match parse_context_window_f64(&value) {
        Some(n) => js_number(n),
        None => Value::Null,
    })
}

/// Port of `parseKeyLimit` (`src/gateway-model.ts:439`).
pub fn parse_key_limit(value: String) -> Result<Value, String> {
    let text = es_trim(&value);
    if text.is_empty() {
        return Ok(Value::Null);
    }
    if !text.bytes().all(|b| b.is_ascii_digit()) {
        return Ok(js_number(f64::NAN));
    }
    let limit = text.parse::<f64>().unwrap_or(f64::INFINITY);
    Ok(if limit > 0.0 && limit <= 1_000_000.0 {
        js_number(limit)
    } else {
        js_number(f64::NAN)
    })
}

fn parse_compaction_note_inner(reason: &str) -> Option<Value> {
    if let Some(model) = reason.strip_prefix(COMPACTION_MODEL_REASON_PREFIX) {
        return Some(json!({ "kind": "model", "model": model }));
    }
    if let Some(rest) = reason.strip_prefix(COMPACTION_FALLBACK_REASON_PREFIX) {
        return Some(match rest.split_once(':') {
            None => json!({ "kind": "fallback", "reason": rest, "model": "" }),
            Some((r, m)) => json!({ "kind": "fallback", "reason": r, "model": m }),
        });
    }
    None
}

/// Port of `parseCompactionNote` (`src/gateway-model.ts:466`).
pub fn parse_compaction_note(reason: String) -> Result<Value, String> {
    Ok(parse_compaction_note_inner(&reason).unwrap_or(Value::Null))
}

/// Port of `compactionNote` (`src/gateway-model.ts:479`).
pub fn compaction_note(reasons: Value) -> Result<Value, String> {
    let items = if is_nil(&reasons) {
        Vec::new()
    } else {
        clone_iterable(&reasons)?
    };
    for item in &items {
        let s = match item.as_str() {
            Some(s) => s,
            None => return Err("TypeError: reason.startsWith is not a function".into()),
        };
        if let Some(note) = parse_compaction_note_inner(s) {
            return Ok(note);
        }
    }
    Ok(Value::Null)
}

/// Port of `foreignCompactionDropped` (`src/gateway-model.ts:488`).
pub fn foreign_compaction_dropped(reasons: Value) -> Result<bool, String> {
    if is_nil(&reasons) {
        return Ok(false);
    }
    Ok(req_arr(&reasons, "includes")?
        .iter()
        .any(|v| val_eq_str(v, FOREIGN_COMPACTION_DROPPED_REASON)))
}

/// Port of `queueWaitMs` (`src/gateway-model.ts:493`).
pub fn queue_wait_ms(reasons: Value) -> Result<Value, String> {
    for item in nil_arr(&reasons, "find")? {
        let s = match item.as_str() {
            Some(s) => s,
            None => return Err("TypeError: item.startsWith is not a function".into()),
        };
        if let Some(rest) = s.strip_prefix(QUEUE_REASON_PREFIX) {
            let ms = parse_js_number(rest);
            return Ok(if ms.is_finite() && ms >= 0.0 {
                js_number(ms)
            } else {
                Value::Null
            });
        }
    }
    Ok(Value::Null)
}

/// Port of `formatQueueWait` (`src/gateway-model.ts:501`).
pub fn format_queue_wait(ms: f64) -> Result<String, String> {
    Ok(if ms < 10_000.0 {
        format!("{}s", js_to_fixed(ms / 1000.0, 1))
    } else {
        format!("{}s", number_to_string(js::math_round(ms / 1000.0)))
    })
}

fn split_models_inner(text: &str) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    for part in text.split(['\n', ',']) {
        let line = es_trim(part);
        if !line.is_empty() && seen.insert(line.to_string()) {
            out.push(line.to_string());
        }
    }
    out
}

/// Port of `splitModels` (`src/gateway-model.ts:505`).
pub fn split_models(text: String) -> Result<Vec<String>, String> {
    Ok(split_models_inner(&text))
}

/// Port of `bindingFromDraft` (`src/gateway-model.ts:518`).
pub fn binding_from_draft(draft: Value, previous: Value, item: Value) -> Result<Value, String> {
    let owned_blank;
    let base = if is_nil(&previous) {
        owned_blank = blank_binding()?;
        &owned_blank
    } else {
        &previous
    };
    let cw_raw = req_str(&draft, "contextWindow", "trim")?;
    let window = parse_context_window_f64(cw_raw);
    let model = req_str_trim(&draft, "model")?;
    let default_sel = if is_truthy(&item) {
        get_opt(&item, "defaultModelSelection")?
            .filter(|v| !is_nil(v))
            .cloned()
            .unwrap_or(Value::Null)
    } else {
        let base_model_trim = req_str_trim(base, "model")?;
        if model.to_lowercase() == base_model_trim.to_lowercase() {
            get_opt(base, "defaultModelSelection")?
                .filter(|v| !is_nil(v))
                .cloned()
                .unwrap_or(Value::Null)
        } else {
            Value::Null
        }
    };
    let extra_models: Vec<Value> = split_models_inner(req_str(&draft, "extraModels", "split")?)
        .into_iter()
        .map(Value::String)
        .collect();
    let claude_roles = json!({
        "opus": req_str_trim(&draft, "opus")?,
        "sonnet": req_str_trim(&draft, "sonnet")?,
        "haiku": req_str_trim(&draft, "haiku")?,
        "subagent": req_str_trim(&draft, "subagent")?,
    });
    let unknown_model = prop(&draft, "unknownModel")?.clone();
    let auto_review = req_str_trim(&draft, "autoReviewModel")?;
    let title_model = req_str_trim(&draft, "titleModel")?;
    let compaction_model = req_str_trim(&draft, "compactionModel")?;
    let compaction_effort = req_str_trim(&draft, "compactionReasoningEffort")?;
    let ctx_val = match window {
        Some(w) if w != 0.0 && w.is_finite() => js_number(w),
        _ => Value::Null,
    };
    let claude_effort = prop(&draft, "claudeEnableEffort")?.clone();
    let pool_exclude = clone_iterable(prop(&draft, "poolExclude")?)?;
    let auto_join = prop(&draft, "autoJoinProviders")?.clone();

    let mut out = Map::new();
    match base {
        Value::Object(m) if is_plain_object(base) => {
            for (k, v) in m {
                out.insert(k.clone(), v.clone());
            }
        }
        Value::Array(arr) => {
            for (i, v) in arr.iter().enumerate() {
                if !is_undefined(v) {
                    out.insert(i.to_string(), v.clone());
                }
            }
        }
        Value::String(s) => {
            for (i, c) in s.chars().enumerate() {
                out.insert(i.to_string(), Value::String(c.to_string()));
            }
        }
        _ => {}
    }
    out.insert("model".into(), Value::String(model));
    out.insert("defaultModelSelection".into(), default_sel);
    out.insert("extraModels".into(), Value::Array(extra_models));
    out.insert("claudeRoles".into(), claude_roles);
    out.insert("unknownModel".into(), unknown_model);
    out.insert("autoReviewModel".into(), Value::String(auto_review));
    out.insert("titleModel".into(), Value::String(title_model));
    out.insert("compactionModel".into(), Value::String(compaction_model));
    out.insert(
        "compactionReasoningEffort".into(),
        Value::String(compaction_effort),
    );
    out.insert("contextWindow".into(), ctx_val);
    out.insert("claudeEnableEffort".into(), claude_effort);
    out.insert("poolExclude".into(), Value::Array(pool_exclude));
    out.insert("autoJoinProviders".into(), auto_join);
    Ok(Value::Object(out))
}

/// Port of `groupModelId` (`src/gateway-model.ts:550`).
pub fn group_model_id(id: String) -> Result<String, String> {
    Ok(format!("{GROUP_MODEL_PREFIX}{id}"))
}

fn disabled_group_opt<'a>(
    payload: &'a Value,
    model: &str,
    selection: &Value,
) -> Result<Option<&'a Value>, String> {
    let key = model_slug_from_row_name(model).to_lowercase();
    if is_truthy(selection) {
        let pid = js_str(prop(selection, "providerId")?);
        let up = req_str(selection, "upstreamModel", "trim")?;
        let sel_key = format!("{pid}/{}", model_slug_from_row_name(up)).to_lowercase();
        if key == sel_key {
            return Ok(None);
        }
    }
    if is_nil(payload) {
        return Ok(None);
    }
    let groups = req_arr(prop(payload, "groups")?, "find")?;
    for g in groups {
        if is_nil(g) {
            return Err(nil_err("disabled"));
        }
        if is_truthy(prop(g, "disabled")?) {
            let gid = js_str(prop(g, "id")?);
            if format!("{GROUP_MODEL_PREFIX}{gid}").to_lowercase() == key {
                return Ok(Some(g));
            }
        }
    }
    Ok(None)
}

/// Port of `disabledGroup` (`src/gateway-model.ts:554`).
pub fn disabled_group(payload: Value, model: String, selection: Value) -> Result<Value, String> {
    Ok(disabled_group_opt(&payload, &model, &selection)?
        .cloned()
        .unwrap_or_else(undefined))
}

/// Port of `slugify` (`src/gateway-model.ts:561`).
pub fn slugify(name: String) -> Result<String, String> {
    let lower = es_trim(&name).to_lowercase();
    let mut buf = String::with_capacity(lower.len());
    let mut in_dash = false;
    for c in lower.chars() {
        if is_es_ws(c) || c == '_' || c == '/' {
            if !in_dash {
                buf.push('-');
                in_dash = true;
            }
        } else {
            buf.push(c);
            in_dash = false;
        }
    }
    let slug = buf.trim_matches('-');
    Ok(if slug.is_empty() {
        "unnamed".into()
    } else {
        slug.into()
    })
}

/// Port of `providerEnabled` (`src/gateway-model.ts:567`).
pub fn provider_enabled(entry: Value) -> Result<bool, String> {
    Ok(prop(&entry, "enabled")? != &Value::Bool(false))
}

fn pool_providers_refs<'a>(payload: &'a Value, exclude: &Value) -> Result<Vec<&'a Value>, String> {
    let pool = if is_nil(payload) {
        &[]
    } else {
        match get_opt(payload, "pool")? {
            None | Some(Value::Null) => &[],
            Some(v) => req_arr(v, "filter")?,
        }
    };
    let mut out = Vec::new();
    for entry in pool {
        if provider_enabled(entry.clone())? {
            let id = js_str(prop(entry, "id")?);
            if !arr_includes_str(exclude, &id)? {
                out.push(entry);
            }
        }
    }
    Ok(out)
}

fn default_exclude(exclude: Value) -> Value {
    if is_undefined(&exclude) {
        Value::Array(Vec::new())
    } else {
        exclude
    }
}

/// Port of `agentPoolProviders` (`src/gateway-model.ts:572`).
pub fn agent_pool_providers(payload: Value, exclude: Value) -> Result<Vec<Value>, String> {
    let excl = default_exclude(exclude);
    Ok(pool_providers_refs(&payload, &excl)?
        .into_iter()
        .cloned()
        .collect())
}

fn agent_models_vec(payload: &Value, exclude: &Value) -> Result<Vec<String>, String> {
    let mut models = Vec::new();
    let mut seen = HashSet::new();
    let undef = undefined();
    for entry in pool_providers_refs(payload, exclude)? {
        let entry_id = js_str(prop(entry, "id")?);
        for listed_v in req_arr(prop(entry, "models")?, "Symbol.iterator")? {
            let listed = val_str_lossy(listed_v)?;
            let is_dis = disabled_group_opt(payload, &listed, &undef)?.is_some();
            let model = if is_dis {
                format!("{entry_id}/{listed}")
            } else {
                listed
            };
            let key = model.to_lowercase();
            if seen.insert(key) {
                models.push(model);
            }
        }
    }
    let groups = if is_nil(payload) {
        &[]
    } else {
        match get_opt(payload, "groups")? {
            None | Some(Value::Null) => &[],
            Some(v) => req_arr(v, "Symbol.iterator")?,
        }
    };
    for group in groups {
        if !is_truthy(prop(group, "disabled")?)
            && !req_arr(prop(group, "members")?, "length")?.is_empty()
        {
            models.push(format!(
                "{GROUP_MODEL_PREFIX}{}",
                js_str(prop(group, "id")?)
            ));
        }
    }
    Ok(models)
}

/// Port of `agentModels` (`src/gateway-model.ts:577`).
pub fn agent_models(payload: Value, exclude: Value) -> Result<Vec<String>, String> {
    let excl = default_exclude(exclude);
    agent_models_vec(&payload, &excl)
}

fn serving_providers_refs<'a>(
    payload: &'a Value,
    model: &str,
    exclude: &Value,
) -> Result<Vec<&'a Value>, String> {
    let name = es_trim(model).to_lowercase();
    let mut out = Vec::new();
    for entry in pool_providers_refs(payload, exclude)? {
        let mut hit = false;
        for known in req_arr(prop(entry, "models")?, "some")? {
            if val_str_lossy(known)?.to_lowercase() == name {
                hit = true;
                break;
            }
        }
        if hit {
            out.push(entry);
        }
    }
    Ok(out)
}

fn qualified_provider_val(
    payload: &Value,
    model: &str,
    exclude: &Value,
) -> Result<Option<Value>, String> {
    if model.starts_with(GROUP_MODEL_PREFIX) {
        return Ok(None);
    }
    if !serving_providers_refs(payload, model, exclude)?.is_empty() {
        return Ok(None);
    }
    let Some((prefix, rest)) = model.split_once('/') else {
        return Ok(None);
    };
    let rest_trim = es_trim(rest);
    if prefix.is_empty() || rest_trim.is_empty() {
        return Ok(None);
    }
    let prefix_lo = prefix.to_lowercase();
    for item in pool_providers_refs(payload, exclude)? {
        if val_str_lossy(prop(item, "id")?)?.to_lowercase() == prefix_lo {
            return Ok(Some(json!({ "entry": item, "model": rest_trim })));
        }
    }
    Ok(None)
}

/// Port of `qualifiedProvider` (`src/gateway-model.ts:597`).
pub fn qualified_provider(payload: Value, model: String, exclude: Value) -> Result<Value, String> {
    let excl = default_exclude(exclude);
    Ok(qualified_provider_val(&payload, &model, &excl)?.unwrap_or(Value::Null))
}

fn model_routable_bool(payload: &Value, model: &str, exclude: &Value) -> Result<bool, String> {
    let undef = undefined();
    if disabled_group_opt(payload, model, &undef)?.is_some() {
        return Ok(false);
    }
    let trimmed = es_trim(model);
    let lower = trimmed.to_lowercase();
    if agent_models_vec(payload, exclude)?
        .into_iter()
        .any(|known| known.to_lowercase() == lower)
    {
        return Ok(true);
    }
    Ok(qualified_provider_val(payload, trimmed, exclude)?.is_some())
}

/// Port of `modelRoutable` (`src/gateway-model.ts:608`).
pub fn model_routable(payload: Value, model: String, exclude: Value) -> Result<bool, String> {
    let excl = default_exclude(exclude);
    model_routable_bool(&payload, &model, &excl)
}

/// Port of `validateDraft` (`src/gateway-model.ts:615`).
pub fn validate_draft(
    draft: Value,
    payload: Value,
    retained_model: Value,
    retained_selection: Value,
) -> Result<Value, String> {
    if is_truthy(&payload) {
        let pool_exclude = prop(&draft, "poolExclude")?;
        if pool_providers_refs(&payload, pool_exclude)?.is_empty() {
            return Ok(Value::String("供应商池是空的，先到「供应商」里添加".into()));
        }
        let raw_model = req_str(&draft, "model", "trim")?;
        let model = es_trim(raw_model);
        let model_matches_retained = val_eq_str(&retained_model, raw_model);
        let undef = undefined();
        let retained_disabled =
            model_matches_retained && disabled_group_opt(&payload, model, &undef)?.is_some();
        let retained_provider = if model_matches_retained && is_truthy(&retained_selection) {
            let pid = prop(&retained_selection, "providerId")?;
            let pool = req_arr(prop(&payload, "pool")?, "find")?;
            let mut found = None;
            for e in pool {
                if prop(e, "id")? == pid {
                    found = Some(e);
                    break;
                }
            }
            found
        } else {
            None
        };
        let retained_unavailable = if model_matches_retained && is_truthy(&retained_selection) {
            let sel_pid = js_str(prop(&retained_selection, "providerId")?);
            let sel_up = req_str(&retained_selection, "upstreamModel", "trim")?;
            let slug_matches = model_slug_from_row_name(model).to_lowercase()
                == format!("{sel_pid}/{}", model_slug_from_row_name(sel_up)).to_lowercase();
            if slug_matches {
                if is_truthy(prop(&retained_selection, "providerRemoved")?) {
                    true
                } else if let Some(rp) = retained_provider {
                    !provider_enabled(rp.clone())?
                        || arr_includes_str(pool_exclude, &js_str(prop(rp, "id")?))?
                } else {
                    true
                }
            } else {
                false
            }
        } else {
            false
        };
        if !model.is_empty()
            && !retained_disabled
            && !retained_unavailable
            && !model_routable_bool(&payload, model, pool_exclude)?
        {
            return Ok(Value::String("供应商池里没有这个模型".into()));
        }
    }
    let window = parse_context_window_f64(req_str(&draft, "contextWindow", "trim")?);
    if let Some(w) = window
        && (!w.is_finite() || w < 1_000.0)
    {
        return Ok(Value::String(
            "上下文窗口格式不对，例如 200000、200k 或 1M".into(),
        ));
    }
    Ok(Value::Null)
}

/// Port of `protocolLabel` (`src/gateway-model.ts:631`).
pub fn protocol_label(protocol: Value) -> Result<String, String> {
    Ok(match protocol.as_str() {
        Some("chatCompletions") => "Chat",
        Some("anthropicMessages") => "Anthropic",
        _ => "Responses",
    }
    .into())
}

fn failure_label_str(value: Option<&str>) -> &'static str {
    match value {
        Some("rate_limited") => "被限流",
        Some("quota_exhausted") => "额度耗尽",
        Some("auth_invalid") => "Key 无效",
        Some("server_error") => "上游出错",
        Some("failover") => "已切换",
        Some("client_error") => "请求被拒",
        _ => "",
    }
}

/// Port of `failureLabel` (`src/gateway-model.ts:643`).
pub fn failure_label(value: Option<String>) -> Result<String, String> {
    Ok(failure_label_str(value.as_deref()).into())
}

fn format_remaining_f64(ms: Option<f64>) -> String {
    let Some(ms) = ms.filter(|m| m.is_finite() && *m > 0.0) else {
        if ms == Some(f64::INFINITY) {
            return "Infinityd".into();
        }
        return String::new();
    };
    let seconds = (ms / 1000.0).ceil();
    if seconds < 60.0 {
        return format!("{}s", number_to_string(seconds));
    }
    let minutes = (seconds / 60.0).ceil();
    if minutes < 60.0 {
        return format!("{}m", number_to_string(minutes));
    }
    let hours = (minutes / 60.0).floor();
    if hours >= 24.0 {
        let days = (hours / 24.0).floor();
        let rem_h = hours % 24.0;
        let suffix = if rem_h != 0.0 && !rem_h.is_nan() {
            format!("{}h", number_to_string(rem_h))
        } else {
            String::new()
        };
        return format!("{}d{suffix}", number_to_string(days));
    }
    let rem_m = minutes % 60.0;
    let suffix = if rem_m != 0.0 && !rem_m.is_nan() {
        format!("{}m", number_to_string(rem_m))
    } else {
        String::new()
    };
    format!("{}h{suffix}", number_to_string(hours))
}

/// Port of `formatRemaining` (`src/gateway-model.ts:662`).
pub fn format_remaining(ms: Option<f64>) -> Result<String, String> {
    Ok(format_remaining_f64(ms))
}

/// Port of `glanceAgents` (`src/gateway-model.ts:685`).
pub fn glance_agents(agents: Value) -> Result<Vec<Value>, String> {
    let mut out = Vec::new();
    for item in nil_arr(&agents, "filter")? {
        if is_truthy(prop(item, "enabled")?)
            || get_str_opt(item, "agent")?.is_some_and(|a| BUILTIN_GATEWAY_AGENTS.contains(&a))
        {
            out.push(item.clone());
        }
    }
    Ok(out)
}

fn agent_name_str(agent: &str) -> String {
    GATEWAY_AGENT_NAMES.with(|m| {
        m.borrow()
            .get(agent)
            .cloned()
            .unwrap_or_else(|| agent.to_string())
    })
}

/// Port of `agentName` (`src/gateway-model.ts:689`).
pub fn agent_name(agent: String) -> Result<String, String> {
    Ok(agent_name_str(&agent))
}

/// Port of `rememberAgentNames` (`src/gateway-model.ts:694`).
pub fn remember_agent_names(agents: Value) -> Result<Value, String> {
    let items = if is_nil(&agents) {
        Vec::new()
    } else {
        clone_iterable(&agents)?
    };
    for item in &items {
        let name = prop(item, "name")?;
        if is_truthy(name) {
            let key = js_str(prop(item, "agent")?);
            let name_str = js_str(name);
            GATEWAY_AGENT_NAMES.with(|m| {
                m.borrow_mut().entry(key).or_insert(name_str);
            });
        }
    }
    Ok(undefined())
}

/// Port of `visibleOwnModels` (`src/gateway-model.ts:802`).
pub fn visible_own_models(agent: Value, payload: Value) -> Result<Value, String> {
    let own = prop(&agent, "ownModels")?;
    if !is_truthy(own) {
        return Ok(Value::Null);
    }
    let agent_id = js_str(prop(&agent, "agent")?);
    let hide = if is_nil(&payload) {
        &[][..]
    } else {
        let gw = prop(&payload, "gateway")?;
        match get_opt(gw, "hideOwnModels")? {
            None | Some(Value::Null) => &[][..],
            Some(v) => req_arr(v, "includes")?,
        }
    };
    if !hide.iter().any(|x| val_eq_str(x, &agent_id)) {
        return Ok(own.clone());
    }
    let current = req_str_trim(own, "current")?.to_lowercase();
    let models = if current.is_empty() {
        Vec::new()
    } else {
        let mut kept = Vec::new();
        for m in req_arr(prop(own, "models")?, "filter")? {
            if req_str(m, "id", "toLowerCase")?.to_lowercase() == current {
                kept.push(m.clone());
            }
        }
        kept
    };
    if models.is_empty() {
        return Ok(Value::Null);
    }
    let mut out = Map::new();
    if let Some(m) = own.as_object() {
        for (k, v) in m {
            out.insert(k.clone(), v.clone());
        }
    }
    out.insert("models".into(), Value::Array(models));
    Ok(Value::Object(out))
}

/// Port of `servingProviders` (`src/gateway-model.ts:826`).
pub fn serving_providers(
    payload: Value,
    model: String,
    exclude: Value,
) -> Result<Vec<Value>, String> {
    let excl = default_exclude(exclude);
    Ok(serving_providers_refs(&payload, &model, &excl)?
        .into_iter()
        .cloned()
        .collect())
}

fn group_display_name_opt(payload: &Value, model: &str) -> Result<Option<Value>, String> {
    if is_nil(payload) {
        return Ok(None);
    }
    let key = es_trim(model).to_lowercase();
    for g in req_arr(prop(payload, "groups")?, "find")? {
        let gid = js_str(prop(g, "id")?);
        if format!("{GROUP_MODEL_PREFIX}{gid}").to_lowercase() == key {
            return Ok(Some(prop(g, "name")?.clone()));
        }
    }
    Ok(None)
}

/// Port of `groupDisplayName` (`src/gateway-model.ts:832`).
pub fn group_display_name(payload: Value, model: String) -> Result<Value, String> {
    Ok(group_display_name_opt(&payload, &model)?.unwrap_or_else(undefined))
}

/// Port of `modelDisplay` (`src/gateway-model.ts:838`).
pub fn model_display(
    payload: Value,
    model: String,
    exclude: Value,
    selection: Value,
) -> Result<Value, String> {
    let excl = default_exclude(exclude);
    let trimmed = es_trim(&model);
    let title = if !trimmed.is_empty() {
        trimmed.to_string()
    } else {
        agent_models_vec(&payload, &excl)?
            .into_iter()
            .next()
            .unwrap_or_default()
    };
    let selected_model = model_slug_from_row_name(&title);
    if is_truthy(&selection) {
        let sel_pid = js_str(prop(&selection, "providerId")?);
        let sel_up = req_str(&selection, "upstreamModel", "trim")?;
        let sel_key = format!("{sel_pid}/{}", model_slug_from_row_name(sel_up)).to_lowercase();
        if selected_model.to_lowercase() == sel_key {
            let provider = if is_nil(&payload) {
                None
            } else {
                let pool = req_arr(prop(&payload, "pool")?, "find")?;
                let pid_val = prop(&selection, "providerId")?;
                pool.iter().find(|e| e.get("id") == Some(pid_val))
            };
            let unavailable = is_truthy(prop(&selection, "providerRemoved")?)
                || provider.is_none()
                || !provider.is_some_and(|p| provider_enabled(p.clone()).unwrap_or(false))
                || arr_includes_str(
                    &excl,
                    &js_str(prop(provider.unwrap_or(&Value::Null), "id").unwrap_or(&Value::Null)),
                )?
                || {
                    let p = provider.unwrap_or(&Value::Null);
                    let p_src = get_opt(p, "source")?
                        .filter(|v| !is_nil(v))
                        .map(js_str)
                        .unwrap_or_default();
                    let s_src = prop(&selection, "providerSource")?;
                    !val_eq_str(s_src, &p_src)
                };
            let sel_lo = selected_model.to_lowercase();
            let conflict = sel_lo.starts_with(GROUP_MODEL_PREFIX)
                || selected_model.contains('*')
                || !serving_providers_refs(&payload, &selected_model, &excl)?.is_empty();
            if unavailable || conflict {
                let sub = match provider {
                    Some(p) => mask_account_name_inner(&js_str(prop(p, "name")?)),
                    None => sel_pid,
                };
                return Ok(json!({
                    "title": pretty_model_name_inner(sel_up),
                    "sub": sub,
                    "group": false,
                    "issue": if unavailable { "unavailable" } else { "conflict" },
                }));
            }
        }
    }
    if title.starts_with(GROUP_MODEL_PREFIX) {
        let name_opt = group_display_name_opt(&payload, &title)?;
        let is_undef = name_opt.as_ref().is_none_or(is_undefined);
        let title_out = match &name_opt {
            Some(v) if !is_nil(v) => v.clone(),
            _ => Value::String(title.clone()),
        };
        let sub = if is_undef {
            Value::String(title)
        } else {
            Value::String(format!(
                "{GROUP_MODEL_PREFIX}{}",
                js_str(name_opt.as_ref().unwrap_or(&const_undef()))
            ))
        };
        return Ok(json!({ "title": title_out, "sub": sub, "group": true }));
    }
    if let Some(q) = qualified_provider_val(&payload, &title, &excl)? {
        let q_model = q.get("model").and_then(Value::as_str).unwrap_or("");
        let q_name = q
            .get("entry")
            .and_then(|e| e.get("name"))
            .map(js_str)
            .unwrap_or_default();
        return Ok(json!({
            "title": pretty_model_name_inner(q_model),
            "sub": mask_account_name_inner(&q_name),
            "group": false,
        }));
    }
    let serving = serving_providers_refs(&payload, &title, &excl)?;
    if serving.len() > 1 {
        let subs = serving
            .iter()
            .map(|e| Ok(mask_account_name_inner(&js_str(prop(e, "name")?))))
            .collect::<Result<Vec<_>, String>>()?;
        return Ok(json!({
            "title": pretty_model_name_inner(&title),
            "sub": subs.join(" → "),
            "group": false,
            "auto": true,
        }));
    }
    let sub = match serving.first() {
        Some(e) => mask_account_name_inner(&js_str(prop(e, "name")?)),
        None => String::new(),
    };
    Ok(json!({
        "title": pretty_model_name_inner(&title),
        "sub": sub,
        "group": false,
    }))
}

fn brand_word(lower: &str) -> Option<&'static str> {
    Some(match lower {
        "gpt" => "GPT",
        "glm" => "GLM",
        "deepseek" => "DeepSeek",
        "minimax" => "MiniMax",
        "qwen" => "Qwen",
        "qwq" => "QwQ",
        "kimi" => "Kimi",
        "grok" => "Grok",
        "claude" => "Claude",
        "gemini" => "Gemini",
        "gemma" => "Gemma",
        "llama" => "Llama",
        "mistral" => "Mistral",
        "codestral" => "Codestral",
        "doubao" => "Doubao",
        "ernie" => "ERNIE",
        "hunyuan" => "Hunyuan",
        "step" => "Step",
        "mimo" => "MiMo",
        "longcat" => "LongCat",
        _ => return None,
    })
}

fn is_letter_num_tag(s: &str) -> bool {
    let b = s.as_bytes();
    if b.len() < 2 || !b[0].is_ascii_lowercase() || !b[1].is_ascii_digit() {
        return false;
    }
    let mut i = 1;
    while i < b.len() && b[i].is_ascii_digit() {
        i += 1;
    }
    if i < b.len() && b[i] == b'.' {
        i += 1;
        let d0 = i;
        while i < b.len() && b[i].is_ascii_digit() {
            i += 1;
        }
        if i == d0 {
            return false;
        }
    }
    i == b.len() || (i + 1 == b.len() && b[i].is_ascii_lowercase())
}

fn is_size_tag(s: &str) -> bool {
    let b = s.as_bytes();
    if b.len() < 2 || !b[0].is_ascii_digit() {
        return false;
    }
    let mut i = 0;
    while i < b.len() && b[i].is_ascii_digit() {
        i += 1;
    }
    if i < b.len() && b[i] == b'.' {
        i += 1;
        let d0 = i;
        while i < b.len() && b[i].is_ascii_digit() {
            i += 1;
        }
        if i == d0 {
            return false;
        }
    }
    i + 1 == b.len() && matches!(b[i], b'b' | b'k' | b'm')
}

fn pretty_model_token(token: &str) -> String {
    let lower = token.to_lowercase();
    if let Some(w) = brand_word(&lower) {
        return w.into();
    }
    let b = lower.as_bytes();
    if b.len() >= 2 && b[0] == b'o' && b[1].is_ascii_digit() {
        return lower;
    }
    if is_letter_num_tag(&lower) || is_size_tag(&lower) {
        return lower.to_uppercase();
    }
    if b.first().is_some_and(u8::is_ascii_digit) {
        return lower;
    }
    let alpha_end = b
        .iter()
        .position(|c| !c.is_ascii_lowercase())
        .unwrap_or(b.len());
    if alpha_end == 0 {
        return token.into();
    }
    let (w, rest) = lower.split_at(alpha_end);
    if rest.chars().any(is_line_term) {
        return token.into();
    }
    if let Some(h) = brand_word(w) {
        format!("{h}{rest}")
    } else {
        let first = (w.as_bytes()[0] as char).to_ascii_uppercase();
        format!("{first}{}{rest}", &w[1..])
    }
}

fn pretty_model_name_inner(model: &str) -> String {
    let trimmed = es_trim(model);
    let (prefix, rest) = match trimmed.rfind('/') {
        Some(idx) if !trimmed[..=idx].chars().any(is_line_term) => {
            (&trimmed[..=idx], &trimmed[idx + 1..])
        }
        _ => ("", trimmed),
    };
    if rest.is_empty() || rest.contains('/') {
        return model.into();
    }
    let (name, suffix) = match rest.find('[') {
        Some(idx) => {
            let suf = &rest[idx..];
            if idx == 0 || !suf.ends_with(']') || suf.chars().any(is_line_term) {
                return model.into();
            }
            (&rest[..idx], suf)
        }
        None => (rest, ""),
    };
    if name.bytes().any(|b| b.is_ascii_uppercase()) {
        return trimmed.into();
    }
    let mut out = String::with_capacity(trimmed.len());
    out.push_str(prefix);
    let mut start = 0;
    for (idx, ch) in name.char_indices() {
        if ch == '-' || ch == '_' {
            out.push_str(&pretty_model_token(&name[start..idx]));
            out.push(ch);
            start = idx + 1;
        }
    }
    out.push_str(&pretty_model_token(&name[start..]));
    out.push_str(suffix);
    out
}

/// Port of `prettyModelName` (`src/gateway-model.ts:909`).
pub fn pretty_model_name(model: String) -> Result<String, String> {
    Ok(pretty_model_name_inner(&model))
}

/// Port of `groupModeLabel` (`src/gateway-model.ts:917`).
pub fn group_mode_label(mode: String) -> Result<String, String> {
    Ok(match mode.as_str() {
        "rotate" => "轮换",
        "weighted" => "按权重",
        "usage" => "额度优先",
        "fast" => "速度优先",
        "manual" => "手动",
        _ => "依次",
    }
    .into())
}

/// Port of `groupModeDescription` (`src/gateway-model.ts:934`).
pub fn group_mode_description(mode: String) -> Result<String, String> {
    Ok(match mode.as_str() {
        "rotate" => {
            "轮换：每个新会话从下一个成员开始；成员答不了时换下一个，每个成员再按自己的 Key 轮换。"
        }
        "weighted" => {
            "按权重：按成员权重决定先试谁；成员答不了时换下一个，每个成员再按自己的 Key 轮换。"
        }
        "usage" => {
            "额度优先：订阅账号里剩余额度最多的先试；没有额度数据的排在后面，额度用完的排最后。"
        }
        "fast" => "速度优先：最近响应最快的先试；还没测过的先试一次，之后按实测速度排。",
        "manual" => "手动：只发给选中的那个成员，它答不了也不换；在成员列表里点一下就换成它。",
        _ => "依次：按成员顺序，前一个答不了才换下一个，每个成员再按自己的 Key 轮换。",
    }
    .into())
}

/// Port of `groupStickyLabel` (`src/gateway-model.ts:951`).
pub fn group_sticky_label(sticky: Option<String>) -> Result<String, String> {
    Ok(match sticky.as_deref() {
        Some("session") => "整个会话固定",
        Some("off") => "不固定",
        _ => "缓存期内固定",
    }
    .into())
}

/// Port of `groupStickyDescription` (`src/gateway-model.ts:962`).
pub fn group_sticky_description(sticky: Option<String>) -> Result<String, String> {
    Ok(match sticky.as_deref() {
        Some("session") => "同一会话一直先试上次回答的成员，它答不了才换，最省缓存。",
        Some("off") => "每个请求都按模式重新排，会话中途可能换成员，上游的提示词缓存会失效。",
        _ => "同一会话 10 分钟内继续先试上次回答的成员，保住上游的提示词缓存；空闲超过 10 分钟再按模式重新排。",
    }
    .into())
}

/// Port of `parseWeight` (`src/gateway-model.ts:974`).
pub fn parse_weight(text: String) -> Result<Value, String> {
    let trimmed = es_trim(&text);
    if trimmed.is_empty() {
        return Ok(Value::Null);
    }
    let value = parse_js_number(trimmed);
    if !value.is_finite() {
        return Ok(Value::Null);
    }
    Ok(js_number(value.floor().max(1.0)))
}

/// Port of `parseGatewayPort` (`src/gateway-model.ts:995`).
pub fn parse_gateway_port(text: String) -> Result<Value, String> {
    let trimmed = es_trim(&text);
    if trimmed.is_empty() || trimmed.len() > 5 || !trimmed.bytes().all(|b| b.is_ascii_digit()) {
        return Ok(Value::Null);
    }
    let value = trimmed.parse::<f64>().unwrap_or(0.0);
    Ok(if (MIN_GATEWAY_PORT..=MAX_GATEWAY_PORT).contains(&value) {
        js_number(value)
    } else {
        Value::Null
    })
}

/// Port of `isPattern` (`src/gateway-model.ts:1003`).
pub fn is_pattern(model: String) -> Result<bool, String> {
    Ok(model.contains('*'))
}

fn glob_match_ci(pattern: &str, text: &str) -> bool {
    let p_lo = pattern.to_lowercase();
    let t_lo = text.to_lowercase();
    let parts: Vec<&str> = p_lo.split('*').collect();
    if parts.len() == 1 {
        return t_lo == parts[0];
    }
    let first = parts[0];
    let last = parts[parts.len() - 1];
    let Some(mut rest) = t_lo.strip_prefix(first) else {
        return false;
    };
    for part in &parts[1..parts.len() - 1] {
        let Some(pos) = rest.find(part) else {
            return false;
        };
        rest = &rest[pos + part.len()..];
    }
    rest.ends_with(last)
}

/// Port of `globMatches` (`src/gateway-model.ts:1008`).
pub fn glob_matches(pattern: String, text: String) -> Result<bool, String> {
    Ok(glob_match_ci(&pattern, &text))
}

/// Port of `patternMatches` (`src/gateway-model.ts:1014`).
pub fn pattern_matches(payload: Value, pattern: String) -> Result<Vec<String>, String> {
    let empty_excl = Value::Array(Vec::new());
    let mut matches = Vec::new();
    for entry in pool_providers_refs(&payload, &empty_excl)? {
        let id = js_str(prop(entry, "id")?);
        for model_v in req_arr(prop(entry, "models")?, "Symbol.iterator")? {
            let model = val_str_lossy(model_v)?;
            let qualified = format!("{id}/{model}");
            if glob_match_ci(&pattern, &qualified) || glob_match_ci(&pattern, &model) {
                matches.push(qualified);
            }
        }
    }
    Ok(matches)
}

/// Port of `speedOf` (`src/gateway-model.ts:1026`).
pub fn speed_of(payload: Value, relay_id: String, model: String) -> Result<Value, String> {
    if is_nil(&payload) {
        return Ok(Value::Null);
    }
    let speeds = match get_opt(&payload, "speeds")? {
        None | Some(Value::Null) => return Ok(Value::Null),
        Some(v) => req_arr(v, "find")?,
    };
    let model_lo = model.to_lowercase();
    for item in speeds {
        if val_eq_str(prop(item, "relayId")?, &relay_id)
            && val_str_lossy(prop(item, "model")?)? == model_lo
        {
            return Ok(prop(item, "ms")?.clone());
        }
    }
    Ok(Value::Null)
}

/// Port of `quotaWindowLabel` (`src/gateway-model.ts:1032`).
pub fn quota_window_label(window: Value) -> Result<Value, String> {
    let min_val = prop(&window, "windowMinutes")?;
    if min_val.is_null() {
        return Ok(json!({ "text": "本月", "args": [] }));
    }
    if let Some(m) = crate::as_number(min_val) {
        if m == 300.0 {
            return Ok(json!({ "text": "5 小时", "args": [] }));
        }
        if m == 10080.0 {
            return Ok(json!({ "text": "每周", "args": [] }));
        }
        if m % 1440.0 == 0.0 {
            return Ok(json!({ "text": "{0} 天", "args": [js_number(m / 1440.0)] }));
        }
        if m % 60.0 == 0.0 {
            return Ok(json!({ "text": "{0} 小时", "args": [js_number(m / 60.0)] }));
        }
    }
    Ok(json!({ "text": "{0} 分钟", "args": [min_val] }))
}

fn quota_supported_bool(entry: &Value) -> Result<bool, String> {
    if let Some(q) = get_opt(entry, "quotaApi")?
        && !q.is_null()
    {
        return Ok(is_truthy(q));
    }
    let acc = get_opt(entry, "account")?;
    Ok(acc.is_some_and(|v| matches!(v.as_str(), Some("chatgpt" | "copilot" | "grok"))))
}

/// Port of `quotaSupported` (`src/gateway-model.ts:1044`).
pub fn quota_supported(entry: Value) -> Result<bool, String> {
    quota_supported_bool(&entry)
}

/// Port of `quotaEntries` (`src/gateway-model.ts:1049`).
pub fn quota_entries(pool: Value) -> Result<Vec<Value>, String> {
    let mut out = Vec::new();
    for entry in nil_arr(&pool, "filter")? {
        if prop(entry, "enabled")? != &Value::Bool(false) && quota_supported_bool(entry)? {
            out.push(entry.clone());
        }
    }
    Ok(out)
}

/// Port of `quotaRemaining` (`src/gateway-model.ts:1054`).
pub fn quota_remaining(quota: Value, now_ms: f64) -> Result<f64, String> {
    if prop(&quota, "officialAllowed")? == &Value::Bool(false) {
        return Ok(0.0);
    }
    let mut least = 1.0_f64;
    for w in req_arr(prop(&quota, "windows")?, "reduce")? {
        let reset = prop(w, "resetsAtMs")?;
        let left = if !reset.is_null() && to_number(reset) <= now_ms {
            0.0
        } else {
            let used = to_number(prop(w, "usedPercent")?);
            js_min(1.0, js_max(0.0, 1.0 - used / 100.0))
        };
        least = js_min(least, left);
    }
    Ok(least)
}

fn quota_window_fresh_bool(
    quota: &Value,
    window: &Value,
    now_ms: f64,
    max_age_seconds: f64,
) -> Result<bool, String> {
    let obs_val = get_opt(window, "observedAtMs")?
        .filter(|v| !v.is_null())
        .unwrap_or(prop(quota, "updatedAtMs")?);
    let observed = to_number(obs_val);
    let used = to_number(prop(window, "usedPercent")?);
    let reset = prop(window, "resetsAtMs")?;
    Ok(observed <= now_ms
        && now_ms - observed <= max_age_seconds * 1000.0
        && used.is_finite()
        && (0.0..=100.0).contains(&used)
        && (reset.is_null() || to_number(reset) > now_ms))
}

/// Port of `quotaWindowFresh` (`src/gateway-model.ts:1062`).
pub fn quota_window_fresh(
    quota: Value,
    window: Value,
    now_ms: f64,
    max_age_seconds: Value,
) -> Result<bool, String> {
    let max_age = if is_undefined(&max_age_seconds) {
        600.0
    } else {
        to_number(&max_age_seconds)
    };
    quota_window_fresh_bool(&quota, &window, now_ms, max_age)
}

/// Port of `subscriptionBlockReason` (`src/gateway-model.ts:1070`).
pub fn subscription_block_reason(entry: Value, now_ms: Value) -> Result<Value, String> {
    let now = parse_now_ms_val(&now_ms);
    if prop(&entry, "account")?.as_str() != Some("chatgpt") {
        return Ok(Value::Null);
    }
    let sub_pol = prop(&entry, "subscriptionPolicy")?;
    if !is_truthy(sub_pol) {
        return Ok(Value::Null);
    }
    let policy = normalize_subscription_policy(Some(sub_pol));
    let def_limit = policy
        .get("defaultUsedPercentLimit")
        .unwrap_or(&Value::Null);
    if crate::as_number(def_limit) == Some(0.0) {
        return Ok(Value::String(
            "订阅额度已用上限设为 0%，已停止新请求".into(),
        ));
    }
    let unknown_msg = |text: &str| -> Value {
        if policy.get("unknownUsage").and_then(Value::as_str) == Some("allow") {
            Value::Null
        } else {
            Value::String(text.into())
        }
    };
    let block_or = |fallback: Value| -> Value {
        let existing = entry.get("subscriptionBlockReason");
        if let Some(v) = existing.filter(|v| is_truthy(v)) {
            v.clone()
        } else {
            fallback
        }
    };
    let quota = prop(&entry, "quota")?;
    if !is_truthy(quota) {
        return Ok(block_or(unknown_msg("官方额度未知，请刷新额度")));
    }
    let max_age = policy.get("maxAgeSeconds").map(to_number).unwrap_or(600.0);
    let updated_at = to_number(prop(quota, "updatedAtMs")?);
    let fresh = updated_at <= now && now - updated_at <= max_age * 1000.0;
    let off_allowed = prop(quota, "officialAllowed")?;
    let off_reached = prop(quota, "officialLimitReached")?;
    if fresh
        && (off_allowed == &Value::Bool(false)
            || (off_reached == &Value::Bool(true) && off_allowed != &Value::Bool(true)))
    {
        return Ok(Value::String(
            "官方订阅额度受限，请等待重置或刷新额度".into(),
        ));
    }
    let windows = req_arr(prop(quota, "windows")?, "some")?;
    let win_limits = policy
        .get("windowUsedPercentLimits")
        .and_then(Value::as_object);
    let win_complete = is_truthy(prop(quota, "windowsComplete")?);
    let missing_limited_win = !win_complete
        && win_limits.is_some_and(|lims| {
            lims.keys().any(|k| {
                !windows
                    .iter()
                    .any(|w| js_str(w.get("windowMinutes").unwrap_or(&const_undef())) == *k)
            })
        });
    let mut uncertain = !fresh
        || missing_limited_win
        || (windows.is_empty() && !(win_complete && off_allowed == &Value::Bool(true)));
    for window in windows {
        if !quota_window_fresh_bool(quota, window, now, max_age)? {
            uncertain = true;
            continue;
        }
        let win_min = prop(window, "windowMinutes")?;
        let win_key = js_str(win_min);
        let cap_val = win_limits
            .and_then(|m| m.get(&win_key))
            .filter(|v| !is_nil(v))
            .or(Some(def_limit).filter(|v| !is_nil(v)));
        let cap = cap_val.map_or(100.0, to_number);
        let used_val = prop(window, "usedPercent")?;
        if to_number(used_val) >= cap {
            let label = match crate::as_number(win_min) {
                Some(300.0) => "5h".into(),
                Some(10080.0) => "7d".into(),
                _ => {
                    let s = if is_nil(win_min) {
                        String::new()
                    } else {
                        js_str(win_min)
                    };
                    format!("{s} 分钟")
                }
            };
            return Ok(Value::String(format!(
                "{label} 已用 {}%，达到 {}% 上限",
                js_str(used_val),
                cap_val.map_or_else(|| "100".into(), js_str)
            )));
        }
    }
    Ok(block_or(if uncertain {
        unknown_msg("官方额度待刷新，请先刷新额度")
    } else {
        Value::Null
    }))
}

/// Port of `memberLabel` (`src/gateway-model.ts:1096`).
pub fn member_label(payload: Value, model: String) -> Result<String, String> {
    if model.contains('*') {
        return Ok(model);
    }
    let view = model_display(payload, model, Value::Array(Vec::new()), undefined())?;
    let title = js_str(view.get("title").unwrap_or(&Value::String(String::new())));
    if is_truthy(view.get("group").unwrap_or(&Value::Bool(false))) {
        return Ok(title);
    }
    let sub = js_str(view.get("sub").unwrap_or(&Value::String(String::new())));
    if is_truthy(view.get("auto").unwrap_or(&const_undef())) {
        return Ok(format!("{title} · {sub}"));
    }
    Ok(if sub.is_empty() {
        title
    } else {
        format!("{sub} · {title}")
    })
}

fn activity_agents(payload: &Value) -> Result<&[Value], String> {
    if is_nil(payload) {
        return Ok(&[]);
    }
    let Some(act) = get_opt(payload, "activity")?.filter(|v| !is_nil(v)) else {
        return Ok(&[]);
    };
    match get_opt(act, "agents")? {
        None | Some(Value::Null) => Ok(&[]),
        Some(v) => req_arr(v, "some"),
    }
}

fn member_status_val(payload: &Value, relay_id: &str, now_ms: f64) -> Result<Value, String> {
    let activity = activity_agents(payload)?;
    for agent in activity {
        if arr_includes_str(prop(agent, "inFlight")?, relay_id)? {
            return Ok(json!({ "state": "busy", "text": "正在回答…", "args": [] }));
        }
    }
    if !is_nil(payload) {
        let health = prop(payload, "health")?;
        for item in req_arr(prop(health, "providers")?, "find")? {
            if val_eq_str(prop(item, "relayId")?, relay_id) && is_truthy(prop(item, "breakerOpen")?)
            {
                let rem = get_opt(item, "remainingMs")?
                    .filter(|v| !is_nil(v))
                    .map(to_number);
                let left = format_remaining_f64(rem);
                return Ok(if left.is_empty() {
                    json!({ "state": "cooling", "text": "连续出错", "args": [] })
                } else {
                    json!({ "state": "cooling", "text": "连续出错 · {0} 后恢复", "args": [left] })
                });
            }
        }
    }
    let entry = if is_nil(payload) {
        None
    } else {
        let pool = req_arr(prop(payload, "pool")?, "find")?;
        let mut found = None;
        for item in pool {
            if val_eq_str(prop(item, "id")?, relay_id) {
                found = Some(item);
                break;
            }
        }
        found
    };
    let key_count_val = match entry {
        Some(e) => get_opt(e, "keyCount")?.filter(|v| !is_nil(v)).cloned(),
        None => None,
    };
    let key_count = key_count_val.as_ref().map_or(1.0, to_number);
    let unhealthy: Vec<&Value> = if is_nil(payload) {
        Vec::new()
    } else {
        let health = prop(payload, "health")?;
        let keys = match get_opt(health, "keys")? {
            None | Some(Value::Null) => &[][..],
            Some(v) => req_arr(v, "filter")?,
        };
        let mut list = Vec::new();
        for k in keys {
            if val_eq_str(prop(k, "relayId")?, relay_id) && prop(k, "state")?.as_str() != Some("ok")
            {
                list.push(k);
            }
        }
        list
    };
    if (unhealthy.len() as f64) >= key_count && !unhealthy.is_empty() {
        let cooling: Vec<&&Value> = unhealthy
            .iter()
            .filter(|k| k.get("state").and_then(Value::as_str) == Some("cooling"))
            .collect();
        if cooling.is_empty() {
            return Ok(json!({ "state": "cooling", "text": "Key 都已停用", "args": [] }));
        }
        let mut min_wait: Option<f64> = None;
        let mut has_quota = false;
        for k in &cooling {
            let ms = get_opt(k, "remainingMs")?
                .filter(|v| !is_nil(v))
                .map_or(0.0, to_number);
            if ms > 0.0 {
                min_wait = Some(min_wait.map_or(ms, |cur| js_min(cur, ms)));
            }
            if get_opt(k, "lastFailure")?.and_then(Value::as_str) == Some("quota_exhausted") {
                has_quota = true;
            }
        }
        let left = min_wait.map_or_else(String::new, |m| format_remaining_f64(Some(m)));
        return Ok(if left.is_empty() {
            json!({ "state": "cooling", "text": if has_quota { "额度耗尽" } else { "被限流" }, "args": [] })
        } else {
            json!({
                "state": "cooling",
                "text": if has_quota { "额度耗尽 · {0} 后恢复" } else { "被限流 · {0} 后恢复" },
                "args": [left],
            })
        });
    }
    if let Some(e) = entry
        && is_truthy(prop(e, "account")?)
        && is_truthy(prop(e, "quota")?)
    {
        let quota = prop(e, "quota")?;
        let max_age = match get_opt(e, "subscriptionPolicy")?.filter(|v| !is_nil(v)) {
            Some(sp) => get_opt(sp, "maxAgeSeconds")?
                .filter(|v| !is_nil(v))
                .map_or(600.0, to_number),
            None => 600.0,
        };
        let windows = req_arr(prop(quota, "windows")?, "filter")?;
        let mut exhausted = Vec::new();
        for w in windows {
            if quota_window_fresh_bool(quota, w, now_ms, max_age)?
                && to_number(prop(w, "usedPercent")?) >= 100.0
            {
                exhausted.push(w);
            }
        }
        if !exhausted.is_empty() {
            let mut resets = Vec::new();
            for w in &exhausted {
                let r = prop(w, "resetsAtMs")?;
                if !r.is_null() {
                    resets.push(to_number(r));
                }
            }
            let left = if resets.len() == exhausted.len() {
                let max_r = resets
                    .into_iter()
                    .reduce(js_max)
                    .unwrap_or(f64::NEG_INFINITY);
                format_remaining_f64(Some(max_r - now_ms))
            } else {
                String::new()
            };
            return Ok(if left.is_empty() {
                json!({ "state": "cooling", "text": "额度耗尽", "args": [] })
            } else {
                json!({ "state": "cooling", "text": "额度耗尽 · {0} 后恢复", "args": [left] })
            });
        }
        let updated_at = to_number(prop(quota, "updatedAtMs")?);
        let fresh = updated_at <= now_ms && now_ms - updated_at <= max_age * 1000.0;
        let off_allowed = prop(quota, "officialAllowed")?;
        let off_reached = prop(quota, "officialLimitReached")?;
        if fresh
            && (off_allowed == &Value::Bool(false)
                || (off_reached == &Value::Bool(true) && off_allowed != &Value::Bool(true)))
        {
            return Ok(json!({ "state": "cooling", "text": "订阅额度受限", "args": [] }));
        }
    }
    let mut answered: Vec<(&Value, f64)> = Vec::new();
    for agent in activity {
        let Some(last) = get_opt(agent, "last")?.filter(|v| !is_nil(v)) else {
            continue;
        };
        if prop(last, "result")?.as_str() != Some("answered") {
            continue;
        }
        let steps = req_arr(prop(last, "steps")?, "at")?;
        let Some(last_step) = steps.last().filter(|s| !is_nil(s)) else {
            continue;
        };
        if !val_eq_str(prop(last_step, "relayId")?, relay_id) {
            continue;
        }
        let finished = to_number(prop(last, "finishedAtMs")?);
        if now_ms - finished < RECENT_USE_MS {
            answered.push((agent, finished));
        }
    }
    if !answered.is_empty() {
        js::js_sort(&mut answered, |a, b| b.1 - a.1);
        let who = agent_name_str(&js_str(prop(answered[0].0, "agent")?));
        return Ok(json!({ "state": "answered", "text": "{0} 刚用过", "args": [who] }));
    }
    if !unhealthy.is_empty() {
        return Ok(json!({
            "state": "idle",
            "text": "{0} 个 Key · {1} 个冷却中",
            "args": [js_number(key_count), unhealthy.len()],
        }));
    }
    Ok(json!({
        "state": "idle",
        "text": "{0} 个 Key",
        "args": [js_number(js_max(key_count, 1.0))],
    }))
}

/// Port of `memberStatus` (`src/gateway-model.ts:1111`).
pub fn member_status(payload: Value, relay_id: String, now_ms: Value) -> Result<Value, String> {
    member_status_val(&payload, &relay_id, parse_now_ms_val(&now_ms))
}

/// Port of `dispatchStatus` (`src/gateway-model.ts:1166`).
pub fn dispatch_status(
    payload: Value,
    relay_id: String,
    model: String,
    now_ms: Value,
) -> Result<Value, String> {
    let now = parse_now_ms_val(&now_ms);
    let status = member_status_val(&payload, &relay_id, now)?;
    if status.get("state").and_then(Value::as_str) == Some("cooling") {
        return Ok(status);
    }
    let model_lo = model.to_lowercase();
    let mut used: Vec<(&Value, f64)> = Vec::new();
    for agent in activity_agents(&payload)? {
        let Some(last) = get_opt(agent, "last")?.filter(|v| !is_nil(v)) else {
            continue;
        };
        if prop(last, "result")?.as_str() != Some("answered") {
            continue;
        }
        let steps = req_arr(prop(last, "steps")?, "at")?;
        let Some(step) = steps.last().filter(|s| !is_nil(s)) else {
            continue;
        };
        if val_eq_str(prop(step, "relayId")?, &relay_id)
            && val_str_lossy(prop(step, "model")?)?.to_lowercase() == model_lo
        {
            let finished = to_number(prop(last, "finishedAtMs")?);
            if now - finished < RECENT_USE_MS {
                used.push((agent, finished));
            }
        }
    }
    if used.is_empty() {
        return Ok(Value::Null);
    }
    js::js_sort(&mut used, |a, b| b.1 - a.1);
    let who = agent_name_str(&js_str(prop(used[0].0, "agent")?));
    Ok(json!({ "state": "answered", "text": "{0} 刚用过", "args": [who] }))
}

fn format_elapsed_f64(ms: f64) -> String {
    if ms < 10_000.0 {
        format!("{} s", js_to_fixed(ms / 1000.0, 1))
    } else {
        format!("{} s", number_to_string(js::math_round(ms / 1000.0)))
    }
}

/// Port of `formatElapsed` (`src/gateway-model.ts:1178`).
pub fn format_elapsed(ms: f64) -> Result<String, String> {
    Ok(format_elapsed_f64(ms))
}

fn resolve_group_name_or(payload: &Value, raw: &Value) -> Result<Value, String> {
    let s = js_str(raw);
    match group_display_name_opt(payload, &s)? {
        Some(v) if !is_nil(&v) => Ok(v),
        _ => Ok(raw.clone()),
    }
}

/// Port of `routeNarrative` (`src/gateway-model.ts:1188`).
pub fn route_narrative(agent: String, record: Value, payload: Value) -> Result<Vec<Value>, String> {
    let mut lines = Vec::new();
    let who = agent_name_str(&agent);
    let req_raw = {
        let r = prop(&record, "requestedModel")?;
        if is_truthy(r) {
            r.clone()
        } else {
            prop(&record, "model")?.clone()
        }
    };
    let requested = resolve_group_name_or(&payload, &req_raw)?;
    let planned = req_arr(prop(&record, "planned")?, "map")?;
    let mut plan_parts = Vec::with_capacity(planned.len());
    for m in planned {
        let rname = mask_account_name_inner(&js_str(prop(m, "relayName")?));
        let mname = js_str(&resolve_group_name_or(&payload, prop(m, "model")?)?);
        plan_parts.push(format!("{rname}/{mname}"));
    }
    let plan = plan_parts.join(", ");
    let group_id = prop(&record, "groupId")?;
    if is_truthy(group_id) {
        let gid_str = js_str(group_id);
        let group_name = if is_nil(&payload) {
            None
        } else {
            let mut found = None;
            for item in req_arr(prop(&payload, "groups")?, "find")? {
                if prop(item, "id")? == group_id {
                    found = Some(prop(item, "name")?.clone());
                    break;
                }
            }
            found
        };
        let gname_val = match group_name {
            Some(v) if !is_nil(&v) => v,
            _ => Value::String(gid_str),
        };
        lines.push(json!({
            "tone": "muted",
            "text": "{0} 请求 {1}，交给模型组「{2}」：{3}",
            "args": [who, requested, gname_val, plan],
        }));
        if let Some(first) = planned.first() {
            lines.push(json!({
                "tone": "step",
                "text": "{0} 排在第一个。",
                "args": [mask_account_name_inner(&js_str(prop(first, "relayName")?))],
            }));
        }
    } else {
        lines.push(json!({
            "tone": "muted",
            "text": "{0} 请求 {1}，转发到 {2}",
            "args": [who, requested, plan],
        }));
    }
    let steps = req_arr(prop(&record, "steps")?, "forEach")?;
    let elapsed_ms = to_number(prop(&record, "elapsedMs")?);
    for (index, step) in steps.iter().enumerate() {
        let last = index == steps.len() - 1;
        let rname = mask_account_name_inner(&js_str(prop(step, "relayName")?));
        if prop(step, "outcome")?.as_str() == Some("answered") {
            let model_disp = resolve_group_name_or(&payload, prop(step, "model")?)?;
            lines.push(json!({
                "tone": "ok",
                "text": "{0}（{1}）回答，用时 {2}。",
                "args": [rname, model_disp, format_elapsed_f64(elapsed_ms)],
            }));
            continue;
        }
        let reason_zh = if prop(step, "outcome")?.as_str() == Some("transport") {
            "连不上"
        } else {
            let lbl = failure_label_str(prop(step, "failure")?.as_str());
            if lbl.is_empty() { "出错" } else { lbl }
        };
        let reason = json!({ "zh": reason_zh });
        let status_val = prop(step, "status")?;
        let status_str = if is_truthy(status_val) {
            format!(" · HTTP {}", js_str(status_val))
        } else {
            String::new()
        };
        let key_id = prop(step, "keyId")?.clone();
        lines.push(if last {
            json!({
                "tone": "fail",
                "text": "{0} 的 Key {1} {2}{3}，错误交回给 {4}。",
                "args": [rname, key_id, reason, status_str, who],
            })
        } else {
            json!({
                "tone": "step",
                "text": "{0} 的 Key {1} {2}{3}，换下一个。",
                "args": [rname, key_id, reason, status_str],
            })
        });
    }
    if prop(&record, "result")?.as_str() == Some("cooling") {
        let cooling_ms = get_opt(&record, "coolingMs")?
            .filter(|v| !is_nil(v))
            .map_or(0.0, to_number);
        let rem = format_remaining_f64(Some(cooling_ms));
        lines.push(json!({
            "tone": "fail",
            "text": "所有 Key 都在冷却，约 {0} 后恢复。",
            "args": [if rem.is_empty() { "1s".into() } else { rem }],
        }));
    }
    Ok(lines)
}

/// Port of `connectedAgents` (`src/gateway-model.ts:1222`).
pub fn connected_agents(payload: Value) -> Result<Vec<Value>, String> {
    let agents = if is_nil(&payload) {
        &[]
    } else {
        match get_opt(&payload, "agents")? {
            None | Some(Value::Null) => &[],
            Some(v) => req_arr(v, "filter")?,
        }
    };
    let mut out = Vec::new();
    for view in agents {
        if is_truthy(prop(view, "enabled")?) {
            out.push(prop(view, "agent")?.clone());
        }
    }
    Ok(out)
}

/// Port of `modelDispatch` (`src/gateway-model.ts:1233`).
pub fn model_dispatch(payload: Value) -> Result<Vec<Value>, String> {
    let connected = connected_agents(payload.clone())?;
    let empty_excl = Value::Array(Vec::new());
    let mut rows: Vec<(String, Vec<Value>)> = Vec::new();
    let mut index_by_model: HashMap<String, usize> = HashMap::new();
    let mut seen_ids: Vec<HashSet<String>> = Vec::new();

    for entry in pool_providers_refs(&payload, &empty_excl)? {
        let entry_id = js_str(prop(entry, "id")?);
        let mut excluded_by = Vec::new();
        for agent in &connected {
            if !is_nil(&payload) {
                let gw = prop(&payload, "gateway")?;
                let bindings = prop(gw, "bindings")?;
                if is_nil(bindings) {
                    return Err(nil_err("binding"));
                }
                if let Some(b) = get_opt(bindings, &js_str(agent))?.filter(|v| !is_nil(v))
                    && let Some(pe) = get_opt(b, "poolExclude")?.filter(|v| !is_nil(v))
                    && arr_includes_str(pe, &entry_id)?
                {
                    excluded_by.push(agent.clone());
                }
            }
        }
        let prov_obj = json!({
            "id": entry_id,
            "name": mask_account_name_inner(&js_str(prop(entry, "name")?)),
            "baseUrl": prop(entry, "baseUrl")?,
            "excludedBy": excluded_by,
        });
        for model_v in req_arr(prop(entry, "models")?, "Symbol.iterator")? {
            let model = val_str_lossy(model_v)?;
            let key = model.to_lowercase();
            let slot = match index_by_model.get(&key) {
                Some(&i) => i,
                None => {
                    let i = rows.len();
                    index_by_model.insert(key, i);
                    rows.push((model, Vec::new()));
                    seen_ids.push(HashSet::new());
                    i
                }
            };
            if seen_ids[slot].insert(entry_id.clone()) {
                rows[slot].1.push(prov_obj.clone());
            }
        }
    }
    let mut out: Vec<Value> = rows
        .into_iter()
        .map(|(model, providers)| json!({ "model": model, "providers": providers }))
        .collect();
    js::js_sort(&mut out, |left, right| {
        let r = i32::from(right["providers"].as_array().is_some_and(|a| a.len() > 1));
        let l = i32::from(left["providers"].as_array().is_some_and(|a| a.len() > 1));
        f64::from(r - l)
    });
    Ok(out)
}

fn active_sponsor_ids_for(dt: Option<DateTime<Local>>) -> Vec<String> {
    let today = local_day_str(dt);
    SPONSORS
        .iter()
        .filter(|(_, exp)| *exp >= today.as_str())
        .map(|(id, _)| (*id).to_string())
        .collect()
}

/// Port of `activeSponsorIds` (`src/gateway-model.ts:1361`).
pub fn active_sponsor_ids(now: Value) -> Result<Vec<String>, String> {
    Ok(active_sponsor_ids_for(parse_date_arg(&now)?))
}

fn extra_presets() -> Vec<Value> {
    vec![
        json!({
            "key": "anthropic", "name": "Anthropic", "baseUrl": "https://api.anthropic.com",
            "protocol": "anthropicMessages", "model": "claude-sonnet-5-5",
            "modelList": ["claude-opus-5-5", "claude-sonnet-5-5", "claude-haiku-5-5"],
            "section": "vendor", "apiKeyUrl": "https://console.anthropic.com/settings/keys",
        }),
        json!({
            "key": "xai", "name": "xAI", "baseUrl": "https://api.x.ai/v1",
            "protocol": "chatCompletions", "model": "", "modelList": [],
            "section": "vendor", "apiKeyUrl": "https://console.x.ai",
        }),
        json!({
            "key": "opencode-go", "name": "OpenCode Go", "baseUrl": "https://opencode.ai/zen/go/v1",
            "protocol": "chatCompletions", "model": "", "modelList": [],
            "section": "relay", "apiKeyUrl": "https://opencode.ai/auth",
        }),
        json!({
            "key": "opencode-zen", "name": "OpenCode Zen", "baseUrl": "https://opencode.ai/zen/v1",
            "protocol": "chatCompletions", "model": "", "modelList": [],
            "section": "relay", "apiKeyUrl": "https://opencode.ai/auth",
        }),
        json!({
            "key": "ollama", "name": "Ollama", "baseUrl": "http://127.0.0.1:11434/v1",
            "protocol": "chatCompletions", "model": "", "modelList": [],
            "section": "local", "defaultKey": "ollama",
        }),
        json!({
            "key": "lm-studio", "name": "LM Studio", "baseUrl": "http://127.0.0.1:1234/v1",
            "protocol": "chatCompletions", "model": "", "modelList": [],
            "section": "local", "defaultKey": "lm-studio",
        }),
    ]
}

/// Port of `gatewayPresets` (`src/gateway-model.ts:1366`).
pub fn gateway_presets(now: Value) -> Result<Vec<Value>, String> {
    let sponsor_ids = active_sponsor_ids_for(parse_date_arg(&now)?);
    let expired: HashSet<&str> = SPONSORS
        .iter()
        .map(|(id, _)| *id)
        .filter(|id| !sponsor_ids.iter().any(|s| s == id))
        .collect();
    let shared: Vec<Value> = presets_value()
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or(&[])
        .iter()
        .filter(|p| {
            p.get("id")
                .and_then(Value::as_str)
                .is_some_and(|id| !expired.contains(id))
        })
        .map(|p| {
            let id = p.get("id").and_then(Value::as_str).unwrap_or("");
            let model = p.get("model").and_then(Value::as_str).unwrap_or("");
            let model_list = match p.get("modelList") {
                Some(v) if !is_nil(v) => v.clone(),
                _ if !model.is_empty() => json!([model]),
                _ => json!([]),
            };
            let cat = p.get("category").and_then(Value::as_str).unwrap_or("");
            let section = if sponsor_ids.iter().any(|s| s == id) {
                "sponsor"
            } else if cat == "official" || cat == "cn_official" {
                "vendor"
            } else {
                "relay"
            };
            json!({
                "key": id,
                "name": p.get("name").cloned().unwrap_or(Value::String(String::new())),
                "baseUrl": p.get("baseUrl").cloned().unwrap_or(Value::String(String::new())),
                "protocol": p.get("protocol").cloned().unwrap_or(Value::String(String::new())),
                "model": model,
                "modelList": model_list,
                "section": section,
                "apiKeyUrl": p.get("apiKeyUrl").cloned().unwrap_or_else(undefined),
            })
        })
        .collect();
    let extras = extra_presets();
    let by_sec = |sec: &str| -> Vec<Value> {
        extras
            .iter()
            .filter(|p| p.get("section").and_then(Value::as_str) == Some(sec))
            .chain(
                shared
                    .iter()
                    .filter(|p| p.get("section").and_then(Value::as_str) == Some(sec)),
            )
            .cloned()
            .collect()
    };
    let mut out: Vec<Value> = sponsor_ids
        .iter()
        .flat_map(|id| {
            shared
                .iter()
                .filter(move |p| p.get("key").and_then(Value::as_str) == Some(id.as_str()))
                .cloned()
        })
        .collect();
    out.extend(by_sec("vendor"));
    out.extend(by_sec("relay"));
    out.extend(
        extras
            .into_iter()
            .filter(|p| p.get("section").and_then(Value::as_str) == Some("local")),
    );
    Ok(out)
}

/// Port of `presetHost` (`src/gateway-model.ts:1393`).
pub fn preset_host(base_url: String) -> Result<String, String> {
    let Ok(url) = Url::parse(&base_url) else {
        return Ok(base_url);
    };
    let host = url.host_str().unwrap_or("");
    Ok(if host == "127.0.0.1" || host == "localhost" {
        let port = url.port().map(|p| p.to_string()).unwrap_or_default();
        format!("localhost:{port}")
    } else {
        host.to_string()
    })
}

fn normalize_url(url: &str) -> String {
    es_trim(url).trim_end_matches('/').to_lowercase()
}

/// Port of `presetAdded` (`src/gateway-model.ts:1403`).
pub fn preset_added(payload: Value, preset: Value) -> Result<bool, String> {
    let target = normalize_url(req_str(&preset, "baseUrl", "trim")?);
    let key = js_str(prop(&preset, "key")?);
    let src = format!("preset:{key}");
    if is_nil(&payload) {
        return Ok(false);
    }
    for entry in req_arr(prop(&payload, "pool")?, "some")? {
        if get_str_opt(entry, "source")? == Some(src.as_str())
            || normalize_url(req_str(entry, "baseUrl", "trim")?) == target
        {
            return Ok(true);
        }
    }
    Ok(false)
}

/// Port of `providerDraftFromPreset` (`src/gateway-model.ts:1438`).
pub fn provider_draft_from_preset(preset: Value) -> Result<Value, String> {
    if !is_truthy(&preset) {
        return Ok(json!({
            "name": "", "baseUrl": "", "keys": "", "protocol": "chatCompletions",
            "model": "", "modelList": "",
            "priceCurrency": "CNY", "priceInput": "", "priceCached": "", "priceOutput": "",
            "proxy": "", "stripImageGenerationTool": false, "stripResponsesImageDetail": false,
            "disableToolSearch": false,
        }));
    }
    let keys = get_opt(&preset, "defaultKey")?
        .filter(|v| !v.is_null())
        .cloned()
        .unwrap_or_else(|| Value::String(String::new()));
    let model_list = req_arr(prop(&preset, "modelList")?, "join")?
        .iter()
        .map(|v| if is_nil(v) { String::new() } else { js_str(v) })
        .collect::<Vec<_>>()
        .join("\n");
    Ok(json!({
        "name": prop(&preset, "name")?,
        "baseUrl": prop(&preset, "baseUrl")?,
        "keys": keys,
        "protocol": prop(&preset, "protocol")?,
        "model": prop(&preset, "model")?,
        "modelList": model_list,
        "priceCurrency": "CNY", "priceInput": "", "priceCached": "", "priceOutput": "",
        "proxy": "", "stripImageGenerationTool": false, "stripResponsesImageDetail": false,
        "disableToolSearch": false,
    }))
}

/// Port of `providerDraftFromEntry` (`src/gateway-model.ts:1452`).
pub fn provider_draft_from_entry(entry: Value) -> Result<Value, String> {
    let models = req_arr(prop(&entry, "models")?, "join")?;
    let first_model = models
        .first()
        .cloned()
        .unwrap_or_else(|| Value::String(String::new()));
    let model_list = models
        .iter()
        .map(|v| if is_nil(v) { String::new() } else { js_str(v) })
        .collect::<Vec<_>>()
        .join("\n");
    let price = get_opt(&entry, "price")?.filter(|v| is_truthy(v));
    let price_currency = match price {
        Some(p) => {
            let c = prop(p, "currency")?;
            if is_truthy(c) {
                c.clone()
            } else {
                Value::String("CNY".into())
            }
        }
        None => Value::String("CNY".into()),
    };
    let p_str = |k: &str| -> Result<String, String> {
        Ok(match price {
            Some(p) => js_str(prop(p, k)?),
            None => String::new(),
        })
    };
    let sub_pol = match entry.as_object().and_then(|m| m.get("subscriptionPolicy")) {
        None => undefined(),
        Some(v) if is_nil(v) => v.clone(),
        Some(v) => normalize_subscription_policy(Some(v)),
    };
    let custom_headers = match get_opt(&entry, "customHeaders")? {
        None | Some(Value::Null) => provider_header_rows(None)?,
        Some(v) => provider_header_rows(Some(req_arr(v, "map")?))?,
    };
    Ok(json!({
        "name": prop(&entry, "name")?,
        "baseUrl": prop(&entry, "baseUrl")?,
        "keys": "",
        "protocol": prop(&entry, "protocol")?,
        "model": first_model,
        "modelList": model_list,
        "priceCurrency": price_currency,
        "priceInput": p_str("input")?,
        "priceCached": p_str("cachedInput")?,
        "priceOutput": p_str("output")?,
        "proxy": opt_trim_raw(&entry, "proxy")?,
        "stripImageGenerationTool": get_opt(&entry, "stripImageGenerationTool")? == Some(&Value::Bool(true)),
        "subscriptionPolicy": sub_pol,
        "stripResponsesImageDetail": get_opt(&entry, "stripResponsesImageDetail")? == Some(&Value::Bool(true)),
        "disableToolSearch": get_opt(&entry, "disableToolSearch")? == Some(&Value::Bool(true)),
        "customHeaders": custom_headers,
    }))
}

fn opt_trim_raw(v: &Value, key: &str) -> Result<Value, String> {
    Ok(match get_opt(v, key)? {
        None | Some(Value::Null) => Value::String(String::new()),
        Some(x) => x.clone(),
    })
}

fn price_number(text: &str) -> f64 {
    let val = parse_js_number(es_trim(text));
    if val.is_finite() && val > 0.0 {
        val
    } else {
        0.0
    }
}

fn price_from_draft_opt(draft: &Value) -> Result<Option<Value>, String> {
    let cur = prop(draft, "priceCurrency")?;
    let currency = if is_truthy(cur) {
        cur.clone()
    } else {
        Value::String("CNY".into())
    };
    let input = price_number(req_str(draft, "priceInput", "trim")?);
    let cached = price_number(req_str(draft, "priceCached", "trim")?);
    let output = price_number(req_str(draft, "priceOutput", "trim")?);
    if input != 0.0 || cached != 0.0 || output != 0.0 {
        Ok(Some(json!({
            "currency": currency,
            "input": js_number(input),
            "cachedInput": js_number(cached),
            "output": js_number(output),
        })))
    } else {
        Ok(None)
    }
}

/// Port of `providerInput` (`src/gateway-model.ts:1474`).
pub fn provider_input(draft: Value) -> Result<Value, String> {
    let name = req_str_trim(&draft, "name")?;
    let base_url = req_str_trim(&draft, "baseUrl")?;
    let protocol = prop(&draft, "protocol")?.clone();
    let keys: Vec<Value> = req_str(&draft, "keys", "split")?
        .split('\n')
        .map(es_trim)
        .filter(|s| !s.is_empty())
        .map(|s| Value::String(s.to_string()))
        .collect();
    let model = req_str_trim(&draft, "model")?;
    let model_list = split_models_inner(req_str(&draft, "modelList", "split")?).join("\n");
    let price = price_from_draft_opt(&draft)?.unwrap_or(Value::Null);
    let model_prices = get_opt(&draft, "modelPrices")?
        .filter(|v| !v.is_null())
        .cloned()
        .unwrap_or(Value::Null);
    let proxy = req_str_trim(&draft, "proxy")?;
    let strip_img = get_opt(&draft, "stripImageGenerationTool")? == Some(&Value::Bool(true));
    let sub_pol = get_opt(&draft, "subscriptionPolicy")?;
    let strip_det = get_opt(&draft, "stripResponsesImageDetail")? == Some(&Value::Bool(true));
    let dis_search = get_opt(&draft, "disableToolSearch")? == Some(&Value::Bool(true));
    let custom_hdrs = get_opt(&draft, "customHeaders")?;

    let mut out = Map::new();
    out.insert("name".into(), Value::String(name));
    out.insert("baseUrl".into(), Value::String(base_url));
    out.insert("protocol".into(), protocol);
    out.insert("keys".into(), Value::Array(keys));
    out.insert("model".into(), Value::String(model));
    out.insert("modelList".into(), Value::String(model_list));
    out.insert("price".into(), price);
    out.insert("modelPrices".into(), model_prices);
    out.insert("proxy".into(), Value::String(proxy));
    out.insert("stripImageGenerationTool".into(), Value::Bool(strip_img));
    if let Some(sp) = sub_pol {
        out.insert("subscriptionPolicy".into(), sp.clone());
    }
    out.insert("stripResponsesImageDetail".into(), Value::Bool(strip_det));
    out.insert("disableToolSearch".into(), Value::Bool(dis_search));
    if let Some(ch) = custom_hdrs {
        let arr = req_arr(ch, "map")?;
        out.insert(
            "customHeaders".into(),
            Value::Array(provider_header_inputs(arr)?),
        );
    }
    Ok(Value::Object(out))
}

/// Port of `priceFromDraft` (`src/gateway-model.ts:1499`).
pub fn price_from_draft(draft: Value) -> Result<Value, String> {
    Ok(price_from_draft_opt(&draft)?.unwrap_or(Value::Null))
}

/// Port of `isDirectProxy` (`src/gateway-model.ts:1507`).
pub fn is_direct_proxy(proxy: Option<String>) -> Result<bool, String> {
    Ok(es_trim(proxy.as_deref().unwrap_or("")).to_lowercase() == DIRECT_PROXY)
}

fn has_scheme_and_non_ws(s: &str, schemes: &[&str]) -> bool {
    let lower = s.to_lowercase();
    schemes.iter().any(|sch| {
        lower
            .strip_prefix(sch)
            .is_some_and(|rest| !rest.is_empty() && !rest.chars().any(is_es_ws))
    })
}

fn proxy_address_error_str(proxy: &str) -> Option<&'static str> {
    let value = es_trim(proxy);
    if value.is_empty() || value.to_lowercase() == DIRECT_PROXY {
        return None;
    }
    if has_scheme_and_non_ws(value, &["http://", "https://", "socks5://", "socks5h://"]) {
        None
    } else {
        Some("代理地址要写成 http://主机:端口（也支持 https:// 与 socks5://）")
    }
}

/// Port of `proxyAddressError` (`src/gateway-model.ts:1512`).
pub fn proxy_address_error(proxy: String) -> Result<Value, String> {
    Ok(proxy_address_error_str(&proxy).map_or(Value::Null, |s| Value::String(s.into())))
}

/// Port of `validateProviderDraft` (`src/gateway-model.ts:1519`).
pub fn validate_provider_draft(draft: Value, editing: Value) -> Result<Value, String> {
    if req_str_trim(&draft, "name")?.is_empty() {
        return Ok(Value::String("请填写名称".into()));
    }
    let base_url = req_str_trim(&draft, "baseUrl")?;
    if !has_scheme_and_non_ws(&base_url, &["http://", "https://"]) {
        return Ok(Value::String("地址要以 http:// 或 https:// 开头".into()));
    }
    if !is_truthy(&editing) && req_str_trim(&draft, "keys")?.is_empty() {
        return Ok(Value::String("请填写 API Key".into()));
    }
    if req_str_trim(&draft, "model")?.is_empty()
        && split_models_inner(req_str(&draft, "modelList", "split")?).is_empty()
    {
        return Ok(Value::String("至少填一个模型".into()));
    }
    if let Some(err) = proxy_address_error_str(req_str(&draft, "proxy", "trim")?) {
        return Ok(Value::String(err.into()));
    }
    let headers = match get_opt(&draft, "customHeaders")? {
        None | Some(Value::Null) => &[],
        Some(v) => req_arr(v, "map")?,
    };
    Ok(provider_headers_validation_message(headers)?.map_or(Value::Null, Value::String))
}

/// Port of `usageRangeStart` (`src/gateway-model.ts:1558`).
pub fn usage_range_start(range: String, now: Value) -> Result<Value, String> {
    let days: u64 = match range.as_str() {
        "today" => 1,
        "7d" => 7,
        "30d" => 30,
        _ => return Ok(undefined()),
    };
    let Some(dt) = parse_date_arg(&now)? else {
        return Ok(js_number(f64::NAN));
    };
    let Some(start_date) = dt.date_naive().checked_sub_days(Days::new(days - 1)) else {
        return Ok(js_number(f64::NAN));
    };
    let ms = start_date
        .and_time(NaiveTime::MIN)
        .and_local_timezone(Local)
        .earliest()
        .map_or(f64::NAN, |d| d.timestamp_millis() as f64);
    Ok(js_number(ms))
}

/// Port of `relayFilterFromFocus` (`src/gateway-model.ts:1570`).
pub fn relay_filter_from_focus(focus: Value) -> Result<Value, String> {
    if is_truthy(prop(&focus, "relayUnknown")?) {
        let lbl = prop(&focus, "relayLabel")?;
        let label = if is_truthy(lbl) {
            lbl.clone()
        } else {
            Value::String(String::new())
        };
        return Ok(json!({ "id": "", "label": label, "unknown": true }));
    }
    let rid = prop(&focus, "relayId")?;
    if is_truthy(rid) {
        let lbl = prop(&focus, "relayLabel")?;
        let label = if is_truthy(lbl) {
            lbl.clone()
        } else {
            rid.clone()
        };
        return Ok(json!({ "id": rid, "label": label }));
    }
    Ok(Value::Null)
}

/// Port of `relayFilterArgs` (`src/gateway-model.ts:1576`).
pub fn relay_filter_args(filter: Value) -> Result<Value, String> {
    if !is_nil(&filter) && is_truthy(prop(&filter, "unknown")?) {
        return Ok(json!({ "relayId": null, "relayUnknown": true }));
    }
    let id = if is_nil(&filter) {
        Value::Null
    } else {
        let rid = prop(&filter, "id")?;
        if is_truthy(rid) {
            rid.clone()
        } else {
            Value::Null
        }
    };
    Ok(json!({ "relayId": id, "relayUnknown": false }))
}

fn currency_symbol_str(currency: &str) -> String {
    match currency.to_uppercase().as_str() {
        "CNY" => "¥".into(),
        "USD" => "$".into(),
        "EUR" => "€".into(),
        up => format!("{up} "),
    }
}

/// Port of `formatRequestCost` (`src/gateway-model.ts:1581`).
pub fn format_request_cost(cost: Value) -> Result<String, String> {
    let amount = to_number(prop(&cost, "amount")?);
    if amount <= 0.0 {
        return Ok("—".into());
    }
    let cur = req_str(&cost, "currency", "toUpperCase")?;
    if amount >= 0.01 {
        return Ok(format!(
            "{}{}",
            currency_symbol_str(cur),
            js_to_fixed(amount, 2)
        ));
    }
    Ok(format!(
        "{}{}",
        currency_symbol_str(cur),
        js_to_precision_2(amount)
    ))
}

fn trim_number(value: f64) -> String {
    if value >= 10.0 {
        js_to_fixed(value, 0)
    } else {
        let s = js_to_fixed(value, 1);
        s.strip_suffix(".0").unwrap_or(&s).to_string()
    }
}

/// Port of `formatTokens` (`src/gateway-model.ts:1588`).
pub fn format_tokens(value: f64) -> Result<String, String> {
    Ok(if value >= 1_000_000_000.0 {
        format!("{}B", trim_number(value / 1_000_000_000.0))
    } else if value >= 1_000_000.0 {
        format!("{}M", trim_number(value / 1_000_000.0))
    } else if value >= 1_000.0 {
        format!("{}K", trim_number(value / 1_000.0))
    } else {
        number_to_string(value)
    })
}

/// Port of `currencySymbol` (`src/gateway-model.ts:1599`).
pub fn currency_symbol(currency: String) -> Result<String, String> {
    Ok(currency_symbol_str(&currency))
}

/// Port of `formatMoney` (`src/gateway-model.ts:1612`).
pub fn format_money(amount: f64, currency: String) -> Result<String, String> {
    Ok(format!(
        "{}{}",
        currency_symbol_str(&currency),
        js_to_fixed(amount, 2)
    ))
}

/// Port of `cacheHitRate` (`src/gateway-model.ts:1617`).
pub fn cache_hit_rate(counts: Value) -> Result<Value, String> {
    let inp = to_number(prop(&counts, "inputTokens")?);
    if inp > 0.0 {
        let cached = to_number(prop(&counts, "cachedTokens")?);
        Ok(js_number(cached / inp))
    } else {
        Ok(Value::Null)
    }
}

/// Port of `averageDuration` (`src/gateway-model.ts:1622`).
pub fn average_duration(counts: Value) -> Result<Value, String> {
    let timed = prop(&counts, "timedCalls")?;
    if !is_truthy(timed) {
        return Ok(Value::Null);
    }
    let dur = match get_opt(&counts, "durationMs")? {
        None | Some(Value::Null) => 0.0,
        Some(v) => to_number(v),
    };
    Ok(js_number(dur / to_number(timed)))
}

/// Port of `costText` (`src/gateway-model.ts:1627`).
pub fn cost_text(costs: Value) -> Result<String, String> {
    let mut parts = Vec::new();
    for c in nil_arr(&costs, "filter")? {
        let amt = to_number(prop(c, "amount")?);
        if amt > 0.0 {
            let cur = req_str(c, "currency", "toUpperCase")?;
            parts.push(format!(
                "{}{}",
                currency_symbol_str(cur),
                js_to_fixed(amt, 2)
            ));
        }
    }
    Ok(parts.join(" + "))
}

/// Port of `niceCeiling` (`src/gateway-model.ts:1632`).
pub fn nice_ceiling(value: f64) -> Result<f64, String> {
    if value <= 0.0 {
        return Ok(1.0);
    }
    let power = 10.0_f64.powf(value.log10().floor());
    for step in [1.0, 2.0, 5.0, 10.0] {
        if step * power >= value {
            return Ok(step * power);
        }
    }
    Ok(10.0 * power)
}

/// Port of `usageDayStats` (`src/gateway-model.ts:1642`).
pub fn usage_day_stats(days: Value) -> Result<Value, String> {
    let arr = req_arr(&days, "filter")?;
    let mut active_days = 0usize;
    let mut peak: Option<&Value> = None;
    let mut best_total = 0.0_f64;
    let mut sum = 0.0_f64;
    for day in arr {
        if to_number(prop(day, "calls")?) > 0.0 {
            active_days += 1;
            let total =
                to_number(prop(day, "inputTokens")?) + to_number(prop(day, "outputTokens")?);
            if peak.is_none() || total > best_total {
                peak = Some(day);
                best_total = total;
            }
            sum += total;
        }
    }
    let avg = if active_days > 0 {
        sum / (active_days as f64)
    } else {
        0.0
    };
    Ok(json!({
        "activeDays": active_days,
        "peak": peak.cloned().unwrap_or(Value::Null),
        "dailyAverage": js_number(avg),
    }))
}

/// Port of `axisLabelIndexes` (`src/gateway-model.ts:1651`).
pub fn axis_label_indexes(length: f64, count: Value) -> Result<Vec<f64>, String> {
    let cnt = if is_undefined(&count) {
        7.0
    } else {
        to_number(&count)
    };
    if length <= 0.0 {
        return Ok(Vec::new());
    }
    if length <= cnt {
        if !length.is_finite() || length > f64::from(u32::MAX) {
            return Err("RangeError: Invalid array length".into());
        }
        let len_u = length.floor() as usize;
        return Ok((0..len_u).map(|i| i as f64).collect());
    }
    if cnt <= 0.0 || cnt.is_nan() {
        return Ok(Vec::new());
    }
    if !cnt.is_finite() || cnt > f64::from(u32::MAX) {
        return Err("RangeError: Invalid array length".into());
    }
    let cnt_u = cnt.floor() as usize;
    let step = (length - 1.0) / (cnt - 1.0);
    let mut out = Vec::new();
    let mut seen = HashSet::new();
    for i in 0..cnt_u {
        let v = js::math_round((i as f64) * step);
        let norm = if v == 0.0 { 0.0 } else { v };
        let key = if norm.is_nan() {
            u64::MAX
        } else {
            norm.to_bits()
        };
        if seen.insert(key) {
            out.push(norm);
        }
    }
    Ok(out)
}

/// Port of `shortDate` (`src/gateway-model.ts:1659`).
pub fn short_date(date: String, english: Value) -> Result<String, String> {
    let mut parts = date.split('-').map(parse_js_number);
    let _ = parts.next();
    let month = parts.next().unwrap_or(f64::NAN);
    let day = parts.next().unwrap_or(f64::NAN);
    if month == 0.0 || month.is_nan() || day == 0.0 || day.is_nan() {
        return Ok(date);
    }
    let day_s = number_to_string(day);
    if is_truthy(&english) {
        let idx = month - 1.0;
        let m_s = if idx.fract() == 0.0 && idx >= 0.0 && (idx as usize) < MONTHS_EN.len() {
            MONTHS_EN[idx as usize]
        } else {
            "undefined"
        };
        return Ok(format!("{m_s} {day_s}"));
    }
    Ok(format!("{}月{day_s}日", number_to_string(month)))
}

/// Port of `filterSessions` (`src/gateway-model.ts:1673`).
pub fn filter_sessions(sessions: Value, agent: Value, query: String) -> Result<Vec<Value>, String> {
    let needle = es_trim(&query).to_lowercase();
    let mut out = Vec::new();
    for s in req_arr(&sessions, "filter")? {
        let s_agent = prop(s, "agent")?;
        if val_eq_str(&agent, "all") || s_agent == &agent {
            let hit = needle.is_empty()
                || [prop(s, "title")?, prop(s, "cwd")?, prop(s, "id")?]
                    .into_iter()
                    .map(|v| Ok(val_str_lossy(v)?.to_lowercase().contains(&needle)))
                    .collect::<Result<Vec<_>, String>>()?
                    .into_iter()
                    .any(|b| b);
            if hit {
                out.push(s.clone());
            }
        }
    }
    let mut keyed: Vec<(f64, Value)> = Vec::with_capacity(out.len());
    for s in out {
        let ts = to_number(prop(&s, "updatedAtMs")?);
        keyed.push((ts, s));
    }
    js::js_sort(&mut keyed, |a, b| b.0 - a.0);
    Ok(keyed.into_iter().map(|(_, s)| s).collect())
}

/// Port of `shortPath` (`src/gateway-model.ts:1682`).
pub fn short_path(path: String, home: Option<String>) -> Result<String, String> {
    let trimmed = es_trim(&path);
    let mut text = if let Some(rest) = trimmed.strip_prefix(r"\\?\UNC\") {
        format!(r"\\{rest}")
    } else if let Some(rest) = trimmed.strip_prefix(r"\\?\")
        && rest.as_bytes().first().is_some_and(u8::is_ascii_alphabetic)
        && rest.as_bytes().get(1) == Some(&b':')
    {
        rest.to_string()
    } else {
        trimmed.to_string()
    };
    let h = home.unwrap_or_default();
    if !h.is_empty() && text.starts_with(&h) {
        text = format!("~{}", &text[h.len()..]);
    } else if let Some(rest) = text
        .strip_prefix("/Users/")
        .or_else(|| text.strip_prefix("/home/"))
        && !rest.is_empty()
        && !rest.starts_with('/')
    {
        let after = rest.find('/').map_or("", |i| &rest[i..]);
        text = format!("~{after}");
    }
    let sep = if !text.contains('/') && text.contains('\\') {
        "\\"
    } else {
        "/"
    };
    let parts: Vec<&str> = text.split(['\\', '/']).filter(|s| !s.is_empty()).collect();
    if parts.len() > 4 {
        Ok(format!("…{sep}{}", parts[parts.len() - 2..].join(sep)))
    } else {
        Ok(text)
    }
}

/// Port of `relativeTime` (`src/gateway-model.ts:1693`).
pub fn relative_time(ms: f64, now: f64) -> Result<Value, String> {
    let diff = js_max(0.0, now - ms);
    let minute = 60_000.0;
    if diff < minute {
        return Ok(json!({ "text": "刚刚", "args": [] }));
    }
    if diff < 60.0 * minute {
        return Ok(json!({ "text": "{0} 分钟前", "args": [js_number((diff / minute).floor())] }));
    }
    if diff < 24.0 * 60.0 * minute {
        return Ok(
            json!({ "text": "{0} 小时前", "args": [js_number((diff / (60.0 * minute)).floor())] }),
        );
    }
    if diff < 30.0 * 24.0 * 60.0 * minute {
        return Ok(
            json!({ "text": "{0} 天前", "args": [js_number((diff / (24.0 * 60.0 * minute)).floor())] }),
        );
    }
    let day = local_day_str(local_datetime_from_ms(ms));
    Ok(json!({ "text": "{0}", "args": [day] }))
}

fn mask_token(value: &str) -> String {
    let chars: Vec<char> = value.chars().collect();
    if chars.len() <= 4 {
        let first = chars.first().map(|c| c.to_string()).unwrap_or_default();
        return format!("{first}***");
    }
    let head: String = chars[..2].iter().collect();
    let tail: String = chars[chars.len() - 2..].iter().collect();
    format!("{head}****{tail}")
}

fn is_email_lead_excluded(c: char) -> bool {
    c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '%' | '+' | '*' | '-')
}

fn is_email_local(b: u8) -> bool {
    b.is_ascii_alphanumeric() || matches!(b, b'.' | b'_' | b'%' | b'+' | b'-')
}

fn is_email_domain(b: u8) -> bool {
    b.is_ascii_alphanumeric() || matches!(b, b'.' | b'-')
}

fn mask_emails_in(text: &str) -> String {
    let bytes = text.as_bytes();
    let mut out = String::with_capacity(text.len());
    let mut pos = 0usize;
    let mut prev_ok = true;
    while pos < bytes.len() {
        if prev_ok && is_email_local(bytes[pos]) {
            let mut at = pos;
            while at < bytes.len() && is_email_local(bytes[at]) {
                at += 1;
            }
            if at < bytes.len() && bytes[at] == b'@' {
                let dom_start = at + 1;
                let mut dom_max = dom_start;
                while dom_max < bytes.len() && is_email_domain(bytes[dom_max]) {
                    dom_max += 1;
                }
                let mut matched_end = None;
                let mut end = dom_max;
                while end >= dom_start + 4 {
                    if bytes[end - 1].is_ascii_alphabetic() {
                        let mut alpha_start = end - 1;
                        while alpha_start > dom_start
                            && bytes[alpha_start - 1].is_ascii_alphabetic()
                        {
                            alpha_start -= 1;
                        }
                        if alpha_start > dom_start
                            && bytes[alpha_start - 1] == b'.'
                            && end - alpha_start >= 2
                        {
                            matched_end = Some(end);
                            break;
                        }
                        end = alpha_start;
                    } else {
                        end -= 1;
                    }
                }
                if let Some(m_end) = matched_end {
                    out.push_str(&mask_token(&text[pos..at]));
                    out.push_str(&text[at..m_end]);
                    pos = m_end;
                    prev_ok = text[..pos]
                        .chars()
                        .next_back()
                        .is_none_or(|c| !is_email_lead_excluded(c));
                    continue;
                }
            }
            out.push_str(&text[pos..at]);
            pos = at;
            prev_ok = text[..pos]
                .chars()
                .next_back()
                .is_none_or(|c| !is_email_lead_excluded(c));
        } else {
            let ch = text[pos..].chars().next().unwrap_or('\0');
            out.push(ch);
            pos += ch.len_utf8();
            prev_ok = !is_email_lead_excluded(ch);
        }
    }
    out
}

fn mask_account_name_inner(name: &str) -> String {
    let masked = mask_emails_in(name);
    let Some(rest) = ["ChatGPT", "Copilot", "SuperGrok", "GitHub Copilot"]
        .iter()
        .find_map(|p| masked.strip_prefix(p))
    else {
        return masked;
    };
    let Some(dot_idx) = rest.find('·') else {
        return masked;
    };
    let after_dot_offset = masked.len() - rest.len() + dot_idx + '·'.len_utf8();
    let after_dot = &masked[after_dot_offset..];
    let ws_len = after_dot
        .char_indices()
        .find(|(_, c)| !is_es_ws(*c))
        .map_or(after_dot.len(), |(i, _)| i);
    let (split_at, tail) = if ws_len < after_dot.len() {
        (after_dot_offset + ws_len, &after_dot[ws_len..])
    } else if let Some(last_ch) = after_dot.chars().next_back()
        && !is_line_term(last_ch)
    {
        let start = after_dot.len() - last_ch.len_utf8();
        (after_dot_offset + start, &after_dot[start..])
    } else {
        return masked;
    };
    if tail.chars().any(is_line_term) || tail.contains('@') || tail.contains('*') {
        return masked;
    }
    format!("{}{}", &masked[..split_at], mask_token(es_trim(tail)))
}

/// Port of `maskAccountName` (`src/gateway-model.ts:1719`).
pub fn mask_account_name(name: String) -> Result<String, String> {
    Ok(mask_account_name_inner(&name))
}

/// Port of `maskAccountText` (`src/gateway-model.ts:1731`).
pub fn mask_account_text(text: String) -> Result<String, String> {
    let mut out = String::with_capacity(text.len());
    let mut pos = 0usize;
    while let Some(open_rel) = text[pos..].find('「') {
        let open = pos + open_rel;
        let inner_start = open + '「'.len_utf8();
        if let Some(close_rel) = text[inner_start..].find('」')
            && close_rel > 0
        {
            out.push_str(&mask_emails_in(&text[pos..open]));
            out.push('「');
            out.push_str(&mask_account_name_inner(
                &text[inner_start..inner_start + close_rel],
            ));
            out.push('」');
            pos = inner_start + close_rel + '」'.len_utf8();
            continue;
        }
        out.push_str(&mask_emails_in(&text[pos..inner_start]));
        pos = inner_start;
    }
    out.push_str(&mask_emails_in(&text[pos..]));
    Ok(out)
}

/// Port of `importCandidates` (`src/gateway-model.ts:1759`).
pub fn import_candidates(payload: Value) -> Result<Vec<Value>, String> {
    let mut out = Vec::new();
    let pooled = |source: &str, id: &Value| -> Result<bool, String> {
        if is_nil(&payload) {
            return Ok(false);
        }
        let target = format!("{source}:{}", js_str(id));
        for entry in req_arr(prop(&payload, "pool")?, "some")? {
            if get_str_opt(entry, "source")? == Some(target.as_str()) {
                return Ok(true);
            }
        }
        Ok(false)
    };
    let opt_list = |k: &str| -> Result<&[Value], String> {
        if is_nil(&payload) {
            return Ok(&[]);
        }
        match get_opt(&payload, k)? {
            None | Some(Value::Null) => Ok(&[]),
            Some(v) => req_arr(v, "map"),
        }
    };
    let push_plain = |out: &mut Vec<Value>, source: &str, key: &str| -> Result<(), String> {
        for item in opt_list(key)? {
            let id = prop(item, "id")?.clone();
            let name = prop(item, "name")?.clone();
            let imported = pooled(source, &id)?;
            out.push(json!({ "source": source, "id": id, "name": name, "imported": imported }));
        }
        Ok(())
    };
    push_plain(&mut out, "codex", "codexProviders")?;
    for item in opt_list("codexAggregates")? {
        out.push(json!({
            "source": "aggregate",
            "id": prop(item, "id")?,
            "name": prop(item, "name")?,
            "imported": prop(item, "imported")?,
        }));
    }
    push_plain(&mut out, "claude", "claudeProviders")?;
    push_plain(&mut out, "grok", "grokProfiles")?;
    push_plain(&mut out, "opencode", "opencodeKeys")?;
    Ok(out)
}

/// Port of `pendingImportCount` (`src/gateway-model.ts:1773`).
pub fn pending_import_count(payload: Value) -> Result<f64, String> {
    let mut count = 0usize;
    for item in import_candidates(payload)? {
        if !is_truthy(prop(&item, "imported")?) {
            count += 1;
        }
    }
    Ok(count as f64)
}

fn pinned_bare(payload: &Value, model: &str) -> Result<Option<String>, String> {
    let Some((prefix, rest)) = model.split_once('/') else {
        return Ok(None);
    };
    if prefix.is_empty() {
        return Ok(None);
    }
    let id_lo = prefix.to_lowercase();
    let rest_lo = rest.to_lowercase();
    let empty_excl = Value::Array(Vec::new());
    for item in pool_providers_refs(payload, &empty_excl)? {
        if val_str_lossy(prop(item, "id")?)?.to_lowercase() == id_lo {
            for known in req_arr(prop(item, "models")?, "some")? {
                if val_str_lossy(known)?.to_lowercase() == rest_lo {
                    return Ok(Some(rest.to_string()));
                }
            }
            return Ok(None);
        }
    }
    Ok(None)
}

fn pinned_copies(payload: &Value, bare: &str) -> Result<Vec<String>, String> {
    let empty_excl = Value::Array(Vec::new());
    let serving = serving_providers_refs(payload, bare, &empty_excl)?;
    if serving.len() <= 1 {
        return Ok(Vec::new());
    }
    let mut out = Vec::with_capacity(serving.len());
    for entry in serving {
        out.push(format!("{}/{bare}", js_str(prop(entry, "id")?)));
    }
    Ok(out)
}

fn is_favorite_model_bool(
    favorites: &[Value],
    model: &str,
    payload: &Value,
) -> Result<bool, String> {
    if favorites.iter().any(|x| val_eq_str(x, model)) {
        return Ok(true);
    }
    let Some(bare) = pinned_bare(payload, model)? else {
        return Ok(false);
    };
    let bare_lo = bare.to_lowercase();
    let mut has_bare = false;
    for item in favorites {
        if val_str_lossy(item)?.to_lowercase() == bare_lo {
            has_bare = true;
            break;
        }
    }
    if !has_bare {
        return Ok(false);
    }
    let copies = pinned_copies(payload, &bare)?;
    Ok(!copies
        .iter()
        .any(|copy| favorites.iter().any(|f| val_eq_str(f, copy))))
}

/// Port of `isFavoriteModel` (`src/gateway-model.ts:1799`).
pub fn is_favorite_model(favorites: Value, model: String, payload: Value) -> Result<bool, String> {
    let favs = req_arr(&favorites, "includes")?;
    is_favorite_model_bool(favs, &model, &payload)
}

/// Port of `toggleFavoriteModel` (`src/gateway-model.ts:1807`).
pub fn toggle_favorite_model(
    favorites: Value,
    model: String,
    payload: Value,
) -> Result<Vec<Value>, String> {
    let favs = req_arr(&favorites, "includes")?;
    if !is_favorite_model_bool(favs, &model, &payload)? {
        let mut out = favs.to_vec();
        out.push(Value::String(model));
        return Ok(out);
    }
    if let Some(bare) = pinned_bare(&payload, &model)?
        && !favs.iter().any(|x| val_eq_str(x, &model))
    {
        let mut out = favs.to_vec();
        for copy in pinned_copies(&payload, &bare)? {
            if copy != model && !favs.iter().any(|x| val_eq_str(x, &copy)) {
                out.push(Value::String(copy));
            }
        }
        return Ok(out);
    }
    let mut next: Vec<Value> = favs
        .iter()
        .filter(|x| !val_eq_str(x, &model))
        .cloned()
        .collect();
    let copies = pinned_copies(&payload, &model)?;
    if !copies.is_empty()
        && !copies
            .iter()
            .any(|copy| favs.iter().any(|x| val_eq_str(x, copy)))
    {
        next.extend(copies.into_iter().map(Value::String));
    }
    Ok(next)
}

/// Port of `activeAutoGroup` (`src/gateway-model.ts:1825`).
pub fn active_auto_group(payload: Value, model: String) -> Result<Value, String> {
    if !is_truthy(&payload) {
        return Ok(Value::Null);
    }
    let gw = prop(&payload, "gateway")?;
    if prop(gw, "autoModels")? == &Value::Bool(false) {
        return Ok(Value::Null);
    }
    let auto_groups = match get_opt(&payload, "autoGroups")? {
        None | Some(Value::Null) => return Ok(Value::Null),
        Some(v) => req_arr(v, "find")?,
    };
    let model_lo = model.to_lowercase();
    for g in auto_groups {
        if val_str_lossy(prop(g, "name")?)?.to_lowercase() == model_lo {
            return Ok(if !is_truthy(prop(g, "hidden")?) {
                g.clone()
            } else {
                Value::Null
            });
        }
    }
    Ok(Value::Null)
}

/// Port of `memberTarget` (`src/gateway-model.ts:1848`).
pub fn member_target(payload: Value, model: String) -> Result<Value, String> {
    let trimmed = es_trim(&model);
    let empty_excl = Value::Array(Vec::new());
    if let Some(q) = qualified_provider_val(&payload, trimmed, &empty_excl)? {
        return Ok(q);
    }
    let serving = serving_providers_refs(&payload, &model, &empty_excl)?;
    Ok(if serving.len() == 1 {
        json!({ "entry": serving[0], "model": trimmed })
    } else {
        Value::Null
    })
}
