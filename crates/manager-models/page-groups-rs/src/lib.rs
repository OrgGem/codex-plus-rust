//! JS bindings for `src/page-groups.ts`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;

/// `pageVisibleForTool`.
fn page_visible_for_tool_inner(page: String, tool: String) -> Result<bool, String> {
    let page = crate::js::from_json(&page)?;
    let out = port::page_visible_for_tool(page, tool)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "pageVisibleForTool")]
pub fn page_visible_for_tool(page: String, tool: String) -> Result<bool, JsError> {
    page_visible_for_tool_inner(page, tool).map_err(|e| JsError::new(&e))
}

/// `agentTarget`.
fn agent_target_inner(
    visible: Vec<String>,
    remembered: Option<String>,
    home: String,
) -> Result<String, String> {
    let out = port::agent_target(visible, remembered, home)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "agentTarget")]
pub fn agent_target(
    visible: Vec<String>,
    remembered: Option<String>,
    home: String,
) -> Result<String, JsError> {
    agent_target_inner(visible, remembered, home).map_err(|e| JsError::new(&e))
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

/// `pageVisibleForTool` over the binary wire.
#[wasm_bindgen(js_name = "pageVisibleForToolWire")]
pub fn page_visible_for_tool_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let page: Option<serde_json::Value> = wire::read_dyn_opt(r)?;
        let tool: String = r.string16()?;
        let out = &port::page_visible_for_tool(page, tool)?;
        w.bool(*out);
        Ok(())
    })
}
