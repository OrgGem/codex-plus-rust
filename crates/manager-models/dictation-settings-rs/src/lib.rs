//! JS bindings for `src/dictation-settings.ts`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;

/// `defaultDictationSettings`.
fn default_dictation_settings_inner() -> Result<String, String> {
    let out = port::default_dictation_settings()?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "defaultDictationSettings")]
pub fn default_dictation_settings() -> Result<String, JsError> {
    default_dictation_settings_inner().map_err(|e| JsError::new(&e))
}

/// `normalizeDictationSettings`.
fn normalize_dictation_settings_inner(value: String) -> Result<String, String> {
    let value = crate::js::from_json(&value)?;
    let out = port::normalize_dictation_settings(value)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "normalizeDictationSettings")]
pub fn normalize_dictation_settings(value: String) -> Result<String, JsError> {
    normalize_dictation_settings_inner(value).map_err(|e| JsError::new(&e))
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

static S_DEFAULT_DICTATION_SETTINGS_OUT: wire::Schema = wire::Schema::Obj(&[
    ("enabled", wire::Schema::Bool),
    ("provider", wire::Schema::Str16),
    ("baseUrl", wire::Schema::Str16),
    ("apiKey", wire::Schema::Str16),
    ("apiKeyEnv", wire::Schema::Str16),
    ("model", wire::Schema::Str16),
    ("language", wire::Schema::Str16),
    ("timeoutSeconds", wire::Schema::Num),
]);

/// `defaultDictationSettings` over the binary wire.
#[wasm_bindgen(js_name = "defaultDictationSettingsWire")]
pub fn default_dictation_settings_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |_r, w| {
        let out = &port::default_dictation_settings()?;
        wire::write_value(w, out, &S_DEFAULT_DICTATION_SETTINGS_OUT)?;
        Ok(())
    })
}

static S_NORMALIZE_DICTATION_SETTINGS_OUT: wire::Schema = wire::Schema::Obj(&[
    ("enabled", wire::Schema::Bool),
    ("provider", wire::Schema::Str16),
    ("baseUrl", wire::Schema::Str16),
    ("apiKey", wire::Schema::Str16),
    ("apiKeyEnv", wire::Schema::Str16),
    ("model", wire::Schema::Str16),
    ("language", wire::Schema::Str16),
    ("timeoutSeconds", wire::Schema::Num),
]);

/// `normalizeDictationSettings` over the binary wire.
#[wasm_bindgen(js_name = "normalizeDictationSettingsWire")]
pub fn normalize_dictation_settings_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let value: Option<serde_json::Value> = wire::read_dyn_opt(r)?;
        let out = &port::normalize_dictation_settings(value)?;
        wire::write_value(w, out, &S_NORMALIZE_DICTATION_SETTINGS_OUT)?;
        Ok(())
    })
}
