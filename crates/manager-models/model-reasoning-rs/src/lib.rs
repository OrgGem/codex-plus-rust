//! JS bindings for `src/model-reasoning.ts`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;
pub mod utf16;

/// `reasoningParameterMode`.
fn reasoning_parameter_mode_inner(metadata: String) -> Result<String, String> {
    let metadata = crate::js::from_json(&metadata)?;
    let out = port::reasoning_parameter_mode(metadata)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "reasoningParameterMode")]
pub fn reasoning_parameter_mode(metadata: String) -> Result<String, JsError> {
    reasoning_parameter_mode_inner(metadata).map_err(|e| JsError::new(&e))
}

/// `setModelReasoningParameterMode`.
fn set_model_reasoning_parameter_mode_inner(
    value: String,
    slug: String,
    mode: String,
) -> Result<String, String> {
    let out = port::set_model_reasoning_parameter_mode(value, slug, mode)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "setModelReasoningParameterMode")]
pub fn set_model_reasoning_parameter_mode(
    value: String,
    slug: String,
    mode: String,
) -> Result<String, JsError> {
    set_model_reasoning_parameter_mode_inner(value, slug, mode).map_err(|e| JsError::new(&e))
}

/// `normalizeReasoningLevels`.
fn normalize_reasoning_levels_inner(value: String) -> Result<Vec<String>, String> {
    let value = crate::js::from_json(&value)?;
    let out = port::normalize_reasoning_levels(value)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "normalizeReasoningLevels")]
pub fn normalize_reasoning_levels(value: String) -> Result<Vec<String>, JsError> {
    normalize_reasoning_levels_inner(value).map_err(|e| JsError::new(&e))
}

/// `reasoningLevelsFromMetadata`.
fn reasoning_levels_from_metadata_inner(metadata: String) -> Result<String, String> {
    let metadata = crate::js::from_json(&metadata)?;
    let out = port::reasoning_levels_from_metadata(metadata)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "reasoningLevelsFromMetadata")]
pub fn reasoning_levels_from_metadata(metadata: String) -> Result<String, JsError> {
    reasoning_levels_from_metadata_inner(metadata).map_err(|e| JsError::new(&e))
}

/// `defaultReasoningLevelFromMetadata`.
fn default_reasoning_level_from_metadata_inner(metadata: String) -> Result<String, String> {
    let metadata = crate::js::from_json(&metadata)?;
    let out = port::default_reasoning_level_from_metadata(metadata)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "defaultReasoningLevelFromMetadata")]
pub fn default_reasoning_level_from_metadata(metadata: String) -> Result<String, JsError> {
    default_reasoning_level_from_metadata_inner(metadata).map_err(|e| JsError::new(&e))
}

/// `formatReasoningLevelsLabel`.
fn format_reasoning_levels_label_inner(levels: Vec<String>) -> Result<String, String> {
    let out = port::format_reasoning_levels_label(levels)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "formatReasoningLevelsLabel")]
pub fn format_reasoning_levels_label(levels: Vec<String>) -> Result<String, JsError> {
    format_reasoning_levels_label_inner(levels).map_err(|e| JsError::new(&e))
}

/// `resolveModelReasoningLevels`.
fn resolve_model_reasoning_levels_inner(
    r#override: String,
    builtin_levels: String,
) -> Result<String, String> {
    let r#override = crate::js::from_json(&r#override)?;
    let builtin_levels = crate::js::from_json(&builtin_levels)?;
    let out = port::resolve_model_reasoning_levels(r#override, builtin_levels)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "resolveModelReasoningLevels")]
pub fn resolve_model_reasoning_levels(
    r#override: String,
    builtin_levels: String,
) -> Result<String, JsError> {
    resolve_model_reasoning_levels_inner(r#override, builtin_levels).map_err(|e| JsError::new(&e))
}

/// `setModelReasoningLevelsForSlug`.
fn set_model_reasoning_levels_for_slug_inner(
    value: String,
    slug: String,
    levels: String,
    default_level: Option<String>,
) -> Result<String, String> {
    let levels = crate::js::from_json(&levels)?;
    let out = port::set_model_reasoning_levels_for_slug(value, slug, levels, default_level)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "setModelReasoningLevelsForSlug")]
pub fn set_model_reasoning_levels_for_slug(
    value: String,
    slug: String,
    levels: String,
    default_level: Option<String>,
) -> Result<String, JsError> {
    set_model_reasoning_levels_for_slug_inner(value, slug, levels, default_level)
        .map_err(|e| JsError::new(&e))
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

/// `reasoningParameterMode` over the binary wire.
#[wasm_bindgen(js_name = "reasoningParameterModeWire")]
pub fn reasoning_parameter_mode_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let metadata: Option<Value> = wire::read_dyn_opt(r)?;
        let out = &port::reasoning_parameter_mode(metadata)?;
        w.string16(out);
        Ok(())
    })
}

static S_NORMALIZE_REASONING_LEVELS_0: wire::Schema = wire::Schema::Dyn;

/// `normalizeReasoningLevels` over the binary wire.
#[wasm_bindgen(js_name = "normalizeReasoningLevelsWire")]
pub fn normalize_reasoning_levels_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let value: Value = wire::read_value(r, &S_NORMALIZE_REASONING_LEVELS_0)?;
        let out = &port::normalize_reasoning_levels(value)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            w.string16(x0);
        }
        Ok(())
    })
}

static S_REASONING_LEVELS_FROM_METADATA_OUT: wire::Schema =
    wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Str16));

/// `reasoningLevelsFromMetadata` over the binary wire.
#[wasm_bindgen(js_name = "reasoningLevelsFromMetadataWire")]
pub fn reasoning_levels_from_metadata_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let metadata: Option<Value> = wire::read_dyn_opt(r)?;
        let out = &port::reasoning_levels_from_metadata(metadata)?;
        wire::write_value(w, out, &S_REASONING_LEVELS_FROM_METADATA_OUT)?;
        Ok(())
    })
}

static S_DEFAULT_REASONING_LEVEL_FROM_METADATA_OUT: wire::Schema =
    wire::Schema::OptJs(&wire::Schema::Str16);

/// `defaultReasoningLevelFromMetadata` over the binary wire.
#[wasm_bindgen(js_name = "defaultReasoningLevelFromMetadataWire")]
pub fn default_reasoning_level_from_metadata_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let metadata: Option<Value> = wire::read_dyn_opt(r)?;
        let out = &port::default_reasoning_level_from_metadata(metadata)?;
        wire::write_value(w, out, &S_DEFAULT_REASONING_LEVEL_FROM_METADATA_OUT)?;
        Ok(())
    })
}

static S_RESOLVE_MODEL_REASONING_LEVELS_1: wire::Schema = wire::Schema::Dyn;
static S_RESOLVE_MODEL_REASONING_LEVELS_OUT: wire::Schema = wire::Schema::Obj(&[
    ("levels", wire::Schema::List(&wire::Schema::Str16)),
    ("overridden", wire::Schema::Bool),
    ("defaultLevel", wire::Schema::OptJs(&wire::Schema::Str16)),
]);

/// `resolveModelReasoningLevels` over the binary wire.
#[wasm_bindgen(js_name = "resolveModelReasoningLevelsWire")]
pub fn resolve_model_reasoning_levels_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let r#override: Option<Value> = wire::read_dyn_opt(r)?;
        let builtin_levels: Value = wire::read_value(r, &S_RESOLVE_MODEL_REASONING_LEVELS_1)?;
        let out = &port::resolve_model_reasoning_levels(r#override, builtin_levels)?;
        wire::write_value(w, out, &S_RESOLVE_MODEL_REASONING_LEVELS_OUT)?;
        Ok(())
    })
}

static S_SET_MODEL_REASONING_LEVELS_FOR_SLUG_2: wire::Schema = wire::Schema::Dyn;

/// `setModelReasoningLevelsForSlug` over the binary wire.
#[wasm_bindgen(js_name = "setModelReasoningLevelsForSlugWire")]
pub fn set_model_reasoning_levels_for_slug_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let value: String = r.string16()?;
        let slug: String = r.string16()?;
        let levels: Value = wire::read_value(r, &S_SET_MODEL_REASONING_LEVELS_FOR_SLUG_2)?;
        let default_level: Option<String> = r.opt_js(|r| r.string16())?;
        let out = &port::set_model_reasoning_levels_for_slug(value, slug, levels, default_level)?;
        w.string16(out);
        Ok(())
    })
}
