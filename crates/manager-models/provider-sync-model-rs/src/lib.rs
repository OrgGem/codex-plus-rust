//! JS bindings for `src/provider-sync-model.ts`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;

/// `modelSlug`.
fn model_slug_inner(model: String) -> Result<String, String> {
    let out = port::model_slug(model)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "modelSlug")]
pub fn model_slug(model: String) -> Result<String, JsError> {
    model_slug_inner(model).map_err(|e| JsError::new(&e))
}

/// `sameModel`.
fn same_model_inner(left: String, right: String) -> Result<bool, String> {
    let out = port::same_model(left, right)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "sameModel")]
pub fn same_model(left: String, right: String) -> Result<bool, JsError> {
    same_model_inner(left, right).map_err(|e| JsError::new(&e))
}

/// `syncOutcome`.
fn sync_outcome_inner(preview: String, remove: String, add: String) -> Result<String, String> {
    let preview = crate::js::from_json(&preview)?;
    let remove = crate::js::from_json(&remove)?;
    let add = crate::js::from_json(&add)?;
    let out = port::sync_outcome(preview, remove, add)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "syncOutcome")]
pub fn sync_outcome(preview: String, remove: String, add: String) -> Result<String, JsError> {
    sync_outcome_inner(preview, remove, add).map_err(|e| JsError::new(&e))
}

/// `pickerRows`.
fn picker_rows_inner(existing: Vec<String>, fetched: Vec<String>) -> Result<String, String> {
    let out = port::picker_rows(existing, fetched)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "pickerRows")]
pub fn picker_rows(existing: Vec<String>, fetched: Vec<String>) -> Result<String, JsError> {
    picker_rows_inner(existing, fetched).map_err(|e| JsError::new(&e))
}

/// `pickerOutcome`.
fn picker_outcome_inner(rows: String, selected: String) -> Result<String, String> {
    let rows = crate::js::from_json(&rows)?;
    let selected = crate::js::from_json(&selected)?;
    let out = port::picker_outcome(rows, selected)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "pickerOutcome")]
pub fn picker_outcome(rows: String, selected: String) -> Result<String, JsError> {
    picker_outcome_inner(rows, selected).map_err(|e| JsError::new(&e))
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

static S_SYNC_OUTCOME_0: wire::Schema = wire::Schema::Dyn;
static S_SYNC_OUTCOME_1: wire::Schema = wire::Schema::Dyn;
static S_SYNC_OUTCOME_2: wire::Schema = wire::Schema::Dyn;
static S_SYNC_OUTCOME_OUT: wire::Schema = wire::Schema::Obj(&[
    ("models", wire::Schema::List(&wire::Schema::Str16)),
    ("defaultModel", wire::Schema::Str16),
    ("defaultReplaced", wire::Schema::Bool),
]);

/// `syncOutcome` over the binary wire.
#[wasm_bindgen(js_name = "syncOutcomeWire")]
pub fn sync_outcome_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let preview: serde_json::Value = wire::read_value(r, &S_SYNC_OUTCOME_0)?;
        let remove: serde_json::Value = wire::read_value(r, &S_SYNC_OUTCOME_1)?;
        let add: serde_json::Value = wire::read_value(r, &S_SYNC_OUTCOME_2)?;
        let out = &port::sync_outcome(preview, remove, add)?;
        wire::write_value(w, out, &S_SYNC_OUTCOME_OUT)?;
        Ok(())
    })
}

static S_PICKER_ROWS_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("model", wire::Schema::Str16),
    ("inList", wire::Schema::Bool),
    ("upstream", wire::Schema::Bool),
]));

/// `pickerRows` over the binary wire.
#[wasm_bindgen(js_name = "pickerRowsWire")]
pub fn picker_rows_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let existing: Vec<String> = r.list(|r| r.string16())?;
        let fetched: Vec<String> = r.list(|r| r.string16())?;
        let out = &port::picker_rows(existing, fetched)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_PICKER_ROWS_OUT.inner())?;
        }
        Ok(())
    })
}

static S_PICKER_OUTCOME_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("model", wire::Schema::Str16),
    ("inList", wire::Schema::Bool),
    ("upstream", wire::Schema::Bool),
]));
static S_PICKER_OUTCOME_1: wire::Schema = wire::Schema::Dyn;
static S_PICKER_OUTCOME_OUT: wire::Schema = wire::Schema::Obj(&[
    ("models", wire::Schema::List(&wire::Schema::Str16)),
    ("added", wire::Schema::Num),
    ("removed", wire::Schema::Num),
]);

/// `pickerOutcome` over the binary wire.
#[wasm_bindgen(js_name = "pickerOutcomeWire")]
pub fn picker_outcome_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let rows: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_PICKER_OUTCOME_0.inner()))?;
        let selected: serde_json::Value = wire::read_value(r, &S_PICKER_OUTCOME_1)?;
        let out = &port::picker_outcome(rows, selected)?;
        wire::write_value(w, out, &S_PICKER_OUTCOME_OUT)?;
        Ok(())
    })
}
