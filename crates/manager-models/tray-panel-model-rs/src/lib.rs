//! JS bindings for `src/tray-panel-model.ts`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;
pub mod utf16;

/// `restoreTrayTab`.
fn restore_tray_tab_inner(raw: String) -> Result<String, String> {
    let raw = crate::js::from_json(&raw)?;
    let out = port::restore_tray_tab(raw)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "restoreTrayTab")]
pub fn restore_tray_tab(raw: String) -> Result<String, JsError> {
    restore_tray_tab_inner(raw).map_err(|e| JsError::new(&e))
}

/// `codexState`.
fn codex_state_inner(overview: String) -> Result<String, String> {
    let overview = crate::js::from_json(&overview)?;
    let out = port::codex_state(overview)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "codexState")]
pub fn codex_state(overview: String) -> Result<String, JsError> {
    codex_state_inner(overview).map_err(|e| JsError::new(&e))
}

/// `codexEnhanced`.
fn codex_enhanced_inner(overview: String) -> Result<bool, String> {
    let overview = crate::js::from_json(&overview)?;
    let out = port::codex_enhanced(overview)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "codexEnhanced")]
pub fn codex_enhanced(overview: String) -> Result<bool, JsError> {
    codex_enhanced_inner(overview).map_err(|e| JsError::new(&e))
}

/// `trayRelayItems`.
fn tray_relay_items_inner(settings: String) -> Result<String, String> {
    let settings = crate::js::from_json(&settings)?;
    let out = port::tray_relay_items(settings)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "trayRelayItems")]
pub fn tray_relay_items(settings: String) -> Result<String, JsError> {
    tray_relay_items_inner(settings).map_err(|e| JsError::new(&e))
}

/// `activeRelayItem`.
fn active_relay_item_inner(settings: String) -> Result<String, String> {
    let settings = crate::js::from_json(&settings)?;
    let out = port::active_relay_item(settings)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "activeRelayItem")]
pub fn active_relay_item(settings: String) -> Result<String, JsError> {
    active_relay_item_inner(settings).map_err(|e| JsError::new(&e))
}

/// `gatewayConnected`.
fn gateway_connected_inner(payload: String) -> Result<bool, String> {
    let payload = crate::js::from_json(&payload)?;
    let out = port::gateway_connected(payload)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "gatewayConnected")]
pub fn gateway_connected(payload: String) -> Result<bool, JsError> {
    gateway_connected_inner(payload).map_err(|e| JsError::new(&e))
}

/// `gatewayIntroNeeded`.
fn gateway_intro_needed_inner(payload: String) -> Result<bool, String> {
    let payload = crate::js::from_json(&payload)?;
    let out = port::gateway_intro_needed(payload)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "gatewayIntroNeeded")]
pub fn gateway_intro_needed(payload: String) -> Result<bool, JsError> {
    gateway_intro_needed_inner(payload).map_err(|e| JsError::new(&e))
}

/// `usageGlance`.
fn usage_glance_inner(summary: String, top_count: String) -> Result<String, String> {
    let summary = crate::js::from_json(&summary)?;
    let top_count = crate::js::from_json(&top_count)?;
    let out = port::usage_glance(summary, top_count)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "usageGlance")]
pub fn usage_glance(summary: String, top_count: String) -> Result<String, JsError> {
    usage_glance_inner(summary, top_count).map_err(|e| JsError::new(&e))
}

/// `trayQuotaRows`.
fn tray_quota_rows_inner(pool: String, now_ms: String) -> Result<String, String> {
    let pool = crate::js::from_json(&pool)?;
    let now_ms = crate::js::from_json(&now_ms)?;
    let out = port::tray_quota_rows(pool, now_ms)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "trayQuotaRows")]
pub fn tray_quota_rows(pool: String, now_ms: String) -> Result<String, JsError> {
    tray_quota_rows_inner(pool, now_ms).map_err(|e| JsError::new(&e))
}

/// `isTrayRoute`.
fn is_tray_route_inner(value: String) -> Result<bool, String> {
    let value = crate::js::from_json(&value)?;
    let out = port::is_tray_route(value)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "isTrayRoute")]
pub fn is_tray_route(value: String) -> Result<bool, JsError> {
    is_tray_route_inner(value).map_err(|e| JsError::new(&e))
}

/// `parseTrayRequest`.
fn parse_tray_request_inner(value: String) -> Result<String, String> {
    let value = crate::js::from_json(&value)?;
    let out = port::parse_tray_request(value)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "parseTrayRequest")]
pub fn parse_tray_request(value: String) -> Result<String, JsError> {
    parse_tray_request_inner(value).map_err(|e| JsError::new(&e))
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

static S_RESTORE_TRAY_TAB_0: wire::Schema = wire::Schema::Dyn;

/// `restoreTrayTab` over the binary wire.
#[wasm_bindgen(js_name = "restoreTrayTabWire")]
pub fn restore_tray_tab_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let raw: Value = wire::read_value(r, &S_RESTORE_TRAY_TAB_0)?;
        let out = &port::restore_tray_tab(raw)?;
        w.string16(out);
        Ok(())
    })
}

static S_CODEX_STATE_0: wire::Schema = wire::Schema::Dyn;

/// `codexState` over the binary wire.
#[wasm_bindgen(js_name = "codexStateWire")]
pub fn codex_state_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let overview: Value = wire::read_value(r, &S_CODEX_STATE_0)?;
        let out = &port::codex_state(overview)?;
        w.string16(out);
        Ok(())
    })
}

static S_CODEX_ENHANCED_0: wire::Schema = wire::Schema::Dyn;

/// `codexEnhanced` over the binary wire.
#[wasm_bindgen(js_name = "codexEnhancedWire")]
pub fn codex_enhanced_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let overview: Value = wire::read_value(r, &S_CODEX_ENHANCED_0)?;
        let out = &port::codex_enhanced(overview)?;
        w.bool(*out);
        Ok(())
    })
}

static S_TRAY_RELAY_ITEMS_0: wire::Schema = wire::Schema::Dyn;
static S_TRAY_RELAY_ITEMS_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("name", wire::Schema::Str16),
    ("detail", wire::Schema::Str16),
    ("baseUrl", wire::Schema::Str16),
    ("kind", wire::Schema::Str16),
    ("active", wire::Schema::Bool),
]));

/// `trayRelayItems` over the binary wire.
#[wasm_bindgen(js_name = "trayRelayItemsWire")]
pub fn tray_relay_items_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let settings: Value = wire::read_value(r, &S_TRAY_RELAY_ITEMS_0)?;
        let out = &port::tray_relay_items(settings)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_TRAY_RELAY_ITEMS_OUT.inner())?;
        }
        Ok(())
    })
}

static S_ACTIVE_RELAY_ITEM_0: wire::Schema = wire::Schema::Dyn;
static S_ACTIVE_RELAY_ITEM_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("name", wire::Schema::Str16),
    ("detail", wire::Schema::Str16),
    ("baseUrl", wire::Schema::Str16),
    ("kind", wire::Schema::Str16),
    ("active", wire::Schema::Bool),
]));

/// `activeRelayItem` over the binary wire.
#[wasm_bindgen(js_name = "activeRelayItemWire")]
pub fn active_relay_item_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let settings: Value = wire::read_value(r, &S_ACTIVE_RELAY_ITEM_0)?;
        let out = &port::active_relay_item(settings)?;
        wire::write_value(w, out, &S_ACTIVE_RELAY_ITEM_OUT)?;
        Ok(())
    })
}

static S_GATEWAY_CONNECTED_0: wire::Schema = wire::Schema::Dyn;

/// `gatewayConnected` over the binary wire.
#[wasm_bindgen(js_name = "gatewayConnectedWire")]
pub fn gateway_connected_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let payload: Value = wire::read_value(r, &S_GATEWAY_CONNECTED_0)?;
        let out = &port::gateway_connected(payload)?;
        w.bool(*out);
        Ok(())
    })
}

static S_GATEWAY_INTRO_NEEDED_0: wire::Schema = wire::Schema::Dyn;

/// `gatewayIntroNeeded` over the binary wire.
#[wasm_bindgen(js_name = "gatewayIntroNeededWire")]
pub fn gateway_intro_needed_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let payload: Value = wire::read_value(r, &S_GATEWAY_INTRO_NEEDED_0)?;
        let out = &port::gateway_intro_needed(payload)?;
        w.bool(*out);
        Ok(())
    })
}

static S_USAGE_GLANCE_0: wire::Schema = wire::Schema::Dyn;
static S_USAGE_GLANCE_1: wire::Schema = wire::Schema::Dyn;
static S_USAGE_GLANCE_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Obj(&[
    ("calls", wire::Schema::Num),
    ("errors", wire::Schema::Num),
    ("successRate", wire::Schema::OptJs(&wire::Schema::Num)),
    ("inputTokens", wire::Schema::Num),
    ("outputTokens", wire::Schema::Num),
    ("cachedTokens", wire::Schema::Num),
    (
        "topModels",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("key", wire::Schema::Str16),
            ("label", wire::Schema::Str16),
            ("tokens", wire::Schema::Num),
            ("share", wire::Schema::Num),
        ])),
    ),
]));

/// `usageGlance` over the binary wire.
#[wasm_bindgen(js_name = "usageGlanceWire")]
pub fn usage_glance_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let summary: Value = wire::read_value(r, &S_USAGE_GLANCE_0)?;
        let top_count: Value = wire::read_value(r, &S_USAGE_GLANCE_1)?;
        let out = &port::usage_glance(summary, top_count)?;
        wire::write_value(w, out, &S_USAGE_GLANCE_OUT)?;
        Ok(())
    })
}

static S_TRAY_QUOTA_ROWS_0: wire::Schema = wire::Schema::Dyn;
static S_TRAY_QUOTA_ROWS_1: wire::Schema = wire::Schema::Dyn;
static S_TRAY_QUOTA_ROWS_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("providerId", wire::Schema::Str16),
    ("name", wire::Schema::Str16),
    ("plan", wire::Schema::Str16),
    (
        "windows",
        wire::Schema::List(&wire::Schema::Obj(&[
            (
                "window",
                wire::Schema::Obj(&[
                    ("windowMinutes", wire::Schema::OptJs(&wire::Schema::Num)),
                    ("usedPercent", wire::Schema::Num),
                    ("resetsAtMs", wire::Schema::OptJs(&wire::Schema::Num)),
                    ("observedAtMs", wire::Schema::OptJs(&wire::Schema::Num)),
                ]),
            ),
            ("remaining", wire::Schema::Num),
        ])),
    ),
]));

/// `trayQuotaRows` over the binary wire.
#[wasm_bindgen(js_name = "trayQuotaRowsWire")]
pub fn tray_quota_rows_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let pool: Value = wire::read_value(r, &S_TRAY_QUOTA_ROWS_0)?;
        let now_ms: Value = wire::read_value(r, &S_TRAY_QUOTA_ROWS_1)?;
        let out = &port::tray_quota_rows(pool, now_ms)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_TRAY_QUOTA_ROWS_OUT.inner())?;
        }
        Ok(())
    })
}

static S_IS_TRAY_ROUTE_0: wire::Schema = wire::Schema::Dyn;

/// `isTrayRoute` over the binary wire.
#[wasm_bindgen(js_name = "isTrayRouteWire")]
pub fn is_tray_route_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let value: Value = wire::read_value(r, &S_IS_TRAY_ROUTE_0)?;
        let out = &port::is_tray_route(value)?;
        wire::write_dyn(w, &serde_json::to_value(out).map_err(|e| e.to_string())?);
        Ok(())
    })
}

static S_PARSE_TRAY_REQUEST_0: wire::Schema = wire::Schema::Dyn;
static S_PARSE_TRAY_REQUEST_OUT: wire::Schema = wire::Schema::Dyn;

/// `parseTrayRequest` over the binary wire.
#[wasm_bindgen(js_name = "parseTrayRequestWire")]
pub fn parse_tray_request_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let value: Value = wire::read_value(r, &S_PARSE_TRAY_REQUEST_0)?;
        let out = &port::parse_tray_request(value)?;
        wire::write_value(w, out, &S_PARSE_TRAY_REQUEST_OUT)?;
        Ok(())
    })
}
