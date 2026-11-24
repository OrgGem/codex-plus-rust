//! JS bindings for `src/model-routes.ts`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;

/// `setIndependentProxyConfigured`.
fn set_independent_proxy_configured_inner(
    profile_id: String,
    configured: bool,
) -> Result<(), String> {
    port::set_independent_proxy_configured(profile_id, configured)?;
    Ok(())
}

#[wasm_bindgen(js_name = "setIndependentProxyConfigured")]
pub fn set_independent_proxy_configured(
    profile_id: String,
    configured: bool,
) -> Result<(), JsError> {
    set_independent_proxy_configured_inner(profile_id, configured).map_err(|e| JsError::new(&e))
}

/// `independentProxyEnabled`.
fn independent_proxy_enabled_inner(profile: String) -> Result<bool, String> {
    let profile = crate::js::from_json(&profile)?;
    let out = port::independent_proxy_enabled(profile)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "independentProxyEnabled")]
pub fn independent_proxy_enabled(profile: String) -> Result<bool, JsError> {
    independent_proxy_enabled_inner(profile).map_err(|e| JsError::new(&e))
}

/// `normalizeRelayModelRoutes`.
fn normalize_relay_model_routes_inner(routes: String) -> Result<String, String> {
    let routes = crate::js::from_json(&routes)?;
    let out = port::normalize_relay_model_routes(routes)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "normalizeRelayModelRoutes")]
pub fn normalize_relay_model_routes(routes: String) -> Result<String, JsError> {
    normalize_relay_model_routes_inner(routes).map_err(|e| JsError::new(&e))
}

/// `findRelayModelRouteIssue`.
fn find_relay_model_route_issue_inner(
    sources: String,
    all_profiles: String,
) -> Result<String, String> {
    let sources = crate::js::from_json(&sources)?;
    let all_profiles = crate::js::from_json(&all_profiles)?;
    let out = port::find_relay_model_route_issue(sources, all_profiles)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "findRelayModelRouteIssue")]
pub fn find_relay_model_route_issue(
    sources: String,
    all_profiles: String,
) -> Result<String, JsError> {
    find_relay_model_route_issue_inner(sources, all_profiles).map_err(|e| JsError::new(&e))
}

/// `webSearchHistoryCompatEnabled`.
fn web_search_history_compat_enabled_inner(profile: String) -> Result<bool, String> {
    let profile = crate::js::from_json(&profile)?;
    let out = port::web_search_history_compat_enabled(profile)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "webSearchHistoryCompatEnabled")]
pub fn web_search_history_compat_enabled(profile: String) -> Result<bool, JsError> {
    web_search_history_compat_enabled_inner(profile).map_err(|e| JsError::new(&e))
}

/// `localCompactionPromptAtEndEnabled`.
fn local_compaction_prompt_at_end_enabled_inner(profile: String) -> Result<bool, String> {
    let profile = crate::js::from_json(&profile)?;
    let out = port::local_compaction_prompt_at_end_enabled(profile)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "localCompactionPromptAtEndEnabled")]
pub fn local_compaction_prompt_at_end_enabled(profile: String) -> Result<bool, JsError> {
    local_compaction_prompt_at_end_enabled_inner(profile).map_err(|e| JsError::new(&e))
}

/// `stripImageGenerationToolEnabled`.
fn strip_image_generation_tool_enabled_inner(profile: String) -> Result<bool, String> {
    let profile = crate::js::from_json(&profile)?;
    let out = port::strip_image_generation_tool_enabled(profile)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "stripImageGenerationToolEnabled")]
pub fn strip_image_generation_tool_enabled(profile: String) -> Result<bool, JsError> {
    strip_image_generation_tool_enabled_inner(profile).map_err(|e| JsError::new(&e))
}

/// `stripResponsesImageDetailEnabled`.
fn strip_responses_image_detail_enabled_inner(profile: String) -> Result<bool, String> {
    let profile = crate::js::from_json(&profile)?;
    let out = port::strip_responses_image_detail_enabled(profile)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "stripResponsesImageDetailEnabled")]
pub fn strip_responses_image_detail_enabled(profile: String) -> Result<bool, JsError> {
    strip_responses_image_detail_enabled_inner(profile).map_err(|e| JsError::new(&e))
}

/// `pureApiImageGenerationEnabled`.
fn pure_api_image_generation_enabled_inner(profile: String) -> Result<bool, String> {
    let profile = crate::js::from_json(&profile)?;
    let out = port::pure_api_image_generation_enabled(profile)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "pureApiImageGenerationEnabled")]
pub fn pure_api_image_generation_enabled(profile: String) -> Result<bool, JsError> {
    pure_api_image_generation_enabled_inner(profile).map_err(|e| JsError::new(&e))
}

/// `requestCompatibilityEnabled`.
fn request_compatibility_enabled_inner(profile: String) -> Result<bool, String> {
    let profile = crate::js::from_json(&profile)?;
    let out = port::request_compatibility_enabled(profile)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "requestCompatibilityEnabled")]
pub fn request_compatibility_enabled(profile: String) -> Result<bool, JsError> {
    request_compatibility_enabled_inner(profile).map_err(|e| JsError::new(&e))
}

/// `settingsRequireLocalHelper`.
fn settings_require_local_helper_inner(settings: String) -> Result<bool, String> {
    let settings = crate::js::from_json(&settings)?;
    let out = port::settings_require_local_helper(settings)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "settingsRequireLocalHelper")]
pub fn settings_require_local_helper(settings: String) -> Result<bool, JsError> {
    settings_require_local_helper_inner(settings).map_err(|e| JsError::new(&e))
}

/// `modelRouteSaveRequiresRestart`.
fn model_route_save_requires_restart_inner(
    current: String,
    proposed: String,
    active_live_base_url: String,
) -> Result<bool, String> {
    let current = crate::js::from_json(&current)?;
    let proposed = crate::js::from_json(&proposed)?;
    let out = port::model_route_save_requires_restart(current, proposed, active_live_base_url)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "modelRouteSaveRequiresRestart")]
pub fn model_route_save_requires_restart(
    current: String,
    proposed: String,
    active_live_base_url: String,
) -> Result<bool, JsError> {
    model_route_save_requires_restart_inner(current, proposed, active_live_base_url)
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

static S_INDEPENDENT_PROXY_ENABLED_0: wire::Schema = wire::Schema::Dyn;

/// `independentProxyEnabled` over the binary wire.
#[wasm_bindgen(js_name = "independentProxyEnabledWire")]
pub fn independent_proxy_enabled_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let profile: Value = wire::read_value(r, &S_INDEPENDENT_PROXY_ENABLED_0)?;
        let out = &port::independent_proxy_enabled(profile)?;
        w.bool(*out);
        Ok(())
    })
}

static S_NORMALIZE_RELAY_MODEL_ROUTES_0: wire::Schema = wire::Schema::Dyn;
static S_NORMALIZE_RELAY_MODEL_ROUTES_OUT: wire::Schema =
    wire::Schema::List(&wire::Schema::Obj(&[
        ("model", wire::Schema::Str16),
        ("targetRelayId", wire::Schema::Str16),
        ("targetModel", wire::Schema::Str16),
    ]));

/// `normalizeRelayModelRoutes` over the binary wire.
#[wasm_bindgen(js_name = "normalizeRelayModelRoutesWire")]
pub fn normalize_relay_model_routes_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let routes: Value = wire::read_value(r, &S_NORMALIZE_RELAY_MODEL_ROUTES_0)?;
        let out = &port::normalize_relay_model_routes(routes)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_NORMALIZE_RELAY_MODEL_ROUTES_OUT.inner())?;
        }
        Ok(())
    })
}

static S_FIND_RELAY_MODEL_ROUTE_ISSUE_0: wire::Schema = wire::Schema::Dyn;
static S_FIND_RELAY_MODEL_ROUTE_ISSUE_1: wire::Schema = wire::Schema::Dyn;
static S_FIND_RELAY_MODEL_ROUTE_ISSUE_OUT: wire::Schema =
    wire::Schema::OptJs(&wire::Schema::Obj(&[
        ("kind", wire::Schema::Str16),
        ("model", wire::Schema::Str16),
        ("sourceRelayId", wire::Schema::Str16),
    ]));

/// `findRelayModelRouteIssue` over the binary wire.
#[wasm_bindgen(js_name = "findRelayModelRouteIssueWire")]
pub fn find_relay_model_route_issue_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let sources: Value = wire::read_value(r, &S_FIND_RELAY_MODEL_ROUTE_ISSUE_0)?;
        let all_profiles: Value = wire::read_value(r, &S_FIND_RELAY_MODEL_ROUTE_ISSUE_1)?;
        let out = &port::find_relay_model_route_issue(sources, all_profiles)?;
        match out {
            Some(x0) => {
                w.tag(1);
                wire::write_value(w, x0, S_FIND_RELAY_MODEL_ROUTE_ISSUE_OUT.inner())?;
            }
            None => w.tag(2),
        }
        Ok(())
    })
}

static S_WEB_SEARCH_HISTORY_COMPAT_ENABLED_0: wire::Schema = wire::Schema::Dyn;

/// `webSearchHistoryCompatEnabled` over the binary wire.
#[wasm_bindgen(js_name = "webSearchHistoryCompatEnabledWire")]
pub fn web_search_history_compat_enabled_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let profile: Value = wire::read_value(r, &S_WEB_SEARCH_HISTORY_COMPAT_ENABLED_0)?;
        let out = &port::web_search_history_compat_enabled(profile)?;
        w.bool(*out);
        Ok(())
    })
}

static S_LOCAL_COMPACTION_PROMPT_AT_END_ENABLED_0: wire::Schema = wire::Schema::Dyn;

/// `localCompactionPromptAtEndEnabled` over the binary wire.
#[wasm_bindgen(js_name = "localCompactionPromptAtEndEnabledWire")]
pub fn local_compaction_prompt_at_end_enabled_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let profile: Value = wire::read_value(r, &S_LOCAL_COMPACTION_PROMPT_AT_END_ENABLED_0)?;
        let out = &port::local_compaction_prompt_at_end_enabled(profile)?;
        w.bool(*out);
        Ok(())
    })
}

static S_STRIP_IMAGE_GENERATION_TOOL_ENABLED_0: wire::Schema = wire::Schema::Dyn;

/// `stripImageGenerationToolEnabled` over the binary wire.
#[wasm_bindgen(js_name = "stripImageGenerationToolEnabledWire")]
pub fn strip_image_generation_tool_enabled_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let profile: Value = wire::read_value(r, &S_STRIP_IMAGE_GENERATION_TOOL_ENABLED_0)?;
        let out = &port::strip_image_generation_tool_enabled(profile)?;
        w.bool(*out);
        Ok(())
    })
}

static S_STRIP_RESPONSES_IMAGE_DETAIL_ENABLED_0: wire::Schema = wire::Schema::Dyn;

/// `stripResponsesImageDetailEnabled` over the binary wire.
#[wasm_bindgen(js_name = "stripResponsesImageDetailEnabledWire")]
pub fn strip_responses_image_detail_enabled_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let profile: Value = wire::read_value(r, &S_STRIP_RESPONSES_IMAGE_DETAIL_ENABLED_0)?;
        let out = &port::strip_responses_image_detail_enabled(profile)?;
        w.bool(*out);
        Ok(())
    })
}

static S_PURE_API_IMAGE_GENERATION_ENABLED_0: wire::Schema = wire::Schema::Dyn;

/// `pureApiImageGenerationEnabled` over the binary wire.
#[wasm_bindgen(js_name = "pureApiImageGenerationEnabledWire")]
pub fn pure_api_image_generation_enabled_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let profile: Value = wire::read_value(r, &S_PURE_API_IMAGE_GENERATION_ENABLED_0)?;
        let out = &port::pure_api_image_generation_enabled(profile)?;
        w.bool(*out);
        Ok(())
    })
}

static S_REQUEST_COMPATIBILITY_ENABLED_0: wire::Schema = wire::Schema::Dyn;

/// `requestCompatibilityEnabled` over the binary wire.
#[wasm_bindgen(js_name = "requestCompatibilityEnabledWire")]
pub fn request_compatibility_enabled_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let profile: Value = wire::read_value(r, &S_REQUEST_COMPATIBILITY_ENABLED_0)?;
        let out = &port::request_compatibility_enabled(profile)?;
        w.bool(*out);
        Ok(())
    })
}

static S_SETTINGS_REQUIRE_LOCAL_HELPER_0: wire::Schema = wire::Schema::Dyn;

/// `settingsRequireLocalHelper` over the binary wire.
#[wasm_bindgen(js_name = "settingsRequireLocalHelperWire")]
pub fn settings_require_local_helper_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let settings: Value = wire::read_value(r, &S_SETTINGS_REQUIRE_LOCAL_HELPER_0)?;
        let out = &port::settings_require_local_helper(settings)?;
        w.bool(*out);
        Ok(())
    })
}

static S_MODEL_ROUTE_SAVE_REQUIRES_RESTART_0: wire::Schema = wire::Schema::Dyn;
static S_MODEL_ROUTE_SAVE_REQUIRES_RESTART_1: wire::Schema = wire::Schema::Dyn;

/// `modelRouteSaveRequiresRestart` over the binary wire.
#[wasm_bindgen(js_name = "modelRouteSaveRequiresRestartWire")]
pub fn model_route_save_requires_restart_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let current: Value = wire::read_value(r, &S_MODEL_ROUTE_SAVE_REQUIRES_RESTART_0)?;
        let proposed: Value = wire::read_value(r, &S_MODEL_ROUTE_SAVE_REQUIRES_RESTART_1)?;
        let active_live_base_url: String = r.string16()?;
        let out =
            &port::model_route_save_requires_restart(current, proposed, active_live_base_url)?;
        w.bool(*out);
        Ok(())
    })
}
