//! JS bindings for `organize.mjs`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;
pub mod utf16;

/// `organizationPrompt`.
fn organization_prompt_inner(messages: String, request_id: String) -> Result<String, String> {
    let messages = crate::js::from_json(&messages)?;
    let request_id = crate::js::from_json(&request_id)?;
    let out = port::organization_prompt(messages, request_id)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "organizationPrompt")]
pub fn organization_prompt(messages: String, request_id: String) -> Result<String, JsError> {
    organization_prompt_inner(messages, request_id).map_err(|e| JsError::new(&e))
}

/// `validateOrganization`.
fn validate_organization_inner(
    value: String,
    messages: String,
    request_id: String,
) -> Result<String, String> {
    let value = crate::js::from_json(&value)?;
    let messages = crate::js::from_json(&messages)?;
    let request_id = crate::js::from_json(&request_id)?;
    let out = port::validate_organization(value, messages, request_id)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "validateOrganization")]
pub fn validate_organization(
    value: String,
    messages: String,
    request_id: String,
) -> Result<String, JsError> {
    validate_organization_inner(value, messages, request_id).map_err(|e| JsError::new(&e))
}

/// `parseOrganization`.
fn parse_organization_inner(
    text: String,
    messages: String,
    request_id: String,
) -> Result<String, String> {
    let text = crate::js::from_json(&text)?;
    let messages = crate::js::from_json(&messages)?;
    let request_id = crate::js::from_json(&request_id)?;
    let out = port::parse_organization(text, messages, request_id)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "parseOrganization")]
pub fn parse_organization(
    text: String,
    messages: String,
    request_id: String,
) -> Result<String, JsError> {
    parse_organization_inner(text, messages, request_id).map_err(|e| JsError::new(&e))
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

static S_ORGANIZATION_PROMPT_0: wire::Schema = wire::Schema::Dyn;
static S_ORGANIZATION_PROMPT_1: wire::Schema = wire::Schema::Dyn;
static S_ORGANIZATION_PROMPT_OUT: wire::Schema = wire::Schema::Dyn;

/// `organizationPrompt` over the binary wire.
#[wasm_bindgen(js_name = "organizationPromptWire")]
pub fn organization_prompt_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let messages: serde_json::Value = wire::read_value(r, &S_ORGANIZATION_PROMPT_0)?;
        let request_id: serde_json::Value = wire::read_value(r, &S_ORGANIZATION_PROMPT_1)?;
        let out = &port::organization_prompt(messages, request_id)?;
        wire::write_value(w, out, &S_ORGANIZATION_PROMPT_OUT)?;
        Ok(())
    })
}

static S_VALIDATE_ORGANIZATION_0: wire::Schema = wire::Schema::Dyn;
static S_VALIDATE_ORGANIZATION_1: wire::Schema = wire::Schema::Dyn;
static S_VALIDATE_ORGANIZATION_2: wire::Schema = wire::Schema::Dyn;
static S_VALIDATE_ORGANIZATION_OUT: wire::Schema = wire::Schema::Dyn;

/// `validateOrganization` over the binary wire.
#[wasm_bindgen(js_name = "validateOrganizationWire")]
pub fn validate_organization_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let value: serde_json::Value = wire::read_value(r, &S_VALIDATE_ORGANIZATION_0)?;
        let messages: serde_json::Value = wire::read_value(r, &S_VALIDATE_ORGANIZATION_1)?;
        let request_id: serde_json::Value = wire::read_value(r, &S_VALIDATE_ORGANIZATION_2)?;
        let out = &port::validate_organization(value, messages, request_id)?;
        wire::write_value(w, out, &S_VALIDATE_ORGANIZATION_OUT)?;
        Ok(())
    })
}

static S_PARSE_ORGANIZATION_0: wire::Schema = wire::Schema::Dyn;
static S_PARSE_ORGANIZATION_1: wire::Schema = wire::Schema::Dyn;
static S_PARSE_ORGANIZATION_2: wire::Schema = wire::Schema::Dyn;
static S_PARSE_ORGANIZATION_OUT: wire::Schema = wire::Schema::Dyn;

/// `parseOrganization` over the binary wire.
#[wasm_bindgen(js_name = "parseOrganizationWire")]
pub fn parse_organization_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let text: serde_json::Value = wire::read_value(r, &S_PARSE_ORGANIZATION_0)?;
        let messages: serde_json::Value = wire::read_value(r, &S_PARSE_ORGANIZATION_1)?;
        let request_id: serde_json::Value = wire::read_value(r, &S_PARSE_ORGANIZATION_2)?;
        let out = &port::parse_organization(text, messages, request_id)?;
        wire::write_value(w, out, &S_PARSE_ORGANIZATION_OUT)?;
        Ok(())
    })
}
