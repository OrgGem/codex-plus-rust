//! The Rust port of `src/extension-manager-bridge.ts`.

#![forbid(unsafe_code)]

use manager_common_rs::{as_number, is_plain_object, is_truthy, string_units};
use serde_json::Value;

pub const MANAGER_PANEL_MIN_HEIGHT: i64 = 80;
pub const MANAGER_PANEL_MAX_HEIGHT: i64 = 2000;

pub const MANAGER_PANEL_CSP: &str = concat!(
    "default-src 'none'; form-action 'none'; base-uri 'none'; script-src 'un",
    "safe-inline'; style-src 'un",
    "safe-inline'; img-src data: blob:; font-src data:; media-src data: blob:"
);

pub const MANAGER_PANEL_BOOTSTRAP: &str = r#"(() => {
  let seq = 0;
  const pending = new Map();
  const listeners = { theme: new Set(), config: new Set() };
  const request = (method, params) => new Promise((resolve, reject) => {
    const id = ++seq;
    pending.set(id, { resolve, reject });
    parent.postMessage({ __codexPlusManager: 1, id, method, params }, "*");
  });
  addEventListener("message", (event) => {
    const data = event.data;
    if (event.source !== parent || !data || data.__codexPlusManager !== 1) return;
    if (data.event) {
      if (data.event === "theme") document.documentElement.dataset.theme = data.value;
      (listeners[data.event] || new Set()).forEach((handler) => { try { handler(data.value); } catch (error) { console.error(error); } });
      return;
    }
    const entry = pending.get(data.id);
    if (!entry) return;
    pending.delete(data.id);
    if (data.ok) entry.resolve(data.value); else entry.reject(new Error(data.error || "调用失败"));
  });
  const on = (name) => (handler) => { listeners[name].add(handler); return () => listeners[name].delete(handler); };
  window.codexPlusManager = Object.freeze({
    config: Object.freeze({ get: () => request("config.get"), set: (values) => request("config.set", { values }), onChange: on("config") }),
    backend: Object.freeze({ call: (method, params) => request("backend.call", { method, params }) }),
    asset: (path) => request("asset", { path }),
    toast: (message) => request("toast", { message }),
    theme: () => request("theme"),
    onTheme: on("theme"),
    data: (name, params) => request("data", { name, params }),
    navigate: (target) => request("navigate", { target }),
    call: (route, payload) => request("call", { route, payload }),
  });
  const report = () => request("resize", { height: document.documentElement.scrollHeight }).catch(() => {});
  addEventListener("load", report);
  if (typeof ResizeObserver === "function") {
    const observer = new ResizeObserver(report);
    const watch = () => document.body ? observer.observe(document.body) : setTimeout(watch, 0);
    watch();
  }
})();"#;

/// Port of `isManagerRequest` (`src/extension-manager-bridge.ts:36`).
pub fn is_manager_request(data: Value) -> Result<bool, String> {
    if !is_truthy(&data) {
        return Ok(false);
    }
    let Some(obj) = data.as_object().filter(|_| is_plain_object(&data)) else {
        return Ok(false);
    };
    let valid_tag = obj.get("__codexPlusManager").and_then(as_number) == Some(1.0);
    let valid_id = obj.get("id").and_then(as_number).is_some();
    let valid_method = obj.get("method").and_then(string_units).is_some();
    Ok(valid_tag && valid_id && valid_method)
}

/// Port of `managerPanelSrcdoc` (`src/extension-manager-bridge.ts:164`).
pub fn manager_panel_srcdoc(html: String, theme: String) -> Result<String, String> {
    let theme_json = serde_json::to_string(&theme).map_err(|e| e.to_string())?;
    let head = format!(
        "<meta charset=\"utf-8\"><meta http-equiv=\"Content-Security-Policy\" content=\"{MANAGER_PANEL_CSP}\"><script>{MANAGER_PANEL_BOOTSTRAP}</script><script>document.documentElement.dataset.theme={theme_json}</script>"
    );
    Ok(format!(
        "<!doctype html><html><head>{head}{html}</head></html>"
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn request_validation_and_srcdoc_wrap() {
        assert!(
            is_manager_request(json!({ "__codexPlusManager": 1, "id": 7, "method": "config.get" }))
                .unwrap()
        );
        assert!(!is_manager_request(json!({ "id": 1, "method": "config.get" })).unwrap());
        assert!(!is_manager_request(json!(null)).unwrap());

        let page = manager_panel_srcdoc("<p>片段</p>".into(), "dark".into()).unwrap();
        assert!(page.starts_with("<!doctype html><html><head><meta charset=\"utf-8\"><meta http-equiv=\"Content-Security-Policy\""));
        assert!(page.contains("window.codexPlusManager"));
        assert!(page.contains("dataset.theme=\"dark\""));
        assert!(page.ends_with("<p>片段</p></head></html>"));
    }
}
