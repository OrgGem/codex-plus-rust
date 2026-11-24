//! JS bindings for `src/provider-test.ts`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;

/// `formatProbeMs`.
fn format_probe_ms_inner(ms: Option<f64>) -> Result<String, String> {
    let out = port::format_probe_ms(ms)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "formatProbeMs")]
pub fn format_probe_ms(ms: Option<f64>) -> Result<String, JsError> {
    format_probe_ms_inner(ms).map_err(|e| JsError::new(&e))
}

/// `probeWireLabel`.
fn probe_wire_label_inner(wire: String) -> Result<String, String> {
    let out = port::probe_wire_label(wire)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "probeWireLabel")]
pub fn probe_wire_label(wire: String) -> Result<String, JsError> {
    probe_wire_label_inner(wire).map_err(|e| JsError::new(&e))
}

/// `probeKeyHint`.
fn probe_key_hint_inner(result: String) -> Result<String, String> {
    let result = crate::js::from_json(&result)?;
    let out = port::probe_key_hint(result)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "probeKeyHint")]
pub fn probe_key_hint(result: String) -> Result<String, JsError> {
    probe_key_hint_inner(result).map_err(|e| JsError::new(&e))
}

/// `probeHints`.
fn probe_hints_inner(result: String, account: Option<bool>) -> Result<String, String> {
    let result = crate::js::from_json(&result)?;
    let out = port::probe_hints(result, account)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "probeHints")]
pub fn probe_hints(result: String, account: Option<bool>) -> Result<String, JsError> {
    probe_hints_inner(result, account).map_err(|e| JsError::new(&e))
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

static S_PROBE_KEY_HINT_0: wire::Schema = wire::Schema::Dyn;
static S_PROBE_KEY_HINT_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Obj(&[
    ("text", wire::Schema::Str16),
    ("args", wire::Schema::List(&wire::Schema::Str16)),
]));

/// `probeKeyHint` over the binary wire.
#[wasm_bindgen(js_name = "probeKeyHintWire")]
pub fn probe_key_hint_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let result: serde_json::Value = wire::read_value(r, &S_PROBE_KEY_HINT_0)?;
        let out = &port::probe_key_hint(result)?;
        match out {
            Some(x0) => {
                w.tag(1);
                wire::write_value(w, x0, S_PROBE_KEY_HINT_OUT.inner())?;
            }
            None => w.tag(2),
        }
        Ok(())
    })
}

static S_PROBE_HINTS_0: wire::Schema = wire::Schema::Dyn;
static S_PROBE_HINTS_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("text", wire::Schema::Str16),
    ("args", wire::Schema::List(&wire::Schema::Str16)),
]));

/// `probeHints` over the binary wire.
#[wasm_bindgen(js_name = "probeHintsWire")]
pub fn probe_hints_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let result: serde_json::Value = wire::read_value(r, &S_PROBE_HINTS_0)?;
        let account: Option<bool> = r.opt_js(|r| r.bool())?;
        let out = &port::probe_hints(result, account)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_PROBE_HINTS_OUT.inner())?;
        }
        Ok(())
    })
}
