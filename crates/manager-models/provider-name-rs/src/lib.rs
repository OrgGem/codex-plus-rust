//! JS bindings for `src/provider-name.ts`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod port;
pub mod utf16;

/// `tomlSectionStringValue`.
fn toml_section_string_value_inner(
    contents: String,
    section_name: String,
    key: String,
) -> Result<String, String> {
    let out = port::toml_section_string_value(contents, section_name, key)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "tomlSectionStringValue")]
pub fn toml_section_string_value(
    contents: String,
    section_name: String,
    key: String,
) -> Result<String, JsError> {
    toml_section_string_value_inner(contents, section_name, key).map_err(|e| JsError::new(&e))
}

/// `resolveProviderName`.
fn resolve_provider_name_inner(contents: String, provider: String) -> Result<String, String> {
    let out = port::resolve_provider_name(contents, provider)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "resolveProviderName")]
pub fn resolve_provider_name(contents: String, provider: String) -> Result<String, JsError> {
    resolve_provider_name_inner(contents, provider).map_err(|e| JsError::new(&e))
}

/// `providerAvatarText`.
fn provider_avatar_text_inner(name: String, fallback: Option<String>) -> Result<String, String> {
    let out = port::provider_avatar_text(name, fallback)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "providerAvatarText")]
pub fn provider_avatar_text(name: String, fallback: Option<String>) -> Result<String, JsError> {
    provider_avatar_text_inner(name, fallback).map_err(|e| JsError::new(&e))
}
