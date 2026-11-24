//! The Rust port of `src/page-groups.ts`.

#![forbid(unsafe_code)]

use manager_common_rs::{is_nil, is_plain_object, is_truthy, string_units};
use serde_json::Value;

/// Port of `pageVisibleForTool` (`src/page-groups.ts:9`).
pub fn page_visible_for_tool(page: Option<Value>, tool: String) -> Result<bool, String> {
    let Some(page) = page.filter(|v| !is_nil(v)) else {
        return Ok(true);
    };
    let obj = page.as_object().filter(|_| is_plain_object(&page));
    if let Some(tools) = obj.and_then(|o| o.get("tools")).filter(|v| is_truthy(v)) {
        return match tools {
            Value::Array(items) => Ok(items
                .iter()
                .filter_map(string_units)
                .any(|u| String::from_utf16_lossy(&u) == tool)),
            _ if string_units(tools).is_some() => {
                let s = string_units(tools)
                    .map(|u| String::from_utf16_lossy(&u))
                    .unwrap_or_default();
                Ok(s.contains(&tool))
            }
            _ => Err("TypeError: page.tools.includes is not a function".into()),
        };
    }
    let Some(owner) = obj.and_then(|o| o.get("tool")).filter(|v| is_truthy(v)) else {
        return Ok(true);
    };
    Ok(string_units(owner).is_some_and(|u| String::from_utf16_lossy(&u) == tool))
}

/// Port of `agentTarget` (`src/page-groups.ts:29`).
pub fn agent_target(
    visible: Vec<String>,
    remembered: Option<String>,
    home: String,
) -> Result<String, String> {
    if let Some(target) = remembered.filter(|r| !r.is_empty() && visible.iter().any(|v| v == r)) {
        Ok(target)
    } else {
        Ok(home)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn resolves_page_visibility_and_agent_targets() {
        let shared = json!({ "tool": "codex", "tools": ["codex", "claude"] });
        assert!(page_visible_for_tool(Some(shared.clone()), "codex".into()).unwrap());
        assert!(page_visible_for_tool(Some(shared.clone()), "claude".into()).unwrap());
        assert!(!page_visible_for_tool(Some(shared), "grok".into()).unwrap());
        assert!(page_visible_for_tool(None, "codex".into()).unwrap());

        let visible = vec!["overview".into(), "relay".into()];
        assert_eq!(
            agent_target(visible.clone(), Some("relay".into()), "overview".into()).unwrap(),
            "relay"
        );
        assert_eq!(
            agent_target(visible.clone(), Some("enhance".into()), "overview".into()).unwrap(),
            "overview"
        );
        assert_eq!(
            agent_target(
                vec!["".into(), "overview".into()],
                Some(String::new()),
                "overview".into()
            )
            .unwrap(),
            "overview"
        );
    }
}
