//! Unified Rust + WASM core for `tools/conversation-canvas`.

#![forbid(unsafe_code)]

pub use canvas_common_rs::{
    build_graph, excerpt, parse_message, transcript_page, visible_tree_rows,
};

use canvas_common_rs::{
    as_number, eq_key, is_js_ws, is_nil, is_truthy, js_number, js_str, number_to_string,
    string_units, to_number, undefined, utf16_string,
};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use url::Url;
use wasm_bindgen::prelude::*;

pub const CANVAS_NODE_WIDTH: f64 = 268.0;
pub const CANVAS_NODE_HEIGHT: f64 = 132.0;
pub const CANVAS_GAP_X: f64 = 88.0;
pub const CANVAS_GAP_Y: f64 = 34.0;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskTreeView {
    pub roots: Vec<Value>,
    pub children: Vec<(String, Vec<Value>)>,
    pub pending: Vec<Value>,
    pub active_path: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanvasLayout {
    pub cards: Vec<Value>,
    pub by_id: Vec<(String, Value)>,
    pub edges: Vec<Value>,
    pub pending: Vec<Value>,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeBatchError {
    pub message: String,
    pub missing_part_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApiErrorInfo {
    pub message: String,
    pub retryable: bool,
}

pub fn task_tree_view(
    nodes: &[Value],
    current_node_id: Option<&Value>,
) -> Result<TaskTreeView, String> {
    let mut pending = Vec::new();
    let mut ready = Vec::new();
    for n in nodes {
        if is_nil(n) {
            return Err("TypeError: null node".into());
        }
        if n.get("summaryKind").and_then(Value::as_str) == Some("原文摘录") {
            pending.push(n.clone());
        } else {
            ready.push(n.clone());
        }
    }
    let u = undefined();
    let mut by_id: HashMap<String, Value> = HashMap::new();
    for n in &ready {
        by_id.insert(eq_key(n.get("id").unwrap_or(&u)), n.clone());
    }
    let mut children_map: HashMap<String, Vec<Value>> = HashMap::new();
    let mut child_order = Vec::new();
    let mut roots = Vec::new();
    for n in &ready {
        let p = n.get("parent").unwrap_or(&u);
        let pk = eq_key(p);
        if is_truthy(p) && by_id.contains_key(&pk) {
            if !children_map.contains_key(&pk) {
                child_order.push((js_str(p), pk.clone()));
            }
            children_map.entry(pk).or_default().push(n.clone());
        } else {
            roots.push(n.clone());
        }
    }
    let mut active_path = Vec::new();
    let mut active_seen = HashSet::new();
    if let Some(cur) = current_node_id {
        let mut ck = eq_key(cur);
        while let Some(node) = by_id.get(&ck) {
            if !active_seen.insert(ck.clone()) {
                break;
            }
            active_path.push(js_str(node.get("id").unwrap_or(&u)));
            ck = eq_key(node.get("parent").unwrap_or(&u));
        }
    }
    let children = child_order
        .into_iter()
        .filter_map(|(raw_k, key)| children_map.remove(&key).map(|v| (raw_k, v)))
        .collect();
    Ok(TaskTreeView {
        roots,
        children,
        pending,
        active_path,
    })
}

pub fn layout_task_canvas(
    nodes: Value,
    current_node_id: Value,
    collapsed: Value,
) -> Result<CanvasLayout, String> {
    let view = visible_tree_rows(nodes, current_node_id, collapsed)?;
    let rows = view["rows"].as_array().ok_or("invalid rows")?;
    let u = undefined();
    let mut children: HashMap<String, Vec<String>> = HashMap::new();
    for row in rows {
        let n = &row["node"];
        let pk = eq_key(n.get("parent").unwrap_or(&u));
        let id = eq_key(n.get("id").unwrap_or(&u));
        children.entry(pk).or_default().push(id);
    }
    let mut positions: HashMap<String, f64> = HashMap::new();
    let mut leaf = 0_usize;
    for row in rows {
        let id = eq_key(row["node"].get("id").unwrap_or(&u));
        if !children.contains_key(&id) {
            positions.insert(id, (leaf as f64) * (CANVAS_NODE_HEIGHT + CANVAS_GAP_Y));
            leaf += 1;
        }
    }
    for row in rows.iter().rev() {
        let id = eq_key(row["node"].get("id").unwrap_or(&u));
        if let Some(list) = children.get(&id)
            && !list.is_empty()
        {
            let first = positions.get(&list[0]).copied().unwrap_or(0.0);
            let last = positions.get(&list[list.len() - 1]).copied().unwrap_or(0.0);
            positions.insert(id, (first + last) / 2.0);
        }
    }
    let mut cards = Vec::with_capacity(rows.len());
    let mut by_id_map: HashMap<String, Value> = HashMap::new();
    let mut by_id = Vec::with_capacity(rows.len());
    let mut width = 0.0_f64;
    let mut height = 0.0_f64;
    for row in rows {
        let mut obj = row.as_object().cloned().unwrap_or_default();
        let depth = row["depth"].as_f64().unwrap_or(0.0);
        let x = depth * (CANVAS_NODE_WIDTH + CANVAS_GAP_X);
        let raw_id = row["node"].get("id").unwrap_or(&u);
        let id_k = eq_key(raw_id);
        let y = positions.get(&id_k).copied().unwrap_or(0.0);
        obj.insert("x".into(), js_number(x));
        obj.insert("y".into(), js_number(y));
        obj.insert("width".into(), js_number(CANVAS_NODE_WIDTH));
        obj.insert("height".into(), js_number(CANVAS_NODE_HEIGHT));
        width = width.max(x + CANVAS_NODE_WIDTH);
        height = height.max(y + CANVAS_NODE_HEIGHT);
        let card = Value::Object(obj);
        by_id_map.insert(id_k, card.clone());
        by_id.push((js_str(raw_id), card.clone()));
        cards.push(card);
    }
    let mut edges = Vec::new();
    for card in &cards {
        let pk = eq_key(card["node"].get("parent").unwrap_or(&u));
        if let Some(from) = by_id_map.get(&pk) {
            let active =
                card["active"].as_bool() == Some(true) && from["active"].as_bool() == Some(true);
            edges.push(json!({ "from": from, "to": card, "active": active }));
        }
    }
    let pending = view["pending"].as_array().cloned().unwrap_or_default();
    Ok(CanvasLayout {
        cards,
        by_id,
        edges,
        pending,
        width,
        height,
    })
}

pub fn message_digest(message: &Value) -> Result<String, String> {
    let role = message.get("role").and_then(Value::as_str).unwrap_or("");
    let phase = message.get("phase").and_then(Value::as_str).unwrap_or("");
    let text = message
        .get("text")
        .and_then(string_units)
        .map(|u| String::from_utf16_lossy(&u))
        .unwrap_or_else(|| js_str(message.get("text").unwrap_or(&Value::String(String::new()))));
    let raw = format!("{role}\n{phase}\n{text}");
    let sum = Sha256::digest(raw.as_bytes());
    Ok(sum.iter().map(|b| format!("{b:02x}")).collect())
}

pub fn prepare_history(messages: &[Value]) -> Result<Value, String> {
    let mut manifest = Map::new();
    let mut parts = Vec::new();
    for message in messages {
        let role = message.get("role").and_then(Value::as_str).unwrap_or("");
        let turn_status = message
            .get("turnStatus")
            .and_then(Value::as_str)
            .unwrap_or("");
        if role == "assistant" && turn_status == "inProgress" {
            continue;
        }
        let id = js_str(message.get("id").unwrap_or(&Value::Null));
        let digest = message_digest(message)?;
        manifest.insert(id.clone(), Value::String(digest.clone()));
        let text_val = message.get("text").ok_or("TypeError: missing text")?;
        let units =
            string_units(text_val).unwrap_or_else(|| js_str(text_val).encode_utf16().collect());
        let mut start = 0_usize;
        while start < units.len() {
            let mut end = (start + 6000).min(units.len());
            if end < units.len() && (0xD800..=0xDBFF).contains(&units[end - 1]) {
                end -= 1;
            }
            if end == start {
                end = (start + 1).min(units.len());
            }
            let part_id = format!("{id}@{}:{start}-{end}", &digest[..16]);
            parts.push(json!({
                "partId": part_id,
                "messageId": id,
                "digest": digest,
                "role": role,
                "turnId": message.get("turnId").filter(|v| is_truthy(v)).cloned().unwrap_or(Value::Null),
                "start": start,
                "end": end,
                "total": units.len(),
                "text": utf16_string(&units[start..end]),
            }));
            start = end;
        }
    }
    Ok(json!({ "manifest": Value::Object(manifest), "parts": parts }))
}

fn validate_org(value: &Value, allowed_sources: &[Value], request_id: &str) -> Result<(), String> {
    let ok_req = value.get("requestId").and_then(Value::as_str) == Some(request_id);
    let raw_nodes = value
        .get("nodes")
        .and_then(Value::as_array)
        .filter(|v| ok_req && !v.is_empty())
        .ok_or("整理结果格式不完整")?;
    let source_keys: HashSet<String> = allowed_sources.iter().map(eq_key).collect();
    let mut ids = Vec::new();
    let mut parents = Vec::new();
    let mut lanes = Vec::new();
    for n in raw_nodes {
        let id = n
            .get("id")
            .and_then(Value::as_str)
            .filter(|s| {
                (1..=80).contains(&s.len())
                    && s.bytes()
                        .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
                    && !ids.iter().any(|x| x == s)
            })
            .ok_or("整理结果节点 ID 无效")?;
        let lane = n.get("lane").and_then(Value::as_str).unwrap_or("");
        let sources = n.get("sources").and_then(Value::as_array);
        if !matches!(lane, "main" | "branch")
            || !sources.is_some_and(|s| {
                !s.is_empty() && s.iter().all(|sid| source_keys.contains(&eq_key(sid)))
            })
        {
            return Err("整理结果包含无效来源".into());
        }
        if ["title", "summary", "description", "status"]
            .into_iter()
            .any(|k| {
                !n.get(k).and_then(Value::as_str).is_some_and(|s| {
                    !s.trim_matches(is_js_ws).is_empty() && s.encode_utf16().count() <= 12_000
                })
            })
        {
            return Err("整理结果缺少节点描述".into());
        }
        let parent = n.get("parent");
        if !parent.is_some_and(|p| p.is_null() || p.is_string()) {
            return Err("整理结果父节点无效".into());
        }
        ids.push(id.to_string());
        lanes.push(lane.to_string());
        parents.push(parent.and_then(Value::as_str).map(str::to_string));
    }
    let mut root_iter = parents.iter().enumerate().filter(|(_, p)| p.is_none());
    let first_root = root_iter.next();
    if first_root.is_none_or(|(idx, _)| lanes[idx] != "main") || root_iter.next().is_some() {
        return Err("任务树必须有且只有一个主目标根节点".into());
    }
    for start in 0..ids.len() {
        let mut cur = Some(start);
        for _ in 0..ids.len() {
            let Some(i) = cur else { break };
            cur = parents[i]
                .as_deref()
                .map(|p| {
                    ids.iter()
                        .position(|x| x == p)
                        .ok_or("任务树包含不存在的父节点")
                })
                .transpose()?;
        }
        if cur.is_some() {
            return Err("任务树存在循环关联".into());
        }
    }
    if let Some(c) = value.get("currentNodeId").filter(|v| !is_nil(v))
        && !c.as_str().is_some_and(|id| ids.iter().any(|x| x == id))
    {
        return Err("当前推进节点不存在".into());
    }
    Ok(())
}

pub fn merge_batch(
    text: &str,
    state: &Value,
    batch: &[Value],
    request_id: &str,
    catalog: &[Value],
    prefix: &str,
    aliases: Option<&Value>,
) -> Result<Value, MergeBatchError> {
    let err = |msg: &str| MergeBatchError {
        message: msg.to_string(),
        missing_part_ids: None,
    };
    let fence_re = Regex::new(r"(?s)```(?:json)?\s*(.*?)```").expect("valid fence regex");
    let mut candidates: Vec<String> = fence_re
        .captures_iter(text)
        .map(|c| c[1].to_string())
        .collect();
    candidates.push(text.trim_matches(is_js_ws).to_string());
    let mut parsed_val = None;
    for raw in candidates {
        if let Ok(v) = serde_json::from_str::<Value>(&raw)
            && v.get("requestId").and_then(Value::as_str) == Some(request_id)
        {
            parsed_val = Some(v);
            break;
        }
    }
    let Some(mut val) = parsed_val else {
        return Err(err("本批整理格式无效"));
    };
    let Some(upserts) = val
        .get_mut("upserts")
        .and_then(Value::as_array_mut)
        .filter(|u| !u.is_empty() && u.len() <= 48)
    else {
        return Err(err("本批整理格式无效"));
    };
    if let Some(Value::Object(al)) = aliases {
        for n in upserts.iter_mut() {
            if let Some(arr) = n.get_mut("sources").and_then(Value::as_array_mut) {
                for sid in arr.iter_mut() {
                    if let Some(k) = sid.as_str()
                        && let Some(mapped) = al.get(k)
                    {
                        *sid = mapped.clone();
                    }
                }
            }
        }
    }
    let parts: HashMap<&str, &Value> = batch
        .iter()
        .filter_map(|p| Some((p.get("partId")?.as_str()?, p)))
        .collect();
    let allowed: HashSet<&str> = catalog
        .iter()
        .filter_map(|n| n.get("id")?.as_str())
        .collect();
    let state_nodes = state
        .get("nodes")
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .unwrap_or(&[]);
    let old: HashMap<&str, &Value> = state_nodes
        .iter()
        .filter_map(|n| Some((n.get("id")?.as_str()?, n)))
        .collect();
    let mut patch_map: HashMap<String, Value> = HashMap::new();
    let mut patch_order: Vec<String> = Vec::new();
    let mut covered: HashSet<String> = HashSet::new();
    let batch_count = state.get("batchCount").and_then(Value::as_u64).unwrap_or(0);

    for n in upserts.iter() {
        let Some(id) = n.get("id").and_then(Value::as_str) else {
            return Err(err("本批节点 ID 无效或覆盖了未提供的节点"));
        };
        if patch_map.contains_key(id)
            || (!allowed.contains(id) && (!id.starts_with(prefix) || old.contains_key(id)))
        {
            return Err(err("本批节点 ID 无效或覆盖了未提供的节点"));
        }
        let Some(src_arr) = n.get("sources").and_then(Value::as_array).filter(|s| {
            !s.is_empty()
                && s.iter()
                    .all(|x| x.as_str().is_some_and(|k| parts.contains_key(k)))
        }) else {
            return Err(err("本批引用了不存在的消息片段"));
        };
        for field in ["title", "summary", "description", "status"] {
            if !n
                .get(field)
                .and_then(Value::as_str)
                .is_some_and(|s| s.encode_utf16().count() <= 3000)
            {
                return Err(err("本批节点描述过长或无效"));
            }
        }
        let previous = old.get(id).copied();
        let mut evidence = previous
            .and_then(|p| p.get("evidence"))
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        let mut seen_src = HashSet::new();
        let mut msg_ids = Vec::new();
        for sid in src_arr {
            let k = sid.as_str().unwrap_or("");
            if seen_src.insert(k) {
                covered.insert(k.to_string());
                let part = parts[k];
                evidence.push(json!({ "messageId": part["messageId"], "start": part["start"], "end": part["end"], "digest": part["digest"] }));
            }
            msg_ids.push(parts[k]["messageId"].clone());
        }
        let mut combined_sources = Vec::new();
        let mut src_keys = HashSet::new();
        for s in previous
            .and_then(|p| p.get("sources"))
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .chain(msg_ids.iter())
        {
            if src_keys.insert(eq_key(s)) {
                combined_sources.push(s.clone());
            }
        }
        let mut revisions = previous
            .and_then(|p| p.get("revisions"))
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        if let Some(prev) = previous
            && ["summary", "description", "status"]
                .into_iter()
                .any(|k| prev.get(k) != n.get(k))
        {
            revisions.push(json!({ "batch": batch_count, "summary": prev["summary"], "description": prev["description"], "status": prev["status"] }));
        }
        let mut obj = n.as_object().cloned().unwrap_or_default();
        obj.insert("sources".into(), Value::Array(combined_sources));
        obj.insert("evidence".into(), Value::Array(evidence));
        obj.insert("revisions".into(), Value::Array(revisions));
        patch_order.push(id.to_string());
        patch_map.insert(id.to_string(), Value::Object(obj));
    }

    if covered.len() != batch.len() {
        let missing = batch
            .iter()
            .filter_map(|p| {
                let k = p.get("partId")?.as_str()?;
                (!covered.contains(k)).then(|| k.to_string())
            })
            .collect();
        return Err(MergeBatchError {
            message: "本批仍有未归入任务树的消息片段，进度未提交".into(),
            missing_part_ids: Some(missing),
        });
    }
    for n in patch_map.values() {
        if let Some(p) = n.get("parent")
            && !p.is_null()
        {
            let id = n["id"].as_str().unwrap_or("");
            let old_p = old
                .get(id)
                .and_then(|o| o.get("parent"))
                .and_then(Value::as_str);
            let ok = p.as_str().is_some_and(|ps| {
                allowed.contains(ps) || patch_map.contains_key(ps) || old_p == Some(ps)
            });
            if !ok {
                return Err(err("本批父节点未提供或不存在"));
            }
        }
    }
    let mut nodes: Vec<Value> = state_nodes
        .iter()
        .map(|n| {
            n.get("id")
                .and_then(Value::as_str)
                .and_then(|id| patch_map.get(id))
                .cloned()
                .unwrap_or_else(|| n.clone())
        })
        .collect();
    for id in patch_order {
        if !old.contains_key(id.as_str())
            && let Some(n) = patch_map.remove(&id)
        {
            nodes.push(n);
        }
    }
    let old_root = state_nodes
        .iter()
        .find(|n| n.get("parent") == Some(&Value::Null))
        .and_then(|n| n.get("id"))
        .and_then(Value::as_str);
    let new_root = nodes
        .iter()
        .find(|n| n.get("parent") == Some(&Value::Null))
        .and_then(|n| n.get("id"))
        .and_then(Value::as_str);
    if let Some(or) = old_root
        && new_root != Some(or)
    {
        return Err(err("本批改变了共同目标根节点"));
    }
    let cur = val
        .get("currentNodeId")
        .filter(|v| !v.is_null())
        .cloned()
        .or_else(|| state.get("currentNodeId").filter(|v| !v.is_null()).cloned())
        .unwrap_or(Value::Null);
    let all_src: Vec<Value> = nodes
        .iter()
        .filter_map(|n| n.get("sources").and_then(Value::as_array))
        .flatten()
        .cloned()
        .collect();
    validate_org(
        &json!({ "requestId": request_id, "nodes": nodes, "currentNodeId": cur }),
        &all_src,
        request_id,
    )
    .map_err(|m| err(&m))?;

    let mut done = state
        .get("done")
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();
    for part in batch {
        if let Some(pid) = part.get("partId").and_then(Value::as_str) {
            done.insert(pid.to_string(), Value::Bool(true));
        }
    }
    let mut out = state.as_object().cloned().unwrap_or_default();
    out.insert("schemaVersion".into(), json!(2));
    out.insert("nodes".into(), Value::Array(nodes));
    out.insert("currentNodeId".into(), cur);
    out.insert("batchCount".into(), json!(batch_count + 1));
    out.insert("done".into(), Value::Object(done));
    Ok(Value::Object(out))
}

pub fn api_config(input: &Value) -> Result<Value, String> {
    let channel = if input.get("channel").and_then(Value::as_str) == Some("external") {
        "external"
    } else {
        "native"
    };
    let base_url = input
        .get("baseUrl")
        .filter(|v| !is_nil(v))
        .map(js_str)
        .unwrap_or_default()
        .trim_matches(is_js_ws)
        .to_string();
    let model = input
        .get("model")
        .filter(|v| !is_nil(v))
        .map(js_str)
        .unwrap_or_default()
        .trim_matches(is_js_ws)
        .to_string();
    let key = input
        .get("key")
        .filter(|v| !is_nil(v))
        .map(js_str)
        .unwrap_or_default()
        .trim_matches(is_js_ws)
        .to_string();
    let remember = input.get("remember") == Some(&Value::Bool(true));
    let speed = if input.get("speed").and_then(Value::as_str) == Some("provider") {
        "provider"
    } else {
        "fast"
    };
    let revision = match input.get("revision").filter(|v| is_truthy(v)) {
        Some(v) => js_str(v),
        None => {
            let mut b = [0_u8; 16];
            getrandom::getrandom(&mut b).map_err(|e| format!("RNG error: {e}"))?;
            b[6] = (b[6] & 0x0f) | 0x40;
            b[8] = (b[8] & 0x3f) | 0x80;
            format!(
                "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
                b[0],
                b[1],
                b[2],
                b[3],
                b[4],
                b[5],
                b[6],
                b[7],
                b[8],
                b[9],
                b[10],
                b[11],
                b[12],
                b[13],
                b[14],
                b[15]
            )
        }
    };
    let mut out = json!({ "channel": channel, "baseUrl": base_url, "model": model, "key": key, "remember": remember, "speed": speed, "revision": revision });
    if channel == "external" {
        let ep = api_endpoint(&base_url)?;
        if model.is_empty() || model.encode_utf16().count() > 200 {
            return Err("请填写 API 的模型名称".into());
        }
        if key.is_empty() || key.contains('\r') || key.contains('\n') {
            return Err("请在设置中填写有效 API Key".into());
        }
        out["endpoint"] = Value::String(ep);
    }
    Ok(out)
}

pub fn api_endpoint(raw: &str) -> Result<String, String> {
    let mut u = Url::parse(raw.trim_matches(is_js_ws))
        .map_err(|_| "请输入完整的 HTTPS API 地址".to_string())?;
    let bad_q = u.query().is_some_and(|q| !q.is_empty());
    let bad_f = u.fragment().is_some_and(|f| !f.is_empty());
    if u.scheme() != "https" || !u.username().is_empty() || u.password().is_some() || bad_q || bad_f
    {
        return Err("API 地址须使用 HTTPS，且不含账号、密码、查询参数或片段".into());
    }
    let p = u.path().trim_end_matches('/').to_string();
    let target = if p.ends_with("/chat/completions") {
        p
    } else {
        format!("{}/chat/completions", if p.is_empty() { "/v1" } else { &p })
    };
    u.set_path(&target);
    Ok(u.to_string())
}

pub fn classify_api_error(status: Option<u16>) -> ApiErrorInfo {
    let code = status.unwrap_or(0);
    let hint = match code {
        401 => Some("密钥无效或已过期"),
        403 => Some("接口拒绝访问，请检查权限"),
        404 => Some("地址或模型不存在"),
        408 => Some("接口请求超时"),
        413 => Some("本批资料超过接口大小限制"),
        429 => Some("接口限流或额度不足"),
        _ => None,
    };
    let message = match hint {
        Some(h) => format!("API {code}：{h}"),
        None if code >= 400 => format!("API 请求失败（HTTP {code}），请检查服务状态"),
        None => "API 连接失败，请检查地址、网络及服务状态".into(),
    };
    ApiErrorInfo {
        message,
        retryable: code == 0 || code == 408 || code == 429 || code >= 500,
    }
}

fn decode_utf8_stream(pending: &mut Vec<u8>, chunk: &[u8], eof: bool) -> String {
    let mut combined: Vec<u8>;
    let mut input: &[u8] = if pending.is_empty() {
        chunk
    } else {
        combined = std::mem::take(pending);
        combined.extend_from_slice(chunk);
        &combined
    };
    let mut out = String::with_capacity(input.len());
    loop {
        match std::str::from_utf8(input) {
            Ok(valid) => {
                out.push_str(valid);
                break;
            }
            Err(err) => {
                let valid_up_to = err.valid_up_to();
                if valid_up_to > 0
                    && let Ok(valid) = std::str::from_utf8(&input[..valid_up_to])
                {
                    out.push_str(valid);
                }
                match err.error_len() {
                    Some(invalid_len) => {
                        out.push('\u{FFFD}');
                        input = &input[valid_up_to + invalid_len..];
                    }
                    None => {
                        if eof {
                            if valid_up_to < input.len() {
                                out.push('\u{FFFD}');
                            }
                        } else {
                            pending.extend_from_slice(&input[valid_up_to..]);
                        }
                        break;
                    }
                }
            }
        }
    }
    out
}

/// Pure-Rust SSE incremental state machine matching `readApiResponse` (`external-api.mjs:44-74`).
#[derive(Debug, Default)]
pub struct SseStreamDecoder {
    sse: bool,
    pending_utf8: Vec<u8>,
    buffer: String,
    raw: String,
    bytes: usize,
    content: String,
    finish_reason: Option<String>,
    done: bool,
}

impl SseStreamDecoder {
    pub fn new(content_type: &str) -> Self {
        Self {
            sse: content_type
                .to_ascii_lowercase()
                .contains("text/event-stream"),
            ..Self::default()
        }
    }

    pub fn push_chunk(&mut self, chunk: &[u8]) -> Result<(usize, usize), String> {
        self.bytes += chunk.len();
        if self.bytes > 4 * 1024 * 1024 {
            return Err("API 响应超过大小限制".into());
        }
        let text = decode_utf8_stream(&mut self.pending_utf8, chunk, false);
        if !self.sse {
            self.raw.push_str(&text);
            let trimmed = self.raw.trim_start();
            if trimmed.starts_with("data:") || trimmed.starts_with(':') {
                self.sse = true;
                self.buffer = std::mem::take(&mut self.raw);
            }
        } else {
            self.buffer.push_str(&text);
        }
        if self.sse {
            while let Some(pos) = self.buffer.find('\n') {
                let line = self.buffer[..pos].trim_end_matches('\r').to_string();
                self.buffer = self.buffer[pos + 1..].to_string();
                self.consume_line(&line)?;
            }
        }
        Ok((self.bytes, self.content.encode_utf16().count()))
    }

    fn consume_line(&mut self, line: &str) -> Result<(), String> {
        let Some(rest) = line.strip_prefix("data:") else {
            return Ok(());
        };
        let data = rest.trim();
        if data.is_empty() {
            return Ok(());
        }
        if data == "[DONE]" {
            self.done = true;
            return Ok(());
        }
        let value: Value = serde_json::from_str(data)
            .map_err(|_| "API 流式数据格式无效，本批未提交".to_string())?;
        if value.get("error").is_some_and(|e| !e.is_null()) {
            return Err("API 流式生成中断，本批未提交".into());
        }
        let choice = value.get("choices").and_then(|c| c.get(0));
        if let Some(delta) = choice.and_then(|c| c.get("delta")) {
            if let Some(c) = delta.get("content").and_then(Value::as_str) {
                self.content.push_str(c);
            }
            if delta.get("refusal").is_some_and(is_truthy) {
                return Err("API 未能生成本批整理结果".into());
            }
        }
        if let Some(fr) = choice
            .and_then(|c| c.get("finish_reason"))
            .and_then(Value::as_str)
        {
            self.finish_reason = Some(fr.to_string());
        }
        Ok(())
    }

    pub fn finish(mut self) -> Result<Value, String> {
        let tail_utf8 = decode_utf8_stream(&mut self.pending_utf8, &[], true);
        if !tail_utf8.is_empty() {
            if !self.sse {
                self.raw.push_str(&tail_utf8);
                let trimmed = self.raw.trim_start();
                if trimmed.starts_with("data:") || trimmed.starts_with(':') {
                    self.sse = true;
                    self.buffer = std::mem::take(&mut self.raw);
                }
            } else {
                self.buffer.push_str(&tail_utf8);
            }
        }
        if self.sse {
            while let Some(pos) = self.buffer.find('\n') {
                let line = self.buffer[..pos].trim_end_matches('\r').to_string();
                self.buffer = self.buffer[pos + 1..].to_string();
                self.consume_line(&line)?;
            }
            if !self.buffer.trim().is_empty() {
                let tail = self.buffer.trim_end_matches('\r').to_string();
                self.consume_line(&tail)?;
            }
            if !self.done && self.finish_reason.is_none() {
                return Err("API 流式连接提前结束，本批未提交".into());
            }
            Ok(
                json!({ "choices": [{ "finish_reason": self.finish_reason, "message": { "content": self.content } }] }),
            )
        } else {
            serde_json::from_str(&self.raw)
                .map_err(|_| "API 返回的不是 JSON，请检查 API 地址".into())
        }
    }
}

/// Pure-Rust history paginator matching `readNativeHistory` (`native-history.mjs:9-70`).
pub trait HistoryCache {
    fn get(&mut self, key: &str) -> Option<Value>;
    fn set(&mut self, key: &str, value: Value);
    fn delete(&mut self, key: &str);
}

#[derive(Default)]
pub struct InMemoryHistoryCache {
    entries: Vec<(String, Value)>,
}

impl HistoryCache for InMemoryHistoryCache {
    fn get(&mut self, key: &str) -> Option<Value> {
        self.entries
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.clone())
    }
    fn set(&mut self, key: &str, value: Value) {
        if let Some(pos) = self.entries.iter().position(|(k, _)| k == key) {
            self.entries[pos].1 = value;
        } else {
            self.entries.push((key.to_string(), value));
        }
    }
    fn delete(&mut self, key: &str) {
        if let Some(pos) = self.entries.iter().position(|(k, _)| k == key) {
            self.entries.remove(pos);
        }
    }
}

impl InMemoryHistoryCache {
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    pub fn trim_to_limit(&mut self, max_entries: usize) {
        while self.entries.len() > max_entries {
            self.entries.remove(0);
        }
    }
}

pub fn read_native_history_with<F>(
    mut send: F,
    thread_id: &str,
    cache: &mut InMemoryHistoryCache,
) -> Result<Vec<Value>, String>
where
    F: FnMut(&str, Value) -> Result<Value, String>,
{
    let mut turns = Vec::new();
    let (mut turn_ids, mut turn_cursors) = (HashSet::new(), HashSet::new());
    let mut cursor: Option<String> = None;
    loop {
        if !turn_cursors.insert(cursor.clone()) {
            return Err("历史轮次分页游标重复".into());
        }
        let page = send(
            "thread/turns/list",
            json!({ "threadId": thread_id, "cursor": cursor, "limit": 20, "itemsView": "notLoaded", "sortDirection": "asc" }),
        )?;
        let Some(data) = page.get("data").and_then(Value::as_array) else {
            return Err("历史轮次接口返回格式无效".into());
        };
        for turn in data {
            let Some(id) = turn.get("id").and_then(Value::as_str) else {
                return Err("历史轮次缺少 ID".into());
            };
            if turn_ids.insert(id.to_string()) {
                turns.push(turn.clone());
            }
        }
        cursor = page
            .get("nextCursor")
            .and_then(Value::as_str)
            .map(str::to_string);
        if cursor.is_none() {
            break;
        }
    }

    let (mut messages, mut seen) = (Vec::new(), HashSet::new());
    for turn in &turns {
        let turn_id = turn.get("id").and_then(Value::as_str).unwrap_or("");
        let turn_status = turn.get("status").cloned().unwrap_or(Value::Null);
        let key = format!("{thread_id}:{turn_id}");
        let parsed = if turn_status.as_str() == Some("completed")
            && let Some(Value::Array(cached)) = cache.get(&key)
        {
            cached
        } else {
            let partial = if turn_status.as_str() == Some("completed") {
                cache.get(&format!("{key}:partial"))
            } else {
                None
            };
            let mut acc = partial
                .as_ref()
                .and_then(|p| p.get("messages"))
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default();
            let mut item_cursor: Option<String> = partial
                .as_ref()
                .and_then(|p| p.get("cursor"))
                .and_then(Value::as_str)
                .map(str::to_string);
            let mut limit = 32_usize;
            let mut cursors: HashSet<Option<String>> = HashSet::new();
            loop {
                if !cursors.insert(item_cursor.clone()) {
                    return Err("历史消息分页游标重复".into());
                }
                let page = loop {
                    match send(
                        "thread/items/list",
                        json!({ "threadId": thread_id, "turnId": turn_id, "cursor": item_cursor, "limit": limit, "sortDirection": "asc" }),
                    ) {
                        Ok(p) => break p,
                        Err(e)
                            if limit > 1
                                && e.to_ascii_lowercase()
                                    .contains("decoded message length too large") =>
                        {
                            limit = (limit / 2).max(1);
                        }
                        Err(e) => return Err(e),
                    }
                };
                let Some(rows) = page.get("data").and_then(Value::as_array) else {
                    return Err("历史消息接口返回格式无效".into());
                };
                for row in rows {
                    if let Some(rtid) = row.get("turnId").filter(|v| !v.is_null())
                        && rtid.as_str() != Some(turn_id)
                    {
                        return Err("历史消息所属轮次不匹配".into());
                    }
                    let item = row.get("item").unwrap_or(row);
                    let itype = item.get("type").and_then(Value::as_str).unwrap_or("");
                    if !matches!(itype, "userMessage" | "agentMessage") {
                        continue;
                    }
                    let Some(iid) = item.get("id").and_then(Value::as_str) else {
                        return Err("历史消息缺少稳定 ID".into());
                    };
                    let role = if itype == "userMessage" {
                        "user"
                    } else {
                        "assistant"
                    };
                    let phase = if role == "assistant" {
                        item.get("phase")
                            .and_then(Value::as_str)
                            .unwrap_or("final_answer")
                    } else {
                        ""
                    };
                    let content = if role == "user" {
                        item.get("content").cloned().unwrap_or_else(|| json!([]))
                    } else {
                        json!([{ "text": item.get("text").and_then(Value::as_str).unwrap_or("") }])
                    };
                    let parsed_msg = parse_message(
                        json!({ "type": "response_item", "payload": { "type": "message", "id": iid, "role": role, "phase": phase, "content": content } }),
                        Value::String(iid.to_string()),
                    )?;
                    if let Value::Object(mut o) = parsed_msg {
                        o.insert("turnId".into(), Value::String(turn_id.to_string()));
                        o.insert("turnStatus".into(), turn_status.clone());
                        acc.push(Value::Object(o));
                    }
                }
                item_cursor = page
                    .get("nextCursor")
                    .and_then(Value::as_str)
                    .map(str::to_string);
                if turn_status.as_str() == Some("completed") && item_cursor.is_some() {
                    cache.set(
                        &format!("{key}:partial"),
                        json!({ "messages": acc, "cursor": item_cursor }),
                    );
                }
                if item_cursor.is_none() {
                    break;
                }
            }
            if turn_status.as_str() == Some("completed") {
                cache.set(&key, Value::Array(acc.clone()));
                cache.delete(&format!("{key}:partial"));
            }
            acc
        };
        for m in parsed {
            let mid = m
                .get("id")
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_string();
            if seen.insert(mid)
                && let Value::Object(mut o) = m
            {
                o.insert("turnId".into(), Value::String(turn_id.to_string()));
                o.insert("turnStatus".into(), turn_status.clone());
                messages.push(Value::Object(o));
            }
        }
    }
    cache.trim_to_limit(500);
    Ok(messages)
}

/// Port of `tools/conversation-canvas/build-userscript.mjs`.
pub fn bundle_userscript(canvas_root: &Path) -> Result<String, String> {
    let read_rel = |rel: &str| {
        fs::read_to_string(canvas_root.join(rel)).map_err(|e| format!("read {rel}: {e}"))
    };
    let import_re = Regex::new(r"(?m)^import .*;\r?\n").expect("valid import regex");
    let export_re = Regex::new(r"(?m)^export ").expect("valid export regex");
    let ws_line_re = regex::RegexBuilder::new(r"^[ \t]+$")
        .multi_line(true)
        .crlf(true)
        .build()
        .expect("valid ws regex");
    let portable = |rel: &str| -> Result<String, String> {
        let s = read_rel(rel)?;
        let s = import_re.replace_all(&s, "");
        Ok(export_re.replace_all(&s, "").into_owned())
    };
    let model_bundle = [
        portable("model.mjs")?,
        portable("tree-markup.mjs")?,
        portable("bridge-data.mjs")?,
        portable("organize.mjs")?,
        portable("long-organizer.mjs")?,
        portable("view-pages.mjs")?,
        read_rel("src/canvas-store.js")?,
        portable("native-runtime.mjs")?,
        portable("native-history.mjs")?,
        read_rel("src/native-sidechat.js")?,
    ]
    .join("\n");
    let api_bundle = [
        portable("external-api.mjs")?,
        read_rel("src/native-api.js")?,
    ]
    .join("\n");
    let inter_bundle = [
        portable("tree-canvas.mjs")?,
        read_rel("src/tree-canvas-view.js")?,
    ]
    .join("\n");
    let style = read_rel("src/tree-canvas.css")?;
    let mut output = read_rel("src/canvas-panel.js")?;
    output = output.replacen("/* canvas-model */", &model_bundle, 1);
    output = output.replacen("/* canvas-api */", &api_bundle, 1);
    output = output.replacen("/* canvas-interaction */", &inter_bundle, 1);
    output = output.replacen("/* canvas-style */", &style, 1);
    output = output.replacen("/* canvas-annotations */", "const annotationIndex={};", 1);
    Ok(ws_line_re.replace_all(&output, "").into_owned())
}

#[wasm_bindgen(js_name = "prepareHistoryJson")]
pub fn prepare_history_wasm(messages_json: &str) -> Result<String, JsError> {
    let msgs: Vec<Value> =
        serde_json::from_str(messages_json).map_err(|e| JsError::new(&e.to_string()))?;
    let out = prepare_history(&msgs).map_err(|e| JsError::new(&e))?;
    serde_json::to_string(&out).map_err(|e| JsError::new(&e.to_string()))
}

#[wasm_bindgen(js_name = "layoutTaskCanvasJson")]
pub fn layout_task_canvas_wasm(
    nodes_json: &str,
    current_node_id_json: &str,
    collapsed_json: &str,
) -> Result<String, JsError> {
    let nodes: Value =
        serde_json::from_str(nodes_json).map_err(|e| JsError::new(&e.to_string()))?;
    let cur: Value =
        serde_json::from_str(current_node_id_json).map_err(|e| JsError::new(&e.to_string()))?;
    let col: Value =
        serde_json::from_str(collapsed_json).map_err(|e| JsError::new(&e.to_string()))?;
    let out = layout_task_canvas(nodes, cur, col).map_err(|e| JsError::new(&e))?;
    serde_json::to_string(&out).map_err(|e| JsError::new(&e.to_string()))
}

pub fn clamp_as_number_display(x: f64) -> String {
    let _ = (to_number(&Value::Null), as_number(&Value::Null));
    number_to_string(x)
}
