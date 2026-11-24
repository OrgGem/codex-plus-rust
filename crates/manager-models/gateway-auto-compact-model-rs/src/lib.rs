//! JS bindings for `src/gateway-auto-compact-model.ts`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;

/// `parseCompactPercent`.
fn parse_compact_percent_inner(text: String) -> Result<String, String> {
    let out = port::parse_compact_percent(text)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "parseCompactPercent")]
pub fn parse_compact_percent(text: String) -> Result<String, JsError> {
    parse_compact_percent_inner(text).map_err(|e| JsError::new(&e))
}

/// `withUnlistedAutoCompact`.
fn with_unlisted_auto_compact_inner(
    entry: String,
    shown: String,
    auto_compact: String,
) -> Result<String, String> {
    let entry = crate::js::from_json(&entry)?;
    let shown = crate::js::from_json(&shown)?;
    let auto_compact = crate::js::from_json(&auto_compact)?;
    let out = port::with_unlisted_auto_compact(entry, shown, auto_compact)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "withUnlistedAutoCompact")]
pub fn with_unlisted_auto_compact(
    entry: String,
    shown: String,
    auto_compact: String,
) -> Result<String, JsError> {
    with_unlisted_auto_compact_inner(entry, shown, auto_compact).map_err(|e| JsError::new(&e))
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

static S_PARSE_COMPACT_PERCENT_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Num);

/// `parseCompactPercent` over the binary wire.
#[wasm_bindgen(js_name = "parseCompactPercentWire")]
pub fn parse_compact_percent_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let text: String = r.string16()?;
        let out = &port::parse_compact_percent(text)?;
        wire::write_value(w, out, &S_PARSE_COMPACT_PERCENT_OUT)?;
        Ok(())
    })
}

static S_WITH_UNLISTED_AUTO_COMPACT_0: wire::Schema = wire::Schema::Dyn;
static S_WITH_UNLISTED_AUTO_COMPACT_1: wire::Schema = wire::Schema::Dyn;
static S_WITH_UNLISTED_AUTO_COMPACT_2: wire::Schema = wire::Schema::Dyn;
static S_WITH_UNLISTED_AUTO_COMPACT_OUT: wire::Schema = wire::Schema::Obj(&[
    ("models", wire::Schema::Map16(&wire::Schema::Num)),
    ("fallback", wire::Schema::OptJs(&wire::Schema::Num)),
]);

/// `withUnlistedAutoCompact` over the binary wire.
#[wasm_bindgen(js_name = "withUnlistedAutoCompactWire")]
pub fn with_unlisted_auto_compact_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let entry: Value = wire::read_value(r, &S_WITH_UNLISTED_AUTO_COMPACT_0)?;
        let shown: Value = wire::read_value(r, &S_WITH_UNLISTED_AUTO_COMPACT_1)?;
        let auto_compact: Value = wire::read_value(r, &S_WITH_UNLISTED_AUTO_COMPACT_2)?;
        let out = &port::with_unlisted_auto_compact(entry, shown, auto_compact)?;
        wire::write_value(w, out, &S_WITH_UNLISTED_AUTO_COMPACT_OUT)?;
        Ok(())
    })
}
