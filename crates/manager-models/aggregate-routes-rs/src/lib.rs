//! JS bindings for `src/aggregate-routes.ts`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;

/// `clampAggregateRoutePriority`.
fn clamp_aggregate_route_priority_inner(value: f64) -> Result<f64, String> {
    let out = port::clamp_aggregate_route_priority(value)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "clampAggregateRoutePriority")]
pub fn clamp_aggregate_route_priority(value: f64) -> Result<f64, JsError> {
    clamp_aggregate_route_priority_inner(value).map_err(|e| JsError::new(&e))
}

/// `normalizeAggregateRoutes`.
fn normalize_aggregate_routes_inner(routes: String, options: String) -> Result<String, String> {
    let routes = crate::js::from_json(&routes)?;
    let options = crate::js::from_json(&options)?;
    let out = port::normalize_aggregate_routes(routes, options)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "normalizeAggregateRoutes")]
pub fn normalize_aggregate_routes(routes: String, options: String) -> Result<String, JsError> {
    normalize_aggregate_routes_inner(routes, options).map_err(|e| JsError::new(&e))
}

/// `validateAggregateRoutes`.
fn validate_aggregate_routes_inner(routes: String, member_ids: String) -> Result<String, String> {
    let routes = crate::js::from_json(&routes)?;
    let member_ids = crate::js::from_json(&member_ids)?;
    let out = port::validate_aggregate_routes(routes, member_ids)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "validateAggregateRoutes")]
pub fn validate_aggregate_routes(routes: String, member_ids: String) -> Result<String, JsError> {
    validate_aggregate_routes_inner(routes, member_ids).map_err(|e| JsError::new(&e))
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

static S_NORMALIZE_AGGREGATE_ROUTES_0: wire::Schema = wire::Schema::List(&wire::Schema::Dyn);
static S_NORMALIZE_AGGREGATE_ROUTES_1: wire::Schema = wire::Schema::Dyn;
static S_NORMALIZE_AGGREGATE_ROUTES_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("pattern", wire::Schema::Str16),
    ("profileId", wire::Schema::Str16),
    ("priority", wire::Schema::Num),
]));

/// `normalizeAggregateRoutes` over the binary wire.
#[wasm_bindgen(js_name = "normalizeAggregateRoutesWire")]
pub fn normalize_aggregate_routes_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let routes: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_NORMALIZE_AGGREGATE_ROUTES_0.inner()))?;
        let options: serde_json::Value = wire::read_value(r, &S_NORMALIZE_AGGREGATE_ROUTES_1)?;
        let out = &port::normalize_aggregate_routes(routes, options)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_NORMALIZE_AGGREGATE_ROUTES_OUT.inner())?;
        }
        Ok(())
    })
}

static S_VALIDATE_AGGREGATE_ROUTES_0: wire::Schema = wire::Schema::List(&wire::Schema::Dyn);
static S_VALIDATE_AGGREGATE_ROUTES_1: wire::Schema = wire::Schema::Dyn;
static S_VALIDATE_AGGREGATE_ROUTES_OUT: wire::Schema =
    wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Dyn));

/// `validateAggregateRoutes` over the binary wire.
#[wasm_bindgen(js_name = "validateAggregateRoutesWire")]
pub fn validate_aggregate_routes_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let routes: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_VALIDATE_AGGREGATE_ROUTES_0.inner()))?;
        let member_ids: serde_json::Value = wire::read_value(r, &S_VALIDATE_AGGREGATE_ROUTES_1)?;
        let out = &port::validate_aggregate_routes(routes, member_ids)?;
        match out {
            Some(x0) => {
                w.tag(1);
                w.tag(1);
                w.count(x0.len());
                for x1 in x0.iter() {
                    wire::write_value(w, x1, S_VALIDATE_AGGREGATE_ROUTES_OUT.inner().inner())?;
                }
            }
            None => w.tag(2),
        }
        Ok(())
    })
}
