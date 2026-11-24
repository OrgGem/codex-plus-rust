//! Fake OpenAI Responses API HTTP/SSE server ported from `mock-responses.mjs`.

use regex::Regex;
use serde_json::{Value, json};
use std::collections::BTreeSet;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::time::Duration;

pub fn usage_json() -> Value {
    json!({
        "input_tokens": 100,
        "input_tokens_details": { "cached_tokens": 0 },
        "output_tokens": 10,
        "output_tokens_details": { "reasoning_tokens": 0 },
        "total_tokens": 110
    })
}

fn text_of(item: &Value) -> String {
    match item.get("content") {
        Some(Value::String(s)) => s.clone(),
        Some(Value::Array(arr)) => arr
            .iter()
            .map(|c| c.get("text").and_then(Value::as_str).unwrap_or(""))
            .collect(),
        _ => String::new(),
    }
}

pub fn sse_message(id: usize, text: &str, chunks: &[String]) -> Vec<Value> {
    let msg_id = format!("msg_{id}");
    let resp_id = format!("resp_{id}");
    let mut events = vec![
        json!({ "type": "response.created", "response": { "id": resp_id } }),
        json!({ "type": "response.output_item.added", "output_index": 0, "item": { "id": msg_id, "type": "message", "role": "assistant", "content": [] } }),
    ];
    for delta in chunks {
        events.push(json!({ "type": "response.output_text.delta", "item_id": msg_id, "output_index": 0, "content_index": 0, "delta": delta }));
    }
    events.push(json!({ "type": "response.output_item.done", "output_index": 0, "item": { "id": msg_id, "type": "message", "role": "assistant", "content": [{ "type": "output_text", "text": text }] } }));
    events.push(json!({ "type": "response.completed", "response": { "id": resp_id, "status": "completed", "usage": usage_json() } }));
    events
}

pub fn sse_tool_call(id: usize, name: &str, args: &Value) -> Vec<Value> {
    let item = json!({
        "id": format!("fc_{id}"),
        "type": "function_call",
        "call_id": format!("call_{id}"),
        "name": name,
        "arguments": serde_json::to_string(args).unwrap_or_default()
    });
    let mut item_empty = item.clone();
    item_empty["arguments"] = Value::String(String::new());
    vec![
        json!({ "type": "response.created", "response": { "id": format!("resp_{id}") } }),
        json!({ "type": "response.output_item.added", "output_index": 0, "item": item_empty }),
        json!({ "type": "response.output_item.done", "output_index": 0, "item": item }),
        json!({ "type": "response.completed", "response": { "id": format!("resp_{id}"), "status": "completed", "usage": usage_json() } }),
    ]
}

fn sandbox_of(text: &str) -> Option<&'static str> {
    let low = text.to_ascii_lowercase();
    if low.contains("danger-full-access") || low.contains("full access") {
        Some("full")
    } else if low.contains("workspace-write") {
        Some("workspace")
    } else if low.contains("read-only") || low.contains("read only") {
        Some("read-only")
    } else {
        None
    }
}

pub fn route_mock_request(
    method: &str,
    path: &str,
    body: &str,
    seq: &AtomicUsize,
    work_dir: &str,
    mock_log: Option<&Path>,
) -> (u16, &'static str, Vec<Value>, Option<String>, u64) {
    if method == "GET" && path.starts_with("/v1/models") {
        let json_body =
            json!({ "object": "list", "data": [{ "id": "mock-model", "object": "model" }] })
                .to_string();
        return (200, "application/json", Vec::new(), Some(json_body), 0);
    }
    if method != "POST" || !path.contains("/responses") {
        return (404, "text/plain", Vec::new(), Some(String::new()), 0);
    }
    let id = seq.fetch_add(1, Ordering::Relaxed) + 1;
    let json_val: Value = serde_json::from_str(body).unwrap_or_else(|_| json!({}));
    let input = json_val
        .get("input")
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .unwrap_or(&[]);
    let last = input.last();
    let last_user = input
        .iter()
        .rev()
        .find(|item| item.get("role").and_then(Value::as_str) == Some("user"));
    let user_text = last_user.map(text_of).unwrap_or_default();

    if let Some(log_path) = mock_log {
        let developers: Vec<String> = input
            .iter()
            .filter(|i| i.get("role").and_then(Value::as_str) == Some("developer"))
            .map(text_of)
            .collect();
        let dev_joined = developers.join("\n");
        let dev_low = dev_joined.to_ascii_lowercase();
        let mode_fallback = if dev_low.contains("collaboration mode: plan") {
            Some("plan".to_string())
        } else if dev_low.contains("collaboration mode: default") {
            Some("default".to_string())
        } else {
            None
        };
        let cm_re = Regex::new(r"Collaboration Mode: (\w+)").expect("valid");
        let plan_re = Regex::new(r"# Plan Mode\b").expect("valid");
        let modes: Vec<String> = developers
            .iter()
            .filter_map(|t| {
                if plan_re.is_match(t) {
                    Some("Plan".to_string())
                } else {
                    cm_re.captures(t).map(|c| c[1].to_string())
                }
            })
            .collect();
        let sandbox = developers.iter().filter_map(|t| sandbox_of(t)).next_back();
        let images = last_user
            .and_then(|u| u.get("content"))
            .and_then(Value::as_array)
            .map_or(0, |a| {
                a.iter()
                    .filter(|p| p.get("type").and_then(Value::as_str) == Some("input_image"))
                    .count()
            });
        let file_re = Regex::new(r#"remote-uploads/[^\s)\]"'<>\\]+"#).expect("valid");
        let mut files = BTreeSet::new();
        for m in file_re.find_iter(body) {
            files.insert(m.as_str().to_string());
        }
        let log_line = json!({
            "id": id,
            "model": json_val.get("model"),
            "effort": json_val.get("reasoning").and_then(|r| r.get("effort")).cloned().unwrap_or(Value::Null),
            "mode": modes.last().map(|s| s.to_ascii_lowercase()).or(mode_fallback),
            "modes": modes,
            "sandbox": sandbox,
            "user": user_text.chars().take(60).collect::<String>(),
            "images": images,
            "files": files.into_iter().collect::<Vec<_>>(),
        });
        if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(log_path) {
            let _ = writeln!(f, "{log_line}");
        }
    }

    let run_re = Regex::new(r"\bRUN\b").expect("valid");
    let ask_re = Regex::new(r"\bASK\b").expect("valid");
    let slow_re = Regex::new(r"\bSLOW\b").expect("valid");

    if last.and_then(|i| i.get("type")).and_then(Value::as_str) == Some("function_call_output") {
        let out = match last.and_then(|i| i.get("output")) {
            Some(Value::String(s)) => s.clone(),
            Some(other) => other.to_string(),
            None => String::new(),
        };
        let snippet: String = out.chars().take(160).collect();
        let msg_text = format!("工具结果：{snippet}");
        let events = sse_message(id, &msg_text, std::slice::from_ref(&msg_text));
        return (200, "text/event-stream", events, None, 0);
    }
    if run_re.is_match(&user_text) {
        let events = sse_tool_call(
            id,
            "exec_command",
            &json!({
                "cmd": format!("touch {work_dir}/approved.txt && echo created"),
                "workdir": work_dir,
                "sandbox_permissions": "require_escalated",
                "justification": "远程审批测试：在工作目录创建 approved.txt"
            }),
        );
        return (200, "text/event-stream", events, None, 0);
    }
    if ask_re.is_match(&user_text) {
        let events = sse_tool_call(
            id,
            "request_user_input",
            &json!({
                "questions": [{
                    "id": "color",
                    "header": "颜色",
                    "question": "你喜欢哪个颜色？",
                    "options": [
                        { "label": "红色", "description": "热情" },
                        { "label": "蓝色", "description": "冷静" }
                    ]
                }]
            }),
        );
        return (200, "text/event-stream", events, None, 0);
    }
    if slow_re.is_match(&user_text) {
        let chunks: Vec<String> = (1..=40).map(|i| format!("第{i}段 ")).collect();
        let full = chunks.join("");
        let events = sse_message(id, &full, &chunks);
        return (200, "text/event-stream", events, None, 500);
    }
    let snippet: String = user_text.chars().take(60).collect();
    let msg_text = format!("收到：{snippet}");
    let events = sse_message(id, &msg_text, std::slice::from_ref(&msg_text));
    (200, "text/event-stream", events, None, 0)
}

pub fn format_sse_bytes(events: &[Value]) -> Vec<u8> {
    let mut out = String::new();
    for event in events {
        let ty = event
            .get("type")
            .and_then(Value::as_str)
            .unwrap_or("message");
        out.push_str(&format!(
            "event: {ty}\ndata: {}\n\n",
            serde_json::to_string(event).unwrap_or_default()
        ));
    }
    out.into_bytes()
}

pub fn serve_mock_responses(
    port: u16,
    work_dir: String,
    mock_log: Option<std::path::PathBuf>,
    stop: Option<Arc<AtomicBool>>,
) -> anyhow::Result<()> {
    let listener = TcpListener::bind(("127.0.0.1", port))?;
    let seq = Arc::new(AtomicUsize::new(0));
    println!(
        "mock responses listening on {}",
        listener.local_addr()?.port()
    );
    for stream in listener.incoming() {
        if stop.as_ref().is_some_and(|s| s.load(Ordering::Relaxed)) {
            break;
        }
        let Ok(mut s) = stream else { break };
        let mut buf = Vec::new();
        let mut tmp = [0_u8; 4096];
        let mut hend = None;
        while let Ok(n) = s.read(&mut tmp) {
            if n == 0 {
                break;
            }
            buf.extend_from_slice(&tmp[..n]);
            if let Some(p) = buf.windows(4).position(|w| w == b"\r\n\r\n") {
                hend = Some(p + 4);
                break;
            }
        }
        let Some(hend) = hend else { continue };
        let head = String::from_utf8_lossy(&buf[..hend]).into_owned();
        let clen = head
            .lines()
            .find_map(|l| {
                l.split_once(':')
                    .filter(|(k, _)| k.eq_ignore_ascii_case("content-length"))
                    .and_then(|(_, v)| v.trim().parse::<usize>().ok())
            })
            .unwrap_or(0);
        while buf.len() < hend + clen {
            let Ok(n) = s.read(&mut tmp) else { break };
            if n == 0 {
                break;
            }
            buf.extend_from_slice(&tmp[..n]);
        }
        let body = String::from_utf8_lossy(&buf[hend..buf.len().min(hend + clen)]).into_owned();
        let mut parts = head.lines().next().unwrap_or("").split_whitespace();
        let method = parts.next().unwrap_or("");
        let path = parts.next().unwrap_or("");
        if path == "/__stop" {
            let _ =
                s.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\n{}");
            break;
        }
        let (status, ctype, events, raw_body, gap_ms) =
            route_mock_request(method, path, &body, &seq, &work_dir, mock_log.as_deref());
        if let Some(raw) = raw_body {
            let resp = format!(
                "HTTP/1.1 {status} OK\r\nContent-Type: {ctype}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{raw}",
                raw.len()
            );
            let _ = s.write_all(resp.as_bytes());
        } else {
            let _ = s.write_all(b"HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nCache-Control: no-cache\r\nConnection: close\r\n\r\n");
            for ev in events {
                let chunk = format_sse_bytes(&[ev]);
                let _ = s.write_all(&chunk);
                let _ = s.flush();
                if gap_ms > 0 {
                    std::thread::sleep(Duration::from_millis(gap_ms));
                }
            }
        }
    }
    Ok(())
}
