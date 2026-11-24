//! JS bindings for `src/provider-balances.ts`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;

/// `providerBalancesFresh`.
fn provider_balances_fresh_inner(entry: String, now: f64, ttl: String) -> Result<bool, String> {
    let entry = crate::js::from_json(&entry)?;
    let ttl = crate::js::from_json(&ttl)?;
    let out = port::provider_balances_fresh(entry, now, ttl)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "providerBalancesFresh")]
pub fn provider_balances_fresh(entry: String, now: f64, ttl: String) -> Result<bool, JsError> {
    provider_balances_fresh_inner(entry, now, ttl).map_err(|e| JsError::new(&e))
}

/// `providerBalanceFor`.
fn provider_balance_for_inner(balances: String, id: String) -> Result<String, String> {
    let balances = crate::js::from_json(&balances)?;
    let out = port::provider_balance_for(balances, id)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "providerBalanceFor")]
pub fn provider_balance_for(balances: String, id: String) -> Result<String, JsError> {
    provider_balance_for_inner(balances, id).map_err(|e| JsError::new(&e))
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

static S_PROVIDER_BALANCES_FRESH_0: wire::Schema = wire::Schema::Dyn;
static S_PROVIDER_BALANCES_FRESH_2: wire::Schema = wire::Schema::Dyn;

/// `providerBalancesFresh` over the binary wire.
#[wasm_bindgen(js_name = "providerBalancesFreshWire")]
pub fn provider_balances_fresh_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let entry: Value = wire::read_value(r, &S_PROVIDER_BALANCES_FRESH_0)?;
        let now: f64 = r.f64()?;
        let ttl: Value = wire::read_value(r, &S_PROVIDER_BALANCES_FRESH_2)?;
        let out = &port::provider_balances_fresh(entry, now, ttl)?;
        w.bool(*out);
        Ok(())
    })
}

static S_PROVIDER_BALANCE_FOR_0: wire::Schema = wire::Schema::Dyn;
static S_PROVIDER_BALANCE_FOR_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Obj(&[
    ("providerId", wire::Schema::Str16),
    ("provider", wire::Schema::Str16),
    ("baseUrl", wire::Schema::Str16),
    ("amount", wire::Schema::OptJs(&wire::Schema::Num)),
    ("currency", wire::Schema::Str16),
    ("error", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("note", wire::Schema::OptJs(&wire::Schema::Str16)),
]));

/// `providerBalanceFor` over the binary wire.
#[wasm_bindgen(js_name = "providerBalanceForWire")]
pub fn provider_balance_for_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let balances: Value = wire::read_value(r, &S_PROVIDER_BALANCE_FOR_0)?;
        let id: String = r.string16()?;
        let out = &port::provider_balance_for(balances, id)?;
        wire::write_value(w, out, &S_PROVIDER_BALANCE_FOR_OUT)?;
        Ok(())
    })
}
