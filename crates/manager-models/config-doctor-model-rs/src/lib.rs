//! JS bindings for `src/config-doctor-model.ts`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;
pub mod utf16;

/// `findingLevelLabel`.
fn finding_level_label_inner(level: String) -> Result<String, String> {
    let out = port::finding_level_label(level)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "findingLevelLabel")]
pub fn finding_level_label(level: String) -> Result<String, JsError> {
    finding_level_label_inner(level).map_err(|e| JsError::new(&e))
}

/// `sortFindings`.
fn sort_findings_inner(findings: String) -> Result<String, String> {
    let findings = crate::js::from_json(&findings)?;
    let out = port::sort_findings(findings)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "sortFindings")]
pub fn sort_findings(findings: String) -> Result<String, JsError> {
    sort_findings_inner(findings).map_err(|e| JsError::new(&e))
}

/// `groupFindings`.
fn group_findings_inner(findings: String) -> Result<String, String> {
    let findings = crate::js::from_json(&findings)?;
    let out = port::group_findings(findings)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "groupFindings")]
pub fn group_findings(findings: String) -> Result<String, JsError> {
    group_findings_inner(findings).map_err(|e| JsError::new(&e))
}

/// `summarize`.
fn summarize_inner(findings: String) -> Result<String, String> {
    let findings = crate::js::from_json(&findings)?;
    let out = port::summarize(findings)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "summarize")]
pub fn summarize(findings: String) -> Result<String, JsError> {
    summarize_inner(findings).map_err(|e| JsError::new(&e))
}

/// `summaryTotal`.
fn summary_total_inner(summary: String) -> Result<f64, String> {
    let summary = crate::js::from_json(&summary)?;
    let out = port::summary_total(summary)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "summaryTotal")]
pub fn summary_total(summary: String) -> Result<f64, JsError> {
    summary_total_inner(summary).map_err(|e| JsError::new(&e))
}

/// `summaryLine`.
fn summary_line_inner(summary: String) -> Result<String, String> {
    let summary = crate::js::from_json(&summary)?;
    let out = port::summary_line(summary)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "summaryLine")]
pub fn summary_line(summary: String) -> Result<String, JsError> {
    summary_line_inner(summary).map_err(|e| JsError::new(&e))
}

/// `presetFields`.
fn preset_fields_inner(preset: String) -> Result<String, String> {
    let preset = crate::js::from_json(&preset)?;
    let out = port::preset_fields(preset)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "presetFields")]
pub fn preset_fields(preset: String) -> Result<String, JsError> {
    preset_fields_inner(preset).map_err(|e| JsError::new(&e))
}

/// `matchedPresetId`.
fn matched_preset_id_inner(presets: String, settings: String) -> Result<String, String> {
    let presets = crate::js::from_json(&presets)?;
    let settings = crate::js::from_json(&settings)?;
    let out = port::matched_preset_id(presets, settings)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "matchedPresetId")]
pub fn matched_preset_id(presets: String, settings: String) -> Result<String, JsError> {
    matched_preset_id_inner(presets, settings).map_err(|e| JsError::new(&e))
}

/// `normalizeSource`.
fn normalize_source_inner(value: Option<String>) -> Result<String, String> {
    let out = port::normalize_source(value)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "normalizeSource")]
pub fn normalize_source(value: Option<String>) -> Result<String, JsError> {
    normalize_source_inner(value).map_err(|e| JsError::new(&e))
}

/// `sourceLabel`.
fn source_label_inner(source: String) -> Result<String, String> {
    let out = port::source_label(source)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "sourceLabel")]
pub fn source_label(source: String) -> Result<String, JsError> {
    source_label_inner(source).map_err(|e| JsError::new(&e))
}

/// `sourceHint`.
fn source_hint_inner(source: String, sources: String) -> Result<String, String> {
    let sources = crate::js::from_json(&sources)?;
    let out = port::source_hint(source, sources)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "sourceHint")]
pub fn source_hint(source: String, sources: String) -> Result<String, JsError> {
    source_hint_inner(source, sources).map_err(|e| JsError::new(&e))
}

/// `sourceOptions`.
fn source_options_inner(sources: String) -> Result<String, String> {
    let sources = crate::js::from_json(&sources)?;
    let out = port::source_options(sources)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "sourceOptions")]
pub fn source_options(sources: String) -> Result<String, JsError> {
    source_options_inner(sources).map_err(|e| JsError::new(&e))
}

/// `currentSourceNote`.
fn current_source_note_inner(current: String) -> Result<String, String> {
    let current = crate::js::from_json(&current)?;
    let out = port::current_source_note(current)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "currentSourceNote")]
pub fn current_source_note(current: String) -> Result<String, JsError> {
    current_source_note_inner(current).map_err(|e| JsError::new(&e))
}

/// `localSourceNote`.
fn local_source_note_inner(local: String) -> Result<String, String> {
    let local = crate::js::from_json(&local)?;
    let out = port::local_source_note(local)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "localSourceNote")]
pub fn local_source_note(local: String) -> Result<String, JsError> {
    local_source_note_inner(local).map_err(|e| JsError::new(&e))
}

/// `ollamaDocsUrl`.
fn ollama_docs_url_inner(presets: String) -> Result<String, String> {
    let presets = crate::js::from_json(&presets)?;
    let out = port::ollama_docs_url(presets)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "ollamaDocsUrl")]
pub fn ollama_docs_url(presets: String) -> Result<String, JsError> {
    ollama_docs_url_inner(presets).map_err(|e| JsError::new(&e))
}

/// `draftFromSettings`.
fn draft_from_settings_inner(settings: String) -> Result<String, String> {
    let settings = crate::js::from_json(&settings)?;
    let out = port::draft_from_settings(settings)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "draftFromSettings")]
pub fn draft_from_settings(settings: String) -> Result<String, JsError> {
    draft_from_settings_inner(settings).map_err(|e| JsError::new(&e))
}

/// `savedPayload`.
fn saved_payload_inner(draft: String) -> Result<String, String> {
    let draft = crate::js::from_json(&draft)?;
    let out = port::saved_payload(draft)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "savedPayload")]
pub fn saved_payload(draft: String) -> Result<String, JsError> {
    saved_payload_inner(draft).map_err(|e| JsError::new(&e))
}

/// `aiReady`.
fn ai_ready_inner(settings: String) -> Result<bool, String> {
    let settings = crate::js::from_json(&settings)?;
    let out = port::ai_ready(settings)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "aiReady")]
pub fn ai_ready(settings: String) -> Result<bool, JsError> {
    ai_ready_inner(settings).map_err(|e| JsError::new(&e))
}

/// `aiBlockedHint`.
fn ai_blocked_hint_inner(settings: String) -> Result<String, String> {
    let settings = crate::js::from_json(&settings)?;
    let out = port::ai_blocked_hint(settings)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "aiBlockedHint")]
pub fn ai_blocked_hint(settings: String) -> Result<String, JsError> {
    ai_blocked_hint_inner(settings).map_err(|e| JsError::new(&e))
}

/// `aiStatusView`.
fn ai_status_view_inner(ai: String) -> Result<String, String> {
    let ai = crate::js::from_json(&ai)?;
    let out = port::ai_status_view(ai)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "aiStatusView")]
pub fn ai_status_view(ai: String) -> Result<String, JsError> {
    ai_status_view_inner(ai).map_err(|e| JsError::new(&e))
}

/// `formatElapsed`.
fn format_elapsed_inner(ms: Option<f64>) -> Result<String, String> {
    let out = port::format_elapsed(ms)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "formatElapsed")]
pub fn format_elapsed(ms: Option<f64>) -> Result<String, JsError> {
    format_elapsed_inner(ms).map_err(|e| JsError::new(&e))
}

/// `formatCheckedAt`.
fn format_checked_at_inner(value: Option<String>) -> Result<String, String> {
    let out = port::format_checked_at(value)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "formatCheckedAt")]
pub fn format_checked_at(value: Option<String>) -> Result<String, JsError> {
    format_checked_at_inner(value).map_err(|e| JsError::new(&e))
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

static S_SORT_FINDINGS_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("level", wire::Schema::Str16),
    ("title", wire::Schema::Str16),
    ("detail", wire::Schema::Str16),
    ("fix", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("scope", wire::Schema::OptJs(&wire::Schema::Str16)),
]));
static S_SORT_FINDINGS_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("level", wire::Schema::Str16),
    ("title", wire::Schema::Str16),
    ("detail", wire::Schema::Str16),
    ("fix", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("scope", wire::Schema::OptJs(&wire::Schema::Str16)),
]));

/// `sortFindings` over the binary wire.
#[wasm_bindgen(js_name = "sortFindingsWire")]
pub fn sort_findings_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let findings: Vec<Value> = r.list(|r| wire::read_value(r, S_SORT_FINDINGS_0.inner()))?;
        let out = &port::sort_findings(findings)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_SORT_FINDINGS_OUT.inner())?;
        }
        Ok(())
    })
}

static S_GROUP_FINDINGS_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("level", wire::Schema::Str16),
    ("title", wire::Schema::Str16),
    ("detail", wire::Schema::Str16),
    ("fix", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("scope", wire::Schema::OptJs(&wire::Schema::Str16)),
]));
static S_GROUP_FINDINGS_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("level", wire::Schema::Str16),
    ("label", wire::Schema::Str16),
    (
        "items",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("id", wire::Schema::Str16),
            ("level", wire::Schema::Str16),
            ("title", wire::Schema::Str16),
            ("detail", wire::Schema::Str16),
            ("fix", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("scope", wire::Schema::OptJs(&wire::Schema::Str16)),
        ])),
    ),
]));

/// `groupFindings` over the binary wire.
#[wasm_bindgen(js_name = "groupFindingsWire")]
pub fn group_findings_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let findings: Vec<Value> = r.list(|r| wire::read_value(r, S_GROUP_FINDINGS_0.inner()))?;
        let out = &port::group_findings(findings)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_GROUP_FINDINGS_OUT.inner())?;
        }
        Ok(())
    })
}

static S_SUMMARIZE_0: wire::Schema = wire::Schema::Dyn;
static S_SUMMARIZE_OUT: wire::Schema = wire::Schema::Obj(&[
    ("error", wire::Schema::Num),
    ("warn", wire::Schema::Num),
    ("info", wire::Schema::Num),
]);

/// `summarize` over the binary wire.
#[wasm_bindgen(js_name = "summarizeWire")]
pub fn summarize_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let findings: Value = wire::read_value(r, &S_SUMMARIZE_0)?;
        let out = &port::summarize(findings)?;
        wire::write_value(w, out, &S_SUMMARIZE_OUT)?;
        Ok(())
    })
}

static S_SUMMARY_TOTAL_0: wire::Schema = wire::Schema::Dyn;

/// `summaryTotal` over the binary wire.
#[wasm_bindgen(js_name = "summaryTotalWire")]
pub fn summary_total_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let summary: Value = wire::read_value(r, &S_SUMMARY_TOTAL_0)?;
        let out = &port::summary_total(summary)?;
        w.f64(*out);
        Ok(())
    })
}

static S_SUMMARY_LINE_0: wire::Schema = wire::Schema::Dyn;
static S_SUMMARY_LINE_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Obj(&[
    ("text", wire::Schema::Str16),
    ("args", wire::Schema::List(&wire::Schema::Str16)),
]));

/// `summaryLine` over the binary wire.
#[wasm_bindgen(js_name = "summaryLineWire")]
pub fn summary_line_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let summary: Value = wire::read_value(r, &S_SUMMARY_LINE_0)?;
        let out = &port::summary_line(summary)?;
        match out {
            Some(x0) => {
                w.tag(1);
                wire::write_value(w, x0, S_SUMMARY_LINE_OUT.inner())?;
            }
            None => w.tag(2),
        }
        Ok(())
    })
}

static S_PRESET_FIELDS_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Obj(&[
    ("baseUrl", wire::Schema::Str16),
    ("model", wire::Schema::Str16),
]));

/// `presetFields` over the binary wire.
#[wasm_bindgen(js_name = "presetFieldsWire")]
pub fn preset_fields_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let preset: Option<Value> = wire::read_dyn_opt(r)?;
        let out = &port::preset_fields(preset)?;
        match out {
            Some(x0) => {
                w.tag(1);
                wire::write_value(w, x0, S_PRESET_FIELDS_OUT.inner())?;
            }
            None => w.tag(2),
        }
        Ok(())
    })
}

static S_MATCHED_PRESET_ID_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("label", wire::Schema::Str16),
    ("baseUrl", wire::Schema::Str16),
    ("model", wire::Schema::Str16),
    ("needsKey", wire::Schema::Bool),
    ("note", wire::Schema::Str16),
    ("docsUrl", wire::Schema::Str16),
]));
static S_MATCHED_PRESET_ID_1: wire::Schema = wire::Schema::Dyn;

/// `matchedPresetId` over the binary wire.
#[wasm_bindgen(js_name = "matchedPresetIdWire")]
pub fn matched_preset_id_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let presets: Vec<Value> = r.list(|r| wire::read_value(r, S_MATCHED_PRESET_ID_0.inner()))?;
        let settings: Value = wire::read_value(r, &S_MATCHED_PRESET_ID_1)?;
        let out = &port::matched_preset_id(presets, settings)?;
        w.string16(out);
        Ok(())
    })
}

/// `sourceHint` over the binary wire.
#[wasm_bindgen(js_name = "sourceHintWire")]
pub fn source_hint_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let source: String = r.string16()?;
        let sources: Option<Value> = wire::read_dyn_opt(r)?;
        let out = &port::source_hint(source, sources)?;
        w.string16(out);
        Ok(())
    })
}

static S_SOURCE_OPTIONS_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("value", wire::Schema::Str16),
    ("label", wire::Schema::Str16),
    ("hint", wire::Schema::Str16),
]));

/// `sourceOptions` over the binary wire.
#[wasm_bindgen(js_name = "sourceOptionsWire")]
pub fn source_options_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let sources: Option<Value> = wire::read_dyn_opt(r)?;
        let out = &port::source_options(sources)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_SOURCE_OPTIONS_OUT.inner())?;
        }
        Ok(())
    })
}

static S_CURRENT_SOURCE_NOTE_OUT: wire::Schema = wire::Schema::Obj(&[
    ("label", wire::Schema::Str16),
    ("available", wire::Schema::Bool),
    ("hint", wire::Schema::OptJs(&wire::Schema::Str16)),
]);

/// `currentSourceNote` over the binary wire.
#[wasm_bindgen(js_name = "currentSourceNoteWire")]
pub fn current_source_note_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let current: Option<Value> = wire::read_dyn_opt(r)?;
        let out = &port::current_source_note(current)?;
        wire::write_value(w, out, &S_CURRENT_SOURCE_NOTE_OUT)?;
        Ok(())
    })
}

static S_LOCAL_SOURCE_NOTE_OUT: wire::Schema = wire::Schema::Obj(&[
    ("label", wire::Schema::Str16),
    ("found", wire::Schema::Bool),
]);

/// `localSourceNote` over the binary wire.
#[wasm_bindgen(js_name = "localSourceNoteWire")]
pub fn local_source_note_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let local: Option<Value> = wire::read_dyn_opt(r)?;
        let out = &port::local_source_note(local)?;
        wire::write_value(w, out, &S_LOCAL_SOURCE_NOTE_OUT)?;
        Ok(())
    })
}

static S_OLLAMA_DOCS_URL_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("label", wire::Schema::Str16),
    ("baseUrl", wire::Schema::Str16),
    ("model", wire::Schema::Str16),
    ("needsKey", wire::Schema::Bool),
    ("note", wire::Schema::Str16),
    ("docsUrl", wire::Schema::Str16),
]));

/// `ollamaDocsUrl` over the binary wire.
#[wasm_bindgen(js_name = "ollamaDocsUrlWire")]
pub fn ollama_docs_url_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let presets: Vec<Value> = r.list(|r| wire::read_value(r, S_OLLAMA_DOCS_URL_0.inner()))?;
        let out = &port::ollama_docs_url(presets)?;
        w.string16(out);
        Ok(())
    })
}

static S_DRAFT_FROM_SETTINGS_0: wire::Schema = wire::Schema::Dyn;
static S_DRAFT_FROM_SETTINGS_OUT: wire::Schema = wire::Schema::Obj(&[
    ("enabled", wire::Schema::Bool),
    ("source", wire::Schema::Str16),
    ("baseUrl", wire::Schema::Str16),
    ("model", wire::Schema::Str16),
    ("apiKey", wire::Schema::Str16),
]);

/// `draftFromSettings` over the binary wire.
#[wasm_bindgen(js_name = "draftFromSettingsWire")]
pub fn draft_from_settings_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let settings: Value = wire::read_value(r, &S_DRAFT_FROM_SETTINGS_0)?;
        let out = &port::draft_from_settings(settings)?;
        wire::write_value(w, out, &S_DRAFT_FROM_SETTINGS_OUT)?;
        Ok(())
    })
}

static S_SAVED_PAYLOAD_0: wire::Schema = wire::Schema::Dyn;
static S_SAVED_PAYLOAD_OUT: wire::Schema = wire::Schema::Obj(&[
    ("enabled", wire::Schema::Bool),
    ("source", wire::Schema::Str16),
    ("baseUrl", wire::Schema::Str16),
    ("model", wire::Schema::Str16),
    ("apiKey", wire::Schema::OptJs(&wire::Schema::Str16)),
]);

/// `savedPayload` over the binary wire.
#[wasm_bindgen(js_name = "savedPayloadWire")]
pub fn saved_payload_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let draft: Value = wire::read_value(r, &S_SAVED_PAYLOAD_0)?;
        let out = &port::saved_payload(draft)?;
        wire::write_value(w, out, &S_SAVED_PAYLOAD_OUT)?;
        Ok(())
    })
}

static S_AI_READY_0: wire::Schema = wire::Schema::Dyn;

/// `aiReady` over the binary wire.
#[wasm_bindgen(js_name = "aiReadyWire")]
pub fn ai_ready_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let settings: Value = wire::read_value(r, &S_AI_READY_0)?;
        let out = &port::ai_ready(settings)?;
        w.bool(*out);
        Ok(())
    })
}

static S_AI_BLOCKED_HINT_0: wire::Schema = wire::Schema::Dyn;
static S_AI_BLOCKED_HINT_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Obj(&[
    ("text", wire::Schema::Str16),
    ("args", wire::Schema::List(&wire::Schema::Str16)),
]));

/// `aiBlockedHint` over the binary wire.
#[wasm_bindgen(js_name = "aiBlockedHintWire")]
pub fn ai_blocked_hint_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let settings: Value = wire::read_value(r, &S_AI_BLOCKED_HINT_0)?;
        let out = &port::ai_blocked_hint(settings)?;
        match out {
            Some(x0) => {
                w.tag(1);
                wire::write_value(w, x0, S_AI_BLOCKED_HINT_OUT.inner())?;
            }
            None => w.tag(2),
        }
        Ok(())
    })
}

static S_AI_STATUS_VIEW_0: wire::Schema = wire::Schema::Dyn;
static S_AI_STATUS_VIEW_OUT: wire::Schema = wire::Schema::Obj(&[
    ("tone", wire::Schema::Str16),
    ("label", wire::Schema::Str16),
    ("detail", wire::Schema::Str16),
    ("facts", wire::Schema::List(&wire::Schema::Str16)),
]);

/// `aiStatusView` over the binary wire.
#[wasm_bindgen(js_name = "aiStatusViewWire")]
pub fn ai_status_view_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let ai: Value = wire::read_value(r, &S_AI_STATUS_VIEW_0)?;
        let out = &port::ai_status_view(ai)?;
        wire::write_value(w, out, &S_AI_STATUS_VIEW_OUT)?;
        Ok(())
    })
}
