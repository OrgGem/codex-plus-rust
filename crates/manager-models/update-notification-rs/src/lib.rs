//! JS bindings for `src/update-notification.ts`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;
pub mod utf16;

/// `updatePreferencesFrom`.
fn update_preferences_from_inner(settings: String) -> Result<String, String> {
    let settings = crate::js::from_json(&settings)?;
    let out = port::update_preferences_from(settings)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "updatePreferencesFrom")]
pub fn update_preferences_from(settings: String) -> Result<String, JsError> {
    update_preferences_from_inner(settings).map_err(|e| JsError::new(&e))
}

/// `isVersionSkipped`.
fn is_version_skipped_inner(candidate: String, skipped: String) -> Result<bool, String> {
    let out = port::is_version_skipped(candidate, skipped)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "isVersionSkipped")]
pub fn is_version_skipped(candidate: String, skipped: String) -> Result<bool, JsError> {
    is_version_skipped_inner(candidate, skipped).map_err(|e| JsError::new(&e))
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

static S_UPDATE_PREFERENCES_FROM_OUT: wire::Schema = wire::Schema::Obj(&[
    ("autoCheck", wire::Schema::Bool),
    ("skippedVersion", wire::Schema::Str16),
    ("autoDownload", wire::Schema::Bool),
]);

/// `updatePreferencesFrom` over the binary wire.
#[wasm_bindgen(js_name = "updatePreferencesFromWire")]
pub fn update_preferences_from_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let settings: Option<serde_json::Value> = wire::read_dyn_opt(r)?;
        let out = &port::update_preferences_from(settings)?;
        wire::write_value(w, out, &S_UPDATE_PREFERENCES_FROM_OUT)?;
        Ok(())
    })
}
