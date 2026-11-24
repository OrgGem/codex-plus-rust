//! The Rust port of `src/overview-model.ts`.

#![forbid(unsafe_code)]

use crate::js;
use serde_json::{Value, json};

const DAY_MS: f64 = 24.0 * 60.0 * 60.0 * 1000.0;

fn is_js_ws(c: char) -> bool {
    c.is_whitespace() || c == '\u{FEFF}'
}

fn js_trim(s: &str) -> &str {
    s.trim_matches(is_js_ws)
}

fn is_truthy(v: &Value) -> bool {
    match v {
        Value::Null => false,
        Value::Bool(b) => *b,
        Value::Number(n) => n.as_f64().is_some_and(|x| x != 0.0 && !x.is_nan()),
        Value::String(s) => !s.is_empty(),
        Value::Array(_) => true,
        Value::Object(_) => {
            if js::is_undefined(v) {
                false
            } else if let Some(x) = js::as_number(v) {
                x != 0.0 && !x.is_nan()
            } else {
                true
            }
        }
    }
}

fn to_js_number(v: Option<&Value>) -> f64 {
    match v {
        None => f64::NAN,
        Some(Value::Null) => 0.0,
        Some(Value::Bool(b)) => {
            if *b {
                1.0
            } else {
                0.0
            }
        }
        Some(Value::String(s)) => {
            let t = js_trim(s);
            if t.is_empty() {
                0.0
            } else {
                t.parse::<f64>().unwrap_or(f64::NAN)
            }
        }
        Some(val) => {
            if js::is_undefined(val) {
                f64::NAN
            } else {
                js::as_number(val).unwrap_or(f64::NAN)
            }
        }
    }
}

fn to_string_opt(v: Option<&Value>) -> Option<String> {
    match v {
        Some(Value::String(s)) if !s.is_empty() => Some(s.clone()),
        Some(Value::Bool(b)) => Some(b.to_string()),
        Some(val) if !val.is_null() && !js::is_undefined(val) => {
            if let Some(n) = js::as_number(val) {
                if n != 0.0 && !n.is_nan() {
                    Some(js::number_to_string(n))
                } else {
                    None
                }
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Port of `summarizeRecentRequests` (`src/overview-model.ts:24`).
pub fn summarize_recent_requests(
    entries: Vec<Value>,
    now_ms: f64,
    window_ms: Option<f64>,
) -> Result<Value, String> {
    let window = window_ms.unwrap_or(DAY_MS);
    let empty_map = serde_json::Map::new();
    let mut recent: Vec<&serde_json::Map<String, Value>> = Vec::new();
    for entry in &entries {
        if entry.is_null() || js::is_undefined(entry) {
            return Err("TypeError: Cannot read properties of null/undefined".into());
        }
        let obj = entry.as_object().unwrap_or(&empty_map);
        let ts = to_js_number(obj.get("timestampMs"));
        if ts <= now_ms && now_ms - ts <= window {
            recent.push(obj);
        }
    }

    if recent.is_empty() {
        return Ok(json!({
            "total": 0,
            "failed": 0,
            "successRate": null,
            "avgDurationMs": null,
            "topModel": null,
        }));
    }

    let mut succeeded = 0usize;
    let mut duration_sum = 0.0_f64;
    let mut counts: Vec<(String, usize, f64)> = Vec::new();

    for obj in &recent {
        if obj.get("success").is_some_and(is_truthy) {
            succeeded += 1;
        }
        let dur = to_js_number(obj.get("durationMs"));
        // JS Math.max(0, NaN) is NaN; Math.max(0, x) otherwise
        let clamped_dur = if dur.is_nan() { f64::NAN } else { dur.max(0.0) };
        duration_sum += clamped_dur;

        let model_raw = to_string_opt(obj.get("model"))
            .or_else(|| to_string_opt(obj.get("upstreamModel")))
            .unwrap_or_default();
        let model = js_trim(&model_raw).to_string();
        if model.is_empty() {
            continue;
        }
        let ts = to_js_number(obj.get("timestampMs"));
        if let Some((_, count, latest)) = counts.iter_mut().find(|(k, _, _)| k == &model) {
            *count += 1;
            if ts.is_nan() || latest.is_nan() {
                *latest = f64::NAN;
            } else if ts > *latest {
                *latest = ts;
            }
        } else {
            counts.push((model, 1, ts));
        }
    }

    let mut top_model: Option<String> = None;
    let mut best_count = 0usize;
    let mut best_latest = -1.0_f64;

    for (model, count, latest) in &counts {
        if *count > best_count || (*count == best_count && *latest > best_latest) {
            top_model = Some(model.clone());
            best_count = *count;
            best_latest = *latest;
        }
    }

    let total = recent.len();
    let failed = total - succeeded;
    let success_rate = succeeded as f64 / total as f64;
    let avg_duration = js::math_round(duration_sum / total as f64);

    Ok(json!({
        "total": total,
        "failed": failed,
        "successRate": js::number(success_rate),
        "avgDurationMs": js::number(avg_duration),
        "topModel": top_model,
    }))
}
