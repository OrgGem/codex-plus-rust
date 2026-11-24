//! JS bindings for `src/log-view-model.ts`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;
pub mod utf16;

/// `summarizeDetail`.
fn summarize_detail_inner(detail: String) -> Result<String, String> {
    let detail = crate::js::from_json(&detail)?;
    let out = port::summarize_detail(detail)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "summarizeDetail")]
pub fn summarize_detail(detail: String) -> Result<String, JsError> {
    summarize_detail_inner(detail).map_err(|e| JsError::new(&e))
}

/// `logTone`.
fn log_tone_inner(event: String, detail: String) -> Result<String, String> {
    let detail = crate::js::from_json(&detail)?;
    let out = port::log_tone(event, detail)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "logTone")]
pub fn log_tone(event: String, detail: String) -> Result<String, JsError> {
    log_tone_inner(event, detail).map_err(|e| JsError::new(&e))
}

/// `parseLogLine`.
fn parse_log_line_inner(raw: String, index: f64) -> Result<String, String> {
    let out = port::parse_log_line(raw, index)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "parseLogLine")]
pub fn parse_log_line(raw: String, index: f64) -> Result<String, JsError> {
    parse_log_line_inner(raw, index).map_err(|e| JsError::new(&e))
}

/// `parseLogText`.
fn parse_log_text_inner(text: String, truncated: Option<bool>) -> Result<String, String> {
    let out = port::parse_log_text(text, truncated)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "parseLogText")]
pub fn parse_log_text(text: String, truncated: Option<bool>) -> Result<String, JsError> {
    parse_log_text_inner(text, truncated).map_err(|e| JsError::new(&e))
}

/// `filterLogEntries`.
fn filter_log_entries_inner(
    entries: String,
    query: String,
    errors_only: bool,
) -> Result<String, String> {
    let entries = crate::js::from_json(&entries)?;
    let out = port::filter_log_entries(entries, query, errors_only)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "filterLogEntries")]
pub fn filter_log_entries(
    entries: String,
    query: String,
    errors_only: bool,
) -> Result<String, JsError> {
    filter_log_entries_inner(entries, query, errors_only).map_err(|e| JsError::new(&e))
}

/// `formatLogTime`.
fn format_log_time_inner(timestamp_ms: Option<f64>, now: String) -> Result<String, String> {
    let now = crate::js::from_json(&now)?;
    let out = port::format_log_time(timestamp_ms, now)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "formatLogTime")]
pub fn format_log_time(timestamp_ms: Option<f64>, now: String) -> Result<String, JsError> {
    format_log_time_inner(timestamp_ms, now).map_err(|e| JsError::new(&e))
}

/// `diagnosticStatusText`.
fn diagnostic_status_text_inner(status: String) -> Result<String, String> {
    let out = port::diagnostic_status_text(status)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "diagnosticStatusText")]
pub fn diagnostic_status_text(status: String) -> Result<String, JsError> {
    diagnostic_status_text_inner(status).map_err(|e| JsError::new(&e))
}

/// `diagnosticFacts`.
fn diagnostic_facts_inner(report: String) -> Result<String, String> {
    let out = port::diagnostic_facts(report)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "diagnosticFacts")]
pub fn diagnostic_facts(report: String) -> Result<String, JsError> {
    diagnostic_facts_inner(report).map_err(|e| JsError::new(&e))
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

static S_SUMMARIZE_DETAIL_0: wire::Schema = wire::Schema::Dyn;

/// `summarizeDetail` over the binary wire.
#[wasm_bindgen(js_name = "summarizeDetailWire")]
pub fn summarize_detail_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let detail: Value = wire::read_value(r, &S_SUMMARIZE_DETAIL_0)?;
        let out = &port::summarize_detail(detail)?;
        w.string16(out);
        Ok(())
    })
}

static S_LOG_TONE_1: wire::Schema = wire::Schema::Dyn;

/// `logTone` over the binary wire.
#[wasm_bindgen(js_name = "logToneWire")]
pub fn log_tone_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let event: String = r.string16()?;
        let detail: Value = wire::read_value(r, &S_LOG_TONE_1)?;
        let out = &port::log_tone(event, detail)?;
        w.string16(out);
        Ok(())
    })
}

static S_PARSE_LOG_LINE_OUT: wire::Schema = wire::Schema::Obj(&[
    ("index", wire::Schema::Num),
    ("timestampMs", wire::Schema::OptJs(&wire::Schema::Num)),
    ("pid", wire::Schema::OptJs(&wire::Schema::Num)),
    ("event", wire::Schema::Str16),
    ("tone", wire::Schema::Str16),
    ("summary", wire::Schema::Str16),
    ("body", wire::Schema::Str16),
    ("raw", wire::Schema::Str16),
]);

/// `parseLogLine` over the binary wire.
#[wasm_bindgen(js_name = "parseLogLineWire")]
pub fn parse_log_line_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let raw: String = r.string16()?;
        let index: f64 = r.f64()?;
        let out = &port::parse_log_line(raw, index)?;
        wire::write_value(w, out, &S_PARSE_LOG_LINE_OUT)?;
        Ok(())
    })
}

static S_PARSE_LOG_TEXT_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("index", wire::Schema::Num),
    ("timestampMs", wire::Schema::OptJs(&wire::Schema::Num)),
    ("pid", wire::Schema::OptJs(&wire::Schema::Num)),
    ("event", wire::Schema::Str16),
    ("tone", wire::Schema::Str16),
    ("summary", wire::Schema::Str16),
    ("body", wire::Schema::Str16),
    ("raw", wire::Schema::Str16),
]));

/// `parseLogText` over the binary wire.
#[wasm_bindgen(js_name = "parseLogTextWire")]
pub fn parse_log_text_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let text: String = r.string16()?;
        let truncated: Option<bool> = r.opt_js(|r| r.bool())?;
        let out = &port::parse_log_text(text, truncated)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_PARSE_LOG_TEXT_OUT.inner())?;
        }
        Ok(())
    })
}

static S_FILTER_LOG_ENTRIES_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("index", wire::Schema::Num),
    ("timestampMs", wire::Schema::OptJs(&wire::Schema::Num)),
    ("pid", wire::Schema::OptJs(&wire::Schema::Num)),
    ("event", wire::Schema::Str16),
    ("tone", wire::Schema::Str16),
    ("summary", wire::Schema::Str16),
    ("body", wire::Schema::Str16),
    ("raw", wire::Schema::Str16),
]));
static S_FILTER_LOG_ENTRIES_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("index", wire::Schema::Num),
    ("timestampMs", wire::Schema::OptJs(&wire::Schema::Num)),
    ("pid", wire::Schema::OptJs(&wire::Schema::Num)),
    ("event", wire::Schema::Str16),
    ("tone", wire::Schema::Str16),
    ("summary", wire::Schema::Str16),
    ("body", wire::Schema::Str16),
    ("raw", wire::Schema::Str16),
]));

/// `filterLogEntries` over the binary wire.
#[wasm_bindgen(js_name = "filterLogEntriesWire")]
pub fn filter_log_entries_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let entries: Vec<Value> =
            r.list(|r| wire::read_value(r, S_FILTER_LOG_ENTRIES_0.inner()))?;
        let query: String = r.string16()?;
        let errors_only: bool = r.bool()?;
        let out = &port::filter_log_entries(entries, query, errors_only)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_FILTER_LOG_ENTRIES_OUT.inner())?;
        }
        Ok(())
    })
}

static S_FORMAT_LOG_TIME_1: wire::Schema = wire::Schema::Dyn;

/// `formatLogTime` over the binary wire.
#[wasm_bindgen(js_name = "formatLogTimeWire")]
pub fn format_log_time_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let timestamp_ms: Option<f64> = r.opt_js(|r| r.f64())?;
        let now: Value = wire::read_value(r, &S_FORMAT_LOG_TIME_1)?;
        let out = &port::format_log_time(timestamp_ms, now)?;
        w.string16(out);
        Ok(())
    })
}

static S_DIAGNOSTIC_FACTS_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("label", wire::Schema::Str16),
    ("status", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("value", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("tone", wire::Schema::OptJs(&wire::Schema::Str16)),
]));

/// `diagnosticFacts` over the binary wire.
#[wasm_bindgen(js_name = "diagnosticFactsWire")]
pub fn diagnostic_facts_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let report: String = r.string16()?;
        let out = &port::diagnostic_facts(report)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_DIAGNOSTIC_FACTS_OUT.inner())?;
        }
        Ok(())
    })
}
