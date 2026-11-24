//! The Rust port of `src/multi-launch-model.ts`.

#![forbid(unsafe_code)]

use manager_common_rs::{is_nil, is_plain_object, is_truthy, is_undefined, js_trim, string_units};
use serde_json::{Value, json};

fn require_obj<'a>(
    value: &'a Value,
    prop: &str,
) -> Result<&'a serde_json::Map<String, Value>, String> {
    if is_nil(value) {
        return Err(format!(
            "TypeError: Cannot read properties of null/undefined (reading '{prop}')"
        ));
    }
    Ok(value
        .as_object()
        .filter(|_| is_plain_object(value))
        .unwrap_or_else(|| {
            static EMPTY: std::sync::LazyLock<serde_json::Map<String, Value>> =
                std::sync::LazyLock::new(serde_json::Map::new);
            &EMPTY
        }))
}

fn opt_trimmed_str(
    obj: &serde_json::Map<String, Value>,
    key: &str,
) -> Result<Option<String>, String> {
    let Some(val) = obj.get(key) else {
        return Ok(None);
    };
    if is_nil(val) {
        return Ok(None);
    }
    let Some(units) = string_units(val) else {
        return Err(format!("TypeError: instance.{key}?.trim is not a function"));
    };
    Ok(Some(js_trim(&String::from_utf16_lossy(&units)).to_string()))
}

/// Port of `multiLaunchInstanceLabel` (`src/multi-launch-model.ts:27`).
pub fn multi_launch_instance_label(instance: Value, labels: Value) -> Result<Value, String> {
    let obj = require_obj(&instance, "isMain")?;
    let labels_val = if is_undefined(&labels) {
        json!({ "main": "主 Codex", "fallback": "Codex 实例" })
    } else {
        labels
    };
    if obj.get("isMain").is_some_and(is_truthy) {
        let l_obj = require_obj(&labels_val, "main")?;
        return Ok(l_obj
            .get("main")
            .cloned()
            .unwrap_or_else(crate::js::undefined));
    }
    for key in ["profileName", "label", "instanceId"] {
        if let Some(trimmed) = opt_trimmed_str(obj, key)?
            && !trimmed.is_empty()
        {
            return Ok(Value::String(trimmed));
        }
    }
    let l_obj = require_obj(&labels_val, "fallback")?;
    Ok(l_obj
        .get("fallback")
        .cloned()
        .unwrap_or_else(crate::js::undefined))
}

/// Port of `multiLaunchInstanceStatus` (`src/multi-launch-model.ts:35`).
pub fn multi_launch_instance_status(instance: Value) -> Result<String, String> {
    let obj = require_obj(&instance, "status")?;
    match opt_trimmed_str(obj, "status")? {
        Some(s) if !s.is_empty() => Ok(s.to_lowercase()),
        _ => Ok("unknown".into()),
    }
}

/// Port of `multiLaunchInstanceRunning` (`src/multi-launch-model.ts:39`).
pub fn multi_launch_instance_running(instance: Value) -> Result<bool, String> {
    let obj = require_obj(&instance, "running")?;
    if let Some(Value::Bool(b)) = obj.get("running") {
        return Ok(*b);
    }
    let status = multi_launch_instance_status(instance)?;
    Ok(matches!(
        status.as_str(),
        "running" | "running_degraded" | "ready" | "connected" | "ok"
    ))
}

/// Port of `multiLaunchInstanceVisible` (`src/multi-launch-model.ts:44`).
pub fn multi_launch_instance_visible(instance: Value) -> Result<bool, String> {
    Ok(multi_launch_instance_running(instance.clone())?
        || multi_launch_instance_status(instance)? != "stopped")
}

/// Port of `multiLaunchInstanceIsMain` (`src/multi-launch-model.ts:55`).
pub fn multi_launch_instance_is_main(instance: Value) -> Result<bool, String> {
    let obj = require_obj(&instance, "isMain")?;
    if obj.get("isMain") == Some(&Value::Bool(true)) {
        return Ok(true);
    }
    let id = opt_trimmed_str(obj, "instanceId")?
        .ok_or("TypeError: Cannot read properties of undefined (reading 'trim')")?;
    Ok(id == "main")
}

/// Port of `multiLaunchInstanceCanStop` (`src/multi-launch-model.ts:59`).
pub fn multi_launch_instance_can_stop(instance: Value) -> Result<bool, String> {
    if multi_launch_instance_is_main(instance.clone())? {
        return Ok(false);
    }
    let status = multi_launch_instance_status(instance.clone())?;
    Ok(status != "starting"
        && (multi_launch_instance_running(instance)? || status == "unknown" || status == "failed"))
}

/// Port of `multiLaunchInstanceCanRestart` (`src/multi-launch-model.ts:65`).
pub fn multi_launch_instance_can_restart(instance: Value) -> Result<bool, String> {
    if multi_launch_instance_is_main(instance.clone())? {
        return Ok(false);
    }
    Ok(multi_launch_instance_status(instance)? != "starting")
}

/// Port of `multiLaunchInstanceFailure` (`src/multi-launch-model.ts:76`).
pub fn multi_launch_instance_failure(instance: Value) -> Result<String, String> {
    if multi_launch_instance_running(instance.clone())? {
        return Ok(String::new());
    }
    let Some(result) = instance.get("launchResult").filter(|v| is_truthy(v)) else {
        return Ok(String::new());
    };
    let r_obj = result.as_object().filter(|_| is_plain_object(result));
    let status = r_obj
        .and_then(|o| o.get("status"))
        .and_then(Value::as_str)
        .unwrap_or("");
    if status != "failed" && status != "cleanup_failed" {
        return Ok(String::new());
    }
    let msg = match r_obj {
        Some(o) => opt_trimmed_str(o, "message")?.unwrap_or_default(),
        None => String::new(),
    };
    if msg.is_empty() {
        Ok("Codex 实例启动失败，原因未记录。".into())
    } else {
        Ok(msg)
    }
}

/// Port of `runningInstancesForProfile` (`src/multi-launch-model.ts:89`).
pub fn running_instances_for_profile(
    instances: Vec<Value>,
    profile_id: String,
) -> Result<Vec<Value>, String> {
    let id = js_trim(&profile_id);
    if id.is_empty() {
        return Ok(Vec::new());
    }
    let mut out = Vec::new();
    for item in instances {
        let obj = require_obj(&item, "isMain")?;
        if obj.get("isMain").is_some_and(is_truthy) {
            continue;
        }
        let Some(pid) = opt_trimmed_str(obj, "profileId")? else {
            continue;
        };
        if pid == id && multi_launch_instance_running(item.clone())? {
            out.push(item);
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_main_status_failures_and_profile_filtering() {
        let main = json!({ "instanceId": " main ", "status": "running", "running": true });
        assert!(multi_launch_instance_is_main(main.clone()).unwrap());
        assert!(!multi_launch_instance_can_stop(main.clone()).unwrap());
        assert!(!multi_launch_instance_can_restart(main).unwrap());

        let failed = json!({
            "instanceId": "deepseek",
            "status": "failed",
            "running": false,
            "launchResult": { "status": "failed", "message": "调试端口 9456 未就绪" }
        });
        assert_eq!(
            multi_launch_instance_failure(failed).unwrap(),
            "调试端口 9456 未就绪"
        );

        let list = vec![
            json!({ "instanceId": "main", "isMain": true, "profileId": "ccs-a", "status": "running" }),
            json!({ "instanceId": "a-1", "profileId": " ccs-a ", "status": "running" }),
            json!({ "instanceId": "a-2", "profileId": "ccs-a", "status": "stopped", "running": false }),
        ];
        assert_eq!(
            running_instances_for_profile(list, "ccs-a".into())
                .unwrap()
                .len(),
            1
        );
        assert!(multi_launch_instance_status(Value::Null).is_err());
    }
}
