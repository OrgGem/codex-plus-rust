//! Markdown renderers ported from `services/share-site/app.js`.

use serde_json::Value;

fn normalize_title(raw: Option<&str>) -> &str {
    raw.map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or("未命名会话")
}

/// Port of `sessionToMarkdown` (`app.js:129-136`).
pub fn session_to_markdown(session: &Value) -> String {
    let title = normalize_title(session.get("title").and_then(Value::as_str));
    let session_id = session
        .get("session_id")
        .and_then(Value::as_str)
        .unwrap_or("");
    let msgs = session
        .get("messages")
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .unwrap_or(&[]);
    let rendered: Vec<String> = msgs
        .iter()
        .map(|m| {
            let role = match m.get("role").and_then(Value::as_str) {
                Some("user") => "用户",
                Some("assistant") => "助手",
                _ => "消息",
            };
            let text = m.get("text").and_then(Value::as_str).unwrap_or("").trim();
            format!("### {role}\n\n{text}")
        })
        .collect();
    format!(
        "# {title}\n\n- 会话 ID：`{session_id}`\n\n{}",
        rendered.join("\n\n")
    )
}

/// Port of `rolloutToMarkdown` (`app.js:138-153`).
pub fn rollout_to_markdown(content: &str, title: Option<&str>) -> String {
    let title = normalize_title(title);
    let mut rendered = Vec::new();
    for line in content
        .split('\n')
        .map(|l| l.strip_suffix('\r').unwrap_or(l))
    {
        let Ok(parsed) = serde_json::from_str::<Value>(line) else {
            continue;
        };
        let Some(payload) = parsed.get("payload") else {
            continue;
        };
        if payload.get("type").and_then(Value::as_str) != Some("message") {
            continue;
        }
        let Some(role_raw) = payload.get("role").and_then(Value::as_str) else {
            continue;
        };
        if !matches!(role_raw, "user" | "assistant") {
            continue;
        }
        let text = payload
            .get("content")
            .and_then(Value::as_array)
            .map(|parts| {
                parts
                    .iter()
                    .map(|p| p.get("text").and_then(Value::as_str).unwrap_or(""))
                    .collect::<Vec<_>>()
                    .join("\n")
                    .trim()
                    .to_string()
            })
            .unwrap_or_default();
        if !text.is_empty() {
            let role = if role_raw == "user" {
                "用户"
            } else {
                "助手"
            };
            rendered.push(format!("### {role}\n\n{text}"));
        }
    }
    format!("# {title}\n\n{}", rendered.join("\n\n"))
}
