//! Unified Rust & WASM core for pure logic inside `assets/inject/renderer-inject`.

#![forbid(unsafe_code)]

pub mod model_catalog;
pub mod prompt_args;
pub mod security_sanitizers;
pub mod statsig_overrides;

pub use model_catalog::*;
pub use prompt_args::*;
pub use security_sanitizers::*;
pub use statsig_overrides::*;

use indexmap::IndexMap;
use serde_json::Value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = "promptArgsSplitJson")]
pub fn prompt_args_split_wasm(text: &str) -> Result<String, JsError> {
    serde_json::to_string(&prompt_args_split(text)).map_err(|e| JsError::new(&e.to_string()))
}

#[wasm_bindgen(js_name = "promptArgsExpandJson")]
pub fn prompt_args_expand_wasm(
    prompt_id: &str,
    content: &str,
    arg_text: &str,
) -> Result<String, JsError> {
    serde_json::to_string(&prompt_args_expand(prompt_id, content, arg_text))
        .map_err(|e| JsError::new(&e.to_string()))
}

#[wasm_bindgen(js_name = "statsigEffectiveGatesJson")]
pub fn statsig_effective_gates_wasm(
    defaults_json: &str,
    backend_settings_json: &str,
) -> Result<String, JsError> {
    let defaults: Value =
        serde_json::from_str(defaults_json).map_err(|e| JsError::new(&e.to_string()))?;
    let settings: Value =
        serde_json::from_str(backend_settings_json).map_err(|e| JsError::new(&e.to_string()))?;
    let user: IndexMap<String, String> = statsig_user_gate_overrides(&settings);
    serde_json::to_string(&statsig_effective_gates(&defaults, &user))
        .map_err(|e| JsError::new(&e.to_string()))
}

#[wasm_bindgen(js_name = "statsigMergeObjectJson")]
pub fn statsig_merge_object_wasm(
    target_json: &str,
    overrides_json: &str,
) -> Result<String, JsError> {
    let target: Value =
        serde_json::from_str(target_json).map_err(|e| JsError::new(&e.to_string()))?;
    let overrides: Value =
        serde_json::from_str(overrides_json).map_err(|e| JsError::new(&e.to_string()))?;
    serde_json::to_string(&statsig_merge_object_value(&target, &overrides))
        .map_err(|e| JsError::new(&e.to_string()))
}

#[wasm_bindgen(js_name = "modelCatalogIdentityJson")]
pub fn model_catalog_identity_wasm(settings_json: &str) -> Result<String, JsError> {
    let settings: Value =
        serde_json::from_str(settings_json).map_err(|e| JsError::new(&e.to_string()))?;
    Ok(model_catalog_identity(&settings))
}

#[wasm_bindgen(js_name = "buildModelDescriptorJson")]
pub fn build_model_descriptor_wasm(
    catalog_json: &str,
    model_name: &str,
) -> Result<String, JsError> {
    let catalog: Value =
        serde_json::from_str(catalog_json).map_err(|e| JsError::new(&e.to_string()))?;
    serde_json::to_string(&build_model_descriptor(&catalog, model_name))
        .map_err(|e| JsError::new(&e.to_string()))
}

#[wasm_bindgen(js_name = "applyModelMetadataJson")]
pub fn apply_model_metadata_wasm(
    descriptor_json: &str,
    catalog_json: &str,
    model_name: &str,
) -> Result<String, JsError> {
    let mut descriptor: Value =
        serde_json::from_str(descriptor_json).map_err(|e| JsError::new(&e.to_string()))?;
    let catalog: Value =
        serde_json::from_str(catalog_json).map_err(|e| JsError::new(&e.to_string()))?;
    let changed = apply_model_metadata_to_descriptor(&mut descriptor, &catalog, model_name);
    serde_json::to_string(&serde_json::json!({
        "changed": changed,
        "descriptor": descriptor,
    }))
    .map_err(|e| JsError::new(&e.to_string()))
}

#[wasm_bindgen(js_name = "parsePickerGroupSuffixJson")]
pub fn parse_picker_group_suffix_wasm(label: &str) -> Result<String, JsError> {
    let (name, provider) = parse_picker_group_suffix(label);
    serde_json::to_string(&serde_json::json!({ "name": name, "provider": provider }))
        .map_err(|e| JsError::new(&e.to_string()))
}

#[wasm_bindgen(js_name = "isExtensionRouteAllowed")]
pub fn is_extension_route_allowed_wasm(route: &str) -> bool {
    is_extension_route_allowed(route)
}

#[wasm_bindgen(js_name = "escapeHtml")]
pub fn escape_html_wasm(raw: &str) -> String {
    escape_html(raw)
}

#[wasm_bindgen(js_name = "escapeCssIdent")]
pub fn escape_css_ident_wasm(raw: &str) -> String {
    escape_css_ident(raw)
}

#[wasm_bindgen(js_name = "sanitizeSvgIcon")]
pub fn sanitize_svg_icon_wasm(svg: &str) -> Option<String> {
    sanitize_svg_icon(svg)
}

#[wasm_bindgen(js_name = "sanitizeExternalHttpUrl")]
pub fn sanitize_external_http_url_wasm(raw: &str) -> Option<String> {
    sanitize_external_http_url(raw)
}
