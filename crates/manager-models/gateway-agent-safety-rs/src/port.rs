//! The Rust port of `src/gateway-agent-safety.ts`.

#![forbid(unsafe_code)]

use manager_common_rs::domain::is_es_ws;
use manager_common_rs::js_rt::{is_nil, is_plain_object, is_truthy, string_units, undefined};
use serde_json::{Value, json};
use std::sync::LazyLock;

static UNDEF: LazyLock<Value> = LazyLock::new(undefined);

fn field_or_undef<'a>(v: &'a Value, key: &str) -> &'a Value {
    v.as_object()
        .filter(|_| is_plain_object(v))
        .and_then(|m| m.get(key))
        .unwrap_or(&UNDEF)
}

fn eq_ascii(v: &Value, expected: &str) -> bool {
    matches!(v, Value::String(s) if s == expected)
}

fn eq_str_utf16(v: &Value, expected: &str) -> bool {
    let expected_u16: Vec<u16> = expected.encode_utf16().collect();
    string_units(v).is_some_and(|u| u == expected_u16)
}

fn is_es_ws_u16(u: u16) -> bool {
    char::from_u32(u32::from(u)).is_some_and(is_es_ws)
}

fn trim_non_empty_u16(v: &Value, prop: &str) -> Result<bool, String> {
    if is_nil(v) {
        return Err("TypeError: Cannot read properties of null/undefined (reading 'trim')".into());
    }
    let Some(units) = string_units(v) else {
        return Err(format!("TypeError: {prop}.trim is not a function"));
    };
    Ok(units.iter().any(|&u| !is_es_ws_u16(u)))
}

fn presentation(badge: &str, tone: &str, reconnect: bool) -> Value {
    json!({
        "badge": badge,
        "tone": tone,
        "reconnect": reconnect,
    })
}

/// Port of `canReconnectAgent` (`src/gateway-agent-safety.ts:10`).
pub fn can_reconnect_agent(agent: Value, relay_profiles_enabled: Value) -> Result<bool, String> {
    if is_nil(&agent) {
        return Err("TypeError: Cannot read properties of null/undefined (reading 'agent')".into());
    }
    let is_codex = eq_str_utf16(field_or_undef(&agent, "agent"), "codex");
    Ok(!is_codex || relay_profiles_enabled != Value::Bool(false))
}

/// Port of `agentDiagnosticPresentation` (`src/gateway-agent-safety.ts:15`).
pub fn agent_diagnostic_presentation(agent: Value, helper_running: bool) -> Result<Value, String> {
    if is_nil(&agent) {
        return Err(
            "TypeError: Cannot read properties of null/undefined (reading 'enabled')".into(),
        );
    }
    if !is_truthy(field_or_undef(&agent, "enabled")) {
        return Ok(presentation("未接入", "off", false));
    }
    let diag = field_or_undef(&agent, "diagnostics");
    let state = if is_nil(diag) {
        &UNDEF
    } else {
        field_or_undef(diag, "state")
    };
    if eq_ascii(state, "configUnreadable") {
        return Ok(presentation("配置无法读取", "error", false));
    }
    if eq_ascii(state, "configUnverified") {
        return Ok(presentation("配置无法验证", "error", false));
    }
    if eq_ascii(state, "configMissing") {
        return Ok(presentation("配置缺失", "error", true));
    }
    if eq_ascii(state, "configChanged")
        || (!is_truthy(state) && is_truthy(field_or_undef(&agent, "drift")))
    {
        return Ok(presentation("配置被改动", "error", true));
    }
    if !helper_running || eq_ascii(state, "gatewayStopped") {
        return Ok(presentation("网关未运行", "error", false));
    }
    if eq_ascii(state, "upstreamError") {
        return Ok(presentation("上游请求失败", "error", false));
    }
    if eq_ascii(state, "active") {
        return Ok(presentation("请求已到达", "on", false));
    }
    if eq_ascii(state, "disabled") {
        return Ok(presentation("未接入", "off", false));
    }
    if is_truthy(state) && !eq_ascii(state, "awaitingTraffic") {
        return Ok(presentation("状态待确认", "pending", false));
    }
    Ok(presentation("等待请求验证", "pending", false))
}

/// Port of `disconnectPreviewReady` (`src/gateway-agent-safety.ts:30`).
pub fn disconnect_preview_ready(preview: Value, agent: String) -> Result<bool, String> {
    if is_nil(&preview) {
        return Ok(false);
    }
    if !is_truthy(field_or_undef(&preview, "canDisconnect")) {
        return Ok(false);
    }
    if !eq_str_utf16(field_or_undef(&preview, "agent"), &agent) {
        return Ok(false);
    }
    trim_non_empty_u16(field_or_undef(&preview, "previewToken"), "previewToken")
}

/// Port of `fullSwitchReady` (`src/gateway-agent-safety.ts:35`).
pub fn full_switch_ready(preview: Value, agent: String) -> Result<bool, String> {
    if !is_truthy(&preview) {
        return Ok(false);
    }
    if is_truthy(field_or_undef(&preview, "canDisconnect")) {
        return Ok(false);
    }
    if !is_truthy(field_or_undef(&preview, "fullSwitch")) {
        return Ok(false);
    }
    if !eq_str_utf16(field_or_undef(&preview, "agent"), &agent) {
        return Ok(false);
    }
    trim_non_empty_u16(field_or_undef(&preview, "previewToken"), "previewToken")
}

/// Port of `disconnectFileLabel` (`src/gateway-agent-safety.ts:39`).
pub fn disconnect_file_label(kind: Value) -> Result<Value, String> {
    let label = match kind.as_str() {
        Some("create") => "将创建",
        Some("remove") => "将移除",
        Some("modify") => "将修改",
        _ => return Ok(undefined()),
    };
    Ok(json!(label))
}

/// Port of `diagnosticEvidenceLabel` (`src/gateway-agent-safety.ts:47`).
pub fn diagnostic_evidence_label(code: String) -> Result<String, String> {
    let label = match code.as_str() {
        "matches"
        | "matched"
        | "endpointMatched"
        | "endpoint_match"
        | "credential_match"
        | "selection_match"
        | "managed_config_match" => "与网关配置一致",
        "missing" => "配置项缺失",
        "missingFile" | "missing_file" | "fileMissing" => "文件不存在",
        "changed" => "配置项已改变",
        "mismatch" => "与网关配置不符",
        "endpointChanged" | "endpoint_changed" => "地址已改变",
        "unreadable" | "fileUnreadable" | "config_unreadable" => "无法读取",
        "parseError" | "parse_error" => "无法解析",
        "credentialChanged" | "credential_changed" => "本地凭据已改变",
        "credentialMissing" | "credential_missing" => "本地凭据缺失",
        "endpointPortMismatch" | "endpoint_port_mismatch" => "端口与网关配置不符",
        "endpointHostMismatch" | "endpoint_host_mismatch" => "主机与网关配置不符",
        "endpointPathMismatch" | "endpoint_path_mismatch" => "路径与网关配置不符",
        "endpointMissing" | "endpoint_missing" => "地址缺失",
        "selectionChanged" | "selection_changed" => "当前供应商或模型已改变",
        "invalidConfig" | "config_invalid" => "配置格式无效",
        "unsupportedConfig" | "config_unverified" => "此配置暂无法验证",
        "credential_sealed" => "凭据已由系统安全存储加密保存",
        "config_missing" => "配置缺失",
        "unsupported_agent" => "此 Agent 暂无法验证",
        "endpoint_invalid" => "地址格式无效",
        "managed_config_changed" => "托管配置已改变",
        _ => "诊断信息待确认",
    };
    Ok(label.into())
}

/// Port of `diagnosticErrorLabel` (`src/gateway-agent-safety.ts:96`).
pub fn diagnostic_error_label(kind: String) -> Result<String, String> {
    let label = match kind.as_str() {
        "authInvalid" | "auth_invalid" => "认证已失效",
        "rateLimited" | "rate_limited" => "请求频率受限",
        "quotaExhausted" | "quota_exhausted" => "可用额度不足",
        "serverError" | "server_error" => "上游服务异常",
        "clientError" | "client_error" => "上游拒绝请求",
        "transport" => "上游连接失败",
        "cooling" => "上游暂时冷却",
        "upstreamError" | "upstream_error" => "上游请求失败",
        _ => "错误原因待确认",
    };
    Ok(label.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn agent_with_state(state: Option<&str>) -> Value {
        match state {
            Some(s) => {
                json!({ "agent": "opencode", "enabled": true, "diagnostics": { "state": s } })
            }
            None => json!({ "agent": "opencode", "enabled": true }),
        }
    }

    #[test]
    fn checks_reconnect_and_diagnostic_presentations() {
        assert!(!can_reconnect_agent(json!({ "agent": "codex" }), json!(false)).unwrap());
        assert!(can_reconnect_agent(json!({ "agent": "claude" }), json!(false)).unwrap());
        assert!(can_reconnect_agent(json!({ "agent": "codex" }), undefined()).unwrap());
        assert!(can_reconnect_agent(Value::Null, undefined()).is_err());

        assert_eq!(
            agent_diagnostic_presentation(agent_with_state(Some("awaitingTraffic")), true).unwrap(),
            json!({ "badge": "等待请求验证", "tone": "pending", "reconnect": false })
        );
        assert_eq!(
            agent_diagnostic_presentation(agent_with_state(Some("active")), true).unwrap(),
            json!({ "badge": "请求已到达", "tone": "on", "reconnect": false })
        );
        assert_eq!(
            agent_diagnostic_presentation(agent_with_state(Some("active")), false).unwrap()["badge"],
            json!("网关未运行")
        );
        assert_eq!(
            agent_diagnostic_presentation(
                json!({ "agent": "opencode", "enabled": true, "drift": { "endpoint": "x" } }),
                true
            )
            .unwrap()["reconnect"],
            json!(true)
        );
        assert_eq!(
            agent_diagnostic_presentation(
                json!({ "agent": "opencode", "enabled": false, "diagnostics": { "state": "active" } }),
                true
            )
            .unwrap()["badge"],
            json!("未接入")
        );
    }

    #[test]
    fn checks_previews_file_labels_and_diagnostic_codes() {
        let preview = json!({
            "agent": "opencode",
            "previewToken": "snapshot-1",
            "canDisconnect": true,
            "fullSwitch": null,
        });
        assert!(disconnect_preview_ready(preview.clone(), "opencode".into()).unwrap());
        assert!(!disconnect_preview_ready(preview.clone(), "codex".into()).unwrap());
        assert!(!disconnect_preview_ready(Value::Null, "opencode".into()).unwrap());

        let blocked = json!({
            "agent": "codex",
            "previewToken": "snapshot-1",
            "canDisconnect": false,
            "fullSwitch": { "relayId": "direct", "name": "Direct" },
        });
        assert!(full_switch_ready(blocked, "codex".into()).unwrap());
        assert!(!full_switch_ready(preview, "opencode".into()).unwrap());

        assert_eq!(
            disconnect_file_label(json!("create")).unwrap(),
            json!("将创建")
        );
        assert_eq!(
            disconnect_file_label(json!("remove")).unwrap(),
            json!("将移除")
        );
        assert_eq!(
            disconnect_file_label(json!("modify")).unwrap(),
            json!("将修改")
        );
        assert_eq!(disconnect_file_label(json!("other")).unwrap(), undefined());

        assert_eq!(
            diagnostic_evidence_label("endpoint_match".into()).unwrap(),
            "与网关配置一致"
        );
        assert_eq!(
            diagnostic_evidence_label("unknown_code".into()).unwrap(),
            "诊断信息待确认"
        );
        assert_eq!(
            diagnostic_error_label("auth_invalid".into()).unwrap(),
            "认证已失效"
        );
        assert_eq!(
            diagnostic_error_label("unknown_err".into()).unwrap(),
            "错误原因待确认"
        );
    }
}
