//! JS bindings for `src/provider-api-keys.ts`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;
pub mod utf16;

/// `normalizeRelayApiKeys`.
fn normalize_relay_api_keys_inner(profile: String) -> Result<String, String> {
    let profile = crate::js::from_json(&profile)?;
    let out = port::normalize_relay_api_keys(profile)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "normalizeRelayApiKeys")]
pub fn normalize_relay_api_keys(profile: String) -> Result<String, JsError> {
    normalize_relay_api_keys_inner(profile).map_err(|e| JsError::new(&e))
}

/// `selectedRelayApiKey`.
fn selected_relay_api_key_inner(profile: String) -> Result<String, String> {
    let profile = crate::js::from_json(&profile)?;
    let out = port::selected_relay_api_key(profile)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "selectedRelayApiKey")]
pub fn selected_relay_api_key(profile: String) -> Result<String, JsError> {
    selected_relay_api_key_inner(profile).map_err(|e| JsError::new(&e))
}

/// `relayProfileWithNormalizedApiKeys`.
fn relay_profile_with_normalized_api_keys_inner(profile: String) -> Result<String, String> {
    let profile = crate::js::from_json(&profile)?;
    let out = port::relay_profile_with_normalized_api_keys(profile)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "relayProfileWithNormalizedApiKeys")]
pub fn relay_profile_with_normalized_api_keys(profile: String) -> Result<String, JsError> {
    relay_profile_with_normalized_api_keys_inner(profile).map_err(|e| JsError::new(&e))
}

/// `relayProfileWithUpdatedApiKey`.
fn relay_profile_with_updated_api_key_inner(
    profile: String,
    api_key: String,
) -> Result<String, String> {
    let profile = crate::js::from_json(&profile)?;
    let out = port::relay_profile_with_updated_api_key(profile, api_key)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "relayProfileWithUpdatedApiKey")]
pub fn relay_profile_with_updated_api_key(
    profile: String,
    api_key: String,
) -> Result<String, JsError> {
    relay_profile_with_updated_api_key_inner(profile, api_key).map_err(|e| JsError::new(&e))
}

/// `apiKeyPreview`.
fn api_key_preview_inner(api_key: String) -> Result<String, String> {
    let out = port::api_key_preview(api_key)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "apiKeyPreview")]
pub fn api_key_preview(api_key: String) -> Result<String, JsError> {
    api_key_preview_inner(api_key).map_err(|e| JsError::new(&e))
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

static S_NORMALIZE_RELAY_API_KEYS_0: wire::Schema = wire::Schema::Dyn;
static S_NORMALIZE_RELAY_API_KEYS_OUT: wire::Schema = wire::Schema::Obj(&[
    (
        "apiKeys",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("id", wire::Schema::Str16),
            ("name", wire::Schema::Str16),
            ("apiKey", wire::Schema::Str16),
        ])),
    ),
    ("activeApiKeyId", wire::Schema::Str16),
]);

/// `normalizeRelayApiKeys` over the binary wire.
#[wasm_bindgen(js_name = "normalizeRelayApiKeysWire")]
pub fn normalize_relay_api_keys_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let profile: Value = wire::read_value(r, &S_NORMALIZE_RELAY_API_KEYS_0)?;
        let out = &port::normalize_relay_api_keys(profile)?;
        wire::write_value(w, out, &S_NORMALIZE_RELAY_API_KEYS_OUT)?;
        Ok(())
    })
}

static S_SELECTED_RELAY_API_KEY_0: wire::Schema = wire::Schema::Dyn;
static S_SELECTED_RELAY_API_KEY_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("name", wire::Schema::Str16),
    ("apiKey", wire::Schema::Str16),
]));

/// `selectedRelayApiKey` over the binary wire.
#[wasm_bindgen(js_name = "selectedRelayApiKeyWire")]
pub fn selected_relay_api_key_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let profile: Value = wire::read_value(r, &S_SELECTED_RELAY_API_KEY_0)?;
        let out = &port::selected_relay_api_key(profile)?;
        wire::write_value(w, out, &S_SELECTED_RELAY_API_KEY_OUT)?;
        Ok(())
    })
}

static S_RELAY_PROFILE_WITH_NORMALIZED_API_KEYS_0: wire::Schema = wire::Schema::Dyn;
static S_RELAY_PROFILE_WITH_NORMALIZED_API_KEYS_OUT: wire::Schema = wire::Schema::Dyn;

/// `relayProfileWithNormalizedApiKeys` over the binary wire.
#[wasm_bindgen(js_name = "relayProfileWithNormalizedApiKeysWire")]
pub fn relay_profile_with_normalized_api_keys_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let profile: Value = wire::read_value(r, &S_RELAY_PROFILE_WITH_NORMALIZED_API_KEYS_0)?;
        let out = &port::relay_profile_with_normalized_api_keys(profile)?;
        wire::write_value(w, out, &S_RELAY_PROFILE_WITH_NORMALIZED_API_KEYS_OUT)?;
        Ok(())
    })
}

static S_RELAY_PROFILE_WITH_UPDATED_API_KEY_0: wire::Schema = wire::Schema::Dyn;
static S_RELAY_PROFILE_WITH_UPDATED_API_KEY_OUT: wire::Schema = wire::Schema::Dyn;

/// `relayProfileWithUpdatedApiKey` over the binary wire.
#[wasm_bindgen(js_name = "relayProfileWithUpdatedApiKeyWire")]
pub fn relay_profile_with_updated_api_key_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let profile: Value = wire::read_value(r, &S_RELAY_PROFILE_WITH_UPDATED_API_KEY_0)?;
        let api_key: String = r.string16()?;
        let out = &port::relay_profile_with_updated_api_key(profile, api_key)?;
        wire::write_value(w, out, &S_RELAY_PROFILE_WITH_UPDATED_API_KEY_OUT)?;
        Ok(())
    })
}
