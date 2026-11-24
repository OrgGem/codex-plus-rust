//! JS bindings for `src/hub-overview-model.ts`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;

/// `hubAgents`.
fn hub_agents_inner(payload: String, codex: String) -> Result<String, String> {
    let payload = crate::js::from_json(&payload)?;
    let codex = crate::js::from_json(&codex)?;
    let out = port::hub_agents(payload, codex)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "hubAgents")]
pub fn hub_agents(payload: String, codex: String) -> Result<String, JsError> {
    hub_agents_inner(payload, codex).map_err(|e| JsError::new(&e))
}

/// `hubProviders`.
fn hub_providers_inner(payload: String, calls_by_provider: String) -> Result<String, String> {
    let payload = crate::js::from_json(&payload)?;
    let calls_by_provider = crate::js::from_json(&calls_by_provider)?;
    let out = port::hub_providers(payload, calls_by_provider)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "hubProviders")]
pub fn hub_providers(payload: String, calls_by_provider: String) -> Result<String, JsError> {
    hub_providers_inner(payload, calls_by_provider).map_err(|e| JsError::new(&e))
}

/// `hubPoolState`.
fn hub_pool_state_inner(payload: String) -> Result<String, String> {
    let payload = crate::js::from_json(&payload)?;
    let out = port::hub_pool_state(payload)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "hubPoolState")]
pub fn hub_pool_state(payload: String) -> Result<String, JsError> {
    hub_pool_state_inner(payload).map_err(|e| JsError::new(&e))
}

/// `hubLinks`.
fn hub_links_inner(agents: String, providers: String) -> Result<String, String> {
    let agents = crate::js::from_json(&agents)?;
    let providers = crate::js::from_json(&providers)?;
    let out = port::hub_links(agents, providers)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "hubLinks")]
pub fn hub_links(agents: String, providers: String) -> Result<String, JsError> {
    hub_links_inner(agents, providers).map_err(|e| JsError::new(&e))
}

/// `hubFeed`.
fn hub_feed_inner(payload: String) -> Result<String, String> {
    let payload = crate::js::from_json(&payload)?;
    let out = port::hub_feed(payload)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "hubFeed")]
pub fn hub_feed(payload: String) -> Result<String, JsError> {
    hub_feed_inner(payload).map_err(|e| JsError::new(&e))
}

/// `hubTotals`.
fn hub_totals_inner(payload: String) -> Result<String, String> {
    let payload = crate::js::from_json(&payload)?;
    let out = port::hub_totals(payload)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "hubTotals")]
pub fn hub_totals(payload: String) -> Result<String, JsError> {
    hub_totals_inner(payload).map_err(|e| JsError::new(&e))
}

/// `successRate`.
fn success_rate_inner(counts: String) -> Result<String, String> {
    let counts = crate::js::from_json(&counts)?;
    let out = port::success_rate(counts)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "successRate")]
pub fn success_rate(counts: String) -> Result<String, JsError> {
    success_rate_inner(counts).map_err(|e| JsError::new(&e))
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

static S_HUB_AGENTS_0: wire::Schema = wire::Schema::Dyn;
static S_HUB_AGENTS_1: wire::Schema = wire::Schema::Dyn;
static S_HUB_AGENTS_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("name", wire::Schema::Str16),
    ("enabled", wire::Schema::Bool),
    ("direct", wire::Schema::Bool),
    ("detail", wire::Schema::Str16),
    ("requests", wire::Schema::Num),
    ("live", wire::Schema::Bool),
]));

/// `hubAgents` over the binary wire.
#[wasm_bindgen(js_name = "hubAgentsWire")]
pub fn hub_agents_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let payload: Value = wire::read_value(r, &S_HUB_AGENTS_0)?;
        let codex: Value = wire::read_value(r, &S_HUB_AGENTS_1)?;
        let out = &port::hub_agents(payload, codex)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_HUB_AGENTS_OUT.inner())?;
        }
        Ok(())
    })
}

static S_HUB_PROVIDERS_0: wire::Schema = wire::Schema::Dyn;
static S_HUB_PROVIDERS_1: wire::Schema = wire::Schema::Dyn;
static S_HUB_PROVIDERS_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("name", wire::Schema::Str16),
    ("baseUrl", wire::Schema::Str16),
    ("keyCount", wire::Schema::Num),
    (
        "status",
        wire::Schema::Obj(&[
            ("state", wire::Schema::Str16),
            ("text", wire::Schema::Str16),
            ("args", wire::Schema::List(&wire::Schema::Dyn)),
        ]),
    ),
    ("calls", wire::Schema::OptJs(&wire::Schema::Num)),
]));

/// `hubProviders` over the binary wire.
#[wasm_bindgen(js_name = "hubProvidersWire")]
pub fn hub_providers_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let payload: Value = wire::read_value(r, &S_HUB_PROVIDERS_0)?;
        let calls_by_provider: Value = wire::read_value(r, &S_HUB_PROVIDERS_1)?;
        let out = &port::hub_providers(payload, calls_by_provider)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_HUB_PROVIDERS_OUT.inner())?;
        }
        Ok(())
    })
}

static S_HUB_POOL_STATE_0: wire::Schema = wire::Schema::Dyn;

/// `hubPoolState` over the binary wire.
#[wasm_bindgen(js_name = "hubPoolStateWire")]
pub fn hub_pool_state_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let payload: Value = wire::read_value(r, &S_HUB_POOL_STATE_0)?;
        let out = &port::hub_pool_state(payload)?;
        w.string16(out);
        Ok(())
    })
}

static S_HUB_LINKS_0: wire::Schema = wire::Schema::Dyn;
static S_HUB_LINKS_1: wire::Schema = wire::Schema::Dyn;
static S_HUB_LINKS_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("from", wire::Schema::Str16),
    ("to", wire::Schema::Str16),
    ("tone", wire::Schema::Str16),
    ("weight", wire::Schema::Num),
]));

/// `hubLinks` over the binary wire.
#[wasm_bindgen(js_name = "hubLinksWire")]
pub fn hub_links_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let agents: Value = wire::read_value(r, &S_HUB_LINKS_0)?;
        let providers: Value = wire::read_value(r, &S_HUB_LINKS_1)?;
        let out = &port::hub_links(agents, providers)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_HUB_LINKS_OUT.inner())?;
        }
        Ok(())
    })
}

static S_HUB_FEED_0: wire::Schema = wire::Schema::Dyn;
static S_HUB_FEED_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("agent", wire::Schema::Str16),
    ("requested", wire::Schema::Str16),
    ("provider", wire::Schema::Str16),
    ("model", wire::Schema::Str16),
    ("elapsedMs", wire::Schema::Num),
    ("result", wire::Schema::Str16),
    ("finishedAtMs", wire::Schema::Num),
]));

/// `hubFeed` over the binary wire.
#[wasm_bindgen(js_name = "hubFeedWire")]
pub fn hub_feed_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let payload: Value = wire::read_value(r, &S_HUB_FEED_0)?;
        let out = &port::hub_feed(payload)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_HUB_FEED_OUT.inner())?;
        }
        Ok(())
    })
}

static S_HUB_TOTALS_0: wire::Schema = wire::Schema::Dyn;
static S_HUB_TOTALS_OUT: wire::Schema = wire::Schema::Dyn;

/// `hubTotals` over the binary wire.
#[wasm_bindgen(js_name = "hubTotalsWire")]
pub fn hub_totals_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let payload: Value = wire::read_value(r, &S_HUB_TOTALS_0)?;
        let out = &port::hub_totals(payload)?;
        wire::write_value(w, out, &S_HUB_TOTALS_OUT)?;
        Ok(())
    })
}

static S_SUCCESS_RATE_0: wire::Schema = wire::Schema::Dyn;
static S_SUCCESS_RATE_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Num);

/// `successRate` over the binary wire.
#[wasm_bindgen(js_name = "successRateWire")]
pub fn success_rate_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let counts: Value = wire::read_value(r, &S_SUCCESS_RATE_0)?;
        let out = &port::success_rate(counts)?;
        wire::write_value(w, out, &S_SUCCESS_RATE_OUT)?;
        Ok(())
    })
}
