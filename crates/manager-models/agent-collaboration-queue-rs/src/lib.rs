//! JS bindings for `src/agent-collaboration-queue.ts`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;

/// `normalizeConcurrency`.
fn normalize_concurrency_inner(value: String, max: String) -> Result<f64, String> {
    let value = crate::js::from_json(&value)?;
    let max = crate::js::from_json(&max)?;
    let out = port::normalize_concurrency(value, max)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "normalizeConcurrency")]
pub fn normalize_concurrency(value: String, max: String) -> Result<f64, JsError> {
    normalize_concurrency_inner(value, max).map_err(|e| JsError::new(&e))
}

/// `agentQueueState`.
fn agent_queue_state_inner(
    tasks: String,
    agent_id: String,
    max_queued: Option<f64>,
    max_concurrent: Option<f64>,
) -> Result<String, String> {
    let tasks = crate::js::from_json(&tasks)?;
    let out = port::agent_queue_state(tasks, agent_id, max_queued, max_concurrent)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "agentQueueState")]
pub fn agent_queue_state(
    tasks: String,
    agent_id: String,
    max_queued: Option<f64>,
    max_concurrent: Option<f64>,
) -> Result<String, JsError> {
    agent_queue_state_inner(tasks, agent_id, max_queued, max_concurrent)
        .map_err(|e| JsError::new(&e))
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

static S_NORMALIZE_CONCURRENCY_0: wire::Schema = wire::Schema::Dyn;

/// `normalizeConcurrency` over the binary wire.
#[wasm_bindgen(js_name = "normalizeConcurrencyWire")]
pub fn normalize_concurrency_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let value: serde_json::Value = wire::read_value(r, &S_NORMALIZE_CONCURRENCY_0)?;
        let max: Option<serde_json::Value> = wire::read_dyn_opt(r)?;
        let out = &port::normalize_concurrency(value, max)?;
        w.f64(*out);
        Ok(())
    })
}

static S_AGENT_QUEUE_STATE_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("agentId", wire::Schema::Str16),
    ("status", wire::Schema::Str16),
]));
static S_AGENT_QUEUE_STATE_OUT: wire::Schema = wire::Schema::Obj(&[
    ("active", wire::Schema::Num),
    ("running", wire::Schema::Num),
    ("queued", wire::Schema::Num),
    ("limit", wire::Schema::Num),
    ("concurrency", wire::Schema::Num),
    ("willQueue", wire::Schema::Bool),
    ("full", wire::Schema::Bool),
]);

/// `agentQueueState` over the binary wire.
#[wasm_bindgen(js_name = "agentQueueStateWire")]
pub fn agent_queue_state_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let tasks: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_AGENT_QUEUE_STATE_0.inner()))?;
        let agent_id: String = r.string16()?;
        let max_queued: Option<f64> = r.opt_js(|r| r.f64())?;
        let max_concurrent: Option<f64> = r.opt_js(|r| r.f64())?;
        let out = &port::agent_queue_state(tasks, agent_id, max_queued, max_concurrent)?;
        wire::write_value(w, out, &S_AGENT_QUEUE_STATE_OUT)?;
        Ok(())
    })
}
