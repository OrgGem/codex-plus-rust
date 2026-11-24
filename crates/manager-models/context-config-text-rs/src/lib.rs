//! JS bindings for `src/context-config-text.ts`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;
pub mod utf16;

/// `unquoteTomlKey`.
fn unquote_toml_key_inner(key: String) -> Result<String, String> {
    let out = port::unquote_toml_key(key)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "unquoteTomlKey")]
pub fn unquote_toml_key(key: String) -> Result<String, JsError> {
    unquote_toml_key_inner(key).map_err(|e| JsError::new(&e))
}

/// `tomlTablePathFromLine`.
fn toml_table_path_from_line_inner(line: String) -> Result<String, String> {
    let out = port::toml_table_path_from_line(line)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "tomlTablePathFromLine")]
pub fn toml_table_path_from_line(line: String) -> Result<String, JsError> {
    toml_table_path_from_line_inner(line).map_err(|e| JsError::new(&e))
}

/// `parseTomlDottedPath`.
fn parse_toml_dotted_path_inner(path: String) -> Result<String, String> {
    let out = port::parse_toml_dotted_path(path)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "parseTomlDottedPath")]
pub fn parse_toml_dotted_path(path: String) -> Result<String, JsError> {
    parse_toml_dotted_path_inner(path).map_err(|e| JsError::new(&e))
}

/// `contextEntrySummary`.
fn context_entry_summary_inner(toml_body: String) -> Result<String, String> {
    let out = port::context_entry_summary(toml_body)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "contextEntrySummary")]
pub fn context_entry_summary(toml_body: String) -> Result<String, JsError> {
    context_entry_summary_inner(toml_body).map_err(|e| JsError::new(&e))
}

/// `contextEntryEnabled`.
fn context_entry_enabled_inner(toml_body: String) -> Result<String, String> {
    let out = port::context_entry_enabled(toml_body)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "contextEntryEnabled")]
pub fn context_entry_enabled(toml_body: String) -> Result<String, JsError> {
    context_entry_enabled_inner(toml_body).map_err(|e| JsError::new(&e))
}

/// `setContextEntryEnabled`.
fn set_context_entry_enabled_inner(toml_body: String, enabled: bool) -> Result<String, String> {
    let out = port::set_context_entry_enabled(toml_body, enabled)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "setContextEntryEnabled")]
pub fn set_context_entry_enabled(toml_body: String, enabled: bool) -> Result<String, JsError> {
    set_context_entry_enabled_inner(toml_body, enabled).map_err(|e| JsError::new(&e))
}

/// `contextHeaderFromLine`.
fn context_header_from_line_inner(line: String) -> Result<String, String> {
    let out = port::context_header_from_line(line)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "contextHeaderFromLine")]
pub fn context_header_from_line(line: String) -> Result<String, JsError> {
    context_header_from_line_inner(line).map_err(|e| JsError::new(&e))
}

/// `parseContextEntries`.
fn parse_context_entries_inner(
    common_config: String,
    kind: String,
    table_name: String,
) -> Result<String, String> {
    let out = port::parse_context_entries(common_config, kind, table_name)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "parseContextEntries")]
pub fn parse_context_entries(
    common_config: String,
    kind: String,
    table_name: String,
) -> Result<String, JsError> {
    parse_context_entries_inner(common_config, kind, table_name).map_err(|e| JsError::new(&e))
}

/// `dedupeContextEntryList`.
fn dedupe_context_entry_list_inner(entries: String) -> Result<String, String> {
    let entries = crate::js::from_json(&entries)?;
    let out = port::dedupe_context_entry_list(entries)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "dedupeContextEntryList")]
pub fn dedupe_context_entry_list(entries: String) -> Result<String, JsError> {
    dedupe_context_entry_list_inner(entries).map_err(|e| JsError::new(&e))
}

/// `contextEntriesByKind`.
fn context_entries_by_kind_inner(entries: String, kind: String) -> Result<String, String> {
    let entries = crate::js::from_json(&entries)?;
    let out = port::context_entries_by_kind(entries, kind)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "contextEntriesByKind")]
pub fn context_entries_by_kind(entries: String, kind: String) -> Result<String, JsError> {
    context_entries_by_kind_inner(entries, kind).map_err(|e| JsError::new(&e))
}

/// `contextEntryToTomlSection`.
fn context_entry_to_toml_section_inner(
    table_name: String,
    entry: String,
) -> Result<String, String> {
    let entry = crate::js::from_json(&entry)?;
    let out = port::context_entry_to_toml_section(table_name, entry)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "contextEntryToTomlSection")]
pub fn context_entry_to_toml_section(table_name: String, entry: String) -> Result<String, JsError> {
    context_entry_to_toml_section_inner(table_name, entry).map_err(|e| JsError::new(&e))
}

/// `allContextConfigToml`.
fn all_context_config_toml_inner(entries: String) -> Result<String, String> {
    let entries = crate::js::from_json(&entries)?;
    let out = port::all_context_config_toml(entries)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "allContextConfigToml")]
pub fn all_context_config_toml(entries: String) -> Result<String, JsError> {
    all_context_config_toml_inner(entries).map_err(|e| JsError::new(&e))
}

/// `selectedContextConfigToml`.
fn selected_context_config_toml_inner(entries: String) -> Result<String, String> {
    let entries = crate::js::from_json(&entries)?;
    let out = port::selected_context_config_toml(entries)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "selectedContextConfigToml")]
pub fn selected_context_config_toml(entries: String) -> Result<String, JsError> {
    selected_context_config_toml_inner(entries).map_err(|e| JsError::new(&e))
}

/// `contextEntriesFromConfig`.
fn context_entries_from_config_inner(config_contents: String) -> Result<String, String> {
    let out = port::context_entries_from_config(config_contents)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "contextEntriesFromConfig")]
pub fn context_entries_from_config(config_contents: String) -> Result<String, JsError> {
    context_entries_from_config_inner(config_contents).map_err(|e| JsError::new(&e))
}

/// `splitContextConfigText`.
fn split_context_config_text_inner(config_contents: String) -> Result<String, String> {
    let out = port::split_context_config_text(config_contents)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "splitContextConfigText")]
pub fn split_context_config_text(config_contents: String) -> Result<String, JsError> {
    split_context_config_text_inner(config_contents).map_err(|e| JsError::new(&e))
}

/// `stripContextEntriesFromConfig`.
fn strip_context_entries_from_config_inner(
    config_contents: String,
    entries: String,
) -> Result<String, String> {
    let entries = crate::js::from_json(&entries)?;
    let out = port::strip_context_entries_from_config(config_contents, entries)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "stripContextEntriesFromConfig")]
pub fn strip_context_entries_from_config(
    config_contents: String,
    entries: String,
) -> Result<String, JsError> {
    strip_context_entries_from_config_inner(config_contents, entries).map_err(|e| JsError::new(&e))
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

static S_UNQUOTE_TOML_KEY_OUT: wire::Schema = wire::Schema::Dyn;

/// `unquoteTomlKey` over the binary wire.
#[wasm_bindgen(js_name = "unquoteTomlKeyWire")]
pub fn unquote_toml_key_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let key: String = r.string16()?;
        let out = &port::unquote_toml_key(key)?;
        wire::write_value(w, out, &S_UNQUOTE_TOML_KEY_OUT)?;
        Ok(())
    })
}

/// `tomlTablePathFromLine` over the binary wire.
#[wasm_bindgen(js_name = "tomlTablePathFromLineWire")]
pub fn toml_table_path_from_line_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let line: String = r.string16()?;
        let out = &port::toml_table_path_from_line(line)?;
        match out {
            Some(x0) => {
                w.tag(1);
                w.tag(1);
                w.count(x0.len());
                for x1 in x0.iter() {
                    w.string16(x1);
                }
            }
            None => w.tag(2),
        }
        Ok(())
    })
}

/// `parseTomlDottedPath` over the binary wire.
#[wasm_bindgen(js_name = "parseTomlDottedPathWire")]
pub fn parse_toml_dotted_path_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let path: String = r.string16()?;
        let out = &port::parse_toml_dotted_path(path)?;
        match out {
            Some(x0) => {
                w.tag(1);
                w.tag(1);
                w.count(x0.len());
                for x1 in x0.iter() {
                    w.string16(x1);
                }
            }
            None => w.tag(2),
        }
        Ok(())
    })
}

static S_CONTEXT_ENTRY_SUMMARY_OUT: wire::Schema = wire::Schema::Dyn;

/// `contextEntrySummary` over the binary wire.
#[wasm_bindgen(js_name = "contextEntrySummaryWire")]
pub fn context_entry_summary_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let toml_body: String = r.string16()?;
        let out = &port::context_entry_summary(toml_body)?;
        wire::write_value(w, out, &S_CONTEXT_ENTRY_SUMMARY_OUT)?;
        Ok(())
    })
}

static S_CONTEXT_ENTRY_ENABLED_OUT: wire::Schema = wire::Schema::Dyn;

/// `contextEntryEnabled` over the binary wire.
#[wasm_bindgen(js_name = "contextEntryEnabledWire")]
pub fn context_entry_enabled_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let toml_body: String = r.string16()?;
        let out = &port::context_entry_enabled(toml_body)?;
        wire::write_value(w, out, &S_CONTEXT_ENTRY_ENABLED_OUT)?;
        Ok(())
    })
}

static S_SET_CONTEXT_ENTRY_ENABLED_OUT: wire::Schema = wire::Schema::Dyn;

/// `setContextEntryEnabled` over the binary wire.
#[wasm_bindgen(js_name = "setContextEntryEnabledWire")]
pub fn set_context_entry_enabled_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let toml_body: String = r.string16()?;
        let enabled: bool = r.bool()?;
        let out = &port::set_context_entry_enabled(toml_body, enabled)?;
        wire::write_value(w, out, &S_SET_CONTEXT_ENTRY_ENABLED_OUT)?;
        Ok(())
    })
}

static S_CONTEXT_HEADER_FROM_LINE_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Obj(&[
    ("kind", wire::Schema::Str16),
    ("id", wire::Schema::Str16),
]));

/// `contextHeaderFromLine` over the binary wire.
#[wasm_bindgen(js_name = "contextHeaderFromLineWire")]
pub fn context_header_from_line_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let line: String = r.string16()?;
        let out = &port::context_header_from_line(line)?;
        match out {
            Some(x0) => {
                w.tag(1);
                wire::write_value(w, x0, S_CONTEXT_HEADER_FROM_LINE_OUT.inner())?;
            }
            None => w.tag(2),
        }
        Ok(())
    })
}

static S_PARSE_CONTEXT_ENTRIES_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("kind", wire::Schema::Str16),
    ("title", wire::Schema::Str16),
    ("summary", wire::Schema::Str16),
    ("tomlBody", wire::Schema::Str16),
    ("enabled", wire::Schema::Bool),
]));

/// `parseContextEntries` over the binary wire.
#[wasm_bindgen(js_name = "parseContextEntriesWire")]
pub fn parse_context_entries_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let common_config: String = r.string16()?;
        let kind: String = r.string16()?;
        let table_name: String = r.string16()?;
        let out = &port::parse_context_entries(common_config, kind, table_name)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_PARSE_CONTEXT_ENTRIES_OUT.inner())?;
        }
        Ok(())
    })
}

static S_DEDUPE_CONTEXT_ENTRY_LIST_0: wire::Schema = wire::Schema::Dyn;
static S_DEDUPE_CONTEXT_ENTRY_LIST_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("kind", wire::Schema::Str16),
    ("title", wire::Schema::Str16),
    ("summary", wire::Schema::Str16),
    ("tomlBody", wire::Schema::Str16),
    ("enabled", wire::Schema::Bool),
]));

/// `dedupeContextEntryList` over the binary wire.
#[wasm_bindgen(js_name = "dedupeContextEntryListWire")]
pub fn dedupe_context_entry_list_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let entries: Value = wire::read_value(r, &S_DEDUPE_CONTEXT_ENTRY_LIST_0)?;
        let out = &port::dedupe_context_entry_list(entries)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_DEDUPE_CONTEXT_ENTRY_LIST_OUT.inner())?;
        }
        Ok(())
    })
}

static S_CONTEXT_ENTRIES_BY_KIND_0: wire::Schema = wire::Schema::Dyn;
static S_CONTEXT_ENTRIES_BY_KIND_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("kind", wire::Schema::Str16),
    ("title", wire::Schema::Str16),
    ("summary", wire::Schema::Str16),
    ("tomlBody", wire::Schema::Str16),
    ("enabled", wire::Schema::Bool),
]));

/// `contextEntriesByKind` over the binary wire.
#[wasm_bindgen(js_name = "contextEntriesByKindWire")]
pub fn context_entries_by_kind_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let entries: Value = wire::read_value(r, &S_CONTEXT_ENTRIES_BY_KIND_0)?;
        let kind: String = r.string16()?;
        let out = &port::context_entries_by_kind(entries, kind)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_CONTEXT_ENTRIES_BY_KIND_OUT.inner())?;
        }
        Ok(())
    })
}

static S_CONTEXT_ENTRY_TO_TOML_SECTION_1: wire::Schema = wire::Schema::Dyn;

/// `contextEntryToTomlSection` over the binary wire.
#[wasm_bindgen(js_name = "contextEntryToTomlSectionWire")]
pub fn context_entry_to_toml_section_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let table_name: String = r.string16()?;
        let entry: Value = wire::read_value(r, &S_CONTEXT_ENTRY_TO_TOML_SECTION_1)?;
        let out = &port::context_entry_to_toml_section(table_name, entry)?;
        w.string16(out);
        Ok(())
    })
}

static S_ALL_CONTEXT_CONFIG_TOML_0: wire::Schema = wire::Schema::Dyn;

/// `allContextConfigToml` over the binary wire.
#[wasm_bindgen(js_name = "allContextConfigTomlWire")]
pub fn all_context_config_toml_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let entries: Value = wire::read_value(r, &S_ALL_CONTEXT_CONFIG_TOML_0)?;
        let out = &port::all_context_config_toml(entries)?;
        w.string16(out);
        Ok(())
    })
}

static S_SELECTED_CONTEXT_CONFIG_TOML_0: wire::Schema = wire::Schema::Dyn;

/// `selectedContextConfigToml` over the binary wire.
#[wasm_bindgen(js_name = "selectedContextConfigTomlWire")]
pub fn selected_context_config_toml_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let entries: Value = wire::read_value(r, &S_SELECTED_CONTEXT_CONFIG_TOML_0)?;
        let out = &port::selected_context_config_toml(entries)?;
        w.string16(out);
        Ok(())
    })
}

static S_CONTEXT_ENTRIES_FROM_CONFIG_OUT: wire::Schema = wire::Schema::Obj(&[
    (
        "mcpServers",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("id", wire::Schema::Str16),
            ("kind", wire::Schema::Str16),
            ("title", wire::Schema::Str16),
            ("summary", wire::Schema::Str16),
            ("tomlBody", wire::Schema::Str16),
            ("enabled", wire::Schema::Bool),
        ])),
    ),
    (
        "skills",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("id", wire::Schema::Str16),
            ("kind", wire::Schema::Str16),
            ("title", wire::Schema::Str16),
            ("summary", wire::Schema::Str16),
            ("tomlBody", wire::Schema::Str16),
            ("enabled", wire::Schema::Bool),
        ])),
    ),
    (
        "plugins",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("id", wire::Schema::Str16),
            ("kind", wire::Schema::Str16),
            ("title", wire::Schema::Str16),
            ("summary", wire::Schema::Str16),
            ("tomlBody", wire::Schema::Str16),
            ("enabled", wire::Schema::Bool),
        ])),
    ),
]);

/// `contextEntriesFromConfig` over the binary wire.
#[wasm_bindgen(js_name = "contextEntriesFromConfigWire")]
pub fn context_entries_from_config_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let config_contents: String = r.string16()?;
        let out = &port::context_entries_from_config(config_contents)?;
        wire::write_value(w, out, &S_CONTEXT_ENTRIES_FROM_CONFIG_OUT)?;
        Ok(())
    })
}

static S_SPLIT_CONTEXT_CONFIG_TEXT_OUT: wire::Schema = wire::Schema::Obj(&[
    ("common", wire::Schema::Str16),
    ("context", wire::Schema::Str16),
]);

/// `splitContextConfigText` over the binary wire.
#[wasm_bindgen(js_name = "splitContextConfigTextWire")]
pub fn split_context_config_text_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let config_contents: String = r.string16()?;
        let out = &port::split_context_config_text(config_contents)?;
        wire::write_value(w, out, &S_SPLIT_CONTEXT_CONFIG_TEXT_OUT)?;
        Ok(())
    })
}

static S_STRIP_CONTEXT_ENTRIES_FROM_CONFIG_1: wire::Schema = wire::Schema::Dyn;

/// `stripContextEntriesFromConfig` over the binary wire.
#[wasm_bindgen(js_name = "stripContextEntriesFromConfigWire")]
pub fn strip_context_entries_from_config_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let config_contents: String = r.string16()?;
        let entries: Value = wire::read_value(r, &S_STRIP_CONTEXT_ENTRIES_FROM_CONFIG_1)?;
        let out = &port::strip_context_entries_from_config(config_contents, entries)?;
        w.string16(out);
        Ok(())
    })
}
