//! The Rust port of `src/log-view-model.ts`.

#![forbid(unsafe_code)]

use crate::{js, utf16};
use chrono::{Datelike, NaiveDate, NaiveDateTime, Timelike};
use manager_common_rs::{
    as_number, is_nil, is_plain_object, is_undefined, js_number, js_trim, local_datetime_from_ms,
    now_ms, number_to_string, split_crlf_lines, string_units, to_number,
};
use serde_json::{Map, Value, json};

const SUMMARY_LIMIT: usize = 180;

fn matches_ci_or_zh(text: &str, ascii: &[&str], zh: &[&str]) -> bool {
    let lower = text.to_ascii_lowercase();
    ascii.iter().any(|n| lower.contains(n)) || zh.iter().any(|n| text.contains(n))
}

fn is_error_text(text: &str) -> bool {
    matches_ci_or_zh(
        text,
        &[
            "fail",
            "error",
            "panic",
            "crash",
            "denied",
            "refused",
            "exception",
        ],
        &["崩溃", "失败", "出错"],
    )
}

fn scalar_text(value: &Value) -> Option<String> {
    if is_nil(value) {
        return None;
    }
    if let Some(units) = string_units(value) {
        return Some(if units.len() > 60 {
            format!("{}…", String::from_utf16_lossy(&units[..57]))
        } else {
            String::from_utf16_lossy(&units)
        });
    }
    if let Some(n) = as_number(value) {
        return Some(number_to_string(n));
    }
    match value {
        Value::Bool(b) => Some(b.to_string()),
        Value::Array(arr) => Some(format!("[{}]", arr.len())),
        _ if is_plain_object(value) => Some("{…}".into()),
        _ => None,
    }
}

fn stringify_pretty_2(value: &Value, depth: usize, out: &mut String) {
    let push_items = |len: usize,
                      out: &mut String,
                      empty: &str,
                      open: char,
                      close: char,
                      f: &mut dyn FnMut(&mut String, usize)| {
        if len == 0 {
            return out.push_str(empty);
        }
        out.push(open);
        out.push('\n');
        let pad = " ".repeat((depth + 1) * 2);
        for i in 0..len {
            if i > 0 {
                out.push_str(",\n");
            }
            out.push_str(&pad);
            f(out, i);
        }
        out.push('\n');
        out.push_str(&" ".repeat(depth * 2));
        out.push(close);
    };
    match value {
        Value::Null => out.push_str("null"),
        Value::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        Value::Number(n) => {
            let f = n.as_f64().unwrap_or(0.0);
            if f.is_finite() {
                out.push_str(&number_to_string(f));
            } else {
                out.push_str("null");
            }
        }
        Value::String(s) => out.push_str(&serde_json::to_string(s).unwrap_or_default()),
        Value::Array(items) => push_items(items.len(), out, "[]", '[', ']', &mut |o, i| {
            stringify_pretty_2(&items[i], depth + 1, o);
        }),
        Value::Object(map) => {
            let pairs: Vec<(&String, &Value)> = map.iter().collect();
            push_items(pairs.len(), out, "{}", '{', '}', &mut |o, i| {
                o.push_str(&serde_json::to_string(pairs[i].0).unwrap_or_default());
                o.push_str(": ");
                stringify_pretty_2(pairs[i].1, depth + 1, o);
            });
        }
    }
}

/// Port of `summarizeDetail` (`src/log-view-model.ts:41`).
pub fn summarize_detail(detail: Value) -> Result<String, String> {
    if is_nil(&detail) {
        return Ok(String::new());
    }
    let Some(obj) = detail.as_object().filter(|_| is_plain_object(&detail)) else {
        return Ok(scalar_text(&detail).unwrap_or_default());
    };
    let mut entries: Vec<(&String, &Value)> = obj.iter().collect();
    entries.sort_by_key(|(_, v)| usize::from(v.is_array() || is_plain_object(v)));
    let (mut parts, mut total_u16) = (Vec::new(), 0usize);
    for (key, value) in entries {
        let Some(text) = scalar_text(value) else {
            continue;
        };
        let item = format!("{key}={text}");
        total_u16 += usize::from(!parts.is_empty()) * 3 + item.encode_utf16().count();
        parts.push(item);
        if total_u16 > SUMMARY_LIMIT {
            break;
        }
    }
    let joined = parts.join(" · ");
    let units = utf16::units(&joined);
    Ok(if units.len() > SUMMARY_LIMIT {
        format!("{}…", String::from_utf16_lossy(&units[..SUMMARY_LIMIT - 1]))
    } else {
        joined
    })
}

/// Port of `logTone` (`src/log-view-model.ts:58`).
pub fn log_tone(event: String, detail: Value) -> Result<String, String> {
    let record = detail.as_object().filter(|_| is_plain_object(&detail));
    let status = record
        .map(|obj| {
            ["status", "level", "result"]
                .iter()
                .filter_map(|k| {
                    obj.get(*k)
                        .and_then(|v| string_units(v).map(|u| String::from_utf16_lossy(&u)))
                })
                .collect::<Vec<_>>()
                .join(" ")
        })
        .unwrap_or_default();
    let has_error = record
        .and_then(|obj| obj.get("error"))
        .and_then(string_units)
        .is_some_and(|u| !u.is_empty());
    let trimmed = js_trim(&status);
    if is_error_text(&event)
        || has_error
        || trimmed.eq_ignore_ascii_case("failed")
        || trimmed.eq_ignore_ascii_case("error")
    {
        return Ok("error".into());
    }
    let is_warn = matches_ci_or_zh(
        &event,
        &[
            "warn",
            "timeout",
            "timed_out",
            "retry",
            "degrad",
            "fallback",
            "stale",
        ],
        &["超时", "重试", "降级"],
    );
    if is_warn || status.to_ascii_lowercase().contains("warn") {
        Ok("warn".into())
    } else {
        Ok("info".into())
    }
}

/// Port of `parseLogLine` (`src/log-view-model.ts:66`).
pub fn parse_log_line(raw: String, index: f64) -> Result<Value, String> {
    let line = js_trim(&raw);
    if line.starts_with('{')
        && let Ok(Value::Object(record)) = serde_json::from_str::<Value>(line)
        && let Some(Value::String(event)) = record.get("event")
    {
        let detail_opt = record.get("detail");
        let detail_val = detail_opt.cloned().unwrap_or_else(js::undefined);
        let num_field = |k: &str| {
            record
                .get(k)
                .and_then(Value::as_f64)
                .map(js_number)
                .unwrap_or(Value::Null)
        };
        let mut body = String::new();
        if let Some(d) = detail_opt {
            stringify_pretty_2(d, 0, &mut body);
        }
        return Ok(json!({
            "index": js_number(index),
            "timestampMs": num_field("timestamp_ms"),
            "pid": num_field("pid"),
            "event": event,
            "tone": log_tone(event.clone(), detail_val.clone())?,
            "summary": summarize_detail(detail_val)?,
            "body": body,
            "raw": raw,
        }));
    }
    let tone = if is_error_text(line) { "error" } else { "info" };
    Ok(json!({
        "index": js_number(index),
        "timestampMs": null,
        "pid": null,
        "event": "",
        "tone": tone,
        "summary": line,
        "body": line,
        "raw": raw,
    }))
}

/// Port of `parseLogText` (`src/log-view-model.ts:95`).
pub fn parse_log_text(text: String, truncated: Option<bool>) -> Result<Vec<Value>, String> {
    let mut entries = Vec::new();
    for (idx, line) in split_crlf_lines(&text).into_iter().enumerate() {
        if !js_trim(line).is_empty() {
            entries.push(parse_log_line(line.to_string(), (idx + 1) as f64)?);
        }
    }
    if truncated.unwrap_or(false)
        && entries.len() > 1
        && entries[0]
            .get("event")
            .and_then(Value::as_str)
            .is_none_or(str::is_empty)
    {
        entries.remove(0);
    }
    entries.reverse();
    Ok(entries)
}

/// Port of `filterLogEntries` (`src/log-view-model.ts:104`).
pub fn filter_log_entries(
    entries: Vec<Value>,
    query: String,
    errors_only: bool,
) -> Result<Vec<Value>, String> {
    let needle = js_trim(&query).to_lowercase();
    let mut out = Vec::new();
    for entry in entries {
        if is_nil(&entry) {
            return Err(
                "TypeError: Cannot read properties of null/undefined (reading 'tone')".into(),
            );
        }
        if errors_only && entry.get("tone").and_then(Value::as_str) != Some("error") {
            continue;
        }
        if needle.is_empty() {
            out.push(entry);
            continue;
        }
        let raw = entry
            .get("raw")
            .and_then(string_units)
            .ok_or("TypeError: Cannot read properties of undefined (reading 'toLowerCase')")?;
        if String::from_utf16_lossy(&raw)
            .to_lowercase()
            .contains(&needle)
        {
            out.push(entry);
        }
    }
    Ok(out)
}

fn parse_js_date_ms(val: &Value) -> f64 {
    if is_undefined(val) {
        return now_ms();
    }
    if val.is_null() {
        return 0.0;
    }
    if let Some(n) = as_number(val) {
        return n;
    }
    if let Some(b) = val.as_bool() {
        return if b { 1.0 } else { 0.0 };
    }
    if let Some(s) = val.as_str() {
        if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(s) {
            return dt.timestamp_millis() as f64;
        }
        if let Ok(d) = NaiveDate::parse_from_str(s, "%Y-%m-%d") {
            return d
                .and_hms_opt(0, 0, 0)
                .map_or(f64::NAN, |dt| dt.and_utc().timestamp_millis() as f64);
        }
        if let Ok(dt) = NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S")
            && let Some(local) = dt.and_local_timezone(chrono::Local).earliest()
        {
            return local.timestamp_millis() as f64;
        }
        return f64::NAN;
    }
    let _ = to_number(val);
    f64::NAN
}

/// Port of `formatLogTime` (`src/log-view-model.ts:113`).
pub fn format_log_time(timestamp_ms: Option<f64>, now: Value) -> Result<String, String> {
    let Some(ts) = timestamp_ms.filter(|t| t.is_finite()) else {
        return Ok(String::new());
    };
    let Some(date) = local_datetime_from_ms(ts) else {
        return Ok("NaN-NaN NaN:NaN:NaN".into());
    };
    let same_day = local_datetime_from_ms(parse_js_date_ms(&now)).is_some_and(|t| {
        date.year() == t.year() && date.month() == t.month() && date.day() == t.day()
    });
    let time = format!(
        "{:02}:{:02}:{:02}",
        date.hour(),
        date.minute(),
        date.second()
    );
    Ok(if same_day {
        time
    } else {
        format!("{:02}-{:02} {time}", date.month(), date.day())
    })
}

/// Port of `diagnosticStatusText` (`src/log-view-model.ts:148`).
pub fn diagnostic_status_text(status: String) -> Result<String, String> {
    let mapped = match status.to_ascii_lowercase().as_str() {
        "ok" => "正常",
        "found" => "已找到",
        "installed" => "已安装",
        "running" => "运行中",
        "enabled" => "已启用",
        "ready" => "就绪",
        "missing" | "not_found" | "notfound" => "未找到",
        "stopped" => "已停止",
        "disabled" => "已禁用",
        "unknown" => "未知",
        "failed" => "失败",
        "error" => "出错",
        "user_stopped" => "手动停止",
        _ => return Ok(status),
    };
    Ok(mapped.to_string())
}

fn pick_path<'a>(mut source: &'a Value, path: &[&str]) -> Option<&'a Value> {
    for key in path {
        source = is_plain_object(source)
            .then(|| source.get(*key))
            .flatten()?;
    }
    Some(source)
}

fn status_tone(status: Option<&str>) -> Option<&'static str> {
    let lower = status?.to_ascii_lowercase();
    match lower.as_str() {
        "ok" | "found" | "running" | "installed" | "enabled" | "ready" => Some("ok"),
        "missing" | "stopped" | "disabled" | "unknown" | "not_found" | "notfound" => Some("warn"),
        _ if ["fail", "error", "crash"].iter().any(|k| lower.contains(k)) => Some("danger"),
        _ => None,
    }
}

fn scalar_fact_text(value: Option<&Value>) -> Option<String> {
    match value? {
        Value::String(s) if !s.is_empty() => Some(s.clone()),
        Value::Number(n) => Some(number_to_string(n.as_f64().unwrap_or(0.0))),
        _ => None,
    }
}

fn push_fact_status(
    facts: &mut Vec<Value>,
    label: &str,
    record: Option<&Value>,
    details: &[&str],
) -> Result<(), String> {
    let Some(obj) = record.and_then(|v| v.as_object().filter(|_| is_plain_object(v))) else {
        return Ok(());
    };
    let status = scalar_fact_text(obj.get("status"));
    let rest: Vec<String> = details
        .iter()
        .filter_map(|k| scalar_fact_text(obj.get(*k)))
        .map(diagnostic_status_text)
        .collect::<Result<_, _>>()?;
    if status.is_none() && rest.is_empty() {
        return Ok(());
    }
    let mut fact = Map::new();
    fact.insert("label".into(), Value::String(label.into()));
    fact.insert(
        "status".into(),
        match status.as_deref() {
            Some(s) => Value::String(diagnostic_status_text(s.to_string())?),
            None => js::undefined(),
        },
    );
    let joined = rest.join(" · ");
    fact.insert(
        "value".into(),
        if joined.is_empty() {
            js::undefined()
        } else {
            Value::String(joined)
        },
    );
    fact.insert(
        "tone".into(),
        status_tone(status.as_deref()).map_or_else(js::undefined, |t| Value::String(t.into())),
    );
    facts.push(Value::Object(fact));
    Ok(())
}

/// Port of `diagnosticFacts` (`src/log-view-model.ts:173`).
pub fn diagnostic_facts(report: String) -> Result<Vec<Value>, String> {
    let Ok(parsed) = serde_json::from_str::<Value>(&report) else {
        return Ok(Vec::new());
    };
    if !is_plain_object(&parsed) {
        return Ok(Vec::new());
    }
    let mut facts = Vec::new();
    let push_val = |facts: &mut Vec<Value>, label: &str, val: Option<&Value>| {
        if let Some(shown) = scalar_fact_text(val) {
            facts.push(json!({ "label": label, "value": shown }));
        }
    };
    push_val(&mut facts, "Codex++ 版本", parsed.get("version"));
    push_fact_status(
        &mut facts,
        "Codex 应用",
        pick_path(&parsed, &["overview", "codex_app"]),
        &["path"],
    )?;
    push_val(
        &mut facts,
        "Codex 版本",
        pick_path(&parsed, &["overview", "codex_version"]),
    );
    push_fact_status(
        &mut facts,
        "静默启动入口",
        pick_path(&parsed, &["overview", "silent_shortcut"]),
        &["path"],
    )?;
    push_fact_status(
        &mut facts,
        "管理入口",
        pick_path(&parsed, &["overview", "management_shortcut"]),
        &["path"],
    )?;
    push_fact_status(
        &mut facts,
        "最近一次启动",
        pick_path(&parsed, &["overview", "latest_launch"]),
        &["phase", "message"],
    )?;
    Ok(facts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_filters_formats_and_extracts_facts() {
        let raw = r#"{"timestamp_ms":1793689309389,"pid":17040,"event":"manager.save_settings","detail":{"activeRelayId":"gw","relayProfiles":[1,2],"ok":true}}"#;
        let entry = parse_log_line(raw.into(), 6.0).unwrap();
        assert_eq!(
            entry["summary"],
            "activeRelayId=gw · ok=true · relayProfiles=[2]"
        );
        assert_eq!(
            parse_log_text("bad\n{\"event\":\"a\"}".into(), Some(true))
                .unwrap()
                .len(),
            1
        );
        assert!(
            !format_log_time(Some(1_793_667_907_000.0), json!(1_793_690_000_000.0))
                .unwrap()
                .contains('-')
        );
        assert!(filter_log_entries(vec![Value::Null], String::new(), true).is_err());
        let facts = diagnostic_facts(r#"{"version":"2.2.0","overview":{"latest_launch":{"status":"stopped","phase":"user_stopped"}}}"#.into()).unwrap();
        assert_eq!(facts[1]["status"], "已停止");
        assert_eq!(facts[1]["value"], "手动停止");
    }
}
