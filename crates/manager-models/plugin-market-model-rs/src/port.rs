//! The Rust port of `src/plugin-market-model.ts`.

#![forbid(unsafe_code)]

use crate::js::{number, undefined};
use manager_common_rs::{
    as_number, is_nil, is_plain_object, is_truthy, is_undefined, js_str, spread_object,
    string_units, utf16_string,
};
use serde_json::{Map, Value, json};

fn is_js_ws(c: char) -> bool {
    c != '\u{0085}' && (c.is_whitespace() || c == '\u{FEFF}')
}

fn js_trim(s: &str) -> &str {
    s.trim_matches(is_js_ws)
}

fn js_max(a: f64, b: f64) -> f64 {
    if a.is_nan() || b.is_nan() {
        f64::NAN
    } else if a == 0.0 && b == 0.0 {
        if a.is_sign_positive() || b.is_sign_positive() {
            0.0
        } else {
            -0.0
        }
    } else if a > b {
        a
    } else {
        b
    }
}

fn js_min(a: f64, b: f64) -> f64 {
    if a.is_nan() || b.is_nan() {
        f64::NAN
    } else if a == 0.0 && b == 0.0 {
        if a.is_sign_negative() || b.is_sign_negative() {
            -0.0
        } else {
            0.0
        }
    } else if a < b {
        a
    } else {
        b
    }
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

fn js_strict_eq(a: &Value, b: &Value) -> bool {
    if let (Some(na), Some(nb)) = (as_number(a), as_number(b)) {
        return na == nb;
    }
    if is_undefined(a) && is_undefined(b) {
        return true;
    }
    if let (Some(ua), Some(ub)) = (string_units(a), string_units(b)) {
        return ua == ub;
    }
    match (a, b) {
        (Value::Null, Value::Null) => true,
        (Value::Bool(x), Value::Bool(y)) => x == y,
        _ => false,
    }
}

fn join_part(v: Option<&Value>) -> String {
    match v {
        None | Some(Value::Null) => String::new(),
        Some(val) if is_undefined(val) => String::new(),
        Some(val) => string_units(val)
            .map(|u| String::from_utf16_lossy(&u))
            .unwrap_or_else(|| js_str(val)),
    }
}

/// Port of `isPluginMarketSuccess` (`src/plugin-market-model.ts:35`).
pub fn is_plugin_market_success(status: String) -> Result<bool, String> {
    Ok(matches!(status.as_str(), "ok" | "success" | "warning"))
}

/// Port of `mergePluginMarketCatalog` (`src/plugin-market-model.ts:39`).
pub fn merge_plugin_market_catalog(
    catalogs: Value,
    source: String,
    result: Value,
) -> Result<Value, String> {
    if is_nil(&result) {
        return Err(
            "TypeError: Cannot read properties of null/undefined (reading 'status')".into(),
        );
    }
    let obj = result.as_object().filter(|_| is_plain_object(&result));
    let status_ok = obj
        .and_then(|o| o.get("status"))
        .and_then(Value::as_str)
        .is_some_and(|s| matches!(s, "ok" | "success" | "warning"));
    let source_ok =
        obj.and_then(|o| o.get("source")).and_then(Value::as_str) == Some(source.as_str());
    if !status_ok || !source_ok {
        return Ok(catalogs);
    }
    let mut out = spread_js(&catalogs);
    out.insert(source, result);
    Ok(Value::Object(out))
}

/// Port of `filterMarketPlugins` (`src/plugin-market-model.ts:44`).
pub fn filter_market_plugins(
    plugins: Vec<Value>,
    query: String,
    filter: String,
) -> Result<Vec<Value>, String> {
    let lower_q = js_trim(&query).to_lowercase();
    let terms: Vec<&str> = lower_q.split(is_js_ws).filter(|t| !t.is_empty()).collect();
    let mut out = Vec::new();
    for plugin in plugins {
        if is_nil(&plugin) {
            return Err(
                "TypeError: Cannot read properties of null/undefined (reading 'installed')".into(),
            );
        }
        let obj = plugin.as_object().filter(|_| is_plain_object(&plugin));
        let truthy = |k: &str| obj.and_then(|o| o.get(k)).is_some_and(is_truthy);
        if (filter == "available" && truthy("installed"))
            || (filter == "installed" && !truthy("installed"))
            || (filter == "updates" && !truthy("updateAvailable"))
            || (filter == "auth" && !truthy("requiresAuth"))
        {
            continue;
        }
        let tags_val = obj.and_then(|o| o.get("tags"));
        let tags: Vec<Value> = match tags_val {
            Some(Value::Array(arr)) => arr.clone(),
            Some(v) if let Some(u) = string_units(v) => {
                u.into_iter().map(|c| utf16_string(&[c])).collect()
            }
            _ => return Err("TypeError: plugin.tags is not iterable".into()),
        };
        let mut parts: Vec<String> = Vec::with_capacity(4 + tags.len());
        for key in ["name", "displayName", "description", "author"] {
            parts.push(join_part(obj.and_then(|o| o.get(key))));
        }
        for tag in &tags {
            parts.push(join_part(Some(tag)));
        }
        let text = parts.join(" ").to_lowercase();
        if terms.iter().all(|term| text.contains(term)) {
            out.push(plugin);
        }
    }
    Ok(out)
}

fn slice_idx(val: f64, len: usize) -> usize {
    let len_f = len as f64;
    let int = if val.is_nan() {
        0.0
    } else if val.is_infinite() {
        val
    } else {
        val.trunc()
    };
    if int < 0.0 {
        (len_f + int).max(0.0) as usize
    } else {
        int.min(len_f) as usize
    }
}

/// Port of `pluginMarketPage` (`src/plugin-market-model.ts:56`).
pub fn plugin_market_page(
    plugins: Value,
    page: f64,
    page_size: Option<f64>,
) -> Result<Value, String> {
    if is_nil(&plugins) {
        return Err(
            "TypeError: Cannot read properties of null/undefined (reading 'length')".into(),
        );
    }
    let arr = plugins.as_array();
    let units = string_units(&plugins);
    let len = match (arr, &units) {
        (Some(a), _) => a.len(),
        (_, Some(u)) => u.len(),
        _ => return Err("TypeError: plugins.slice is not a function".into()),
    };
    let page_size = page_size.unwrap_or(50.0);
    let total_pages = js_max(1.0, (len as f64 / page_size).ceil());
    let floored = if page.is_finite() { page.floor() } else { 0.0 };
    let current_page = js_max(0.0, js_min(floored, total_pages - 1.0));
    let start = slice_idx(current_page * page_size, len);
    let end = slice_idx((current_page + 1.0) * page_size, len);
    let visible = match (arr, units) {
        (Some(a), _) => Value::Array(if end > start {
            a[start..end].to_vec()
        } else {
            Vec::new()
        }),
        (_, Some(u)) => {
            if end > start {
                utf16_string(&u[start..end])
            } else {
                Value::String(String::new())
            }
        }
        _ => Value::Null,
    };
    Ok(json!({
        "totalPages": number(total_pages),
        "currentPage": number(current_page),
        "visible": visible,
    }))
}

/// Port of `applyPluginInstallation` (`src/plugin-market-model.ts:66`).
pub fn apply_plugin_installation(
    catalogs: Value,
    source: String,
    installed: Value,
) -> Result<Value, String> {
    if is_nil(&catalogs) {
        return Err("TypeError: Cannot read properties of null/undefined".into());
    }
    let Some(catalog) = catalogs
        .as_object()
        .filter(|_| is_plain_object(&catalogs))
        .and_then(|o| o.get(&source))
        .filter(|v| is_truthy(v))
    else {
        return Ok(catalogs);
    };
    let Some(plugins) = catalog
        .as_object()
        .filter(|_| is_plain_object(catalog))
        .and_then(|o| o.get("plugins"))
        .and_then(Value::as_array)
    else {
        return Err("TypeError: Cannot read properties of null/undefined (reading 'some')".into());
    };
    let undef = undefined();
    let mut matched = false;
    for plugin in plugins {
        if is_nil(plugin) {
            return Err(
                "TypeError: Cannot read properties of null/undefined (reading 'id')".into(),
            );
        }
        if is_nil(&installed) {
            return Err(
                "TypeError: Cannot read properties of null/undefined (reading 'id')".into(),
            );
        }
        let pid = plugin.get("id").unwrap_or(&undef);
        let iid = installed.get("id").unwrap_or(&undef);
        if js_strict_eq(pid, iid) {
            matched = true;
            break;
        }
    }
    if !matched {
        return Ok(catalogs);
    }
    let installed_obj = installed
        .as_object()
        .filter(|_| is_plain_object(&installed));
    let iid = installed_obj.and_then(|o| o.get("id")).unwrap_or(&undef);
    let mut next_plugins = Vec::with_capacity(plugins.len());
    for plugin in plugins {
        if is_nil(plugin) {
            return Err(
                "TypeError: Cannot read properties of null/undefined (reading 'id')".into(),
            );
        }
        let pid = plugin.get("id").unwrap_or(&undef);
        if js_strict_eq(pid, iid) {
            let mut next = spread_js(plugin);
            for key in ["installed", "installedVersion", "updateAvailable"] {
                next.insert(
                    key.into(),
                    installed_obj
                        .and_then(|o| o.get(key))
                        .cloned()
                        .unwrap_or_else(undefined),
                );
            }
            next_plugins.push(Value::Object(next));
        } else {
            next_plugins.push(plugin.clone());
        }
    }
    let mut next_catalog = spread_js(catalog);
    next_catalog.insert("plugins".into(), Value::Array(next_plugins));
    let mut next_catalogs = spread_js(&catalogs);
    next_catalogs.insert(source, Value::Object(next_catalog));
    Ok(Value::Object(next_catalogs))
}

/// Port of class `PluginMarketRequests` (`src/plugin-market-model.ts:85`): holds the instance state.
pub struct PluginMarketRequests {
    active: bool,
    catalog_sequence: u64,
    install_sequence: u64,
    catalog_pending: bool,
    install_pending: bool,
}

impl PluginMarketRequests {
    /// Port of `PluginMarketRequests` constructor (`src/plugin-market-model.ts:85`).
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            active: false,
            catalog_sequence: 0,
            install_sequence: 0,
            catalog_pending: false,
            install_pending: false,
        })
    }

    pub fn activate(&mut self) -> Result<Value, String> {
        self.active = true;
        Ok(undefined())
    }

    pub fn dispose(&mut self) -> Result<Value, String> {
        self.active = false;
        self.catalog_sequence += 1;
        self.install_sequence += 1;
        self.catalog_pending = false;
        self.install_pending = false;
        Ok(undefined())
    }

    pub fn begin_catalog(&mut self) -> Result<Value, String> {
        if !self.active || self.install_pending {
            return Ok(Value::Null);
        }
        self.catalog_pending = true;
        self.catalog_sequence += 1;
        Ok(json!(self.catalog_sequence as f64))
    }

    pub fn is_current_catalog(&mut self, sequence: f64) -> Result<bool, String> {
        Ok(self.active && self.catalog_pending && sequence == (self.catalog_sequence as f64))
    }

    pub fn invalidate_catalog(&mut self) -> Result<Value, String> {
        self.catalog_sequence += 1;
        self.catalog_pending = false;
        Ok(undefined())
    }

    pub fn finish_catalog(&mut self, sequence: f64) -> Result<bool, String> {
        if !self.is_current_catalog(sequence)? {
            return Ok(false);
        }
        self.catalog_pending = false;
        Ok(true)
    }

    pub fn begin_install(&mut self) -> Result<Value, String> {
        if !self.active || self.catalog_pending || self.install_pending {
            return Ok(Value::Null);
        }
        self.install_pending = true;
        self.install_sequence += 1;
        Ok(json!(self.install_sequence as f64))
    }

    pub fn is_current_install(&mut self, sequence: f64) -> Result<bool, String> {
        Ok(self.active && self.install_pending && sequence == (self.install_sequence as f64))
    }

    pub fn finish_install(&mut self, sequence: f64) -> Result<bool, String> {
        if !self.is_current_install(sequence)? {
            return Ok(false);
        }
        self.install_pending = false;
        Ok(true)
    }

    pub fn can_select_source(&mut self) -> Result<bool, String> {
        Ok(self.active && !self.install_pending)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mk_plugin(id: &str) -> Value {
        json!({
            "id": id, "name": id, "displayName": id, "description": "", "version": "1.0",
            "author": "", "tags": [], "license": "MIT", "path": format!("plugins/{id}"),
            "tree": "tree", "bytes": 100, "skills": 1, "requiresAuth": false,
            "installed": false, "installedVersion": "", "updateAvailable": false,
        })
    }

    #[test]
    fn filtering_paging_and_requests_lifecycle() {
        let p = json!([mk_plugin("a"), mk_plugin("b"), mk_plugin("c")]);
        assert_eq!(
            filter_market_plugins(p.as_array().unwrap().clone(), " B ".into(), "all".into())
                .unwrap()
                .len(),
            1
        );
        let page = plugin_market_page(p, 5.0, Some(2.0)).unwrap();
        assert_eq!(page["totalPages"], 2);
        assert_eq!(page["currentPage"], 1);
        assert_eq!(page["visible"].as_array().unwrap().len(), 1);

        let mut req = PluginMarketRequests::new().unwrap();
        assert_eq!(req.begin_catalog().unwrap(), Value::Null);
        req.activate().unwrap();
        let c1 = req.begin_catalog().unwrap().as_f64().unwrap();
        assert_eq!(req.begin_install().unwrap(), Value::Null);
        req.invalidate_catalog().unwrap();
        assert!(!req.finish_catalog(c1).unwrap());
        let i1 = req.begin_install().unwrap().as_f64().unwrap();
        assert!(!req.can_select_source().unwrap());
        assert!(req.finish_install(i1).unwrap());
        assert!(req.can_select_source().unwrap());
        assert!(apply_plugin_installation(Value::Null, "public".into(), mk_plugin("x")).is_err());
    }
}
