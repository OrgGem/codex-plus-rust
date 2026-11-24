//! JS bindings for `src/goals-config.ts`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;
pub mod utf16;

/// `codexGoalsFeatureValue`.
fn codex_goals_feature_value_inner(config_contents: String) -> Result<Option<bool>, String> {
    let out = port::codex_goals_feature_value(config_contents)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "codexGoalsFeatureValue")]
pub fn codex_goals_feature_value(config_contents: String) -> Result<Option<bool>, JsError> {
    codex_goals_feature_value_inner(config_contents).map_err(|e| JsError::new(&e))
}

/// `codexGoalsFeatureState`.
fn codex_goals_feature_state_inner(
    profile_config_contents: String,
    common_config_contents: String,
    use_common_config: bool,
) -> Result<String, String> {
    let out = port::codex_goals_feature_state(
        profile_config_contents,
        common_config_contents,
        use_common_config,
    )?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "codexGoalsFeatureState")]
pub fn codex_goals_feature_state(
    profile_config_contents: String,
    common_config_contents: String,
    use_common_config: bool,
) -> Result<String, JsError> {
    codex_goals_feature_state_inner(
        profile_config_contents,
        common_config_contents,
        use_common_config,
    )
    .map_err(|e| JsError::new(&e))
}

/// `setCodexGoalsFeatureInConfig`.
fn set_codex_goals_feature_in_config_inner(
    config_contents: String,
    enabled: bool,
) -> Result<String, String> {
    let out = port::set_codex_goals_feature_in_config(config_contents, enabled)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "setCodexGoalsFeatureInConfig")]
pub fn set_codex_goals_feature_in_config(
    config_contents: String,
    enabled: bool,
) -> Result<String, JsError> {
    set_codex_goals_feature_in_config_inner(config_contents, enabled).map_err(|e| JsError::new(&e))
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

static S_CODEX_GOALS_FEATURE_STATE_OUT: wire::Schema = wire::Schema::Obj(&[
    ("enabled", wire::Schema::Bool),
    ("inherited", wire::Schema::Bool),
]);

/// `codexGoalsFeatureState` over the binary wire.
#[wasm_bindgen(js_name = "codexGoalsFeatureStateWire")]
pub fn codex_goals_feature_state_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let profile_config_contents: String = r.string16()?;
        let common_config_contents: String = r.string16()?;
        let use_common_config: bool = r.bool()?;
        let out = &port::codex_goals_feature_state(
            profile_config_contents,
            common_config_contents,
            use_common_config,
        )?;
        wire::write_value(w, out, &S_CODEX_GOALS_FEATURE_STATE_OUT)?;
        Ok(())
    })
}
