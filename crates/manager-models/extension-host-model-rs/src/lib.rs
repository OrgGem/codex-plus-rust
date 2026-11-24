//! JS bindings for `src/extension-host-model.ts`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;

/// `extensionHostForTool`.
fn extension_host_for_tool_inner(tool: String) -> Result<String, String> {
    let out = port::extension_host_for_tool(tool)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "extensionHostForTool")]
pub fn extension_host_for_tool(tool: String) -> Result<String, JsError> {
    extension_host_for_tool_inner(tool).map_err(|e| JsError::new(&e))
}

/// `extensionsForHost`.
fn extensions_for_host_inner(scripts: String, host: String) -> Result<String, String> {
    let scripts = crate::js::from_json(&scripts)?;
    let out = port::extensions_for_host(scripts, host)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "extensionsForHost")]
pub fn extensions_for_host(scripts: String, host: String) -> Result<String, JsError> {
    extensions_for_host_inner(scripts, host).map_err(|e| JsError::new(&e))
}

/// `extensionEnabledForHost`.
fn extension_enabled_for_host_inner(script: String, host: String) -> Result<String, String> {
    let script = crate::js::from_json(&script)?;
    let out = port::extension_enabled_for_host(script, host)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "extensionEnabledForHost")]
pub fn extension_enabled_for_host(script: String, host: String) -> Result<String, JsError> {
    extension_enabled_for_host_inner(script, host).map_err(|e| JsError::new(&e))
}

/// `extensionStateForHost`.
fn extension_state_for_host_inner(
    script: String,
    host: String,
    global_enabled: bool,
) -> Result<String, String> {
    let script = crate::js::from_json(&script)?;
    let out = port::extension_state_for_host(script, host, global_enabled)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "extensionStateForHost")]
pub fn extension_state_for_host(
    script: String,
    host: String,
    global_enabled: bool,
) -> Result<String, JsError> {
    extension_state_for_host_inner(script, host, global_enabled).map_err(|e| JsError::new(&e))
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

static S_EXTENSIONS_FOR_HOST_0: wire::Schema = wire::Schema::List(&wire::Schema::Dyn);
static S_EXTENSIONS_FOR_HOST_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Dyn);

/// `extensionsForHost` over the binary wire.
#[wasm_bindgen(js_name = "extensionsForHostWire")]
pub fn extensions_for_host_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let scripts: Vec<Value> =
            r.list(|r| wire::read_value(r, S_EXTENSIONS_FOR_HOST_0.inner()))?;
        let host: String = r.string16()?;
        let out = &port::extensions_for_host(scripts, host)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_EXTENSIONS_FOR_HOST_OUT.inner())?;
        }
        Ok(())
    })
}

static S_EXTENSION_ENABLED_FOR_HOST_0: wire::Schema = wire::Schema::Dyn;
static S_EXTENSION_ENABLED_FOR_HOST_OUT: wire::Schema = wire::Schema::Bool;

/// `extensionEnabledForHost` over the binary wire.
#[wasm_bindgen(js_name = "extensionEnabledForHostWire")]
pub fn extension_enabled_for_host_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let script: Value = wire::read_value(r, &S_EXTENSION_ENABLED_FOR_HOST_0)?;
        let host: String = r.string16()?;
        let out = &port::extension_enabled_for_host(script, host)?;
        wire::write_value(w, out, &S_EXTENSION_ENABLED_FOR_HOST_OUT)?;
        Ok(())
    })
}

static S_EXTENSION_STATE_FOR_HOST_0: wire::Schema = wire::Schema::Dyn;

/// `extensionStateForHost` over the binary wire.
#[wasm_bindgen(js_name = "extensionStateForHostWire")]
pub fn extension_state_for_host_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let script: Value = wire::read_value(r, &S_EXTENSION_STATE_FOR_HOST_0)?;
        let host: String = r.string16()?;
        let global_enabled: bool = r.bool()?;
        let out = &port::extension_state_for_host(script, host, global_enabled)?;
        w.string16(out);
        Ok(())
    })
}
