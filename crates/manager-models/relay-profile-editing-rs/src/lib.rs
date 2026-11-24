//! JS bindings for `src/relay-profile-editing.ts`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;
pub mod utf16;

/// `withGeneratedRelayFiles`.
fn with_generated_relay_files_inner(profile: String) -> Result<String, String> {
    let profile = crate::js::from_json(&profile)?;
    let out = port::with_generated_relay_files(profile)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "withGeneratedRelayFiles")]
pub fn with_generated_relay_files(profile: String) -> Result<String, JsError> {
    with_generated_relay_files_inner(profile).map_err(|e| JsError::new(&e))
}

/// `buildRelayConfigToml`.
fn build_relay_config_toml_inner(profile: String, options: String) -> Result<String, String> {
    let profile = crate::js::from_json(&profile)?;
    let options = crate::js::from_json(&options)?;
    let out = port::build_relay_config_toml(profile, options)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "buildRelayConfigToml")]
pub fn build_relay_config_toml(profile: String, options: String) -> Result<String, JsError> {
    build_relay_config_toml_inner(profile, options).map_err(|e| JsError::new(&e))
}

/// `buildRelayAuthJson`.
fn build_relay_auth_json_inner(profile: String) -> Result<String, String> {
    let profile = crate::js::from_json(&profile)?;
    let out = port::build_relay_auth_json(profile)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "buildRelayAuthJson")]
pub fn build_relay_auth_json(profile: String) -> Result<String, JsError> {
    build_relay_auth_json_inner(profile).map_err(|e| JsError::new(&e))
}

/// `buildOfficialRelayAuthJson`.
fn build_official_relay_auth_json_inner(contents: String) -> Result<String, String> {
    let out = port::build_official_relay_auth_json(contents)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "buildOfficialRelayAuthJson")]
pub fn build_official_relay_auth_json(contents: String) -> Result<String, JsError> {
    build_official_relay_auth_json_inner(contents).map_err(|e| JsError::new(&e))
}

/// `deriveRelayProfileFromFiles`.
fn derive_relay_profile_from_files_inner(
    profile: String,
    edited_file: String,
) -> Result<String, String> {
    let profile = crate::js::from_json(&profile)?;
    let edited_file = crate::js::from_json(&edited_file)?;
    let out = port::derive_relay_profile_from_files(profile, edited_file)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "deriveRelayProfileFromFiles")]
pub fn derive_relay_profile_from_files(
    profile: String,
    edited_file: String,
) -> Result<String, JsError> {
    derive_relay_profile_from_files_inner(profile, edited_file).map_err(|e| JsError::new(&e))
}

/// `applyRelayProfilePatchToFiles`.
fn apply_relay_profile_patch_to_files_inner(
    profile: String,
    patch: String,
    options: String,
) -> Result<String, String> {
    let profile = crate::js::from_json(&profile)?;
    let patch = crate::js::from_json(&patch)?;
    let options = crate::js::from_json(&options)?;
    let out = port::apply_relay_profile_patch_to_files(profile, patch, options)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "applyRelayProfilePatchToFiles")]
pub fn apply_relay_profile_patch_to_files(
    profile: String,
    patch: String,
    options: String,
) -> Result<String, JsError> {
    apply_relay_profile_patch_to_files_inner(profile, patch, options).map_err(|e| JsError::new(&e))
}

/// `codexModelFromConfig`.
fn codex_model_from_config_inner(contents: String) -> Result<String, String> {
    let out = port::codex_model_from_config(contents)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "codexModelFromConfig")]
pub fn codex_model_from_config(contents: String) -> Result<String, JsError> {
    codex_model_from_config_inner(contents).map_err(|e| JsError::new(&e))
}

/// `parseModelSuffix`.
fn parse_model_suffix_inner(raw: String) -> Result<String, String> {
    let out = port::parse_model_suffix(raw)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "parseModelSuffix")]
pub fn parse_model_suffix(raw: String) -> Result<String, JsError> {
    parse_model_suffix_inner(raw).map_err(|e| JsError::new(&e))
}

/// `codexBaseUrlFromConfig`.
fn codex_base_url_from_config_inner(contents: String) -> Result<String, String> {
    let out = port::codex_base_url_from_config(contents)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "codexBaseUrlFromConfig")]
pub fn codex_base_url_from_config(contents: String) -> Result<String, JsError> {
    codex_base_url_from_config_inner(contents).map_err(|e| JsError::new(&e))
}

/// `codexExperimentalBearerTokenFromConfig`.
fn codex_experimental_bearer_token_from_config_inner(contents: String) -> Result<String, String> {
    let out = port::codex_experimental_bearer_token_from_config(contents)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "codexExperimentalBearerTokenFromConfig")]
pub fn codex_experimental_bearer_token_from_config(contents: String) -> Result<String, JsError> {
    codex_experimental_bearer_token_from_config_inner(contents).map_err(|e| JsError::new(&e))
}

/// `codexProviderStringFromConfig`.
fn codex_provider_string_from_config_inner(
    contents: String,
    key: String,
) -> Result<String, String> {
    let out = port::codex_provider_string_from_config(contents, key)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "codexProviderStringFromConfig")]
pub fn codex_provider_string_from_config(contents: String, key: String) -> Result<String, JsError> {
    codex_provider_string_from_config_inner(contents, key).map_err(|e| JsError::new(&e))
}

/// `codexApiKeyFromAuth`.
fn codex_api_key_from_auth_inner(contents: String) -> Result<String, String> {
    let out = port::codex_api_key_from_auth(contents)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "codexApiKeyFromAuth")]
pub fn codex_api_key_from_auth(contents: String) -> Result<String, JsError> {
    codex_api_key_from_auth_inner(contents).map_err(|e| JsError::new(&e))
}

/// `codexTopLevelIntFromConfig`.
fn codex_top_level_int_from_config_inner(contents: String, key: String) -> Result<String, String> {
    let out = port::codex_top_level_int_from_config(contents, key)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "codexTopLevelIntFromConfig")]
pub fn codex_top_level_int_from_config(contents: String, key: String) -> Result<String, JsError> {
    codex_top_level_int_from_config_inner(contents, key).map_err(|e| JsError::new(&e))
}

/// `rootTomlStringValue`.
fn root_toml_string_value_inner(contents: String, key: String) -> Result<String, String> {
    let out = port::root_toml_string_value(contents, key)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "rootTomlStringValue")]
pub fn root_toml_string_value(contents: String, key: String) -> Result<String, JsError> {
    root_toml_string_value_inner(contents, key).map_err(|e| JsError::new(&e))
}

/// `tomlStructureMask`.
fn toml_structure_mask_inner(lines: Vec<String>) -> Result<String, String> {
    let out = port::toml_structure_mask(lines)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "tomlStructureMask")]
pub fn toml_structure_mask(lines: Vec<String>) -> Result<String, JsError> {
    toml_structure_mask_inner(lines).map_err(|e| JsError::new(&e))
}

/// `tomlSectionName`.
fn toml_section_name_inner(line: String) -> Result<String, String> {
    let out = port::toml_section_name(line)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "tomlSectionName")]
pub fn toml_section_name(line: String) -> Result<String, JsError> {
    toml_section_name_inner(line).map_err(|e| JsError::new(&e))
}

/// `tomlQuotedValue`.
fn toml_quoted_value_inner(text: String, start: f64) -> Result<String, String> {
    let out = port::toml_quoted_value(text, start)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "tomlQuotedValue")]
pub fn toml_quoted_value(text: String, start: f64) -> Result<String, JsError> {
    toml_quoted_value_inner(text, start).map_err(|e| JsError::new(&e))
}

/// `tomlDottedKeyPath`.
fn toml_dotted_key_path_inner(text: String) -> Result<String, String> {
    let out = port::toml_dotted_key_path(text)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "tomlDottedKeyPath")]
pub fn toml_dotted_key_path(text: String) -> Result<String, JsError> {
    toml_dotted_key_path_inner(text).map_err(|e| JsError::new(&e))
}

/// `tomlKeyPathName`.
fn toml_key_path_name_inner(path: Vec<String>) -> Result<String, String> {
    let out = port::toml_key_path_name(path)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "tomlKeyPathName")]
pub fn toml_key_path_name(path: Vec<String>) -> Result<String, JsError> {
    toml_key_path_name_inner(path).map_err(|e| JsError::new(&e))
}

/// `codexProviderSectionName`.
fn codex_provider_section_name_inner(provider: String) -> Result<String, String> {
    let out = port::codex_provider_section_name(provider)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "codexProviderSectionName")]
pub fn codex_provider_section_name(provider: String) -> Result<String, JsError> {
    codex_provider_section_name_inner(provider).map_err(|e| JsError::new(&e))
}

/// `tomlKeyAssignment`.
fn toml_key_assignment_inner(line: String) -> Result<String, String> {
    let out = port::toml_key_assignment(line)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "tomlKeyAssignment")]
pub fn toml_key_assignment(line: String) -> Result<String, JsError> {
    toml_key_assignment_inner(line).map_err(|e| JsError::new(&e))
}

/// `tomlInlineComment`.
fn toml_inline_comment_inner(line: String, start: f64) -> Result<String, String> {
    let out = port::toml_inline_comment(line, start)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "tomlInlineComment")]
pub fn toml_inline_comment(line: String, start: f64) -> Result<String, JsError> {
    toml_inline_comment_inner(line, start).map_err(|e| JsError::new(&e))
}

/// `tomlStringAssignmentValue`.
fn toml_string_assignment_value_inner(line: String, key: String) -> Result<String, String> {
    let out = port::toml_string_assignment_value(line, key)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "tomlStringAssignmentValue")]
pub fn toml_string_assignment_value(line: String, key: String) -> Result<String, JsError> {
    toml_string_assignment_value_inner(line, key).map_err(|e| JsError::new(&e))
}

/// `setAuthOpenAiApiKey`.
fn set_auth_open_ai_api_key_inner(contents: String, api_key: String) -> Result<String, String> {
    let out = port::set_auth_open_ai_api_key(contents, api_key)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "setAuthOpenAiApiKey")]
pub fn set_auth_open_ai_api_key(contents: String, api_key: String) -> Result<String, JsError> {
    set_auth_open_ai_api_key_inner(contents, api_key).map_err(|e| JsError::new(&e))
}

/// `setRootTomlStringKey`.
fn set_root_toml_string_key_inner(
    contents: String,
    key: String,
    value: String,
) -> Result<String, String> {
    let out = port::set_root_toml_string_key(contents, key, value)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "setRootTomlStringKey")]
pub fn set_root_toml_string_key(
    contents: String,
    key: String,
    value: String,
) -> Result<String, JsError> {
    set_root_toml_string_key_inner(contents, key, value).map_err(|e| JsError::new(&e))
}

/// `setManagedOpenAiBaseUrl`.
fn set_managed_open_ai_base_url_inner(contents: String, enabled: bool) -> Result<String, String> {
    let out = port::set_managed_open_ai_base_url(contents, enabled)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "setManagedOpenAiBaseUrl")]
pub fn set_managed_open_ai_base_url(contents: String, enabled: bool) -> Result<String, JsError> {
    set_managed_open_ai_base_url_inner(contents, enabled).map_err(|e| JsError::new(&e))
}

/// `setRootTomlIntKey`.
fn set_root_toml_int_key_inner(
    contents: String,
    key: String,
    value: String,
) -> Result<String, String> {
    let out = port::set_root_toml_int_key(contents, key, value)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "setRootTomlIntKey")]
pub fn set_root_toml_int_key(
    contents: String,
    key: String,
    value: String,
) -> Result<String, JsError> {
    set_root_toml_int_key_inner(contents, key, value).map_err(|e| JsError::new(&e))
}

/// `setRootTomlLine`.
fn set_root_toml_line_inner(
    contents: String,
    key: String,
    line_text: String,
) -> Result<String, String> {
    let out = port::set_root_toml_line(contents, key, line_text)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "setRootTomlLine")]
pub fn set_root_toml_line(
    contents: String,
    key: String,
    line_text: String,
) -> Result<String, JsError> {
    set_root_toml_line_inner(contents, key, line_text).map_err(|e| JsError::new(&e))
}

/// `codexRequiresOpenAiAuthFromConfig`.
fn codex_requires_open_ai_auth_from_config_inner(contents: String) -> Result<bool, String> {
    let out = port::codex_requires_open_ai_auth_from_config(contents)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "codexRequiresOpenAiAuthFromConfig")]
pub fn codex_requires_open_ai_auth_from_config(contents: String) -> Result<bool, JsError> {
    codex_requires_open_ai_auth_from_config_inner(contents).map_err(|e| JsError::new(&e))
}

/// `setCodexProviderStringKey`.
fn set_codex_provider_string_key_inner(
    contents: String,
    key: String,
    value: String,
    options: String,
) -> Result<String, String> {
    let options = crate::js::from_json(&options)?;
    let out = port::set_codex_provider_string_key(contents, key, value, options)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "setCodexProviderStringKey")]
pub fn set_codex_provider_string_key(
    contents: String,
    key: String,
    value: String,
    options: String,
) -> Result<String, JsError> {
    set_codex_provider_string_key_inner(contents, key, value, options).map_err(|e| JsError::new(&e))
}

/// `setCodexExperimentalBearerToken`.
fn set_codex_experimental_bearer_token_inner(
    contents: String,
    api_key: String,
    options: String,
) -> Result<String, String> {
    let options = crate::js::from_json(&options)?;
    let out = port::set_codex_experimental_bearer_token(contents, api_key, options)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "setCodexExperimentalBearerToken")]
pub fn set_codex_experimental_bearer_token(
    contents: String,
    api_key: String,
    options: String,
) -> Result<String, JsError> {
    set_codex_experimental_bearer_token_inner(contents, api_key, options)
        .map_err(|e| JsError::new(&e))
}

/// `removeCodexExperimentalBearerToken`.
fn remove_codex_experimental_bearer_token_inner(contents: String) -> Result<String, String> {
    let out = port::remove_codex_experimental_bearer_token(contents)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "removeCodexExperimentalBearerToken")]
pub fn remove_codex_experimental_bearer_token(contents: String) -> Result<String, JsError> {
    remove_codex_experimental_bearer_token_inner(contents).map_err(|e| JsError::new(&e))
}

/// `ensureCodexProviderDefaults`.
fn ensure_codex_provider_defaults_inner(
    contents: String,
    provider: String,
    options: String,
) -> Result<String, String> {
    let options = crate::js::from_json(&options)?;
    let out = port::ensure_codex_provider_defaults(contents, provider, options)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "ensureCodexProviderDefaults")]
pub fn ensure_codex_provider_defaults(
    contents: String,
    provider: String,
    options: String,
) -> Result<String, JsError> {
    ensure_codex_provider_defaults_inner(contents, provider, options).map_err(|e| JsError::new(&e))
}

/// `setTomlSectionBoolKey`.
fn set_toml_section_bool_key_inner(
    contents: String,
    section_name: String,
    key: String,
    value: bool,
) -> Result<String, String> {
    let out = port::set_toml_section_bool_key(contents, section_name, key, value)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "setTomlSectionBoolKey")]
pub fn set_toml_section_bool_key(
    contents: String,
    section_name: String,
    key: String,
    value: bool,
) -> Result<String, JsError> {
    set_toml_section_bool_key_inner(contents, section_name, key, value)
        .map_err(|e| JsError::new(&e))
}

/// `setTomlSectionStringKey`.
fn set_toml_section_string_key_inner(
    contents: String,
    section_name: String,
    key: String,
    value: String,
) -> Result<String, String> {
    let out = port::set_toml_section_string_key(contents, section_name, key, value)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "setTomlSectionStringKey")]
pub fn set_toml_section_string_key(
    contents: String,
    section_name: String,
    key: String,
    value: String,
) -> Result<String, JsError> {
    set_toml_section_string_key_inner(contents, section_name, key, value)
        .map_err(|e| JsError::new(&e))
}

/// `setTomlSectionRawKey`.
fn set_toml_section_raw_key_inner(
    contents: String,
    section_name: String,
    key: String,
    value: String,
) -> Result<String, String> {
    let out = port::set_toml_section_raw_key(contents, section_name, key, value)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "setTomlSectionRawKey")]
pub fn set_toml_section_raw_key(
    contents: String,
    section_name: String,
    key: String,
    value: String,
) -> Result<String, JsError> {
    set_toml_section_raw_key_inner(contents, section_name, key, value).map_err(|e| JsError::new(&e))
}

/// `removeTomlSectionKey`.
fn remove_toml_section_key_inner(
    contents: String,
    section_name: String,
    key: String,
) -> Result<String, String> {
    let out = port::remove_toml_section_key(contents, section_name, key)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "removeTomlSectionKey")]
pub fn remove_toml_section_key(
    contents: String,
    section_name: String,
    key: String,
) -> Result<String, JsError> {
    remove_toml_section_key_inner(contents, section_name, key).map_err(|e| JsError::new(&e))
}

/// `relayProfileSwitchValidation`.
fn relay_profile_switch_validation_inner(
    profile: String,
    settings: String,
) -> Result<String, String> {
    let profile = crate::js::from_json(&profile)?;
    let settings = crate::js::from_json(&settings)?;
    let out = port::relay_profile_switch_validation(profile, settings)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "relayProfileSwitchValidation")]
pub fn relay_profile_switch_validation(
    profile: String,
    settings: String,
) -> Result<String, JsError> {
    relay_profile_switch_validation_inner(profile, settings).map_err(|e| JsError::new(&e))
}

/// `relayModelRoutesValidation`.
fn relay_model_routes_validation_inner(
    profile: String,
    settings: String,
) -> Result<String, String> {
    let profile = crate::js::from_json(&profile)?;
    let settings = crate::js::from_json(&settings)?;
    let out = port::relay_model_routes_validation(profile, settings)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "relayModelRoutesValidation")]
pub fn relay_model_routes_validation(profile: String, settings: String) -> Result<String, JsError> {
    relay_model_routes_validation_inner(profile, settings).map_err(|e| JsError::new(&e))
}

/// `relayModelRoutesSettingsValidation`.
fn relay_model_routes_settings_validation_inner(settings: String) -> Result<String, String> {
    let settings = crate::js::from_json(&settings)?;
    let out = port::relay_model_routes_settings_validation(settings)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "relayModelRoutesSettingsValidation")]
pub fn relay_model_routes_settings_validation(settings: String) -> Result<String, JsError> {
    relay_model_routes_settings_validation_inner(settings).map_err(|e| JsError::new(&e))
}

/// `relaySettingsValidation`.
fn relay_settings_validation_inner(settings: String) -> Result<String, String> {
    let settings = crate::js::from_json(&settings)?;
    let out = port::relay_settings_validation(settings)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "relaySettingsValidation")]
pub fn relay_settings_validation(settings: String) -> Result<String, JsError> {
    relay_settings_validation_inner(settings).map_err(|e| JsError::new(&e))
}

/// `relaySessionProviderValidation`.
fn relay_session_provider_validation_inner(profile: String) -> Result<String, String> {
    let profile = crate::js::from_json(&profile)?;
    let out = port::relay_session_provider_validation(profile)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "relaySessionProviderValidation")]
pub fn relay_session_provider_validation(profile: String) -> Result<String, JsError> {
    relay_session_provider_validation_inner(profile).map_err(|e| JsError::new(&e))
}

/// `relayModelRouteIssueMessage`.
fn relay_model_route_issue_message_inner(issue: String) -> Result<String, String> {
    let issue = crate::js::from_json(&issue)?;
    let out = port::relay_model_route_issue_message(issue)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "relayModelRouteIssueMessage")]
pub fn relay_model_route_issue_message(issue: String) -> Result<String, JsError> {
    relay_model_route_issue_message_inner(issue).map_err(|e| JsError::new(&e))
}

/// `relaySettingsWithDraft`.
fn relay_settings_with_draft_inner(
    settings: String,
    profile_id: String,
    draft: String,
    is_new: bool,
) -> Result<String, String> {
    let settings = crate::js::from_json(&settings)?;
    let draft = crate::js::from_json(&draft)?;
    let out = port::relay_settings_with_draft(settings, profile_id, draft, is_new)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "relaySettingsWithDraft")]
pub fn relay_settings_with_draft(
    settings: String,
    profile_id: String,
    draft: String,
    is_new: bool,
) -> Result<String, JsError> {
    relay_settings_with_draft_inner(settings, profile_id, draft, is_new)
        .map_err(|e| JsError::new(&e))
}

/// `relayProfileUsesLiveFiles`.
fn relay_profile_uses_live_files_inner(profile: String) -> Result<bool, String> {
    let profile = crate::js::from_json(&profile)?;
    let out = port::relay_profile_uses_live_files(profile)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "relayProfileUsesLiveFiles")]
pub fn relay_profile_uses_live_files(profile: String) -> Result<bool, JsError> {
    relay_profile_uses_live_files_inner(profile).map_err(|e| JsError::new(&e))
}

/// `authJsonHasOpenAiApiKey`.
fn auth_json_has_open_ai_api_key_inner(contents: String) -> Result<bool, String> {
    let out = port::auth_json_has_open_ai_api_key(contents)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "authJsonHasOpenAiApiKey")]
pub fn auth_json_has_open_ai_api_key(contents: String) -> Result<bool, JsError> {
    auth_json_has_open_ai_api_key_inner(contents).map_err(|e| JsError::new(&e))
}

/// `tomlString`.
fn toml_string_inner(value: String) -> Result<String, String> {
    let out = port::toml_string(value)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "tomlString")]
pub fn toml_string(value: String) -> Result<String, JsError> {
    toml_string_inner(value).map_err(|e| JsError::new(&e))
}

/// `syncLegacyRelayFields`.
fn sync_legacy_relay_fields_inner(settings: String) -> Result<String, String> {
    let settings = crate::js::from_json(&settings)?;
    let out = port::sync_legacy_relay_fields(settings)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "syncLegacyRelayFields")]
pub fn sync_legacy_relay_fields(settings: String) -> Result<String, JsError> {
    sync_legacy_relay_fields_inner(settings).map_err(|e| JsError::new(&e))
}

/// `normalizeAggregateProfilesFromRelayProfiles`.
fn normalize_aggregate_profiles_from_relay_profiles_inner(
    profiles: String,
) -> Result<String, String> {
    let profiles = crate::js::from_json(&profiles)?;
    let out = port::normalize_aggregate_profiles_from_relay_profiles(profiles)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "normalizeAggregateProfilesFromRelayProfiles")]
pub fn normalize_aggregate_profiles_from_relay_profiles(
    profiles: String,
) -> Result<String, JsError> {
    normalize_aggregate_profiles_from_relay_profiles_inner(profiles).map_err(|e| JsError::new(&e))
}

/// `updateRelayProfile`.
fn update_relay_profile_inner(
    settings: String,
    id: String,
    patch: String,
) -> Result<String, String> {
    let settings = crate::js::from_json(&settings)?;
    let patch = crate::js::from_json(&patch)?;
    let out = port::update_relay_profile(settings, id, patch)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "updateRelayProfile")]
pub fn update_relay_profile(
    settings: String,
    id: String,
    patch: String,
) -> Result<String, JsError> {
    update_relay_profile_inner(settings, id, patch).map_err(|e| JsError::new(&e))
}

/// `addRelayProfile`.
fn add_relay_profile_inner(settings: String, profile: String) -> Result<String, String> {
    let settings = crate::js::from_json(&settings)?;
    let profile = crate::js::from_json(&profile)?;
    let out = port::add_relay_profile(settings, profile)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "addRelayProfile")]
pub fn add_relay_profile(settings: String, profile: String) -> Result<String, JsError> {
    add_relay_profile_inner(settings, profile).map_err(|e| JsError::new(&e))
}

/// `reorderRelayProfiles`.
fn reorder_relay_profiles_inner(
    settings: String,
    source_id: String,
    target_id: String,
) -> Result<String, String> {
    let settings = crate::js::from_json(&settings)?;
    let out = port::reorder_relay_profiles(settings, source_id, target_id)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "reorderRelayProfiles")]
pub fn reorder_relay_profiles(
    settings: String,
    source_id: String,
    target_id: String,
) -> Result<String, JsError> {
    reorder_relay_profiles_inner(settings, source_id, target_id).map_err(|e| JsError::new(&e))
}

/// `removeRelayProfile`.
fn remove_relay_profile_inner(settings: String, id: String) -> Result<String, String> {
    let settings = crate::js::from_json(&settings)?;
    let out = port::remove_relay_profile(settings, id)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "removeRelayProfile")]
pub fn remove_relay_profile(settings: String, id: String) -> Result<String, JsError> {
    remove_relay_profile_inner(settings, id).map_err(|e| JsError::new(&e))
}

/// `relayProfileDeleteIssue`.
fn relay_profile_delete_issue_inner(settings: String, id: String) -> Result<String, String> {
    let settings = crate::js::from_json(&settings)?;
    let out = port::relay_profile_delete_issue(settings, id)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "relayProfileDeleteIssue")]
pub fn relay_profile_delete_issue(settings: String, id: String) -> Result<String, JsError> {
    relay_profile_delete_issue_inner(settings, id).map_err(|e| JsError::new(&e))
}

/// `isAggregateRelayProfile`.
fn is_aggregate_relay_profile_inner(profile: String) -> Result<bool, String> {
    let profile = crate::js::from_json(&profile)?;
    let out = port::is_aggregate_relay_profile(profile)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "isAggregateRelayProfile")]
pub fn is_aggregate_relay_profile(profile: String) -> Result<bool, JsError> {
    is_aggregate_relay_profile_inner(profile).map_err(|e| JsError::new(&e))
}

/// `relayProviderView`.
fn relay_provider_view_inner(profile: String) -> Result<String, String> {
    let profile = crate::js::from_json(&profile)?;
    let out = port::relay_provider_view(profile)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "relayProviderView")]
pub fn relay_provider_view(profile: String) -> Result<String, JsError> {
    relay_provider_view_inner(profile).map_err(|e| JsError::new(&e))
}

/// `normalizeAggregateRelayProfile`.
fn normalize_aggregate_relay_profile_inner(
    profile: String,
    settings: String,
) -> Result<String, String> {
    let profile = crate::js::from_json(&profile)?;
    let settings = crate::js::from_json(&settings)?;
    let out = port::normalize_aggregate_relay_profile(profile, settings)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "normalizeAggregateRelayProfile")]
pub fn normalize_aggregate_relay_profile(
    profile: String,
    settings: String,
) -> Result<String, JsError> {
    normalize_aggregate_relay_profile_inner(profile, settings).map_err(|e| JsError::new(&e))
}

/// `aggregateProfileForMode`.
fn aggregate_profile_for_mode_inner(
    profile: String,
    settings: String,
    mode: String,
) -> Result<String, String> {
    let profile = crate::js::from_json(&profile)?;
    let settings = crate::js::from_json(&settings)?;
    let mode = crate::js::from_json(&mode)?;
    let out = port::aggregate_profile_for_mode(profile, settings, mode)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "aggregateProfileForMode")]
pub fn aggregate_profile_for_mode(
    profile: String,
    settings: String,
    mode: String,
) -> Result<String, JsError> {
    aggregate_profile_for_mode_inner(profile, settings, mode).map_err(|e| JsError::new(&e))
}

/// `normalizeAggregateConfig`.
fn normalize_aggregate_config_inner(
    aggregate: String,
    candidates: String,
    existing_profile_ids: String,
) -> Result<String, String> {
    let aggregate = crate::js::from_json(&aggregate)?;
    let candidates = crate::js::from_json(&candidates)?;
    let existing_profile_ids = crate::js::from_json(&existing_profile_ids)?;
    let out = port::normalize_aggregate_config(aggregate, candidates, existing_profile_ids)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "normalizeAggregateConfig")]
pub fn normalize_aggregate_config(
    aggregate: String,
    candidates: String,
    existing_profile_ids: String,
) -> Result<String, JsError> {
    normalize_aggregate_config_inner(aggregate, candidates, existing_profile_ids)
        .map_err(|e| JsError::new(&e))
}

/// `clampAggregateWeight`.
fn clamp_aggregate_weight_inner(value: f64) -> Result<f64, String> {
    let out = port::clamp_aggregate_weight(value)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "clampAggregateWeight")]
pub fn clamp_aggregate_weight(value: f64) -> Result<f64, JsError> {
    clamp_aggregate_weight_inner(value).map_err(|e| JsError::new(&e))
}

/// `aggregateStrategyLabel`.
fn aggregate_strategy_label_inner(strategy: String) -> Result<String, String> {
    let strategy = crate::js::from_json(&strategy)?;
    let out = port::aggregate_strategy_label(strategy)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "aggregateStrategyLabel")]
pub fn aggregate_strategy_label(strategy: String) -> Result<String, JsError> {
    aggregate_strategy_label_inner(strategy).map_err(|e| JsError::new(&e))
}

/// `aggregateStrategyHelp`.
fn aggregate_strategy_help_inner(strategy: String) -> Result<String, String> {
    let strategy = crate::js::from_json(&strategy)?;
    let out = port::aggregate_strategy_help(strategy)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "aggregateStrategyHelp")]
pub fn aggregate_strategy_help(strategy: String) -> Result<String, JsError> {
    aggregate_strategy_help_inner(strategy).map_err(|e| JsError::new(&e))
}

/// `aggregateRelayProfileValidation`.
fn aggregate_relay_profile_validation_inner(profile: String) -> Result<String, String> {
    let profile = crate::js::from_json(&profile)?;
    let out = port::aggregate_relay_profile_validation(profile)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "aggregateRelayProfileValidation")]
pub fn aggregate_relay_profile_validation(profile: String) -> Result<String, JsError> {
    aggregate_relay_profile_validation_inner(profile).map_err(|e| JsError::new(&e))
}

/// `ensureTrailingNewline`.
fn ensure_trailing_newline_inner(value: String) -> Result<String, String> {
    let out = port::ensure_trailing_newline(value)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "ensureTrailingNewline")]
pub fn ensure_trailing_newline(value: String) -> Result<String, JsError> {
    ensure_trailing_newline_inner(value).map_err(|e| JsError::new(&e))
}

/// `removeRootTomlKey`.
fn remove_root_toml_key_inner(contents: String, key: String) -> Result<String, String> {
    let out = port::remove_root_toml_key(contents, key)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "removeRootTomlKey")]
pub fn remove_root_toml_key(contents: String, key: String) -> Result<String, JsError> {
    remove_root_toml_key_inner(contents, key).map_err(|e| JsError::new(&e))
}

/// `joinTomlSections`.
fn join_toml_sections_inner(sections: Vec<String>) -> Result<String, String> {
    let out = port::join_toml_sections(sections)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "joinTomlSections")]
pub fn join_toml_sections(sections: Vec<String>) -> Result<String, JsError> {
    join_toml_sections_inner(sections).map_err(|e| JsError::new(&e))
}

/// `splitTomlRootAndTables`.
fn split_toml_root_and_tables_inner(section: String) -> Result<String, String> {
    let out = port::split_toml_root_and_tables(section)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "splitTomlRootAndTables")]
pub fn split_toml_root_and_tables(section: String) -> Result<String, JsError> {
    split_toml_root_and_tables_inner(section).map_err(|e| JsError::new(&e))
}

/// `normalizeRelaySessionProvider`.
fn normalize_relay_session_provider_inner(value: String) -> Result<String, String> {
    let value = crate::js::from_json(&value)?;
    let out = port::normalize_relay_session_provider(value)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "normalizeRelaySessionProvider")]
pub fn normalize_relay_session_provider(value: String) -> Result<String, JsError> {
    normalize_relay_session_provider_inner(value).map_err(|e| JsError::new(&e))
}

/// `relaySessionProviderFromConfig`.
fn relay_session_provider_from_config_inner(contents: String) -> Result<String, String> {
    let out = port::relay_session_provider_from_config(contents)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "relaySessionProviderFromConfig")]
pub fn relay_session_provider_from_config(contents: String) -> Result<String, JsError> {
    relay_session_provider_from_config_inner(contents).map_err(|e| JsError::new(&e))
}

/// `relaySessionProvider`.
fn relay_session_provider_inner(profile: String) -> Result<String, String> {
    let profile = crate::js::from_json(&profile)?;
    let out = port::relay_session_provider(profile)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "relaySessionProvider")]
pub fn relay_session_provider(profile: String) -> Result<String, JsError> {
    relay_session_provider_inner(profile).map_err(|e| JsError::new(&e))
}

/// `activeRelayProfile`.
fn active_relay_profile_inner(settings: String) -> Result<String, String> {
    let settings = crate::js::from_json(&settings)?;
    let out = port::active_relay_profile(settings)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "activeRelayProfile")]
pub fn active_relay_profile(settings: String) -> Result<String, JsError> {
    active_relay_profile_inner(settings).map_err(|e| JsError::new(&e))
}

/// `emptyContextSelection`.
fn empty_context_selection_inner() -> Result<String, String> {
    let out = port::empty_context_selection()?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "emptyContextSelection")]
pub fn empty_context_selection() -> Result<String, JsError> {
    empty_context_selection_inner().map_err(|e| JsError::new(&e))
}

/// `normalizeContextSelection`.
fn normalize_context_selection_inner(
    selection: String,
    fallback: String,
) -> Result<String, String> {
    let selection = crate::js::from_json(&selection)?;
    let fallback = crate::js::from_json(&fallback)?;
    let out = port::normalize_context_selection(selection, fallback)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "normalizeContextSelection")]
pub fn normalize_context_selection(selection: String, fallback: String) -> Result<String, JsError> {
    normalize_context_selection_inner(selection, fallback).map_err(|e| JsError::new(&e))
}

/// `normalizeRelayMode`.
fn normalize_relay_mode_inner(mode: String) -> Result<String, String> {
    let mode = crate::js::from_json(&mode)?;
    let out = port::normalize_relay_mode(mode)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "normalizeRelayMode")]
pub fn normalize_relay_mode(mode: String) -> Result<String, JsError> {
    normalize_relay_mode_inner(mode).map_err(|e| JsError::new(&e))
}

/// `normalizeRelayProtocol`.
fn normalize_relay_protocol_inner(value: String) -> Result<String, String> {
    let value = crate::js::from_json(&value)?;
    let out = port::normalize_relay_protocol(value)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "normalizeRelayProtocol")]
pub fn normalize_relay_protocol(value: String) -> Result<String, JsError> {
    normalize_relay_protocol_inner(value).map_err(|e| JsError::new(&e))
}

/// `normalizeAnthropicAuthHeader`.
fn normalize_anthropic_auth_header_inner(value: String) -> Result<String, String> {
    let value = crate::js::from_json(&value)?;
    let out = port::normalize_anthropic_auth_header(value)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "normalizeAnthropicAuthHeader")]
pub fn normalize_anthropic_auth_header(value: String) -> Result<String, JsError> {
    normalize_anthropic_auth_header_inner(value).map_err(|e| JsError::new(&e))
}

/// `normalizeAnthropicThinkingMode`.
fn normalize_anthropic_thinking_mode_inner(value: String) -> Result<String, String> {
    let value = crate::js::from_json(&value)?;
    let out = port::normalize_anthropic_thinking_mode(value)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "normalizeAnthropicThinkingMode")]
pub fn normalize_anthropic_thinking_mode(value: String) -> Result<String, JsError> {
    normalize_anthropic_thinking_mode_inner(value).map_err(|e| JsError::new(&e))
}

/// `normalizeAnthropicMaxTokens`.
fn normalize_anthropic_max_tokens_inner(value: String) -> Result<f64, String> {
    let value = crate::js::from_json(&value)?;
    let out = port::normalize_anthropic_max_tokens(value)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "normalizeAnthropicMaxTokens")]
pub fn normalize_anthropic_max_tokens(value: String) -> Result<f64, JsError> {
    normalize_anthropic_max_tokens_inner(value).map_err(|e| JsError::new(&e))
}

/// `hydrateAggregateRelayProfile`.
fn hydrate_aggregate_relay_profile_inner(
    profile: String,
    aggregate: String,
) -> Result<String, String> {
    let profile = crate::js::from_json(&profile)?;
    let aggregate = crate::js::from_json(&aggregate)?;
    let out = port::hydrate_aggregate_relay_profile(profile, aggregate)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "hydrateAggregateRelayProfile")]
pub fn hydrate_aggregate_relay_profile(
    profile: String,
    aggregate: String,
) -> Result<String, JsError> {
    hydrate_aggregate_relay_profile_inner(profile, aggregate).map_err(|e| JsError::new(&e))
}

/// `normalizeRelayProfile`.
fn normalize_relay_profile_inner(
    profile: String,
    default_context_selection: String,
) -> Result<String, String> {
    let profile = crate::js::from_json(&profile)?;
    let default_context_selection = crate::js::from_json(&default_context_selection)?;
    let out = port::normalize_relay_profile(profile, default_context_selection)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "normalizeRelayProfile")]
pub fn normalize_relay_profile(
    profile: String,
    default_context_selection: String,
) -> Result<String, JsError> {
    normalize_relay_profile_inner(profile, default_context_selection).map_err(|e| JsError::new(&e))
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

static S_WITH_GENERATED_RELAY_FILES_0: wire::Schema = wire::Schema::Dyn;
static S_WITH_GENERATED_RELAY_FILES_OUT: wire::Schema = wire::Schema::Dyn;

/// `withGeneratedRelayFiles` over the binary wire.
#[wasm_bindgen(js_name = "withGeneratedRelayFilesWire")]
pub fn with_generated_relay_files_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let profile: Value = wire::read_value(r, &S_WITH_GENERATED_RELAY_FILES_0)?;
        let out = &port::with_generated_relay_files(profile)?;
        wire::write_value(w, out, &S_WITH_GENERATED_RELAY_FILES_OUT)?;
        Ok(())
    })
}

static S_BUILD_RELAY_CONFIG_TOML_0: wire::Schema = wire::Schema::Dyn;
static S_BUILD_RELAY_CONFIG_TOML_1: wire::Schema = wire::Schema::Dyn;

/// `buildRelayConfigToml` over the binary wire.
#[wasm_bindgen(js_name = "buildRelayConfigTomlWire")]
pub fn build_relay_config_toml_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let profile: Value = wire::read_value(r, &S_BUILD_RELAY_CONFIG_TOML_0)?;
        let options: Value = wire::read_value(r, &S_BUILD_RELAY_CONFIG_TOML_1)?;
        let out = &port::build_relay_config_toml(profile, options)?;
        w.string16(out);
        Ok(())
    })
}

static S_BUILD_RELAY_AUTH_JSON_0: wire::Schema = wire::Schema::Dyn;

/// `buildRelayAuthJson` over the binary wire.
#[wasm_bindgen(js_name = "buildRelayAuthJsonWire")]
pub fn build_relay_auth_json_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let profile: Value = wire::read_value(r, &S_BUILD_RELAY_AUTH_JSON_0)?;
        let out = &port::build_relay_auth_json(profile)?;
        w.string16(out);
        Ok(())
    })
}

static S_DERIVE_RELAY_PROFILE_FROM_FILES_0: wire::Schema = wire::Schema::Dyn;
static S_DERIVE_RELAY_PROFILE_FROM_FILES_1: wire::Schema = wire::Schema::Dyn;
static S_DERIVE_RELAY_PROFILE_FROM_FILES_OUT: wire::Schema = wire::Schema::Dyn;

/// `deriveRelayProfileFromFiles` over the binary wire.
#[wasm_bindgen(js_name = "deriveRelayProfileFromFilesWire")]
pub fn derive_relay_profile_from_files_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let profile: Value = wire::read_value(r, &S_DERIVE_RELAY_PROFILE_FROM_FILES_0)?;
        let edited_file: Value = wire::read_value(r, &S_DERIVE_RELAY_PROFILE_FROM_FILES_1)?;
        let out = &port::derive_relay_profile_from_files(profile, edited_file)?;
        wire::write_value(w, out, &S_DERIVE_RELAY_PROFILE_FROM_FILES_OUT)?;
        Ok(())
    })
}

static S_APPLY_RELAY_PROFILE_PATCH_TO_FILES_0: wire::Schema = wire::Schema::Dyn;
static S_APPLY_RELAY_PROFILE_PATCH_TO_FILES_1: wire::Schema = wire::Schema::Dyn;
static S_APPLY_RELAY_PROFILE_PATCH_TO_FILES_2: wire::Schema = wire::Schema::Dyn;
static S_APPLY_RELAY_PROFILE_PATCH_TO_FILES_OUT: wire::Schema = wire::Schema::Dyn;

/// `applyRelayProfilePatchToFiles` over the binary wire.
#[wasm_bindgen(js_name = "applyRelayProfilePatchToFilesWire")]
pub fn apply_relay_profile_patch_to_files_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let profile: Value = wire::read_value(r, &S_APPLY_RELAY_PROFILE_PATCH_TO_FILES_0)?;
        let patch: Value = wire::read_value(r, &S_APPLY_RELAY_PROFILE_PATCH_TO_FILES_1)?;
        let options: Value = wire::read_value(r, &S_APPLY_RELAY_PROFILE_PATCH_TO_FILES_2)?;
        let out = &port::apply_relay_profile_patch_to_files(profile, patch, options)?;
        wire::write_value(w, out, &S_APPLY_RELAY_PROFILE_PATCH_TO_FILES_OUT)?;
        Ok(())
    })
}

static S_PARSE_MODEL_SUFFIX_OUT: wire::Schema = wire::Schema::Obj(&[
    ("slug", wire::Schema::Str16),
    ("window", wire::Schema::OptJs(&wire::Schema::Num)),
]);

/// `parseModelSuffix` over the binary wire.
#[wasm_bindgen(js_name = "parseModelSuffixWire")]
pub fn parse_model_suffix_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let raw: String = r.string16()?;
        let out = &port::parse_model_suffix(raw)?;
        wire::write_value(w, out, &S_PARSE_MODEL_SUFFIX_OUT)?;
        Ok(())
    })
}

/// `tomlStructureMask` over the binary wire.
#[wasm_bindgen(js_name = "tomlStructureMaskWire")]
pub fn toml_structure_mask_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let lines: Vec<String> = r.list(|r| r.string16())?;
        let out = &port::toml_structure_mask(lines)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            w.bool(*x0);
        }
        Ok(())
    })
}

static S_TOML_SECTION_NAME_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Str16);

/// `tomlSectionName` over the binary wire.
#[wasm_bindgen(js_name = "tomlSectionNameWire")]
pub fn toml_section_name_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let line: String = r.string16()?;
        let out = &port::toml_section_name(line)?;
        wire::write_value(w, out, &S_TOML_SECTION_NAME_OUT)?;
        Ok(())
    })
}

static S_TOML_QUOTED_VALUE_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Obj(&[
    ("value", wire::Schema::Str16),
    ("end", wire::Schema::Num),
]));

/// `tomlQuotedValue` over the binary wire.
#[wasm_bindgen(js_name = "tomlQuotedValueWire")]
pub fn toml_quoted_value_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let text: String = r.string16()?;
        let start: f64 = r.f64()?;
        let out = &port::toml_quoted_value(text, start)?;
        wire::write_value(w, out, &S_TOML_QUOTED_VALUE_OUT)?;
        Ok(())
    })
}

static S_TOML_DOTTED_KEY_PATH_OUT: wire::Schema =
    wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Str16));

/// `tomlDottedKeyPath` over the binary wire.
#[wasm_bindgen(js_name = "tomlDottedKeyPathWire")]
pub fn toml_dotted_key_path_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let text: String = r.string16()?;
        let out = &port::toml_dotted_key_path(text)?;
        wire::write_value(w, out, &S_TOML_DOTTED_KEY_PATH_OUT)?;
        Ok(())
    })
}

static S_TOML_KEY_ASSIGNMENT_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Obj(&[
    ("path", wire::Schema::List(&wire::Schema::Str16)),
    ("valueStart", wire::Schema::Num),
]));

/// `tomlKeyAssignment` over the binary wire.
#[wasm_bindgen(js_name = "tomlKeyAssignmentWire")]
pub fn toml_key_assignment_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let line: String = r.string16()?;
        let out = &port::toml_key_assignment(line)?;
        wire::write_value(w, out, &S_TOML_KEY_ASSIGNMENT_OUT)?;
        Ok(())
    })
}

static S_TOML_STRING_ASSIGNMENT_VALUE_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Str16);

/// `tomlStringAssignmentValue` over the binary wire.
#[wasm_bindgen(js_name = "tomlStringAssignmentValueWire")]
pub fn toml_string_assignment_value_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let line: String = r.string16()?;
        let key: String = r.string16()?;
        let out = &port::toml_string_assignment_value(line, key)?;
        wire::write_value(w, out, &S_TOML_STRING_ASSIGNMENT_VALUE_OUT)?;
        Ok(())
    })
}

static S_SET_CODEX_PROVIDER_STRING_KEY_3: wire::Schema = wire::Schema::Dyn;

/// `setCodexProviderStringKey` over the binary wire.
#[wasm_bindgen(js_name = "setCodexProviderStringKeyWire")]
pub fn set_codex_provider_string_key_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let contents: String = r.string16()?;
        let key: String = r.string16()?;
        let value: String = r.string16()?;
        let options: Value = wire::read_value(r, &S_SET_CODEX_PROVIDER_STRING_KEY_3)?;
        let out = &port::set_codex_provider_string_key(contents, key, value, options)?;
        w.string16(out);
        Ok(())
    })
}

static S_SET_CODEX_EXPERIMENTAL_BEARER_TOKEN_2: wire::Schema = wire::Schema::Dyn;

/// `setCodexExperimentalBearerToken` over the binary wire.
#[wasm_bindgen(js_name = "setCodexExperimentalBearerTokenWire")]
pub fn set_codex_experimental_bearer_token_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let contents: String = r.string16()?;
        let api_key: String = r.string16()?;
        let options: Value = wire::read_value(r, &S_SET_CODEX_EXPERIMENTAL_BEARER_TOKEN_2)?;
        let out = &port::set_codex_experimental_bearer_token(contents, api_key, options)?;
        w.string16(out);
        Ok(())
    })
}

static S_ENSURE_CODEX_PROVIDER_DEFAULTS_2: wire::Schema = wire::Schema::Dyn;

/// `ensureCodexProviderDefaults` over the binary wire.
#[wasm_bindgen(js_name = "ensureCodexProviderDefaultsWire")]
pub fn ensure_codex_provider_defaults_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let contents: String = r.string16()?;
        let provider: String = r.string16()?;
        let options: Value = wire::read_value(r, &S_ENSURE_CODEX_PROVIDER_DEFAULTS_2)?;
        let out = &port::ensure_codex_provider_defaults(contents, provider, options)?;
        w.string16(out);
        Ok(())
    })
}

static S_RELAY_PROFILE_SWITCH_VALIDATION_0: wire::Schema = wire::Schema::Dyn;
static S_RELAY_PROFILE_SWITCH_VALIDATION_1: wire::Schema = wire::Schema::Dyn;
static S_RELAY_PROFILE_SWITCH_VALIDATION_OUT: wire::Schema =
    wire::Schema::OptJs(&wire::Schema::Str16);

/// `relayProfileSwitchValidation` over the binary wire.
#[wasm_bindgen(js_name = "relayProfileSwitchValidationWire")]
pub fn relay_profile_switch_validation_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let profile: Value = wire::read_value(r, &S_RELAY_PROFILE_SWITCH_VALIDATION_0)?;
        let settings: Value = wire::read_value(r, &S_RELAY_PROFILE_SWITCH_VALIDATION_1)?;
        let out = &port::relay_profile_switch_validation(profile, settings)?;
        wire::write_value(w, out, &S_RELAY_PROFILE_SWITCH_VALIDATION_OUT)?;
        Ok(())
    })
}

static S_RELAY_MODEL_ROUTES_VALIDATION_0: wire::Schema = wire::Schema::Dyn;
static S_RELAY_MODEL_ROUTES_VALIDATION_1: wire::Schema = wire::Schema::Dyn;
static S_RELAY_MODEL_ROUTES_VALIDATION_OUT: wire::Schema =
    wire::Schema::OptJs(&wire::Schema::Str16);

/// `relayModelRoutesValidation` over the binary wire.
#[wasm_bindgen(js_name = "relayModelRoutesValidationWire")]
pub fn relay_model_routes_validation_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let profile: Value = wire::read_value(r, &S_RELAY_MODEL_ROUTES_VALIDATION_0)?;
        let settings: Value = wire::read_value(r, &S_RELAY_MODEL_ROUTES_VALIDATION_1)?;
        let out = &port::relay_model_routes_validation(profile, settings)?;
        wire::write_value(w, out, &S_RELAY_MODEL_ROUTES_VALIDATION_OUT)?;
        Ok(())
    })
}

static S_RELAY_MODEL_ROUTES_SETTINGS_VALIDATION_0: wire::Schema = wire::Schema::Dyn;
static S_RELAY_MODEL_ROUTES_SETTINGS_VALIDATION_OUT: wire::Schema =
    wire::Schema::OptJs(&wire::Schema::Str16);

/// `relayModelRoutesSettingsValidation` over the binary wire.
#[wasm_bindgen(js_name = "relayModelRoutesSettingsValidationWire")]
pub fn relay_model_routes_settings_validation_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let settings: Value = wire::read_value(r, &S_RELAY_MODEL_ROUTES_SETTINGS_VALIDATION_0)?;
        let out = &port::relay_model_routes_settings_validation(settings)?;
        wire::write_value(w, out, &S_RELAY_MODEL_ROUTES_SETTINGS_VALIDATION_OUT)?;
        Ok(())
    })
}

static S_RELAY_SETTINGS_VALIDATION_0: wire::Schema = wire::Schema::Dyn;
static S_RELAY_SETTINGS_VALIDATION_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Str16);

/// `relaySettingsValidation` over the binary wire.
#[wasm_bindgen(js_name = "relaySettingsValidationWire")]
pub fn relay_settings_validation_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let settings: Value = wire::read_value(r, &S_RELAY_SETTINGS_VALIDATION_0)?;
        let out = &port::relay_settings_validation(settings)?;
        wire::write_value(w, out, &S_RELAY_SETTINGS_VALIDATION_OUT)?;
        Ok(())
    })
}

static S_RELAY_SESSION_PROVIDER_VALIDATION_0: wire::Schema = wire::Schema::Dyn;
static S_RELAY_SESSION_PROVIDER_VALIDATION_OUT: wire::Schema =
    wire::Schema::OptJs(&wire::Schema::Str16);

/// `relaySessionProviderValidation` over the binary wire.
#[wasm_bindgen(js_name = "relaySessionProviderValidationWire")]
pub fn relay_session_provider_validation_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let profile: Value = wire::read_value(r, &S_RELAY_SESSION_PROVIDER_VALIDATION_0)?;
        let out = &port::relay_session_provider_validation(profile)?;
        wire::write_value(w, out, &S_RELAY_SESSION_PROVIDER_VALIDATION_OUT)?;
        Ok(())
    })
}

static S_RELAY_MODEL_ROUTE_ISSUE_MESSAGE_0: wire::Schema = wire::Schema::Dyn;
static S_RELAY_MODEL_ROUTE_ISSUE_MESSAGE_OUT: wire::Schema =
    wire::Schema::OptJs(&wire::Schema::Str16);

/// `relayModelRouteIssueMessage` over the binary wire.
#[wasm_bindgen(js_name = "relayModelRouteIssueMessageWire")]
pub fn relay_model_route_issue_message_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let issue: Value = wire::read_value(r, &S_RELAY_MODEL_ROUTE_ISSUE_MESSAGE_0)?;
        let out = &port::relay_model_route_issue_message(issue)?;
        wire::write_value(w, out, &S_RELAY_MODEL_ROUTE_ISSUE_MESSAGE_OUT)?;
        Ok(())
    })
}

static S_RELAY_SETTINGS_WITH_DRAFT_0: wire::Schema = wire::Schema::Dyn;
static S_RELAY_SETTINGS_WITH_DRAFT_2: wire::Schema = wire::Schema::Dyn;
static S_RELAY_SETTINGS_WITH_DRAFT_OUT: wire::Schema = wire::Schema::Dyn;

/// `relaySettingsWithDraft` over the binary wire.
#[wasm_bindgen(js_name = "relaySettingsWithDraftWire")]
pub fn relay_settings_with_draft_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let settings: Value = wire::read_value(r, &S_RELAY_SETTINGS_WITH_DRAFT_0)?;
        let profile_id: String = r.string16()?;
        let draft: Value = wire::read_value(r, &S_RELAY_SETTINGS_WITH_DRAFT_2)?;
        let is_new: bool = r.bool()?;
        let out = &port::relay_settings_with_draft(settings, profile_id, draft, is_new)?;
        wire::write_value(w, out, &S_RELAY_SETTINGS_WITH_DRAFT_OUT)?;
        Ok(())
    })
}

static S_RELAY_PROFILE_USES_LIVE_FILES_0: wire::Schema = wire::Schema::Dyn;

/// `relayProfileUsesLiveFiles` over the binary wire.
#[wasm_bindgen(js_name = "relayProfileUsesLiveFilesWire")]
pub fn relay_profile_uses_live_files_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let profile: Value = wire::read_value(r, &S_RELAY_PROFILE_USES_LIVE_FILES_0)?;
        let out = &port::relay_profile_uses_live_files(profile)?;
        w.bool(*out);
        Ok(())
    })
}

static S_SYNC_LEGACY_RELAY_FIELDS_0: wire::Schema = wire::Schema::Dyn;
static S_SYNC_LEGACY_RELAY_FIELDS_OUT: wire::Schema = wire::Schema::Dyn;

/// `syncLegacyRelayFields` over the binary wire.
#[wasm_bindgen(js_name = "syncLegacyRelayFieldsWire")]
pub fn sync_legacy_relay_fields_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let settings: Value = wire::read_value(r, &S_SYNC_LEGACY_RELAY_FIELDS_0)?;
        let out = &port::sync_legacy_relay_fields(settings)?;
        wire::write_value(w, out, &S_SYNC_LEGACY_RELAY_FIELDS_OUT)?;
        Ok(())
    })
}

static S_NORMALIZE_AGGREGATE_PROFILES_FROM_RELAY_PROFILES_0: wire::Schema = wire::Schema::Dyn;
static S_NORMALIZE_AGGREGATE_PROFILES_FROM_RELAY_PROFILES_OUT: wire::Schema =
    wire::Schema::List(&wire::Schema::Dyn);

/// `normalizeAggregateProfilesFromRelayProfiles` over the binary wire.
#[wasm_bindgen(js_name = "normalizeAggregateProfilesFromRelayProfilesWire")]
pub fn normalize_aggregate_profiles_from_relay_profiles_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let profiles: Value =
            wire::read_value(r, &S_NORMALIZE_AGGREGATE_PROFILES_FROM_RELAY_PROFILES_0)?;
        let out = &port::normalize_aggregate_profiles_from_relay_profiles(profiles)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(
                w,
                x0,
                S_NORMALIZE_AGGREGATE_PROFILES_FROM_RELAY_PROFILES_OUT.inner(),
            )?;
        }
        Ok(())
    })
}

static S_UPDATE_RELAY_PROFILE_0: wire::Schema = wire::Schema::Dyn;
static S_UPDATE_RELAY_PROFILE_2: wire::Schema = wire::Schema::Dyn;
static S_UPDATE_RELAY_PROFILE_OUT: wire::Schema = wire::Schema::Dyn;

/// `updateRelayProfile` over the binary wire.
#[wasm_bindgen(js_name = "updateRelayProfileWire")]
pub fn update_relay_profile_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let settings: Value = wire::read_value(r, &S_UPDATE_RELAY_PROFILE_0)?;
        let id: String = r.string16()?;
        let patch: Value = wire::read_value(r, &S_UPDATE_RELAY_PROFILE_2)?;
        let out = &port::update_relay_profile(settings, id, patch)?;
        wire::write_value(w, out, &S_UPDATE_RELAY_PROFILE_OUT)?;
        Ok(())
    })
}

static S_ADD_RELAY_PROFILE_0: wire::Schema = wire::Schema::Dyn;
static S_ADD_RELAY_PROFILE_1: wire::Schema = wire::Schema::Dyn;
static S_ADD_RELAY_PROFILE_OUT: wire::Schema = wire::Schema::Dyn;

/// `addRelayProfile` over the binary wire.
#[wasm_bindgen(js_name = "addRelayProfileWire")]
pub fn add_relay_profile_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let settings: Value = wire::read_value(r, &S_ADD_RELAY_PROFILE_0)?;
        let profile: Value = wire::read_value(r, &S_ADD_RELAY_PROFILE_1)?;
        let out = &port::add_relay_profile(settings, profile)?;
        wire::write_value(w, out, &S_ADD_RELAY_PROFILE_OUT)?;
        Ok(())
    })
}

static S_REORDER_RELAY_PROFILES_0: wire::Schema = wire::Schema::Dyn;
static S_REORDER_RELAY_PROFILES_OUT: wire::Schema = wire::Schema::Dyn;

/// `reorderRelayProfiles` over the binary wire.
#[wasm_bindgen(js_name = "reorderRelayProfilesWire")]
pub fn reorder_relay_profiles_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let settings: Value = wire::read_value(r, &S_REORDER_RELAY_PROFILES_0)?;
        let source_id: String = r.string16()?;
        let target_id: String = r.string16()?;
        let out = &port::reorder_relay_profiles(settings, source_id, target_id)?;
        wire::write_value(w, out, &S_REORDER_RELAY_PROFILES_OUT)?;
        Ok(())
    })
}

static S_REMOVE_RELAY_PROFILE_0: wire::Schema = wire::Schema::Dyn;
static S_REMOVE_RELAY_PROFILE_OUT: wire::Schema = wire::Schema::Dyn;

/// `removeRelayProfile` over the binary wire.
#[wasm_bindgen(js_name = "removeRelayProfileWire")]
pub fn remove_relay_profile_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let settings: Value = wire::read_value(r, &S_REMOVE_RELAY_PROFILE_0)?;
        let id: String = r.string16()?;
        let out = &port::remove_relay_profile(settings, id)?;
        wire::write_value(w, out, &S_REMOVE_RELAY_PROFILE_OUT)?;
        Ok(())
    })
}

static S_RELAY_PROFILE_DELETE_ISSUE_0: wire::Schema = wire::Schema::Dyn;
static S_RELAY_PROFILE_DELETE_ISSUE_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Str16);

/// `relayProfileDeleteIssue` over the binary wire.
#[wasm_bindgen(js_name = "relayProfileDeleteIssueWire")]
pub fn relay_profile_delete_issue_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let settings: Value = wire::read_value(r, &S_RELAY_PROFILE_DELETE_ISSUE_0)?;
        let id: String = r.string16()?;
        let out = &port::relay_profile_delete_issue(settings, id)?;
        wire::write_value(w, out, &S_RELAY_PROFILE_DELETE_ISSUE_OUT)?;
        Ok(())
    })
}

static S_IS_AGGREGATE_RELAY_PROFILE_0: wire::Schema = wire::Schema::Dyn;

/// `isAggregateRelayProfile` over the binary wire.
#[wasm_bindgen(js_name = "isAggregateRelayProfileWire")]
pub fn is_aggregate_relay_profile_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let profile: Value = wire::read_value(r, &S_IS_AGGREGATE_RELAY_PROFILE_0)?;
        let out = &port::is_aggregate_relay_profile(profile)?;
        w.bool(*out);
        Ok(())
    })
}

static S_RELAY_PROVIDER_VIEW_0: wire::Schema = wire::Schema::Dyn;

/// `relayProviderView` over the binary wire.
#[wasm_bindgen(js_name = "relayProviderViewWire")]
pub fn relay_provider_view_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let profile: Value = wire::read_value(r, &S_RELAY_PROVIDER_VIEW_0)?;
        let out = &port::relay_provider_view(profile)?;
        w.string16(out);
        Ok(())
    })
}

static S_NORMALIZE_AGGREGATE_RELAY_PROFILE_0: wire::Schema = wire::Schema::Dyn;
static S_NORMALIZE_AGGREGATE_RELAY_PROFILE_1: wire::Schema = wire::Schema::Dyn;
static S_NORMALIZE_AGGREGATE_RELAY_PROFILE_OUT: wire::Schema = wire::Schema::Dyn;

/// `normalizeAggregateRelayProfile` over the binary wire.
#[wasm_bindgen(js_name = "normalizeAggregateRelayProfileWire")]
pub fn normalize_aggregate_relay_profile_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let profile: Value = wire::read_value(r, &S_NORMALIZE_AGGREGATE_RELAY_PROFILE_0)?;
        let settings: Value = wire::read_value(r, &S_NORMALIZE_AGGREGATE_RELAY_PROFILE_1)?;
        let out = &port::normalize_aggregate_relay_profile(profile, settings)?;
        wire::write_value(w, out, &S_NORMALIZE_AGGREGATE_RELAY_PROFILE_OUT)?;
        Ok(())
    })
}

static S_AGGREGATE_PROFILE_FOR_MODE_0: wire::Schema = wire::Schema::Dyn;
static S_AGGREGATE_PROFILE_FOR_MODE_1: wire::Schema = wire::Schema::Dyn;
static S_AGGREGATE_PROFILE_FOR_MODE_2: wire::Schema = wire::Schema::Dyn;
static S_AGGREGATE_PROFILE_FOR_MODE_OUT: wire::Schema = wire::Schema::Dyn;

/// `aggregateProfileForMode` over the binary wire.
#[wasm_bindgen(js_name = "aggregateProfileForModeWire")]
pub fn aggregate_profile_for_mode_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let profile: Value = wire::read_value(r, &S_AGGREGATE_PROFILE_FOR_MODE_0)?;
        let settings: Value = wire::read_value(r, &S_AGGREGATE_PROFILE_FOR_MODE_1)?;
        let mode: Value = wire::read_value(r, &S_AGGREGATE_PROFILE_FOR_MODE_2)?;
        let out = &port::aggregate_profile_for_mode(profile, settings, mode)?;
        wire::write_value(w, out, &S_AGGREGATE_PROFILE_FOR_MODE_OUT)?;
        Ok(())
    })
}

static S_NORMALIZE_AGGREGATE_CONFIG_0: wire::Schema = wire::Schema::Dyn;
static S_NORMALIZE_AGGREGATE_CONFIG_1: wire::Schema = wire::Schema::Dyn;
static S_NORMALIZE_AGGREGATE_CONFIG_2: wire::Schema = wire::Schema::Dyn;
static S_NORMALIZE_AGGREGATE_CONFIG_OUT: wire::Schema = wire::Schema::Dyn;

/// `normalizeAggregateConfig` over the binary wire.
#[wasm_bindgen(js_name = "normalizeAggregateConfigWire")]
pub fn normalize_aggregate_config_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let aggregate: Value = wire::read_value(r, &S_NORMALIZE_AGGREGATE_CONFIG_0)?;
        let candidates: Value = wire::read_value(r, &S_NORMALIZE_AGGREGATE_CONFIG_1)?;
        let existing_profile_ids: Value = wire::read_value(r, &S_NORMALIZE_AGGREGATE_CONFIG_2)?;
        let out = &port::normalize_aggregate_config(aggregate, candidates, existing_profile_ids)?;
        wire::write_value(w, out, &S_NORMALIZE_AGGREGATE_CONFIG_OUT)?;
        Ok(())
    })
}

static S_AGGREGATE_STRATEGY_LABEL_0: wire::Schema = wire::Schema::Dyn;

/// `aggregateStrategyLabel` over the binary wire.
#[wasm_bindgen(js_name = "aggregateStrategyLabelWire")]
pub fn aggregate_strategy_label_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let strategy: Value = wire::read_value(r, &S_AGGREGATE_STRATEGY_LABEL_0)?;
        let out = &port::aggregate_strategy_label(strategy)?;
        w.string16(out);
        Ok(())
    })
}

static S_AGGREGATE_STRATEGY_HELP_0: wire::Schema = wire::Schema::Dyn;

/// `aggregateStrategyHelp` over the binary wire.
#[wasm_bindgen(js_name = "aggregateStrategyHelpWire")]
pub fn aggregate_strategy_help_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let strategy: Value = wire::read_value(r, &S_AGGREGATE_STRATEGY_HELP_0)?;
        let out = &port::aggregate_strategy_help(strategy)?;
        w.string16(out);
        Ok(())
    })
}

static S_AGGREGATE_RELAY_PROFILE_VALIDATION_0: wire::Schema = wire::Schema::Dyn;
static S_AGGREGATE_RELAY_PROFILE_VALIDATION_OUT: wire::Schema =
    wire::Schema::OptJs(&wire::Schema::Str16);

/// `aggregateRelayProfileValidation` over the binary wire.
#[wasm_bindgen(js_name = "aggregateRelayProfileValidationWire")]
pub fn aggregate_relay_profile_validation_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let profile: Value = wire::read_value(r, &S_AGGREGATE_RELAY_PROFILE_VALIDATION_0)?;
        let out = &port::aggregate_relay_profile_validation(profile)?;
        wire::write_value(w, out, &S_AGGREGATE_RELAY_PROFILE_VALIDATION_OUT)?;
        Ok(())
    })
}

static S_SPLIT_TOML_ROOT_AND_TABLES_OUT: wire::Schema = wire::Schema::Obj(&[
    ("root", wire::Schema::Str16),
    ("tables", wire::Schema::Str16),
]);

/// `splitTomlRootAndTables` over the binary wire.
#[wasm_bindgen(js_name = "splitTomlRootAndTablesWire")]
pub fn split_toml_root_and_tables_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let section: String = r.string16()?;
        let out = &port::split_toml_root_and_tables(section)?;
        wire::write_value(w, out, &S_SPLIT_TOML_ROOT_AND_TABLES_OUT)?;
        Ok(())
    })
}

static S_NORMALIZE_RELAY_SESSION_PROVIDER_0: wire::Schema = wire::Schema::Dyn;
static S_NORMALIZE_RELAY_SESSION_PROVIDER_OUT: wire::Schema = wire::Schema::Dyn;

/// `normalizeRelaySessionProvider` over the binary wire.
#[wasm_bindgen(js_name = "normalizeRelaySessionProviderWire")]
pub fn normalize_relay_session_provider_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let value: Value = wire::read_value(r, &S_NORMALIZE_RELAY_SESSION_PROVIDER_0)?;
        let out = &port::normalize_relay_session_provider(value)?;
        wire::write_value(w, out, &S_NORMALIZE_RELAY_SESSION_PROVIDER_OUT)?;
        Ok(())
    })
}

static S_RELAY_SESSION_PROVIDER_FROM_CONFIG_OUT: wire::Schema = wire::Schema::Dyn;

/// `relaySessionProviderFromConfig` over the binary wire.
#[wasm_bindgen(js_name = "relaySessionProviderFromConfigWire")]
pub fn relay_session_provider_from_config_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let contents: String = r.string16()?;
        let out = &port::relay_session_provider_from_config(contents)?;
        wire::write_value(w, out, &S_RELAY_SESSION_PROVIDER_FROM_CONFIG_OUT)?;
        Ok(())
    })
}

static S_RELAY_SESSION_PROVIDER_0: wire::Schema = wire::Schema::Dyn;
static S_RELAY_SESSION_PROVIDER_OUT: wire::Schema = wire::Schema::Dyn;

/// `relaySessionProvider` over the binary wire.
#[wasm_bindgen(js_name = "relaySessionProviderWire")]
pub fn relay_session_provider_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let profile: Value = wire::read_value(r, &S_RELAY_SESSION_PROVIDER_0)?;
        let out = &port::relay_session_provider(profile)?;
        wire::write_value(w, out, &S_RELAY_SESSION_PROVIDER_OUT)?;
        Ok(())
    })
}

static S_ACTIVE_RELAY_PROFILE_0: wire::Schema = wire::Schema::Dyn;
static S_ACTIVE_RELAY_PROFILE_OUT: wire::Schema = wire::Schema::Dyn;

/// `activeRelayProfile` over the binary wire.
#[wasm_bindgen(js_name = "activeRelayProfileWire")]
pub fn active_relay_profile_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let settings: Value = wire::read_value(r, &S_ACTIVE_RELAY_PROFILE_0)?;
        let out = &port::active_relay_profile(settings)?;
        wire::write_value(w, out, &S_ACTIVE_RELAY_PROFILE_OUT)?;
        Ok(())
    })
}

static S_EMPTY_CONTEXT_SELECTION_OUT: wire::Schema = wire::Schema::Dyn;

/// `emptyContextSelection` over the binary wire.
#[wasm_bindgen(js_name = "emptyContextSelectionWire")]
pub fn empty_context_selection_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |_r, w| {
        let out = &port::empty_context_selection()?;
        wire::write_value(w, out, &S_EMPTY_CONTEXT_SELECTION_OUT)?;
        Ok(())
    })
}

static S_NORMALIZE_CONTEXT_SELECTION_0: wire::Schema = wire::Schema::Dyn;
static S_NORMALIZE_CONTEXT_SELECTION_1: wire::Schema = wire::Schema::Dyn;
static S_NORMALIZE_CONTEXT_SELECTION_OUT: wire::Schema = wire::Schema::Dyn;

/// `normalizeContextSelection` over the binary wire.
#[wasm_bindgen(js_name = "normalizeContextSelectionWire")]
pub fn normalize_context_selection_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let selection: Value = wire::read_value(r, &S_NORMALIZE_CONTEXT_SELECTION_0)?;
        let fallback: Value = wire::read_value(r, &S_NORMALIZE_CONTEXT_SELECTION_1)?;
        let out = &port::normalize_context_selection(selection, fallback)?;
        wire::write_value(w, out, &S_NORMALIZE_CONTEXT_SELECTION_OUT)?;
        Ok(())
    })
}

static S_NORMALIZE_RELAY_MODE_0: wire::Schema = wire::Schema::Dyn;
static S_NORMALIZE_RELAY_MODE_OUT: wire::Schema = wire::Schema::Dyn;

/// `normalizeRelayMode` over the binary wire.
#[wasm_bindgen(js_name = "normalizeRelayModeWire")]
pub fn normalize_relay_mode_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let mode: Value = wire::read_value(r, &S_NORMALIZE_RELAY_MODE_0)?;
        let out = &port::normalize_relay_mode(mode)?;
        wire::write_value(w, out, &S_NORMALIZE_RELAY_MODE_OUT)?;
        Ok(())
    })
}

static S_NORMALIZE_RELAY_PROTOCOL_0: wire::Schema = wire::Schema::Dyn;
static S_NORMALIZE_RELAY_PROTOCOL_OUT: wire::Schema = wire::Schema::Dyn;

/// `normalizeRelayProtocol` over the binary wire.
#[wasm_bindgen(js_name = "normalizeRelayProtocolWire")]
pub fn normalize_relay_protocol_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let value: Value = wire::read_value(r, &S_NORMALIZE_RELAY_PROTOCOL_0)?;
        let out = &port::normalize_relay_protocol(value)?;
        wire::write_value(w, out, &S_NORMALIZE_RELAY_PROTOCOL_OUT)?;
        Ok(())
    })
}

static S_NORMALIZE_ANTHROPIC_AUTH_HEADER_0: wire::Schema = wire::Schema::Dyn;

/// `normalizeAnthropicAuthHeader` over the binary wire.
#[wasm_bindgen(js_name = "normalizeAnthropicAuthHeaderWire")]
pub fn normalize_anthropic_auth_header_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let value: Value = wire::read_value(r, &S_NORMALIZE_ANTHROPIC_AUTH_HEADER_0)?;
        let out = &port::normalize_anthropic_auth_header(value)?;
        w.string16(out);
        Ok(())
    })
}

static S_NORMALIZE_ANTHROPIC_THINKING_MODE_0: wire::Schema = wire::Schema::Dyn;

/// `normalizeAnthropicThinkingMode` over the binary wire.
#[wasm_bindgen(js_name = "normalizeAnthropicThinkingModeWire")]
pub fn normalize_anthropic_thinking_mode_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let value: Value = wire::read_value(r, &S_NORMALIZE_ANTHROPIC_THINKING_MODE_0)?;
        let out = &port::normalize_anthropic_thinking_mode(value)?;
        w.string16(out);
        Ok(())
    })
}

static S_NORMALIZE_ANTHROPIC_MAX_TOKENS_0: wire::Schema = wire::Schema::Dyn;

/// `normalizeAnthropicMaxTokens` over the binary wire.
#[wasm_bindgen(js_name = "normalizeAnthropicMaxTokensWire")]
pub fn normalize_anthropic_max_tokens_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let value: Value = wire::read_value(r, &S_NORMALIZE_ANTHROPIC_MAX_TOKENS_0)?;
        let out = &port::normalize_anthropic_max_tokens(value)?;
        w.f64(*out);
        Ok(())
    })
}

static S_HYDRATE_AGGREGATE_RELAY_PROFILE_0: wire::Schema = wire::Schema::Dyn;
static S_HYDRATE_AGGREGATE_RELAY_PROFILE_1: wire::Schema = wire::Schema::Dyn;
static S_HYDRATE_AGGREGATE_RELAY_PROFILE_OUT: wire::Schema = wire::Schema::Dyn;

/// `hydrateAggregateRelayProfile` over the binary wire.
#[wasm_bindgen(js_name = "hydrateAggregateRelayProfileWire")]
pub fn hydrate_aggregate_relay_profile_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let profile: Value = wire::read_value(r, &S_HYDRATE_AGGREGATE_RELAY_PROFILE_0)?;
        let aggregate: Value = wire::read_value(r, &S_HYDRATE_AGGREGATE_RELAY_PROFILE_1)?;
        let out = &port::hydrate_aggregate_relay_profile(profile, aggregate)?;
        wire::write_value(w, out, &S_HYDRATE_AGGREGATE_RELAY_PROFILE_OUT)?;
        Ok(())
    })
}

static S_NORMALIZE_RELAY_PROFILE_0: wire::Schema = wire::Schema::Dyn;
static S_NORMALIZE_RELAY_PROFILE_1: wire::Schema = wire::Schema::Dyn;
static S_NORMALIZE_RELAY_PROFILE_OUT: wire::Schema = wire::Schema::Dyn;

/// `normalizeRelayProfile` over the binary wire.
#[wasm_bindgen(js_name = "normalizeRelayProfileWire")]
pub fn normalize_relay_profile_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let profile: Value = wire::read_value(r, &S_NORMALIZE_RELAY_PROFILE_0)?;
        let default_context_selection: Value = wire::read_value(r, &S_NORMALIZE_RELAY_PROFILE_1)?;
        let out = &port::normalize_relay_profile(profile, default_context_selection)?;
        wire::write_value(w, out, &S_NORMALIZE_RELAY_PROFILE_OUT)?;
        Ok(())
    })
}
