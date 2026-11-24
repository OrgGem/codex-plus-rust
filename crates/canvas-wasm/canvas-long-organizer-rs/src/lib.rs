//! JS bindings for `long-organizer.mjs`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;
pub mod utf16;

/// `nextBatch`.
fn next_batch_inner(parts: String, done: String, max_chars: String) -> Result<String, String> {
    let parts = crate::js::from_json(&parts)?;
    let done = crate::js::from_json(&done)?;
    let max_chars = crate::js::from_json(&max_chars)?;
    let out = port::next_batch(parts, done, max_chars)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "nextBatch")]
pub fn next_batch(parts: String, done: String, max_chars: String) -> Result<String, JsError> {
    next_batch_inner(parts, done, max_chars).map_err(|e| JsError::new(&e))
}

/// `selectTreeContext`.
fn select_tree_context_inner(
    nodes: String,
    batch: String,
    current_node_id: String,
) -> Result<String, String> {
    let nodes = crate::js::from_json(&nodes)?;
    let batch = crate::js::from_json(&batch)?;
    let current_node_id = crate::js::from_json(&current_node_id)?;
    let out = port::select_tree_context(nodes, batch, current_node_id)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "selectTreeContext")]
pub fn select_tree_context(
    nodes: String,
    batch: String,
    current_node_id: String,
) -> Result<String, JsError> {
    select_tree_context_inner(nodes, batch, current_node_id).map_err(|e| JsError::new(&e))
}

/// `batchPrompt`.
fn batch_prompt_inner(
    batch: String,
    state: String,
    request_id: String,
    compact: String,
) -> Result<String, String> {
    let batch = crate::js::from_json(&batch)?;
    let state = crate::js::from_json(&state)?;
    let request_id = crate::js::from_json(&request_id)?;
    let compact = crate::js::from_json(&compact)?;
    let out = port::batch_prompt(batch, state, request_id, compact)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "batchPrompt")]
pub fn batch_prompt(
    batch: String,
    state: String,
    request_id: String,
    compact: String,
) -> Result<String, JsError> {
    batch_prompt_inner(batch, state, request_id, compact).map_err(|e| JsError::new(&e))
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

static S_NEXT_BATCH_0: wire::Schema = wire::Schema::Dyn;
static S_NEXT_BATCH_1: wire::Schema = wire::Schema::Dyn;
static S_NEXT_BATCH_2: wire::Schema = wire::Schema::Dyn;
static S_NEXT_BATCH_OUT: wire::Schema = wire::Schema::Dyn;

/// `nextBatch` over the binary wire.
#[wasm_bindgen(js_name = "nextBatchWire")]
pub fn next_batch_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let parts: Value = wire::read_value(r, &S_NEXT_BATCH_0)?;
        let done: Value = wire::read_value(r, &S_NEXT_BATCH_1)?;
        let max_chars: Value = wire::read_value(r, &S_NEXT_BATCH_2)?;
        let out = &port::next_batch(parts, done, max_chars)?;
        wire::write_value(w, out, &S_NEXT_BATCH_OUT)?;
        Ok(())
    })
}

static S_SELECT_TREE_CONTEXT_0: wire::Schema = wire::Schema::Dyn;
static S_SELECT_TREE_CONTEXT_1: wire::Schema = wire::Schema::Dyn;
static S_SELECT_TREE_CONTEXT_2: wire::Schema = wire::Schema::Dyn;
static S_SELECT_TREE_CONTEXT_OUT: wire::Schema = wire::Schema::Dyn;

/// `selectTreeContext` over the binary wire.
#[wasm_bindgen(js_name = "selectTreeContextWire")]
pub fn select_tree_context_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let nodes: Value = wire::read_value(r, &S_SELECT_TREE_CONTEXT_0)?;
        let batch: Value = wire::read_value(r, &S_SELECT_TREE_CONTEXT_1)?;
        let current_node_id: Value = wire::read_value(r, &S_SELECT_TREE_CONTEXT_2)?;
        let out = &port::select_tree_context(nodes, batch, current_node_id)?;
        wire::write_value(w, out, &S_SELECT_TREE_CONTEXT_OUT)?;
        Ok(())
    })
}

static S_BATCH_PROMPT_0: wire::Schema = wire::Schema::Dyn;
static S_BATCH_PROMPT_1: wire::Schema = wire::Schema::Dyn;
static S_BATCH_PROMPT_2: wire::Schema = wire::Schema::Dyn;
static S_BATCH_PROMPT_3: wire::Schema = wire::Schema::Dyn;
static S_BATCH_PROMPT_OUT: wire::Schema = wire::Schema::Dyn;

/// `batchPrompt` over the binary wire.
#[wasm_bindgen(js_name = "batchPromptWire")]
pub fn batch_prompt_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let batch: Value = wire::read_value(r, &S_BATCH_PROMPT_0)?;
        let state: Value = wire::read_value(r, &S_BATCH_PROMPT_1)?;
        let request_id: Value = wire::read_value(r, &S_BATCH_PROMPT_2)?;
        let compact: Value = wire::read_value(r, &S_BATCH_PROMPT_3)?;
        let out = &port::batch_prompt(batch, state, request_id, compact)?;
        wire::write_value(w, out, &S_BATCH_PROMPT_OUT)?;
        Ok(())
    })
}
