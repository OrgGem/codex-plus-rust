//! JS bindings for `src/settings-rebase.ts`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;

/// `rebaseSettingsEdits`.
fn rebase_settings_edits_inner(
    form: String,
    previous: String,
    latest: String,
) -> Result<String, String> {
    let form = crate::js::from_json(&form)?;
    let previous = crate::js::from_json(&previous)?;
    let latest = crate::js::from_json(&latest)?;
    let out = port::rebase_settings_edits(form, previous, latest)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "rebaseSettingsEdits")]
pub fn rebase_settings_edits(
    form: String,
    previous: String,
    latest: String,
) -> Result<String, JsError> {
    rebase_settings_edits_inner(form, previous, latest).map_err(|e| JsError::new(&e))
}

/// `mergeRotatedAuthContents`.
fn merge_rotated_auth_contents_inner(
    merged: String,
    previous: String,
    latest: String,
) -> Result<String, String> {
    let merged = crate::js::from_json(&merged)?;
    let previous = crate::js::from_json(&previous)?;
    let latest = crate::js::from_json(&latest)?;
    let out = port::merge_rotated_auth_contents(merged, previous, latest)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "mergeRotatedAuthContents")]
pub fn merge_rotated_auth_contents(
    merged: String,
    previous: String,
    latest: String,
) -> Result<String, JsError> {
    merge_rotated_auth_contents_inner(merged, previous, latest).map_err(|e| JsError::new(&e))
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

static S_REBASE_SETTINGS_EDITS_0: wire::Schema = wire::Schema::Dyn;
static S_REBASE_SETTINGS_EDITS_1: wire::Schema = wire::Schema::Dyn;
static S_REBASE_SETTINGS_EDITS_2: wire::Schema = wire::Schema::Dyn;
static S_REBASE_SETTINGS_EDITS_OUT: wire::Schema = wire::Schema::Dyn;

/// `rebaseSettingsEdits` over the binary wire.
#[wasm_bindgen(js_name = "rebaseSettingsEditsWire")]
pub fn rebase_settings_edits_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let form: serde_json::Value = wire::read_value(r, &S_REBASE_SETTINGS_EDITS_0)?;
        let previous: serde_json::Value = wire::read_value(r, &S_REBASE_SETTINGS_EDITS_1)?;
        let latest: serde_json::Value = wire::read_value(r, &S_REBASE_SETTINGS_EDITS_2)?;
        let out = &port::rebase_settings_edits(form, previous, latest)?;
        wire::write_value(w, out, &S_REBASE_SETTINGS_EDITS_OUT)?;
        Ok(())
    })
}

static S_MERGE_ROTATED_AUTH_CONTENTS_0: wire::Schema = wire::Schema::Dyn;
static S_MERGE_ROTATED_AUTH_CONTENTS_1: wire::Schema = wire::Schema::Dyn;
static S_MERGE_ROTATED_AUTH_CONTENTS_2: wire::Schema = wire::Schema::Dyn;
static S_MERGE_ROTATED_AUTH_CONTENTS_OUT: wire::Schema = wire::Schema::Dyn;

/// `mergeRotatedAuthContents` over the binary wire.
#[wasm_bindgen(js_name = "mergeRotatedAuthContentsWire")]
pub fn merge_rotated_auth_contents_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let merged: serde_json::Value = wire::read_value(r, &S_MERGE_ROTATED_AUTH_CONTENTS_0)?;
        let previous: serde_json::Value = wire::read_value(r, &S_MERGE_ROTATED_AUTH_CONTENTS_1)?;
        let latest: serde_json::Value = wire::read_value(r, &S_MERGE_ROTATED_AUTH_CONTENTS_2)?;
        let out = &port::merge_rotated_auth_contents(merged, previous, latest)?;
        wire::write_value(w, out, &S_MERGE_ROTATED_AUTH_CONTENTS_OUT)?;
        Ok(())
    })
}
