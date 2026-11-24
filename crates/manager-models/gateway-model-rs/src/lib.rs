//! JS bindings for `src/gateway-model.ts`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;
pub mod utf16;

/// `keyLabel`.
fn key_label_inner(keys: String, key_id: String) -> Result<String, String> {
    let keys = crate::js::from_json(&keys)?;
    let out = port::key_label(keys, key_id)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "keyLabel")]
pub fn key_label(keys: String, key_id: String) -> Result<String, JsError> {
    key_label_inner(keys, key_id).map_err(|e| JsError::new(&e))
}

/// `imageModelLabel`.
fn image_model_label_inner(model: String, vendor_picks: bool) -> Result<String, String> {
    let out = port::image_model_label(model, vendor_picks)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "imageModelLabel")]
pub fn image_model_label(model: String, vendor_picks: bool) -> Result<String, JsError> {
    image_model_label_inner(model, vendor_picks).map_err(|e| JsError::new(&e))
}

/// `blankBinding`.
fn blank_binding_inner() -> Result<String, String> {
    let out = port::blank_binding()?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "blankBinding")]
pub fn blank_binding() -> Result<String, JsError> {
    blank_binding_inner().map_err(|e| JsError::new(&e))
}

/// `draftFromBinding`.
fn draft_from_binding_inner(binding: String) -> Result<String, String> {
    let binding = crate::js::from_json(&binding)?;
    let out = port::draft_from_binding(binding)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "draftFromBinding")]
pub fn draft_from_binding(binding: String) -> Result<String, JsError> {
    draft_from_binding_inner(binding).map_err(|e| JsError::new(&e))
}

/// `defaultProviderAgents`.
fn default_provider_agents_inner(payload: String) -> Result<String, String> {
    let payload = crate::js::from_json(&payload)?;
    let out = port::default_provider_agents(payload)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "defaultProviderAgents")]
pub fn default_provider_agents(payload: String) -> Result<String, JsError> {
    default_provider_agents_inner(payload).map_err(|e| JsError::new(&e))
}

/// `skippedAgents`.
fn skipped_agents_inner(payload: String, picked: String) -> Result<String, String> {
    let payload = crate::js::from_json(&payload)?;
    let picked = crate::js::from_json(&picked)?;
    let out = port::skipped_agents(payload, picked)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "skippedAgents")]
pub fn skipped_agents(payload: String, picked: String) -> Result<String, JsError> {
    skipped_agents_inner(payload, picked).map_err(|e| JsError::new(&e))
}

/// `poolExcludeToggle`.
fn pool_exclude_toggle_inner(
    previous: String,
    provider_id: String,
    on: bool,
) -> Result<Vec<String>, String> {
    let previous = crate::js::from_json(&previous)?;
    let out = port::pool_exclude_toggle(previous, provider_id, on)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "poolExcludeToggle")]
pub fn pool_exclude_toggle(
    previous: String,
    provider_id: String,
    on: bool,
) -> Result<Vec<String>, JsError> {
    pool_exclude_toggle_inner(previous, provider_id, on).map_err(|e| JsError::new(&e))
}

/// `parseContextWindow`.
fn parse_context_window_inner(value: String) -> Result<String, String> {
    let out = port::parse_context_window(value)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "parseContextWindow")]
pub fn parse_context_window(value: String) -> Result<String, JsError> {
    parse_context_window_inner(value).map_err(|e| JsError::new(&e))
}

/// `parseKeyLimit`.
fn parse_key_limit_inner(value: String) -> Result<String, String> {
    let out = port::parse_key_limit(value)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "parseKeyLimit")]
pub fn parse_key_limit(value: String) -> Result<String, JsError> {
    parse_key_limit_inner(value).map_err(|e| JsError::new(&e))
}

/// `parseCompactionNote`.
fn parse_compaction_note_inner(reason: String) -> Result<String, String> {
    let out = port::parse_compaction_note(reason)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "parseCompactionNote")]
pub fn parse_compaction_note(reason: String) -> Result<String, JsError> {
    parse_compaction_note_inner(reason).map_err(|e| JsError::new(&e))
}

/// `compactionNote`.
fn compaction_note_inner(reasons: String) -> Result<String, String> {
    let reasons = crate::js::from_json(&reasons)?;
    let out = port::compaction_note(reasons)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "compactionNote")]
pub fn compaction_note(reasons: String) -> Result<String, JsError> {
    compaction_note_inner(reasons).map_err(|e| JsError::new(&e))
}

/// `foreignCompactionDropped`.
fn foreign_compaction_dropped_inner(reasons: String) -> Result<bool, String> {
    let reasons = crate::js::from_json(&reasons)?;
    let out = port::foreign_compaction_dropped(reasons)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "foreignCompactionDropped")]
pub fn foreign_compaction_dropped(reasons: String) -> Result<bool, JsError> {
    foreign_compaction_dropped_inner(reasons).map_err(|e| JsError::new(&e))
}

/// `queueWaitMs`.
fn queue_wait_ms_inner(reasons: String) -> Result<String, String> {
    let reasons = crate::js::from_json(&reasons)?;
    let out = port::queue_wait_ms(reasons)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "queueWaitMs")]
pub fn queue_wait_ms(reasons: String) -> Result<String, JsError> {
    queue_wait_ms_inner(reasons).map_err(|e| JsError::new(&e))
}

/// `formatQueueWait`.
fn format_queue_wait_inner(ms: f64) -> Result<String, String> {
    let out = port::format_queue_wait(ms)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "formatQueueWait")]
pub fn format_queue_wait(ms: f64) -> Result<String, JsError> {
    format_queue_wait_inner(ms).map_err(|e| JsError::new(&e))
}

/// `splitModels`.
fn split_models_inner(text: String) -> Result<Vec<String>, String> {
    let out = port::split_models(text)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "splitModels")]
pub fn split_models(text: String) -> Result<Vec<String>, JsError> {
    split_models_inner(text).map_err(|e| JsError::new(&e))
}

/// `bindingFromDraft`.
fn binding_from_draft_inner(
    draft: String,
    previous: String,
    item: String,
) -> Result<String, String> {
    let draft = crate::js::from_json(&draft)?;
    let previous = crate::js::from_json(&previous)?;
    let item = crate::js::from_json(&item)?;
    let out = port::binding_from_draft(draft, previous, item)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "bindingFromDraft")]
pub fn binding_from_draft(
    draft: String,
    previous: String,
    item: String,
) -> Result<String, JsError> {
    binding_from_draft_inner(draft, previous, item).map_err(|e| JsError::new(&e))
}

/// `groupModelId`.
fn group_model_id_inner(id: String) -> Result<String, String> {
    let out = port::group_model_id(id)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "groupModelId")]
pub fn group_model_id(id: String) -> Result<String, JsError> {
    group_model_id_inner(id).map_err(|e| JsError::new(&e))
}

/// `disabledGroup`.
fn disabled_group_inner(
    payload: String,
    model: String,
    selection: String,
) -> Result<String, String> {
    let payload = crate::js::from_json(&payload)?;
    let selection = crate::js::from_json(&selection)?;
    let out = port::disabled_group(payload, model, selection)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "disabledGroup")]
pub fn disabled_group(
    payload: String,
    model: String,
    selection: String,
) -> Result<String, JsError> {
    disabled_group_inner(payload, model, selection).map_err(|e| JsError::new(&e))
}

/// `slugify`.
fn slugify_inner(name: String) -> Result<String, String> {
    let out = port::slugify(name)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "slugify")]
pub fn slugify(name: String) -> Result<String, JsError> {
    slugify_inner(name).map_err(|e| JsError::new(&e))
}

/// `providerEnabled`.
fn provider_enabled_inner(entry: String) -> Result<bool, String> {
    let entry = crate::js::from_json(&entry)?;
    let out = port::provider_enabled(entry)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "providerEnabled")]
pub fn provider_enabled(entry: String) -> Result<bool, JsError> {
    provider_enabled_inner(entry).map_err(|e| JsError::new(&e))
}

/// `agentPoolProviders`.
fn agent_pool_providers_inner(payload: String, exclude: String) -> Result<String, String> {
    let payload = crate::js::from_json(&payload)?;
    let exclude = crate::js::from_json(&exclude)?;
    let out = port::agent_pool_providers(payload, exclude)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "agentPoolProviders")]
pub fn agent_pool_providers(payload: String, exclude: String) -> Result<String, JsError> {
    agent_pool_providers_inner(payload, exclude).map_err(|e| JsError::new(&e))
}

/// `agentModels`.
fn agent_models_inner(payload: String, exclude: String) -> Result<Vec<String>, String> {
    let payload = crate::js::from_json(&payload)?;
    let exclude = crate::js::from_json(&exclude)?;
    let out = port::agent_models(payload, exclude)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "agentModels")]
pub fn agent_models(payload: String, exclude: String) -> Result<Vec<String>, JsError> {
    agent_models_inner(payload, exclude).map_err(|e| JsError::new(&e))
}

/// `qualifiedProvider`.
fn qualified_provider_inner(
    payload: String,
    model: String,
    exclude: String,
) -> Result<String, String> {
    let payload = crate::js::from_json(&payload)?;
    let exclude = crate::js::from_json(&exclude)?;
    let out = port::qualified_provider(payload, model, exclude)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "qualifiedProvider")]
pub fn qualified_provider(
    payload: String,
    model: String,
    exclude: String,
) -> Result<String, JsError> {
    qualified_provider_inner(payload, model, exclude).map_err(|e| JsError::new(&e))
}

/// `modelRoutable`.
fn model_routable_inner(payload: String, model: String, exclude: String) -> Result<bool, String> {
    let payload = crate::js::from_json(&payload)?;
    let exclude = crate::js::from_json(&exclude)?;
    let out = port::model_routable(payload, model, exclude)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "modelRoutable")]
pub fn model_routable(payload: String, model: String, exclude: String) -> Result<bool, JsError> {
    model_routable_inner(payload, model, exclude).map_err(|e| JsError::new(&e))
}

/// `validateDraft`.
fn validate_draft_inner(
    draft: String,
    payload: String,
    retained_model: String,
    retained_selection: String,
) -> Result<String, String> {
    let draft = crate::js::from_json(&draft)?;
    let payload = crate::js::from_json(&payload)?;
    let retained_model = crate::js::from_json(&retained_model)?;
    let retained_selection = crate::js::from_json(&retained_selection)?;
    let out = port::validate_draft(draft, payload, retained_model, retained_selection)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "validateDraft")]
pub fn validate_draft(
    draft: String,
    payload: String,
    retained_model: String,
    retained_selection: String,
) -> Result<String, JsError> {
    validate_draft_inner(draft, payload, retained_model, retained_selection)
        .map_err(|e| JsError::new(&e))
}

/// `protocolLabel`.
fn protocol_label_inner(protocol: String) -> Result<String, String> {
    let protocol = crate::js::from_json(&protocol)?;
    let out = port::protocol_label(protocol)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "protocolLabel")]
pub fn protocol_label(protocol: String) -> Result<String, JsError> {
    protocol_label_inner(protocol).map_err(|e| JsError::new(&e))
}

/// `failureLabel`.
fn failure_label_inner(value: Option<String>) -> Result<String, String> {
    let out = port::failure_label(value)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "failureLabel")]
pub fn failure_label(value: Option<String>) -> Result<String, JsError> {
    failure_label_inner(value).map_err(|e| JsError::new(&e))
}

/// `formatRemaining`.
fn format_remaining_inner(ms: Option<f64>) -> Result<String, String> {
    let out = port::format_remaining(ms)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "formatRemaining")]
pub fn format_remaining(ms: Option<f64>) -> Result<String, JsError> {
    format_remaining_inner(ms).map_err(|e| JsError::new(&e))
}

/// `glanceAgents`.
fn glance_agents_inner(agents: String) -> Result<String, String> {
    let agents = crate::js::from_json(&agents)?;
    let out = port::glance_agents(agents)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "glanceAgents")]
pub fn glance_agents(agents: String) -> Result<String, JsError> {
    glance_agents_inner(agents).map_err(|e| JsError::new(&e))
}

/// `agentName`.
fn agent_name_inner(agent: String) -> Result<String, String> {
    let out = port::agent_name(agent)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "agentName")]
pub fn agent_name(agent: String) -> Result<String, JsError> {
    agent_name_inner(agent).map_err(|e| JsError::new(&e))
}

/// `rememberAgentNames`.
fn remember_agent_names_inner(agents: String) -> Result<String, String> {
    let agents = crate::js::from_json(&agents)?;
    let out = port::remember_agent_names(agents)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "rememberAgentNames")]
pub fn remember_agent_names(agents: String) -> Result<String, JsError> {
    remember_agent_names_inner(agents).map_err(|e| JsError::new(&e))
}

/// `visibleOwnModels`.
fn visible_own_models_inner(agent: String, payload: String) -> Result<String, String> {
    let agent = crate::js::from_json(&agent)?;
    let payload = crate::js::from_json(&payload)?;
    let out = port::visible_own_models(agent, payload)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "visibleOwnModels")]
pub fn visible_own_models(agent: String, payload: String) -> Result<String, JsError> {
    visible_own_models_inner(agent, payload).map_err(|e| JsError::new(&e))
}

/// `servingProviders`.
fn serving_providers_inner(
    payload: String,
    model: String,
    exclude: String,
) -> Result<String, String> {
    let payload = crate::js::from_json(&payload)?;
    let exclude = crate::js::from_json(&exclude)?;
    let out = port::serving_providers(payload, model, exclude)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "servingProviders")]
pub fn serving_providers(
    payload: String,
    model: String,
    exclude: String,
) -> Result<String, JsError> {
    serving_providers_inner(payload, model, exclude).map_err(|e| JsError::new(&e))
}

/// `groupDisplayName`.
fn group_display_name_inner(payload: String, model: String) -> Result<String, String> {
    let payload = crate::js::from_json(&payload)?;
    let out = port::group_display_name(payload, model)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "groupDisplayName")]
pub fn group_display_name(payload: String, model: String) -> Result<String, JsError> {
    group_display_name_inner(payload, model).map_err(|e| JsError::new(&e))
}

/// `modelDisplay`.
fn model_display_inner(
    payload: String,
    model: String,
    exclude: String,
    selection: String,
) -> Result<String, String> {
    let payload = crate::js::from_json(&payload)?;
    let exclude = crate::js::from_json(&exclude)?;
    let selection = crate::js::from_json(&selection)?;
    let out = port::model_display(payload, model, exclude, selection)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "modelDisplay")]
pub fn model_display(
    payload: String,
    model: String,
    exclude: String,
    selection: String,
) -> Result<String, JsError> {
    model_display_inner(payload, model, exclude, selection).map_err(|e| JsError::new(&e))
}

/// `prettyModelName`.
fn pretty_model_name_inner(model: String) -> Result<String, String> {
    let out = port::pretty_model_name(model)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "prettyModelName")]
pub fn pretty_model_name(model: String) -> Result<String, JsError> {
    pretty_model_name_inner(model).map_err(|e| JsError::new(&e))
}

/// `groupModeLabel`.
fn group_mode_label_inner(mode: String) -> Result<String, String> {
    let out = port::group_mode_label(mode)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "groupModeLabel")]
pub fn group_mode_label(mode: String) -> Result<String, JsError> {
    group_mode_label_inner(mode).map_err(|e| JsError::new(&e))
}

/// `groupModeDescription`.
fn group_mode_description_inner(mode: String) -> Result<String, String> {
    let out = port::group_mode_description(mode)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "groupModeDescription")]
pub fn group_mode_description(mode: String) -> Result<String, JsError> {
    group_mode_description_inner(mode).map_err(|e| JsError::new(&e))
}

/// `groupStickyLabel`.
fn group_sticky_label_inner(sticky: Option<String>) -> Result<String, String> {
    let out = port::group_sticky_label(sticky)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "groupStickyLabel")]
pub fn group_sticky_label(sticky: Option<String>) -> Result<String, JsError> {
    group_sticky_label_inner(sticky).map_err(|e| JsError::new(&e))
}

/// `groupStickyDescription`.
fn group_sticky_description_inner(sticky: Option<String>) -> Result<String, String> {
    let out = port::group_sticky_description(sticky)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "groupStickyDescription")]
pub fn group_sticky_description(sticky: Option<String>) -> Result<String, JsError> {
    group_sticky_description_inner(sticky).map_err(|e| JsError::new(&e))
}

/// `parseWeight`.
fn parse_weight_inner(text: String) -> Result<String, String> {
    let out = port::parse_weight(text)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "parseWeight")]
pub fn parse_weight(text: String) -> Result<String, JsError> {
    parse_weight_inner(text).map_err(|e| JsError::new(&e))
}

/// `parseGatewayPort`.
fn parse_gateway_port_inner(text: String) -> Result<String, String> {
    let out = port::parse_gateway_port(text)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "parseGatewayPort")]
pub fn parse_gateway_port(text: String) -> Result<String, JsError> {
    parse_gateway_port_inner(text).map_err(|e| JsError::new(&e))
}

/// `isPattern`.
fn is_pattern_inner(model: String) -> Result<bool, String> {
    let out = port::is_pattern(model)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "isPattern")]
pub fn is_pattern(model: String) -> Result<bool, JsError> {
    is_pattern_inner(model).map_err(|e| JsError::new(&e))
}

/// `globMatches`.
fn glob_matches_inner(pattern: String, text: String) -> Result<bool, String> {
    let out = port::glob_matches(pattern, text)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "globMatches")]
pub fn glob_matches(pattern: String, text: String) -> Result<bool, JsError> {
    glob_matches_inner(pattern, text).map_err(|e| JsError::new(&e))
}

/// `patternMatches`.
fn pattern_matches_inner(payload: String, pattern: String) -> Result<Vec<String>, String> {
    let payload = crate::js::from_json(&payload)?;
    let out = port::pattern_matches(payload, pattern)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "patternMatches")]
pub fn pattern_matches(payload: String, pattern: String) -> Result<Vec<String>, JsError> {
    pattern_matches_inner(payload, pattern).map_err(|e| JsError::new(&e))
}

/// `speedOf`.
fn speed_of_inner(payload: String, relay_id: String, model: String) -> Result<String, String> {
    let payload = crate::js::from_json(&payload)?;
    let out = port::speed_of(payload, relay_id, model)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "speedOf")]
pub fn speed_of(payload: String, relay_id: String, model: String) -> Result<String, JsError> {
    speed_of_inner(payload, relay_id, model).map_err(|e| JsError::new(&e))
}

/// `quotaWindowLabel`.
fn quota_window_label_inner(window: String) -> Result<String, String> {
    let window = crate::js::from_json(&window)?;
    let out = port::quota_window_label(window)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "quotaWindowLabel")]
pub fn quota_window_label(window: String) -> Result<String, JsError> {
    quota_window_label_inner(window).map_err(|e| JsError::new(&e))
}

/// `quotaSupported`.
fn quota_supported_inner(entry: String) -> Result<bool, String> {
    let entry = crate::js::from_json(&entry)?;
    let out = port::quota_supported(entry)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "quotaSupported")]
pub fn quota_supported(entry: String) -> Result<bool, JsError> {
    quota_supported_inner(entry).map_err(|e| JsError::new(&e))
}

/// `quotaEntries`.
fn quota_entries_inner(pool: String) -> Result<String, String> {
    let pool = crate::js::from_json(&pool)?;
    let out = port::quota_entries(pool)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "quotaEntries")]
pub fn quota_entries(pool: String) -> Result<String, JsError> {
    quota_entries_inner(pool).map_err(|e| JsError::new(&e))
}

/// `quotaRemaining`.
fn quota_remaining_inner(quota: String, now_ms: f64) -> Result<f64, String> {
    let quota = crate::js::from_json(&quota)?;
    let out = port::quota_remaining(quota, now_ms)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "quotaRemaining")]
pub fn quota_remaining(quota: String, now_ms: f64) -> Result<f64, JsError> {
    quota_remaining_inner(quota, now_ms).map_err(|e| JsError::new(&e))
}

/// `quotaWindowFresh`.
fn quota_window_fresh_inner(
    quota: String,
    window: String,
    now_ms: f64,
    max_age_seconds: String,
) -> Result<bool, String> {
    let quota = crate::js::from_json(&quota)?;
    let window = crate::js::from_json(&window)?;
    let max_age_seconds = crate::js::from_json(&max_age_seconds)?;
    let out = port::quota_window_fresh(quota, window, now_ms, max_age_seconds)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "quotaWindowFresh")]
pub fn quota_window_fresh(
    quota: String,
    window: String,
    now_ms: f64,
    max_age_seconds: String,
) -> Result<bool, JsError> {
    quota_window_fresh_inner(quota, window, now_ms, max_age_seconds).map_err(|e| JsError::new(&e))
}

/// `subscriptionBlockReason`.
fn subscription_block_reason_inner(entry: String, now_ms: String) -> Result<String, String> {
    let entry = crate::js::from_json(&entry)?;
    let now_ms = crate::js::from_json(&now_ms)?;
    let out = port::subscription_block_reason(entry, now_ms)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "subscriptionBlockReason")]
pub fn subscription_block_reason(entry: String, now_ms: String) -> Result<String, JsError> {
    subscription_block_reason_inner(entry, now_ms).map_err(|e| JsError::new(&e))
}

/// `memberLabel`.
fn member_label_inner(payload: String, model: String) -> Result<String, String> {
    let payload = crate::js::from_json(&payload)?;
    let out = port::member_label(payload, model)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "memberLabel")]
pub fn member_label(payload: String, model: String) -> Result<String, JsError> {
    member_label_inner(payload, model).map_err(|e| JsError::new(&e))
}

/// `memberStatus`.
fn member_status_inner(
    payload: String,
    relay_id: String,
    now_ms: String,
) -> Result<String, String> {
    let payload = crate::js::from_json(&payload)?;
    let now_ms = crate::js::from_json(&now_ms)?;
    let out = port::member_status(payload, relay_id, now_ms)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "memberStatus")]
pub fn member_status(payload: String, relay_id: String, now_ms: String) -> Result<String, JsError> {
    member_status_inner(payload, relay_id, now_ms).map_err(|e| JsError::new(&e))
}

/// `dispatchStatus`.
fn dispatch_status_inner(
    payload: String,
    relay_id: String,
    model: String,
    now_ms: String,
) -> Result<String, String> {
    let payload = crate::js::from_json(&payload)?;
    let now_ms = crate::js::from_json(&now_ms)?;
    let out = port::dispatch_status(payload, relay_id, model, now_ms)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "dispatchStatus")]
pub fn dispatch_status(
    payload: String,
    relay_id: String,
    model: String,
    now_ms: String,
) -> Result<String, JsError> {
    dispatch_status_inner(payload, relay_id, model, now_ms).map_err(|e| JsError::new(&e))
}

/// `formatElapsed`.
fn format_elapsed_inner(ms: f64) -> Result<String, String> {
    let out = port::format_elapsed(ms)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "formatElapsed")]
pub fn format_elapsed(ms: f64) -> Result<String, JsError> {
    format_elapsed_inner(ms).map_err(|e| JsError::new(&e))
}

/// `routeNarrative`.
fn route_narrative_inner(agent: String, record: String, payload: String) -> Result<String, String> {
    let record = crate::js::from_json(&record)?;
    let payload = crate::js::from_json(&payload)?;
    let out = port::route_narrative(agent, record, payload)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "routeNarrative")]
pub fn route_narrative(agent: String, record: String, payload: String) -> Result<String, JsError> {
    route_narrative_inner(agent, record, payload).map_err(|e| JsError::new(&e))
}

/// `connectedAgents`.
fn connected_agents_inner(payload: String) -> Result<String, String> {
    let payload = crate::js::from_json(&payload)?;
    let out = port::connected_agents(payload)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "connectedAgents")]
pub fn connected_agents(payload: String) -> Result<String, JsError> {
    connected_agents_inner(payload).map_err(|e| JsError::new(&e))
}

/// `modelDispatch`.
fn model_dispatch_inner(payload: String) -> Result<String, String> {
    let payload = crate::js::from_json(&payload)?;
    let out = port::model_dispatch(payload)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "modelDispatch")]
pub fn model_dispatch(payload: String) -> Result<String, JsError> {
    model_dispatch_inner(payload).map_err(|e| JsError::new(&e))
}

/// `activeSponsorIds`.
fn active_sponsor_ids_inner(now: String) -> Result<Vec<String>, String> {
    let now = crate::js::from_json(&now)?;
    let out = port::active_sponsor_ids(now)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "activeSponsorIds")]
pub fn active_sponsor_ids(now: String) -> Result<Vec<String>, JsError> {
    active_sponsor_ids_inner(now).map_err(|e| JsError::new(&e))
}

/// `gatewayPresets`.
fn gateway_presets_inner(now: String) -> Result<String, String> {
    let now = crate::js::from_json(&now)?;
    let out = port::gateway_presets(now)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "gatewayPresets")]
pub fn gateway_presets(now: String) -> Result<String, JsError> {
    gateway_presets_inner(now).map_err(|e| JsError::new(&e))
}

/// `presetHost`.
fn preset_host_inner(base_url: String) -> Result<String, String> {
    let out = port::preset_host(base_url)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "presetHost")]
pub fn preset_host(base_url: String) -> Result<String, JsError> {
    preset_host_inner(base_url).map_err(|e| JsError::new(&e))
}

/// `presetAdded`.
fn preset_added_inner(payload: String, preset: String) -> Result<bool, String> {
    let payload = crate::js::from_json(&payload)?;
    let preset = crate::js::from_json(&preset)?;
    let out = port::preset_added(payload, preset)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "presetAdded")]
pub fn preset_added(payload: String, preset: String) -> Result<bool, JsError> {
    preset_added_inner(payload, preset).map_err(|e| JsError::new(&e))
}

/// `providerDraftFromPreset`.
fn provider_draft_from_preset_inner(preset: String) -> Result<String, String> {
    let preset = crate::js::from_json(&preset)?;
    let out = port::provider_draft_from_preset(preset)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "providerDraftFromPreset")]
pub fn provider_draft_from_preset(preset: String) -> Result<String, JsError> {
    provider_draft_from_preset_inner(preset).map_err(|e| JsError::new(&e))
}

/// `providerDraftFromEntry`.
fn provider_draft_from_entry_inner(entry: String) -> Result<String, String> {
    let entry = crate::js::from_json(&entry)?;
    let out = port::provider_draft_from_entry(entry)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "providerDraftFromEntry")]
pub fn provider_draft_from_entry(entry: String) -> Result<String, JsError> {
    provider_draft_from_entry_inner(entry).map_err(|e| JsError::new(&e))
}

/// `providerInput`.
fn provider_input_inner(draft: String) -> Result<String, String> {
    let draft = crate::js::from_json(&draft)?;
    let out = port::provider_input(draft)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "providerInput")]
pub fn provider_input(draft: String) -> Result<String, JsError> {
    provider_input_inner(draft).map_err(|e| JsError::new(&e))
}

/// `priceFromDraft`.
fn price_from_draft_inner(draft: String) -> Result<String, String> {
    let draft = crate::js::from_json(&draft)?;
    let out = port::price_from_draft(draft)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "priceFromDraft")]
pub fn price_from_draft(draft: String) -> Result<String, JsError> {
    price_from_draft_inner(draft).map_err(|e| JsError::new(&e))
}

/// `isDirectProxy`.
fn is_direct_proxy_inner(proxy: Option<String>) -> Result<bool, String> {
    let out = port::is_direct_proxy(proxy)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "isDirectProxy")]
pub fn is_direct_proxy(proxy: Option<String>) -> Result<bool, JsError> {
    is_direct_proxy_inner(proxy).map_err(|e| JsError::new(&e))
}

/// `proxyAddressError`.
fn proxy_address_error_inner(proxy: String) -> Result<String, String> {
    let out = port::proxy_address_error(proxy)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "proxyAddressError")]
pub fn proxy_address_error(proxy: String) -> Result<String, JsError> {
    proxy_address_error_inner(proxy).map_err(|e| JsError::new(&e))
}

/// `validateProviderDraft`.
fn validate_provider_draft_inner(draft: String, editing: String) -> Result<String, String> {
    let draft = crate::js::from_json(&draft)?;
    let editing = crate::js::from_json(&editing)?;
    let out = port::validate_provider_draft(draft, editing)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "validateProviderDraft")]
pub fn validate_provider_draft(draft: String, editing: String) -> Result<String, JsError> {
    validate_provider_draft_inner(draft, editing).map_err(|e| JsError::new(&e))
}

/// `usageRangeStart`.
fn usage_range_start_inner(range: String, now: String) -> Result<String, String> {
    let now = crate::js::from_json(&now)?;
    let out = port::usage_range_start(range, now)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "usageRangeStart")]
pub fn usage_range_start(range: String, now: String) -> Result<String, JsError> {
    usage_range_start_inner(range, now).map_err(|e| JsError::new(&e))
}

/// `relayFilterFromFocus`.
fn relay_filter_from_focus_inner(focus: String) -> Result<String, String> {
    let focus = crate::js::from_json(&focus)?;
    let out = port::relay_filter_from_focus(focus)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "relayFilterFromFocus")]
pub fn relay_filter_from_focus(focus: String) -> Result<String, JsError> {
    relay_filter_from_focus_inner(focus).map_err(|e| JsError::new(&e))
}

/// `relayFilterArgs`.
fn relay_filter_args_inner(filter: String) -> Result<String, String> {
    let filter = crate::js::from_json(&filter)?;
    let out = port::relay_filter_args(filter)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "relayFilterArgs")]
pub fn relay_filter_args(filter: String) -> Result<String, JsError> {
    relay_filter_args_inner(filter).map_err(|e| JsError::new(&e))
}

/// `formatRequestCost`.
fn format_request_cost_inner(cost: String) -> Result<String, String> {
    let cost = crate::js::from_json(&cost)?;
    let out = port::format_request_cost(cost)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "formatRequestCost")]
pub fn format_request_cost(cost: String) -> Result<String, JsError> {
    format_request_cost_inner(cost).map_err(|e| JsError::new(&e))
}

/// `formatTokens`.
fn format_tokens_inner(value: f64) -> Result<String, String> {
    let out = port::format_tokens(value)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "formatTokens")]
pub fn format_tokens(value: f64) -> Result<String, JsError> {
    format_tokens_inner(value).map_err(|e| JsError::new(&e))
}

/// `currencySymbol`.
fn currency_symbol_inner(currency: String) -> Result<String, String> {
    let out = port::currency_symbol(currency)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "currencySymbol")]
pub fn currency_symbol(currency: String) -> Result<String, JsError> {
    currency_symbol_inner(currency).map_err(|e| JsError::new(&e))
}

/// `formatMoney`.
fn format_money_inner(amount: f64, currency: String) -> Result<String, String> {
    let out = port::format_money(amount, currency)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "formatMoney")]
pub fn format_money(amount: f64, currency: String) -> Result<String, JsError> {
    format_money_inner(amount, currency).map_err(|e| JsError::new(&e))
}

/// `cacheHitRate`.
fn cache_hit_rate_inner(counts: String) -> Result<String, String> {
    let counts = crate::js::from_json(&counts)?;
    let out = port::cache_hit_rate(counts)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "cacheHitRate")]
pub fn cache_hit_rate(counts: String) -> Result<String, JsError> {
    cache_hit_rate_inner(counts).map_err(|e| JsError::new(&e))
}

/// `averageDuration`.
fn average_duration_inner(counts: String) -> Result<String, String> {
    let counts = crate::js::from_json(&counts)?;
    let out = port::average_duration(counts)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "averageDuration")]
pub fn average_duration(counts: String) -> Result<String, JsError> {
    average_duration_inner(counts).map_err(|e| JsError::new(&e))
}

/// `costText`.
fn cost_text_inner(costs: String) -> Result<String, String> {
    let costs = crate::js::from_json(&costs)?;
    let out = port::cost_text(costs)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "costText")]
pub fn cost_text(costs: String) -> Result<String, JsError> {
    cost_text_inner(costs).map_err(|e| JsError::new(&e))
}

/// `niceCeiling`.
fn nice_ceiling_inner(value: f64) -> Result<f64, String> {
    let out = port::nice_ceiling(value)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "niceCeiling")]
pub fn nice_ceiling(value: f64) -> Result<f64, JsError> {
    nice_ceiling_inner(value).map_err(|e| JsError::new(&e))
}

/// `usageDayStats`.
fn usage_day_stats_inner(days: String) -> Result<String, String> {
    let days = crate::js::from_json(&days)?;
    let out = port::usage_day_stats(days)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "usageDayStats")]
pub fn usage_day_stats(days: String) -> Result<String, JsError> {
    usage_day_stats_inner(days).map_err(|e| JsError::new(&e))
}

/// `axisLabelIndexes`.
fn axis_label_indexes_inner(length: f64, count: String) -> Result<Vec<f64>, String> {
    let count = crate::js::from_json(&count)?;
    let out = port::axis_label_indexes(length, count)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "axisLabelIndexes")]
pub fn axis_label_indexes(length: f64, count: String) -> Result<Vec<f64>, JsError> {
    axis_label_indexes_inner(length, count).map_err(|e| JsError::new(&e))
}

/// `shortDate`.
fn short_date_inner(date: String, english: String) -> Result<String, String> {
    let english = crate::js::from_json(&english)?;
    let out = port::short_date(date, english)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "shortDate")]
pub fn short_date(date: String, english: String) -> Result<String, JsError> {
    short_date_inner(date, english).map_err(|e| JsError::new(&e))
}

/// `filterSessions`.
fn filter_sessions_inner(sessions: String, agent: String, query: String) -> Result<String, String> {
    let sessions = crate::js::from_json(&sessions)?;
    let agent = crate::js::from_json(&agent)?;
    let out = port::filter_sessions(sessions, agent, query)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "filterSessions")]
pub fn filter_sessions(sessions: String, agent: String, query: String) -> Result<String, JsError> {
    filter_sessions_inner(sessions, agent, query).map_err(|e| JsError::new(&e))
}

/// `shortPath`.
fn short_path_inner(path: String, home: Option<String>) -> Result<String, String> {
    let out = port::short_path(path, home)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "shortPath")]
pub fn short_path(path: String, home: Option<String>) -> Result<String, JsError> {
    short_path_inner(path, home).map_err(|e| JsError::new(&e))
}

/// `relativeTime`.
fn relative_time_inner(ms: f64, now: f64) -> Result<String, String> {
    let out = port::relative_time(ms, now)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "relativeTime")]
pub fn relative_time(ms: f64, now: f64) -> Result<String, JsError> {
    relative_time_inner(ms, now).map_err(|e| JsError::new(&e))
}

/// `maskAccountName`.
fn mask_account_name_inner(name: String) -> Result<String, String> {
    let out = port::mask_account_name(name)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "maskAccountName")]
pub fn mask_account_name(name: String) -> Result<String, JsError> {
    mask_account_name_inner(name).map_err(|e| JsError::new(&e))
}

/// `maskAccountText`.
fn mask_account_text_inner(text: String) -> Result<String, String> {
    let out = port::mask_account_text(text)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "maskAccountText")]
pub fn mask_account_text(text: String) -> Result<String, JsError> {
    mask_account_text_inner(text).map_err(|e| JsError::new(&e))
}

/// `importCandidates`.
fn import_candidates_inner(payload: String) -> Result<String, String> {
    let payload = crate::js::from_json(&payload)?;
    let out = port::import_candidates(payload)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "importCandidates")]
pub fn import_candidates(payload: String) -> Result<String, JsError> {
    import_candidates_inner(payload).map_err(|e| JsError::new(&e))
}

/// `pendingImportCount`.
fn pending_import_count_inner(payload: String) -> Result<f64, String> {
    let payload = crate::js::from_json(&payload)?;
    let out = port::pending_import_count(payload)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "pendingImportCount")]
pub fn pending_import_count(payload: String) -> Result<f64, JsError> {
    pending_import_count_inner(payload).map_err(|e| JsError::new(&e))
}

/// `isFavoriteModel`.
fn is_favorite_model_inner(
    favorites: String,
    model: String,
    payload: String,
) -> Result<bool, String> {
    let favorites = crate::js::from_json(&favorites)?;
    let payload = crate::js::from_json(&payload)?;
    let out = port::is_favorite_model(favorites, model, payload)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "isFavoriteModel")]
pub fn is_favorite_model(
    favorites: String,
    model: String,
    payload: String,
) -> Result<bool, JsError> {
    is_favorite_model_inner(favorites, model, payload).map_err(|e| JsError::new(&e))
}

/// `toggleFavoriteModel`.
fn toggle_favorite_model_inner(
    favorites: String,
    model: String,
    payload: String,
) -> Result<String, String> {
    let favorites = crate::js::from_json(&favorites)?;
    let payload = crate::js::from_json(&payload)?;
    let out = port::toggle_favorite_model(favorites, model, payload)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "toggleFavoriteModel")]
pub fn toggle_favorite_model(
    favorites: String,
    model: String,
    payload: String,
) -> Result<String, JsError> {
    toggle_favorite_model_inner(favorites, model, payload).map_err(|e| JsError::new(&e))
}

/// `activeAutoGroup`.
fn active_auto_group_inner(payload: String, model: String) -> Result<String, String> {
    let payload = crate::js::from_json(&payload)?;
    let out = port::active_auto_group(payload, model)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "activeAutoGroup")]
pub fn active_auto_group(payload: String, model: String) -> Result<String, JsError> {
    active_auto_group_inner(payload, model).map_err(|e| JsError::new(&e))
}

/// `memberTarget`.
fn member_target_inner(payload: String, model: String) -> Result<String, String> {
    let payload = crate::js::from_json(&payload)?;
    let out = port::member_target(payload, model)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "memberTarget")]
pub fn member_target(payload: String, model: String) -> Result<String, JsError> {
    member_target_inner(payload, model).map_err(|e| JsError::new(&e))
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

static S_KEY_LABEL_0: wire::Schema = wire::Schema::Dyn;

/// `keyLabel` over the binary wire.
#[wasm_bindgen(js_name = "keyLabelWire")]
pub fn key_label_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let keys: Value = wire::read_value(r, &S_KEY_LABEL_0)?;
        let key_id: String = r.string16()?;
        let out = &port::key_label(keys, key_id)?;
        w.string16(out);
        Ok(())
    })
}

static S_BLANK_BINDING_OUT: wire::Schema = wire::Schema::Obj(&[
    ("enabled", wire::Schema::Bool),
    ("model", wire::Schema::Str16),
    (
        "defaultModelSelection",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("providerId", wire::Schema::Str16),
            ("upstreamModel", wire::Schema::Str16),
            ("providerSource", wire::Schema::Str16),
            ("providerRemoved", wire::Schema::OptJs(&wire::Schema::Bool)),
        ])),
    ),
    ("extraModels", wire::Schema::List(&wire::Schema::Str16)),
    (
        "claudeRoles",
        wire::Schema::Obj(&[
            ("opus", wire::Schema::Str16),
            ("sonnet", wire::Schema::Str16),
            ("haiku", wire::Schema::Str16),
            ("subagent", wire::Schema::Str16),
        ]),
    ),
    ("unknownModel", wire::Schema::Str16),
    ("autoReviewModel", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("titleModel", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("compactionModel", wire::Schema::OptJs(&wire::Schema::Str16)),
    (
        "compactionReasoningEffort",
        wire::Schema::OptJs(&wire::Schema::Str16),
    ),
    ("contextWindow", wire::Schema::OptJs(&wire::Schema::Num)),
    ("claudeEnableEffort", wire::Schema::Bool),
    (
        "poolExclude",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Str16)),
    ),
    (
        "autoJoinProviders",
        wire::Schema::OptJs(&wire::Schema::Bool),
    ),
    ("token", wire::Schema::Str16),
    (
        "restore",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            (
                "claudePreviousProviderId",
                wire::Schema::OptJs(&wire::Schema::Str16),
            ),
            ("appliedPort", wire::Schema::OptJs(&wire::Schema::Num)),
        ])),
    ),
]);

/// `blankBinding` over the binary wire.
#[wasm_bindgen(js_name = "blankBindingWire")]
pub fn blank_binding_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |_r, w| {
        let out = &port::blank_binding()?;
        wire::write_value(w, out, &S_BLANK_BINDING_OUT)?;
        Ok(())
    })
}

static S_DRAFT_FROM_BINDING_0: wire::Schema = wire::Schema::Dyn;
static S_DRAFT_FROM_BINDING_OUT: wire::Schema = wire::Schema::Obj(&[
    ("model", wire::Schema::Str16),
    ("extraModels", wire::Schema::Str16),
    ("opus", wire::Schema::Str16),
    ("sonnet", wire::Schema::Str16),
    ("haiku", wire::Schema::Str16),
    ("subagent", wire::Schema::Str16),
    ("unknownModel", wire::Schema::Str16),
    ("autoReviewModel", wire::Schema::Str16),
    ("titleModel", wire::Schema::Str16),
    ("compactionModel", wire::Schema::Str16),
    ("compactionReasoningEffort", wire::Schema::Str16),
    ("contextWindow", wire::Schema::Str16),
    ("claudeEnableEffort", wire::Schema::Bool),
    ("poolExclude", wire::Schema::List(&wire::Schema::Str16)),
    ("autoJoinProviders", wire::Schema::Bool),
]);

/// `draftFromBinding` over the binary wire.
#[wasm_bindgen(js_name = "draftFromBindingWire")]
pub fn draft_from_binding_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let binding: Value = wire::read_value(r, &S_DRAFT_FROM_BINDING_0)?;
        let out = &port::draft_from_binding(binding)?;
        wire::write_value(w, out, &S_DRAFT_FROM_BINDING_OUT)?;
        Ok(())
    })
}

static S_DEFAULT_PROVIDER_AGENTS_0: wire::Schema = wire::Schema::Dyn;
static S_DEFAULT_PROVIDER_AGENTS_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Str16);

/// `defaultProviderAgents` over the binary wire.
#[wasm_bindgen(js_name = "defaultProviderAgentsWire")]
pub fn default_provider_agents_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let payload: Value = wire::read_value(r, &S_DEFAULT_PROVIDER_AGENTS_0)?;
        let out = &port::default_provider_agents(payload)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_DEFAULT_PROVIDER_AGENTS_OUT.inner())?;
        }
        Ok(())
    })
}

static S_SKIPPED_AGENTS_0: wire::Schema = wire::Schema::Dyn;
static S_SKIPPED_AGENTS_1: wire::Schema = wire::Schema::Dyn;
static S_SKIPPED_AGENTS_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Str16);

/// `skippedAgents` over the binary wire.
#[wasm_bindgen(js_name = "skippedAgentsWire")]
pub fn skipped_agents_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let payload: Value = wire::read_value(r, &S_SKIPPED_AGENTS_0)?;
        let picked: Value = wire::read_value(r, &S_SKIPPED_AGENTS_1)?;
        let out = &port::skipped_agents(payload, picked)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_SKIPPED_AGENTS_OUT.inner())?;
        }
        Ok(())
    })
}

static S_POOL_EXCLUDE_TOGGLE_0: wire::Schema = wire::Schema::Dyn;

/// `poolExcludeToggle` over the binary wire.
#[wasm_bindgen(js_name = "poolExcludeToggleWire")]
pub fn pool_exclude_toggle_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let previous: Value = wire::read_value(r, &S_POOL_EXCLUDE_TOGGLE_0)?;
        let provider_id: String = r.string16()?;
        let on: bool = r.bool()?;
        let out = &port::pool_exclude_toggle(previous, provider_id, on)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            w.string16(x0);
        }
        Ok(())
    })
}

static S_PARSE_CONTEXT_WINDOW_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Num);

/// `parseContextWindow` over the binary wire.
#[wasm_bindgen(js_name = "parseContextWindowWire")]
pub fn parse_context_window_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let value: String = r.string16()?;
        let out = &port::parse_context_window(value)?;
        wire::write_value(w, out, &S_PARSE_CONTEXT_WINDOW_OUT)?;
        Ok(())
    })
}

static S_PARSE_KEY_LIMIT_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Num);

/// `parseKeyLimit` over the binary wire.
#[wasm_bindgen(js_name = "parseKeyLimitWire")]
pub fn parse_key_limit_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let value: String = r.string16()?;
        let out = &port::parse_key_limit(value)?;
        wire::write_value(w, out, &S_PARSE_KEY_LIMIT_OUT)?;
        Ok(())
    })
}

static S_PARSE_COMPACTION_NOTE_OUT: wire::Schema = wire::Schema::Dyn;

/// `parseCompactionNote` over the binary wire.
#[wasm_bindgen(js_name = "parseCompactionNoteWire")]
pub fn parse_compaction_note_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let reason: String = r.string16()?;
        let out = &port::parse_compaction_note(reason)?;
        wire::write_value(w, out, &S_PARSE_COMPACTION_NOTE_OUT)?;
        Ok(())
    })
}

static S_COMPACTION_NOTE_0: wire::Schema = wire::Schema::Dyn;
static S_COMPACTION_NOTE_OUT: wire::Schema = wire::Schema::Dyn;

/// `compactionNote` over the binary wire.
#[wasm_bindgen(js_name = "compactionNoteWire")]
pub fn compaction_note_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let reasons: Value = wire::read_value(r, &S_COMPACTION_NOTE_0)?;
        let out = &port::compaction_note(reasons)?;
        wire::write_value(w, out, &S_COMPACTION_NOTE_OUT)?;
        Ok(())
    })
}

static S_FOREIGN_COMPACTION_DROPPED_0: wire::Schema = wire::Schema::Dyn;

/// `foreignCompactionDropped` over the binary wire.
#[wasm_bindgen(js_name = "foreignCompactionDroppedWire")]
pub fn foreign_compaction_dropped_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let reasons: Value = wire::read_value(r, &S_FOREIGN_COMPACTION_DROPPED_0)?;
        let out = &port::foreign_compaction_dropped(reasons)?;
        w.bool(*out);
        Ok(())
    })
}

static S_QUEUE_WAIT_MS_0: wire::Schema = wire::Schema::Dyn;
static S_QUEUE_WAIT_MS_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Num);

/// `queueWaitMs` over the binary wire.
#[wasm_bindgen(js_name = "queueWaitMsWire")]
pub fn queue_wait_ms_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let reasons: Value = wire::read_value(r, &S_QUEUE_WAIT_MS_0)?;
        let out = &port::queue_wait_ms(reasons)?;
        wire::write_value(w, out, &S_QUEUE_WAIT_MS_OUT)?;
        Ok(())
    })
}

static S_BINDING_FROM_DRAFT_0: wire::Schema = wire::Schema::Dyn;
static S_BINDING_FROM_DRAFT_1: wire::Schema = wire::Schema::Dyn;
static S_BINDING_FROM_DRAFT_2: wire::Schema = wire::Schema::Dyn;
static S_BINDING_FROM_DRAFT_OUT: wire::Schema = wire::Schema::Obj(&[
    ("enabled", wire::Schema::Bool),
    ("model", wire::Schema::Str16),
    (
        "defaultModelSelection",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("providerId", wire::Schema::Str16),
            ("upstreamModel", wire::Schema::Str16),
            ("providerSource", wire::Schema::Str16),
            ("providerRemoved", wire::Schema::OptJs(&wire::Schema::Bool)),
        ])),
    ),
    ("extraModels", wire::Schema::List(&wire::Schema::Str16)),
    (
        "claudeRoles",
        wire::Schema::Obj(&[
            ("opus", wire::Schema::Str16),
            ("sonnet", wire::Schema::Str16),
            ("haiku", wire::Schema::Str16),
            ("subagent", wire::Schema::Str16),
        ]),
    ),
    ("unknownModel", wire::Schema::Str16),
    ("autoReviewModel", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("titleModel", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("compactionModel", wire::Schema::OptJs(&wire::Schema::Str16)),
    (
        "compactionReasoningEffort",
        wire::Schema::OptJs(&wire::Schema::Str16),
    ),
    ("contextWindow", wire::Schema::OptJs(&wire::Schema::Num)),
    ("claudeEnableEffort", wire::Schema::Bool),
    (
        "poolExclude",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Str16)),
    ),
    (
        "autoJoinProviders",
        wire::Schema::OptJs(&wire::Schema::Bool),
    ),
    ("token", wire::Schema::Str16),
    (
        "restore",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            (
                "claudePreviousProviderId",
                wire::Schema::OptJs(&wire::Schema::Str16),
            ),
            ("appliedPort", wire::Schema::OptJs(&wire::Schema::Num)),
        ])),
    ),
]);

/// `bindingFromDraft` over the binary wire.
#[wasm_bindgen(js_name = "bindingFromDraftWire")]
pub fn binding_from_draft_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let draft: Value = wire::read_value(r, &S_BINDING_FROM_DRAFT_0)?;
        let previous: Value = wire::read_value(r, &S_BINDING_FROM_DRAFT_1)?;
        let item: Value = wire::read_value(r, &S_BINDING_FROM_DRAFT_2)?;
        let out = &port::binding_from_draft(draft, previous, item)?;
        wire::write_value(w, out, &S_BINDING_FROM_DRAFT_OUT)?;
        Ok(())
    })
}

static S_DISABLED_GROUP_0: wire::Schema = wire::Schema::Dyn;
static S_DISABLED_GROUP_2: wire::Schema = wire::Schema::Dyn;
static S_DISABLED_GROUP_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("name", wire::Schema::Str16),
    ("disabled", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("mode", wire::Schema::Str16),
    ("sticky", wire::Schema::OptJs(&wire::Schema::Str16)),
    (
        "members",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("model", wire::Schema::Str16),
            ("weight", wire::Schema::Num),
        ])),
    ),
    ("hideMembers", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("pick", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("imageInputModel", wire::Schema::OptJs(&wire::Schema::Str16)),
]));

/// `disabledGroup` over the binary wire.
#[wasm_bindgen(js_name = "disabledGroupWire")]
pub fn disabled_group_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let payload: Value = wire::read_value(r, &S_DISABLED_GROUP_0)?;
        let model: String = r.string16()?;
        let selection: Value = wire::read_value(r, &S_DISABLED_GROUP_2)?;
        let out = &port::disabled_group(payload, model, selection)?;
        wire::write_value(w, out, &S_DISABLED_GROUP_OUT)?;
        Ok(())
    })
}

static S_PROVIDER_ENABLED_0: wire::Schema = wire::Schema::Dyn;

/// `providerEnabled` over the binary wire.
#[wasm_bindgen(js_name = "providerEnabledWire")]
pub fn provider_enabled_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let entry: Value = wire::read_value(r, &S_PROVIDER_ENABLED_0)?;
        let out = &port::provider_enabled(entry)?;
        w.bool(*out);
        Ok(())
    })
}

static S_AGENT_POOL_PROVIDERS_0: wire::Schema = wire::Schema::Dyn;
static S_AGENT_POOL_PROVIDERS_1: wire::Schema = wire::Schema::Dyn;
static S_AGENT_POOL_PROVIDERS_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("name", wire::Schema::Str16),
    ("baseUrl", wire::Schema::Str16),
    ("protocol", wire::Schema::Str16),
    ("official", wire::Schema::Bool),
    ("models", wire::Schema::List(&wire::Schema::Str16)),
    ("keyCount", wire::Schema::Num),
    ("enabled", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("source", wire::Schema::OptJs(&wire::Schema::Str16)),
    (
        "price",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("currency", wire::Schema::Str16),
            ("input", wire::Schema::Num),
            ("cachedInput", wire::Schema::Num),
            ("output", wire::Schema::Num),
        ])),
    ),
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
        "imageModels",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Str16)),
    ),
    ("modelMetadata", wire::Schema::OptJs(&wire::Schema::Str16)),
    (
        "modelWindows",
        wire::Schema::OptJs(&wire::Schema::Map16(&wire::Schema::Num)),
    ),
    ("contextWindow", wire::Schema::OptJs(&wire::Schema::Num)),
    (
        "autoCompact",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            (
                "models",
                wire::Schema::OptJs(&wire::Schema::Map16(&wire::Schema::Num)),
            ),
            ("fallback", wire::Schema::OptJs(&wire::Schema::Num)),
        ])),
    ),
    ("proxy", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("keyRpm", wire::Schema::OptJs(&wire::Schema::Num)),
    ("keyConcurrency", wire::Schema::OptJs(&wire::Schema::Num)),
    (
        "stripImageGenerationTool",
        wire::Schema::OptJs(&wire::Schema::Bool),
    ),
    (
        "stripResponsesImageDetail",
        wire::Schema::OptJs(&wire::Schema::Bool),
    ),
    (
        "disableToolSearch",
        wire::Schema::OptJs(&wire::Schema::Bool),
    ),
    ("account", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("subscriptionPolicy", wire::Schema::Dyn),
    (
        "subscriptionBlockReason",
        wire::Schema::OptJs(&wire::Schema::Str16),
    ),
    ("plan", wire::Schema::OptJs(&wire::Schema::Str16)),
    (
        "upstreamNew",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Str16)),
    ),
    (
        "upstreamGone",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Str16)),
    ),
    ("lentToCodex", wire::Schema::OptJs(&wire::Schema::Bool)),
    (
        "quota",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            (
                "windows",
                wire::Schema::List(&wire::Schema::Obj(&[
                    ("windowMinutes", wire::Schema::OptJs(&wire::Schema::Num)),
                    ("usedPercent", wire::Schema::Num),
                    ("resetsAtMs", wire::Schema::OptJs(&wire::Schema::Num)),
                    ("observedAtMs", wire::Schema::OptJs(&wire::Schema::Num)),
                ])),
            ),
            ("updatedAtMs", wire::Schema::Num),
            ("windowsComplete", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("officialAllowed", wire::Schema::OptJs(&wire::Schema::Bool)),
            (
                "officialLimitReached",
                wire::Schema::OptJs(&wire::Schema::Bool),
            ),
            ("creditsAvailable", wire::Schema::OptJs(&wire::Schema::Bool)),
        ])),
    ),
    ("quotaApi", wire::Schema::OptJs(&wire::Schema::Bool)),
    (
        "keys",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Obj(&[
            ("id", wire::Schema::Str16),
            ("masked", wire::Schema::Str16),
        ]))),
    ),
    ("balanceApi", wire::Schema::OptJs(&wire::Schema::Bool)),
    (
        "newapiAccess",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("masked", wire::Schema::Str16),
            ("userId", wire::Schema::Str16),
        ])),
    ),
    (
        "customHeaders",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Obj(&[
            ("key", wire::Schema::Str16),
            ("value", wire::Schema::Str16),
            ("masked", wire::Schema::OptJs(&wire::Schema::Str16)),
        ]))),
    ),
]));

/// `agentPoolProviders` over the binary wire.
#[wasm_bindgen(js_name = "agentPoolProvidersWire")]
pub fn agent_pool_providers_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let payload: Value = wire::read_value(r, &S_AGENT_POOL_PROVIDERS_0)?;
        let exclude: Value = wire::read_value(r, &S_AGENT_POOL_PROVIDERS_1)?;
        let out = &port::agent_pool_providers(payload, exclude)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_AGENT_POOL_PROVIDERS_OUT.inner())?;
        }
        Ok(())
    })
}

static S_AGENT_MODELS_0: wire::Schema = wire::Schema::Dyn;
static S_AGENT_MODELS_1: wire::Schema = wire::Schema::Dyn;

/// `agentModels` over the binary wire.
#[wasm_bindgen(js_name = "agentModelsWire")]
pub fn agent_models_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let payload: Value = wire::read_value(r, &S_AGENT_MODELS_0)?;
        let exclude: Value = wire::read_value(r, &S_AGENT_MODELS_1)?;
        let out = &port::agent_models(payload, exclude)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            w.string16(x0);
        }
        Ok(())
    })
}

static S_QUALIFIED_PROVIDER_0: wire::Schema = wire::Schema::Dyn;
static S_QUALIFIED_PROVIDER_2: wire::Schema = wire::Schema::Dyn;
static S_QUALIFIED_PROVIDER_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Obj(&[
    (
        "entry",
        wire::Schema::Obj(&[
            ("id", wire::Schema::Str16),
            ("name", wire::Schema::Str16),
            ("baseUrl", wire::Schema::Str16),
            ("protocol", wire::Schema::Str16),
            ("official", wire::Schema::Bool),
            ("models", wire::Schema::List(&wire::Schema::Str16)),
            ("keyCount", wire::Schema::Num),
            ("enabled", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("source", wire::Schema::OptJs(&wire::Schema::Str16)),
            (
                "price",
                wire::Schema::OptJs(&wire::Schema::Obj(&[
                    ("currency", wire::Schema::Str16),
                    ("input", wire::Schema::Num),
                    ("cachedInput", wire::Schema::Num),
                    ("output", wire::Schema::Num),
                ])),
            ),
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
                "imageModels",
                wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Str16)),
            ),
            ("modelMetadata", wire::Schema::OptJs(&wire::Schema::Str16)),
            (
                "modelWindows",
                wire::Schema::OptJs(&wire::Schema::Map16(&wire::Schema::Num)),
            ),
            ("contextWindow", wire::Schema::OptJs(&wire::Schema::Num)),
            (
                "autoCompact",
                wire::Schema::OptJs(&wire::Schema::Obj(&[
                    (
                        "models",
                        wire::Schema::OptJs(&wire::Schema::Map16(&wire::Schema::Dyn)),
                    ),
                    ("fallback", wire::Schema::OptJs(&wire::Schema::Num)),
                ])),
            ),
            ("proxy", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("keyRpm", wire::Schema::OptJs(&wire::Schema::Num)),
            ("keyConcurrency", wire::Schema::OptJs(&wire::Schema::Num)),
            (
                "stripImageGenerationTool",
                wire::Schema::OptJs(&wire::Schema::Bool),
            ),
            (
                "stripResponsesImageDetail",
                wire::Schema::OptJs(&wire::Schema::Bool),
            ),
            (
                "disableToolSearch",
                wire::Schema::OptJs(&wire::Schema::Bool),
            ),
            ("account", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("subscriptionPolicy", wire::Schema::Dyn),
            (
                "subscriptionBlockReason",
                wire::Schema::OptJs(&wire::Schema::Str16),
            ),
            ("plan", wire::Schema::OptJs(&wire::Schema::Str16)),
            (
                "upstreamNew",
                wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Str16)),
            ),
            (
                "upstreamGone",
                wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Str16)),
            ),
            ("lentToCodex", wire::Schema::OptJs(&wire::Schema::Bool)),
            (
                "quota",
                wire::Schema::OptJs(&wire::Schema::Obj(&[
                    (
                        "windows",
                        wire::Schema::List(&wire::Schema::Obj(&[
                            ("windowMinutes", wire::Schema::Dyn),
                            ("usedPercent", wire::Schema::Dyn),
                            ("resetsAtMs", wire::Schema::Dyn),
                            ("observedAtMs", wire::Schema::Dyn),
                        ])),
                    ),
                    ("updatedAtMs", wire::Schema::Num),
                    ("windowsComplete", wire::Schema::OptJs(&wire::Schema::Bool)),
                    ("officialAllowed", wire::Schema::OptJs(&wire::Schema::Bool)),
                    (
                        "officialLimitReached",
                        wire::Schema::OptJs(&wire::Schema::Bool),
                    ),
                    ("creditsAvailable", wire::Schema::OptJs(&wire::Schema::Bool)),
                ])),
            ),
            ("quotaApi", wire::Schema::OptJs(&wire::Schema::Bool)),
            (
                "keys",
                wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Obj(&[
                    ("id", wire::Schema::Str16),
                    ("masked", wire::Schema::Str16),
                ]))),
            ),
            ("balanceApi", wire::Schema::OptJs(&wire::Schema::Bool)),
            (
                "newapiAccess",
                wire::Schema::OptJs(&wire::Schema::Obj(&[
                    ("masked", wire::Schema::Str16),
                    ("userId", wire::Schema::Str16),
                ])),
            ),
            (
                "customHeaders",
                wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Obj(&[
                    ("key", wire::Schema::Str16),
                    ("value", wire::Schema::Str16),
                    ("masked", wire::Schema::OptJs(&wire::Schema::Str16)),
                ]))),
            ),
        ]),
    ),
    ("model", wire::Schema::Str16),
]));

/// `qualifiedProvider` over the binary wire.
#[wasm_bindgen(js_name = "qualifiedProviderWire")]
pub fn qualified_provider_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let payload: Value = wire::read_value(r, &S_QUALIFIED_PROVIDER_0)?;
        let model: String = r.string16()?;
        let exclude: Value = wire::read_value(r, &S_QUALIFIED_PROVIDER_2)?;
        let out = &port::qualified_provider(payload, model, exclude)?;
        wire::write_value(w, out, &S_QUALIFIED_PROVIDER_OUT)?;
        Ok(())
    })
}

static S_MODEL_ROUTABLE_0: wire::Schema = wire::Schema::Dyn;
static S_MODEL_ROUTABLE_2: wire::Schema = wire::Schema::Dyn;

/// `modelRoutable` over the binary wire.
#[wasm_bindgen(js_name = "modelRoutableWire")]
pub fn model_routable_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let payload: Value = wire::read_value(r, &S_MODEL_ROUTABLE_0)?;
        let model: String = r.string16()?;
        let exclude: Value = wire::read_value(r, &S_MODEL_ROUTABLE_2)?;
        let out = &port::model_routable(payload, model, exclude)?;
        w.bool(*out);
        Ok(())
    })
}

static S_VALIDATE_DRAFT_0: wire::Schema = wire::Schema::Dyn;
static S_VALIDATE_DRAFT_1: wire::Schema = wire::Schema::Dyn;
static S_VALIDATE_DRAFT_2: wire::Schema = wire::Schema::Dyn;
static S_VALIDATE_DRAFT_3: wire::Schema = wire::Schema::Dyn;
static S_VALIDATE_DRAFT_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Str16);

/// `validateDraft` over the binary wire.
#[wasm_bindgen(js_name = "validateDraftWire")]
pub fn validate_draft_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let draft: Value = wire::read_value(r, &S_VALIDATE_DRAFT_0)?;
        let payload: Value = wire::read_value(r, &S_VALIDATE_DRAFT_1)?;
        let retained_model: Value = wire::read_value(r, &S_VALIDATE_DRAFT_2)?;
        let retained_selection: Value = wire::read_value(r, &S_VALIDATE_DRAFT_3)?;
        let out = &port::validate_draft(draft, payload, retained_model, retained_selection)?;
        wire::write_value(w, out, &S_VALIDATE_DRAFT_OUT)?;
        Ok(())
    })
}

static S_PROTOCOL_LABEL_0: wire::Schema = wire::Schema::Dyn;

/// `protocolLabel` over the binary wire.
#[wasm_bindgen(js_name = "protocolLabelWire")]
pub fn protocol_label_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let protocol: Value = wire::read_value(r, &S_PROTOCOL_LABEL_0)?;
        let out = &port::protocol_label(protocol)?;
        w.string16(out);
        Ok(())
    })
}

static S_GLANCE_AGENTS_0: wire::Schema = wire::Schema::Dyn;
static S_GLANCE_AGENTS_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("agent", wire::Schema::Str16),
    ("enabled", wire::Schema::Bool),
]));

/// `glanceAgents` over the binary wire.
#[wasm_bindgen(js_name = "glanceAgentsWire")]
pub fn glance_agents_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let agents: Value = wire::read_value(r, &S_GLANCE_AGENTS_0)?;
        let out = &port::glance_agents(agents)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_GLANCE_AGENTS_OUT.inner())?;
        }
        Ok(())
    })
}

static S_REMEMBER_AGENT_NAMES_0: wire::Schema = wire::Schema::Dyn;
static S_REMEMBER_AGENT_NAMES_OUT: wire::Schema = wire::Schema::Dyn;

/// `rememberAgentNames` over the binary wire.
#[wasm_bindgen(js_name = "rememberAgentNamesWire")]
pub fn remember_agent_names_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let agents: Value = wire::read_value(r, &S_REMEMBER_AGENT_NAMES_0)?;
        let out = &port::remember_agent_names(agents)?;
        wire::write_value(w, out, &S_REMEMBER_AGENT_NAMES_OUT)?;
        Ok(())
    })
}

static S_VISIBLE_OWN_MODELS_0: wire::Schema = wire::Schema::Dyn;
static S_VISIBLE_OWN_MODELS_1: wire::Schema = wire::Schema::Dyn;
static S_VISIBLE_OWN_MODELS_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Obj(&[
    ("vendor", wire::Schema::Str16),
    (
        "models",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("id", wire::Schema::Str16),
            ("label", wire::Schema::Str16),
        ])),
    ),
    ("current", wire::Schema::Str16),
]));

/// `visibleOwnModels` over the binary wire.
#[wasm_bindgen(js_name = "visibleOwnModelsWire")]
pub fn visible_own_models_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let agent: Value = wire::read_value(r, &S_VISIBLE_OWN_MODELS_0)?;
        let payload: Value = wire::read_value(r, &S_VISIBLE_OWN_MODELS_1)?;
        let out = &port::visible_own_models(agent, payload)?;
        wire::write_value(w, out, &S_VISIBLE_OWN_MODELS_OUT)?;
        Ok(())
    })
}

static S_SERVING_PROVIDERS_0: wire::Schema = wire::Schema::Dyn;
static S_SERVING_PROVIDERS_2: wire::Schema = wire::Schema::Dyn;
static S_SERVING_PROVIDERS_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("name", wire::Schema::Str16),
    ("baseUrl", wire::Schema::Str16),
    ("protocol", wire::Schema::Str16),
    ("official", wire::Schema::Bool),
    ("models", wire::Schema::List(&wire::Schema::Str16)),
    ("keyCount", wire::Schema::Num),
    ("enabled", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("source", wire::Schema::OptJs(&wire::Schema::Str16)),
    (
        "price",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("currency", wire::Schema::Str16),
            ("input", wire::Schema::Num),
            ("cachedInput", wire::Schema::Num),
            ("output", wire::Schema::Num),
        ])),
    ),
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
        "imageModels",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Str16)),
    ),
    ("modelMetadata", wire::Schema::OptJs(&wire::Schema::Str16)),
    (
        "modelWindows",
        wire::Schema::OptJs(&wire::Schema::Map16(&wire::Schema::Num)),
    ),
    ("contextWindow", wire::Schema::OptJs(&wire::Schema::Num)),
    (
        "autoCompact",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            (
                "models",
                wire::Schema::OptJs(&wire::Schema::Map16(&wire::Schema::Num)),
            ),
            ("fallback", wire::Schema::OptJs(&wire::Schema::Num)),
        ])),
    ),
    ("proxy", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("keyRpm", wire::Schema::OptJs(&wire::Schema::Num)),
    ("keyConcurrency", wire::Schema::OptJs(&wire::Schema::Num)),
    (
        "stripImageGenerationTool",
        wire::Schema::OptJs(&wire::Schema::Bool),
    ),
    (
        "stripResponsesImageDetail",
        wire::Schema::OptJs(&wire::Schema::Bool),
    ),
    (
        "disableToolSearch",
        wire::Schema::OptJs(&wire::Schema::Bool),
    ),
    ("account", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("subscriptionPolicy", wire::Schema::Dyn),
    (
        "subscriptionBlockReason",
        wire::Schema::OptJs(&wire::Schema::Str16),
    ),
    ("plan", wire::Schema::OptJs(&wire::Schema::Str16)),
    (
        "upstreamNew",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Str16)),
    ),
    (
        "upstreamGone",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Str16)),
    ),
    ("lentToCodex", wire::Schema::OptJs(&wire::Schema::Bool)),
    (
        "quota",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            (
                "windows",
                wire::Schema::List(&wire::Schema::Obj(&[
                    ("windowMinutes", wire::Schema::OptJs(&wire::Schema::Num)),
                    ("usedPercent", wire::Schema::Num),
                    ("resetsAtMs", wire::Schema::OptJs(&wire::Schema::Num)),
                    ("observedAtMs", wire::Schema::OptJs(&wire::Schema::Num)),
                ])),
            ),
            ("updatedAtMs", wire::Schema::Num),
            ("windowsComplete", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("officialAllowed", wire::Schema::OptJs(&wire::Schema::Bool)),
            (
                "officialLimitReached",
                wire::Schema::OptJs(&wire::Schema::Bool),
            ),
            ("creditsAvailable", wire::Schema::OptJs(&wire::Schema::Bool)),
        ])),
    ),
    ("quotaApi", wire::Schema::OptJs(&wire::Schema::Bool)),
    (
        "keys",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Obj(&[
            ("id", wire::Schema::Str16),
            ("masked", wire::Schema::Str16),
        ]))),
    ),
    ("balanceApi", wire::Schema::OptJs(&wire::Schema::Bool)),
    (
        "newapiAccess",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("masked", wire::Schema::Str16),
            ("userId", wire::Schema::Str16),
        ])),
    ),
    (
        "customHeaders",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Obj(&[
            ("key", wire::Schema::Str16),
            ("value", wire::Schema::Str16),
            ("masked", wire::Schema::OptJs(&wire::Schema::Str16)),
        ]))),
    ),
]));

/// `servingProviders` over the binary wire.
#[wasm_bindgen(js_name = "servingProvidersWire")]
pub fn serving_providers_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let payload: Value = wire::read_value(r, &S_SERVING_PROVIDERS_0)?;
        let model: String = r.string16()?;
        let exclude: Value = wire::read_value(r, &S_SERVING_PROVIDERS_2)?;
        let out = &port::serving_providers(payload, model, exclude)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_SERVING_PROVIDERS_OUT.inner())?;
        }
        Ok(())
    })
}

static S_GROUP_DISPLAY_NAME_0: wire::Schema = wire::Schema::Dyn;
static S_GROUP_DISPLAY_NAME_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Str16);

/// `groupDisplayName` over the binary wire.
#[wasm_bindgen(js_name = "groupDisplayNameWire")]
pub fn group_display_name_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let payload: Value = wire::read_value(r, &S_GROUP_DISPLAY_NAME_0)?;
        let model: String = r.string16()?;
        let out = &port::group_display_name(payload, model)?;
        wire::write_value(w, out, &S_GROUP_DISPLAY_NAME_OUT)?;
        Ok(())
    })
}

static S_MODEL_DISPLAY_0: wire::Schema = wire::Schema::Dyn;
static S_MODEL_DISPLAY_2: wire::Schema = wire::Schema::Dyn;
static S_MODEL_DISPLAY_3: wire::Schema = wire::Schema::Dyn;
static S_MODEL_DISPLAY_OUT: wire::Schema = wire::Schema::Obj(&[
    ("title", wire::Schema::Str16),
    ("sub", wire::Schema::Str16),
    ("group", wire::Schema::Bool),
    ("auto", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("issue", wire::Schema::OptJs(&wire::Schema::Str16)),
]);

/// `modelDisplay` over the binary wire.
#[wasm_bindgen(js_name = "modelDisplayWire")]
pub fn model_display_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let payload: Value = wire::read_value(r, &S_MODEL_DISPLAY_0)?;
        let model: String = r.string16()?;
        let exclude: Value = wire::read_value(r, &S_MODEL_DISPLAY_2)?;
        let selection: Value = wire::read_value(r, &S_MODEL_DISPLAY_3)?;
        let out = &port::model_display(payload, model, exclude, selection)?;
        wire::write_value(w, out, &S_MODEL_DISPLAY_OUT)?;
        Ok(())
    })
}

static S_PARSE_WEIGHT_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Num);

/// `parseWeight` over the binary wire.
#[wasm_bindgen(js_name = "parseWeightWire")]
pub fn parse_weight_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let text: String = r.string16()?;
        let out = &port::parse_weight(text)?;
        wire::write_value(w, out, &S_PARSE_WEIGHT_OUT)?;
        Ok(())
    })
}

static S_PARSE_GATEWAY_PORT_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Num);

/// `parseGatewayPort` over the binary wire.
#[wasm_bindgen(js_name = "parseGatewayPortWire")]
pub fn parse_gateway_port_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let text: String = r.string16()?;
        let out = &port::parse_gateway_port(text)?;
        wire::write_value(w, out, &S_PARSE_GATEWAY_PORT_OUT)?;
        Ok(())
    })
}

static S_PATTERN_MATCHES_0: wire::Schema = wire::Schema::Dyn;

/// `patternMatches` over the binary wire.
#[wasm_bindgen(js_name = "patternMatchesWire")]
pub fn pattern_matches_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let payload: Value = wire::read_value(r, &S_PATTERN_MATCHES_0)?;
        let pattern: String = r.string16()?;
        let out = &port::pattern_matches(payload, pattern)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            w.string16(x0);
        }
        Ok(())
    })
}

static S_SPEED_OF_0: wire::Schema = wire::Schema::Dyn;
static S_SPEED_OF_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Num);

/// `speedOf` over the binary wire.
#[wasm_bindgen(js_name = "speedOfWire")]
pub fn speed_of_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let payload: Value = wire::read_value(r, &S_SPEED_OF_0)?;
        let relay_id: String = r.string16()?;
        let model: String = r.string16()?;
        let out = &port::speed_of(payload, relay_id, model)?;
        wire::write_value(w, out, &S_SPEED_OF_OUT)?;
        Ok(())
    })
}

static S_QUOTA_WINDOW_LABEL_0: wire::Schema = wire::Schema::Dyn;
static S_QUOTA_WINDOW_LABEL_OUT: wire::Schema = wire::Schema::Obj(&[
    ("text", wire::Schema::Str16),
    ("args", wire::Schema::List(&wire::Schema::Num)),
]);

/// `quotaWindowLabel` over the binary wire.
#[wasm_bindgen(js_name = "quotaWindowLabelWire")]
pub fn quota_window_label_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let window: Value = wire::read_value(r, &S_QUOTA_WINDOW_LABEL_0)?;
        let out = &port::quota_window_label(window)?;
        wire::write_value(w, out, &S_QUOTA_WINDOW_LABEL_OUT)?;
        Ok(())
    })
}

static S_QUOTA_SUPPORTED_0: wire::Schema = wire::Schema::Dyn;

/// `quotaSupported` over the binary wire.
#[wasm_bindgen(js_name = "quotaSupportedWire")]
pub fn quota_supported_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let entry: Value = wire::read_value(r, &S_QUOTA_SUPPORTED_0)?;
        let out = &port::quota_supported(entry)?;
        w.bool(*out);
        Ok(())
    })
}

static S_QUOTA_ENTRIES_0: wire::Schema = wire::Schema::Dyn;
static S_QUOTA_ENTRIES_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("name", wire::Schema::Str16),
    ("baseUrl", wire::Schema::Str16),
    ("protocol", wire::Schema::Str16),
    ("official", wire::Schema::Bool),
    ("models", wire::Schema::List(&wire::Schema::Str16)),
    ("keyCount", wire::Schema::Num),
    ("enabled", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("source", wire::Schema::OptJs(&wire::Schema::Str16)),
    (
        "price",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("currency", wire::Schema::Str16),
            ("input", wire::Schema::Num),
            ("cachedInput", wire::Schema::Num),
            ("output", wire::Schema::Num),
        ])),
    ),
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
        "imageModels",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Str16)),
    ),
    ("modelMetadata", wire::Schema::OptJs(&wire::Schema::Str16)),
    (
        "modelWindows",
        wire::Schema::OptJs(&wire::Schema::Map16(&wire::Schema::Num)),
    ),
    ("contextWindow", wire::Schema::OptJs(&wire::Schema::Num)),
    (
        "autoCompact",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            (
                "models",
                wire::Schema::OptJs(&wire::Schema::Map16(&wire::Schema::Num)),
            ),
            ("fallback", wire::Schema::OptJs(&wire::Schema::Num)),
        ])),
    ),
    ("proxy", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("keyRpm", wire::Schema::OptJs(&wire::Schema::Num)),
    ("keyConcurrency", wire::Schema::OptJs(&wire::Schema::Num)),
    (
        "stripImageGenerationTool",
        wire::Schema::OptJs(&wire::Schema::Bool),
    ),
    (
        "stripResponsesImageDetail",
        wire::Schema::OptJs(&wire::Schema::Bool),
    ),
    (
        "disableToolSearch",
        wire::Schema::OptJs(&wire::Schema::Bool),
    ),
    ("account", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("subscriptionPolicy", wire::Schema::Dyn),
    (
        "subscriptionBlockReason",
        wire::Schema::OptJs(&wire::Schema::Str16),
    ),
    ("plan", wire::Schema::OptJs(&wire::Schema::Str16)),
    (
        "upstreamNew",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Str16)),
    ),
    (
        "upstreamGone",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Str16)),
    ),
    ("lentToCodex", wire::Schema::OptJs(&wire::Schema::Bool)),
    (
        "quota",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            (
                "windows",
                wire::Schema::List(&wire::Schema::Obj(&[
                    ("windowMinutes", wire::Schema::OptJs(&wire::Schema::Num)),
                    ("usedPercent", wire::Schema::Num),
                    ("resetsAtMs", wire::Schema::OptJs(&wire::Schema::Num)),
                    ("observedAtMs", wire::Schema::OptJs(&wire::Schema::Num)),
                ])),
            ),
            ("updatedAtMs", wire::Schema::Num),
            ("windowsComplete", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("officialAllowed", wire::Schema::OptJs(&wire::Schema::Bool)),
            (
                "officialLimitReached",
                wire::Schema::OptJs(&wire::Schema::Bool),
            ),
            ("creditsAvailable", wire::Schema::OptJs(&wire::Schema::Bool)),
        ])),
    ),
    ("quotaApi", wire::Schema::OptJs(&wire::Schema::Bool)),
    (
        "keys",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Obj(&[
            ("id", wire::Schema::Str16),
            ("masked", wire::Schema::Str16),
        ]))),
    ),
    ("balanceApi", wire::Schema::OptJs(&wire::Schema::Bool)),
    (
        "newapiAccess",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("masked", wire::Schema::Str16),
            ("userId", wire::Schema::Str16),
        ])),
    ),
    (
        "customHeaders",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Obj(&[
            ("key", wire::Schema::Str16),
            ("value", wire::Schema::Str16),
            ("masked", wire::Schema::OptJs(&wire::Schema::Str16)),
        ]))),
    ),
]));

/// `quotaEntries` over the binary wire.
#[wasm_bindgen(js_name = "quotaEntriesWire")]
pub fn quota_entries_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let pool: Value = wire::read_value(r, &S_QUOTA_ENTRIES_0)?;
        let out = &port::quota_entries(pool)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_QUOTA_ENTRIES_OUT.inner())?;
        }
        Ok(())
    })
}

static S_QUOTA_REMAINING_0: wire::Schema = wire::Schema::Dyn;

/// `quotaRemaining` over the binary wire.
#[wasm_bindgen(js_name = "quotaRemainingWire")]
pub fn quota_remaining_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let quota: Value = wire::read_value(r, &S_QUOTA_REMAINING_0)?;
        let now_ms: f64 = r.f64()?;
        let out = &port::quota_remaining(quota, now_ms)?;
        w.f64(*out);
        Ok(())
    })
}

static S_QUOTA_WINDOW_FRESH_0: wire::Schema = wire::Schema::Dyn;
static S_QUOTA_WINDOW_FRESH_1: wire::Schema = wire::Schema::Dyn;
static S_QUOTA_WINDOW_FRESH_3: wire::Schema = wire::Schema::Dyn;

/// `quotaWindowFresh` over the binary wire.
#[wasm_bindgen(js_name = "quotaWindowFreshWire")]
pub fn quota_window_fresh_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let quota: Value = wire::read_value(r, &S_QUOTA_WINDOW_FRESH_0)?;
        let window: Value = wire::read_value(r, &S_QUOTA_WINDOW_FRESH_1)?;
        let now_ms: f64 = r.f64()?;
        let max_age_seconds: Value = wire::read_value(r, &S_QUOTA_WINDOW_FRESH_3)?;
        let out = &port::quota_window_fresh(quota, window, now_ms, max_age_seconds)?;
        w.bool(*out);
        Ok(())
    })
}

static S_SUBSCRIPTION_BLOCK_REASON_0: wire::Schema = wire::Schema::Dyn;
static S_SUBSCRIPTION_BLOCK_REASON_1: wire::Schema = wire::Schema::Dyn;
static S_SUBSCRIPTION_BLOCK_REASON_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Str16);

/// `subscriptionBlockReason` over the binary wire.
#[wasm_bindgen(js_name = "subscriptionBlockReasonWire")]
pub fn subscription_block_reason_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let entry: Value = wire::read_value(r, &S_SUBSCRIPTION_BLOCK_REASON_0)?;
        let now_ms: Value = wire::read_value(r, &S_SUBSCRIPTION_BLOCK_REASON_1)?;
        let out = &port::subscription_block_reason(entry, now_ms)?;
        wire::write_value(w, out, &S_SUBSCRIPTION_BLOCK_REASON_OUT)?;
        Ok(())
    })
}

static S_MEMBER_LABEL_0: wire::Schema = wire::Schema::Dyn;

/// `memberLabel` over the binary wire.
#[wasm_bindgen(js_name = "memberLabelWire")]
pub fn member_label_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let payload: Value = wire::read_value(r, &S_MEMBER_LABEL_0)?;
        let model: String = r.string16()?;
        let out = &port::member_label(payload, model)?;
        w.string16(out);
        Ok(())
    })
}

static S_MEMBER_STATUS_0: wire::Schema = wire::Schema::Dyn;
static S_MEMBER_STATUS_2: wire::Schema = wire::Schema::Dyn;
static S_MEMBER_STATUS_OUT: wire::Schema = wire::Schema::Obj(&[
    ("state", wire::Schema::Str16),
    ("text", wire::Schema::Str16),
    ("args", wire::Schema::List(&wire::Schema::Dyn)),
]);

/// `memberStatus` over the binary wire.
#[wasm_bindgen(js_name = "memberStatusWire")]
pub fn member_status_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let payload: Value = wire::read_value(r, &S_MEMBER_STATUS_0)?;
        let relay_id: String = r.string16()?;
        let now_ms: Value = wire::read_value(r, &S_MEMBER_STATUS_2)?;
        let out = &port::member_status(payload, relay_id, now_ms)?;
        wire::write_value(w, out, &S_MEMBER_STATUS_OUT)?;
        Ok(())
    })
}

static S_DISPATCH_STATUS_0: wire::Schema = wire::Schema::Dyn;
static S_DISPATCH_STATUS_3: wire::Schema = wire::Schema::Dyn;
static S_DISPATCH_STATUS_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Obj(&[
    ("state", wire::Schema::Str16),
    ("text", wire::Schema::Str16),
    ("args", wire::Schema::List(&wire::Schema::Dyn)),
]));

/// `dispatchStatus` over the binary wire.
#[wasm_bindgen(js_name = "dispatchStatusWire")]
pub fn dispatch_status_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let payload: Value = wire::read_value(r, &S_DISPATCH_STATUS_0)?;
        let relay_id: String = r.string16()?;
        let model: String = r.string16()?;
        let now_ms: Value = wire::read_value(r, &S_DISPATCH_STATUS_3)?;
        let out = &port::dispatch_status(payload, relay_id, model, now_ms)?;
        wire::write_value(w, out, &S_DISPATCH_STATUS_OUT)?;
        Ok(())
    })
}

static S_ROUTE_NARRATIVE_1: wire::Schema = wire::Schema::Dyn;
static S_ROUTE_NARRATIVE_2: wire::Schema = wire::Schema::Dyn;
static S_ROUTE_NARRATIVE_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("tone", wire::Schema::Str16),
    ("text", wire::Schema::Str16),
    ("args", wire::Schema::List(&wire::Schema::Dyn)),
]));

/// `routeNarrative` over the binary wire.
#[wasm_bindgen(js_name = "routeNarrativeWire")]
pub fn route_narrative_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let agent: String = r.string16()?;
        let record: Value = wire::read_value(r, &S_ROUTE_NARRATIVE_1)?;
        let payload: Value = wire::read_value(r, &S_ROUTE_NARRATIVE_2)?;
        let out = &port::route_narrative(agent, record, payload)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_ROUTE_NARRATIVE_OUT.inner())?;
        }
        Ok(())
    })
}

static S_CONNECTED_AGENTS_0: wire::Schema = wire::Schema::Dyn;
static S_CONNECTED_AGENTS_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Str16);

/// `connectedAgents` over the binary wire.
#[wasm_bindgen(js_name = "connectedAgentsWire")]
pub fn connected_agents_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let payload: Value = wire::read_value(r, &S_CONNECTED_AGENTS_0)?;
        let out = &port::connected_agents(payload)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_CONNECTED_AGENTS_OUT.inner())?;
        }
        Ok(())
    })
}

static S_MODEL_DISPATCH_0: wire::Schema = wire::Schema::Dyn;
static S_MODEL_DISPATCH_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("model", wire::Schema::Str16),
    (
        "providers",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("id", wire::Schema::Str16),
            ("name", wire::Schema::Str16),
            ("baseUrl", wire::Schema::Str16),
            ("excludedBy", wire::Schema::List(&wire::Schema::Str16)),
        ])),
    ),
]));

/// `modelDispatch` over the binary wire.
#[wasm_bindgen(js_name = "modelDispatchWire")]
pub fn model_dispatch_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let payload: Value = wire::read_value(r, &S_MODEL_DISPATCH_0)?;
        let out = &port::model_dispatch(payload)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_MODEL_DISPATCH_OUT.inner())?;
        }
        Ok(())
    })
}

static S_ACTIVE_SPONSOR_IDS_0: wire::Schema = wire::Schema::Dyn;

/// `activeSponsorIds` over the binary wire.
#[wasm_bindgen(js_name = "activeSponsorIdsWire")]
pub fn active_sponsor_ids_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let now: Value = wire::read_value(r, &S_ACTIVE_SPONSOR_IDS_0)?;
        let out = &port::active_sponsor_ids(now)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            w.string16(x0);
        }
        Ok(())
    })
}

static S_GATEWAY_PRESETS_0: wire::Schema = wire::Schema::Dyn;
static S_GATEWAY_PRESETS_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("key", wire::Schema::Str16),
    ("name", wire::Schema::Str16),
    ("baseUrl", wire::Schema::Str16),
    ("protocol", wire::Schema::Dyn),
    ("model", wire::Schema::Str16),
    ("modelList", wire::Schema::List(&wire::Schema::Str16)),
    ("section", wire::Schema::Str16),
    ("defaultKey", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("apiKeyUrl", wire::Schema::OptJs(&wire::Schema::Str16)),
]));

/// `gatewayPresets` over the binary wire.
#[wasm_bindgen(js_name = "gatewayPresetsWire")]
pub fn gateway_presets_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let now: Value = wire::read_value(r, &S_GATEWAY_PRESETS_0)?;
        let out = &port::gateway_presets(now)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_GATEWAY_PRESETS_OUT.inner())?;
        }
        Ok(())
    })
}

static S_PRESET_ADDED_0: wire::Schema = wire::Schema::Dyn;
static S_PRESET_ADDED_1: wire::Schema = wire::Schema::Dyn;

/// `presetAdded` over the binary wire.
#[wasm_bindgen(js_name = "presetAddedWire")]
pub fn preset_added_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let payload: Value = wire::read_value(r, &S_PRESET_ADDED_0)?;
        let preset: Value = wire::read_value(r, &S_PRESET_ADDED_1)?;
        let out = &port::preset_added(payload, preset)?;
        w.bool(*out);
        Ok(())
    })
}

static S_PROVIDER_DRAFT_FROM_PRESET_0: wire::Schema = wire::Schema::Dyn;
static S_PROVIDER_DRAFT_FROM_PRESET_OUT: wire::Schema = wire::Schema::Obj(&[
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

/// `providerDraftFromPreset` over the binary wire.
#[wasm_bindgen(js_name = "providerDraftFromPresetWire")]
pub fn provider_draft_from_preset_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let preset: Value = wire::read_value(r, &S_PROVIDER_DRAFT_FROM_PRESET_0)?;
        let out = &port::provider_draft_from_preset(preset)?;
        wire::write_value(w, out, &S_PROVIDER_DRAFT_FROM_PRESET_OUT)?;
        Ok(())
    })
}

static S_PROVIDER_DRAFT_FROM_ENTRY_0: wire::Schema = wire::Schema::Dyn;
static S_PROVIDER_DRAFT_FROM_ENTRY_OUT: wire::Schema = wire::Schema::Obj(&[
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

/// `providerDraftFromEntry` over the binary wire.
#[wasm_bindgen(js_name = "providerDraftFromEntryWire")]
pub fn provider_draft_from_entry_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let entry: Value = wire::read_value(r, &S_PROVIDER_DRAFT_FROM_ENTRY_0)?;
        let out = &port::provider_draft_from_entry(entry)?;
        wire::write_value(w, out, &S_PROVIDER_DRAFT_FROM_ENTRY_OUT)?;
        Ok(())
    })
}

static S_PROVIDER_INPUT_0: wire::Schema = wire::Schema::Dyn;
static S_PROVIDER_INPUT_OUT: wire::Schema = wire::Schema::Dyn;

/// `providerInput` over the binary wire.
#[wasm_bindgen(js_name = "providerInputWire")]
pub fn provider_input_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let draft: Value = wire::read_value(r, &S_PROVIDER_INPUT_0)?;
        let out = &port::provider_input(draft)?;
        wire::write_value(w, out, &S_PROVIDER_INPUT_OUT)?;
        Ok(())
    })
}

static S_PRICE_FROM_DRAFT_0: wire::Schema = wire::Schema::Dyn;
static S_PRICE_FROM_DRAFT_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Obj(&[
    ("currency", wire::Schema::Str16),
    ("input", wire::Schema::Num),
    ("cachedInput", wire::Schema::Num),
    ("output", wire::Schema::Num),
]));

/// `priceFromDraft` over the binary wire.
#[wasm_bindgen(js_name = "priceFromDraftWire")]
pub fn price_from_draft_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let draft: Value = wire::read_value(r, &S_PRICE_FROM_DRAFT_0)?;
        let out = &port::price_from_draft(draft)?;
        wire::write_value(w, out, &S_PRICE_FROM_DRAFT_OUT)?;
        Ok(())
    })
}

static S_PROXY_ADDRESS_ERROR_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Str16);

/// `proxyAddressError` over the binary wire.
#[wasm_bindgen(js_name = "proxyAddressErrorWire")]
pub fn proxy_address_error_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let proxy: String = r.string16()?;
        let out = &port::proxy_address_error(proxy)?;
        wire::write_value(w, out, &S_PROXY_ADDRESS_ERROR_OUT)?;
        Ok(())
    })
}

static S_VALIDATE_PROVIDER_DRAFT_0: wire::Schema = wire::Schema::Dyn;
static S_VALIDATE_PROVIDER_DRAFT_1: wire::Schema = wire::Schema::Dyn;
static S_VALIDATE_PROVIDER_DRAFT_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Str16);

/// `validateProviderDraft` over the binary wire.
#[wasm_bindgen(js_name = "validateProviderDraftWire")]
pub fn validate_provider_draft_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let draft: Value = wire::read_value(r, &S_VALIDATE_PROVIDER_DRAFT_0)?;
        let editing: Value = wire::read_value(r, &S_VALIDATE_PROVIDER_DRAFT_1)?;
        let out = &port::validate_provider_draft(draft, editing)?;
        wire::write_value(w, out, &S_VALIDATE_PROVIDER_DRAFT_OUT)?;
        Ok(())
    })
}

static S_USAGE_RANGE_START_1: wire::Schema = wire::Schema::Dyn;
static S_USAGE_RANGE_START_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Num);

/// `usageRangeStart` over the binary wire.
#[wasm_bindgen(js_name = "usageRangeStartWire")]
pub fn usage_range_start_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let range: String = r.string16()?;
        let now: Value = wire::read_value(r, &S_USAGE_RANGE_START_1)?;
        let out = &port::usage_range_start(range, now)?;
        wire::write_value(w, out, &S_USAGE_RANGE_START_OUT)?;
        Ok(())
    })
}

static S_RELAY_FILTER_FROM_FOCUS_0: wire::Schema = wire::Schema::Dyn;
static S_RELAY_FILTER_FROM_FOCUS_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("label", wire::Schema::Str16),
    ("unknown", wire::Schema::OptJs(&wire::Schema::Bool)),
]));

/// `relayFilterFromFocus` over the binary wire.
#[wasm_bindgen(js_name = "relayFilterFromFocusWire")]
pub fn relay_filter_from_focus_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let focus: Value = wire::read_value(r, &S_RELAY_FILTER_FROM_FOCUS_0)?;
        let out = &port::relay_filter_from_focus(focus)?;
        wire::write_value(w, out, &S_RELAY_FILTER_FROM_FOCUS_OUT)?;
        Ok(())
    })
}

static S_RELAY_FILTER_ARGS_0: wire::Schema = wire::Schema::Dyn;
static S_RELAY_FILTER_ARGS_OUT: wire::Schema = wire::Schema::Obj(&[
    ("relayId", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("relayUnknown", wire::Schema::Bool),
]);

/// `relayFilterArgs` over the binary wire.
#[wasm_bindgen(js_name = "relayFilterArgsWire")]
pub fn relay_filter_args_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let filter: Value = wire::read_value(r, &S_RELAY_FILTER_ARGS_0)?;
        let out = &port::relay_filter_args(filter)?;
        wire::write_value(w, out, &S_RELAY_FILTER_ARGS_OUT)?;
        Ok(())
    })
}

static S_FORMAT_REQUEST_COST_0: wire::Schema = wire::Schema::Dyn;

/// `formatRequestCost` over the binary wire.
#[wasm_bindgen(js_name = "formatRequestCostWire")]
pub fn format_request_cost_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let cost: Value = wire::read_value(r, &S_FORMAT_REQUEST_COST_0)?;
        let out = &port::format_request_cost(cost)?;
        w.string16(out);
        Ok(())
    })
}

static S_CACHE_HIT_RATE_0: wire::Schema = wire::Schema::Dyn;
static S_CACHE_HIT_RATE_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Num);

/// `cacheHitRate` over the binary wire.
#[wasm_bindgen(js_name = "cacheHitRateWire")]
pub fn cache_hit_rate_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let counts: Value = wire::read_value(r, &S_CACHE_HIT_RATE_0)?;
        let out = &port::cache_hit_rate(counts)?;
        wire::write_value(w, out, &S_CACHE_HIT_RATE_OUT)?;
        Ok(())
    })
}

static S_AVERAGE_DURATION_0: wire::Schema = wire::Schema::Dyn;
static S_AVERAGE_DURATION_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Num);

/// `averageDuration` over the binary wire.
#[wasm_bindgen(js_name = "averageDurationWire")]
pub fn average_duration_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let counts: Value = wire::read_value(r, &S_AVERAGE_DURATION_0)?;
        let out = &port::average_duration(counts)?;
        wire::write_value(w, out, &S_AVERAGE_DURATION_OUT)?;
        Ok(())
    })
}

static S_COST_TEXT_0: wire::Schema = wire::Schema::Dyn;

/// `costText` over the binary wire.
#[wasm_bindgen(js_name = "costTextWire")]
pub fn cost_text_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let costs: Value = wire::read_value(r, &S_COST_TEXT_0)?;
        let out = &port::cost_text(costs)?;
        w.string16(out);
        Ok(())
    })
}

static S_USAGE_DAY_STATS_0: wire::Schema = wire::Schema::Dyn;
static S_USAGE_DAY_STATS_OUT: wire::Schema = wire::Schema::Obj(&[
    ("activeDays", wire::Schema::Num),
    (
        "peak",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("calls", wire::Schema::Num),
            ("errors", wire::Schema::Num),
            ("inputTokens", wire::Schema::Num),
            ("outputTokens", wire::Schema::Num),
            ("cachedTokens", wire::Schema::Num),
            ("reasoningTokens", wire::Schema::Num),
            ("durationMs", wire::Schema::OptJs(&wire::Schema::Num)),
            ("timedCalls", wire::Schema::OptJs(&wire::Schema::Num)),
            ("images", wire::Schema::OptJs(&wire::Schema::Num)),
            ("date", wire::Schema::Str16),
        ])),
    ),
    ("dailyAverage", wire::Schema::Num),
]);

/// `usageDayStats` over the binary wire.
#[wasm_bindgen(js_name = "usageDayStatsWire")]
pub fn usage_day_stats_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let days: Value = wire::read_value(r, &S_USAGE_DAY_STATS_0)?;
        let out = &port::usage_day_stats(days)?;
        wire::write_value(w, out, &S_USAGE_DAY_STATS_OUT)?;
        Ok(())
    })
}

static S_AXIS_LABEL_INDEXES_1: wire::Schema = wire::Schema::Dyn;

/// `axisLabelIndexes` over the binary wire.
#[wasm_bindgen(js_name = "axisLabelIndexesWire")]
pub fn axis_label_indexes_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let length: f64 = r.f64()?;
        let count: Value = wire::read_value(r, &S_AXIS_LABEL_INDEXES_1)?;
        let out = &port::axis_label_indexes(length, count)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            w.f64(*x0);
        }
        Ok(())
    })
}

static S_SHORT_DATE_1: wire::Schema = wire::Schema::Dyn;

/// `shortDate` over the binary wire.
#[wasm_bindgen(js_name = "shortDateWire")]
pub fn short_date_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let date: String = r.string16()?;
        let english: Value = wire::read_value(r, &S_SHORT_DATE_1)?;
        let out = &port::short_date(date, english)?;
        w.string16(out);
        Ok(())
    })
}

static S_FILTER_SESSIONS_0: wire::Schema = wire::Schema::Dyn;
static S_FILTER_SESSIONS_1: wire::Schema = wire::Schema::Dyn;
static S_FILTER_SESSIONS_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("agent", wire::Schema::Str16),
    ("id", wire::Schema::Str16),
    ("title", wire::Schema::Str16),
    ("cwd", wire::Schema::Str16),
    ("updatedAtMs", wire::Schema::Num),
    ("surface", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("archived", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("resumeId", wire::Schema::OptJs(&wire::Schema::Str16)),
]));

/// `filterSessions` over the binary wire.
#[wasm_bindgen(js_name = "filterSessionsWire")]
pub fn filter_sessions_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let sessions: Value = wire::read_value(r, &S_FILTER_SESSIONS_0)?;
        let agent: Value = wire::read_value(r, &S_FILTER_SESSIONS_1)?;
        let query: String = r.string16()?;
        let out = &port::filter_sessions(sessions, agent, query)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_FILTER_SESSIONS_OUT.inner())?;
        }
        Ok(())
    })
}

static S_RELATIVE_TIME_OUT: wire::Schema = wire::Schema::Obj(&[
    ("text", wire::Schema::Str16),
    ("args", wire::Schema::List(&wire::Schema::Dyn)),
]);

/// `relativeTime` over the binary wire.
#[wasm_bindgen(js_name = "relativeTimeWire")]
pub fn relative_time_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let ms: f64 = r.f64()?;
        let now: f64 = r.f64()?;
        let out = &port::relative_time(ms, now)?;
        wire::write_value(w, out, &S_RELATIVE_TIME_OUT)?;
        Ok(())
    })
}

static S_IMPORT_CANDIDATES_0: wire::Schema = wire::Schema::Dyn;
static S_IMPORT_CANDIDATES_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("source", wire::Schema::Str16),
    ("id", wire::Schema::Str16),
    ("name", wire::Schema::Str16),
    ("imported", wire::Schema::Bool),
]));

/// `importCandidates` over the binary wire.
#[wasm_bindgen(js_name = "importCandidatesWire")]
pub fn import_candidates_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let payload: Value = wire::read_value(r, &S_IMPORT_CANDIDATES_0)?;
        let out = &port::import_candidates(payload)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_IMPORT_CANDIDATES_OUT.inner())?;
        }
        Ok(())
    })
}

static S_PENDING_IMPORT_COUNT_0: wire::Schema = wire::Schema::Dyn;

/// `pendingImportCount` over the binary wire.
#[wasm_bindgen(js_name = "pendingImportCountWire")]
pub fn pending_import_count_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let payload: Value = wire::read_value(r, &S_PENDING_IMPORT_COUNT_0)?;
        let out = &port::pending_import_count(payload)?;
        w.f64(*out);
        Ok(())
    })
}

static S_IS_FAVORITE_MODEL_0: wire::Schema = wire::Schema::Dyn;
static S_IS_FAVORITE_MODEL_2: wire::Schema = wire::Schema::Dyn;

/// `isFavoriteModel` over the binary wire.
#[wasm_bindgen(js_name = "isFavoriteModelWire")]
pub fn is_favorite_model_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let favorites: Value = wire::read_value(r, &S_IS_FAVORITE_MODEL_0)?;
        let model: String = r.string16()?;
        let payload: Value = wire::read_value(r, &S_IS_FAVORITE_MODEL_2)?;
        let out = &port::is_favorite_model(favorites, model, payload)?;
        w.bool(*out);
        Ok(())
    })
}

static S_TOGGLE_FAVORITE_MODEL_0: wire::Schema = wire::Schema::Dyn;
static S_TOGGLE_FAVORITE_MODEL_2: wire::Schema = wire::Schema::Dyn;
static S_TOGGLE_FAVORITE_MODEL_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Str16);

/// `toggleFavoriteModel` over the binary wire.
#[wasm_bindgen(js_name = "toggleFavoriteModelWire")]
pub fn toggle_favorite_model_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let favorites: Value = wire::read_value(r, &S_TOGGLE_FAVORITE_MODEL_0)?;
        let model: String = r.string16()?;
        let payload: Value = wire::read_value(r, &S_TOGGLE_FAVORITE_MODEL_2)?;
        let out = &port::toggle_favorite_model(favorites, model, payload)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_TOGGLE_FAVORITE_MODEL_OUT.inner())?;
        }
        Ok(())
    })
}

static S_ACTIVE_AUTO_GROUP_0: wire::Schema = wire::Schema::Dyn;
static S_ACTIVE_AUTO_GROUP_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("name", wire::Schema::Str16),
    ("disabled", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("mode", wire::Schema::Str16),
    ("sticky", wire::Schema::OptJs(&wire::Schema::Str16)),
    (
        "members",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("model", wire::Schema::Str16),
            ("weight", wire::Schema::Num),
        ])),
    ),
    ("hideMembers", wire::Schema::OptJs(&wire::Schema::Bool)),
    ("pick", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("imageInputModel", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("hidden", wire::Schema::Bool),
    ("customized", wire::Schema::Bool),
]));

/// `activeAutoGroup` over the binary wire.
#[wasm_bindgen(js_name = "activeAutoGroupWire")]
pub fn active_auto_group_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let payload: Value = wire::read_value(r, &S_ACTIVE_AUTO_GROUP_0)?;
        let model: String = r.string16()?;
        let out = &port::active_auto_group(payload, model)?;
        wire::write_value(w, out, &S_ACTIVE_AUTO_GROUP_OUT)?;
        Ok(())
    })
}

static S_MEMBER_TARGET_0: wire::Schema = wire::Schema::Dyn;
static S_MEMBER_TARGET_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Obj(&[
    (
        "entry",
        wire::Schema::Obj(&[
            ("id", wire::Schema::Str16),
            ("name", wire::Schema::Str16),
            ("baseUrl", wire::Schema::Str16),
            ("protocol", wire::Schema::Str16),
            ("official", wire::Schema::Bool),
            ("models", wire::Schema::List(&wire::Schema::Str16)),
            ("keyCount", wire::Schema::Num),
            ("enabled", wire::Schema::OptJs(&wire::Schema::Bool)),
            ("source", wire::Schema::OptJs(&wire::Schema::Str16)),
            (
                "price",
                wire::Schema::OptJs(&wire::Schema::Obj(&[
                    ("currency", wire::Schema::Str16),
                    ("input", wire::Schema::Num),
                    ("cachedInput", wire::Schema::Num),
                    ("output", wire::Schema::Num),
                ])),
            ),
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
                "imageModels",
                wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Str16)),
            ),
            ("modelMetadata", wire::Schema::OptJs(&wire::Schema::Str16)),
            (
                "modelWindows",
                wire::Schema::OptJs(&wire::Schema::Map16(&wire::Schema::Num)),
            ),
            ("contextWindow", wire::Schema::OptJs(&wire::Schema::Num)),
            (
                "autoCompact",
                wire::Schema::OptJs(&wire::Schema::Obj(&[
                    (
                        "models",
                        wire::Schema::OptJs(&wire::Schema::Map16(&wire::Schema::Dyn)),
                    ),
                    ("fallback", wire::Schema::OptJs(&wire::Schema::Num)),
                ])),
            ),
            ("proxy", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("keyRpm", wire::Schema::OptJs(&wire::Schema::Num)),
            ("keyConcurrency", wire::Schema::OptJs(&wire::Schema::Num)),
            (
                "stripImageGenerationTool",
                wire::Schema::OptJs(&wire::Schema::Bool),
            ),
            (
                "stripResponsesImageDetail",
                wire::Schema::OptJs(&wire::Schema::Bool),
            ),
            (
                "disableToolSearch",
                wire::Schema::OptJs(&wire::Schema::Bool),
            ),
            ("account", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("subscriptionPolicy", wire::Schema::Dyn),
            (
                "subscriptionBlockReason",
                wire::Schema::OptJs(&wire::Schema::Str16),
            ),
            ("plan", wire::Schema::OptJs(&wire::Schema::Str16)),
            (
                "upstreamNew",
                wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Str16)),
            ),
            (
                "upstreamGone",
                wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Str16)),
            ),
            ("lentToCodex", wire::Schema::OptJs(&wire::Schema::Bool)),
            (
                "quota",
                wire::Schema::OptJs(&wire::Schema::Obj(&[
                    (
                        "windows",
                        wire::Schema::List(&wire::Schema::Obj(&[
                            ("windowMinutes", wire::Schema::Dyn),
                            ("usedPercent", wire::Schema::Dyn),
                            ("resetsAtMs", wire::Schema::Dyn),
                            ("observedAtMs", wire::Schema::Dyn),
                        ])),
                    ),
                    ("updatedAtMs", wire::Schema::Num),
                    ("windowsComplete", wire::Schema::OptJs(&wire::Schema::Bool)),
                    ("officialAllowed", wire::Schema::OptJs(&wire::Schema::Bool)),
                    (
                        "officialLimitReached",
                        wire::Schema::OptJs(&wire::Schema::Bool),
                    ),
                    ("creditsAvailable", wire::Schema::OptJs(&wire::Schema::Bool)),
                ])),
            ),
            ("quotaApi", wire::Schema::OptJs(&wire::Schema::Bool)),
            (
                "keys",
                wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Obj(&[
                    ("id", wire::Schema::Str16),
                    ("masked", wire::Schema::Str16),
                ]))),
            ),
            ("balanceApi", wire::Schema::OptJs(&wire::Schema::Bool)),
            (
                "newapiAccess",
                wire::Schema::OptJs(&wire::Schema::Obj(&[
                    ("masked", wire::Schema::Str16),
                    ("userId", wire::Schema::Str16),
                ])),
            ),
            (
                "customHeaders",
                wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Obj(&[
                    ("key", wire::Schema::Str16),
                    ("value", wire::Schema::Str16),
                    ("masked", wire::Schema::OptJs(&wire::Schema::Str16)),
                ]))),
            ),
        ]),
    ),
    ("model", wire::Schema::Str16),
]));

/// `memberTarget` over the binary wire.
#[wasm_bindgen(js_name = "memberTargetWire")]
pub fn member_target_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let payload: Value = wire::read_value(r, &S_MEMBER_TARGET_0)?;
        let model: String = r.string16()?;
        let out = &port::member_target(payload, model)?;
        wire::write_value(w, out, &S_MEMBER_TARGET_OUT)?;
        Ok(())
    })
}
