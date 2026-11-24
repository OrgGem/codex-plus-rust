//! JS bindings for `src/subscription-policy.ts`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;

/// `defaultSubscriptionPolicy`.
fn default_subscription_policy_inner() -> Result<String, String> {
    let out = port::default_subscription_policy()?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "defaultSubscriptionPolicy")]
pub fn default_subscription_policy() -> Result<String, JsError> {
    default_subscription_policy_inner().map_err(|e| JsError::new(&e))
}

/// `normalizeSubscriptionPolicy`.
fn normalize_subscription_policy_inner(value: String) -> Result<String, String> {
    let value = crate::js::from_json(&value)?;
    let out = port::normalize_subscription_policy(value)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "normalizeSubscriptionPolicy")]
pub fn normalize_subscription_policy(value: String) -> Result<String, JsError> {
    normalize_subscription_policy_inner(value).map_err(|e| JsError::new(&e))
}

/// `subscriptionPercentLimit`.
fn subscription_percent_limit_inner(text: String) -> Result<String, String> {
    let out = port::subscription_percent_limit(text)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "subscriptionPercentLimit")]
pub fn subscription_percent_limit(text: String) -> Result<String, JsError> {
    subscription_percent_limit_inner(text).map_err(|e| JsError::new(&e))
}

/// `withSubscriptionWindowLimit`.
fn with_subscription_window_limit_inner(
    policy: String,
    minutes: String,
    text: String,
) -> Result<String, String> {
    let policy = crate::js::from_json(&policy)?;
    let out = port::with_subscription_window_limit(policy, minutes, text)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "withSubscriptionWindowLimit")]
pub fn with_subscription_window_limit(
    policy: String,
    minutes: String,
    text: String,
) -> Result<String, JsError> {
    with_subscription_window_limit_inner(policy, minutes, text).map_err(|e| JsError::new(&e))
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

static S_DEFAULT_SUBSCRIPTION_POLICY_OUT: wire::Schema = wire::Schema::Obj(&[
    (
        "defaultUsedPercentLimit",
        wire::Schema::OptJs(&wire::Schema::Num),
    ),
    (
        "windowUsedPercentLimits",
        wire::Schema::Map16(&wire::Schema::Num),
    ),
    ("onLimit", wire::Schema::Str16),
    ("unknownUsage", wire::Schema::Str16),
    ("maxAgeSeconds", wire::Schema::Num),
]);

/// `defaultSubscriptionPolicy` over the binary wire.
#[wasm_bindgen(js_name = "defaultSubscriptionPolicyWire")]
pub fn default_subscription_policy_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |_r, w| {
        let out = &port::default_subscription_policy()?;
        wire::write_value(w, out, &S_DEFAULT_SUBSCRIPTION_POLICY_OUT)?;
        Ok(())
    })
}

static S_NORMALIZE_SUBSCRIPTION_POLICY_OUT: wire::Schema = wire::Schema::Obj(&[
    (
        "defaultUsedPercentLimit",
        wire::Schema::OptJs(&wire::Schema::Num),
    ),
    (
        "windowUsedPercentLimits",
        wire::Schema::Map16(&wire::Schema::Num),
    ),
    ("onLimit", wire::Schema::Str16),
    ("unknownUsage", wire::Schema::Str16),
    ("maxAgeSeconds", wire::Schema::Num),
]);

/// `normalizeSubscriptionPolicy` over the binary wire.
#[wasm_bindgen(js_name = "normalizeSubscriptionPolicyWire")]
pub fn normalize_subscription_policy_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let value: Option<serde_json::Value> = wire::read_dyn_opt(r)?;
        let out = &port::normalize_subscription_policy(value)?;
        wire::write_value(w, out, &S_NORMALIZE_SUBSCRIPTION_POLICY_OUT)?;
        Ok(())
    })
}

static S_SUBSCRIPTION_PERCENT_LIMIT_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Num);

/// `subscriptionPercentLimit` over the binary wire.
#[wasm_bindgen(js_name = "subscriptionPercentLimitWire")]
pub fn subscription_percent_limit_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let text: String = r.string16()?;
        let out = &port::subscription_percent_limit(text)?;
        wire::write_value(w, out, &S_SUBSCRIPTION_PERCENT_LIMIT_OUT)?;
        Ok(())
    })
}

static S_WITH_SUBSCRIPTION_WINDOW_LIMIT_0: wire::Schema = wire::Schema::Dyn;
static S_WITH_SUBSCRIPTION_WINDOW_LIMIT_OUT: wire::Schema = wire::Schema::Obj(&[
    (
        "defaultUsedPercentLimit",
        wire::Schema::OptJs(&wire::Schema::Num),
    ),
    (
        "windowUsedPercentLimits",
        wire::Schema::Map16(&wire::Schema::Num),
    ),
    ("onLimit", wire::Schema::Str16),
    ("unknownUsage", wire::Schema::Str16),
    ("maxAgeSeconds", wire::Schema::Num),
]);

/// `withSubscriptionWindowLimit` over the binary wire.
#[wasm_bindgen(js_name = "withSubscriptionWindowLimitWire")]
pub fn with_subscription_window_limit_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let policy: serde_json::Value = wire::read_value(r, &S_WITH_SUBSCRIPTION_WINDOW_LIMIT_0)?;
        let minutes: String = r.string16()?;
        let text: String = r.string16()?;
        let out = &port::with_subscription_window_limit(policy, minutes, text)?;
        wire::write_value(w, out, &S_WITH_SUBSCRIPTION_WINDOW_LIMIT_OUT)?;
        Ok(())
    })
}
