//! Unified Rust/WASM library covering remaining manager domain units and `js_only` helpers.

#![forbid(unsafe_code)]

pub mod js_only_bridges;
pub mod lifted;
pub mod relay_editing;
pub mod static_units;

pub use js_only_bridges::*;
pub use lifted::*;
pub use manager_common_rs as common;
pub use static_units::*;

use serde_json::Value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = "presetsJson")]
pub fn presets_json_wasm() -> String {
    manager_common_rs::assets_data::PRESETS_JSON.to_string()
}

#[wasm_bindgen(js_name = "i18nTranslateEn")]
pub fn i18n_translate_en_wasm(zh: &str) -> String {
    translate_en(zh)
}

#[wasm_bindgen(js_name = "i18nTranslateTemplateEnJson")]
pub fn i18n_translate_template_en_wasm(key: &str, args_json: &str) -> Result<String, JsError> {
    let args: Vec<Value> =
        serde_json::from_str(args_json).map_err(|e| JsError::new(&e.to_string()))?;
    Ok(translate_template_en(key, &args))
}

#[wasm_bindgen(js_name = "mcpPresetsJson")]
pub fn mcp_presets_json_wasm() -> Result<String, JsError> {
    serde_json::to_string(&mcp_presets()).map_err(|e| JsError::new(&e.to_string()))
}

#[wasm_bindgen(js_name = "mcpPresetTomlBody")]
pub fn mcp_preset_toml_body_wasm(id: &str, windows: bool) -> Option<String> {
    mcp_preset_toml_body(id, windows)
}

#[wasm_bindgen(js_name = "vlmTestTranslationZh")]
pub fn vlm_test_translation_zh_wasm(
    status: &str,
    http_code: Option<i32>,
    duration_ms: f64,
) -> String {
    vlm_test_translation_zh(status, http_code.map(i64::from), duration_ms)
}

#[wasm_bindgen(js_name = "usesMacOverlayTitlebar")]
pub fn uses_mac_overlay_titlebar_wasm(
    search: &str,
    is_tauri: bool,
    platform: &str,
    user_agent: &str,
) -> bool {
    uses_mac_overlay_titlebar(search, is_tauri, platform, user_agent)
}

#[wasm_bindgen(js_name = "brandIconAssetFilename")]
pub fn brand_icon_asset_filename_wasm(key: &str) -> Option<String> {
    brand_icon_asset_filename(key)
}

#[wasm_bindgen(js_name = "cnClassesJson")]
pub fn cn_classes_json_wasm(inputs_json: &str) -> Result<String, JsError> {
    let items: Vec<String> =
        serde_json::from_str(inputs_json).map_err(|e| JsError::new(&e.to_string()))?;
    let refs: Vec<&str> = items.iter().map(String::as_str).collect();
    Ok(cn_classes(&refs))
}

#[wasm_bindgen(js_name = "unlistedModelsJson")]
pub fn unlisted_models_wasm(payload_json: &str) -> Result<String, JsError> {
    let payload: Value =
        serde_json::from_str(payload_json).map_err(|e| JsError::new(&e.to_string()))?;
    let mut out: Vec<String> = unlisted_models(&payload).into_iter().collect();
    out.sort();
    serde_json::to_string(&out).map_err(|e| JsError::new(&e.to_string()))
}
