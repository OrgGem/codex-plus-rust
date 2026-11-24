//! JS bindings for `src/relay-session.ts`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod port;

/// `sessionProviderForProtocol`.
fn session_provider_for_protocol_inner(
    session_provider: String,
    protocol: String,
) -> Result<String, String> {
    let out = port::session_provider_for_protocol(session_provider, protocol)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "sessionProviderForProtocol")]
pub fn session_provider_for_protocol(
    session_provider: String,
    protocol: String,
) -> Result<String, JsError> {
    session_provider_for_protocol_inner(session_provider, protocol).map_err(|e| JsError::new(&e))
}

/// `openAiSessionBlocksModelSave`.
fn open_ai_session_blocks_model_save_inner(
    session_provider: String,
    protocol: String,
) -> Result<bool, String> {
    let out = port::open_ai_session_blocks_model_save(session_provider, protocol)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "openAiSessionBlocksModelSave")]
pub fn open_ai_session_blocks_model_save(
    session_provider: String,
    protocol: String,
) -> Result<bool, JsError> {
    open_ai_session_blocks_model_save_inner(session_provider, protocol)
        .map_err(|e| JsError::new(&e))
}
