//! The Rust port of `bridge-data.mjs`.

#![forbid(unsafe_code)]

use canvas_common_rs::{
    build_graph, eq_key, is_nil, is_truthy, is_undefined, js_str, js_strict_eq, parse_message,
    undefined,
};
use serde_json::{Map, Value, json};
use std::collections::HashSet;

/// Port of `graphFromExport` (`bridge-data.mjs:3`).
pub fn graph_from_export(
    result: Value,
    thread_id: Value,
    annotations: Value,
) -> Result<Value, String> {
    let ann = if is_undefined(&annotations) {
        json!({ "nodes": [] })
    } else {
        annotations
    };
    let st = result.get("status").and_then(Value::as_str);
    if is_nil(&result) || st != Some("ok") {
        let msg = result.get("message").filter(|v| is_truthy(v)).map(js_str);
        return Err(msg.unwrap_or_else(|| "Codex++ 会话读取失败".into()));
    }
    let Some(content) = result.get("content").and_then(Value::as_str) else {
        return Err("会话接口返回了不匹配的数据".into());
    };
    let kind_ok = result.get("kind").and_then(Value::as_str) == Some("codex-rollout");
    let sid_ok = js_strict_eq(result.get("session_id").unwrap_or(&undefined()), &thread_id);
    if !kind_ok || !sid_ok {
        return Err("会话接口返回了不匹配的数据".into());
    }
    let tid_str = js_str(&thread_id);
    let end = content.rfind('\n').map_or(0, |i| i + 1);
    let (mut messages, mut seen, mut declared_id) = (Vec::new(), HashSet::new(), Value::Null);
    for (i, line) in content[..end].split('\n').enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let Ok(r) = serde_json::from_str::<Value>(line) else {
            continue;
        };
        if r.get("type").and_then(Value::as_str) == Some("session_meta") {
            declared_id = r
                .get("payload")
                .and_then(|p| p.get("id"))
                .cloned()
                .unwrap_or_else(undefined);
        }
        let m = parse_message(r, json!(format!("message-{tid_str}-line-{i}")))?;
        if !m.is_null() {
            let k = eq_key(m.get("id").unwrap_or(&undefined()));
            if seen.insert(k) {
                messages.push(m);
            }
        }
    }
    if is_truthy(&declared_id) && !js_strict_eq(&declared_id, &thread_id) {
        return Err("记录中的任务 ID 与当前任务不一致".into());
    }
    if ann.is_null() {
        return Err("TypeError: Cannot read properties of null (reading 'title')".into());
    }
    let title = ann
        .get("title")
        .filter(|v| is_truthy(v))
        .cloned()
        .unwrap_or_else(|| json!("当前对话"));
    let graph = build_graph(Value::Array(messages.clone()), ann)?;
    let mut out = Map::new();
    out.insert("threadId".into(), thread_id);
    out.insert("title".into(), title);
    out.insert("messages".into(), Value::Array(messages));
    if let Value::Object(g) = graph {
        out.extend(g);
    }
    Ok(Value::Object(out))
}
