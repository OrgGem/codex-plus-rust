//! The Rust port of `view-pages.mjs`.

#![forbid(unsafe_code)]

use serde_json::Value;

/// Port of `transcriptPage` (`view-pages.mjs:1`).
pub fn transcript_page(messages: Value, state: Value) -> Result<Value, String> {
    canvas_common_rs::transcript_page(messages, state)
}
