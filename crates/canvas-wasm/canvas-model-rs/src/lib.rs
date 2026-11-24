//! JS bindings for `model.mjs`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;
pub mod utf16;

/// `parseMessage`.
fn parse_message_inner(record: String, fallback_id: String) -> Result<String, String> {
    let record = crate::js::from_json(&record)?;
    let fallback_id = crate::js::from_json(&fallback_id)?;
    let out = port::parse_message(record, fallback_id)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "parseMessage")]
pub fn parse_message(record: String, fallback_id: String) -> Result<String, JsError> {
    parse_message_inner(record, fallback_id).map_err(|e| JsError::new(&e))
}

/// `excerpt`.
fn excerpt_inner(text: String, length: Option<f64>) -> Result<String, String> {
    let text = crate::js::from_json(&text)?;
    let out = port::excerpt(text, length)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "excerpt")]
pub fn excerpt(text: String, length: Option<f64>) -> Result<String, JsError> {
    excerpt_inner(text, length).map_err(|e| JsError::new(&e))
}

/// `buildGraph`.
fn build_graph_inner(messages: String, annotations: String) -> Result<String, String> {
    let messages = crate::js::from_json(&messages)?;
    let annotations = crate::js::from_json(&annotations)?;
    let out = port::build_graph(messages, annotations)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "buildGraph")]
pub fn build_graph(messages: String, annotations: String) -> Result<String, JsError> {
    build_graph_inner(messages, annotations).map_err(|e| JsError::new(&e))
}

/// `visibleTreeRows`.
fn visible_tree_rows_inner(
    nodes: String,
    current_node_id: String,
    collapsed: String,
) -> Result<String, String> {
    let nodes = crate::js::from_json(&nodes)?;
    let current_node_id = crate::js::from_json(&current_node_id)?;
    let collapsed = crate::js::from_json(&collapsed)?;
    let out = port::visible_tree_rows(nodes, current_node_id, collapsed)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "visibleTreeRows")]
pub fn visible_tree_rows(
    nodes: String,
    current_node_id: String,
    collapsed: String,
) -> Result<String, JsError> {
    visible_tree_rows_inner(nodes, current_node_id, collapsed).map_err(|e| JsError::new(&e))
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

static S_PARSE_MESSAGE_0: wire::Schema = wire::Schema::Dyn;
static S_PARSE_MESSAGE_1: wire::Schema = wire::Schema::Dyn;
static S_PARSE_MESSAGE_OUT: wire::Schema = wire::Schema::Dyn;

/// `parseMessage` over the binary wire.
#[wasm_bindgen(js_name = "parseMessageWire")]
pub fn parse_message_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let record: Value = wire::read_value(r, &S_PARSE_MESSAGE_0)?;
        let fallback_id: Value = wire::read_value(r, &S_PARSE_MESSAGE_1)?;
        let out = &port::parse_message(record, fallback_id)?;
        wire::write_value(w, out, &S_PARSE_MESSAGE_OUT)?;
        Ok(())
    })
}

static S_EXCERPT_0: wire::Schema = wire::Schema::Dyn;
static S_EXCERPT_OUT: wire::Schema = wire::Schema::Dyn;

/// `excerpt` over the binary wire.
#[wasm_bindgen(js_name = "excerptWire")]
pub fn excerpt_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let text: Value = wire::read_value(r, &S_EXCERPT_0)?;
        let length: Option<f64> = r.opt_js(|r| r.f64())?;
        let out = &port::excerpt(text, length)?;
        wire::write_value(w, out, &S_EXCERPT_OUT)?;
        Ok(())
    })
}

static S_BUILD_GRAPH_0: wire::Schema = wire::Schema::Dyn;
static S_BUILD_GRAPH_1: wire::Schema = wire::Schema::Dyn;
static S_BUILD_GRAPH_OUT: wire::Schema = wire::Schema::Dyn;

/// `buildGraph` over the binary wire.
#[wasm_bindgen(js_name = "buildGraphWire")]
pub fn build_graph_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let messages: Value = wire::read_value(r, &S_BUILD_GRAPH_0)?;
        let annotations: Value = wire::read_value(r, &S_BUILD_GRAPH_1)?;
        let out = &port::build_graph(messages, annotations)?;
        wire::write_value(w, out, &S_BUILD_GRAPH_OUT)?;
        Ok(())
    })
}

static S_VISIBLE_TREE_ROWS_0: wire::Schema = wire::Schema::Dyn;
static S_VISIBLE_TREE_ROWS_1: wire::Schema = wire::Schema::Dyn;
static S_VISIBLE_TREE_ROWS_2: wire::Schema = wire::Schema::Dyn;
static S_VISIBLE_TREE_ROWS_OUT: wire::Schema = wire::Schema::Dyn;

/// `visibleTreeRows` over the binary wire.
#[wasm_bindgen(js_name = "visibleTreeRowsWire")]
pub fn visible_tree_rows_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let nodes: Value = wire::read_value(r, &S_VISIBLE_TREE_ROWS_0)?;
        let current_node_id: Value = wire::read_value(r, &S_VISIBLE_TREE_ROWS_1)?;
        let collapsed: Value = wire::read_value(r, &S_VISIBLE_TREE_ROWS_2)?;
        let out = &port::visible_tree_rows(nodes, current_node_id, collapsed)?;
        wire::write_value(w, out, &S_VISIBLE_TREE_ROWS_OUT)?;
        Ok(())
    })
}
