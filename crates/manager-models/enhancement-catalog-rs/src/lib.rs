//! JS bindings for `src/enhancement-catalog.ts`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;

/// `enhancementPlatformFromUserAgent`.
fn enhancement_platform_from_user_agent_inner(user_agent: String) -> Result<String, String> {
    let out = port::enhancement_platform_from_user_agent(user_agent)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "enhancementPlatformFromUserAgent")]
pub fn enhancement_platform_from_user_agent(user_agent: String) -> Result<String, JsError> {
    enhancement_platform_from_user_agent_inner(user_agent).map_err(|e| JsError::new(&e))
}

/// `enhancementItem`.
fn enhancement_item_inner(key: String, catalog: String) -> Result<String, String> {
    let catalog = crate::js::from_json(&catalog)?;
    let out = port::enhancement_item(key, catalog)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "enhancementItem")]
pub fn enhancement_item(key: String, catalog: String) -> Result<String, JsError> {
    enhancement_item_inner(key, catalog).map_err(|e| JsError::new(&e))
}

/// `enhancementGroup`.
fn enhancement_group_inner(id: String, catalog: String) -> Result<String, String> {
    let catalog = crate::js::from_json(&catalog)?;
    let out = port::enhancement_group(id, catalog)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "enhancementGroup")]
pub fn enhancement_group(id: String, catalog: String) -> Result<String, JsError> {
    enhancement_group_inner(id, catalog).map_err(|e| JsError::new(&e))
}

/// `isEnhancementItemVisible`.
fn is_enhancement_item_visible_inner(
    item: String,
    surface: String,
    platform: String,
) -> Result<bool, String> {
    let item = crate::js::from_json(&item)?;
    let out = port::is_enhancement_item_visible(item, surface, platform)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "isEnhancementItemVisible")]
pub fn is_enhancement_item_visible(
    item: String,
    surface: String,
    platform: String,
) -> Result<bool, JsError> {
    is_enhancement_item_visible_inner(item, surface, platform).map_err(|e| JsError::new(&e))
}

/// `managerBuiltinCards`.
fn manager_builtin_cards_inner(
    group_id: String,
    platform: String,
    catalog: String,
) -> Result<String, String> {
    let catalog = crate::js::from_json(&catalog)?;
    let out = port::manager_builtin_cards(group_id, platform, catalog)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "managerBuiltinCards")]
pub fn manager_builtin_cards(
    group_id: String,
    platform: String,
    catalog: String,
) -> Result<String, JsError> {
    manager_builtin_cards_inner(group_id, platform, catalog).map_err(|e| JsError::new(&e))
}

/// `builtinSettingValue`.
fn builtin_setting_value_inner(settings: String, path: String) -> Result<String, String> {
    let settings = crate::js::from_json(&settings)?;
    let out = port::builtin_setting_value(settings, path)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "builtinSettingValue")]
pub fn builtin_setting_value(settings: String, path: String) -> Result<String, JsError> {
    builtin_setting_value_inner(settings, path).map_err(|e| JsError::new(&e))
}

/// `managerStandaloneGroup`.
fn manager_standalone_group_inner(catalog: String) -> Result<String, String> {
    let catalog = crate::js::from_json(&catalog)?;
    let out = port::manager_standalone_group(catalog)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "managerStandaloneGroup")]
pub fn manager_standalone_group(catalog: String) -> Result<String, JsError> {
    manager_standalone_group_inner(catalog).map_err(|e| JsError::new(&e))
}

/// `isEnhancementItemMasterGated`.
fn is_enhancement_item_master_gated_inner(item: String) -> Result<bool, String> {
    let item = crate::js::from_json(&item)?;
    let out = port::is_enhancement_item_master_gated(item)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "isEnhancementItemMasterGated")]
pub fn is_enhancement_item_master_gated(item: String) -> Result<bool, JsError> {
    is_enhancement_item_master_gated_inner(item).map_err(|e| JsError::new(&e))
}

/// `enhancementCatalogStrings`.
fn enhancement_catalog_strings_inner(catalog: String) -> Result<Vec<String>, String> {
    let catalog = crate::js::from_json(&catalog)?;
    let out = port::enhancement_catalog_strings(catalog)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "enhancementCatalogStrings")]
pub fn enhancement_catalog_strings(catalog: String) -> Result<Vec<String>, JsError> {
    enhancement_catalog_strings_inner(catalog).map_err(|e| JsError::new(&e))
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

static S_ENHANCEMENT_ITEM_1: wire::Schema = wire::Schema::Dyn;
static S_ENHANCEMENT_ITEM_OUT: wire::Schema = wire::Schema::Obj(&[
    ("key", wire::Schema::Str16),
    ("builtin", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("parent", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("builtinOn", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("builtinOff", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("restart", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("replaceable", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("group", wire::Schema::Str16),
    ("control", wire::Schema::Str16),
    ("title", wire::Schema::Str16),
    ("description", wire::Schema::Str16),
    (
        "descriptionWhenEnhancementsOn",
        wire::Schema::OptJs(&wire::Schema::Str16),
    ),
    (
        "options",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Obj(&[
            ("value", wire::Schema::Str16),
            ("label", wire::Schema::Str16),
        ]))),
    ),
    (
        "platforms",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Str16)),
    ),
    ("masterGated", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("surfaces", wire::Schema::List(&wire::Schema::Str16)),
    (
        "codexInputs",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Obj(&[
            ("id", wire::Schema::Str16),
            ("label", wire::Schema::Str16),
            ("hint", wire::Schema::OptJs(&wire::Schema::Str16)),
        ]))),
    ),
]);

/// `enhancementItem` over the binary wire.
#[wasm_bindgen(js_name = "enhancementItemWire")]
pub fn enhancement_item_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let key: String = r.string16()?;
        let catalog: Value = wire::read_value(r, &S_ENHANCEMENT_ITEM_1)?;
        let out = &port::enhancement_item(key, catalog)?;
        wire::write_value(w, out, &S_ENHANCEMENT_ITEM_OUT)?;
        Ok(())
    })
}

static S_ENHANCEMENT_GROUP_1: wire::Schema = wire::Schema::Dyn;
static S_ENHANCEMENT_GROUP_OUT: wire::Schema = wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("title", wire::Schema::Str16),
    ("description", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("managerTab", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("standalone", wire::Schema::OptJs(&wire::Schema::Bool)),
]);

/// `enhancementGroup` over the binary wire.
#[wasm_bindgen(js_name = "enhancementGroupWire")]
pub fn enhancement_group_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let id: String = r.string16()?;
        let catalog: Value = wire::read_value(r, &S_ENHANCEMENT_GROUP_1)?;
        let out = &port::enhancement_group(id, catalog)?;
        wire::write_value(w, out, &S_ENHANCEMENT_GROUP_OUT)?;
        Ok(())
    })
}

static S_IS_ENHANCEMENT_ITEM_VISIBLE_0: wire::Schema = wire::Schema::Dyn;

/// `isEnhancementItemVisible` over the binary wire.
#[wasm_bindgen(js_name = "isEnhancementItemVisibleWire")]
pub fn is_enhancement_item_visible_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let item: Value = wire::read_value(r, &S_IS_ENHANCEMENT_ITEM_VISIBLE_0)?;
        let surface: String = r.string16()?;
        let platform: String = r.string16()?;
        let out = &port::is_enhancement_item_visible(item, surface, platform)?;
        w.bool(*out);
        Ok(())
    })
}

static S_MANAGER_BUILTIN_CARDS_2: wire::Schema = wire::Schema::Dyn;
static S_MANAGER_BUILTIN_CARDS_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    (
        "item",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("key", wire::Schema::Str16),
            ("builtin", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("parent", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("builtinOn", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("builtinOff", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("restart", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("replaceable", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("group", wire::Schema::Str16),
            ("control", wire::Schema::Str16),
            ("title", wire::Schema::Str16),
            ("description", wire::Schema::Str16),
            (
                "descriptionWhenEnhancementsOn",
                wire::Schema::OptJs(&wire::Schema::Str16),
            ),
            (
                "options",
                wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Dyn)),
            ),
            (
                "platforms",
                wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Dyn)),
            ),
            ("masterGated", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("surfaces", wire::Schema::List(&wire::Schema::Dyn)),
            (
                "codexInputs",
                wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Dyn)),
            ),
        ])),
    ),
    (
        "entry",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("id", wire::Schema::Str16),
            ("title", wire::Schema::Str16),
            ("description", wire::Schema::Str16),
            ("group", wire::Schema::Str16),
            ("setting", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("replaceable", wire::Schema::OptJs(&wire::Schema::Bool)),
        ])),
    ),
    (
        "params",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("key", wire::Schema::Str16),
            ("builtin", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("parent", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("builtinOn", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("builtinOff", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("restart", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("replaceable", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("group", wire::Schema::Str16),
            ("control", wire::Schema::Dyn),
            ("title", wire::Schema::Str16),
            ("description", wire::Schema::Str16),
            (
                "descriptionWhenEnhancementsOn",
                wire::Schema::OptJs(&wire::Schema::Str16),
            ),
            (
                "options",
                wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Dyn)),
            ),
            (
                "platforms",
                wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Dyn)),
            ),
            ("masterGated", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("surfaces", wire::Schema::List(&wire::Schema::Dyn)),
            (
                "codexInputs",
                wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Dyn)),
            ),
        ])),
    ),
]));

/// `managerBuiltinCards` over the binary wire.
#[wasm_bindgen(js_name = "managerBuiltinCardsWire")]
pub fn manager_builtin_cards_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let group_id: String = r.string16()?;
        let platform: String = r.string16()?;
        let catalog: Value = wire::read_value(r, &S_MANAGER_BUILTIN_CARDS_2)?;
        let out = &port::manager_builtin_cards(group_id, platform, catalog)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_MANAGER_BUILTIN_CARDS_OUT.inner())?;
        }
        Ok(())
    })
}

static S_BUILTIN_SETTING_VALUE_0: wire::Schema = wire::Schema::Dyn;
static S_BUILTIN_SETTING_VALUE_OUT: wire::Schema = wire::Schema::Dyn;

/// `builtinSettingValue` over the binary wire.
#[wasm_bindgen(js_name = "builtinSettingValueWire")]
pub fn builtin_setting_value_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let settings: Value = wire::read_value(r, &S_BUILTIN_SETTING_VALUE_0)?;
        let path: String = r.string16()?;
        let out = &port::builtin_setting_value(settings, path)?;
        wire::write_value(w, out, &S_BUILTIN_SETTING_VALUE_OUT)?;
        Ok(())
    })
}

static S_MANAGER_STANDALONE_GROUP_0: wire::Schema = wire::Schema::Dyn;
static S_MANAGER_STANDALONE_GROUP_OUT: wire::Schema = wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("title", wire::Schema::Str16),
    ("description", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("managerTab", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("standalone", wire::Schema::OptJs(&wire::Schema::Bool)),
]);

/// `managerStandaloneGroup` over the binary wire.
#[wasm_bindgen(js_name = "managerStandaloneGroupWire")]
pub fn manager_standalone_group_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let catalog: Value = wire::read_value(r, &S_MANAGER_STANDALONE_GROUP_0)?;
        let out = &port::manager_standalone_group(catalog)?;
        wire::write_value(w, out, &S_MANAGER_STANDALONE_GROUP_OUT)?;
        Ok(())
    })
}

static S_IS_ENHANCEMENT_ITEM_MASTER_GATED_0: wire::Schema = wire::Schema::Dyn;

/// `isEnhancementItemMasterGated` over the binary wire.
#[wasm_bindgen(js_name = "isEnhancementItemMasterGatedWire")]
pub fn is_enhancement_item_master_gated_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let item: Value = wire::read_value(r, &S_IS_ENHANCEMENT_ITEM_MASTER_GATED_0)?;
        let out = &port::is_enhancement_item_master_gated(item)?;
        w.bool(*out);
        Ok(())
    })
}

static S_ENHANCEMENT_CATALOG_STRINGS_0: wire::Schema = wire::Schema::Dyn;

/// `enhancementCatalogStrings` over the binary wire.
#[wasm_bindgen(js_name = "enhancementCatalogStringsWire")]
pub fn enhancement_catalog_strings_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let catalog: Value = wire::read_value(r, &S_ENHANCEMENT_CATALOG_STRINGS_0)?;
        let out = &port::enhancement_catalog_strings(catalog)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            w.string16(x0);
        }
        Ok(())
    })
}
