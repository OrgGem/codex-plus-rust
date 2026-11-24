//! The Rust port of `src/tray-panel-model.ts`.

#![forbid(unsafe_code)]

use manager_common_rs::domain::es_trim;
use manager_common_rs::{
    is_nil, is_plain_object, is_truthy, is_undefined, js_number, js_sort, js_str, to_number,
    undefined, utf16_string,
};
use serde_json::{Value, json};
use url::Url;

const TRAY_TABS: &[&str] = &["home", "usage", "gateway", "flow"];
const GATEWAY_RELAY_ID: &str = "codex-plus-gateway";
const TRAY_ROUTES: &[&str] = &[
    "overview",
    "sessions",
    "requestLog",
    "gateway",
    "gatewayProviders",
    "gatewayRouting",
    "gatewayUsage",
    "settings",
];

fn js_max(a: f64, b: f64) -> f64 {
    if a.is_nan() || b.is_nan() {
        f64::NAN
    } else if a > b {
        a
    } else {
        b
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

fn slice_end(len: usize, end: f64) -> usize {
    if end.is_nan() {
        return 0;
    }
    if end.is_infinite() {
        return if end.is_sign_positive() { len } else { 0 };
    }
    let t = end.trunc();
    if t < 0.0 {
        len.saturating_sub((-t) as usize)
    } else if t >= len as f64 {
        len
    } else {
        t as usize
    }
}

fn host_of(url: Option<&str>) -> String {
    let text = es_trim(url.unwrap_or(""));
    if text.is_empty() {
        return String::new();
    }
    let Ok(parsed) = Url::parse(text) else {
        return text.to_string();
    };
    let Some(host) = parsed.host_str() else {
        return String::new();
    };
    match parsed.port() {
        Some(port) => format!("{host}:{port}"),
        None => host.to_string(),
    }
}

fn strip_window_suffix(raw: &str) -> &str {
    if let Some(before_close) = raw.strip_suffix(']')
        && let Some(open_idx) = before_close.rfind('[')
        && !before_close[open_idx + 1..].contains(']')
    {
        return &before_close[..open_idx];
    }
    raw
}

fn relay_kind(profile: &Value) -> &'static str {
    if profile.get("id").and_then(Value::as_str) == Some(GATEWAY_RELAY_ID) {
        return "gateway";
    }
    let mode = profile.get("relayMode").and_then(Value::as_str);
    if mode == Some("aggregate") || profile.get("aggregate").is_some_and(is_truthy) {
        return "aggregate";
    }
    if mode == Some("official") {
        return "official";
    }
    if profile.get("protocol").is_some_and(is_truthy)
        && profile.get("protocol").and_then(Value::as_str) != Some("responses")
    {
        return "conversion";
    }
    "direct"
}

fn opt_str_field<'a>(obj: &'a Value, key: &str, expr: &str) -> Result<&'a str, String> {
    match obj.get(key) {
        None => Ok(""),
        Some(v) if is_nil(v) => Ok(""),
        Some(Value::String(s)) => Ok(s.as_str()),
        Some(_) => Err(format!("TypeError: {expr} is not a function")),
    }
}

/// Port of `restoreTrayTab` (`src/tray-panel-model.ts:12`).
pub fn restore_tray_tab(raw: Value) -> Result<String, String> {
    let tab = raw
        .as_str()
        .filter(|s| TRAY_TABS.contains(s))
        .unwrap_or("home");
    Ok(tab.to_string())
}

/// Port of `codexState` (`src/tray-panel-model.ts:56`).
pub fn codex_state(overview: Value) -> Result<String, String> {
    if !is_truthy(&overview) {
        return Ok("unknown".into());
    }
    if overview
        .get("codex_app")
        .and_then(|c| c.get("status"))
        .and_then(Value::as_str)
        == Some("missing")
    {
        return Ok("missing".into());
    }
    let status = overview
        .get("runtime_health")
        .and_then(|rh| rh.get("codex_app"))
        .and_then(|ca| ca.get("status"));
    if status.and_then(Value::as_str) == Some("running") {
        return Ok("running".into());
    }
    if !status.is_some_and(is_truthy) || status.and_then(Value::as_str) == Some("not_checked") {
        return Ok("unknown".into());
    }
    Ok("stopped".into())
}

/// Port of `codexEnhanced` (`src/tray-panel-model.ts:66`).
pub fn codex_enhanced(overview: Value) -> Result<bool, String> {
    if codex_state(overview.clone())? != "running" {
        return Ok(false);
    }
    Ok(overview
        .get("runtime_health")
        .and_then(|rh| rh.get("debugger"))
        .and_then(|d| d.get("status"))
        .and_then(Value::as_str)
        == Some("connected"))
}

/// Port of `trayRelayItems` (`src/tray-panel-model.ts:122`).
pub fn tray_relay_items(settings: Value) -> Result<Vec<Value>, String> {
    if !is_truthy(&settings) {
        return Ok(Vec::new());
    }
    let active = settings
        .get("activeRelayId")
        .filter(|v| !is_nil(v))
        .cloned()
        .unwrap_or_else(|| Value::String(String::new()));
    let profiles = match settings.get("relayProfiles") {
        None => &[][..],
        Some(v) if is_nil(v) => &[][..],
        Some(Value::Array(arr)) => arr.as_slice(),
        Some(_) => {
            return Err("TypeError: (settings.relayProfiles ?? []).map is not a function".into());
        }
    };
    let mut active_items = Vec::new();
    let mut rest_items = Vec::new();
    for profile in profiles {
        if is_nil(profile) {
            return Err(
                "TypeError: Cannot read properties of null/undefined (reading 'model')".into(),
            );
        }
        let raw_model = opt_str_field(profile, "model", "(profile.model ?? \"\").replace")?;
        let model = es_trim(strip_window_suffix(raw_model));
        let name = match profile.get("name") {
            None => {
                return Err(
                    "TypeError: Cannot read properties of undefined (reading 'trim')".into(),
                );
            }
            Some(v) if is_nil(v) => {
                return Err(
                    "TypeError: Cannot read properties of null/undefined (reading 'trim')".into(),
                );
            }
            Some(Value::String(s)) => es_trim(s),
            Some(_) => return Err("TypeError: profile.name.trim is not a function".into()),
        };
        let base_raw = opt_str_field(profile, "baseUrl", "(profile.baseUrl ?? \"\").trim")?;
        let base_url = es_trim(base_raw);
        let host = host_of(Some(base_url));
        let detail = [model, host.as_str()]
            .into_iter()
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join(" · ");
        let id_val = profile.get("id").cloned().unwrap_or_else(undefined);
        let is_active = id_val == active;
        let item = json!({
            "id": id_val,
            "name": name,
            "detail": detail,
            "baseUrl": base_url,
            "kind": relay_kind(profile),
            "active": is_active,
        });
        if is_active {
            active_items.push(item);
        } else {
            rest_items.push(item);
        }
    }
    active_items.extend(rest_items);
    Ok(active_items)
}

/// Port of `activeRelayItem` (`src/tray-panel-model.ts:139`).
pub fn active_relay_item(settings: Value) -> Result<Value, String> {
    let items = tray_relay_items(settings)?;
    Ok(items
        .into_iter()
        .find(|item| item.get("active") == Some(&Value::Bool(true)))
        .unwrap_or(Value::Null))
}

/// Port of `gatewayConnected` (`src/tray-panel-model.ts:144`).
pub fn gateway_connected(payload: Value) -> Result<bool, String> {
    if is_nil(&payload) {
        return Ok(false);
    }
    let agents = payload
        .get("agents")
        .filter(|v| !is_nil(v))
        .and_then(Value::as_array)
        .ok_or_else(|| "TypeError: payload?.agents.some is not a function".to_string())?;
    for agent in agents {
        if is_nil(agent) {
            return Err(
                "TypeError: Cannot read properties of null/undefined (reading 'enabled')".into(),
            );
        }
        if agent.get("enabled").is_some_and(is_truthy) {
            return Ok(true);
        }
    }
    Ok(false)
}

/// Port of `gatewayIntroNeeded` (`src/tray-panel-model.ts:149`).
pub fn gateway_intro_needed(payload: Value) -> Result<bool, String> {
    if !is_truthy(&payload) {
        return Ok(true);
    }
    let pool = payload.get("pool").filter(|v| !is_nil(v)).ok_or_else(|| {
        "TypeError: Cannot read properties of null/undefined (reading 'length')".to_string()
    })?;
    let len = match pool {
        Value::Array(arr) => Some(arr.len()),
        Value::String(s) => Some(s.encode_utf16().count()),
        _ => None,
    };
    Ok(len == Some(0) && !gateway_connected(payload)?)
}

fn token_of(group: &Value) -> Result<f64, String> {
    if is_nil(group) {
        return Err(
            "TypeError: Cannot read properties of null/undefined (reading 'inputTokens')".into(),
        );
    }
    let input = group.get("inputTokens").map(to_number).unwrap_or(f64::NAN);
    let output = group.get("outputTokens").map(to_number).unwrap_or(f64::NAN);
    Ok(input + output)
}

/// Port of `usageGlance` (`src/tray-panel-model.ts:167`).
pub fn usage_glance(summary: Value, top_count: Value) -> Result<Value, String> {
    if is_nil(&summary) {
        return Ok(Value::Null);
    }
    let Some(totals) = summary.get("totals").filter(|v| is_truthy(v)) else {
        return Ok(Value::Null);
    };
    let count_num = if is_undefined(&top_count) {
        3.0
    } else {
        to_number(&top_count)
    };
    let total_tokens = js_max(1.0, token_of(totals)?);
    let raw_models = match summary.get("byModel") {
        None => &[][..],
        Some(v) if is_nil(v) => &[][..],
        Some(Value::Array(arr)) => arr.as_slice(),
        Some(_) => return Err("TypeError: summary.byModel is not iterable".into()),
    };
    for m in raw_models {
        if is_nil(m) {
            return Err(
                "TypeError: Cannot read properties of null/undefined (reading 'inputTokens')"
                    .into(),
            );
        }
    }
    let mut models: Vec<&Value> = raw_models.iter().collect();
    js_sort(&mut models, |a, b| {
        token_of(b).unwrap_or(f64::NAN) - token_of(a).unwrap_or(f64::NAN)
    });
    let end = slice_end(models.len(), count_num);
    let mut top_models = Vec::new();
    for group in models.into_iter().take(end) {
        let tok = token_of(group)?;
        if tok > 0.0 {
            let key = group.get("key").cloned().unwrap_or_else(undefined);
            let label = group
                .get("label")
                .filter(|v| is_truthy(v))
                .cloned()
                .unwrap_or_else(|| key.clone());
            top_models.push(json!({
                "key": key,
                "label": label,
                "tokens": js_number(tok),
                "share": js_number(tok / total_tokens),
            }));
        }
    }
    let calls = totals.get("calls").map(to_number).unwrap_or(f64::NAN);
    let success_rate = if calls > 0.0 {
        let errs = totals.get("errors").map(to_number).unwrap_or(f64::NAN);
        js_number((calls - errs) / calls)
    } else {
        Value::Null
    };
    let pick = |k: &str| totals.get(k).cloned().unwrap_or_else(undefined);
    Ok(json!({
        "calls": pick("calls"),
        "errors": pick("errors"),
        "successRate": success_rate,
        "inputTokens": pick("inputTokens"),
        "outputTokens": pick("outputTokens"),
        "cachedTokens": pick("cachedTokens"),
        "topModels": top_models,
    }))
}

/// Port of `trayQuotaRows` (`src/tray-panel-model.ts:196`).
pub fn tray_quota_rows(pool: Value, now_ms: Value) -> Result<Vec<Value>, String> {
    if is_nil(&pool) {
        return Ok(Vec::new());
    }
    let entries = pool
        .as_array()
        .ok_or_else(|| "TypeError: (pool ?? []).filter is not a function".to_string())?;
    let now = to_number(&now_ms);
    let mut rows = Vec::new();
    for entry in entries {
        if is_nil(entry) {
            return Err(
                "TypeError: Cannot read properties of null/undefined (reading 'enabled')".into(),
            );
        }
        if entry.get("enabled") == Some(&Value::Bool(false)) {
            continue;
        }
        let Some(quota) = entry.get("quota").filter(|v| is_truthy(v)) else {
            continue;
        };
        let windows = quota
            .get("windows")
            .filter(|v| !is_nil(v))
            .and_then(Value::as_array)
            .ok_or_else(|| {
                "TypeError: Cannot read properties of null/undefined (reading 'length')".to_string()
            })?;
        if windows.is_empty() {
            continue;
        }
        let mut mapped_windows = Vec::with_capacity(windows.len());
        for window in windows {
            if is_nil(window) {
                return Err(
                    "TypeError: Cannot read properties of null/undefined (reading 'resetsAtMs')"
                        .into(),
                );
            }
            let resets = window.get("resetsAtMs");
            let remaining = if resets != Some(&Value::Null)
                && resets.is_some_and(|r| !is_undefined(r) && to_number(r) <= now)
            {
                1.0
            } else {
                let used = window.get("usedPercent").map(to_number).unwrap_or(f64::NAN);
                js_min(1.0, js_max(0.0, 1.0 - used / 100.0))
            };
            mapped_windows.push(json!({
                "window": window,
                "remaining": js_number(remaining),
            }));
        }
        let plan = entry
            .get("plan")
            .filter(|v| !is_nil(v))
            .cloned()
            .unwrap_or_else(|| Value::String(String::new()));
        rows.push(json!({
            "providerId": entry.get("id").cloned().unwrap_or_else(undefined),
            "name": entry.get("name").cloned().unwrap_or_else(undefined),
            "plan": plan,
            "windows": mapped_windows,
        }));
    }
    Ok(rows)
}

/// Port of `isTrayRoute` (`src/tray-panel-model.ts:221`).
pub fn is_tray_route(value: Value) -> Result<bool, String> {
    Ok(value.as_str().is_some_and(|s| TRAY_ROUTES.contains(&s)))
}

/// Port of `parseTrayRequest` (`src/tray-panel-model.ts:231`).
pub fn parse_tray_request(value: Value) -> Result<Value, String> {
    let Some(obj) = value.as_object().filter(|_| is_plain_object(&value)) else {
        return Ok(Value::Null);
    };
    let Some(id_val) = obj.get("id") else {
        return Ok(Value::Null);
    };
    let Some(id_units) = manager_common_rs::string_units(id_val) else {
        return Ok(Value::Null);
    };
    if id_units.is_empty() {
        return Ok(Value::Null);
    }
    let id_out = utf16_string(&id_units);
    match obj.get("kind").and_then(Value::as_str) {
        Some("navigate") => {
            let route = obj.get("route").cloned().unwrap_or_else(undefined);
            if is_tray_route(route.clone())? {
                Ok(json!({ "id": id_out, "kind": "navigate", "route": route }))
            } else {
                Ok(Value::Null)
            }
        }
        Some(kind @ ("launch" | "restart" | "stop")) => Ok(json!({ "id": id_out, "kind": kind })),
        _ => Ok(Value::Null),
    }
}

#[allow(dead_code)]
fn keep_js_str_used(v: &Value) -> String {
    js_str(v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tabs_state_relays_usage_quota_and_requests() {
        assert_eq!(restore_tray_tab(json!("flow")).ok(), Some("flow".into()));
        assert_eq!(
            restore_tray_tab(json!("providers")).ok(),
            Some("home".into())
        );
        assert_eq!(restore_tray_tab(Value::Null).ok(), Some("home".into()));
        assert_eq!(
            codex_enhanced(json!({
                "runtime_health": {
                    "codex_app": {"status": "running"},
                    "debugger": {"status": "connected"}
                }
            }))
            .ok(),
            Some(true)
        );
        let items = tray_relay_items(json!({
            "activeRelayId": "b",
            "relayProfiles": [
                {"id": "a", "name": "DeepSeek", "model": "deepseek-v4-pro[1M]", "baseUrl": "https://api.deepseek.com/v1", "protocol": "responses"},
                {"id": "b", "name": " Kimi ", "model": "kimi-k2", "baseUrl": "https://api.moonshot.cn/v1", "protocol": "chatCompletions"}
            ]
        }))
        .ok()
        .unwrap_or_default();
        assert_eq!(items.len(), 2);
        assert_eq!(items[0]["id"], "b");
        assert_eq!(items[0]["kind"], "conversion");
        assert_eq!(items[1]["detail"], "deepseek-v4-pro · api.deepseek.com");
        assert_eq!(
            parse_tray_request(json!({"id": "1", "kind": "navigate", "route": "gatewayRouting"}))
                .ok(),
            Some(json!({"id": "1", "kind": "navigate", "route": "gatewayRouting"}))
        );
        assert_eq!(
            parse_tray_request(json!({"id": "1", "kind": "navigate", "route": "relay"})).ok(),
            Some(Value::Null)
        );
        assert_eq!(
            usage_glance(Value::Null, undefined()).ok(),
            Some(Value::Null)
        );
    }
}
