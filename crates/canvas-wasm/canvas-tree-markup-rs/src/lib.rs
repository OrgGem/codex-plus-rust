//! JS bindings for `tree-markup.mjs`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;
pub mod utf16;

/// `treeMarkup`.
fn tree_markup_inner(
    nodes: String,
    current_node_id: String,
    collapsed: String,
    selected: String,
    options: String,
) -> Result<String, String> {
    let nodes = crate::js::from_json(&nodes)?;
    let current_node_id = crate::js::from_json(&current_node_id)?;
    let collapsed = crate::js::from_json(&collapsed)?;
    let selected = crate::js::from_json(&selected)?;
    let options = crate::js::from_json(&options)?;
    let out = port::tree_markup(nodes, current_node_id, collapsed, selected, options)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "treeMarkup")]
pub fn tree_markup(
    nodes: String,
    current_node_id: String,
    collapsed: String,
    selected: String,
    options: String,
) -> Result<String, JsError> {
    tree_markup_inner(nodes, current_node_id, collapsed, selected, options)
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

static S_TREE_MARKUP_0: wire::Schema = wire::Schema::Dyn;
static S_TREE_MARKUP_1: wire::Schema = wire::Schema::Dyn;
static S_TREE_MARKUP_2: wire::Schema = wire::Schema::Dyn;
static S_TREE_MARKUP_3: wire::Schema = wire::Schema::Dyn;
static S_TREE_MARKUP_4: wire::Schema = wire::Schema::Dyn;
static S_TREE_MARKUP_OUT: wire::Schema = wire::Schema::Dyn;

/// `treeMarkup` over the binary wire.
#[wasm_bindgen(js_name = "treeMarkupWire")]
pub fn tree_markup_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let nodes: Value = wire::read_value(r, &S_TREE_MARKUP_0)?;
        let current_node_id: Value = wire::read_value(r, &S_TREE_MARKUP_1)?;
        let collapsed: Value = wire::read_value(r, &S_TREE_MARKUP_2)?;
        let selected: Value = wire::read_value(r, &S_TREE_MARKUP_3)?;
        let options: Value = wire::read_value(r, &S_TREE_MARKUP_4)?;
        let out = &port::tree_markup(nodes, current_node_id, collapsed, selected, options)?;
        wire::write_value(w, out, &S_TREE_MARKUP_OUT)?;
        Ok(())
    })
}
