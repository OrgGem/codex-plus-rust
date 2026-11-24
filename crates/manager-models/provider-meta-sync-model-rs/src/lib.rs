//! JS bindings for `src/provider-meta-sync-model.ts`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;

/// `normalizePreview`.
fn normalize_preview_inner(raw: String) -> Result<String, String> {
    let raw = crate::js::from_json(&raw)?;
    let out = port::normalize_preview(raw)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "normalizePreview")]
pub fn normalize_preview(raw: String) -> Result<String, JsError> {
    normalize_preview_inner(raw).map_err(|e| JsError::new(&e))
}

/// `formatLevels`.
fn format_levels_inner(
    levels: Vec<String>,
    default_level: String,
    default_label: String,
) -> Result<String, String> {
    let default_level = crate::js::from_json(&default_level)?;
    let default_label = crate::js::from_json(&default_label)?;
    let out = port::format_levels(levels, default_level, default_label)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "formatLevels")]
pub fn format_levels(
    levels: Vec<String>,
    default_level: String,
    default_label: String,
) -> Result<String, JsError> {
    format_levels_inner(levels, default_level, default_label).map_err(|e| JsError::new(&e))
}

/// `changeRows`.
fn change_rows_inner(
    preview: String,
    kind: String,
    default_label: String,
) -> Result<String, String> {
    let preview = crate::js::from_json(&preview)?;
    let default_label = crate::js::from_json(&default_label)?;
    let out = port::change_rows(preview, kind, default_label)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "changeRows")]
pub fn change_rows(
    preview: String,
    kind: String,
    default_label: String,
) -> Result<String, JsError> {
    change_rows_inner(preview, kind, default_label).map_err(|e| JsError::new(&e))
}

/// `windowPicks`.
fn window_picks_inner(preview: String, picked: String) -> Result<String, String> {
    let preview = crate::js::from_json(&preview)?;
    let picked = crate::js::from_json(&picked)?;
    let out = port::window_picks(preview, picked)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "windowPicks")]
pub fn window_picks(preview: String, picked: String) -> Result<String, JsError> {
    window_picks_inner(preview, picked).map_err(|e| JsError::new(&e))
}

/// `reasoningPicks`.
fn reasoning_picks_inner(preview: String, picked: String) -> Result<String, String> {
    let preview = crate::js::from_json(&preview)?;
    let picked = crate::js::from_json(&picked)?;
    let out = port::reasoning_picks(preview, picked)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "reasoningPicks")]
pub fn reasoning_picks(preview: String, picked: String) -> Result<String, JsError> {
    reasoning_picks_inner(preview, picked).map_err(|e| JsError::new(&e))
}

/// `ignoredFor`.
fn ignored_for_inner(preview: String, kind: String) -> Result<String, String> {
    let preview = crate::js::from_json(&preview)?;
    let out = port::ignored_for(preview, kind)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "ignoredFor")]
pub fn ignored_for(preview: String, kind: String) -> Result<String, JsError> {
    ignored_for_inner(preview, kind).map_err(|e| JsError::new(&e))
}

/// `currentSourceLabel`.
fn current_source_label_inner(source: String) -> Result<String, String> {
    let out = port::current_source_label(source)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "currentSourceLabel")]
pub fn current_source_label(source: String) -> Result<String, JsError> {
    current_source_label_inner(source).map_err(|e| JsError::new(&e))
}

/// `ignoredReasonLabel`.
fn ignored_reason_label_inner(reason: String) -> Result<String, String> {
    let out = port::ignored_reason_label(reason)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "ignoredReasonLabel")]
pub fn ignored_reason_label(reason: String) -> Result<String, JsError> {
    ignored_reason_label_inner(reason).map_err(|e| JsError::new(&e))
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

static S_NORMALIZE_PREVIEW_0: wire::Schema = wire::Schema::Dyn;
static S_NORMALIZE_PREVIEW_OUT: wire::Schema = wire::Schema::Obj(&[
    ("upstreamModels", wire::Schema::Num),
    ("matched", wire::Schema::Num),
    ("withWindow", wire::Schema::Num),
    ("withReasoning", wire::Schema::Num),
    (
        "windows",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("model", wire::Schema::Str16),
            ("window", wire::Schema::Num),
            ("source", wire::Schema::Str16),
            ("options", wire::Schema::List(&wire::Schema::Num)),
            ("current", wire::Schema::OptJs(&wire::Schema::Num)),
            ("currentSource", wire::Schema::Str16),
            ("manual", wire::Schema::Bool),
        ])),
    ),
    (
        "reasoning",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("model", wire::Schema::Str16),
            ("levels", wire::Schema::List(&wire::Schema::Str16)),
            ("defaultLevel", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("source", wire::Schema::Str16),
            ("defaultSource", wire::Schema::Str16),
            ("currentLevels", wire::Schema::List(&wire::Schema::Str16)),
            ("currentDefault", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("currentSource", wire::Schema::Str16),
            ("manual", wire::Schema::Bool),
        ])),
    ),
    (
        "ignored",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("model", wire::Schema::Str16),
            ("field", wire::Schema::Str16),
            ("value", wire::Schema::Str16),
            ("reason", wire::Schema::Str16),
        ])),
    ),
]);

/// `normalizePreview` over the binary wire.
#[wasm_bindgen(js_name = "normalizePreviewWire")]
pub fn normalize_preview_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let raw: Value = wire::read_value(r, &S_NORMALIZE_PREVIEW_0)?;
        let out = &port::normalize_preview(raw)?;
        wire::write_value(w, out, &S_NORMALIZE_PREVIEW_OUT)?;
        Ok(())
    })
}

static S_FORMAT_LEVELS_1: wire::Schema = wire::Schema::Dyn;
static S_FORMAT_LEVELS_2: wire::Schema = wire::Schema::Dyn;

/// `formatLevels` over the binary wire.
#[wasm_bindgen(js_name = "formatLevelsWire")]
pub fn format_levels_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let levels: Vec<String> = r.list(|r| r.string16())?;
        let default_level: Value = wire::read_value(r, &S_FORMAT_LEVELS_1)?;
        let default_label: Value = wire::read_value(r, &S_FORMAT_LEVELS_2)?;
        let out = &port::format_levels(levels, default_level, default_label)?;
        w.string16(out);
        Ok(())
    })
}

static S_CHANGE_ROWS_0: wire::Schema = wire::Schema::Dyn;
static S_CHANGE_ROWS_2: wire::Schema = wire::Schema::Dyn;
static S_CHANGE_ROWS_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("model", wire::Schema::Str16),
    ("next", wire::Schema::Str16),
    ("current", wire::Schema::Str16),
    ("currentSource", wire::Schema::Str16),
    ("source", wire::Schema::Str16),
    ("manual", wire::Schema::Bool),
    ("options", wire::Schema::Str16),
]));

/// `changeRows` over the binary wire.
#[wasm_bindgen(js_name = "changeRowsWire")]
pub fn change_rows_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let preview: Value = wire::read_value(r, &S_CHANGE_ROWS_0)?;
        let kind: String = r.string16()?;
        let default_label: Value = wire::read_value(r, &S_CHANGE_ROWS_2)?;
        let out = &port::change_rows(preview, kind, default_label)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_CHANGE_ROWS_OUT.inner())?;
        }
        Ok(())
    })
}

static S_WINDOW_PICKS_0: wire::Schema = wire::Schema::Dyn;
static S_WINDOW_PICKS_1: wire::Schema = wire::Schema::Dyn;
static S_WINDOW_PICKS_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("model", wire::Schema::Str16),
    ("window", wire::Schema::Num),
]));

/// `windowPicks` over the binary wire.
#[wasm_bindgen(js_name = "windowPicksWire")]
pub fn window_picks_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let preview: Value = wire::read_value(r, &S_WINDOW_PICKS_0)?;
        let picked: Value = wire::read_value(r, &S_WINDOW_PICKS_1)?;
        let out = &port::window_picks(preview, picked)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_WINDOW_PICKS_OUT.inner())?;
        }
        Ok(())
    })
}

static S_REASONING_PICKS_0: wire::Schema = wire::Schema::Dyn;
static S_REASONING_PICKS_1: wire::Schema = wire::Schema::Dyn;
static S_REASONING_PICKS_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("model", wire::Schema::Str16),
    ("levels", wire::Schema::List(&wire::Schema::Str16)),
    ("defaultLevel", wire::Schema::OptJs(&wire::Schema::Str16)),
]));

/// `reasoningPicks` over the binary wire.
#[wasm_bindgen(js_name = "reasoningPicksWire")]
pub fn reasoning_picks_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let preview: Value = wire::read_value(r, &S_REASONING_PICKS_0)?;
        let picked: Value = wire::read_value(r, &S_REASONING_PICKS_1)?;
        let out = &port::reasoning_picks(preview, picked)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_REASONING_PICKS_OUT.inner())?;
        }
        Ok(())
    })
}

static S_IGNORED_FOR_0: wire::Schema = wire::Schema::Dyn;
static S_IGNORED_FOR_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("model", wire::Schema::Str16),
    ("field", wire::Schema::Str16),
    ("value", wire::Schema::Str16),
    ("reason", wire::Schema::Str16),
]));

/// `ignoredFor` over the binary wire.
#[wasm_bindgen(js_name = "ignoredForWire")]
pub fn ignored_for_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let preview: Value = wire::read_value(r, &S_IGNORED_FOR_0)?;
        let kind: String = r.string16()?;
        let out = &port::ignored_for(preview, kind)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_IGNORED_FOR_OUT.inner())?;
        }
        Ok(())
    })
}
