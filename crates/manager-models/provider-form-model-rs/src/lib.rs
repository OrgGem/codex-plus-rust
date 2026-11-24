//! JS bindings for `src/provider-form-model.ts`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;

/// `draftModels`.
fn draft_models_inner(draft: String) -> Result<Vec<String>, String> {
    let draft = crate::js::from_json(&draft)?;
    let out = port::draft_models(draft)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "draftModels")]
pub fn draft_models(draft: String) -> Result<Vec<String>, JsError> {
    draft_models_inner(draft).map_err(|e| JsError::new(&e))
}

/// `defaultModel`.
fn default_model_inner(draft: String) -> Result<String, String> {
    let draft = crate::js::from_json(&draft)?;
    let out = port::default_model(draft)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "defaultModel")]
pub fn default_model(draft: String) -> Result<String, JsError> {
    default_model_inner(draft).map_err(|e| JsError::new(&e))
}

/// `withModels`.
fn with_models_inner(draft: String, models: Vec<String>) -> Result<String, String> {
    let draft = crate::js::from_json(&draft)?;
    let out = port::with_models(draft, models)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "withModels")]
pub fn with_models(draft: String, models: Vec<String>) -> Result<String, JsError> {
    with_models_inner(draft, models).map_err(|e| JsError::new(&e))
}

/// `addModel`.
fn add_model_inner(draft: String, model: String) -> Result<String, String> {
    let draft = crate::js::from_json(&draft)?;
    let out = port::add_model(draft, model)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "addModel")]
pub fn add_model(draft: String, model: String) -> Result<String, JsError> {
    add_model_inner(draft, model).map_err(|e| JsError::new(&e))
}

/// `removeModel`.
fn remove_model_inner(draft: String, model: String) -> Result<String, String> {
    let draft = crate::js::from_json(&draft)?;
    let out = port::remove_model(draft, model)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "removeModel")]
pub fn remove_model(draft: String, model: String) -> Result<String, JsError> {
    remove_model_inner(draft, model).map_err(|e| JsError::new(&e))
}

/// `draftKeys`.
fn draft_keys_inner(draft: String) -> Result<Vec<String>, String> {
    let draft = crate::js::from_json(&draft)?;
    let out = port::draft_keys(draft)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "draftKeys")]
pub fn draft_keys(draft: String) -> Result<Vec<String>, JsError> {
    draft_keys_inner(draft).map_err(|e| JsError::new(&e))
}

/// `withKeys`.
fn with_keys_inner(draft: String, keys: Vec<String>) -> Result<String, String> {
    let draft = crate::js::from_json(&draft)?;
    let out = port::with_keys(draft, keys)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "withKeys")]
pub fn with_keys(draft: String, keys: Vec<String>) -> Result<String, JsError> {
    with_keys_inner(draft, keys).map_err(|e| JsError::new(&e))
}

/// `formatPrice`.
fn format_price_inner(value: f64) -> Result<String, String> {
    let out = port::format_price(value)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "formatPrice")]
pub fn format_price(value: f64) -> Result<String, JsError> {
    format_price_inner(value).map_err(|e| JsError::new(&e))
}

/// `draftWithPrice`.
fn draft_with_price_inner(draft: String, r#match: String) -> Result<String, String> {
    let draft = crate::js::from_json(&draft)?;
    let r#match = crate::js::from_json(&r#match)?;
    let out = port::draft_with_price(draft, r#match)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "draftWithPrice")]
pub fn draft_with_price(draft: String, r#match: String) -> Result<String, JsError> {
    draft_with_price_inner(draft, r#match).map_err(|e| JsError::new(&e))
}

/// `hasPrice`.
fn has_price_inner(draft: String) -> Result<bool, String> {
    let draft = crate::js::from_json(&draft)?;
    let out = port::has_price(draft)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "hasPrice")]
pub fn has_price(draft: String) -> Result<bool, JsError> {
    has_price_inner(draft).map_err(|e| JsError::new(&e))
}

/// `priceRowFrom`.
fn price_row_from_inner(price: String) -> Result<String, String> {
    let price = crate::js::from_json(&price)?;
    let out = port::price_row_from(price)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "priceRowFrom")]
pub fn price_row_from(price: String) -> Result<String, JsError> {
    price_row_from_inner(price).map_err(|e| JsError::new(&e))
}

/// `priceRowFromMatch`.
fn price_row_from_match_inner(r#match: String) -> Result<String, String> {
    let r#match = crate::js::from_json(&r#match)?;
    let out = port::price_row_from_match(r#match)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "priceRowFromMatch")]
pub fn price_row_from_match(r#match: String) -> Result<String, JsError> {
    price_row_from_match_inner(r#match).map_err(|e| JsError::new(&e))
}

/// `priceFromRow`.
fn price_from_row_inner(row: String, currency: String) -> Result<String, String> {
    let row = crate::js::from_json(&row)?;
    let out = port::price_from_row(row, currency)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "priceFromRow")]
pub fn price_from_row(row: String, currency: String) -> Result<String, JsError> {
    price_from_row_inner(row, currency).map_err(|e| JsError::new(&e))
}

/// `priceFromMatch`.
fn price_from_match_inner(r#match: String) -> Result<String, String> {
    let r#match = crate::js::from_json(&r#match)?;
    let out = port::price_from_match(r#match)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "priceFromMatch")]
pub fn price_from_match(r#match: String) -> Result<String, JsError> {
    price_from_match_inner(r#match).map_err(|e| JsError::new(&e))
}

/// `modelPricesFromRows`.
fn model_prices_from_rows_inner(rows: String, currency: String) -> Result<String, String> {
    let rows = crate::js::from_json(&rows)?;
    let out = port::model_prices_from_rows(rows, currency)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "modelPricesFromRows")]
pub fn model_prices_from_rows(rows: String, currency: String) -> Result<String, JsError> {
    model_prices_from_rows_inner(rows, currency).map_err(|e| JsError::new(&e))
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

static S_DRAFT_MODELS_0: wire::Schema = wire::Schema::Dyn;

/// `draftModels` over the binary wire.
#[wasm_bindgen(js_name = "draftModelsWire")]
pub fn draft_models_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let draft: Value = wire::read_value(r, &S_DRAFT_MODELS_0)?;
        let out = &port::draft_models(draft)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            w.string16(x0);
        }
        Ok(())
    })
}

static S_DEFAULT_MODEL_0: wire::Schema = wire::Schema::Dyn;

/// `defaultModel` over the binary wire.
#[wasm_bindgen(js_name = "defaultModelWire")]
pub fn default_model_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let draft: Value = wire::read_value(r, &S_DEFAULT_MODEL_0)?;
        let out = &port::default_model(draft)?;
        w.string16(out);
        Ok(())
    })
}

static S_WITH_MODELS_0: wire::Schema = wire::Schema::Dyn;
static S_WITH_MODELS_OUT: wire::Schema = wire::Schema::Obj(&[
    ("name", wire::Schema::Str16),
    ("baseUrl", wire::Schema::Str16),
    ("keys", wire::Schema::Str16),
    ("protocol", wire::Schema::Dyn),
    ("model", wire::Schema::Str16),
    ("modelList", wire::Schema::Str16),
    ("priceCurrency", wire::Schema::Str16),
    ("priceInput", wire::Schema::Str16),
    ("priceCached", wire::Schema::Str16),
    ("priceOutput", wire::Schema::Str16),
    ("proxy", wire::Schema::Str16),
    (
        "modelPrices",
        wire::Schema::OptJs(&wire::Schema::Map16(&wire::Schema::Obj(&[
            ("currency", wire::Schema::Str16),
            ("input", wire::Schema::Num),
            ("cachedInput", wire::Schema::Num),
            ("output", wire::Schema::Num),
        ]))),
    ),
    (
        "stripImageGenerationTool",
        wire::Schema::OptJs(&wire::Schema::Bool),
    ),
    (
        "subscriptionPolicy",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
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
        ])),
    ),
    (
        "stripResponsesImageDetail",
        wire::Schema::OptJs(&wire::Schema::Bool),
    ),
    (
        "disableToolSearch",
        wire::Schema::OptJs(&wire::Schema::Bool),
    ),
    (
        "customHeaders",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Obj(&[
            ("key", wire::Schema::Str16),
            ("value", wire::Schema::Str16),
            ("masked", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("savedKey", wire::Schema::OptJs(&wire::Schema::Str16)),
        ]))),
    ),
]);

/// `withModels` over the binary wire.
#[wasm_bindgen(js_name = "withModelsWire")]
pub fn with_models_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let draft: Value = wire::read_value(r, &S_WITH_MODELS_0)?;
        let models: Vec<String> = r.list(|r| r.string16())?;
        let out = &port::with_models(draft, models)?;
        wire::write_value(w, out, &S_WITH_MODELS_OUT)?;
        Ok(())
    })
}

static S_ADD_MODEL_0: wire::Schema = wire::Schema::Dyn;
static S_ADD_MODEL_OUT: wire::Schema = wire::Schema::Obj(&[
    ("name", wire::Schema::Str16),
    ("baseUrl", wire::Schema::Str16),
    ("keys", wire::Schema::Str16),
    ("protocol", wire::Schema::Dyn),
    ("model", wire::Schema::Str16),
    ("modelList", wire::Schema::Str16),
    ("priceCurrency", wire::Schema::Str16),
    ("priceInput", wire::Schema::Str16),
    ("priceCached", wire::Schema::Str16),
    ("priceOutput", wire::Schema::Str16),
    ("proxy", wire::Schema::Str16),
    (
        "modelPrices",
        wire::Schema::OptJs(&wire::Schema::Map16(&wire::Schema::Obj(&[
            ("currency", wire::Schema::Str16),
            ("input", wire::Schema::Num),
            ("cachedInput", wire::Schema::Num),
            ("output", wire::Schema::Num),
        ]))),
    ),
    (
        "stripImageGenerationTool",
        wire::Schema::OptJs(&wire::Schema::Bool),
    ),
    (
        "subscriptionPolicy",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
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
        ])),
    ),
    (
        "stripResponsesImageDetail",
        wire::Schema::OptJs(&wire::Schema::Bool),
    ),
    (
        "disableToolSearch",
        wire::Schema::OptJs(&wire::Schema::Bool),
    ),
    (
        "customHeaders",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Obj(&[
            ("key", wire::Schema::Str16),
            ("value", wire::Schema::Str16),
            ("masked", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("savedKey", wire::Schema::OptJs(&wire::Schema::Str16)),
        ]))),
    ),
]);

/// `addModel` over the binary wire.
#[wasm_bindgen(js_name = "addModelWire")]
pub fn add_model_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let draft: Value = wire::read_value(r, &S_ADD_MODEL_0)?;
        let model: String = r.string16()?;
        let out = &port::add_model(draft, model)?;
        wire::write_value(w, out, &S_ADD_MODEL_OUT)?;
        Ok(())
    })
}

static S_REMOVE_MODEL_0: wire::Schema = wire::Schema::Dyn;
static S_REMOVE_MODEL_OUT: wire::Schema = wire::Schema::Obj(&[
    ("name", wire::Schema::Str16),
    ("baseUrl", wire::Schema::Str16),
    ("keys", wire::Schema::Str16),
    ("protocol", wire::Schema::Dyn),
    ("model", wire::Schema::Str16),
    ("modelList", wire::Schema::Str16),
    ("priceCurrency", wire::Schema::Str16),
    ("priceInput", wire::Schema::Str16),
    ("priceCached", wire::Schema::Str16),
    ("priceOutput", wire::Schema::Str16),
    ("proxy", wire::Schema::Str16),
    (
        "modelPrices",
        wire::Schema::OptJs(&wire::Schema::Map16(&wire::Schema::Obj(&[
            ("currency", wire::Schema::Str16),
            ("input", wire::Schema::Num),
            ("cachedInput", wire::Schema::Num),
            ("output", wire::Schema::Num),
        ]))),
    ),
    (
        "stripImageGenerationTool",
        wire::Schema::OptJs(&wire::Schema::Bool),
    ),
    (
        "subscriptionPolicy",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
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
        ])),
    ),
    (
        "stripResponsesImageDetail",
        wire::Schema::OptJs(&wire::Schema::Bool),
    ),
    (
        "disableToolSearch",
        wire::Schema::OptJs(&wire::Schema::Bool),
    ),
    (
        "customHeaders",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Obj(&[
            ("key", wire::Schema::Str16),
            ("value", wire::Schema::Str16),
            ("masked", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("savedKey", wire::Schema::OptJs(&wire::Schema::Str16)),
        ]))),
    ),
]);

/// `removeModel` over the binary wire.
#[wasm_bindgen(js_name = "removeModelWire")]
pub fn remove_model_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let draft: Value = wire::read_value(r, &S_REMOVE_MODEL_0)?;
        let model: String = r.string16()?;
        let out = &port::remove_model(draft, model)?;
        wire::write_value(w, out, &S_REMOVE_MODEL_OUT)?;
        Ok(())
    })
}

static S_DRAFT_KEYS_0: wire::Schema = wire::Schema::Dyn;

/// `draftKeys` over the binary wire.
#[wasm_bindgen(js_name = "draftKeysWire")]
pub fn draft_keys_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let draft: Value = wire::read_value(r, &S_DRAFT_KEYS_0)?;
        let out = &port::draft_keys(draft)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            w.string16(x0);
        }
        Ok(())
    })
}

static S_WITH_KEYS_0: wire::Schema = wire::Schema::Dyn;
static S_WITH_KEYS_OUT: wire::Schema = wire::Schema::Obj(&[
    ("name", wire::Schema::Str16),
    ("baseUrl", wire::Schema::Str16),
    ("keys", wire::Schema::Str16),
    ("protocol", wire::Schema::Dyn),
    ("model", wire::Schema::Str16),
    ("modelList", wire::Schema::Str16),
    ("priceCurrency", wire::Schema::Str16),
    ("priceInput", wire::Schema::Str16),
    ("priceCached", wire::Schema::Str16),
    ("priceOutput", wire::Schema::Str16),
    ("proxy", wire::Schema::Str16),
    (
        "modelPrices",
        wire::Schema::OptJs(&wire::Schema::Map16(&wire::Schema::Obj(&[
            ("currency", wire::Schema::Str16),
            ("input", wire::Schema::Num),
            ("cachedInput", wire::Schema::Num),
            ("output", wire::Schema::Num),
        ]))),
    ),
    (
        "stripImageGenerationTool",
        wire::Schema::OptJs(&wire::Schema::Bool),
    ),
    (
        "subscriptionPolicy",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
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
        ])),
    ),
    (
        "stripResponsesImageDetail",
        wire::Schema::OptJs(&wire::Schema::Bool),
    ),
    (
        "disableToolSearch",
        wire::Schema::OptJs(&wire::Schema::Bool),
    ),
    (
        "customHeaders",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Obj(&[
            ("key", wire::Schema::Str16),
            ("value", wire::Schema::Str16),
            ("masked", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("savedKey", wire::Schema::OptJs(&wire::Schema::Str16)),
        ]))),
    ),
]);

/// `withKeys` over the binary wire.
#[wasm_bindgen(js_name = "withKeysWire")]
pub fn with_keys_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let draft: Value = wire::read_value(r, &S_WITH_KEYS_0)?;
        let keys: Vec<String> = r.list(|r| r.string16())?;
        let out = &port::with_keys(draft, keys)?;
        wire::write_value(w, out, &S_WITH_KEYS_OUT)?;
        Ok(())
    })
}

static S_DRAFT_WITH_PRICE_0: wire::Schema = wire::Schema::Dyn;
static S_DRAFT_WITH_PRICE_1: wire::Schema = wire::Schema::Dyn;
static S_DRAFT_WITH_PRICE_OUT: wire::Schema = wire::Schema::Obj(&[
    ("name", wire::Schema::Str16),
    ("baseUrl", wire::Schema::Str16),
    ("keys", wire::Schema::Str16),
    ("protocol", wire::Schema::Dyn),
    ("model", wire::Schema::Str16),
    ("modelList", wire::Schema::Str16),
    ("priceCurrency", wire::Schema::Str16),
    ("priceInput", wire::Schema::Str16),
    ("priceCached", wire::Schema::Str16),
    ("priceOutput", wire::Schema::Str16),
    ("proxy", wire::Schema::Str16),
    (
        "modelPrices",
        wire::Schema::OptJs(&wire::Schema::Map16(&wire::Schema::Obj(&[
            ("currency", wire::Schema::Str16),
            ("input", wire::Schema::Num),
            ("cachedInput", wire::Schema::Num),
            ("output", wire::Schema::Num),
        ]))),
    ),
    (
        "stripImageGenerationTool",
        wire::Schema::OptJs(&wire::Schema::Bool),
    ),
    (
        "subscriptionPolicy",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
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
        ])),
    ),
    (
        "stripResponsesImageDetail",
        wire::Schema::OptJs(&wire::Schema::Bool),
    ),
    (
        "disableToolSearch",
        wire::Schema::OptJs(&wire::Schema::Bool),
    ),
    (
        "customHeaders",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Obj(&[
            ("key", wire::Schema::Str16),
            ("value", wire::Schema::Str16),
            ("masked", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("savedKey", wire::Schema::OptJs(&wire::Schema::Str16)),
        ]))),
    ),
]);

/// `draftWithPrice` over the binary wire.
#[wasm_bindgen(js_name = "draftWithPriceWire")]
pub fn draft_with_price_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let draft: Value = wire::read_value(r, &S_DRAFT_WITH_PRICE_0)?;
        let r#match: Value = wire::read_value(r, &S_DRAFT_WITH_PRICE_1)?;
        let out = &port::draft_with_price(draft, r#match)?;
        wire::write_value(w, out, &S_DRAFT_WITH_PRICE_OUT)?;
        Ok(())
    })
}

static S_HAS_PRICE_0: wire::Schema = wire::Schema::Dyn;

/// `hasPrice` over the binary wire.
#[wasm_bindgen(js_name = "hasPriceWire")]
pub fn has_price_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let draft: Value = wire::read_value(r, &S_HAS_PRICE_0)?;
        let out = &port::has_price(draft)?;
        w.bool(*out);
        Ok(())
    })
}

static S_PRICE_ROW_FROM_0: wire::Schema = wire::Schema::Dyn;
static S_PRICE_ROW_FROM_OUT: wire::Schema = wire::Schema::Obj(&[
    ("input", wire::Schema::Str16),
    ("cached", wire::Schema::Str16),
    ("output", wire::Schema::Str16),
]);

/// `priceRowFrom` over the binary wire.
#[wasm_bindgen(js_name = "priceRowFromWire")]
pub fn price_row_from_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let price: Value = wire::read_value(r, &S_PRICE_ROW_FROM_0)?;
        let out = &port::price_row_from(price)?;
        wire::write_value(w, out, &S_PRICE_ROW_FROM_OUT)?;
        Ok(())
    })
}

static S_PRICE_ROW_FROM_MATCH_0: wire::Schema = wire::Schema::Dyn;
static S_PRICE_ROW_FROM_MATCH_OUT: wire::Schema = wire::Schema::Obj(&[
    ("input", wire::Schema::Str16),
    ("cached", wire::Schema::Str16),
    ("output", wire::Schema::Str16),
]);

/// `priceRowFromMatch` over the binary wire.
#[wasm_bindgen(js_name = "priceRowFromMatchWire")]
pub fn price_row_from_match_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let r#match: Value = wire::read_value(r, &S_PRICE_ROW_FROM_MATCH_0)?;
        let out = &port::price_row_from_match(r#match)?;
        wire::write_value(w, out, &S_PRICE_ROW_FROM_MATCH_OUT)?;
        Ok(())
    })
}

static S_PRICE_FROM_ROW_0: wire::Schema = wire::Schema::Dyn;
static S_PRICE_FROM_ROW_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Obj(&[
    ("currency", wire::Schema::Str16),
    ("input", wire::Schema::Num),
    ("cachedInput", wire::Schema::Num),
    ("output", wire::Schema::Num),
]));

/// `priceFromRow` over the binary wire.
#[wasm_bindgen(js_name = "priceFromRowWire")]
pub fn price_from_row_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let row: Value = wire::read_value(r, &S_PRICE_FROM_ROW_0)?;
        let currency: String = r.string16()?;
        let out = &port::price_from_row(row, currency)?;
        wire::write_value(w, out, &S_PRICE_FROM_ROW_OUT)?;
        Ok(())
    })
}

static S_PRICE_FROM_MATCH_0: wire::Schema = wire::Schema::Dyn;
static S_PRICE_FROM_MATCH_OUT: wire::Schema = wire::Schema::Obj(&[
    ("currency", wire::Schema::Str16),
    ("input", wire::Schema::Num),
    ("cachedInput", wire::Schema::Num),
    ("output", wire::Schema::Num),
]);

/// `priceFromMatch` over the binary wire.
#[wasm_bindgen(js_name = "priceFromMatchWire")]
pub fn price_from_match_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let r#match: Value = wire::read_value(r, &S_PRICE_FROM_MATCH_0)?;
        let out = &port::price_from_match(r#match)?;
        wire::write_value(w, out, &S_PRICE_FROM_MATCH_OUT)?;
        Ok(())
    })
}

static S_MODEL_PRICES_FROM_ROWS_0: wire::Schema = wire::Schema::Dyn;
static S_MODEL_PRICES_FROM_ROWS_OUT: wire::Schema = wire::Schema::Map16(&wire::Schema::Obj(&[
    ("currency", wire::Schema::Str16),
    ("input", wire::Schema::Num),
    ("cachedInput", wire::Schema::Num),
    ("output", wire::Schema::Num),
]));

/// `modelPricesFromRows` over the binary wire.
#[wasm_bindgen(js_name = "modelPricesFromRowsWire")]
pub fn model_prices_from_rows_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let rows: Value = wire::read_value(r, &S_MODEL_PRICES_FROM_ROWS_0)?;
        let currency: String = r.string16()?;
        let out = &port::model_prices_from_rows(rows, currency)?;
        wire::write_value(w, out, &S_MODEL_PRICES_FROM_ROWS_OUT)?;
        Ok(())
    })
}
