//! The Rust port of `src/agent-collaboration-queue.ts`.

#![forbid(unsafe_code)]

use crate::js;
use serde_json::{Value, json};

pub const DEFAULT_MAX_QUEUED_PER_AGENT: f64 = 8.0;
pub const DEFAULT_MAX_CONCURRENT_TASKS: f64 = 1.0;
pub const DEFAULT_MAX_CONCURRENT_TASKS_LIMIT: f64 = 4.0;

fn positive_integer(value: Option<&Value>, fallback: f64) -> f64 {
    value
        .and_then(js::as_number)
        .filter(|&n| n.is_finite() && n.fract() == 0.0 && n > 0.0)
        .unwrap_or(fallback)
}

fn positive_opt(value: Option<f64>, fallback: f64) -> f64 {
    value
        .filter(|&n| n.is_finite() && n.fract() == 0.0 && n > 0.0)
        .unwrap_or(fallback)
}

/// Port of `normalizeConcurrency` (`src/agent-collaboration-queue.ts:35`).
pub fn normalize_concurrency(value: Value, max: Option<Value>) -> Result<f64, String> {
    let val = positive_integer(Some(&value), DEFAULT_MAX_CONCURRENT_TASKS);
    let cap = positive_integer(max.as_ref(), DEFAULT_MAX_CONCURRENT_TASKS_LIMIT);
    Ok(val.min(cap))
}

/// Port of `agentQueueState` (`src/agent-collaboration-queue.ts:39`).
pub fn agent_queue_state(
    tasks: Vec<Value>,
    agent_id: String,
    max_queued: Option<f64>,
    max_concurrent: Option<f64>,
) -> Result<Value, String> {
    let limit = positive_opt(max_queued, DEFAULT_MAX_QUEUED_PER_AGENT);
    let concurrency = positive_opt(max_concurrent, DEFAULT_MAX_CONCURRENT_TASKS);
    let (mut running, mut queued) = (0.0_f64, 0.0_f64);
    for task in &tasks {
        if task.is_null() || js::is_undefined(task) {
            return Err(
                "TypeError: Cannot read properties of null/undefined (reading 'agentId')".into(),
            );
        }
        if task.get("agentId").and_then(Value::as_str) != Some(agent_id.as_str()) {
            continue;
        }
        match task.get("status").and_then(Value::as_str) {
            Some("running") => running += 1.0,
            Some("queued") => queued += 1.0,
            _ => {}
        }
    }
    Ok(json!({
        "active": js::number(running + queued),
        "running": js::number(running),
        "queued": js::number(queued),
        "limit": js::number(limit),
        "concurrency": js::number(concurrency),
        "willQueue": queued > 0.0 || running >= concurrency,
        "full": queued >= limit,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queue_state_and_concurrency() {
        let tasks = vec![
            json!({"agentId": "deepseek", "status": "running"}),
            json!({"agentId": "deepseek", "status": "queued"}),
            json!({"agentId": "deepseek", "status": "completed"}),
            json!({"agentId": "claude", "status": "running"}),
        ];
        assert_eq!(
            agent_queue_state(tasks, "deepseek".into(), Some(8.0), None).unwrap(),
            json!({"active":2,"running":1,"queued":1,"limit":8,"concurrency":1,"willQueue":true,"full":false})
        );
        assert_eq!(normalize_concurrency(js::undefined(), None).unwrap(), 1.0);
        assert_eq!(normalize_concurrency(json!(2.5), None).unwrap(), 1.0);
        assert_eq!(normalize_concurrency(json!(9), None).unwrap(), 4.0);
        assert_eq!(
            normalize_concurrency(json!(3), Some(json!(2))).unwrap(),
            2.0
        );
        assert!(agent_queue_state(vec![Value::Null], "x".into(), None, None).is_err());
    }
}
