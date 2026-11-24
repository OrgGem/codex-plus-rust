//! JS bindings for `src/onboarding-setup-model.ts`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;
pub mod utf16;

/// `hasCodexEnhancements`.
fn has_codex_enhancements_inner(config: String) -> Result<bool, String> {
    let config = crate::js::from_json(&config)?;
    let out = port::has_codex_enhancements(config)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "hasCodexEnhancements")]
pub fn has_codex_enhancements(config: String) -> Result<bool, JsError> {
    has_codex_enhancements_inner(config).map_err(|e| JsError::new(&e))
}

/// `serializeSetupState`.
fn serialize_setup_state_inner(state: String) -> Result<String, String> {
    let out = port::serialize_setup_state(state)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "serializeSetupState")]
pub fn serialize_setup_state(state: String) -> Result<String, JsError> {
    serialize_setup_state_inner(state).map_err(|e| JsError::new(&e))
}

/// `shouldShowSetup`.
fn should_show_setup_inner(
    stored_setup: Option<String>,
    stored_tour: Option<String>,
) -> Result<bool, String> {
    let out = port::should_show_setup(stored_setup, stored_tour)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "shouldShowSetup")]
pub fn should_show_setup(
    stored_setup: Option<String>,
    stored_tour: Option<String>,
) -> Result<bool, JsError> {
    should_show_setup_inner(stored_setup, stored_tour).map_err(|e| JsError::new(&e))
}

/// `setupGatewayState`.
fn setup_gateway_state_inner(
    focus: String,
    payload: String,
    selected_agent_id: String,
) -> Result<String, String> {
    let focus = crate::js::from_json(&focus)?;
    let payload = crate::js::from_json(&payload)?;
    let out = port::setup_gateway_state(focus, payload, selected_agent_id)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "setupGatewayState")]
pub fn setup_gateway_state(
    focus: String,
    payload: String,
    selected_agent_id: String,
) -> Result<String, JsError> {
    setup_gateway_state_inner(focus, payload, selected_agent_id).map_err(|e| JsError::new(&e))
}

/// `probeMatches`.
fn probe_matches_inner(state: String, probe: String) -> Result<String, String> {
    let state = crate::js::from_json(&state)?;
    let probe = crate::js::from_json(&probe)?;
    let out = port::probe_matches(state, probe)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "probeMatches")]
pub fn probe_matches(state: String, probe: String) -> Result<String, JsError> {
    probe_matches_inner(state, probe).map_err(|e| JsError::new(&e))
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

static S_HAS_CODEX_ENHANCEMENTS_0: wire::Schema = wire::Schema::Dyn;

/// `hasCodexEnhancements` over the binary wire.
#[wasm_bindgen(js_name = "hasCodexEnhancementsWire")]
pub fn has_codex_enhancements_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let config: Value = wire::read_value(r, &S_HAS_CODEX_ENHANCEMENTS_0)?;
        let out = &port::has_codex_enhancements(config)?;
        w.bool(*out);
        Ok(())
    })
}

static S_SETUP_GATEWAY_STATE_0: wire::Schema = wire::Schema::Dyn;
static S_SETUP_GATEWAY_STATE_1: wire::Schema = wire::Schema::Dyn;
static S_SETUP_GATEWAY_STATE_OUT: wire::Schema = wire::Schema::Obj(&[
    (
        "agent",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("agent", wire::Schema::Str16),
            ("name", wire::Schema::Str16),
            ("version", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("configPath", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("hint", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("enabled", wire::Schema::Bool),
            (
                "requiresDisconnectPreview",
                wire::Schema::OptJs(&wire::Schema::Bool),
            ),
            ("baseUrl", wire::Schema::Str16),
            ("defaultModel", wire::Schema::Str16),
            (
                "defaultSelectionStatus",
                wire::Schema::OptJs(&wire::Schema::Str16),
            ),
            (
                "defaultModelDiffers",
                wire::Schema::OptJs(&wire::Schema::Bool),
            ),
            ("models", wire::Schema::List(&wire::Schema::Str16)),
            (
                "drift",
                wire::Schema::OptJs(&wire::Schema::Obj(&[
                    ("by", wire::Schema::OptJs(&wire::Schema::Str16)),
                    ("endpoint", wire::Schema::Str16),
                ])),
            ),
            (
                "diagnostics",
                wire::Schema::OptJs(&wire::Schema::Obj(&[
                    ("state", wire::Schema::Str16),
                    ("endpoint", wire::Schema::OptJs(&wire::Schema::Str16)),
                    ("expectedEndpoint", wire::Schema::Str16),
                    ("configEvidence", wire::Schema::List(&wire::Schema::Dyn)),
                    ("requests", wire::Schema::Num),
                    ("errors", wire::Schema::Num),
                    ("inFlight", wire::Schema::Num),
                    ("lastSeenMs", wire::Schema::OptJs(&wire::Schema::Num)),
                    (
                        "recentError",
                        wire::Schema::OptJs(&wire::Schema::Obj(&[
                            ("kind", wire::Schema::Dyn),
                            ("status", wire::Schema::Dyn),
                            ("atMs", wire::Schema::Dyn),
                        ])),
                    ),
                ])),
            ),
            (
                "ownModels",
                wire::Schema::OptJs(&wire::Schema::Obj(&[
                    ("vendor", wire::Schema::Str16),
                    (
                        "models",
                        wire::Schema::List(&wire::Schema::Obj(&[
                            ("id", wire::Schema::Dyn),
                            ("label", wire::Schema::Dyn),
                        ])),
                    ),
                    ("current", wire::Schema::Str16),
                ])),
            ),
        ])),
    ),
    ("model", wire::Schema::Str16),
    ("providersConfigured", wire::Schema::Bool),
    ("bindingReady", wire::Schema::Bool),
    ("issue", wire::Schema::OptJs(&wire::Schema::Str16)),
    (
        "candidates",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("id", wire::Schema::Str16),
            ("providerId", wire::Schema::Str16),
            ("providerName", wire::Schema::Str16),
            ("model", wire::Schema::Str16),
        ])),
    ),
    ("fingerprint", wire::Schema::Str16),
    ("multipleTargets", wire::Schema::Bool),
]);

/// `setupGatewayState` over the binary wire.
#[wasm_bindgen(js_name = "setupGatewayStateWire")]
pub fn setup_gateway_state_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let focus: Value = wire::read_value(r, &S_SETUP_GATEWAY_STATE_0)?;
        let payload: Value = wire::read_value(r, &S_SETUP_GATEWAY_STATE_1)?;
        let selected_agent_id: String = r.string16()?;
        let out = &port::setup_gateway_state(focus, payload, selected_agent_id)?;
        wire::write_value(w, out, &S_SETUP_GATEWAY_STATE_OUT)?;
        Ok(())
    })
}

static S_PROBE_MATCHES_0: wire::Schema = wire::Schema::Dyn;
static S_PROBE_MATCHES_1: wire::Schema = wire::Schema::Dyn;
static S_PROBE_MATCHES_OUT: wire::Schema = wire::Schema::Bool;

/// `probeMatches` over the binary wire.
#[wasm_bindgen(js_name = "probeMatchesWire")]
pub fn probe_matches_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let state: Value = wire::read_value(r, &S_PROBE_MATCHES_0)?;
        let probe: Value = wire::read_value(r, &S_PROBE_MATCHES_1)?;
        let out = &port::probe_matches(state, probe)?;
        wire::write_value(w, out, &S_PROBE_MATCHES_OUT)?;
        Ok(())
    })
}
