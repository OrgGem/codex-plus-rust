//! JS bindings for `src/theme-mode.ts`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod port;

/// `parseThemePreference`.
fn parse_theme_preference_inner(raw: Option<String>) -> Result<String, String> {
    let out = port::parse_theme_preference(raw)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "parseThemePreference")]
pub fn parse_theme_preference(raw: Option<String>) -> Result<String, JsError> {
    parse_theme_preference_inner(raw).map_err(|e| JsError::new(&e))
}

/// `resolveTheme`.
fn resolve_theme_inner(preference: String, system_prefers_dark: bool) -> Result<String, String> {
    let out = port::resolve_theme(preference, system_prefers_dark)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "resolveTheme")]
pub fn resolve_theme(preference: String, system_prefers_dark: bool) -> Result<String, JsError> {
    resolve_theme_inner(preference, system_prefers_dark).map_err(|e| JsError::new(&e))
}

/// `toggledThemePreference`.
fn toggled_theme_preference_inner(current: String) -> Result<String, String> {
    let out = port::toggled_theme_preference(current)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "toggledThemePreference")]
pub fn toggled_theme_preference(current: String) -> Result<String, JsError> {
    toggled_theme_preference_inner(current).map_err(|e| JsError::new(&e))
}
