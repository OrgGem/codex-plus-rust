//! Native browser service AST inspector CLI & identification policy evaluator.
//!
//! Replaces `assets/native-browser/inspect-service.cjs --stdin` and
//! `assets/native-browser/require-identification.mjs` without Node/Babel.

use anyhow::{Result, ensure};
use native_browser_inspector_rs::port::{inspect_entry, inspect_service};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::io::Read;
use std::path::Path;

pub const MAX_STDIN_BYTES: usize = 200 * 1024 * 1024;
pub const FAILURE_MESSAGE: &str = "Browser service does not match the supported semantic contract.";

const EDGE_EXT_ID: &str = "odlomjlbamekndcpllcnffbgeohgkmjh";
const CHROME_EXT_ID: &str = "hehggadaopoacecdllhhajmbjkdcmajg";

/// Evaluates `{"entry": ..., "service": ...}` JSON payload from `inspect-service.mjs --stdin`.
pub fn inspect_request_json(raw: &str) -> Result<Value> {
    let req: Value = serde_json::from_str(raw)?;
    let entry = req.get("entry").cloned().unwrap_or(Value::Null);
    let service = req.get("service").cloned().unwrap_or(Value::Null);
    inspect_entry(entry).map_err(anyhow::Error::msg)?;
    inspect_service(service).map_err(anyhow::Error::msg)
}

/// Reads bounded UTF-8 stdin (max 200 MiB) and runs the inspector contract.
pub fn run_stdin<R: Read>(reader: R) -> Result<String> {
    let mut limited = reader.take((MAX_STDIN_BYTES + 1) as u64);
    let mut buf = Vec::new();
    limited.read_to_end(&mut buf)?;
    ensure!(buf.len() <= MAX_STDIN_BYTES, "stdin exceeds 200 MiB cap");
    let raw = std::str::from_utf8(&buf)?;
    let result = inspect_request_json(raw)?;
    Ok(serde_json::to_string(&result)?)
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientMetadata {
    pub extension_id: Option<String>,
    pub extension_instance_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientInfo {
    #[serde(rename = "type")]
    pub kind: Option<String>,
    pub family: Option<String>,
    pub agent_request_header_enabled: Option<bool>,
    pub metadata: Option<ClientMetadata>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TurnMetadata {
    pub session_id: Option<String>,
    pub turn_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ControlFile {
    schema: Option<u64>,
    require_identification: Option<bool>,
}

fn supported_browser(family: Option<&str>, extension_id: Option<&str>) -> bool {
    matches!(
        (family, extension_id),
        (Some("edge"), Some(EDGE_EXT_ID)) | (Some("chrome"), Some(CHROME_EXT_ID))
    )
}

/// Port of `cppNativeIdentificationReader` (`assets/native-browser/require-identification.mjs`).
/// Returns `Ok(true)` when identification header enforcement is required, or `Ok(false)` to use fallback.
pub fn eval_require_identification(
    initial_info: Option<&ClientInfo>,
    initial_turn: Option<&TurnMetadata>,
    control_path: &Path,
    after_io: impl FnOnce() -> (Option<ClientInfo>, Option<TurnMetadata>),
) -> bool {
    let (Some(info), Some(turn)) = (initial_info, initial_turn) else {
        return false;
    };
    let meta = info.metadata.as_ref();
    let family = info.family.as_deref();
    let ext_id = meta.and_then(|m| m.extension_id.as_deref());
    let inst_id = meta.and_then(|m| m.extension_instance_id.as_deref());
    let session_id = turn.session_id.as_deref();
    let turn_id = turn.turn_id.as_deref();

    if info.kind.as_deref() != Some("extension")
        || !supported_browser(family, ext_id)
        || info.agent_request_header_enabled.is_none()
        || inst_id.is_none_or(str::is_empty)
        || session_id.is_none_or(str::is_empty)
        || turn_id.is_none_or(str::is_empty)
    {
        return false;
    }

    let Ok(stat) = fs::symlink_metadata(control_path) else {
        return false;
    };
    if !stat.is_file() || stat.file_type().is_symlink() || stat.len() > 1024 {
        return false;
    }
    let Ok(raw) = fs::read_to_string(control_path) else {
        return false;
    };
    let Ok(control) = serde_json::from_str::<ControlFile>(&raw) else {
        return false;
    };

    let (current_info, current_turn) = after_io();
    let Some(cur_info) = current_info.as_ref() else {
        return false;
    };
    let Some(cur_turn) = current_turn.as_ref() else {
        return false;
    };
    let cur_meta = cur_info.metadata.as_ref();
    cur_info == info
        && cur_info.kind.as_deref() == Some("extension")
        && cur_info.family.as_deref() == family
        && cur_meta.and_then(|m| m.extension_id.as_deref()) == ext_id
        && cur_meta.and_then(|m| m.extension_instance_id.as_deref()) == inst_id
        && cur_info.agent_request_header_enabled.is_some()
        && cur_turn.session_id.as_deref() == session_id
        && cur_turn.turn_id.as_deref() == turn_id
        && control.schema == Some(1)
        && control.require_identification == Some(true)
}
