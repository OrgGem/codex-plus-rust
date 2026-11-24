//! JS bindings for `external-api.mjs`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;
pub mod utf16;

/// `apiEndpoint`.
fn api_endpoint_inner(raw: String) -> Result<String, String> {
    let raw = crate::js::from_json(&raw)?;
    let out = port::api_endpoint(raw)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "apiEndpoint")]
pub fn api_endpoint(raw: String) -> Result<String, JsError> {
    api_endpoint_inner(raw).map_err(|e| JsError::new(&e))
}

/// `storedApiConfig`.
fn stored_api_config_inner(config: String) -> Result<String, String> {
    let config = crate::js::from_json(&config)?;
    let out = port::stored_api_config(config)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "storedApiConfig")]
pub fn stored_api_config(config: String) -> Result<String, JsError> {
    stored_api_config_inner(config).map_err(|e| JsError::new(&e))
}

/// `apiRequestBody`.
fn api_request_body_inner(config: String, prompt: String) -> Result<String, String> {
    let config = crate::js::from_json(&config)?;
    let prompt = crate::js::from_json(&prompt)?;
    let out = port::api_request_body(config, prompt)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "apiRequestBody")]
pub fn api_request_body(config: String, prompt: String) -> Result<String, JsError> {
    api_request_body_inner(config, prompt).map_err(|e| JsError::new(&e))
}

/// `completionText`.
fn completion_text_inner(body: String) -> Result<String, String> {
    let body = crate::js::from_json(&body)?;
    let out = port::completion_text(body)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "completionText")]
pub fn completion_text(body: String) -> Result<String, JsError> {
    completion_text_inner(body).map_err(|e| JsError::new(&e))
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

static S_API_ENDPOINT_0: wire::Schema = wire::Schema::Dyn;
static S_API_ENDPOINT_OUT: wire::Schema = wire::Schema::Dyn;

/// `apiEndpoint` over the binary wire.
#[wasm_bindgen(js_name = "apiEndpointWire")]
pub fn api_endpoint_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let raw: serde_json::Value = wire::read_value(r, &S_API_ENDPOINT_0)?;
        let out = &port::api_endpoint(raw)?;
        wire::write_value(w, out, &S_API_ENDPOINT_OUT)?;
        Ok(())
    })
}

static S_STORED_API_CONFIG_0: wire::Schema = wire::Schema::Dyn;
static S_STORED_API_CONFIG_OUT: wire::Schema = wire::Schema::Dyn;

/// `storedApiConfig` over the binary wire.
#[wasm_bindgen(js_name = "storedApiConfigWire")]
pub fn stored_api_config_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let config: serde_json::Value = wire::read_value(r, &S_STORED_API_CONFIG_0)?;
        let out = &port::stored_api_config(config)?;
        wire::write_value(w, out, &S_STORED_API_CONFIG_OUT)?;
        Ok(())
    })
}

static S_API_REQUEST_BODY_0: wire::Schema = wire::Schema::Dyn;
static S_API_REQUEST_BODY_1: wire::Schema = wire::Schema::Dyn;
static S_API_REQUEST_BODY_OUT: wire::Schema = wire::Schema::Dyn;

/// `apiRequestBody` over the binary wire.
#[wasm_bindgen(js_name = "apiRequestBodyWire")]
pub fn api_request_body_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let config: serde_json::Value = wire::read_value(r, &S_API_REQUEST_BODY_0)?;
        let prompt: serde_json::Value = wire::read_value(r, &S_API_REQUEST_BODY_1)?;
        let out = &port::api_request_body(config, prompt)?;
        wire::write_value(w, out, &S_API_REQUEST_BODY_OUT)?;
        Ok(())
    })
}

static S_COMPLETION_TEXT_0: wire::Schema = wire::Schema::Dyn;
static S_COMPLETION_TEXT_OUT: wire::Schema = wire::Schema::Dyn;

/// `completionText` over the binary wire.
#[wasm_bindgen(js_name = "completionTextWire")]
pub fn completion_text_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let body: serde_json::Value = wire::read_value(r, &S_COMPLETION_TEXT_0)?;
        let out = &port::completion_text(body)?;
        wire::write_value(w, out, &S_COMPLETION_TEXT_OUT)?;
        Ok(())
    })
}
