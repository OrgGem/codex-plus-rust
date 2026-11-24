//! The Rust port of `src/enhancement-navigation.ts`.

#![forbid(unsafe_code)]

use manager_common_rs::js_rt::{is_nil, is_truthy};
use serde_json::{Value, json};

/// Port of `managerNavigationDestination` (`src/enhancement-navigation.ts:7`).
pub fn manager_navigation_destination(intent: Value) -> Result<Value, String> {
    if is_nil(&intent) {
        return Err("TypeError: Cannot read properties of null/undefined (reading 'page')".into());
    }
    let page = intent
        .get("page")
        .cloned()
        .unwrap_or_else(crate::js::undefined);
    let keep = matches!(page.as_str(), Some("pluginMarket" | "weixin"));
    let route = if !keep && intent.get("section").is_some_and(is_truthy) {
        json!("enhance")
    } else {
        page
    };
    Ok(json!({ "route": route }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn routes_match_ts_cases() {
        let g = |p, s| manager_navigation_destination(json!({"page": p, "section": s})).unwrap();
        assert_eq!(g("settings", "stepwise"), json!({"route": "enhance"}));
        assert_eq!(g("enhance", "dictation"), json!({"route": "enhance"}));
        assert_eq!(g("settings", ""), json!({"route": "settings"}));
        assert_eq!(g("pluginMarket", "x"), json!({"route": "pluginMarket"}));
        assert_eq!(g("weixin", "x"), json!({"route": "weixin"}));
        assert!(manager_navigation_destination(Value::Null).is_err());
    }
}
