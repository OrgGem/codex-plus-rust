//! The Rust port of `src/config-doctor-model.ts`.

#![forbid(unsafe_code)]

use crate::js;
use chrono::{DateTime, Datelike, Local, NaiveDate, NaiveDateTime, NaiveTime, Timelike};
use manager_common_rs::{
    as_number, is_nil, is_truthy, is_undefined, js_str, js_trim, number_to_string, string_units,
    to_number, undefined,
};
use serde_json::{Map, Value, json};

pub const FINDING_LEVELS: [&str; 3] = ["error", "warn", "info"];
pub const DOCTOR_SOURCES: [&str; 3] = ["current", "local", "custom"];
pub const LOCAL_SOURCE_MISSING: &str = "没检测到本机模型服务（Ollama 11434 / LM Studio 1234）";
pub const CURRENT_SOURCE_MISSING: &str =
    "Codex 还没设默认模型，先在供应商里选一个或启动一次 Codex。";
pub const OLLAMA_DOCS_URL: &str = "https://docs.ollama.com/api/openai-compatible";

fn doctor_text(value: &str, args: Vec<String>) -> Value {
    json!({ "text": value, "args": args })
}

fn level_rank(finding: &Value) -> Result<i32, String> {
    if is_nil(finding) {
        return Err("TypeError: Cannot read properties of null/undefined (reading 'level')".into());
    }
    Ok(match finding.get("level").and_then(Value::as_str) {
        Some("error") => 0,
        Some("warn") => 1,
        Some("info") => 2,
        _ => -1,
    })
}

fn level_label(level: &str) -> &'static str {
    match level {
        "error" => "错误",
        "warn" => "警告",
        _ => "提示",
    }
}

fn normalize_source_value(val: Option<&Value>) -> Result<&'static str, String> {
    match val {
        None | Some(Value::Null) => Ok("current"),
        Some(v) if is_undefined(v) => Ok("current"),
        Some(Value::String(s)) => Ok(match js_trim(s) {
            "local" => "local",
            "custom" => "custom",
            _ => "current",
        }),
        Some(_) => Err("TypeError: (value ?? \"\").trim is not a function".into()),
    }
}

fn trimmed_field(obj: &Value, key: &str) -> Result<String, String> {
    obj.get(key)
        .and_then(Value::as_str)
        .map(|s| js_trim(s).to_string())
        .ok_or_else(|| {
            "TypeError: Cannot read properties of undefined (reading 'trim')".to_string()
        })
}

fn to_fixed_1(x: f64) -> String {
    if x.abs() >= 1e21 {
        return number_to_string(x);
    }
    if x >= 4_503_599_627_370_496.0 {
        return format!("{x:.1}");
    }
    let bits = x.to_bits();
    let exp = ((bits >> 52) & 0x7ff) as i32 - 1023 - 52;
    let mant = u128::from((bits & ((1u64 << 52) - 1)) | (1u64 << 52));
    if exp >= 0 {
        return format!("{}.0", mant << exp);
    }
    let shift = (-exp) as u32;
    let scaled = mant * 10;
    let mut q = scaled >> shift;
    let mask = (1u128 << shift) - 1;
    let half = 1u128 << (shift - 1);
    if (scaled & mask) >= half {
        q += 1;
    }
    format!("{}.{}", q / 10, q % 10)
}

fn parse_js_date_local(raw: &str) -> Option<DateTime<Local>> {
    let s = js_trim(raw);
    if s.is_empty() {
        return None;
    }
    if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
        return Some(dt.with_timezone(&Local));
    }
    if let Ok(dt) = DateTime::parse_from_rfc2822(s) {
        return Some(dt.with_timezone(&Local));
    }
    if let Ok(ndt) = NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%.f")
        .or_else(|_| NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S"))
        .or_else(|_| NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M"))
        .or_else(|_| NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S"))
        .or_else(|_| NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M"))
    {
        return ndt.and_local_timezone(Local).earliest();
    }
    if let Ok(d) = NaiveDate::parse_from_str(s, "%Y-%m-%d") {
        return Some(d.and_time(NaiveTime::MIN).and_utc().with_timezone(&Local));
    }
    if s.len() == 7
        && s.as_bytes()[4] == b'-'
        && let (Ok(y), Ok(m)) = (s[..4].parse::<i32>(), s[5..].parse::<u32>())
        && let Some(d) = NaiveDate::from_ymd_opt(y, m, 1)
    {
        return Some(d.and_time(NaiveTime::MIN).and_utc().with_timezone(&Local));
    }
    if s.len() == 4
        && s.bytes().all(|b| b.is_ascii_digit())
        && let Ok(y) = s.parse::<i32>()
        && let Some(d) = NaiveDate::from_ymd_opt(y, 1, 1)
    {
        return Some(d.and_time(NaiveTime::MIN).and_utc().with_timezone(&Local));
    }
    let trimmed_ascii = raw.trim_end_matches(|c: char| c.is_ascii_whitespace());
    let head = trimmed_ascii.trim_end_matches(|c: char| c.is_ascii_digit());
    let tail = &trimmed_ascii[head.len()..];
    if !tail.is_empty()
        && (head.is_empty() || head.ends_with('-') || head.ends_with(' '))
        && !head.bytes().any(|b| b.is_ascii_digit())
        && let Ok(n) = tail.parse::<i32>()
        && (0..=99).contains(&n)
    {
        let date = match n {
            0 => NaiveDate::from_ymd_opt(2000, 1, 1),
            1..=12 => NaiveDate::from_ymd_opt(2001, n as u32, 1),
            13..=49 => NaiveDate::from_ymd_opt(2000 + n, 1, 1),
            _ => NaiveDate::from_ymd_opt(1900 + n, 1, 1),
        }?;
        return date
            .and_time(NaiveTime::MIN)
            .and_local_timezone(Local)
            .earliest();
    }
    None
}

/// Port of `findingLevelLabel` (`src/config-doctor-model.ts:73`).
pub fn finding_level_label(level: String) -> Result<String, String> {
    Ok(level_label(&level).into())
}

/// Port of `sortFindings` (`src/config-doctor-model.ts:85`).
pub fn sort_findings(findings: Vec<Value>) -> Result<Vec<Value>, String> {
    if findings.len() < 2 {
        return Ok(findings);
    }
    let mut keyed = Vec::with_capacity(findings.len());
    for (idx, item) in findings.into_iter().enumerate() {
        keyed.push((level_rank(&item)?, idx, item));
    }
    keyed.sort_by_key(|(r, idx, _)| (*r, *idx));
    Ok(keyed.into_iter().map(|(_, _, item)| item).collect())
}

/// Port of `groupFindings` (`src/config-doctor-model.ts:93`).
pub fn group_findings(findings: Vec<Value>) -> Result<Vec<Value>, String> {
    for item in &findings {
        level_rank(item)?;
    }
    let mut groups = Vec::new();
    for level in FINDING_LEVELS {
        let items: Vec<Value> = findings
            .iter()
            .filter(|f| f.get("level").and_then(Value::as_str) == Some(level))
            .cloned()
            .collect();
        if !items.is_empty() {
            groups.push(json!({
                "level": level,
                "label": level_label(level),
                "items": items,
            }));
        }
    }
    Ok(groups)
}

/// Port of `summarize` (`src/config-doctor-model.ts:102`).
pub fn summarize(findings: Value) -> Result<Value, String> {
    let (mut err, mut warn, mut info) = (0usize, 0usize, 0usize);
    match &findings {
        Value::Array(arr) => {
            for item in arr {
                if is_nil(item) {
                    return Err(
                        "TypeError: Cannot read properties of null/undefined (reading 'level')"
                            .into(),
                    );
                }
                match item.get("level").and_then(Value::as_str) {
                    Some("error") => err += 1,
                    Some("warn") => warn += 1,
                    Some("info") => info += 1,
                    _ => {}
                }
            }
        }
        v if string_units(v).is_some() => {}
        _ => return Err("TypeError: findings is not iterable".into()),
    }
    Ok(json!({ "error": err, "warn": warn, "info": info }))
}

/// Port of `summaryTotal` (`src/config-doctor-model.ts:112`).
pub fn summary_total(summary: Value) -> Result<f64, String> {
    if is_nil(&summary) {
        return Err("TypeError: Cannot read properties of null/undefined (reading 'error')".into());
    }
    let undef = undefined();
    let get = |k: &str| to_number(summary.get(k).unwrap_or(&undef));
    Ok(get("error") + get("warn") + get("info"))
}

/// Port of `summaryLine` (`src/config-doctor-model.ts:117`).
pub fn summary_line(summary: Value) -> Result<Option<Value>, String> {
    let total = summary_total(summary.clone())?;
    if total == 0.0 || total.is_nan() {
        return Ok(None);
    }
    let undef = undefined();
    let s = |k: &str| js_str(summary.get(k).unwrap_or(&undef));
    Ok(Some(doctor_text(
        "共 {0} 项：{1} 个错误、{2} 个警告、{3} 条提示",
        vec![number_to_string(total), s("error"), s("warn"), s("info")],
    )))
}

/// Port of `presetFields` (`src/config-doctor-model.ts:123`).
pub fn preset_fields(preset: Option<Value>) -> Result<Option<Value>, String> {
    let Some(p) = preset.as_ref().filter(|v| is_truthy(v)) else {
        return Ok(None);
    };
    let undef = undefined();
    Ok(Some(json!({
        "baseUrl": p.get("baseUrl").unwrap_or(&undef),
        "model": p.get("model").unwrap_or(&undef),
    })))
}

/// Port of `matchedPresetId` (`src/config-doctor-model.ts:131`).
pub fn matched_preset_id(presets: Vec<Value>, settings: Value) -> Result<String, String> {
    let norm = |u: &str| js_trim(u).trim_end_matches('/').to_lowercase();
    let undef = undefined();
    for preset in &presets {
        if is_nil(preset) {
            return Err(
                "TypeError: Cannot read properties of null/undefined (reading 'baseUrl')".into(),
            );
        }
        let a = trimmed_field(preset, "baseUrl")?;
        if is_nil(&settings) {
            return Err(
                "TypeError: Cannot read properties of null/undefined (reading 'baseUrl')".into(),
            );
        }
        let b = trimmed_field(&settings, "baseUrl")?;
        if norm(&a) == norm(&b) {
            let id = preset.get("id").unwrap_or(&undef);
            return Ok(if is_nil(id) {
                String::new()
            } else {
                js_str(id)
            });
        }
    }
    Ok(String::new())
}

/// Port of `normalizeSource` (`src/config-doctor-model.ts:136`).
pub fn normalize_source(value: Option<String>) -> Result<String, String> {
    Ok(match value.as_deref().map(js_trim) {
        Some("local") => "local",
        Some("custom") => "custom",
        _ => "current",
    }
    .into())
}

/// Port of `sourceLabel` (`src/config-doctor-model.ts:148`).
pub fn source_label(source: String) -> Result<String, String> {
    Ok(match source.as_str() {
        "local" => "本机模型",
        "custom" => "自定义",
        _ => "复用当前供应商（推荐）",
    }
    .into())
}

/// Port of `sourceHint` (`src/config-doctor-model.ts:160`).
pub fn source_hint(source: String, sources: Option<Value>) -> Result<String, String> {
    let srcs = sources.as_ref().filter(|v| !is_nil(v));
    match source.as_str() {
        "local" => {
            let local = srcs.and_then(|s| s.get("local")).filter(|v| !is_nil(v));
            if local.and_then(|l| l.get("found")).is_some_and(is_truthy) {
                Ok(local
                    .and_then(|l| l.get("label"))
                    .filter(|v| !is_nil(v))
                    .map(js_str)
                    .unwrap_or_default())
            } else {
                Ok(LOCAL_SOURCE_MISSING.into())
            }
        }
        "custom" => Ok("自己填地址、模型和 Key（不占 Codex++ 的额度）。".into()),
        _ => Ok(srcs
            .and_then(|s| s.get("current"))
            .filter(|v| !is_nil(v))
            .and_then(|c| c.get("label"))
            .filter(|v| is_truthy(v))
            .map(js_str)
            .unwrap_or_else(|| "用你现在 Codex 用的供应商和模型，不用再填任何字段。".into())),
    }
}

/// Port of `sourceOptions` (`src/config-doctor-model.ts:172`).
pub fn source_options(sources: Option<Value>) -> Result<Vec<Value>, String> {
    DOCTOR_SOURCES
        .iter()
        .map(|&value| {
            Ok(json!({
                "value": value,
                "label": source_label(value.to_string())?,
                "hint": source_hint(value.to_string(), sources.clone())?,
            }))
        })
        .collect()
}

/// Port of `currentSourceNote` (`src/config-doctor-model.ts:177`).
pub fn current_source_note(current: Option<Value>) -> Result<Value, String> {
    let cur = current.as_ref().filter(|v| !is_nil(v));
    let available = cur.and_then(|c| c.get("available")).is_some_and(is_truthy);
    let label = cur
        .and_then(|c| c.get("label"))
        .filter(|v| is_truthy(v))
        .cloned()
        .unwrap_or_else(|| json!("复用当前供应商（还没读到默认模型）"));
    let hint = if available {
        Value::Null
    } else {
        json!(CURRENT_SOURCE_MISSING)
    };
    Ok(json!({ "label": label, "available": available, "hint": hint }))
}

/// Port of `localSourceNote` (`src/config-doctor-model.ts:187`).
pub fn local_source_note(local: Option<Value>) -> Result<Value, String> {
    let loc = local.as_ref().filter(|v| !is_nil(v));
    let found = loc.and_then(|l| l.get("found")).is_some_and(is_truthy);
    let label = if found {
        loc.and_then(|l| l.get("label"))
            .filter(|v| is_truthy(v))
            .cloned()
            .unwrap_or_else(|| json!(LOCAL_SOURCE_MISSING))
    } else {
        json!(LOCAL_SOURCE_MISSING)
    };
    Ok(json!({ "label": label, "found": found }))
}

/// Port of `ollamaDocsUrl` (`src/config-doctor-model.ts:193`).
pub fn ollama_docs_url(presets: Vec<Value>) -> Result<String, String> {
    for preset in &presets {
        if is_nil(preset) {
            return Err(
                "TypeError: Cannot read properties of null/undefined (reading 'id')".into(),
            );
        }
        if preset.get("id").and_then(Value::as_str) == Some("ollama") {
            if let Some(docs) = preset.get("docsUrl").filter(|v| !is_nil(v)) {
                let s = docs
                    .as_str()
                    .ok_or("TypeError: docsUrl.trim is not a function")?;
                let trimmed = js_trim(s);
                if !trimmed.is_empty() {
                    return Ok(trimmed.into());
                }
            }
            return Ok(OLLAMA_DOCS_URL.into());
        }
    }
    Ok(OLLAMA_DOCS_URL.into())
}

/// Port of `draftFromSettings` (`src/config-doctor-model.ts:199`).
pub fn draft_from_settings(settings: Value) -> Result<Value, String> {
    if is_nil(&settings) {
        return Err(
            "TypeError: Cannot read properties of null/undefined (reading 'enabled')".into(),
        );
    }
    let undef = undefined();
    let source = normalize_source_value(settings.get("source"))?;
    Ok(json!({
        "enabled": settings.get("enabled").unwrap_or(&undef),
        "source": source,
        "baseUrl": settings.get("baseUrl").unwrap_or(&undef),
        "model": settings.get("model").unwrap_or(&undef),
        "apiKey": "",
    }))
}

/// Port of `savedPayload` (`src/config-doctor-model.ts:210`).
pub fn saved_payload(draft: Value) -> Result<Value, String> {
    if is_nil(&draft) {
        return Err(
            "TypeError: Cannot read properties of null/undefined (reading 'enabled')".into(),
        );
    }
    let undef = undefined();
    let source = normalize_source_value(draft.get("source"))?;
    let base_url = trimmed_field(&draft, "baseUrl")?;
    let model = trimmed_field(&draft, "model")?;
    let api_key = trimmed_field(&draft, "apiKey")?;
    let mut map = Map::new();
    map.insert(
        "enabled".into(),
        draft.get("enabled").cloned().unwrap_or(undef),
    );
    map.insert("source".into(), json!(source));
    map.insert("baseUrl".into(), json!(base_url));
    map.insert("model".into(), json!(model));
    if !api_key.is_empty() {
        map.insert("apiKey".into(), json!(api_key));
    }
    Ok(Value::Object(map))
}

/// Port of `aiReady` (`src/config-doctor-model.ts:223`).
pub fn ai_ready(settings: Value) -> Result<bool, String> {
    if is_nil(&settings) {
        return Err(
            "TypeError: Cannot read properties of null/undefined (reading 'enabled')".into(),
        );
    }
    if !settings.get("enabled").is_some_and(is_truthy) {
        return Ok(false);
    }
    if normalize_source_value(settings.get("source"))? != "custom" {
        return Ok(true);
    }
    Ok(!trimmed_field(&settings, "baseUrl")?.is_empty()
        && !trimmed_field(&settings, "model")?.is_empty())
}

/// Port of `aiBlockedHint` (`src/config-doctor-model.ts:230`).
pub fn ai_blocked_hint(settings: Value) -> Result<Option<Value>, String> {
    if is_nil(&settings) {
        return Err(
            "TypeError: Cannot read properties of null/undefined (reading 'enabled')".into(),
        );
    }
    if !settings.get("enabled").is_some_and(is_truthy) {
        return Ok(Some(doctor_text(
            "先启用 AI 解释并保存，再用 AI 深度体检。",
            vec![],
        )));
    }
    if normalize_source_value(settings.get("source"))? == "custom"
        && (trimmed_field(&settings, "baseUrl")?.is_empty()
            || trimmed_field(&settings, "model")?.is_empty())
    {
        return Ok(Some(doctor_text(
            "先填好 Base URL 和模型并保存，再用 AI 深度体检。",
            vec![],
        )));
    }
    Ok(None)
}

/// Port of `aiStatusView` (`src/config-doctor-model.ts:239`).
pub fn ai_status_view(ai: Value) -> Result<Value, String> {
    if is_nil(&ai) {
        return Err(
            "TypeError: Cannot read properties of null/undefined (reading 'status')".into(),
        );
    }
    let undef = undefined();
    let get = |k: &str| ai.get(k).cloned().unwrap_or_else(|| undef.clone());
    let truthy_list =
        |items: Vec<Value>| -> Vec<Value> { items.into_iter().filter(is_truthy).collect() };
    let (model, endpoint, msg) = (get("model"), get("endpoint"), get("message"));
    match ai.get("status").and_then(Value::as_str) {
        Some("ok") => {
            let summary = get("summary");
            let detail = if is_truthy(&summary) { summary } else { msg };
            let elapsed = Value::String(format_elapsed(ai.get("elapsedMs").and_then(as_number))?);
            Ok(json!({
                "tone": "ok",
                "label": "AI 解释完成",
                "detail": detail,
                "facts": truthy_list(vec![model, endpoint, elapsed]),
            }))
        }
        Some("failed") => Ok(json!({
            "tone": "danger",
            "label": "AI 解释失败",
            "detail": msg,
            "facts": truthy_list(vec![model, endpoint]),
        })),
        _ => {
            let detail = if is_truthy(&msg) {
                msg
            } else {
                json!("这次只跑了本地规则体检。")
            };
            Ok(json!({
                "tone": "off",
                "label": "未用 AI 解释",
                "detail": detail,
                "facts": [],
            }))
        }
    }
}

/// Port of `formatElapsed` (`src/config-doctor-model.ts:266`).
pub fn format_elapsed(ms: Option<f64>) -> Result<String, String> {
    let Some(ms) = ms.filter(|v| v.is_finite()) else {
        return Ok("—".into());
    };
    if ms < 1000.0 {
        let rounded = js::math_round(ms).max(0.0);
        return Ok(format!("{} ms", number_to_string(rounded)));
    }
    Ok(format!("{} s", to_fixed_1(ms / 1000.0)))
}

/// Port of `formatCheckedAt` (`src/config-doctor-model.ts:273`).
pub fn format_checked_at(value: Option<String>) -> Result<String, String> {
    let Some(raw) = value.filter(|s| !s.is_empty()) else {
        return Ok(String::new());
    };
    let Some(dt) = parse_js_date_local(&raw) else {
        return Ok(String::new());
    };
    Ok(format!(
        "{:04}-{:02}-{:02} {:02}:{:02}",
        dt.year(),
        dt.month(),
        dt.day(),
        dt.hour(),
        dt.minute()
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_groups_and_summarizes_findings() {
        let items = vec![
            json!({"id":"c","level":"info","title":"c","detail":"c"}),
            json!({"id":"a","level":"error","title":"a","detail":"a"}),
            json!({"id":"b","level":"warn","title":"b","detail":"b"}),
            json!({"id":"d","level":"error","title":"d","detail":"d"}),
        ];
        let sorted = sort_findings(items.clone()).unwrap();
        assert_eq!(
            sorted
                .iter()
                .map(|v| v["id"].as_str().unwrap())
                .collect::<Vec<_>>(),
            vec!["a", "d", "b", "c"]
        );
        let groups = group_findings(items.clone()).unwrap();
        assert_eq!(groups.len(), 3);
        let sum = summarize(Value::Array(items)).unwrap();
        assert_eq!(summary_total(sum.clone()).unwrap(), 4.0);
        assert_eq!(
            summary_line(sum).unwrap().unwrap()["args"],
            json!(["4", "2", "1", "1"])
        );
    }

    #[test]
    fn formats_elapsed_and_checked_at() {
        assert_eq!(format_elapsed(None).unwrap(), "—");
        assert_eq!(format_elapsed(Some(999.5)).unwrap(), "1000 ms");
        assert_eq!(format_elapsed(Some(1150.0)).unwrap(), "1.1 s");
        assert_eq!(format_elapsed(Some(1250.0)).unwrap(), "1.3 s");
        assert_eq!(format_checked_at(Some("not-a-date".into())).unwrap(), "");
        assert_eq!(
            format_checked_at(Some("2026-11-07T09:05:42".into())).unwrap(),
            "2026-11-07 09:05"
        );
    }
}
