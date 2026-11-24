//! The Rust port of `ipc-versions.mjs` (`extractIpcVersions`).

#![forbid(unsafe_code)]

use crate::js;
use serde_json::Value;

const NEEDLE: &str = "\"thread-follower-start-turn\":";

/// Port of `extractIpcVersions` (`ipc-versions.mjs:33`).
pub fn extract_ipc_versions(source: Value) -> Result<Value, String> {
    let Some(s) = js::string_units(&source).map(|u| String::from_utf16_lossy(&u)) else {
        return Err("TypeError: source.match is not a function".into());
    };
    let bytes = s.as_bytes();
    let mut i = 0;
    while let Some(open) = bytes[i..].iter().position(|&b| b == b'{').map(|p| i + p) {
        let mut j = open + 1;
        while j < bytes.len() && bytes[j] != b'{' && bytes[j] != b'}' {
            j += 1;
        }
        if j < bytes.len() && bytes[j] == b'}' {
            let candidate = &s[open..=j];
            let has_key = candidate.find(NEEDLE).is_some_and(|p| {
                candidate
                    .as_bytes()
                    .get(p + NEEDLE.len())
                    .is_some_and(u8::is_ascii_digit)
            });
            if has_key {
                return serde_json::from_str::<Value>(candidate)
                    .map_err(|e| format!("SyntaxError: {e}"));
            }
        }
        i = j.max(open + 1);
    }
    Err("没找到 IPC 版本表（Codex 改了实现）".into())
}
