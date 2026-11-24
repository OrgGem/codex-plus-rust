//! JS bindings for `src/notice-model.ts`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod port;

/// `noticeTone`.
fn notice_tone_inner(status: Option<String>) -> Result<String, String> {
    let out = port::notice_tone(status)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "noticeTone")]
pub fn notice_tone(status: Option<String>) -> Result<String, JsError> {
    notice_tone_inner(status).map_err(|e| JsError::new(&e))
}

/// `noticeDuration`.
fn notice_duration_inner(tone: String, message: String) -> Result<f64, String> {
    let out = port::notice_duration(tone, message)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "noticeDuration")]
pub fn notice_duration(tone: String, message: String) -> Result<f64, JsError> {
    notice_duration_inner(tone, message).map_err(|e| JsError::new(&e))
}
