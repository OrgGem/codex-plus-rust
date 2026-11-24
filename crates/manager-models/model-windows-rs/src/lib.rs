//! JS bindings for `src/model-windows.ts`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;

/// `modelWindowsMapToText`.
fn model_windows_map_to_text_inner(
    model_list: String,
    model_windows: String,
) -> Result<String, String> {
    let out = port::model_windows_map_to_text(model_list, model_windows)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "modelWindowsMapToText")]
pub fn model_windows_map_to_text(
    model_list: String,
    model_windows: String,
) -> Result<String, JsError> {
    model_windows_map_to_text_inner(model_list, model_windows).map_err(|e| JsError::new(&e))
}

/// `modelWindowsTextToMap`.
fn model_windows_text_to_map_inner(
    model_list: String,
    model_windows_text: String,
) -> Result<String, String> {
    let out = port::model_windows_text_to_map(model_list, model_windows_text)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "modelWindowsTextToMap")]
pub fn model_windows_text_to_map(
    model_list: String,
    model_windows_text: String,
) -> Result<String, JsError> {
    model_windows_text_to_map_inner(model_list, model_windows_text).map_err(|e| JsError::new(&e))
}

/// `reorderModelWindowRows`.
fn reorder_model_window_rows_inner(
    rows: String,
    active_index: f64,
    over_index: f64,
) -> Result<String, String> {
    let rows = crate::js::from_json(&rows)?;
    let out = port::reorder_model_window_rows(rows, active_index, over_index)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "reorderModelWindowRows")]
pub fn reorder_model_window_rows(
    rows: String,
    active_index: f64,
    over_index: f64,
) -> Result<String, JsError> {
    reorder_model_window_rows_inner(rows, active_index, over_index).map_err(|e| JsError::new(&e))
}

/// `modelMapKeyFromRowName`.
fn model_map_key_from_row_name_inner(row_name: String) -> Result<String, String> {
    let out = port::model_map_key_from_row_name(row_name)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "modelMapKeyFromRowName")]
pub fn model_map_key_from_row_name(row_name: String) -> Result<String, JsError> {
    model_map_key_from_row_name_inner(row_name).map_err(|e| JsError::new(&e))
}

/// `lookupModelMapEntry`.
fn lookup_model_map_entry_inner(map: String, model_name: String) -> Result<String, String> {
    let map = crate::js::from_json(&map)?;
    let out = port::lookup_model_map_entry(map, model_name)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "lookupModelMapEntry")]
pub fn lookup_model_map_entry(map: String, model_name: String) -> Result<String, JsError> {
    lookup_model_map_entry_inner(map, model_name).map_err(|e| JsError::new(&e))
}

/// `isValidModelWindow`.
fn is_valid_model_window_inner(value: String) -> Result<bool, String> {
    let out = port::is_valid_model_window(value)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "isValidModelWindow")]
pub fn is_valid_model_window(value: String) -> Result<bool, JsError> {
    is_valid_model_window_inner(value).map_err(|e| JsError::new(&e))
}

/// `modelWindowRowsValidationError`.
fn model_window_rows_validation_error_inner(rows: String) -> Result<String, String> {
    let rows = crate::js::from_json(&rows)?;
    let out = port::model_window_rows_validation_error(rows)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "modelWindowRowsValidationError")]
pub fn model_window_rows_validation_error(rows: String) -> Result<String, JsError> {
    model_window_rows_validation_error_inner(rows).map_err(|e| JsError::new(&e))
}

/// `mergeModelWindowRows`.
fn merge_model_window_rows_inner(
    current_rows: String,
    incoming_rows: String,
) -> Result<String, String> {
    let current_rows = crate::js::from_json(&current_rows)?;
    let incoming_rows = crate::js::from_json(&incoming_rows)?;
    let out = port::merge_model_window_rows(current_rows, incoming_rows)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "mergeModelWindowRows")]
pub fn merge_model_window_rows(
    current_rows: String,
    incoming_rows: String,
) -> Result<String, JsError> {
    merge_model_window_rows_inner(current_rows, incoming_rows).map_err(|e| JsError::new(&e))
}

/// `modelWindowRowsFromProfile`.
fn model_window_rows_from_profile_inner(
    model_list: String,
    model_windows: String,
    model_vlm: Option<String>,
    model_auto_compact: Option<String>,
) -> Result<String, String> {
    let out = port::model_window_rows_from_profile(
        model_list,
        model_windows,
        model_vlm,
        model_auto_compact,
    )?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "modelWindowRowsFromProfile")]
pub fn model_window_rows_from_profile(
    model_list: String,
    model_windows: String,
    model_vlm: Option<String>,
    model_auto_compact: Option<String>,
) -> Result<String, JsError> {
    model_window_rows_from_profile_inner(model_list, model_windows, model_vlm, model_auto_compact)
        .map_err(|e| JsError::new(&e))
}

/// `serializeModelWindowRows`.
fn serialize_model_window_rows_inner(rows: String) -> Result<String, String> {
    let rows = crate::js::from_json(&rows)?;
    let out = port::serialize_model_window_rows(rows)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "serializeModelWindowRows")]
pub fn serialize_model_window_rows(rows: String) -> Result<String, JsError> {
    serialize_model_window_rows_inner(rows).map_err(|e| JsError::new(&e))
}

/// `buildModelWindows`.
fn build_model_windows_inner(
    model_list: String,
    model_windows_text: String,
) -> Result<String, String> {
    let out = port::build_model_windows(model_list, model_windows_text)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "buildModelWindows")]
pub fn build_model_windows(
    model_list: String,
    model_windows_text: String,
) -> Result<String, JsError> {
    build_model_windows_inner(model_list, model_windows_text).map_err(|e| JsError::new(&e))
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

static S_REORDER_MODEL_WINDOW_ROWS_0: wire::Schema = wire::Schema::Dyn;
static S_REORDER_MODEL_WINDOW_ROWS_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("model", wire::Schema::Str16),
    ("window", wire::Schema::Str16),
    ("autoCompact", wire::Schema::Str16),
    ("imageHandling", wire::Schema::Str16),
]));

/// `reorderModelWindowRows` over the binary wire.
#[wasm_bindgen(js_name = "reorderModelWindowRowsWire")]
pub fn reorder_model_window_rows_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let rows: Value = wire::read_value(r, &S_REORDER_MODEL_WINDOW_ROWS_0)?;
        let active_index: f64 = r.f64()?;
        let over_index: f64 = r.f64()?;
        let out = &port::reorder_model_window_rows(rows, active_index, over_index)?;
        wire::write_value(w, out, &S_REORDER_MODEL_WINDOW_ROWS_OUT)?;
        Ok(())
    })
}

static S_LOOKUP_MODEL_MAP_ENTRY_0: wire::Schema = wire::Schema::Dyn;
static S_LOOKUP_MODEL_MAP_ENTRY_OUT: wire::Schema = wire::Schema::Dyn;

/// `lookupModelMapEntry` over the binary wire.
#[wasm_bindgen(js_name = "lookupModelMapEntryWire")]
pub fn lookup_model_map_entry_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let map: Value = wire::read_value(r, &S_LOOKUP_MODEL_MAP_ENTRY_0)?;
        let model_name: String = r.string16()?;
        let out = &port::lookup_model_map_entry(map, model_name)?;
        wire::write_value(w, out, &S_LOOKUP_MODEL_MAP_ENTRY_OUT)?;
        Ok(())
    })
}

static S_MODEL_WINDOW_ROWS_VALIDATION_ERROR_0: wire::Schema =
    wire::Schema::List(&wire::Schema::Obj(&[
        ("model", wire::Schema::Str16),
        ("window", wire::Schema::Str16),
        ("autoCompact", wire::Schema::Str16),
        ("imageHandling", wire::Schema::Str16),
    ]));
static S_MODEL_WINDOW_ROWS_VALIDATION_ERROR_OUT: wire::Schema =
    wire::Schema::OptJs(&wire::Schema::Obj(&[
        ("code", wire::Schema::Str16),
        ("model", wire::Schema::Str16),
    ]));

/// `modelWindowRowsValidationError` over the binary wire.
#[wasm_bindgen(js_name = "modelWindowRowsValidationErrorWire")]
pub fn model_window_rows_validation_error_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let rows: Vec<Value> =
            r.list(|r| wire::read_value(r, S_MODEL_WINDOW_ROWS_VALIDATION_ERROR_0.inner()))?;
        let out = &port::model_window_rows_validation_error(rows)?;
        wire::write_value(w, out, &S_MODEL_WINDOW_ROWS_VALIDATION_ERROR_OUT)?;
        Ok(())
    })
}

static S_MERGE_MODEL_WINDOW_ROWS_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("model", wire::Schema::Str16),
    ("window", wire::Schema::Str16),
    ("autoCompact", wire::Schema::Str16),
    ("imageHandling", wire::Schema::Str16),
]));
static S_MERGE_MODEL_WINDOW_ROWS_1: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("model", wire::Schema::Str16),
    ("window", wire::Schema::Str16),
    ("autoCompact", wire::Schema::Str16),
    ("imageHandling", wire::Schema::Str16),
]));
static S_MERGE_MODEL_WINDOW_ROWS_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("model", wire::Schema::Str16),
    ("window", wire::Schema::Str16),
    ("autoCompact", wire::Schema::Str16),
    ("imageHandling", wire::Schema::Str16),
]));

/// `mergeModelWindowRows` over the binary wire.
#[wasm_bindgen(js_name = "mergeModelWindowRowsWire")]
pub fn merge_model_window_rows_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let current_rows: Vec<Value> =
            r.list(|r| wire::read_value(r, S_MERGE_MODEL_WINDOW_ROWS_0.inner()))?;
        let incoming_rows: Vec<Value> =
            r.list(|r| wire::read_value(r, S_MERGE_MODEL_WINDOW_ROWS_1.inner()))?;
        let out = &port::merge_model_window_rows(current_rows, incoming_rows)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_MERGE_MODEL_WINDOW_ROWS_OUT.inner())?;
        }
        Ok(())
    })
}

static S_MODEL_WINDOW_ROWS_FROM_PROFILE_OUT: wire::Schema =
    wire::Schema::List(&wire::Schema::Obj(&[
        ("model", wire::Schema::Str16),
        ("window", wire::Schema::Str16),
        ("autoCompact", wire::Schema::Str16),
        ("imageHandling", wire::Schema::Str16),
    ]));

/// `modelWindowRowsFromProfile` over the binary wire.
#[wasm_bindgen(js_name = "modelWindowRowsFromProfileWire")]
pub fn model_window_rows_from_profile_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let model_list: String = r.string16()?;
        let model_windows: String = r.string16()?;
        let model_vlm: Option<String> = r.opt_js(|r| r.string16())?;
        let model_auto_compact: Option<String> = r.opt_js(|r| r.string16())?;
        let out = &port::model_window_rows_from_profile(
            model_list,
            model_windows,
            model_vlm,
            model_auto_compact,
        )?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_MODEL_WINDOW_ROWS_FROM_PROFILE_OUT.inner())?;
        }
        Ok(())
    })
}

static S_SERIALIZE_MODEL_WINDOW_ROWS_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("model", wire::Schema::Str16),
    ("window", wire::Schema::Str16),
    ("autoCompact", wire::Schema::Str16),
    ("imageHandling", wire::Schema::Str16),
]));
static S_SERIALIZE_MODEL_WINDOW_ROWS_OUT: wire::Schema = wire::Schema::Obj(&[
    ("modelList", wire::Schema::Str16),
    ("modelWindows", wire::Schema::Str16),
    ("modelVlm", wire::Schema::Str16),
    ("modelAutoCompact", wire::Schema::Str16),
]);

/// `serializeModelWindowRows` over the binary wire.
#[wasm_bindgen(js_name = "serializeModelWindowRowsWire")]
pub fn serialize_model_window_rows_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let rows: Vec<Value> =
            r.list(|r| wire::read_value(r, S_SERIALIZE_MODEL_WINDOW_ROWS_0.inner()))?;
        let out = &port::serialize_model_window_rows(rows)?;
        wire::write_value(w, out, &S_SERIALIZE_MODEL_WINDOW_ROWS_OUT)?;
        Ok(())
    })
}

static S_BUILD_MODEL_WINDOWS_OUT: wire::Schema = wire::Schema::Dyn;

/// `buildModelWindows` over the binary wire.
#[wasm_bindgen(js_name = "buildModelWindowsWire")]
pub fn build_model_windows_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let model_list: String = r.string16()?;
        let model_windows_text: String = r.string16()?;
        let out = &port::build_model_windows(model_list, model_windows_text)?;
        wire::write_value(w, out, &S_BUILD_MODEL_WINDOWS_OUT)?;
        Ok(())
    })
}
