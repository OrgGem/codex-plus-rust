//! JS bindings for `src/dream-skin.ts`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;

/// `defaultDreamSkinColors`.
fn default_dream_skin_colors_inner() -> Result<String, String> {
    let out = port::default_dream_skin_colors()?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "defaultDreamSkinColors")]
pub fn default_dream_skin_colors() -> Result<String, JsError> {
    default_dream_skin_colors_inner().map_err(|e| JsError::new(&e))
}

/// `defaultDreamSkinTheme`.
fn default_dream_skin_theme_inner() -> Result<String, String> {
    let out = port::default_dream_skin_theme()?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "defaultDreamSkinTheme")]
pub fn default_dream_skin_theme() -> Result<String, JsError> {
    default_dream_skin_theme_inner().map_err(|e| JsError::new(&e))
}

/// `resolveDreamSkinStylePreset`.
fn resolve_dream_skin_style_preset_inner(
    id: String,
    style_preset: String,
) -> Result<String, String> {
    let style_preset = crate::js::from_json(&style_preset)?;
    let out = port::resolve_dream_skin_style_preset(id, style_preset)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "resolveDreamSkinStylePreset")]
pub fn resolve_dream_skin_style_preset(
    id: String,
    style_preset: String,
) -> Result<String, JsError> {
    resolve_dream_skin_style_preset_inner(id, style_preset).map_err(|e| JsError::new(&e))
}

/// `normalizeDreamSkinTheme`.
fn normalize_dream_skin_theme_inner(value: String) -> Result<String, String> {
    let value = crate::js::from_json(&value)?;
    let out = port::normalize_dream_skin_theme(value)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "normalizeDreamSkinTheme")]
pub fn normalize_dream_skin_theme(value: String) -> Result<String, JsError> {
    normalize_dream_skin_theme_inner(value).map_err(|e| JsError::new(&e))
}

/// `dreamSkinDraftFingerprint`.
fn dream_skin_draft_fingerprint_inner(draft: String) -> Result<String, String> {
    let draft = crate::js::from_json(&draft)?;
    let out = port::dream_skin_draft_fingerprint(draft)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "dreamSkinDraftFingerprint")]
pub fn dream_skin_draft_fingerprint(draft: String) -> Result<String, JsError> {
    dream_skin_draft_fingerprint_inner(draft).map_err(|e| JsError::new(&e))
}

/// `isDreamSkinDraftDirty`.
fn is_dream_skin_draft_dirty_inner(saved: String, draft: String) -> Result<bool, String> {
    let saved = crate::js::from_json(&saved)?;
    let draft = crate::js::from_json(&draft)?;
    let out = port::is_dream_skin_draft_dirty(saved, draft)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "isDreamSkinDraftDirty")]
pub fn is_dream_skin_draft_dirty(saved: String, draft: String) -> Result<bool, JsError> {
    is_dream_skin_draft_dirty_inner(saved, draft).map_err(|e| JsError::new(&e))
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

static S_DEFAULT_DREAM_SKIN_COLORS_OUT: wire::Schema = wire::Schema::Dyn;

/// `defaultDreamSkinColors` over the binary wire.
#[wasm_bindgen(js_name = "defaultDreamSkinColorsWire")]
pub fn default_dream_skin_colors_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |_r, w| {
        let out = &port::default_dream_skin_colors()?;
        wire::write_value(w, out, &S_DEFAULT_DREAM_SKIN_COLORS_OUT)?;
        Ok(())
    })
}

static S_DEFAULT_DREAM_SKIN_THEME_OUT: wire::Schema = wire::Schema::Dyn;

/// `defaultDreamSkinTheme` over the binary wire.
#[wasm_bindgen(js_name = "defaultDreamSkinThemeWire")]
pub fn default_dream_skin_theme_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |_r, w| {
        let out = &port::default_dream_skin_theme()?;
        wire::write_value(w, out, &S_DEFAULT_DREAM_SKIN_THEME_OUT)?;
        Ok(())
    })
}

static S_RESOLVE_DREAM_SKIN_STYLE_PRESET_1: wire::Schema = wire::Schema::Dyn;

/// `resolveDreamSkinStylePreset` over the binary wire.
#[wasm_bindgen(js_name = "resolveDreamSkinStylePresetWire")]
pub fn resolve_dream_skin_style_preset_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let id: String = r.string16()?;
        let style_preset: serde_json::Value =
            wire::read_value(r, &S_RESOLVE_DREAM_SKIN_STYLE_PRESET_1)?;
        let out = &port::resolve_dream_skin_style_preset(id, style_preset)?;
        w.string16(out);
        Ok(())
    })
}

static S_NORMALIZE_DREAM_SKIN_THEME_OUT: wire::Schema = wire::Schema::Dyn;

/// `normalizeDreamSkinTheme` over the binary wire.
#[wasm_bindgen(js_name = "normalizeDreamSkinThemeWire")]
pub fn normalize_dream_skin_theme_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let value: Option<serde_json::Value> = wire::read_dyn_opt(r)?;
        let out = &port::normalize_dream_skin_theme(value)?;
        wire::write_value(w, out, &S_NORMALIZE_DREAM_SKIN_THEME_OUT)?;
        Ok(())
    })
}

static S_DREAM_SKIN_DRAFT_FINGERPRINT_0: wire::Schema = wire::Schema::Dyn;

/// `dreamSkinDraftFingerprint` over the binary wire.
#[wasm_bindgen(js_name = "dreamSkinDraftFingerprintWire")]
pub fn dream_skin_draft_fingerprint_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let draft: serde_json::Value = wire::read_value(r, &S_DREAM_SKIN_DRAFT_FINGERPRINT_0)?;
        let out = &port::dream_skin_draft_fingerprint(draft)?;
        w.string16(out);
        Ok(())
    })
}

static S_IS_DREAM_SKIN_DRAFT_DIRTY_0: wire::Schema = wire::Schema::Dyn;
static S_IS_DREAM_SKIN_DRAFT_DIRTY_1: wire::Schema = wire::Schema::Dyn;

/// `isDreamSkinDraftDirty` over the binary wire.
#[wasm_bindgen(js_name = "isDreamSkinDraftDirtyWire")]
pub fn is_dream_skin_draft_dirty_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let saved: serde_json::Value = wire::read_value(r, &S_IS_DREAM_SKIN_DRAFT_DIRTY_0)?;
        let draft: serde_json::Value = wire::read_value(r, &S_IS_DREAM_SKIN_DRAFT_DIRTY_1)?;
        let out = &port::is_dream_skin_draft_dirty(saved, draft)?;
        w.bool(*out);
        Ok(())
    })
}
