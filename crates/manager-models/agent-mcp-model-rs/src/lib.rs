//! JS bindings for `src/agent-mcp-model.ts`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;
pub mod utf16;

/// `mcpOn`.
fn mcp_on_inner(row: String, agent: String) -> Result<bool, String> {
    let row = crate::js::from_json(&row)?;
    let out = port::mcp_on(row, agent)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "mcpOn")]
pub fn mcp_on(row: String, agent: String) -> Result<bool, JsError> {
    mcp_on_inner(row, agent).map_err(|e| JsError::new(&e))
}

/// `mcpPresent`.
fn mcp_present_inner(row: String) -> Result<Vec<String>, String> {
    let row = crate::js::from_json(&row)?;
    let out = port::mcp_present(row)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "mcpPresent")]
pub fn mcp_present(row: String) -> Result<Vec<String>, JsError> {
    mcp_present_inner(row).map_err(|e| JsError::new(&e))
}

/// `mergeMcpRows`.
fn merge_mcp_rows_inner(codex: String, others: String) -> Result<String, String> {
    let codex = crate::js::from_json(&codex)?;
    let others = crate::js::from_json(&others)?;
    let out = port::merge_mcp_rows(codex, others)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "mergeMcpRows")]
pub fn merge_mcp_rows(codex: String, others: String) -> Result<String, JsError> {
    merge_mcp_rows_inner(codex, others).map_err(|e| JsError::new(&e))
}

/// `mcpCount`.
fn mcp_count_inner(rows: String, agent: String) -> Result<f64, String> {
    let rows = crate::js::from_json(&rows)?;
    let out = port::mcp_count(rows, agent)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "mcpCount")]
pub fn mcp_count(rows: String, agent: String) -> Result<f64, JsError> {
    mcp_count_inner(rows, agent).map_err(|e| JsError::new(&e))
}

/// `bodyEnabledState`.
fn body_enabled_state_inner(body: Option<String>) -> Result<Option<bool>, String> {
    let out = port::body_enabled_state(body)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "bodyEnabledState")]
pub fn body_enabled_state(body: Option<String>) -> Result<Option<bool>, JsError> {
    body_enabled_state_inner(body).map_err(|e| JsError::new(&e))
}

/// `editedEnabledState`.
fn edited_enabled_state_inner(
    original: Option<String>,
    edited: String,
) -> Result<Option<bool>, String> {
    let out = port::edited_enabled_state(original, edited)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "editedEnabledState")]
pub fn edited_enabled_state(
    original: Option<String>,
    edited: String,
) -> Result<Option<bool>, JsError> {
    edited_enabled_state_inner(original, edited).map_err(|e| JsError::new(&e))
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

static S_MCP_ON_0: wire::Schema = wire::Schema::Dyn;

/// `mcpOn` over the binary wire.
#[wasm_bindgen(js_name = "mcpOnWire")]
pub fn mcp_on_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let row: serde_json::Value = wire::read_value(r, &S_MCP_ON_0)?;
        let agent: String = r.string16()?;
        let out = &port::mcp_on(row, agent)?;
        w.bool(*out);
        Ok(())
    })
}

static S_MCP_PRESENT_0: wire::Schema = wire::Schema::Dyn;

/// `mcpPresent` over the binary wire.
#[wasm_bindgen(js_name = "mcpPresentWire")]
pub fn mcp_present_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let row: serde_json::Value = wire::read_value(r, &S_MCP_PRESENT_0)?;
        let out = &port::mcp_present(row)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            w.string16(x0);
        }
        Ok(())
    })
}

static S_MERGE_MCP_ROWS_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("tomlBody", wire::Schema::Str16),
    ("enabled", wire::Schema::Bool),
]));
static S_MERGE_MCP_ROWS_1: wire::Schema = wire::Schema::Dyn;
static S_MERGE_MCP_ROWS_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("body", wire::Schema::Str16),
    ("byAgent", wire::Schema::Dyn),
]));

/// `mergeMcpRows` over the binary wire.
#[wasm_bindgen(js_name = "mergeMcpRowsWire")]
pub fn merge_mcp_rows_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let codex: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_MERGE_MCP_ROWS_0.inner()))?;
        let others: serde_json::Value = wire::read_value(r, &S_MERGE_MCP_ROWS_1)?;
        let out = &port::merge_mcp_rows(codex, others)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_MERGE_MCP_ROWS_OUT.inner())?;
        }
        Ok(())
    })
}

static S_MCP_COUNT_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("body", wire::Schema::Str16),
    ("byAgent", wire::Schema::Dyn),
]));

/// `mcpCount` over the binary wire.
#[wasm_bindgen(js_name = "mcpCountWire")]
pub fn mcp_count_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let rows: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_MCP_COUNT_0.inner()))?;
        let agent: String = r.string16()?;
        let out = &port::mcp_count(rows, agent)?;
        w.f64(*out);
        Ok(())
    })
}
