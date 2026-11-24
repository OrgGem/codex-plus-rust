//! JS bindings for `src/request-flow-model.ts`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;
pub mod utf16;

/// `flowStatus`.
fn flow_status_inner(entry: String) -> Result<String, String> {
    let entry = crate::js::from_json(&entry)?;
    let out = port::flow_status(entry)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "flowStatus")]
pub fn flow_status(entry: String) -> Result<String, JsError> {
    flow_status_inner(entry).map_err(|e| JsError::new(&e))
}

/// `flowAgent`.
fn flow_agent_inner(entry: String) -> Result<String, String> {
    let entry = crate::js::from_json(&entry)?;
    let out = port::flow_agent(entry)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "flowAgent")]
pub fn flow_agent(entry: String) -> Result<String, JsError> {
    flow_agent_inner(entry).map_err(|e| JsError::new(&e))
}

/// `agentLinkKey`.
fn agent_link_key_inner(agent: String) -> Result<String, String> {
    let out = port::agent_link_key(agent)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "agentLinkKey")]
pub fn agent_link_key(agent: String) -> Result<String, JsError> {
    agent_link_key_inner(agent).map_err(|e| JsError::new(&e))
}

/// `providerLinkKey`.
fn provider_link_key_inner(relay_id: String) -> Result<String, String> {
    let out = port::provider_link_key(relay_id)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "providerLinkKey")]
pub fn provider_link_key(relay_id: String) -> Result<String, JsError> {
    provider_link_key_inner(relay_id).map_err(|e| JsError::new(&e))
}

/// `pulseHops`.
fn pulse_hops_inner(pulse: String) -> Result<String, String> {
    let pulse = crate::js::from_json(&pulse)?;
    let out = port::pulse_hops(pulse)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "pulseHops")]
pub fn pulse_hops(pulse: String) -> Result<String, JsError> {
    pulse_hops_inner(pulse).map_err(|e| JsError::new(&e))
}

/// `liveFlowRoutes`.
fn live_flow_routes_inner(entries: String) -> Result<String, String> {
    let entries = crate::js::from_json(&entries)?;
    let out = port::live_flow_routes(entries)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "liveFlowRoutes")]
pub fn live_flow_routes(entries: String) -> Result<String, JsError> {
    live_flow_routes_inner(entries).map_err(|e| JsError::new(&e))
}

/// `flowRoutePosition`.
fn flow_route_position_inner(lengths: String, distance: String) -> Result<String, String> {
    let lengths = crate::js::from_json(&lengths)?;
    let distance = crate::js::from_json(&distance)?;
    let out = port::flow_route_position(lengths, distance)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "flowRoutePosition")]
pub fn flow_route_position(lengths: String, distance: String) -> Result<String, JsError> {
    flow_route_position_inner(lengths, distance).map_err(|e| JsError::new(&e))
}

/// `flowColumns`.
fn flow_columns_inner(entries: String, max_per_side: String) -> Result<String, String> {
    let entries = crate::js::from_json(&entries)?;
    let max_per_side = crate::js::from_json(&max_per_side)?;
    let out = port::flow_columns(entries, max_per_side)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "flowColumns")]
pub fn flow_columns(entries: String, max_per_side: String) -> Result<String, JsError> {
    flow_columns_inner(entries, max_per_side).map_err(|e| JsError::new(&e))
}

/// `flowElapsed`.
fn flow_elapsed_inner(entry: String, now: String) -> Result<f64, String> {
    let entry = crate::js::from_json(&entry)?;
    let now = crate::js::from_json(&now)?;
    let out = port::flow_elapsed(entry, now)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "flowElapsed")]
pub fn flow_elapsed(entry: String, now: String) -> Result<f64, JsError> {
    flow_elapsed_inner(entry, now).map_err(|e| JsError::new(&e))
}

/// `clockTime`.
fn clock_time_inner(ms: String) -> Result<String, String> {
    let ms = crate::js::from_json(&ms)?;
    let out = port::clock_time(ms)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "clockTime")]
pub fn clock_time(ms: String) -> Result<String, JsError> {
    clock_time_inner(ms).map_err(|e| JsError::new(&e))
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

static S_FLOW_STATUS_0: wire::Schema = wire::Schema::Dyn;

/// `flowStatus` over the binary wire.
#[wasm_bindgen(js_name = "flowStatusWire")]
pub fn flow_status_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let entry: Value = wire::read_value(r, &S_FLOW_STATUS_0)?;
        let out = &port::flow_status(entry)?;
        w.string16(out);
        Ok(())
    })
}

static S_FLOW_AGENT_0: wire::Schema = wire::Schema::Dyn;

/// `flowAgent` over the binary wire.
#[wasm_bindgen(js_name = "flowAgentWire")]
pub fn flow_agent_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let entry: Value = wire::read_value(r, &S_FLOW_AGENT_0)?;
        let out = &port::flow_agent(entry)?;
        w.string16(out);
        Ok(())
    })
}

static S_PULSE_HOPS_0: wire::Schema = wire::Schema::Dyn;
static S_PULSE_HOPS_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("key", wire::Schema::Str16),
    ("reverse", wire::Schema::Bool),
]));

/// `pulseHops` over the binary wire.
#[wasm_bindgen(js_name = "pulseHopsWire")]
pub fn pulse_hops_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let pulse: Value = wire::read_value(r, &S_PULSE_HOPS_0)?;
        let out = &port::pulse_hops(pulse)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_PULSE_HOPS_OUT.inner())?;
        }
        Ok(())
    })
}

static S_LIVE_FLOW_ROUTES_0: wire::Schema = wire::Schema::Dyn;
static S_LIVE_FLOW_ROUTES_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("key", wire::Schema::Str16),
    ("agent", wire::Schema::Str16),
    ("relayId", wire::Schema::Str16),
    ("reverse", wire::Schema::Bool),
    ("count", wire::Schema::Num),
]));

/// `liveFlowRoutes` over the binary wire.
#[wasm_bindgen(js_name = "liveFlowRoutesWire")]
pub fn live_flow_routes_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let entries: Value = wire::read_value(r, &S_LIVE_FLOW_ROUTES_0)?;
        let out = &port::live_flow_routes(entries)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_LIVE_FLOW_ROUTES_OUT.inner())?;
        }
        Ok(())
    })
}

static S_FLOW_ROUTE_POSITION_0: wire::Schema = wire::Schema::Dyn;
static S_FLOW_ROUTE_POSITION_1: wire::Schema = wire::Schema::Dyn;
static S_FLOW_ROUTE_POSITION_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Obj(&[
    ("segment", wire::Schema::Num),
    ("progress", wire::Schema::Num),
]));

/// `flowRoutePosition` over the binary wire.
#[wasm_bindgen(js_name = "flowRoutePositionWire")]
pub fn flow_route_position_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let lengths: Value = wire::read_value(r, &S_FLOW_ROUTE_POSITION_0)?;
        let distance: Value = wire::read_value(r, &S_FLOW_ROUTE_POSITION_1)?;
        let out = &port::flow_route_position(lengths, distance)?;
        wire::write_value(w, out, &S_FLOW_ROUTE_POSITION_OUT)?;
        Ok(())
    })
}

static S_FLOW_COLUMNS_0: wire::Schema = wire::Schema::Dyn;
static S_FLOW_COLUMNS_1: wire::Schema = wire::Schema::Dyn;
static S_FLOW_COLUMNS_OUT: wire::Schema = wire::Schema::Obj(&[
    (
        "agents",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("id", wire::Schema::Str16),
            ("name", wire::Schema::Str16),
            ("live", wire::Schema::Bool),
            ("count", wire::Schema::Num),
        ])),
    ),
    (
        "providers",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("id", wire::Schema::Str16),
            ("name", wire::Schema::Str16),
            ("live", wire::Schema::Bool),
            ("count", wire::Schema::Num),
        ])),
    ),
]);

/// `flowColumns` over the binary wire.
#[wasm_bindgen(js_name = "flowColumnsWire")]
pub fn flow_columns_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let entries: Value = wire::read_value(r, &S_FLOW_COLUMNS_0)?;
        let max_per_side: Value = wire::read_value(r, &S_FLOW_COLUMNS_1)?;
        let out = &port::flow_columns(entries, max_per_side)?;
        wire::write_value(w, out, &S_FLOW_COLUMNS_OUT)?;
        Ok(())
    })
}

static S_FLOW_ELAPSED_0: wire::Schema = wire::Schema::Dyn;
static S_FLOW_ELAPSED_1: wire::Schema = wire::Schema::Dyn;

/// `flowElapsed` over the binary wire.
#[wasm_bindgen(js_name = "flowElapsedWire")]
pub fn flow_elapsed_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let entry: Value = wire::read_value(r, &S_FLOW_ELAPSED_0)?;
        let now: Value = wire::read_value(r, &S_FLOW_ELAPSED_1)?;
        let out = &port::flow_elapsed(entry, now)?;
        w.f64(*out);
        Ok(())
    })
}

static S_CLOCK_TIME_0: wire::Schema = wire::Schema::Dyn;

/// `clockTime` over the binary wire.
#[wasm_bindgen(js_name = "clockTimeWire")]
pub fn clock_time_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let ms: Value = wire::read_value(r, &S_CLOCK_TIME_0)?;
        let out = &port::clock_time(ms)?;
        w.string16(out);
        Ok(())
    })
}
