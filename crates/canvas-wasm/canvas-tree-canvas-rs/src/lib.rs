//! JS bindings for `tree-canvas.mjs`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;

/// `fitCanvas`.
fn fit_canvas_inner(layout: String, width: String, height: String) -> Result<String, String> {
    let layout = crate::js::from_json(&layout)?;
    let width = crate::js::from_json(&width)?;
    let height = crate::js::from_json(&height)?;
    let out = port::fit_canvas(layout, width, height)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "fitCanvas")]
pub fn fit_canvas(layout: String, width: String, height: String) -> Result<String, JsError> {
    fit_canvas_inner(layout, width, height).map_err(|e| JsError::new(&e))
}

/// `zoomCanvas`.
fn zoom_canvas_inner(
    camera: String,
    scale: String,
    x: String,
    y: String,
) -> Result<String, String> {
    let camera = crate::js::from_json(&camera)?;
    let scale = crate::js::from_json(&scale)?;
    let x = crate::js::from_json(&x)?;
    let y = crate::js::from_json(&y)?;
    let out = port::zoom_canvas(camera, scale, x, y)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "zoomCanvas")]
pub fn zoom_canvas(camera: String, scale: String, x: String, y: String) -> Result<String, JsError> {
    zoom_canvas_inner(camera, scale, x, y).map_err(|e| JsError::new(&e))
}

/// `canvasViewport`.
fn canvas_viewport_inner(
    layout: String,
    camera: String,
    width: String,
    height: String,
    limit: Option<f64>,
) -> Result<String, String> {
    let layout = crate::js::from_json(&layout)?;
    let camera = crate::js::from_json(&camera)?;
    let width = crate::js::from_json(&width)?;
    let height = crate::js::from_json(&height)?;
    let out = port::canvas_viewport(layout, camera, width, height, limit)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "canvasViewport")]
pub fn canvas_viewport(
    layout: String,
    camera: String,
    width: String,
    height: String,
    limit: Option<f64>,
) -> Result<String, JsError> {
    canvas_viewport_inner(layout, camera, width, height, limit).map_err(|e| JsError::new(&e))
}

/// `canvasCardsMarkup`.
fn canvas_cards_markup_inner(
    cards: String,
    collapsed: String,
    selected: String,
    current_node_id: String,
) -> Result<String, String> {
    let cards = crate::js::from_json(&cards)?;
    let collapsed = crate::js::from_json(&collapsed)?;
    let selected = crate::js::from_json(&selected)?;
    let current_node_id = crate::js::from_json(&current_node_id)?;
    let out = port::canvas_cards_markup(cards, collapsed, selected, current_node_id)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "canvasCardsMarkup")]
pub fn canvas_cards_markup(
    cards: String,
    collapsed: String,
    selected: String,
    current_node_id: String,
) -> Result<String, JsError> {
    canvas_cards_markup_inner(cards, collapsed, selected, current_node_id)
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

static S_FIT_CANVAS_0: wire::Schema = wire::Schema::Dyn;
static S_FIT_CANVAS_1: wire::Schema = wire::Schema::Dyn;
static S_FIT_CANVAS_2: wire::Schema = wire::Schema::Dyn;
static S_FIT_CANVAS_OUT: wire::Schema = wire::Schema::Dyn;

/// `fitCanvas` over the binary wire.
#[wasm_bindgen(js_name = "fitCanvasWire")]
pub fn fit_canvas_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let layout: Value = wire::read_value(r, &S_FIT_CANVAS_0)?;
        let width: Value = wire::read_value(r, &S_FIT_CANVAS_1)?;
        let height: Value = wire::read_value(r, &S_FIT_CANVAS_2)?;
        let out = &port::fit_canvas(layout, width, height)?;
        wire::write_value(w, out, &S_FIT_CANVAS_OUT)?;
        Ok(())
    })
}

static S_ZOOM_CANVAS_0: wire::Schema = wire::Schema::Dyn;
static S_ZOOM_CANVAS_1: wire::Schema = wire::Schema::Dyn;
static S_ZOOM_CANVAS_2: wire::Schema = wire::Schema::Dyn;
static S_ZOOM_CANVAS_3: wire::Schema = wire::Schema::Dyn;
static S_ZOOM_CANVAS_OUT: wire::Schema = wire::Schema::Dyn;

/// `zoomCanvas` over the binary wire.
#[wasm_bindgen(js_name = "zoomCanvasWire")]
pub fn zoom_canvas_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let camera: Value = wire::read_value(r, &S_ZOOM_CANVAS_0)?;
        let scale: Value = wire::read_value(r, &S_ZOOM_CANVAS_1)?;
        let x: Value = wire::read_value(r, &S_ZOOM_CANVAS_2)?;
        let y: Value = wire::read_value(r, &S_ZOOM_CANVAS_3)?;
        let out = &port::zoom_canvas(camera, scale, x, y)?;
        wire::write_value(w, out, &S_ZOOM_CANVAS_OUT)?;
        Ok(())
    })
}

static S_CANVAS_VIEWPORT_0: wire::Schema = wire::Schema::Dyn;
static S_CANVAS_VIEWPORT_1: wire::Schema = wire::Schema::Dyn;
static S_CANVAS_VIEWPORT_2: wire::Schema = wire::Schema::Dyn;
static S_CANVAS_VIEWPORT_3: wire::Schema = wire::Schema::Dyn;
static S_CANVAS_VIEWPORT_OUT: wire::Schema = wire::Schema::Dyn;

/// `canvasViewport` over the binary wire.
#[wasm_bindgen(js_name = "canvasViewportWire")]
pub fn canvas_viewport_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let layout: Value = wire::read_value(r, &S_CANVAS_VIEWPORT_0)?;
        let camera: Value = wire::read_value(r, &S_CANVAS_VIEWPORT_1)?;
        let width: Value = wire::read_value(r, &S_CANVAS_VIEWPORT_2)?;
        let height: Value = wire::read_value(r, &S_CANVAS_VIEWPORT_3)?;
        let limit: Option<f64> = r.opt_js(|r| r.f64())?;
        let out = &port::canvas_viewport(layout, camera, width, height, limit)?;
        wire::write_value(w, out, &S_CANVAS_VIEWPORT_OUT)?;
        Ok(())
    })
}

static S_CANVAS_CARDS_MARKUP_0: wire::Schema = wire::Schema::Dyn;
static S_CANVAS_CARDS_MARKUP_1: wire::Schema = wire::Schema::Dyn;
static S_CANVAS_CARDS_MARKUP_2: wire::Schema = wire::Schema::Dyn;
static S_CANVAS_CARDS_MARKUP_3: wire::Schema = wire::Schema::Dyn;
static S_CANVAS_CARDS_MARKUP_OUT: wire::Schema = wire::Schema::Dyn;

/// `canvasCardsMarkup` over the binary wire.
#[wasm_bindgen(js_name = "canvasCardsMarkupWire")]
pub fn canvas_cards_markup_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let cards: Value = wire::read_value(r, &S_CANVAS_CARDS_MARKUP_0)?;
        let collapsed: Value = wire::read_value(r, &S_CANVAS_CARDS_MARKUP_1)?;
        let selected: Value = wire::read_value(r, &S_CANVAS_CARDS_MARKUP_2)?;
        let current_node_id: Value = wire::read_value(r, &S_CANVAS_CARDS_MARKUP_3)?;
        let out = &port::canvas_cards_markup(cards, collapsed, selected, current_node_id)?;
        wire::write_value(w, out, &S_CANVAS_CARDS_MARKUP_OUT)?;
        Ok(())
    })
}
