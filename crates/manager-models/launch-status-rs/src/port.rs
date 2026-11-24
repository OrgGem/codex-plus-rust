//! The Rust port of `src/launch-status.ts`.

#![forbid(unsafe_code)]

use manager_common_rs::{is_truthy, to_number, undefined};
use serde_json::{Value, json};

/// Port of `launchCompletionNotice` (`src/launch-status.ts:12`).
pub fn launch_completion_notice(
    status: Option<Value>,
    request_started_at_ms: f64,
) -> Result<Value, String> {
    let st = status.as_ref();
    match resolve_launch_status(status.clone(), request_started_at_ms)?.as_str() {
        "pending" | "stale" => Ok(json!({
            "status": "accepted",
            "message": "启动仍在后台进行，可在概览的“最近启动”中查看状态。",
        })),
        "failed" => {
            let msg = st
                .and_then(|v| v.get("message"))
                .filter(|v| is_truthy(v))
                .cloned();
            Ok(
                json!({ "status": "failed", "message": msg.unwrap_or_else(|| json!("Codex 启动失败。")) }),
            )
        }
        _ => {
            let degraded = st.and_then(|v| v.get("status")).and_then(Value::as_str)
                == Some("running_degraded");
            let msg = if degraded {
                "Codex 已启动，增强功能仍在等待页面连接。"
            } else {
                "Codex 已成功启动。"
            };
            Ok(json!({ "status": "ok", "message": msg }))
        }
    }
}

/// Port of `resolveLaunchStatus` (`src/launch-status.ts:24`).
pub fn resolve_launch_status(
    status: Option<Value>,
    request_started_at_ms: f64,
) -> Result<String, String> {
    let Some(s) = status.as_ref().filter(|v| is_truthy(v)) else {
        return Ok("stale".into());
    };
    let undef = undefined();
    if to_number(s.get("started_at_ms").unwrap_or(&undef)) < request_started_at_ms {
        return Ok("stale".into());
    }
    let out = match s.get("status").and_then(Value::as_str) {
        Some("running" | "running_degraded") => "success",
        Some("failed" | "crashed" | "stopped") => "failed",
        _ => "pending",
    };
    Ok(out.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_status_and_builds_notice() {
        let mk = |st: &str, msg: &str, t: i64| {
            Some(json!({"status":st,"message":msg,"started_at_ms":t}))
        };
        assert_eq!(resolve_launch_status(None, 100.0).unwrap(), "stale");
        assert_eq!(
            resolve_launch_status(mk("failed", "x", 99), 100.0).unwrap(),
            "stale"
        );
        assert_eq!(
            resolve_launch_status(mk("starting", "x", 100), 100.0).unwrap(),
            "pending"
        );
        assert_eq!(
            resolve_launch_status(mk("running_degraded", "x", 102), 100.0).unwrap(),
            "success"
        );
        assert_eq!(
            resolve_launch_status(mk("failed", "err", 101), 100.0).unwrap(),
            "failed"
        );
        assert_eq!(
            launch_completion_notice(None, 100.0).unwrap()["status"],
            "accepted"
        );
        assert_eq!(
            launch_completion_notice(mk("failed", "", 101), 100.0).unwrap()["message"],
            "Codex 启动失败。"
        );
        assert_eq!(
            launch_completion_notice(mk("running", "ok", 101), 100.0).unwrap()["status"],
            "ok"
        );
    }
}
