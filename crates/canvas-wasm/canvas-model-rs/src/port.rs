//! The Rust port of `model.mjs`.

#![forbid(unsafe_code)]

use serde_json::Value;

/// Port of `parseMessage` (`model.mjs:1`).
pub fn parse_message(record: Value, fallback_id: Value) -> Result<Value, String> {
    canvas_common_rs::parse_message(record, fallback_id)
}

/// Port of `excerpt` (`model.mjs:16`).
pub fn excerpt(text: Value, length: Option<f64>) -> Result<Value, String> {
    canvas_common_rs::excerpt(text, length)
}

/// Port of `buildGraph` (`model.mjs:20`).
pub fn build_graph(messages: Value, annotations: Value) -> Result<Value, String> {
    canvas_common_rs::build_graph(messages, annotations)
}

/// Port of `visibleTreeRows` (`model.mjs:51`).
pub fn visible_tree_rows(
    nodes: Value,
    current_node_id: Value,
    collapsed: Value,
) -> Result<Value, String> {
    canvas_common_rs::visible_tree_rows(nodes, current_node_id, collapsed)
}
