//! The Rust port of `src/history-recovery-report.ts`.

#![forbid(unsafe_code)]

use manager_common_rs::{is_nil, undefined};
use serde_json::{Value, json};

/// Port of `historyRecoveryStatusLabel` (`src/history-recovery-report.ts:8`).
pub fn history_recovery_status_label(report: Value) -> Result<Value, String> {
    if is_nil(&report) {
        return Err(
            "TypeError: Cannot read properties of null/undefined (reading 'status')".into(),
        );
    }
    let label = match report.get("status").and_then(Value::as_str) {
        Some("pending") => "后台补载中",
        Some("completed") => "已完成",
        Some("partial") => "部分完成",
        Some("unavailable") => "不可用",
        Some("skipped") => "已跳过",
        _ => return Ok(undefined()),
    };
    Ok(json!(label))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_every_recovery_status_and_rejects_nil() {
        for (status, label) in [
            ("pending", "后台补载中"),
            ("completed", "已完成"),
            ("partial", "部分完成"),
            ("unavailable", "不可用"),
            ("skipped", "已跳过"),
        ] {
            assert_eq!(
                history_recovery_status_label(json!({ "status": status })).unwrap(),
                label
            );
        }
        assert_eq!(
            history_recovery_status_label(json!({})).unwrap(),
            undefined()
        );
        assert!(history_recovery_status_label(Value::Null).is_err());
    }
}
