//! The Rust port of `src/extension-manager-pages.ts`.

#![forbid(unsafe_code)]

use manager_common_rs::{
    as_number, is_nil, is_plain_object, is_truthy, js_str, js_trim, string_units, undefined,
};
use serde_json::{Map, Value, json};

/// Port of `managerExtensionPages` (`src/extension-manager-pages.ts:38`).
pub fn manager_extension_pages(inventory: Option<Value>) -> Result<Vec<Value>, String> {
    let Some(inv) = inventory.as_ref().filter(|v| is_truthy(v)) else {
        return Ok(Vec::new());
    };
    let inv_obj = inv.as_object().filter(|_| is_plain_object(inv));
    if inv_obj.and_then(|o| o.get("enabled")) == Some(&Value::Bool(false)) {
        return Ok(Vec::new());
    }
    let scripts_val = inv_obj
        .and_then(|o| o.get("scripts"))
        .filter(|v| !is_nil(v));
    let Some(scripts_val) = scripts_val else {
        return Ok(Vec::new());
    };
    let Some(scripts) = scripts_val.as_array() else {
        return Err("TypeError: scripts.filter is not a function".into());
    };

    let mut pages = Vec::new();
    for script in scripts {
        if is_nil(script) {
            return Err(
                "TypeError: Cannot read properties of null/undefined (reading 'enabled')".into(),
            );
        }
        let s_obj = script.as_object().filter(|_| is_plain_object(script));
        if !s_obj.and_then(|o| o.get("enabled")).is_some_and(is_truthy) {
            continue;
        }
        if s_obj.and_then(|o| o.get("error")).is_some_and(is_truthy) {
            continue;
        }
        let pkg = s_obj.and_then(|o| o.get("package")).filter(|v| !is_nil(v));
        let page_opt = pkg
            .and_then(Value::as_object)
            .filter(|_| pkg.is_some_and(is_plain_object))
            .and_then(|o| o.get("managerPage"))
            .filter(|v| is_truthy(v));
        let Some(page) = page_opt else {
            continue;
        };
        let p_obj = page.as_object().filter(|_| is_plain_object(page));
        let key = s_obj
            .and_then(|o| o.get("key"))
            .cloned()
            .unwrap_or_else(undefined);
        let page_title = p_obj.and_then(|o| o.get("title")).filter(|v| is_truthy(v));
        let title = page_title
            .cloned()
            .or_else(|| s_obj.and_then(|o| o.get("name")).cloned())
            .unwrap_or_else(undefined);
        let nav = p_obj
            .and_then(|o| o.get("nav"))
            .filter(|v| !is_nil(v))
            .cloned()
            .unwrap_or(Value::Null);
        let icon = p_obj
            .and_then(|o| o.get("icon"))
            .filter(|v| !is_nil(v))
            .cloned()
            .unwrap_or(Value::Null);
        let replaces = p_obj
            .and_then(|o| o.get("replaces"))
            .filter(|v| !is_nil(v))
            .cloned()
            .unwrap_or_else(|| Value::Array(Vec::new()));
        let data = p_obj
            .and_then(|o| o.get("data"))
            .filter(|v| !is_nil(v))
            .cloned()
            .unwrap_or_else(|| Value::Array(Vec::new()));
        pages.push(json!({
            "key": key,
            "title": title,
            "nav": nav,
            "icon": icon,
            "replaces": replaces,
            "data": data,
        }));
    }
    Ok(pages)
}

/// Port of `managerPageOverrides` (`src/extension-manager-pages.ts:56`).
pub fn manager_page_overrides(pages: Vec<Value>) -> Result<Value, String> {
    let mut overrides = Map::new();
    for page in pages {
        if is_nil(&page) {
            return Err(
                "TypeError: Cannot read properties of null/undefined (reading 'replaces')".into(),
            );
        }
        let replaces = page
            .as_object()
            .filter(|_| is_plain_object(&page))
            .and_then(|o| o.get("replaces"));
        let Some(routes) = replaces.and_then(Value::as_array) else {
            return Err("TypeError: page.replaces is not iterable".into());
        };
        for route in routes {
            let route_key = match route {
                Value::String(s) => s.clone(),
                other => string_units(other)
                    .map(|u| String::from_utf16_lossy(&u))
                    .unwrap_or_else(|| js_str(other)),
            };
            if !overrides.get(&route_key).is_some_and(is_truthy) {
                overrides.insert(route_key, page.clone());
            }
        }
    }
    Ok(Value::Object(overrides))
}

/// Port of `managerNavPages` (`src/extension-manager-pages.ts:67`).
pub fn manager_nav_pages(pages: Vec<Value>, group: String) -> Result<Vec<Value>, String> {
    let mut out = Vec::new();
    for page in pages {
        if is_nil(&page) {
            return Err(
                "TypeError: Cannot read properties of null/undefined (reading 'nav')".into(),
            );
        }
        let nav_matches = page
            .as_object()
            .filter(|_| is_plain_object(&page))
            .and_then(|o| o.get("nav"))
            .and_then(Value::as_str)
            == Some(group.as_str());
        if nav_matches {
            out.push(page);
        }
    }
    Ok(out)
}

fn number_param(val: Option<&Value>) -> Option<f64> {
    val.and_then(as_number).filter(|n| n.is_finite())
}

fn number_param_js(val: Option<&Value>) -> Value {
    match number_param(val) {
        Some(n) => manager_common_rs::js_number(n),
        None => undefined(),
    }
}

fn string_param(val: Option<&Value>) -> Option<String> {
    let v = val?;
    if let Some(s) = v.as_str() {
        let trimmed = js_trim(s);
        if !trimmed.is_empty() {
            return Some(trimmed.to_string());
        }
    }
    None
}

fn string_param_js(val: Option<&Value>) -> Value {
    match string_param(val) {
        Some(s) => Value::String(s),
        None => undefined(),
    }
}

/// Port of `managerDataRequest` (`src/extension-manager-pages.ts:114`).
pub fn manager_data_request(
    name: String,
    params: Value,
    declared: Vec<String>,
    script_key: String,
) -> Result<Value, String> {
    const SOURCES: [&str; 11] = [
        "requestFlow",
        "requestLogs",
        "gatewayUsage",
        "localSessions",
        "gatewayProviders",
        "gatewayRouting",
        "gatewayHealth",
        "gatewayQuota",
        "installedSkills",
        "installedPlugins",
        "agentCollaboration",
    ];
    if !SOURCES.contains(&name.as_str()) || !declared.iter().any(|d| d == &name) {
        return Err(format!("清单 manager.data 没有声明 {name}"));
    }

    let params_obj = || -> Result<Option<&Map<String, Value>>, String> {
        if is_nil(&params) {
            return Err("TypeError: Cannot read properties of null/undefined".into());
        }
        Ok(params.as_object().filter(|_| is_plain_object(&params)))
    };

    let (command, args) = match name.as_str() {
        "requestFlow" => {
            let p = params_obj()?;
            let args = json!({ "limit": number_param_js(p.and_then(|o| o.get("limit"))) });
            ("load_request_flow", args)
        }
        "requestLogs" => {
            let p = params_obj()?;
            let get = |k: &str| p.and_then(|o| o.get(k));
            let args = json!({
                "limit": number_param_js(get("limit")),
                "sinceMs": number_param_js(get("sinceMs")),
                "agent": string_param_js(get("agent")),
                "relayId": string_param_js(get("relayId")),
                "model": string_param_js(get("model")),
            });
            ("load_request_logs", args)
        }
        "gatewayUsage" => {
            let p = params_obj()?;
            let range =
                string_param(p.and_then(|o| o.get("range"))).unwrap_or_else(|| "today".into());
            ("load_gateway_usage", json!({ "range": range }))
        }
        "localSessions" => {
            let p = params_obj()?;
            let offset = number_param(p.and_then(|o| o.get("offset")))
                .map(manager_common_rs::js_number)
                .unwrap_or_else(|| json!(0));
            let limit = number_param(p.and_then(|o| o.get("limit")))
                .map(manager_common_rs::js_number)
                .unwrap_or_else(|| json!(50));
            (
                "list_local_sessions",
                json!({ "request": { "offset": offset, "limit": limit } }),
            )
        }
        "agentCollaboration" => {
            let p = params_obj()?;
            let raw = number_param(p.and_then(|o| o.get("limit"))).unwrap_or(50.0);
            let limit = raw.floor().clamp(1.0, 200.0) as i64;
            (
                "extension_manager_data",
                json!({ "name": name, "params": { "limit": limit }, "key": script_key }),
            )
        }
        _ => (
            "extension_manager_data",
            json!({ "name": name, "params": {}, "key": script_key }),
        ),
    };

    Ok(json!({ "command": command, "args": args }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pages_overrides_and_data_requests() {
        let pages = manager_extension_pages(Some(json!({
            "enabled": true,
            "scripts": [
                { "key": "user:a", "name": "A", "enabled": true, "error": "", "package": { "managerPage": { "title": "看板", "nav": "management", "replaces": ["gatewayUsage"] } } },
                { "key": "user:b", "name": "B", "enabled": true, "error": "", "package": { "managerPage": { "title": "B 页", "replaces": ["gatewayUsage", "hub"] } } },
                { "key": "user:off", "name": "Off", "enabled": false, "package": { "managerPage": { "title": "停用" } } },
            ]
        })))
        .unwrap();
        assert_eq!(pages.len(), 2);
        assert_eq!(
            manager_nav_pages(pages.clone(), "management".into())
                .unwrap()
                .len(),
            1
        );
        let overrides = manager_page_overrides(pages).unwrap();
        assert_eq!(overrides["gatewayUsage"]["key"], "user:a");
        assert_eq!(overrides["hub"]["key"], "user:b");

        let req = manager_data_request(
            "agentCollaboration".into(),
            json!({ "limit": 9999 }),
            vec!["agentCollaboration".into()],
            "user:panel".into(),
        )
        .unwrap();
        assert_eq!(req["command"], "extension_manager_data");
        assert_eq!(req["args"]["params"]["limit"], 200);
        assert!(
            manager_data_request(
                "toString".into(),
                json!({}),
                vec!["toString".into()],
                "k".into()
            )
            .is_err()
        );
    }
}
