//! JS bindings for `src/script-market-model.ts`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;
pub mod utf16;

/// `parseMarketTime`.
fn parse_market_time_inner(value: String) -> Result<Option<f64>, String> {
    let value = crate::js::from_json(&value)?;
    let out = port::parse_market_time(value)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "parseMarketTime")]
pub fn parse_market_time(value: String) -> Result<Option<f64>, JsError> {
    parse_market_time_inner(value).map_err(|e| JsError::new(&e))
}

/// `defaultMarketSort`.
fn default_market_sort_inner(scripts: String) -> Result<String, String> {
    let scripts = crate::js::from_json(&scripts)?;
    let out = port::default_market_sort(scripts)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "defaultMarketSort")]
pub fn default_market_sort(scripts: String) -> Result<String, JsError> {
    default_market_sort_inner(scripts).map_err(|e| JsError::new(&e))
}

/// `sortMarketScripts`.
fn sort_market_scripts_inner(scripts: String, sort: String) -> Result<String, String> {
    let scripts = crate::js::from_json(&scripts)?;
    let out = port::sort_market_scripts(scripts, sort)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "sortMarketScripts")]
pub fn sort_market_scripts(scripts: String, sort: String) -> Result<String, JsError> {
    sort_market_scripts_inner(scripts, sort).map_err(|e| JsError::new(&e))
}

/// `relativeAge`.
fn relative_age_inner(time: f64, now: f64) -> Result<String, String> {
    let out = port::relative_age(time, now)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "relativeAge")]
pub fn relative_age(time: f64, now: f64) -> Result<String, JsError> {
    relative_age_inner(time, now).map_err(|e| JsError::new(&e))
}

/// `localScriptState`.
fn local_script_state_inner(script: String, global_enabled: bool) -> Result<String, String> {
    let script = crate::js::from_json(&script)?;
    let out = port::local_script_state(script, global_enabled)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "localScriptState")]
pub fn local_script_state(script: String, global_enabled: bool) -> Result<String, JsError> {
    local_script_state_inner(script, global_enabled).map_err(|e| JsError::new(&e))
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

static S_PARSE_MARKET_TIME_0: wire::Schema = wire::Schema::Dyn;

/// `parseMarketTime` over the binary wire.
#[wasm_bindgen(js_name = "parseMarketTimeWire")]
pub fn parse_market_time_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let value: Value = wire::read_value(r, &S_PARSE_MARKET_TIME_0)?;
        let out = &port::parse_market_time(value)?;
        match out {
            Some(x0) => {
                w.tag(1);
                w.f64(*x0);
            }
            None => w.tag(2),
        }
        Ok(())
    })
}

static S_DEFAULT_MARKET_SORT_0: wire::Schema = wire::Schema::Dyn;

/// `defaultMarketSort` over the binary wire.
#[wasm_bindgen(js_name = "defaultMarketSortWire")]
pub fn default_market_sort_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let scripts: Value = wire::read_value(r, &S_DEFAULT_MARKET_SORT_0)?;
        let out = &port::default_market_sort(scripts)?;
        w.string16(out);
        Ok(())
    })
}

static S_SORT_MARKET_SCRIPTS_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("name", wire::Schema::Str16),
    ("updatedAt", wire::Schema::OptJs(&wire::Schema::Str16)),
]));
static S_SORT_MARKET_SCRIPTS_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("name", wire::Schema::Str16),
    ("updatedAt", wire::Schema::OptJs(&wire::Schema::Str16)),
]));

/// `sortMarketScripts` over the binary wire.
#[wasm_bindgen(js_name = "sortMarketScriptsWire")]
pub fn sort_market_scripts_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let scripts: Vec<Value> =
            r.list(|r| wire::read_value(r, S_SORT_MARKET_SCRIPTS_0.inner()))?;
        let sort: String = r.string16()?;
        let out = &port::sort_market_scripts(scripts, sort)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_SORT_MARKET_SCRIPTS_OUT.inner())?;
        }
        Ok(())
    })
}

static S_RELATIVE_AGE_OUT: wire::Schema = wire::Schema::Dyn;

/// `relativeAge` over the binary wire.
#[wasm_bindgen(js_name = "relativeAgeWire")]
pub fn relative_age_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let time: f64 = r.f64()?;
        let now: f64 = r.f64()?;
        let out = &port::relative_age(time, now)?;
        wire::write_value(w, out, &S_RELATIVE_AGE_OUT)?;
        Ok(())
    })
}

static S_LOCAL_SCRIPT_STATE_0: wire::Schema = wire::Schema::Dyn;

/// `localScriptState` over the binary wire.
#[wasm_bindgen(js_name = "localScriptStateWire")]
pub fn local_script_state_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let script: Value = wire::read_value(r, &S_LOCAL_SCRIPT_STATE_0)?;
        let global_enabled: bool = r.bool()?;
        let out = &port::local_script_state(script, global_enabled)?;
        w.string16(out);
        Ok(())
    })
}
