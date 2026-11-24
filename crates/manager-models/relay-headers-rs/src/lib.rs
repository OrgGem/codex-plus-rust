//! JS bindings for `src/relay-headers.ts`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;

/// `isForbiddenRelayHeader`.
fn is_forbidden_relay_header_inner(name: String) -> Result<bool, String> {
    let out = port::is_forbidden_relay_header(name)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "isForbiddenRelayHeader")]
pub fn is_forbidden_relay_header(name: String) -> Result<bool, JsError> {
    is_forbidden_relay_header_inner(name).map_err(|e| JsError::new(&e))
}

/// `normalizeRelayHeaders`.
fn normalize_relay_headers_inner(rows: String) -> Result<String, String> {
    let rows = crate::js::from_json(&rows)?;
    let out = port::normalize_relay_headers(rows)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "normalizeRelayHeaders")]
pub fn normalize_relay_headers(rows: String) -> Result<String, JsError> {
    normalize_relay_headers_inner(rows).map_err(|e| JsError::new(&e))
}

/// `relayHeadersValidationMessage`.
fn relay_headers_validation_message_inner(rows: String) -> Result<Option<String>, String> {
    let rows = crate::js::from_json(&rows)?;
    let out = port::relay_headers_validation_message(rows)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "relayHeadersValidationMessage")]
pub fn relay_headers_validation_message(rows: String) -> Result<Option<String>, JsError> {
    relay_headers_validation_message_inner(rows).map_err(|e| JsError::new(&e))
}

/// `serializeRelayHeaders`.
fn serialize_relay_headers_inner(rows: String) -> Result<String, String> {
    let rows = crate::js::from_json(&rows)?;
    let out = port::serialize_relay_headers(rows)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "serializeRelayHeaders")]
pub fn serialize_relay_headers(rows: String) -> Result<String, JsError> {
    serialize_relay_headers_inner(rows).map_err(|e| JsError::new(&e))
}

/// `hasCustomAuthorization`.
fn has_custom_authorization_inner(rows: String) -> Result<bool, String> {
    let rows = crate::js::from_json(&rows)?;
    let out = port::has_custom_authorization(rows)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "hasCustomAuthorization")]
pub fn has_custom_authorization(rows: String) -> Result<bool, JsError> {
    has_custom_authorization_inner(rows).map_err(|e| JsError::new(&e))
}

/// `providerHeaderRows`.
fn provider_header_rows_inner(headers: String) -> Result<String, String> {
    let headers = crate::js::from_json(&headers)?;
    let out = port::provider_header_rows(headers)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "providerHeaderRows")]
pub fn provider_header_rows(headers: String) -> Result<String, JsError> {
    provider_header_rows_inner(headers).map_err(|e| JsError::new(&e))
}

/// `providerHeaderInputs`.
fn provider_header_inputs_inner(rows: String) -> Result<String, String> {
    let rows = crate::js::from_json(&rows)?;
    let out = port::provider_header_inputs(rows)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "providerHeaderInputs")]
pub fn provider_header_inputs(rows: String) -> Result<String, JsError> {
    provider_header_inputs_inner(rows).map_err(|e| JsError::new(&e))
}

/// `providerHeadersValidationMessage`.
fn provider_headers_validation_message_inner(rows: String) -> Result<Option<String>, String> {
    let rows = crate::js::from_json(&rows)?;
    let out = port::provider_headers_validation_message(rows)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "providerHeadersValidationMessage")]
pub fn provider_headers_validation_message(rows: String) -> Result<Option<String>, JsError> {
    provider_headers_validation_message_inner(rows).map_err(|e| JsError::new(&e))
}

/// `providerHeadersSummary`.
fn provider_headers_summary_inner(headers: String) -> Result<String, String> {
    let headers = crate::js::from_json(&headers)?;
    let out = port::provider_headers_summary(headers)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "providerHeadersSummary")]
pub fn provider_headers_summary(headers: String) -> Result<String, JsError> {
    provider_headers_summary_inner(headers).map_err(|e| JsError::new(&e))
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

static S_NORMALIZE_RELAY_HEADERS_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("key", wire::Schema::Str16),
    ("value", wire::Schema::Str16),
]));
static S_NORMALIZE_RELAY_HEADERS_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("key", wire::Schema::Str16),
    ("value", wire::Schema::Str16),
]));

/// `normalizeRelayHeaders` over the binary wire.
#[wasm_bindgen(js_name = "normalizeRelayHeadersWire")]
pub fn normalize_relay_headers_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let rows: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_NORMALIZE_RELAY_HEADERS_0.inner()))?;
        let out = &port::normalize_relay_headers(rows)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_NORMALIZE_RELAY_HEADERS_OUT.inner())?;
        }
        Ok(())
    })
}

static S_RELAY_HEADERS_VALIDATION_MESSAGE_0: wire::Schema =
    wire::Schema::List(&wire::Schema::Obj(&[
        ("key", wire::Schema::Str16),
        ("value", wire::Schema::Str16),
    ]));

/// `relayHeadersValidationMessage` over the binary wire.
#[wasm_bindgen(js_name = "relayHeadersValidationMessageWire")]
pub fn relay_headers_validation_message_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let rows: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_RELAY_HEADERS_VALIDATION_MESSAGE_0.inner()))?;
        let out = &port::relay_headers_validation_message(rows)?;
        match out {
            Some(x0) => {
                w.tag(1);
                w.string16(x0);
            }
            None => w.tag(2),
        }
        Ok(())
    })
}

static S_SERIALIZE_RELAY_HEADERS_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("key", wire::Schema::Str16),
    ("value", wire::Schema::Str16),
]));
static S_SERIALIZE_RELAY_HEADERS_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("key", wire::Schema::Str16),
    ("value", wire::Schema::Str16),
]));

/// `serializeRelayHeaders` over the binary wire.
#[wasm_bindgen(js_name = "serializeRelayHeadersWire")]
pub fn serialize_relay_headers_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let rows: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_SERIALIZE_RELAY_HEADERS_0.inner()))?;
        let out = &port::serialize_relay_headers(rows)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_SERIALIZE_RELAY_HEADERS_OUT.inner())?;
        }
        Ok(())
    })
}

static S_HAS_CUSTOM_AUTHORIZATION_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("key", wire::Schema::Str16),
    ("value", wire::Schema::Str16),
]));

/// `hasCustomAuthorization` over the binary wire.
#[wasm_bindgen(js_name = "hasCustomAuthorizationWire")]
pub fn has_custom_authorization_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let rows: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_HAS_CUSTOM_AUTHORIZATION_0.inner()))?;
        let out = &port::has_custom_authorization(rows)?;
        w.bool(*out);
        Ok(())
    })
}

static S_PROVIDER_HEADER_ROWS_0: wire::Schema =
    wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Obj(&[
        ("key", wire::Schema::Str16),
        ("value", wire::Schema::Str16),
        ("masked", wire::Schema::OptJs(&wire::Schema::Str16)),
    ])));
static S_PROVIDER_HEADER_ROWS_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("key", wire::Schema::Str16),
    ("value", wire::Schema::Str16),
    ("masked", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("savedKey", wire::Schema::OptJs(&wire::Schema::Str16)),
]));

/// `providerHeaderRows` over the binary wire.
#[wasm_bindgen(js_name = "providerHeaderRowsWire")]
pub fn provider_header_rows_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let headers: Option<Vec<serde_json::Value>> = r.opt_js(|r| {
            r.list(|r| wire::read_value(r, S_PROVIDER_HEADER_ROWS_0.inner().inner()))
        })?;
        let out = &port::provider_header_rows(headers)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_PROVIDER_HEADER_ROWS_OUT.inner())?;
        }
        Ok(())
    })
}

static S_PROVIDER_HEADER_INPUTS_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("key", wire::Schema::Str16),
    ("value", wire::Schema::Str16),
    ("masked", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("savedKey", wire::Schema::OptJs(&wire::Schema::Str16)),
]));
static S_PROVIDER_HEADER_INPUTS_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("key", wire::Schema::Str16),
    ("value", wire::Schema::Str16),
    ("keep", wire::Schema::Bool),
]));

/// `providerHeaderInputs` over the binary wire.
#[wasm_bindgen(js_name = "providerHeaderInputsWire")]
pub fn provider_header_inputs_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let rows: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_PROVIDER_HEADER_INPUTS_0.inner()))?;
        let out = &port::provider_header_inputs(rows)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_PROVIDER_HEADER_INPUTS_OUT.inner())?;
        }
        Ok(())
    })
}

static S_PROVIDER_HEADERS_VALIDATION_MESSAGE_0: wire::Schema =
    wire::Schema::List(&wire::Schema::Obj(&[
        ("key", wire::Schema::Str16),
        ("value", wire::Schema::Str16),
        ("masked", wire::Schema::OptJs(&wire::Schema::Str16)),
        ("savedKey", wire::Schema::OptJs(&wire::Schema::Str16)),
    ]));

/// `providerHeadersValidationMessage` over the binary wire.
#[wasm_bindgen(js_name = "providerHeadersValidationMessageWire")]
pub fn provider_headers_validation_message_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let rows: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_PROVIDER_HEADERS_VALIDATION_MESSAGE_0.inner()))?;
        let out = &port::provider_headers_validation_message(rows)?;
        match out {
            Some(x0) => {
                w.tag(1);
                w.string16(x0);
            }
            None => w.tag(2),
        }
        Ok(())
    })
}

static S_PROVIDER_HEADERS_SUMMARY_0: wire::Schema =
    wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Obj(&[
        ("key", wire::Schema::Str16),
        ("value", wire::Schema::Str16),
        ("masked", wire::Schema::OptJs(&wire::Schema::Str16)),
    ])));

/// `providerHeadersSummary` over the binary wire.
#[wasm_bindgen(js_name = "providerHeadersSummaryWire")]
pub fn provider_headers_summary_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let headers: Option<Vec<serde_json::Value>> = r.opt_js(|r| {
            r.list(|r| wire::read_value(r, S_PROVIDER_HEADERS_SUMMARY_0.inner().inner()))
        })?;
        let out = &port::provider_headers_summary(headers)?;
        w.string16(out);
        Ok(())
    })
}
