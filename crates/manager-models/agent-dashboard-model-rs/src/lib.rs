//! JS bindings for `src/agent-dashboard-model.ts`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;
pub mod utf16;

/// `recentAgentRequests`.
fn recent_agent_requests_inner(
    entries: String,
    agent: String,
    limit: String,
) -> Result<String, String> {
    let entries = crate::js::from_json(&entries)?;
    let limit = crate::js::from_json(&limit)?;
    let out = port::recent_agent_requests(entries, agent, limit)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "recentAgentRequests")]
pub fn recent_agent_requests(
    entries: String,
    agent: String,
    limit: String,
) -> Result<String, JsError> {
    recent_agent_requests_inner(entries, agent, limit).map_err(|e| JsError::new(&e))
}

/// `agentUsageStats`.
fn agent_usage_stats_inner(usage: String, agent: String) -> Result<String, String> {
    let usage = crate::js::from_json(&usage)?;
    let out = port::agent_usage_stats(usage, agent)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "agentUsageStats")]
pub fn agent_usage_stats(usage: String, agent: String) -> Result<String, JsError> {
    agent_usage_stats_inner(usage, agent).map_err(|e| JsError::new(&e))
}

/// `agentAccessState`.
fn agent_access_state_inner(view: String) -> Result<String, String> {
    let view = crate::js::from_json(&view)?;
    let out = port::agent_access_state(view)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "agentAccessState")]
pub fn agent_access_state(view: String) -> Result<String, JsError> {
    agent_access_state_inner(view).map_err(|e| JsError::new(&e))
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

static S_RECENT_AGENT_REQUESTS_0: wire::Schema = wire::Schema::Dyn;
static S_RECENT_AGENT_REQUESTS_2: wire::Schema = wire::Schema::Dyn;
static S_RECENT_AGENT_REQUESTS_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("model", wire::Schema::Str16),
    ("provider", wire::Schema::Str16),
    ("durationMs", wire::Schema::Num),
    ("success", wire::Schema::Bool),
    ("running", wire::Schema::Bool),
    ("timestampMs", wire::Schema::Num),
]));

/// `recentAgentRequests` over the binary wire.
#[wasm_bindgen(js_name = "recentAgentRequestsWire")]
pub fn recent_agent_requests_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let entries: Value = wire::read_value(r, &S_RECENT_AGENT_REQUESTS_0)?;
        let agent: String = r.string16()?;
        let limit: Value = wire::read_value(r, &S_RECENT_AGENT_REQUESTS_2)?;
        let out = &port::recent_agent_requests(entries, agent, limit)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_RECENT_AGENT_REQUESTS_OUT.inner())?;
        }
        Ok(())
    })
}

static S_AGENT_USAGE_STATS_0: wire::Schema = wire::Schema::Dyn;
static S_AGENT_USAGE_STATS_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Obj(&[
    ("calls", wire::Schema::Num),
    ("successRate", wire::Schema::OptJs(&wire::Schema::Num)),
    ("tokens", wire::Schema::Num),
]));

/// `agentUsageStats` over the binary wire.
#[wasm_bindgen(js_name = "agentUsageStatsWire")]
pub fn agent_usage_stats_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let usage: Value = wire::read_value(r, &S_AGENT_USAGE_STATS_0)?;
        let agent: String = r.string16()?;
        let out = &port::agent_usage_stats(usage, agent)?;
        wire::write_value(w, out, &S_AGENT_USAGE_STATS_OUT)?;
        Ok(())
    })
}

static S_AGENT_ACCESS_STATE_0: wire::Schema = wire::Schema::Dyn;

/// `agentAccessState` over the binary wire.
#[wasm_bindgen(js_name = "agentAccessStateWire")]
pub fn agent_access_state_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let view: Value = wire::read_value(r, &S_AGENT_ACCESS_STATE_0)?;
        let out = &port::agent_access_state(view)?;
        w.string16(out);
        Ok(())
    })
}
