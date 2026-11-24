//! JS bindings for `src/relay-billing.ts`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;
pub mod utf16;

/// `relayPlatformLabel`.
fn relay_platform_label_inner(platform: String) -> Result<String, String> {
    let out = port::relay_platform_label(platform)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "relayPlatformLabel")]
pub fn relay_platform_label(platform: String) -> Result<String, JsError> {
    relay_platform_label_inner(platform).map_err(|e| JsError::new(&e))
}

/// `normalizeRelayBillingProbe`.
fn normalize_relay_billing_probe_inner(result: String) -> Result<String, String> {
    let result = crate::js::from_json(&result)?;
    let out = port::normalize_relay_billing_probe(result)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "normalizeRelayBillingProbe")]
pub fn normalize_relay_billing_probe(result: String) -> Result<String, JsError> {
    normalize_relay_billing_probe_inner(result).map_err(|e| JsError::new(&e))
}

/// `newApiAccessHint`.
fn new_api_access_hint_inner(probe: String) -> Result<String, String> {
    let probe = crate::js::from_json(&probe)?;
    let out = port::new_api_access_hint(probe)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "newApiAccessHint")]
pub fn new_api_access_hint(probe: String) -> Result<String, JsError> {
    new_api_access_hint_inner(probe).map_err(|e| JsError::new(&e))
}

/// `mergeRelayBillingResult`.
fn merge_relay_billing_result_inner(
    previous: String,
    probe: String,
    now: f64,
) -> Result<String, String> {
    let previous = crate::js::from_json(&previous)?;
    let probe = crate::js::from_json(&probe)?;
    let out = port::merge_relay_billing_result(previous, probe, now)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "mergeRelayBillingResult")]
pub fn merge_relay_billing_result(
    previous: String,
    probe: String,
    now: f64,
) -> Result<String, JsError> {
    merge_relay_billing_result_inner(previous, probe, now).map_err(|e| JsError::new(&e))
}

/// `relayBillingCacheUsable`.
fn relay_billing_cache_usable_inner(entry: String, now: f64) -> Result<bool, String> {
    let entry = crate::js::from_json(&entry)?;
    let out = port::relay_billing_cache_usable(entry, now)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "relayBillingCacheUsable")]
pub fn relay_billing_cache_usable(entry: String, now: f64) -> Result<bool, JsError> {
    relay_billing_cache_usable_inner(entry, now).map_err(|e| JsError::new(&e))
}

/// `relayBillingProbeEligible`.
fn relay_billing_probe_eligible_inner(profile: String) -> Result<bool, String> {
    let profile = crate::js::from_json(&profile)?;
    let out = port::relay_billing_probe_eligible(profile)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "relayBillingProbeEligible")]
pub fn relay_billing_probe_eligible(profile: String) -> Result<bool, JsError> {
    relay_billing_probe_eligible_inner(profile).map_err(|e| JsError::new(&e))
}

/// `relayBillingCacheKey`.
fn relay_billing_cache_key_inner(profile: String) -> Result<String, String> {
    let profile = crate::js::from_json(&profile)?;
    let out = port::relay_billing_cache_key(profile)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "relayBillingCacheKey")]
pub fn relay_billing_cache_key(profile: String) -> Result<String, JsError> {
    relay_billing_cache_key_inner(profile).map_err(|e| JsError::new(&e))
}

/// `relayBillingGatewayCacheKey`.
fn relay_billing_gateway_cache_key_inner(entry: String) -> Result<String, String> {
    let entry = crate::js::from_json(&entry)?;
    let out = port::relay_billing_gateway_cache_key(entry)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "relayBillingGatewayCacheKey")]
pub fn relay_billing_gateway_cache_key(entry: String) -> Result<String, JsError> {
    relay_billing_gateway_cache_key_inner(entry).map_err(|e| JsError::new(&e))
}

/// `formatRelayMultiplier`.
fn format_relay_multiplier_inner(value: f64) -> Result<String, String> {
    let out = port::format_relay_multiplier(value)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "formatRelayMultiplier")]
pub fn format_relay_multiplier(value: f64) -> Result<String, JsError> {
    format_relay_multiplier_inner(value).map_err(|e| JsError::new(&e))
}

/// `formatRelayAmount`.
fn format_relay_amount_inner(value: f64, unit: String) -> Result<String, String> {
    let out = port::format_relay_amount(value, unit)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "formatRelayAmount")]
pub fn format_relay_amount(value: f64, unit: String) -> Result<String, JsError> {
    format_relay_amount_inner(value, unit).map_err(|e| JsError::new(&e))
}

/// `relayAppliedPeak`.
fn relay_applied_peak_inner(rate: String) -> Result<Option<f64>, String> {
    let rate = crate::js::from_json(&rate)?;
    let out = port::relay_applied_peak(rate)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "relayAppliedPeak")]
pub fn relay_applied_peak(rate: String) -> Result<Option<f64>, JsError> {
    relay_applied_peak_inner(rate).map_err(|e| JsError::new(&e))
}

/// `relayBalanceBadge`.
fn relay_balance_badge_inner(usage: String) -> Result<String, String> {
    let usage = crate::js::from_json(&usage)?;
    let out = port::relay_balance_badge(usage)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "relayBalanceBadge")]
pub fn relay_balance_badge(usage: String) -> Result<String, JsError> {
    relay_balance_badge_inner(usage).map_err(|e| JsError::new(&e))
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

static S_NORMALIZE_RELAY_BILLING_PROBE_0: wire::Schema = wire::Schema::Dyn;
static S_NORMALIZE_RELAY_BILLING_PROBE_OUT: wire::Schema = wire::Schema::Obj(&[
    ("platform", wire::Schema::OptJs(&wire::Schema::Str16)),
    (
        "rate",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("endpoint", wire::Schema::Str16),
            ("multiplier", wire::Schema::Num),
            (
                "appliedPeakMultiplier",
                wire::Schema::OptJs(&wire::Schema::Num),
            ),
            ("group", wire::Schema::OptJs(&wire::Schema::Str16)),
        ])),
    ),
    (
        "usage",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("endpoint", wire::Schema::Str16),
            ("kind", wire::Schema::Str16),
            ("isValid", wire::Schema::Bool),
            ("planName", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("remaining", wire::Schema::OptJs(&wire::Schema::Num)),
            ("quotaLimit", wire::Schema::OptJs(&wire::Schema::Num)),
            ("quotaUsed", wire::Schema::OptJs(&wire::Schema::Num)),
            ("unit", wire::Schema::Str16),
            (
                "rateLimits",
                wire::Schema::List(&wire::Schema::Obj(&[
                    ("window", wire::Schema::Str16),
                    ("limit", wire::Schema::Num),
                    ("used", wire::Schema::Num),
                    ("remaining", wire::Schema::Num),
                ])),
            ),
        ])),
    ),
    ("detail", wire::Schema::Str16),
    ("outcome", wire::Schema::Str16),
    (
        "newapiAccess",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("configured", wire::Schema::Bool),
            ("error", wire::Schema::OptJs(&wire::Schema::Str16)),
        ])),
    ),
]);

/// `normalizeRelayBillingProbe` over the binary wire.
#[wasm_bindgen(js_name = "normalizeRelayBillingProbeWire")]
pub fn normalize_relay_billing_probe_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let result: serde_json::Value = wire::read_value(r, &S_NORMALIZE_RELAY_BILLING_PROBE_0)?;
        let out = &port::normalize_relay_billing_probe(result)?;
        wire::write_value(w, out, &S_NORMALIZE_RELAY_BILLING_PROBE_OUT)?;
        Ok(())
    })
}

static S_NEW_API_ACCESS_HINT_0: wire::Schema = wire::Schema::Dyn;
static S_NEW_API_ACCESS_HINT_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Obj(&[
    ("text", wire::Schema::Str16),
    ("failed", wire::Schema::Bool),
]));

/// `newApiAccessHint` over the binary wire.
#[wasm_bindgen(js_name = "newApiAccessHintWire")]
pub fn new_api_access_hint_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let probe: serde_json::Value = wire::read_value(r, &S_NEW_API_ACCESS_HINT_0)?;
        let out = &port::new_api_access_hint(probe)?;
        match out {
            Some(x0) => {
                w.tag(1);
                wire::write_value(w, x0, S_NEW_API_ACCESS_HINT_OUT.inner())?;
            }
            None => w.tag(2),
        }
        Ok(())
    })
}

static S_MERGE_RELAY_BILLING_RESULT_1: wire::Schema = wire::Schema::Dyn;
static S_MERGE_RELAY_BILLING_RESULT_OUT: wire::Schema = wire::Schema::Obj(&[
    (
        "entry",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            (
                "probe",
                wire::Schema::OptJs(&wire::Schema::Obj(&[
                    ("platform", wire::Schema::OptJs(&wire::Schema::Str16)),
                    (
                        "rate",
                        wire::Schema::OptJs(&wire::Schema::Obj(&[
                            ("endpoint", wire::Schema::Dyn),
                            ("multiplier", wire::Schema::Dyn),
                            ("appliedPeakMultiplier", wire::Schema::Dyn),
                            ("group", wire::Schema::Dyn),
                        ])),
                    ),
                    (
                        "usage",
                        wire::Schema::OptJs(&wire::Schema::Obj(&[
                            ("endpoint", wire::Schema::Dyn),
                            ("kind", wire::Schema::Dyn),
                            ("isValid", wire::Schema::Dyn),
                            ("planName", wire::Schema::Dyn),
                            ("remaining", wire::Schema::Dyn),
                            ("quotaLimit", wire::Schema::Dyn),
                            ("quotaUsed", wire::Schema::Dyn),
                            ("unit", wire::Schema::Dyn),
                            ("rateLimits", wire::Schema::Dyn),
                        ])),
                    ),
                    ("detail", wire::Schema::Str16),
                    ("outcome", wire::Schema::Str16),
                    (
                        "newapiAccess",
                        wire::Schema::OptJs(&wire::Schema::Obj(&[
                            ("configured", wire::Schema::Dyn),
                            ("error", wire::Schema::Dyn),
                        ])),
                    ),
                ])),
            ),
            ("at", wire::Schema::Num),
            (
                "failure",
                wire::Schema::OptJs(&wire::Schema::Obj(&[
                    ("at", wire::Schema::Num),
                    ("detail", wire::Schema::Str16),
                ])),
            ),
        ])),
    ),
    ("negative", wire::Schema::Str16),
]);

/// `mergeRelayBillingResult` over the binary wire.
#[wasm_bindgen(js_name = "mergeRelayBillingResultWire")]
pub fn merge_relay_billing_result_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let previous: Option<serde_json::Value> = wire::read_dyn_opt(r)?;
        let probe: serde_json::Value = wire::read_value(r, &S_MERGE_RELAY_BILLING_RESULT_1)?;
        let now: f64 = r.f64()?;
        let out = &port::merge_relay_billing_result(previous, probe, now)?;
        wire::write_value(w, out, &S_MERGE_RELAY_BILLING_RESULT_OUT)?;
        Ok(())
    })
}

/// `relayBillingCacheUsable` over the binary wire.
#[wasm_bindgen(js_name = "relayBillingCacheUsableWire")]
pub fn relay_billing_cache_usable_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let entry: Option<serde_json::Value> = wire::read_dyn_opt(r)?;
        let now: f64 = r.f64()?;
        let out = &port::relay_billing_cache_usable(entry, now)?;
        w.bool(*out);
        Ok(())
    })
}

static S_RELAY_BILLING_PROBE_ELIGIBLE_0: wire::Schema = wire::Schema::Dyn;

/// `relayBillingProbeEligible` over the binary wire.
#[wasm_bindgen(js_name = "relayBillingProbeEligibleWire")]
pub fn relay_billing_probe_eligible_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let profile: serde_json::Value = wire::read_value(r, &S_RELAY_BILLING_PROBE_ELIGIBLE_0)?;
        let out = &port::relay_billing_probe_eligible(profile)?;
        w.bool(*out);
        Ok(())
    })
}

static S_RELAY_BILLING_CACHE_KEY_0: wire::Schema = wire::Schema::Dyn;

/// `relayBillingCacheKey` over the binary wire.
#[wasm_bindgen(js_name = "relayBillingCacheKeyWire")]
pub fn relay_billing_cache_key_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let profile: serde_json::Value = wire::read_value(r, &S_RELAY_BILLING_CACHE_KEY_0)?;
        let out = &port::relay_billing_cache_key(profile)?;
        w.string16(out);
        Ok(())
    })
}

static S_RELAY_BILLING_GATEWAY_CACHE_KEY_0: wire::Schema = wire::Schema::Dyn;

/// `relayBillingGatewayCacheKey` over the binary wire.
#[wasm_bindgen(js_name = "relayBillingGatewayCacheKeyWire")]
pub fn relay_billing_gateway_cache_key_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let entry: serde_json::Value = wire::read_value(r, &S_RELAY_BILLING_GATEWAY_CACHE_KEY_0)?;
        let out = &port::relay_billing_gateway_cache_key(entry)?;
        w.string16(out);
        Ok(())
    })
}

/// `relayAppliedPeak` over the binary wire.
#[wasm_bindgen(js_name = "relayAppliedPeakWire")]
pub fn relay_applied_peak_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let rate: Option<serde_json::Value> = wire::read_dyn_opt(r)?;
        let out = &port::relay_applied_peak(rate)?;
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

static S_RELAY_BALANCE_BADGE_0: wire::Schema = wire::Schema::Dyn;
static S_RELAY_BALANCE_BADGE_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Obj(&[
    ("amount", wire::Schema::Str16),
    ("ratio", wire::Schema::OptJs(&wire::Schema::Num)),
    ("tone", wire::Schema::Str16),
]));

/// `relayBalanceBadge` over the binary wire.
#[wasm_bindgen(js_name = "relayBalanceBadgeWire")]
pub fn relay_balance_badge_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let usage: serde_json::Value = wire::read_value(r, &S_RELAY_BALANCE_BADGE_0)?;
        let out = &port::relay_balance_badge(usage)?;
        match out {
            Some(x0) => {
                w.tag(1);
                wire::write_value(w, x0, S_RELAY_BALANCE_BADGE_OUT.inner())?;
            }
            None => w.tag(2),
        }
        Ok(())
    })
}
