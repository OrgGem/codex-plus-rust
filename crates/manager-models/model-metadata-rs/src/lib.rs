//! JS bindings for `src/model-metadata.ts`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;
pub mod utf16;

/// `modelMetadataKey`.
fn model_metadata_key_inner(row_name: String) -> Result<String, String> {
    let out = port::model_metadata_key(row_name)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "modelMetadataKey")]
pub fn model_metadata_key(row_name: String) -> Result<String, JsError> {
    model_metadata_key_inner(row_name).map_err(|e| JsError::new(&e))
}

/// `parseModelMetadataMap`.
fn parse_model_metadata_map_inner(value: String) -> Result<String, String> {
    let out = port::parse_model_metadata_map(value)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "parseModelMetadataMap")]
pub fn parse_model_metadata_map(value: String) -> Result<String, JsError> {
    parse_model_metadata_map_inner(value).map_err(|e| JsError::new(&e))
}

/// `serializeModelMetadataMap`.
fn serialize_model_metadata_map_inner(map: String) -> Result<String, String> {
    let map = crate::js::from_json(&map)?;
    let out = port::serialize_model_metadata_map(map)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "serializeModelMetadataMap")]
pub fn serialize_model_metadata_map(map: String) -> Result<String, JsError> {
    serialize_model_metadata_map_inner(map).map_err(|e| JsError::new(&e))
}

/// `serializeModelMetadataDocument`.
fn serialize_model_metadata_document_inner(
    slug: String,
    metadata: String,
    context_window: String,
    auto_compact_percent: Option<String>,
) -> Result<String, String> {
    let metadata = crate::js::from_json(&metadata)?;
    let out = port::serialize_model_metadata_document(
        slug,
        metadata,
        context_window,
        auto_compact_percent,
    )?;
    Ok(out)
}

#[wasm_bindgen(js_name = "serializeModelMetadataDocument")]
pub fn serialize_model_metadata_document(
    slug: String,
    metadata: String,
    context_window: String,
    auto_compact_percent: Option<String>,
) -> Result<String, JsError> {
    serialize_model_metadata_document_inner(slug, metadata, context_window, auto_compact_percent)
        .map_err(|e| JsError::new(&e))
}

/// `parseModelRowName`.
fn parse_model_row_name_inner(row_name: String) -> Result<String, String> {
    let out = port::parse_model_row_name(row_name)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "parseModelRowName")]
pub fn parse_model_row_name(row_name: String) -> Result<String, JsError> {
    parse_model_row_name_inner(row_name).map_err(|e| JsError::new(&e))
}

/// `modelSlugFromRowName`.
fn model_slug_from_row_name_inner(row_name: String) -> Result<String, String> {
    let out = port::model_slug_from_row_name(row_name)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "modelSlugFromRowName")]
pub fn model_slug_from_row_name(row_name: String) -> Result<String, JsError> {
    model_slug_from_row_name_inner(row_name).map_err(|e| JsError::new(&e))
}

/// `suffixWindowTokens`.
fn suffix_window_tokens_inner(suffix: String) -> Result<String, String> {
    let out = port::suffix_window_tokens(suffix)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "suffixWindowTokens")]
pub fn suffix_window_tokens(suffix: String) -> Result<String, JsError> {
    suffix_window_tokens_inner(suffix).map_err(|e| JsError::new(&e))
}

/// `suffixWindowString`.
fn suffix_window_string_inner(row_name: String) -> Result<String, String> {
    let out = port::suffix_window_string(row_name)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "suffixWindowString")]
pub fn suffix_window_string(row_name: String) -> Result<String, JsError> {
    suffix_window_string_inner(row_name).map_err(|e| JsError::new(&e))
}

/// `normalizeTokenCountInput`.
fn normalize_token_count_input_inner(value: String) -> Result<String, String> {
    let out = port::normalize_token_count_input(value)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "normalizeTokenCountInput")]
pub fn normalize_token_count_input(value: String) -> Result<String, JsError> {
    normalize_token_count_input_inner(value).map_err(|e| JsError::new(&e))
}

/// `builtinRowBackfillValue`.
fn builtin_row_backfill_value_inner(
    slug: String,
    row_window: String,
    row_auto_compact: String,
    builtin: String,
) -> Result<String, String> {
    let builtin = crate::js::from_json(&builtin)?;
    let out = port::builtin_row_backfill_value(slug, row_window, row_auto_compact, builtin)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "builtinRowBackfillValue")]
pub fn builtin_row_backfill_value(
    slug: String,
    row_window: String,
    row_auto_compact: String,
    builtin: String,
) -> Result<String, JsError> {
    builtin_row_backfill_value_inner(slug, row_window, row_auto_compact, builtin)
        .map_err(|e| JsError::new(&e))
}

/// `createActiveImportDraft`.
fn create_active_import_draft_inner(input: String) -> Result<String, String> {
    let input = crate::js::from_json(&input)?;
    let out = port::create_active_import_draft(input)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "createActiveImportDraft")]
pub fn create_active_import_draft(input: String) -> Result<String, JsError> {
    create_active_import_draft_inner(input).map_err(|e| JsError::new(&e))
}

/// `updateActiveImportDraft`.
fn update_active_import_draft_inner(draft: String, patch: String) -> Result<String, String> {
    let draft = crate::js::from_json(&draft)?;
    let patch = crate::js::from_json(&patch)?;
    let out = port::update_active_import_draft(draft, patch)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "updateActiveImportDraft")]
pub fn update_active_import_draft(draft: String, patch: String) -> Result<String, JsError> {
    update_active_import_draft_inner(draft, patch).map_err(|e| JsError::new(&e))
}

/// `cancelActiveImportDraft`.
fn cancel_active_import_draft_inner(draft: String) -> Result<String, String> {
    let draft = crate::js::from_json(&draft)?;
    let out = port::cancel_active_import_draft(draft)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "cancelActiveImportDraft")]
pub fn cancel_active_import_draft(draft: String) -> Result<String, JsError> {
    cancel_active_import_draft_inner(draft).map_err(|e| JsError::new(&e))
}

/// `rematchActiveImportDraft`.
fn rematch_active_import_draft_inner(
    draft: String,
    document: String,
    preview: String,
) -> Result<String, String> {
    let draft = crate::js::from_json(&draft)?;
    let preview = crate::js::from_json(&preview)?;
    let out = port::rematch_active_import_draft(draft, document, preview)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "rematchActiveImportDraft")]
pub fn rematch_active_import_draft(
    draft: String,
    document: String,
    preview: String,
) -> Result<String, JsError> {
    rematch_active_import_draft_inner(draft, document, preview).map_err(|e| JsError::new(&e))
}

/// `builtinEntryToImportDocument`.
fn builtin_entry_to_import_document_inner(
    entry: String,
    row_name: Option<String>,
) -> Result<String, String> {
    let entry = crate::js::from_json(&entry)?;
    let out = port::builtin_entry_to_import_document(entry, row_name)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "builtinEntryToImportDocument")]
pub fn builtin_entry_to_import_document(
    entry: String,
    row_name: Option<String>,
) -> Result<String, JsError> {
    builtin_entry_to_import_document_inner(entry, row_name).map_err(|e| JsError::new(&e))
}

/// `metadataSourceTags`.
fn metadata_source_tags_inner(options: String) -> Result<String, String> {
    let options = crate::js::from_json(&options)?;
    let out = port::metadata_source_tags(options)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "metadataSourceTags")]
pub fn metadata_source_tags(options: String) -> Result<String, JsError> {
    metadata_source_tags_inner(options).map_err(|e| JsError::new(&e))
}

/// `importDocumentSyncPatch`.
fn import_document_sync_patch_inner(row: String, preview: String) -> Result<String, String> {
    let row = crate::js::from_json(&row)?;
    let preview = crate::js::from_json(&preview)?;
    let out = port::import_document_sync_patch(row, preview)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "importDocumentSyncPatch")]
pub fn import_document_sync_patch(row: String, preview: String) -> Result<String, JsError> {
    import_document_sync_patch_inner(row, preview).map_err(|e| JsError::new(&e))
}

/// `metadataMatchesBuiltin`.
fn metadata_matches_builtin_inner(
    metadata: String,
    builtin_metadata: String,
) -> Result<bool, String> {
    let metadata = crate::js::from_json(&metadata)?;
    let builtin_metadata = crate::js::from_json(&builtin_metadata)?;
    let out = port::metadata_matches_builtin(metadata, builtin_metadata)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "metadataMatchesBuiltin")]
pub fn metadata_matches_builtin(
    metadata: String,
    builtin_metadata: String,
) -> Result<bool, JsError> {
    metadata_matches_builtin_inner(metadata, builtin_metadata).map_err(|e| JsError::new(&e))
}

/// `importSaveDecision`.
fn import_save_decision_inner(options: String) -> Result<String, String> {
    let options = crate::js::from_json(&options)?;
    let out = port::import_save_decision(options)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "importSaveDecision")]
pub fn import_save_decision(options: String) -> Result<String, JsError> {
    import_save_decision_inner(options).map_err(|e| JsError::new(&e))
}

/// `importPanelControls`.
fn import_panel_controls_inner(options: String) -> Result<String, String> {
    let options = crate::js::from_json(&options)?;
    let out = port::import_panel_controls(options)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "importPanelControls")]
pub fn import_panel_controls(options: String) -> Result<String, JsError> {
    import_panel_controls_inner(options).map_err(|e| JsError::new(&e))
}

/// `replaceModelMetadataForSlug`.
fn replace_model_metadata_for_slug_inner(
    value: String,
    slug: String,
    metadata: String,
) -> Result<String, String> {
    let metadata = crate::js::from_json(&metadata)?;
    let out = port::replace_model_metadata_for_slug(value, slug, metadata)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "replaceModelMetadataForSlug")]
pub fn replace_model_metadata_for_slug(
    value: String,
    slug: String,
    metadata: String,
) -> Result<String, JsError> {
    replace_model_metadata_for_slug_inner(value, slug, metadata).map_err(|e| JsError::new(&e))
}

/// `resolveModelMetadataRowKey`.
fn resolve_model_metadata_row_key_inner(map: String, row_names: String) -> Result<String, String> {
    let map = crate::js::from_json(&map)?;
    let row_names = crate::js::from_json(&row_names)?;
    let out = port::resolve_model_metadata_row_key(map, row_names)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "resolveModelMetadataRowKey")]
pub fn resolve_model_metadata_row_key(map: String, row_names: String) -> Result<String, JsError> {
    resolve_model_metadata_row_key_inner(map, row_names).map_err(|e| JsError::new(&e))
}

/// `clearModelMetadataForSlug`.
fn clear_model_metadata_for_slug_inner(value: String, slug: String) -> Result<String, String> {
    let out = port::clear_model_metadata_for_slug(value, slug)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "clearModelMetadataForSlug")]
pub fn clear_model_metadata_for_slug(value: String, slug: String) -> Result<String, JsError> {
    clear_model_metadata_for_slug_inner(value, slug).map_err(|e| JsError::new(&e))
}

/// `remapModelMetadataSlugs`.
fn remap_model_metadata_slugs_inner(value: String, mappings: String) -> Result<String, String> {
    let mappings = crate::js::from_json(&mappings)?;
    let out = port::remap_model_metadata_slugs(value, mappings)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "remapModelMetadataSlugs")]
pub fn remap_model_metadata_slugs(value: String, mappings: String) -> Result<String, JsError> {
    remap_model_metadata_slugs_inner(value, mappings).map_err(|e| JsError::new(&e))
}

/// `retainModelMetadataForSlugs`.
fn retain_model_metadata_for_slugs_inner(value: String, slugs: String) -> Result<String, String> {
    let slugs = crate::js::from_json(&slugs)?;
    let out = port::retain_model_metadata_for_slugs(value, slugs)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "retainModelMetadataForSlugs")]
pub fn retain_model_metadata_for_slugs(value: String, slugs: String) -> Result<String, JsError> {
    retain_model_metadata_for_slugs_inner(value, slugs).map_err(|e| JsError::new(&e))
}

/// `modelFastSupportMode`.
fn model_fast_support_mode_inner(metadata: String) -> Result<String, String> {
    let metadata = crate::js::from_json(&metadata)?;
    let out = port::model_fast_support_mode(metadata)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "modelFastSupportMode")]
pub fn model_fast_support_mode(metadata: String) -> Result<String, JsError> {
    model_fast_support_mode_inner(metadata).map_err(|e| JsError::new(&e))
}

/// `setModelFastSupportMode`.
fn set_model_fast_support_mode_inner(metadata: String, mode: String) -> Result<String, String> {
    let metadata = crate::js::from_json(&metadata)?;
    let out = port::set_model_fast_support_mode(metadata, mode)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "setModelFastSupportMode")]
pub fn set_model_fast_support_mode(metadata: String, mode: String) -> Result<String, JsError> {
    set_model_fast_support_mode_inner(metadata, mode).map_err(|e| JsError::new(&e))
}

/// `synchronizeModelMetadataFastSupport`.
fn synchronize_model_metadata_fast_support_inner(
    source: String,
    target_slug: String,
    mode: String,
) -> Result<String, String> {
    let out = port::synchronize_model_metadata_fast_support(source, target_slug, mode)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "synchronizeModelMetadataFastSupport")]
pub fn synchronize_model_metadata_fast_support(
    source: String,
    target_slug: String,
    mode: String,
) -> Result<String, JsError> {
    synchronize_model_metadata_fast_support_inner(source, target_slug, mode)
        .map_err(|e| JsError::new(&e))
}

/// `synchronizeModelMetadataLiteSupport`.
fn synchronize_model_metadata_lite_support_inner(
    source: String,
    target_slug: String,
    mode: String,
) -> Result<String, String> {
    let out = port::synchronize_model_metadata_lite_support(source, target_slug, mode)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "synchronizeModelMetadataLiteSupport")]
pub fn synchronize_model_metadata_lite_support(
    source: String,
    target_slug: String,
    mode: String,
) -> Result<String, JsError> {
    synchronize_model_metadata_lite_support_inner(source, target_slug, mode)
        .map_err(|e| JsError::new(&e))
}

/// `modelMultiAgentMode`.
fn model_multi_agent_mode_inner(metadata: String) -> Result<String, String> {
    let metadata = crate::js::from_json(&metadata)?;
    let out = port::model_multi_agent_mode(metadata)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "modelMultiAgentMode")]
pub fn model_multi_agent_mode(metadata: String) -> Result<String, JsError> {
    model_multi_agent_mode_inner(metadata).map_err(|e| JsError::new(&e))
}

/// `synchronizeModelMetadataMultiAgentVersion`.
fn synchronize_model_metadata_multi_agent_version_inner(
    source: String,
    target_slug: String,
    mode: String,
) -> Result<String, String> {
    let out = port::synchronize_model_metadata_multi_agent_version(source, target_slug, mode)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "synchronizeModelMetadataMultiAgentVersion")]
pub fn synchronize_model_metadata_multi_agent_version(
    source: String,
    target_slug: String,
    mode: String,
) -> Result<String, JsError> {
    synchronize_model_metadata_multi_agent_version_inner(source, target_slug, mode)
        .map_err(|e| JsError::new(&e))
}

/// `synchronizeModelMetadataDocumentContextWindow`.
fn synchronize_model_metadata_document_context_window_inner(
    source: String,
    target_slug: String,
    context_window: String,
) -> Result<String, String> {
    let out = port::synchronize_model_metadata_document_context_window(
        source,
        target_slug,
        context_window,
    )?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "synchronizeModelMetadataDocumentContextWindow")]
pub fn synchronize_model_metadata_document_context_window(
    source: String,
    target_slug: String,
    context_window: String,
) -> Result<String, JsError> {
    synchronize_model_metadata_document_context_window_inner(source, target_slug, context_window)
        .map_err(|e| JsError::new(&e))
}

/// `synchronizeModelMetadataDocumentLimits`.
fn synchronize_model_metadata_document_limits_inner(
    source: String,
    target_slug: String,
    context_window: String,
    auto_compact_percent: String,
) -> Result<String, String> {
    let out = port::synchronize_model_metadata_document_limits(
        source,
        target_slug,
        context_window,
        auto_compact_percent,
    )?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "synchronizeModelMetadataDocumentLimits")]
pub fn synchronize_model_metadata_document_limits(
    source: String,
    target_slug: String,
    context_window: String,
    auto_compact_percent: String,
) -> Result<String, JsError> {
    synchronize_model_metadata_document_limits_inner(
        source,
        target_slug,
        context_window,
        auto_compact_percent,
    )
    .map_err(|e| JsError::new(&e))
}

/// `synchronizeModelMetadataDocumentLimitsPreview`.
fn synchronize_model_metadata_document_limits_preview_inner(
    source: String,
    target_slug: String,
    context_window: String,
    auto_compact_percent: String,
) -> Result<String, String> {
    let out = port::synchronize_model_metadata_document_limits_preview(
        source,
        target_slug,
        context_window,
        auto_compact_percent,
    )?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "synchronizeModelMetadataDocumentLimitsPreview")]
pub fn synchronize_model_metadata_document_limits_preview(
    source: String,
    target_slug: String,
    context_window: String,
    auto_compact_percent: String,
) -> Result<String, JsError> {
    synchronize_model_metadata_document_limits_preview_inner(
        source,
        target_slug,
        context_window,
        auto_compact_percent,
    )
    .map_err(|e| JsError::new(&e))
}

/// `validateModelCapabilities`.
fn validate_model_capabilities_inner(metadata: String) -> Result<String, String> {
    let metadata = crate::js::from_json(&metadata)?;
    let out = port::validate_model_capabilities(metadata)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "validateModelCapabilities")]
pub fn validate_model_capabilities(metadata: String) -> Result<String, JsError> {
    validate_model_capabilities_inner(metadata).map_err(|e| JsError::new(&e))
}

/// `parseModelMetadataDocument`.
fn parse_model_metadata_document_inner(
    source: String,
    target_slug: String,
) -> Result<String, String> {
    let out = port::parse_model_metadata_document(source, target_slug)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "parseModelMetadataDocument")]
pub fn parse_model_metadata_document(
    source: String,
    target_slug: String,
) -> Result<String, JsError> {
    parse_model_metadata_document_inner(source, target_slug).map_err(|e| JsError::new(&e))
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

static S_PARSE_MODEL_METADATA_MAP_OUT: wire::Schema =
    wire::Schema::Map16(&wire::Schema::Map16(&wire::Schema::Dyn));

/// `parseModelMetadataMap` over the binary wire.
#[wasm_bindgen(js_name = "parseModelMetadataMapWire")]
pub fn parse_model_metadata_map_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let value: String = r.string16()?;
        let out = &port::parse_model_metadata_map(value)?;
        wire::write_value(w, out, &S_PARSE_MODEL_METADATA_MAP_OUT)?;
        Ok(())
    })
}

static S_SERIALIZE_MODEL_METADATA_MAP_0: wire::Schema = wire::Schema::Dyn;

/// `serializeModelMetadataMap` over the binary wire.
#[wasm_bindgen(js_name = "serializeModelMetadataMapWire")]
pub fn serialize_model_metadata_map_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let map: Value = wire::read_value(r, &S_SERIALIZE_MODEL_METADATA_MAP_0)?;
        let out = &port::serialize_model_metadata_map(map)?;
        w.string16(out);
        Ok(())
    })
}

static S_SERIALIZE_MODEL_METADATA_DOCUMENT_1: wire::Schema = wire::Schema::Dyn;

/// `serializeModelMetadataDocument` over the binary wire.
#[wasm_bindgen(js_name = "serializeModelMetadataDocumentWire")]
pub fn serialize_model_metadata_document_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let slug: String = r.string16()?;
        let metadata: Value = wire::read_value(r, &S_SERIALIZE_MODEL_METADATA_DOCUMENT_1)?;
        let context_window: String = r.string16()?;
        let auto_compact_percent: Option<String> = r.opt_js(|r| r.string16())?;
        let out = &port::serialize_model_metadata_document(
            slug,
            metadata,
            context_window,
            auto_compact_percent,
        )?;
        w.string16(out);
        Ok(())
    })
}

static S_PARSE_MODEL_ROW_NAME_OUT: wire::Schema = wire::Schema::Obj(&[
    ("rawName", wire::Schema::Str16),
    ("canonicalSlug", wire::Schema::Str16),
    ("suffixWindow", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("key", wire::Schema::Str16),
]);

/// `parseModelRowName` over the binary wire.
#[wasm_bindgen(js_name = "parseModelRowNameWire")]
pub fn parse_model_row_name_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let row_name: String = r.string16()?;
        let out = &port::parse_model_row_name(row_name)?;
        wire::write_value(w, out, &S_PARSE_MODEL_ROW_NAME_OUT)?;
        Ok(())
    })
}

static S_SUFFIX_WINDOW_TOKENS_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Num);

/// `suffixWindowTokens` over the binary wire.
#[wasm_bindgen(js_name = "suffixWindowTokensWire")]
pub fn suffix_window_tokens_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let suffix: String = r.string16()?;
        let out = &port::suffix_window_tokens(suffix)?;
        wire::write_value(w, out, &S_SUFFIX_WINDOW_TOKENS_OUT)?;
        Ok(())
    })
}

static S_SUFFIX_WINDOW_STRING_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Str16);

/// `suffixWindowString` over the binary wire.
#[wasm_bindgen(js_name = "suffixWindowStringWire")]
pub fn suffix_window_string_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let row_name: String = r.string16()?;
        let out = &port::suffix_window_string(row_name)?;
        wire::write_value(w, out, &S_SUFFIX_WINDOW_STRING_OUT)?;
        Ok(())
    })
}

static S_BUILTIN_ROW_BACKFILL_VALUE_OUT: wire::Schema = wire::Schema::Obj(&[
    ("window", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("autoCompact", wire::Schema::OptJs(&wire::Schema::Str16)),
]);

/// `builtinRowBackfillValue` over the binary wire.
#[wasm_bindgen(js_name = "builtinRowBackfillValueWire")]
pub fn builtin_row_backfill_value_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let slug: String = r.string16()?;
        let row_window: String = r.string16()?;
        let row_auto_compact: String = r.string16()?;
        let builtin: Option<Value> = wire::read_dyn_opt(r)?;
        let out = &port::builtin_row_backfill_value(slug, row_window, row_auto_compact, builtin)?;
        wire::write_value(w, out, &S_BUILTIN_ROW_BACKFILL_VALUE_OUT)?;
        Ok(())
    })
}

static S_CREATE_ACTIVE_IMPORT_DRAFT_0: wire::Schema = wire::Schema::Dyn;
static S_CREATE_ACTIVE_IMPORT_DRAFT_OUT: wire::Schema = wire::Schema::Obj(&[
    ("index", wire::Schema::Num),
    ("originalSlug", wire::Schema::Str16),
    ("originalWindow", wire::Schema::Str16),
    ("originalAutoCompact", wire::Schema::Str16),
    ("document", wire::Schema::Str16),
    (
        "preview",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("slug", wire::Schema::Str16),
            ("metadata", wire::Schema::Map16(&wire::Schema::Dyn)),
            ("documentEntry", wire::Schema::Map16(&wire::Schema::Dyn)),
            ("contextWindow", wire::Schema::OptJs(&wire::Schema::Str16)),
            (
                "autoCompactPercent",
                wire::Schema::OptJs(&wire::Schema::Str16),
            ),
            (
                "autoCompactCalculationPercent",
                wire::Schema::OptJs(&wire::Schema::Str16),
            ),
            ("ignoredFields", wire::Schema::List(&wire::Schema::Str16)),
        ])),
    ),
]);

/// `createActiveImportDraft` over the binary wire.
#[wasm_bindgen(js_name = "createActiveImportDraftWire")]
pub fn create_active_import_draft_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let input: Value = wire::read_value(r, &S_CREATE_ACTIVE_IMPORT_DRAFT_0)?;
        let out = &port::create_active_import_draft(input)?;
        wire::write_value(w, out, &S_CREATE_ACTIVE_IMPORT_DRAFT_OUT)?;
        Ok(())
    })
}

static S_UPDATE_ACTIVE_IMPORT_DRAFT_0: wire::Schema = wire::Schema::Dyn;
static S_UPDATE_ACTIVE_IMPORT_DRAFT_1: wire::Schema = wire::Schema::Dyn;
static S_UPDATE_ACTIVE_IMPORT_DRAFT_OUT: wire::Schema = wire::Schema::Obj(&[
    ("index", wire::Schema::Num),
    ("originalSlug", wire::Schema::Str16),
    ("originalWindow", wire::Schema::Str16),
    ("originalAutoCompact", wire::Schema::Str16),
    ("document", wire::Schema::Str16),
    (
        "preview",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("slug", wire::Schema::Str16),
            ("metadata", wire::Schema::Map16(&wire::Schema::Dyn)),
            ("documentEntry", wire::Schema::Map16(&wire::Schema::Dyn)),
            ("contextWindow", wire::Schema::OptJs(&wire::Schema::Str16)),
            (
                "autoCompactPercent",
                wire::Schema::OptJs(&wire::Schema::Str16),
            ),
            (
                "autoCompactCalculationPercent",
                wire::Schema::OptJs(&wire::Schema::Str16),
            ),
            ("ignoredFields", wire::Schema::List(&wire::Schema::Str16)),
        ])),
    ),
]);

/// `updateActiveImportDraft` over the binary wire.
#[wasm_bindgen(js_name = "updateActiveImportDraftWire")]
pub fn update_active_import_draft_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let draft: Value = wire::read_value(r, &S_UPDATE_ACTIVE_IMPORT_DRAFT_0)?;
        let patch: Value = wire::read_value(r, &S_UPDATE_ACTIVE_IMPORT_DRAFT_1)?;
        let out = &port::update_active_import_draft(draft, patch)?;
        wire::write_value(w, out, &S_UPDATE_ACTIVE_IMPORT_DRAFT_OUT)?;
        Ok(())
    })
}

static S_CANCEL_ACTIVE_IMPORT_DRAFT_0: wire::Schema = wire::Schema::Dyn;
static S_CANCEL_ACTIVE_IMPORT_DRAFT_OUT: wire::Schema = wire::Schema::Obj(&[
    ("draft", wire::Schema::Dyn),
    (
        "rowPatch",
        wire::Schema::Obj(&[
            ("window", wire::Schema::Str16),
            ("autoCompact", wire::Schema::Str16),
        ]),
    ),
]);

/// `cancelActiveImportDraft` over the binary wire.
#[wasm_bindgen(js_name = "cancelActiveImportDraftWire")]
pub fn cancel_active_import_draft_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let draft: Value = wire::read_value(r, &S_CANCEL_ACTIVE_IMPORT_DRAFT_0)?;
        let out = &port::cancel_active_import_draft(draft)?;
        wire::write_value(w, out, &S_CANCEL_ACTIVE_IMPORT_DRAFT_OUT)?;
        Ok(())
    })
}

static S_REMATCH_ACTIVE_IMPORT_DRAFT_0: wire::Schema = wire::Schema::Dyn;
static S_REMATCH_ACTIVE_IMPORT_DRAFT_OUT: wire::Schema = wire::Schema::Obj(&[
    ("index", wire::Schema::Num),
    ("originalSlug", wire::Schema::Str16),
    ("originalWindow", wire::Schema::Str16),
    ("originalAutoCompact", wire::Schema::Str16),
    ("document", wire::Schema::Str16),
    (
        "preview",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("slug", wire::Schema::Str16),
            ("metadata", wire::Schema::Map16(&wire::Schema::Dyn)),
            ("documentEntry", wire::Schema::Map16(&wire::Schema::Dyn)),
            ("contextWindow", wire::Schema::OptJs(&wire::Schema::Str16)),
            (
                "autoCompactPercent",
                wire::Schema::OptJs(&wire::Schema::Str16),
            ),
            (
                "autoCompactCalculationPercent",
                wire::Schema::OptJs(&wire::Schema::Str16),
            ),
            ("ignoredFields", wire::Schema::List(&wire::Schema::Str16)),
        ])),
    ),
]);

/// `rematchActiveImportDraft` over the binary wire.
#[wasm_bindgen(js_name = "rematchActiveImportDraftWire")]
pub fn rematch_active_import_draft_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let draft: Value = wire::read_value(r, &S_REMATCH_ACTIVE_IMPORT_DRAFT_0)?;
        let document: String = r.string16()?;
        let preview: Option<Value> = wire::read_dyn_opt(r)?;
        let out = &port::rematch_active_import_draft(draft, document, preview)?;
        wire::write_value(w, out, &S_REMATCH_ACTIVE_IMPORT_DRAFT_OUT)?;
        Ok(())
    })
}

static S_BUILTIN_ENTRY_TO_IMPORT_DOCUMENT_0: wire::Schema = wire::Schema::Dyn;

/// `builtinEntryToImportDocument` over the binary wire.
#[wasm_bindgen(js_name = "builtinEntryToImportDocumentWire")]
pub fn builtin_entry_to_import_document_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let entry: Value = wire::read_value(r, &S_BUILTIN_ENTRY_TO_IMPORT_DOCUMENT_0)?;
        let row_name: Option<String> = r.opt_js(|r| r.string16())?;
        let out = &port::builtin_entry_to_import_document(entry, row_name)?;
        w.string16(out);
        Ok(())
    })
}

static S_METADATA_SOURCE_TAGS_0: wire::Schema = wire::Schema::Dyn;
static S_METADATA_SOURCE_TAGS_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Dyn);

/// `metadataSourceTags` over the binary wire.
#[wasm_bindgen(js_name = "metadataSourceTagsWire")]
pub fn metadata_source_tags_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let options: Value = wire::read_value(r, &S_METADATA_SOURCE_TAGS_0)?;
        let out = &port::metadata_source_tags(options)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_METADATA_SOURCE_TAGS_OUT.inner())?;
        }
        Ok(())
    })
}

static S_IMPORT_DOCUMENT_SYNC_PATCH_0: wire::Schema = wire::Schema::Dyn;
static S_IMPORT_DOCUMENT_SYNC_PATCH_1: wire::Schema = wire::Schema::Dyn;
static S_IMPORT_DOCUMENT_SYNC_PATCH_OUT: wire::Schema = wire::Schema::Obj(&[
    ("window", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("autoCompact", wire::Schema::OptJs(&wire::Schema::Str16)),
]);

/// `importDocumentSyncPatch` over the binary wire.
#[wasm_bindgen(js_name = "importDocumentSyncPatchWire")]
pub fn import_document_sync_patch_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let row: Value = wire::read_value(r, &S_IMPORT_DOCUMENT_SYNC_PATCH_0)?;
        let preview: Value = wire::read_value(r, &S_IMPORT_DOCUMENT_SYNC_PATCH_1)?;
        let out = &port::import_document_sync_patch(row, preview)?;
        wire::write_value(w, out, &S_IMPORT_DOCUMENT_SYNC_PATCH_OUT)?;
        Ok(())
    })
}

/// `metadataMatchesBuiltin` over the binary wire.
#[wasm_bindgen(js_name = "metadataMatchesBuiltinWire")]
pub fn metadata_matches_builtin_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let metadata: Option<Value> = wire::read_dyn_opt(r)?;
        let builtin_metadata: Option<Value> = wire::read_dyn_opt(r)?;
        let out = &port::metadata_matches_builtin(metadata, builtin_metadata)?;
        w.bool(*out);
        Ok(())
    })
}

static S_IMPORT_SAVE_DECISION_0: wire::Schema = wire::Schema::Dyn;
static S_IMPORT_SAVE_DECISION_OUT: wire::Schema = wire::Schema::Obj(&[
    ("needsSave", wire::Schema::Bool),
    ("effect", wire::Schema::Str16),
    ("label", wire::Schema::Str16),
    ("title", wire::Schema::Str16),
]);

/// `importSaveDecision` over the binary wire.
#[wasm_bindgen(js_name = "importSaveDecisionWire")]
pub fn import_save_decision_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let options: Value = wire::read_value(r, &S_IMPORT_SAVE_DECISION_0)?;
        let out = &port::import_save_decision(options)?;
        wire::write_value(w, out, &S_IMPORT_SAVE_DECISION_OUT)?;
        Ok(())
    })
}

static S_IMPORT_PANEL_CONTROLS_0: wire::Schema = wire::Schema::Dyn;
static S_IMPORT_PANEL_CONTROLS_OUT: wire::Schema = wire::Schema::Obj(&[
    (
        "rematch",
        wire::Schema::Obj(&[
            ("disabled", wire::Schema::Bool),
            ("title", wire::Schema::Str16),
        ]),
    ),
    (
        "clear",
        wire::Schema::Obj(&[
            ("disabled", wire::Schema::Bool),
            ("title", wire::Schema::Str16),
        ]),
    ),
    (
        "cancel",
        wire::Schema::Obj(&[
            ("disabled", wire::Schema::Bool),
            ("title", wire::Schema::Str16),
        ]),
    ),
    (
        "save",
        wire::Schema::Obj(&[
            ("disabled", wire::Schema::Bool),
            ("label", wire::Schema::Str16),
            ("title", wire::Schema::Str16),
        ]),
    ),
]);

/// `importPanelControls` over the binary wire.
#[wasm_bindgen(js_name = "importPanelControlsWire")]
pub fn import_panel_controls_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let options: Value = wire::read_value(r, &S_IMPORT_PANEL_CONTROLS_0)?;
        let out = &port::import_panel_controls(options)?;
        wire::write_value(w, out, &S_IMPORT_PANEL_CONTROLS_OUT)?;
        Ok(())
    })
}

static S_REPLACE_MODEL_METADATA_FOR_SLUG_2: wire::Schema = wire::Schema::Dyn;

/// `replaceModelMetadataForSlug` over the binary wire.
#[wasm_bindgen(js_name = "replaceModelMetadataForSlugWire")]
pub fn replace_model_metadata_for_slug_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let value: String = r.string16()?;
        let slug: String = r.string16()?;
        let metadata: Value = wire::read_value(r, &S_REPLACE_MODEL_METADATA_FOR_SLUG_2)?;
        let out = &port::replace_model_metadata_for_slug(value, slug, metadata)?;
        w.string16(out);
        Ok(())
    })
}

static S_RESOLVE_MODEL_METADATA_ROW_KEY_0: wire::Schema = wire::Schema::Dyn;
static S_RESOLVE_MODEL_METADATA_ROW_KEY_1: wire::Schema = wire::Schema::Dyn;
static S_RESOLVE_MODEL_METADATA_ROW_KEY_OUT: wire::Schema =
    wire::Schema::OptJs(&wire::Schema::Str16);

/// `resolveModelMetadataRowKey` over the binary wire.
#[wasm_bindgen(js_name = "resolveModelMetadataRowKeyWire")]
pub fn resolve_model_metadata_row_key_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let map: Value = wire::read_value(r, &S_RESOLVE_MODEL_METADATA_ROW_KEY_0)?;
        let row_names: Value = wire::read_value(r, &S_RESOLVE_MODEL_METADATA_ROW_KEY_1)?;
        let out = &port::resolve_model_metadata_row_key(map, row_names)?;
        wire::write_value(w, out, &S_RESOLVE_MODEL_METADATA_ROW_KEY_OUT)?;
        Ok(())
    })
}

static S_REMAP_MODEL_METADATA_SLUGS_1: wire::Schema = wire::Schema::Dyn;

/// `remapModelMetadataSlugs` over the binary wire.
#[wasm_bindgen(js_name = "remapModelMetadataSlugsWire")]
pub fn remap_model_metadata_slugs_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let value: String = r.string16()?;
        let mappings: Value = wire::read_value(r, &S_REMAP_MODEL_METADATA_SLUGS_1)?;
        let out = &port::remap_model_metadata_slugs(value, mappings)?;
        w.string16(out);
        Ok(())
    })
}

static S_RETAIN_MODEL_METADATA_FOR_SLUGS_1: wire::Schema = wire::Schema::Dyn;

/// `retainModelMetadataForSlugs` over the binary wire.
#[wasm_bindgen(js_name = "retainModelMetadataForSlugsWire")]
pub fn retain_model_metadata_for_slugs_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let value: String = r.string16()?;
        let slugs: Value = wire::read_value(r, &S_RETAIN_MODEL_METADATA_FOR_SLUGS_1)?;
        let out = &port::retain_model_metadata_for_slugs(value, slugs)?;
        w.string16(out);
        Ok(())
    })
}

/// `modelFastSupportMode` over the binary wire.
#[wasm_bindgen(js_name = "modelFastSupportModeWire")]
pub fn model_fast_support_mode_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let metadata: Option<Value> = wire::read_dyn_opt(r)?;
        let out = &port::model_fast_support_mode(metadata)?;
        w.string16(out);
        Ok(())
    })
}

static S_SET_MODEL_FAST_SUPPORT_MODE_0: wire::Schema = wire::Schema::Dyn;
static S_SET_MODEL_FAST_SUPPORT_MODE_OUT: wire::Schema = wire::Schema::Map16(&wire::Schema::Dyn);

/// `setModelFastSupportMode` over the binary wire.
#[wasm_bindgen(js_name = "setModelFastSupportModeWire")]
pub fn set_model_fast_support_mode_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let metadata: Value = wire::read_value(r, &S_SET_MODEL_FAST_SUPPORT_MODE_0)?;
        let mode: String = r.string16()?;
        let out = &port::set_model_fast_support_mode(metadata, mode)?;
        wire::write_value(w, out, &S_SET_MODEL_FAST_SUPPORT_MODE_OUT)?;
        Ok(())
    })
}

static S_SYNCHRONIZE_MODEL_METADATA_FAST_SUPPORT_OUT: wire::Schema =
    wire::Schema::OptJs(&wire::Schema::Obj(&[
        ("document", wire::Schema::Str16),
        (
            "preview",
            wire::Schema::Obj(&[
                ("slug", wire::Schema::Str16),
                ("metadata", wire::Schema::Map16(&wire::Schema::Dyn)),
                ("documentEntry", wire::Schema::Map16(&wire::Schema::Dyn)),
                ("contextWindow", wire::Schema::OptJs(&wire::Schema::Str16)),
                (
                    "autoCompactPercent",
                    wire::Schema::OptJs(&wire::Schema::Str16),
                ),
                (
                    "autoCompactCalculationPercent",
                    wire::Schema::OptJs(&wire::Schema::Str16),
                ),
                ("ignoredFields", wire::Schema::List(&wire::Schema::Str16)),
            ]),
        ),
    ]));

/// `synchronizeModelMetadataFastSupport` over the binary wire.
#[wasm_bindgen(js_name = "synchronizeModelMetadataFastSupportWire")]
pub fn synchronize_model_metadata_fast_support_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let source: String = r.string16()?;
        let target_slug: String = r.string16()?;
        let mode: String = r.string16()?;
        let out = &port::synchronize_model_metadata_fast_support(source, target_slug, mode)?;
        match out {
            Some(x0) => {
                w.tag(1);
                wire::write_value(w, x0, S_SYNCHRONIZE_MODEL_METADATA_FAST_SUPPORT_OUT.inner())?;
            }
            None => w.tag(2),
        }
        Ok(())
    })
}

static S_SYNCHRONIZE_MODEL_METADATA_LITE_SUPPORT_OUT: wire::Schema =
    wire::Schema::OptJs(&wire::Schema::Obj(&[
        ("document", wire::Schema::Str16),
        (
            "preview",
            wire::Schema::Obj(&[
                ("slug", wire::Schema::Str16),
                ("metadata", wire::Schema::Map16(&wire::Schema::Dyn)),
                ("documentEntry", wire::Schema::Map16(&wire::Schema::Dyn)),
                ("contextWindow", wire::Schema::OptJs(&wire::Schema::Str16)),
                (
                    "autoCompactPercent",
                    wire::Schema::OptJs(&wire::Schema::Str16),
                ),
                (
                    "autoCompactCalculationPercent",
                    wire::Schema::OptJs(&wire::Schema::Str16),
                ),
                ("ignoredFields", wire::Schema::List(&wire::Schema::Str16)),
            ]),
        ),
    ]));

/// `synchronizeModelMetadataLiteSupport` over the binary wire.
#[wasm_bindgen(js_name = "synchronizeModelMetadataLiteSupportWire")]
pub fn synchronize_model_metadata_lite_support_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let source: String = r.string16()?;
        let target_slug: String = r.string16()?;
        let mode: String = r.string16()?;
        let out = &port::synchronize_model_metadata_lite_support(source, target_slug, mode)?;
        match out {
            Some(x0) => {
                w.tag(1);
                wire::write_value(w, x0, S_SYNCHRONIZE_MODEL_METADATA_LITE_SUPPORT_OUT.inner())?;
            }
            None => w.tag(2),
        }
        Ok(())
    })
}

/// `modelMultiAgentMode` over the binary wire.
#[wasm_bindgen(js_name = "modelMultiAgentModeWire")]
pub fn model_multi_agent_mode_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let metadata: Option<Value> = wire::read_dyn_opt(r)?;
        let out = &port::model_multi_agent_mode(metadata)?;
        w.string16(out);
        Ok(())
    })
}

static S_SYNCHRONIZE_MODEL_METADATA_MULTI_AGENT_VERSION_OUT: wire::Schema =
    wire::Schema::OptJs(&wire::Schema::Obj(&[
        ("document", wire::Schema::Str16),
        (
            "preview",
            wire::Schema::Obj(&[
                ("slug", wire::Schema::Str16),
                ("metadata", wire::Schema::Map16(&wire::Schema::Dyn)),
                ("documentEntry", wire::Schema::Map16(&wire::Schema::Dyn)),
                ("contextWindow", wire::Schema::OptJs(&wire::Schema::Str16)),
                (
                    "autoCompactPercent",
                    wire::Schema::OptJs(&wire::Schema::Str16),
                ),
                (
                    "autoCompactCalculationPercent",
                    wire::Schema::OptJs(&wire::Schema::Str16),
                ),
                ("ignoredFields", wire::Schema::List(&wire::Schema::Str16)),
            ]),
        ),
    ]));

/// `synchronizeModelMetadataMultiAgentVersion` over the binary wire.
#[wasm_bindgen(js_name = "synchronizeModelMetadataMultiAgentVersionWire")]
pub fn synchronize_model_metadata_multi_agent_version_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let source: String = r.string16()?;
        let target_slug: String = r.string16()?;
        let mode: String = r.string16()?;
        let out = &port::synchronize_model_metadata_multi_agent_version(source, target_slug, mode)?;
        match out {
            Some(x0) => {
                w.tag(1);
                wire::write_value(
                    w,
                    x0,
                    S_SYNCHRONIZE_MODEL_METADATA_MULTI_AGENT_VERSION_OUT.inner(),
                )?;
            }
            None => w.tag(2),
        }
        Ok(())
    })
}

static S_SYNCHRONIZE_MODEL_METADATA_DOCUMENT_CONTEXT_WINDOW_OUT: wire::Schema =
    wire::Schema::OptJs(&wire::Schema::Str16);

/// `synchronizeModelMetadataDocumentContextWindow` over the binary wire.
#[wasm_bindgen(js_name = "synchronizeModelMetadataDocumentContextWindowWire")]
pub fn synchronize_model_metadata_document_context_window_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let source: String = r.string16()?;
        let target_slug: String = r.string16()?;
        let context_window: String = r.string16()?;
        let out = &port::synchronize_model_metadata_document_context_window(
            source,
            target_slug,
            context_window,
        )?;
        wire::write_value(
            w,
            out,
            &S_SYNCHRONIZE_MODEL_METADATA_DOCUMENT_CONTEXT_WINDOW_OUT,
        )?;
        Ok(())
    })
}

static S_SYNCHRONIZE_MODEL_METADATA_DOCUMENT_LIMITS_OUT: wire::Schema =
    wire::Schema::OptJs(&wire::Schema::Str16);

/// `synchronizeModelMetadataDocumentLimits` over the binary wire.
#[wasm_bindgen(js_name = "synchronizeModelMetadataDocumentLimitsWire")]
pub fn synchronize_model_metadata_document_limits_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let source: String = r.string16()?;
        let target_slug: String = r.string16()?;
        let context_window: String = r.string16()?;
        let auto_compact_percent: String = r.string16()?;
        let out = &port::synchronize_model_metadata_document_limits(
            source,
            target_slug,
            context_window,
            auto_compact_percent,
        )?;
        wire::write_value(w, out, &S_SYNCHRONIZE_MODEL_METADATA_DOCUMENT_LIMITS_OUT)?;
        Ok(())
    })
}

static S_SYNCHRONIZE_MODEL_METADATA_DOCUMENT_LIMITS_PREVIEW_OUT: wire::Schema =
    wire::Schema::OptJs(&wire::Schema::Obj(&[
        ("document", wire::Schema::Str16),
        (
            "preview",
            wire::Schema::Obj(&[
                ("slug", wire::Schema::Str16),
                ("metadata", wire::Schema::Map16(&wire::Schema::Dyn)),
                ("documentEntry", wire::Schema::Map16(&wire::Schema::Dyn)),
                ("contextWindow", wire::Schema::OptJs(&wire::Schema::Str16)),
                (
                    "autoCompactPercent",
                    wire::Schema::OptJs(&wire::Schema::Str16),
                ),
                (
                    "autoCompactCalculationPercent",
                    wire::Schema::OptJs(&wire::Schema::Str16),
                ),
                ("ignoredFields", wire::Schema::List(&wire::Schema::Str16)),
            ]),
        ),
    ]));

/// `synchronizeModelMetadataDocumentLimitsPreview` over the binary wire.
#[wasm_bindgen(js_name = "synchronizeModelMetadataDocumentLimitsPreviewWire")]
pub fn synchronize_model_metadata_document_limits_preview_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let source: String = r.string16()?;
        let target_slug: String = r.string16()?;
        let context_window: String = r.string16()?;
        let auto_compact_percent: String = r.string16()?;
        let out = &port::synchronize_model_metadata_document_limits_preview(
            source,
            target_slug,
            context_window,
            auto_compact_percent,
        )?;
        match out {
            Some(x0) => {
                w.tag(1);
                wire::write_value(
                    w,
                    x0,
                    S_SYNCHRONIZE_MODEL_METADATA_DOCUMENT_LIMITS_PREVIEW_OUT.inner(),
                )?;
            }
            None => w.tag(2),
        }
        Ok(())
    })
}

static S_VALIDATE_MODEL_CAPABILITIES_0: wire::Schema = wire::Schema::Dyn;
static S_VALIDATE_MODEL_CAPABILITIES_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Str16);

/// `validateModelCapabilities` over the binary wire.
#[wasm_bindgen(js_name = "validateModelCapabilitiesWire")]
pub fn validate_model_capabilities_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let metadata: Value = wire::read_value(r, &S_VALIDATE_MODEL_CAPABILITIES_0)?;
        let out = &port::validate_model_capabilities(metadata)?;
        wire::write_value(w, out, &S_VALIDATE_MODEL_CAPABILITIES_OUT)?;
        Ok(())
    })
}

static S_PARSE_MODEL_METADATA_DOCUMENT_OUT: wire::Schema = wire::Schema::Dyn;

/// `parseModelMetadataDocument` over the binary wire.
#[wasm_bindgen(js_name = "parseModelMetadataDocumentWire")]
pub fn parse_model_metadata_document_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let source: String = r.string16()?;
        let target_slug: String = r.string16()?;
        let out = &port::parse_model_metadata_document(source, target_slug)?;
        wire::write_value(w, out, &S_PARSE_MODEL_METADATA_DOCUMENT_OUT)?;
        Ok(())
    })
}
