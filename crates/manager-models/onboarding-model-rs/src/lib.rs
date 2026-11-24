//! JS bindings for `src/onboarding-model.ts`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;

/// `tourSteps`.
fn tour_steps_inner(
    solo_name: Option<String>,
    enhance_only: Option<bool>,
) -> Result<String, String> {
    let out = port::tour_steps(solo_name, enhance_only)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "tourSteps")]
pub fn tour_steps(
    solo_name: Option<String>,
    enhance_only: Option<bool>,
) -> Result<String, JsError> {
    tour_steps_inner(solo_name, enhance_only).map_err(|e| JsError::new(&e))
}

/// `shouldShowTour`.
fn should_show_tour_inner(stored: Option<String>) -> Result<bool, String> {
    let out = port::should_show_tour(stored)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "shouldShowTour")]
pub fn should_show_tour(stored: Option<String>) -> Result<bool, JsError> {
    should_show_tour_inner(stored).map_err(|e| JsError::new(&e))
}

/// `placeCard`.
fn place_card_inner(
    target: String,
    card: String,
    viewport: String,
    gap: Option<f64>,
) -> Result<String, String> {
    let target = crate::js::from_json(&target)?;
    let card = crate::js::from_json(&card)?;
    let viewport = crate::js::from_json(&viewport)?;
    let out = port::place_card(target, card, viewport, gap)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "placeCard")]
pub fn place_card(
    target: String,
    card: String,
    viewport: String,
    gap: Option<f64>,
) -> Result<String, JsError> {
    place_card_inner(target, card, viewport, gap).map_err(|e| JsError::new(&e))
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

static S_TOUR_STEPS_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("target", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("title", wire::Schema::Str16),
    ("body", wire::Schema::Str16),
    (
        "args",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Str16)),
    ),
    ("plainTitle", wire::Schema::OptJs(&wire::Schema::Bool)),
]));

/// `tourSteps` over the binary wire.
#[wasm_bindgen(js_name = "tourStepsWire")]
pub fn tour_steps_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let solo_name: Option<String> = r.opt_js(|r| r.string16())?;
        let enhance_only: Option<bool> = r.opt_js(|r| r.bool())?;
        let out = &port::tour_steps(solo_name, enhance_only)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_TOUR_STEPS_OUT.inner())?;
        }
        Ok(())
    })
}

static S_PLACE_CARD_0: wire::Schema = wire::Schema::Dyn;
static S_PLACE_CARD_1: wire::Schema = wire::Schema::Dyn;
static S_PLACE_CARD_2: wire::Schema = wire::Schema::Dyn;
static S_PLACE_CARD_OUT: wire::Schema = wire::Schema::Obj(&[
    ("left", wire::Schema::Num),
    ("top", wire::Schema::Num),
    ("side", wire::Schema::Str16),
]);

/// `placeCard` over the binary wire.
#[wasm_bindgen(js_name = "placeCardWire")]
pub fn place_card_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let target: serde_json::Value = wire::read_value(r, &S_PLACE_CARD_0)?;
        let card: serde_json::Value = wire::read_value(r, &S_PLACE_CARD_1)?;
        let viewport: serde_json::Value = wire::read_value(r, &S_PLACE_CARD_2)?;
        let gap: Option<f64> = r.opt_js(|r| r.f64())?;
        let out = &port::place_card(target, card, viewport, gap)?;
        wire::write_value(w, out, &S_PLACE_CARD_OUT)?;
        Ok(())
    })
}
