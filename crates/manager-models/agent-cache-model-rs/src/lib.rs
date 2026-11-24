//! JS bindings for `src/agent-cache-model.ts`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;
pub mod utf16;

/// `selectableCacheGroups`.
fn selectable_cache_groups_inner(groups: String) -> Result<String, String> {
    let groups = crate::js::from_json(&groups)?;
    let out = port::selectable_cache_groups(groups)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "selectableCacheGroups")]
pub fn selectable_cache_groups(groups: String) -> Result<String, JsError> {
    selectable_cache_groups_inner(groups).map_err(|e| JsError::new(&e))
}

/// `cacheSelection`.
fn cache_selection_inner(groups: String, mode: String) -> Result<String, String> {
    let groups = crate::js::from_json(&groups)?;
    let out = port::cache_selection(groups, mode)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "cacheSelection")]
pub fn cache_selection(groups: String, mode: String) -> Result<String, JsError> {
    cache_selection_inner(groups, mode).map_err(|e| JsError::new(&e))
}

/// `cacheCleanupSummary`.
fn cache_cleanup_summary_inner(groups: String) -> Result<String, String> {
    let groups = crate::js::from_json(&groups)?;
    let out = port::cache_cleanup_summary(groups)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "cacheCleanupSummary")]
pub fn cache_cleanup_summary(groups: String) -> Result<String, JsError> {
    cache_cleanup_summary_inner(groups).map_err(|e| JsError::new(&e))
}

/// `cacheKindTotals`.
fn cache_kind_totals_inner(groups: String) -> Result<String, String> {
    let groups = crate::js::from_json(&groups)?;
    let out = port::cache_kind_totals(groups)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "cacheKindTotals")]
pub fn cache_kind_totals(groups: String) -> Result<String, JsError> {
    cache_kind_totals_inner(groups).map_err(|e| JsError::new(&e))
}

/// `cacheApps`.
fn cache_apps_inner(groups: String) -> Result<String, String> {
    let groups = crate::js::from_json(&groups)?;
    let out = port::cache_apps(groups)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "cacheApps")]
pub fn cache_apps(groups: String) -> Result<String, JsError> {
    cache_apps_inner(groups).map_err(|e| JsError::new(&e))
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

static S_SELECTABLE_CACHE_GROUPS_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("app", wire::Schema::Str16),
    ("kind", wire::Schema::Str16),
    ("path", wire::Schema::Str16),
    ("totalBytes", wire::Schema::Num),
    ("eligibleBytes", wire::Schema::Num),
    ("eligibleFiles", wire::Schema::Num),
    ("skippedFiles", wire::Schema::Num),
    ("cleanable", wire::Schema::Bool),
]));
static S_SELECTABLE_CACHE_GROUPS_OUT: wire::Schema = wire::Schema::Dyn;

/// `selectableCacheGroups` over the binary wire.
#[wasm_bindgen(js_name = "selectableCacheGroupsWire")]
pub fn selectable_cache_groups_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let groups: Vec<Value> =
            r.list(|r| wire::read_value(r, S_SELECTABLE_CACHE_GROUPS_0.inner()))?;
        let out = &port::selectable_cache_groups(groups)?;
        wire::write_value(w, out, &S_SELECTABLE_CACHE_GROUPS_OUT)?;
        Ok(())
    })
}

static S_CACHE_SELECTION_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("app", wire::Schema::Str16),
    ("kind", wire::Schema::Str16),
    ("path", wire::Schema::Str16),
    ("totalBytes", wire::Schema::Num),
    ("eligibleBytes", wire::Schema::Num),
    ("eligibleFiles", wire::Schema::Num),
    ("skippedFiles", wire::Schema::Num),
    ("cleanable", wire::Schema::Bool),
]));
static S_CACHE_SELECTION_OUT: wire::Schema = wire::Schema::Dyn;

/// `cacheSelection` over the binary wire.
#[wasm_bindgen(js_name = "cacheSelectionWire")]
pub fn cache_selection_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let groups: Vec<Value> = r.list(|r| wire::read_value(r, S_CACHE_SELECTION_0.inner()))?;
        let mode: String = r.string16()?;
        let out = &port::cache_selection(groups, mode)?;
        wire::write_value(w, out, &S_CACHE_SELECTION_OUT)?;
        Ok(())
    })
}

static S_CACHE_CLEANUP_SUMMARY_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("app", wire::Schema::Str16),
    ("kind", wire::Schema::Str16),
    ("path", wire::Schema::Str16),
    ("totalBytes", wire::Schema::Num),
    ("eligibleBytes", wire::Schema::Num),
    ("eligibleFiles", wire::Schema::Num),
    ("skippedFiles", wire::Schema::Num),
    ("cleanable", wire::Schema::Bool),
]));
static S_CACHE_CLEANUP_SUMMARY_OUT: wire::Schema = wire::Schema::Dyn;

/// `cacheCleanupSummary` over the binary wire.
#[wasm_bindgen(js_name = "cacheCleanupSummaryWire")]
pub fn cache_cleanup_summary_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let groups: Vec<Value> =
            r.list(|r| wire::read_value(r, S_CACHE_CLEANUP_SUMMARY_0.inner()))?;
        let out = &port::cache_cleanup_summary(groups)?;
        wire::write_value(w, out, &S_CACHE_CLEANUP_SUMMARY_OUT)?;
        Ok(())
    })
}

static S_CACHE_KIND_TOTALS_0: wire::Schema = wire::Schema::Dyn;
static S_CACHE_KIND_TOTALS_OUT: wire::Schema = wire::Schema::Dyn;

/// `cacheKindTotals` over the binary wire.
#[wasm_bindgen(js_name = "cacheKindTotalsWire")]
pub fn cache_kind_totals_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let groups: Value = wire::read_value(r, &S_CACHE_KIND_TOTALS_0)?;
        let out = &port::cache_kind_totals(groups)?;
        wire::write_value(w, out, &S_CACHE_KIND_TOTALS_OUT)?;
        Ok(())
    })
}

static S_CACHE_APPS_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("app", wire::Schema::Str16),
    ("kind", wire::Schema::Str16),
    ("path", wire::Schema::Str16),
    ("totalBytes", wire::Schema::Num),
    ("eligibleBytes", wire::Schema::Num),
    ("eligibleFiles", wire::Schema::Num),
    ("skippedFiles", wire::Schema::Num),
    ("cleanable", wire::Schema::Bool),
]));
static S_CACHE_APPS_OUT: wire::Schema = wire::Schema::Dyn;

/// `cacheApps` over the binary wire.
#[wasm_bindgen(js_name = "cacheAppsWire")]
pub fn cache_apps_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let groups: Vec<Value> = r.list(|r| wire::read_value(r, S_CACHE_APPS_0.inner()))?;
        let out = &port::cache_apps(groups)?;
        wire::write_value(w, out, &S_CACHE_APPS_OUT)?;
        Ok(())
    })
}
