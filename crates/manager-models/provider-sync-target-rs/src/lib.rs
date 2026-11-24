//! JS bindings for `src/provider-sync-target.ts`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;

/// `isProviderSyncTargetSelectable`.
fn is_provider_sync_target_selectable_inner(target: String) -> Result<bool, String> {
    let target = crate::js::from_json(&target)?;
    let out = port::is_provider_sync_target_selectable(target)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "isProviderSyncTargetSelectable")]
pub fn is_provider_sync_target_selectable(target: String) -> Result<bool, JsError> {
    is_provider_sync_target_selectable_inner(target).map_err(|e| JsError::new(&e))
}

/// `preferredProviderSyncTarget`.
fn preferred_provider_sync_target_inner(
    targets: String,
    current_provider: String,
    saved_provider: String,
) -> Result<String, String> {
    let targets = crate::js::from_json(&targets)?;
    let out = port::preferred_provider_sync_target(targets, current_provider, saved_provider)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "preferredProviderSyncTarget")]
pub fn preferred_provider_sync_target(
    targets: String,
    current_provider: String,
    saved_provider: String,
) -> Result<String, JsError> {
    preferred_provider_sync_target_inner(targets, current_provider, saved_provider)
        .map_err(|e| JsError::new(&e))
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

static S_IS_PROVIDER_SYNC_TARGET_SELECTABLE_0: wire::Schema = wire::Schema::Dyn;

/// `isProviderSyncTargetSelectable` over the binary wire.
#[wasm_bindgen(js_name = "isProviderSyncTargetSelectableWire")]
pub fn is_provider_sync_target_selectable_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let target: serde_json::Value =
            wire::read_value(r, &S_IS_PROVIDER_SYNC_TARGET_SELECTABLE_0)?;
        let out = &port::is_provider_sync_target_selectable(target)?;
        w.bool(*out);
        Ok(())
    })
}

static S_PREFERRED_PROVIDER_SYNC_TARGET_0: wire::Schema =
    wire::Schema::List(&wire::Schema::Obj(&[
        ("id", wire::Schema::Str16),
        ("isCurrentProvider", wire::Schema::Bool),
        ("isResolvable", wire::Schema::Bool),
    ]));

/// `preferredProviderSyncTarget` over the binary wire.
#[wasm_bindgen(js_name = "preferredProviderSyncTargetWire")]
pub fn preferred_provider_sync_target_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let targets: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_PREFERRED_PROVIDER_SYNC_TARGET_0.inner()))?;
        let current_provider: String = r.string16()?;
        let saved_provider: String = r.string16()?;
        let out = &port::preferred_provider_sync_target(targets, current_provider, saved_provider)?;
        w.string16(out);
        Ok(())
    })
}
