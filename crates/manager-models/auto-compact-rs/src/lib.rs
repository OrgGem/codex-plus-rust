//! JS bindings for `src/auto-compact.ts`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod port;

/// `autoCompactOptions`.
fn auto_compact_options_inner(current: String) -> Result<Vec<String>, String> {
    let out = port::auto_compact_options(current)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "autoCompactOptions")]
pub fn auto_compact_options(current: String) -> Result<Vec<String>, JsError> {
    auto_compact_options_inner(current).map_err(|e| JsError::new(&e))
}

/// `isValidAutoCompactPercent`.
fn is_valid_auto_compact_percent_inner(value: String) -> Result<bool, String> {
    let out = port::is_valid_auto_compact_percent(value)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "isValidAutoCompactPercent")]
pub fn is_valid_auto_compact_percent(value: String) -> Result<bool, JsError> {
    is_valid_auto_compact_percent_inner(value).map_err(|e| JsError::new(&e))
}

/// `normalizeAutoCompactPercent`.
fn normalize_auto_compact_percent_inner(value: String) -> Result<String, String> {
    let out = port::normalize_auto_compact_percent(value)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "normalizeAutoCompactPercent")]
pub fn normalize_auto_compact_percent(value: String) -> Result<String, JsError> {
    normalize_auto_compact_percent_inner(value).map_err(|e| JsError::new(&e))
}

/// `normalizeAutoCompactEditing`.
fn normalize_auto_compact_editing_inner(
    value: String,
    previous_value: Option<String>,
) -> Result<String, String> {
    let out = port::normalize_auto_compact_editing(value, previous_value)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "normalizeAutoCompactEditing")]
pub fn normalize_auto_compact_editing(
    value: String,
    previous_value: Option<String>,
) -> Result<String, JsError> {
    normalize_auto_compact_editing_inner(value, previous_value).map_err(|e| JsError::new(&e))
}
