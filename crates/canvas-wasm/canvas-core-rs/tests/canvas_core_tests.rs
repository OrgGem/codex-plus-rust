use canvas_core_rs::{
    InMemoryHistoryCache, SseStreamDecoder, api_config, bundle_userscript, classify_api_error,
    layout_task_canvas, merge_batch, message_digest, prepare_history, read_native_history_with,
    task_tree_view,
};
use serde_json::{Value, json};
use std::fs;
use std::path::Path;
use std::process::Command;

#[test]
fn bundle_userscript_matches_public_canvas_user_js() {
    let root = Path::new(r"d:\Git\codex-plus\tools\conversation-canvas");
    let bundled = bundle_userscript(root).expect("bundle userscript");
    let existing =
        fs::read_to_string(root.join("public/canvas.user.js")).expect("read public/canvas.user.js");
    // Normalize CRLF vs LF across git checkout on Windows and `join('\n')` in build-userscript.mjs.
    assert_eq!(
        bundled.replace("\r\n", "\n"),
        existing.replace("\r\n", "\n")
    );
}

#[test]
fn prepare_history_and_merge_batch_match_node() {
    let text = format!("{}{}", "正文🚀".repeat(2500), "结尾");
    let msg = json!({ "id": "m1", "role": "user", "text": text });
    let hist = prepare_history(std::slice::from_ref(&msg)).expect("prepare_history");
    let node_script = format!(
        r#"
        import {{ prepareHistory }} from './long-organizer.mjs';
        const msg = {msg};
        const h = await prepareHistory([msg]);
        console.log(JSON.stringify(h));
        "#,
        msg = serde_json::to_string(&msg).unwrap()
    );
    let out = Command::new("node")
        .current_dir(r"d:\Git\codex-plus\tools\conversation-canvas")
        .args(["--input-type=module", "-e", &node_script])
        .output()
        .expect("run node");
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let node_h: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(hist, node_h);
    assert_eq!(message_digest(&msg).unwrap(), hist["manifest"]["m1"]);

    let parts = hist["parts"].as_array().unwrap().clone();
    let state = json!({ "schemaVersion": 2, "nodes": [], "currentNodeId": null, "batchCount": 0, "done": {} });
    let bad_resp = json!({
        "requestId": "r1",
        "currentNodeId": "b1_0",
        "upserts": [{
            "id": "b1_0", "parent": null, "lane": "main",
            "title": "Root", "summary": "S", "description": "D", "status": "待验证",
            "sources": [parts[0]["partId"]]
        }]
    });
    let err = merge_batch(
        &bad_resp.to_string(),
        &state,
        &parts,
        "r1",
        &[],
        "b1_",
        None,
    )
    .unwrap_err();
    assert_eq!(err.missing_part_ids.unwrap().len(), parts.len() - 1);

    let ok_resp = json!({
        "requestId": "r1",
        "currentNodeId": "b1_0",
        "upserts": [{
            "id": "b1_0", "parent": null, "lane": "main",
            "title": "Root", "summary": "S", "description": "D", "status": "待验证",
            "sources": parts.iter().map(|p| p["partId"].clone()).collect::<Vec<_>>()
        }]
    });
    let merged =
        merge_batch(&ok_resp.to_string(), &state, &parts, "r1", &[], "b1_", None).expect("merge");
    assert_eq!(merged["batchCount"], 1);
    assert_eq!(merged["nodes"].as_array().unwrap().len(), 1);
}

#[test]
fn tree_layout_sse_and_history_paginator() {
    let nodes = vec![
        json!({ "id": "root", "parent": null, "title": "Root", "summary": "S", "status": "待验证", "sources": ["m1"], "summaryKind": "已整理" }),
        json!({ "id": "a", "parent": "root", "title": "A", "summary": "S", "status": "待验证", "sources": ["m1"], "summaryKind": "已整理" }),
        json!({ "id": "b", "parent": "root", "title": "B", "summary": "S", "status": "待验证", "sources": ["m1"], "summaryKind": "已整理" }),
    ];
    let view = task_tree_view(&nodes, Some(&json!("a"))).unwrap();
    assert_eq!(view.active_path, vec!["a", "root"]);
    let layout =
        layout_task_canvas(Value::Array(nodes), json!("a"), json!({ "$set": [] })).unwrap();
    assert_eq!(layout.cards.len(), 3);
    assert_eq!(layout.edges.len(), 2);

    let mut dec = SseStreamDecoder::new("text/event-stream");
    dec.push_chunk(b": keepalive\r\n\r\n").unwrap();
    dec.push_chunk(
        format!(
            "data: {}\n\n",
            json!({"choices":[{"delta":{"content":"树形🚀"}}]})
        )
        .as_bytes(),
    )
    .unwrap();
    dec.push_chunk(
        format!(
            "data: {}\n\ndata: [DONE]\n\n",
            json!({"choices":[{"delta":{},"finish_reason":"stop"}]})
        )
        .as_bytes(),
    )
    .unwrap();
    let sse_out = dec.finish().unwrap();
    assert_eq!(sse_out["choices"][0]["message"]["content"], "树形🚀");

    // Multi-byte UTF-8 split byte-by-byte across chunks must never emit U+FFFD
    let split_stream = format!(
        "data: {}\n\ndata: [DONE]\n\n",
        json!({"choices":[{"delta":{"content":"跨分片中文🚀😀无乱码"},"finish_reason":"stop"}]})
    );
    let mut byte_dec = SseStreamDecoder::new("text/event-stream");
    for b in split_stream.as_bytes() {
        byte_dec.push_chunk(std::slice::from_ref(b)).unwrap();
    }
    let byte_sse_out = byte_dec.finish().unwrap();
    assert_eq!(
        byte_sse_out["choices"][0]["message"]["content"],
        "跨分片中文🚀😀无乱码"
    );

    assert!(!classify_api_error(Some(401)).retryable);
    assert!(classify_api_error(Some(429)).retryable);
    assert!(api_config(&json!({ "channel": "external", "baseUrl": "https://api.deepseek.com", "model": "deepseek-v4-flash", "key": "k" })).is_ok());

    let mut cache = InMemoryHistoryCache::default();
    let mut item_calls = 0;
    let hist = read_native_history_with(
        |method, _params| {
            if method == "thread/turns/list" {
                Ok(json!({ "data": [{ "id": "t1", "status": "completed" }], "nextCursor": null }))
            } else {
                item_calls += 1;
                Ok(json!({
                    "data": [{ "turnId": "t1", "item": { "type": "userMessage", "id": "u1", "content": [{ "text": "最初目标" }] } }],
                    "nextCursor": null
                }))
            }
        },
        "thread-1",
        &mut cache,
    ).unwrap();
    assert_eq!(hist.len(), 1);
    assert_eq!(item_calls, 1);
    // Second call uses cache cleanly without fetching items again
    let hist2 = read_native_history_with(
        |method, _| {
            assert_eq!(method, "thread/turns/list");
            Ok(json!({ "data": [{ "id": "t1", "status": "completed" }], "nextCursor": null }))
        },
        "thread-1",
        &mut cache,
    )
    .unwrap();
    assert_eq!(hist2, hist);
}
