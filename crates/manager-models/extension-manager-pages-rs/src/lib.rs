//! JS bindings for `src/extension-manager-pages.ts`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;

/// `managerExtensionPages`.
fn manager_extension_pages_inner(inventory: String) -> Result<String, String> {
    let inventory = crate::js::from_json(&inventory)?;
    let out = port::manager_extension_pages(inventory)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "managerExtensionPages")]
pub fn manager_extension_pages(inventory: String) -> Result<String, JsError> {
    manager_extension_pages_inner(inventory).map_err(|e| JsError::new(&e))
}

/// `managerPageOverrides`.
fn manager_page_overrides_inner(pages: String) -> Result<String, String> {
    let pages = crate::js::from_json(&pages)?;
    let out = port::manager_page_overrides(pages)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "managerPageOverrides")]
pub fn manager_page_overrides(pages: String) -> Result<String, JsError> {
    manager_page_overrides_inner(pages).map_err(|e| JsError::new(&e))
}

/// `managerNavPages`.
fn manager_nav_pages_inner(pages: String, group: String) -> Result<String, String> {
    let pages = crate::js::from_json(&pages)?;
    let out = port::manager_nav_pages(pages, group)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "managerNavPages")]
pub fn manager_nav_pages(pages: String, group: String) -> Result<String, JsError> {
    manager_nav_pages_inner(pages, group).map_err(|e| JsError::new(&e))
}

/// `managerDataRequest`.
fn manager_data_request_inner(
    name: String,
    params: String,
    declared: Vec<String>,
    script_key: String,
) -> Result<String, String> {
    let params = crate::js::from_json(&params)?;
    let out = port::manager_data_request(name, params, declared, script_key)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "managerDataRequest")]
pub fn manager_data_request(
    name: String,
    params: String,
    declared: Vec<String>,
    script_key: String,
) -> Result<String, JsError> {
    manager_data_request_inner(name, params, declared, script_key).map_err(|e| JsError::new(&e))
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

static S_MANAGER_EXTENSION_PAGES_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("key", wire::Schema::Str16),
    ("title", wire::Schema::Str16),
    ("nav", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("icon", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("replaces", wire::Schema::List(&wire::Schema::Str16)),
    ("data", wire::Schema::List(&wire::Schema::Str16)),
]));

/// `managerExtensionPages` over the binary wire.
#[wasm_bindgen(js_name = "managerExtensionPagesWire")]
pub fn manager_extension_pages_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let inventory: Option<serde_json::Value> = wire::read_dyn_opt(r)?;
        let out = &port::manager_extension_pages(inventory)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_MANAGER_EXTENSION_PAGES_OUT.inner())?;
        }
        Ok(())
    })
}

static S_MANAGER_PAGE_OVERRIDES_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("key", wire::Schema::Str16),
    ("title", wire::Schema::Str16),
    ("nav", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("icon", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("replaces", wire::Schema::List(&wire::Schema::Str16)),
    ("data", wire::Schema::List(&wire::Schema::Str16)),
]));
static S_MANAGER_PAGE_OVERRIDES_OUT: wire::Schema = wire::Schema::Map16(&wire::Schema::Obj(&[
    ("key", wire::Schema::Str16),
    ("title", wire::Schema::Str16),
    ("nav", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("icon", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("replaces", wire::Schema::List(&wire::Schema::Str16)),
    ("data", wire::Schema::List(&wire::Schema::Str16)),
]));

/// `managerPageOverrides` over the binary wire.
#[wasm_bindgen(js_name = "managerPageOverridesWire")]
pub fn manager_page_overrides_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let pages: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_MANAGER_PAGE_OVERRIDES_0.inner()))?;
        let out = &port::manager_page_overrides(pages)?;
        wire::write_value(w, out, &S_MANAGER_PAGE_OVERRIDES_OUT)?;
        Ok(())
    })
}

static S_MANAGER_NAV_PAGES_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("key", wire::Schema::Str16),
    ("title", wire::Schema::Str16),
    ("nav", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("icon", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("replaces", wire::Schema::List(&wire::Schema::Str16)),
    ("data", wire::Schema::List(&wire::Schema::Str16)),
]));
static S_MANAGER_NAV_PAGES_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("key", wire::Schema::Str16),
    ("title", wire::Schema::Str16),
    ("nav", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("icon", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("replaces", wire::Schema::List(&wire::Schema::Str16)),
    ("data", wire::Schema::List(&wire::Schema::Str16)),
]));

/// `managerNavPages` over the binary wire.
#[wasm_bindgen(js_name = "managerNavPagesWire")]
pub fn manager_nav_pages_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let pages: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_MANAGER_NAV_PAGES_0.inner()))?;
        let group: String = r.string16()?;
        let out = &port::manager_nav_pages(pages, group)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_MANAGER_NAV_PAGES_OUT.inner())?;
        }
        Ok(())
    })
}

static S_MANAGER_DATA_REQUEST_1: wire::Schema = wire::Schema::Dyn;
static S_MANAGER_DATA_REQUEST_OUT: wire::Schema = wire::Schema::Dyn;

/// `managerDataRequest` over the binary wire.
#[wasm_bindgen(js_name = "managerDataRequestWire")]
pub fn manager_data_request_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let name: String = r.string16()?;
        let params: serde_json::Value = wire::read_value(r, &S_MANAGER_DATA_REQUEST_1)?;
        let declared: Vec<String> = r.list(|r| r.string16())?;
        let script_key: String = r.string16()?;
        let out = &port::manager_data_request(name, params, declared, script_key)?;
        wire::write_value(w, out, &S_MANAGER_DATA_REQUEST_OUT)?;
        Ok(())
    })
}
