//! The Rust port of `src/provider-sync-flow.ts`.

#![forbid(unsafe_code)]

use crate::js::undefined;
use manager_common_rs::{
    as_number, is_nil, is_plain_object, is_truthy, spread_object, string_units, utf16_string,
};
use serde_json::{Map, Value, json};

fn bounded_percent(value: Option<&Value>, total: Option<&Value>, start: f64, end: f64) -> f64 {
    let Some(total_num) = total.and_then(as_number).filter(|t| t.is_finite()) else {
        return end;
    };
    if total_num <= 0.0 {
        return end;
    }
    let completed = match value.and_then(as_number).filter(|v| v.is_finite()) {
        Some(v) => v.max(0.0).min(total_num),
        None => 0.0,
    };
    (start + ((end - start) * completed) / total_num).clamp(0.0, 100.0)
}

/// Port of `providerSyncStreamPercent` (`src/provider-sync-flow.ts:28`).
pub fn provider_sync_stream_percent(progress: Value) -> Result<Option<f64>, String> {
    if is_nil(&progress) {
        return Err("TypeError: Cannot read properties of null/undefined (reading 'phase')".into());
    }
    let obj = progress.as_object().filter(|_| is_plain_object(&progress));
    let phase = obj.and_then(|o| o.get("phase")).and_then(Value::as_str);
    let pct = match phase {
        Some("scanning") => bounded_percent(
            obj.and_then(|o| o.get("scannedRolloutFiles")),
            obj.and_then(|o| o.get("totalRolloutFiles")),
            0.0,
            45.0,
        ),
        Some("planning") => 50.0,
        Some("backing_up") => 55.0,
        Some("rewriting") => bounded_percent(
            obj.and_then(|o| o.get("appliedRewriteFiles")),
            obj.and_then(|o| o.get("plannedRewriteFiles")),
            60.0,
            88.0,
        ),
        Some("updating_indexes") => 92.0,
        Some("rolling_back") => 96.0,
        Some("complete") => 100.0,
        _ => return Ok(None),
    };
    Ok(Some(pct))
}

fn spread_js(v: &Value) -> Map<String, Value> {
    if let Some(units) = string_units(v) {
        return units
            .into_iter()
            .enumerate()
            .map(|(i, u)| (i.to_string(), utf16_string(&[u])))
            .collect();
    }
    spread_object(v)
}

/// Port of `resolveProviderSyncCompletion` (`src/provider-sync-flow.ts:47`).
pub fn resolve_provider_sync_completion(
    sync_result: Value,
    cleanup_failure: Option<Value>,
) -> Result<Value, String> {
    let Some(failure) = cleanup_failure.filter(is_truthy) else {
        return Ok(json!({
            "result": sync_result,
            "progressMessage": null,
            "noticeKind": "sync",
        }));
    };
    let failure_obj = failure.as_object().filter(|_| is_plain_object(&failure));
    let status = failure_obj
        .and_then(|o| o.get("status"))
        .cloned()
        .unwrap_or_else(undefined);
    let message = failure_obj
        .and_then(|o| o.get("message"))
        .cloned()
        .unwrap_or_else(undefined);
    let mut merged = spread_js(&sync_result);
    merged.insert("status".into(), status);
    merged.insert("message".into(), message.clone());
    Ok(json!({
        "result": Value::Object(merged),
        "progressMessage": message,
        "noticeKind": "cleanup",
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_phases_and_completion_notices() {
        assert_eq!(
            provider_sync_stream_percent(json!({
                "phase": "scanning", "totalRolloutFiles": 10, "scannedRolloutFiles": 5
            }))
            .unwrap(),
            Some(22.5)
        );
        assert_eq!(
            provider_sync_stream_percent(json!({
                "phase": "rewriting", "plannedRewriteFiles": 0, "appliedRewriteFiles": 0
            }))
            .unwrap(),
            Some(88.0)
        );
        assert_eq!(provider_sync_stream_percent(json!({})).unwrap(), None);
        assert!(provider_sync_stream_percent(Value::Null).is_err());

        let sync_res =
            json!({ "status": "ok", "message": "sync complete", "changedSessionFiles": 2 });
        let ok = resolve_provider_sync_completion(sync_res.clone(), None).unwrap();
        assert_eq!(ok["noticeKind"], "sync");
        assert_eq!(ok["progressMessage"], Value::Null);

        let fail = resolve_provider_sync_completion(
            sync_res,
            Some(json!({ "status": "failed", "message": "cleanup failed" })),
        )
        .unwrap();
        assert_eq!(fail["noticeKind"], "cleanup");
        assert_eq!(fail["result"]["status"], "failed");
        assert_eq!(fail["result"]["changedSessionFiles"], 2);
    }
}
