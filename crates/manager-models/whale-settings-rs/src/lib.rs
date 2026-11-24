//! JS bindings for `src/whale-settings.ts`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;

/// `defaultWhaleBalanceSettings`.
fn default_whale_balance_settings_inner() -> Result<String, String> {
    let out = port::default_whale_balance_settings()?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "defaultWhaleBalanceSettings")]
pub fn default_whale_balance_settings() -> Result<String, JsError> {
    default_whale_balance_settings_inner().map_err(|e| JsError::new(&e))
}

/// `normalizeWhaleBalanceSettings`.
fn normalize_whale_balance_settings_inner(raw: String) -> Result<String, String> {
    let raw = crate::js::from_json(&raw)?;
    let out = port::normalize_whale_balance_settings(raw)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "normalizeWhaleBalanceSettings")]
pub fn normalize_whale_balance_settings(raw: String) -> Result<String, JsError> {
    normalize_whale_balance_settings_inner(raw).map_err(|e| JsError::new(&e))
}

/// `whaleBalanceSettingsIssue`.
fn whale_balance_settings_issue_inner(
    settings: String,
    whale_active: bool,
) -> Result<Option<String>, String> {
    let settings = crate::js::from_json(&settings)?;
    let out = port::whale_balance_settings_issue(settings, whale_active)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "whaleBalanceSettingsIssue")]
pub fn whale_balance_settings_issue(
    settings: String,
    whale_active: bool,
) -> Result<Option<String>, JsError> {
    whale_balance_settings_issue_inner(settings, whale_active).map_err(|e| JsError::new(&e))
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

static S_DEFAULT_WHALE_BALANCE_SETTINGS_OUT: wire::Schema = wire::Schema::Obj(&[
    ("codexAppWhaleBalanceProtocol", wire::Schema::Str16),
    ("codexAppWhaleBalancePath", wire::Schema::Str16),
    ("codexAppWhaleBalanceField", wire::Schema::Str16),
    ("codexAppWhaleBalanceCurrency", wire::Schema::Str16),
    ("codexAppWhaleBalanceScale", wire::Schema::Num),
]);

/// `defaultWhaleBalanceSettings` over the binary wire.
#[wasm_bindgen(js_name = "defaultWhaleBalanceSettingsWire")]
pub fn default_whale_balance_settings_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |_r, w| {
        let out = &port::default_whale_balance_settings()?;
        wire::write_value(w, out, &S_DEFAULT_WHALE_BALANCE_SETTINGS_OUT)?;
        Ok(())
    })
}

static S_NORMALIZE_WHALE_BALANCE_SETTINGS_0: wire::Schema = wire::Schema::Dyn;
static S_NORMALIZE_WHALE_BALANCE_SETTINGS_OUT: wire::Schema = wire::Schema::Obj(&[
    ("codexAppWhaleBalanceProtocol", wire::Schema::Str16),
    ("codexAppWhaleBalancePath", wire::Schema::Str16),
    ("codexAppWhaleBalanceField", wire::Schema::Str16),
    ("codexAppWhaleBalanceCurrency", wire::Schema::Str16),
    ("codexAppWhaleBalanceScale", wire::Schema::Num),
]);

/// `normalizeWhaleBalanceSettings` over the binary wire.
#[wasm_bindgen(js_name = "normalizeWhaleBalanceSettingsWire")]
pub fn normalize_whale_balance_settings_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let raw: serde_json::Value = wire::read_value(r, &S_NORMALIZE_WHALE_BALANCE_SETTINGS_0)?;
        let out = &port::normalize_whale_balance_settings(raw)?;
        wire::write_value(w, out, &S_NORMALIZE_WHALE_BALANCE_SETTINGS_OUT)?;
        Ok(())
    })
}

static S_WHALE_BALANCE_SETTINGS_ISSUE_0: wire::Schema = wire::Schema::Dyn;

/// `whaleBalanceSettingsIssue` over the binary wire.
#[wasm_bindgen(js_name = "whaleBalanceSettingsIssueWire")]
pub fn whale_balance_settings_issue_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let settings: serde_json::Value = wire::read_value(r, &S_WHALE_BALANCE_SETTINGS_ISSUE_0)?;
        let whale_active: bool = r.bool()?;
        let out = &port::whale_balance_settings_issue(settings, whale_active)?;
        match out {
            Some(x0) => {
                w.tag(1);
                w.string16(x0);
            }
            None => w.tag(2),
        }
        Ok(())
    })
}
