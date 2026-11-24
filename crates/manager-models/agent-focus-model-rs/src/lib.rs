//! JS bindings for `src/agent-focus-model.ts`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;

/// `parseAgentFocus`.
fn parse_agent_focus_inner(stored: Option<String>) -> Result<String, String> {
    let out = port::parse_agent_focus(stored)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "parseAgentFocus")]
pub fn parse_agent_focus(stored: Option<String>) -> Result<String, JsError> {
    parse_agent_focus_inner(stored).map_err(|e| JsError::new(&e))
}

/// `serializeAgentFocus`.
fn serialize_agent_focus_inner(mode: String) -> Result<String, String> {
    let mode = crate::js::from_json(&mode)?;
    let out = port::serialize_agent_focus(mode)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "serializeAgentFocus")]
pub fn serialize_agent_focus(mode: String) -> Result<String, JsError> {
    serialize_agent_focus_inner(mode).map_err(|e| JsError::new(&e))
}

/// `shouldAskAgentFocus`.
fn should_ask_agent_focus_inner(stored: Option<String>) -> Result<bool, String> {
    let out = port::should_ask_agent_focus(stored)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "shouldAskAgentFocus")]
pub fn should_ask_agent_focus(stored: Option<String>) -> Result<bool, JsError> {
    should_ask_agent_focus_inner(stored).map_err(|e| JsError::new(&e))
}

/// `agentVisible`.
fn agent_visible_inner(focus: String, agent: String) -> Result<bool, String> {
    let focus = crate::js::from_json(&focus)?;
    let out = port::agent_visible(focus, agent)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "agentVisible")]
pub fn agent_visible(focus: String, agent: String) -> Result<bool, JsError> {
    agent_visible_inner(focus, agent).map_err(|e| JsError::new(&e))
}

/// `providersHidden`.
fn providers_hidden_inner(focus: String) -> Result<bool, String> {
    let focus = crate::js::from_json(&focus)?;
    let out = port::providers_hidden(focus)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "providersHidden")]
pub fn providers_hidden(focus: String) -> Result<bool, JsError> {
    providers_hidden_inner(focus).map_err(|e| JsError::new(&e))
}

/// `singleAgent`.
fn single_agent_inner(focus: String) -> Result<String, String> {
    let focus = crate::js::from_json(&focus)?;
    let out = port::single_agent(focus)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "singleAgent")]
pub fn single_agent(focus: String) -> Result<String, JsError> {
    single_agent_inner(focus).map_err(|e| JsError::new(&e))
}

/// `visibleAgents`.
fn visible_agents_inner(focus: String, items: String) -> Result<String, String> {
    let focus = crate::js::from_json(&focus)?;
    let items = crate::js::from_json(&items)?;
    let out = port::visible_agents(focus, items)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "visibleAgents")]
pub fn visible_agents(focus: String, items: String) -> Result<String, JsError> {
    visible_agents_inner(focus, items).map_err(|e| JsError::new(&e))
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

/// `parseAgentFocus` over the binary wire.
#[wasm_bindgen(js_name = "parseAgentFocusWire")]
pub fn parse_agent_focus_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let stored: Option<String> = r.opt_js(|r| r.string16())?;
        let out = &port::parse_agent_focus(stored)?;
        wire::write_dyn(w, &serde_json::to_value(out).map_err(|e| e.to_string())?);
        Ok(())
    })
}

static S_SERIALIZE_AGENT_FOCUS_0: wire::Schema = wire::Schema::Dyn;

/// `serializeAgentFocus` over the binary wire.
#[wasm_bindgen(js_name = "serializeAgentFocusWire")]
pub fn serialize_agent_focus_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let mode: serde_json::Value = wire::read_value(r, &S_SERIALIZE_AGENT_FOCUS_0)?;
        let out = &port::serialize_agent_focus(mode)?;
        w.string16(out);
        Ok(())
    })
}

/// `agentVisible` over the binary wire.
#[wasm_bindgen(js_name = "agentVisibleWire")]
pub fn agent_visible_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let focus: Option<serde_json::Value> = wire::read_dyn_opt(r)?;
        let agent: String = r.string16()?;
        let out = &port::agent_visible(focus, agent)?;
        w.bool(*out);
        Ok(())
    })
}

/// `providersHidden` over the binary wire.
#[wasm_bindgen(js_name = "providersHiddenWire")]
pub fn providers_hidden_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let focus: Option<serde_json::Value> = wire::read_dyn_opt(r)?;
        let out = &port::providers_hidden(focus)?;
        w.bool(*out);
        Ok(())
    })
}

static S_SINGLE_AGENT_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Str16);

/// `singleAgent` over the binary wire.
#[wasm_bindgen(js_name = "singleAgentWire")]
pub fn single_agent_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let focus: Option<serde_json::Value> = wire::read_dyn_opt(r)?;
        let out = &port::single_agent(focus)?;
        wire::write_value(w, out, &S_SINGLE_AGENT_OUT)?;
        Ok(())
    })
}

static S_VISIBLE_AGENTS_1: wire::Schema = wire::Schema::Dyn;
static S_VISIBLE_AGENTS_OUT: wire::Schema =
    wire::Schema::List(&wire::Schema::Obj(&[("agent", wire::Schema::Str16)]));

/// `visibleAgents` over the binary wire.
#[wasm_bindgen(js_name = "visibleAgentsWire")]
pub fn visible_agents_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let focus: Option<serde_json::Value> = wire::read_dyn_opt(r)?;
        let items: serde_json::Value = wire::read_value(r, &S_VISIBLE_AGENTS_1)?;
        let out = &port::visible_agents(focus, items)?;
        wire::write_value(w, out, &S_VISIBLE_AGENTS_OUT)?;
        Ok(())
    })
}
