//! JS bindings for `src/gateway-agent-safety.ts`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;

/// `canReconnectAgent`.
fn can_reconnect_agent_inner(
    agent: String,
    relay_profiles_enabled: String,
) -> Result<bool, String> {
    let agent = crate::js::from_json(&agent)?;
    let relay_profiles_enabled = crate::js::from_json(&relay_profiles_enabled)?;
    let out = port::can_reconnect_agent(agent, relay_profiles_enabled)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "canReconnectAgent")]
pub fn can_reconnect_agent(agent: String, relay_profiles_enabled: String) -> Result<bool, JsError> {
    can_reconnect_agent_inner(agent, relay_profiles_enabled).map_err(|e| JsError::new(&e))
}

/// `agentDiagnosticPresentation`.
fn agent_diagnostic_presentation_inner(
    agent: String,
    helper_running: bool,
) -> Result<String, String> {
    let agent = crate::js::from_json(&agent)?;
    let out = port::agent_diagnostic_presentation(agent, helper_running)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "agentDiagnosticPresentation")]
pub fn agent_diagnostic_presentation(
    agent: String,
    helper_running: bool,
) -> Result<String, JsError> {
    agent_diagnostic_presentation_inner(agent, helper_running).map_err(|e| JsError::new(&e))
}

/// `disconnectPreviewReady`.
fn disconnect_preview_ready_inner(preview: String, agent: String) -> Result<bool, String> {
    let preview = crate::js::from_json(&preview)?;
    let out = port::disconnect_preview_ready(preview, agent)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "disconnectPreviewReady")]
pub fn disconnect_preview_ready(preview: String, agent: String) -> Result<bool, JsError> {
    disconnect_preview_ready_inner(preview, agent).map_err(|e| JsError::new(&e))
}

/// `fullSwitchReady`.
fn full_switch_ready_inner(preview: String, agent: String) -> Result<bool, String> {
    let preview = crate::js::from_json(&preview)?;
    let out = port::full_switch_ready(preview, agent)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "fullSwitchReady")]
pub fn full_switch_ready(preview: String, agent: String) -> Result<bool, JsError> {
    full_switch_ready_inner(preview, agent).map_err(|e| JsError::new(&e))
}

/// `disconnectFileLabel`.
fn disconnect_file_label_inner(kind: String) -> Result<String, String> {
    let kind = crate::js::from_json(&kind)?;
    let out = port::disconnect_file_label(kind)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "disconnectFileLabel")]
pub fn disconnect_file_label(kind: String) -> Result<String, JsError> {
    disconnect_file_label_inner(kind).map_err(|e| JsError::new(&e))
}

/// `diagnosticEvidenceLabel`.
fn diagnostic_evidence_label_inner(code: String) -> Result<String, String> {
    let out = port::diagnostic_evidence_label(code)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "diagnosticEvidenceLabel")]
pub fn diagnostic_evidence_label(code: String) -> Result<String, JsError> {
    diagnostic_evidence_label_inner(code).map_err(|e| JsError::new(&e))
}

/// `diagnosticErrorLabel`.
fn diagnostic_error_label_inner(kind: String) -> Result<String, String> {
    let out = port::diagnostic_error_label(kind)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "diagnosticErrorLabel")]
pub fn diagnostic_error_label(kind: String) -> Result<String, JsError> {
    diagnostic_error_label_inner(kind).map_err(|e| JsError::new(&e))
}

// ---- the binary wire ------------------------------------------------------------------------

thread_local! {
    static WIRE_IN: std::cell::RefCell<Vec<u8>> = const { std::cell::RefCell::new(Vec::new()) };
    static WIRE_OUT: std::cell::RefCell<Vec<u8>> = const { std::cell::RefCell::new(Vec::new()) };
}

/// This module's memory, where `index.cjs` reads and writes the wire buffers.
#[wasm_bindgen(js_name = "rustAnythingWireMemory")]
pub fn rust_anything_wire_memory() -> JsValue {
    wasm_bindgen::memory()
}

/// The address of an argument buffer of `len` bytes.
#[wasm_bindgen(js_name = "rustAnythingWireIn")]
pub fn rust_anything_wire_in(len: u32) -> u32 {
    WIRE_IN.with(|b| {
        let mut b = b.borrow_mut();
        b.resize(len as usize, 0);
        b.as_mut_ptr() as usize as u32
    })
}

/// The address of the last result.
#[wasm_bindgen(js_name = "rustAnythingWireOut")]
pub fn rust_anything_wire_out() -> u32 {
    WIRE_OUT.with(|b| b.borrow().as_ptr() as usize as u32)
}

/// Reads `len` argument bytes, runs `f`, and keeps its result for `rustAnythingWireOut`.
fn wire_io(
    len: u32,
    f: impl FnOnce(&mut wire::Reader, &mut wire::Writer) -> Result<(), String>,
) -> Result<u32, JsError> {
    let input = WIRE_IN.with(|b| std::mem::take(&mut *b.borrow_mut()));
    let mut w = wire::Writer::default();
    let result = f(
        &mut wire::Reader::new(&input[..(len as usize).min(input.len())]),
        &mut w,
    );
    WIRE_IN.with(|b| *b.borrow_mut() = input);
    match result {
        // A result of another shape than the TS type: JS runs the call through JSON.
        Err(e) if e.starts_with(wire::MISMATCH) => return Ok(u32::MAX),
        other => other.map_err(|e| JsError::new(&e))?,
    }
    let n = w.0.len();
    WIRE_OUT.with(|b| *b.borrow_mut() = w.0);
    Ok(u32::try_from(n).unwrap_or(u32::MAX))
}

static S_CAN_RECONNECT_AGENT_0: wire::Schema = wire::Schema::Dyn;
static S_CAN_RECONNECT_AGENT_1: wire::Schema = wire::Schema::Dyn;

/// `canReconnectAgent` over the binary wire.
#[wasm_bindgen(js_name = "canReconnectAgentWire")]
pub fn can_reconnect_agent_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let agent: Value = wire::read_value(r, &S_CAN_RECONNECT_AGENT_0)?;
        let relay_profiles_enabled: Value = wire::read_value(r, &S_CAN_RECONNECT_AGENT_1)?;
        let out = &port::can_reconnect_agent(agent, relay_profiles_enabled)?;
        w.bool(*out);
        Ok(())
    })
}

static S_AGENT_DIAGNOSTIC_PRESENTATION_0: wire::Schema = wire::Schema::Dyn;
static S_AGENT_DIAGNOSTIC_PRESENTATION_OUT: wire::Schema = wire::Schema::Obj(&[
    ("badge", wire::Schema::Str16),
    ("tone", wire::Schema::Str16),
    ("reconnect", wire::Schema::Bool),
]);

/// `agentDiagnosticPresentation` over the binary wire.
#[wasm_bindgen(js_name = "agentDiagnosticPresentationWire")]
pub fn agent_diagnostic_presentation_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let agent: Value = wire::read_value(r, &S_AGENT_DIAGNOSTIC_PRESENTATION_0)?;
        let helper_running: bool = r.bool()?;
        let out = &port::agent_diagnostic_presentation(agent, helper_running)?;
        wire::write_value(w, out, &S_AGENT_DIAGNOSTIC_PRESENTATION_OUT)?;
        Ok(())
    })
}

static S_DISCONNECT_PREVIEW_READY_0: wire::Schema = wire::Schema::Dyn;

/// `disconnectPreviewReady` over the binary wire.
#[wasm_bindgen(js_name = "disconnectPreviewReadyWire")]
pub fn disconnect_preview_ready_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let preview: Value = wire::read_value(r, &S_DISCONNECT_PREVIEW_READY_0)?;
        let agent: String = r.string16()?;
        let out = &port::disconnect_preview_ready(preview, agent)?;
        wire::write_dyn(w, &serde_json::to_value(out).map_err(|e| e.to_string())?);
        Ok(())
    })
}

static S_FULL_SWITCH_READY_0: wire::Schema = wire::Schema::Dyn;

/// `fullSwitchReady` over the binary wire.
#[wasm_bindgen(js_name = "fullSwitchReadyWire")]
pub fn full_switch_ready_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let preview: Value = wire::read_value(r, &S_FULL_SWITCH_READY_0)?;
        let agent: String = r.string16()?;
        let out = &port::full_switch_ready(preview, agent)?;
        wire::write_dyn(w, &serde_json::to_value(out).map_err(|e| e.to_string())?);
        Ok(())
    })
}

static S_DISCONNECT_FILE_LABEL_0: wire::Schema = wire::Schema::Dyn;
static S_DISCONNECT_FILE_LABEL_OUT: wire::Schema = wire::Schema::Str16;

/// `disconnectFileLabel` over the binary wire.
#[wasm_bindgen(js_name = "disconnectFileLabelWire")]
pub fn disconnect_file_label_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let kind: Value = wire::read_value(r, &S_DISCONNECT_FILE_LABEL_0)?;
        let out = &port::disconnect_file_label(kind)?;
        wire::write_value(w, out, &S_DISCONNECT_FILE_LABEL_OUT)?;
        Ok(())
    })
}
