//! JS bindings for `src/management-model.ts`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;
pub mod utf16;

/// `matchesManagementQuery`.
fn matches_management_query_inner(values: Vec<String>, query: String) -> Result<bool, String> {
    let out = port::matches_management_query(values, query)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "matchesManagementQuery")]
pub fn matches_management_query(values: Vec<String>, query: String) -> Result<bool, JsError> {
    matches_management_query_inner(values, query).map_err(|e| JsError::new(&e))
}

/// `normalizeProjectPath`.
fn normalize_project_path_inner(cwd: String) -> Result<String, String> {
    let out = port::normalize_project_path(cwd)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "normalizeProjectPath")]
pub fn normalize_project_path(cwd: String) -> Result<String, JsError> {
    normalize_project_path_inner(cwd).map_err(|e| JsError::new(&e))
}

/// `sessionGroups`.
fn session_groups_inner(sessions: String, mode: String, now: String) -> Result<String, String> {
    let sessions = crate::js::from_json(&sessions)?;
    let now = crate::js::from_json(&now)?;
    let out = port::session_groups(sessions, mode, now)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "sessionGroups")]
pub fn session_groups(sessions: String, mode: String, now: String) -> Result<String, JsError> {
    session_groups_inner(sessions, mode, now).map_err(|e| JsError::new(&e))
}

/// `skillRepoKey`.
fn skill_repo_key_inner(repo: String) -> Result<String, String> {
    let repo = crate::js::from_json(&repo)?;
    let out = port::skill_repo_key(repo)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "skillRepoKey")]
pub fn skill_repo_key(repo: String) -> Result<String, JsError> {
    skill_repo_key_inner(repo).map_err(|e| JsError::new(&e))
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

static S_SESSION_GROUPS_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("title", wire::Schema::Str16),
    ("cwd", wire::Schema::Str16),
    ("modelProvider", wire::Schema::Str16),
    ("archived", wire::Schema::Bool),
    ("updatedAtMs", wire::Schema::OptJs(&wire::Schema::Num)),
    ("rolloutPath", wire::Schema::Str16),
    ("dbPath", wire::Schema::Str16),
]));
static S_SESSION_GROUPS_2: wire::Schema = wire::Schema::Dyn;
static S_SESSION_GROUPS_OUT: wire::Schema = wire::Schema::Dyn;

/// `sessionGroups` over the binary wire.
#[wasm_bindgen(js_name = "sessionGroupsWire")]
pub fn session_groups_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let sessions: Vec<Value> = r.list(|r| wire::read_value(r, S_SESSION_GROUPS_0.inner()))?;
        let mode: String = r.string16()?;
        let now: Value = wire::read_value(r, &S_SESSION_GROUPS_2)?;
        let out = &port::session_groups(sessions, mode, now)?;
        wire::write_value(w, out, &S_SESSION_GROUPS_OUT)?;
        Ok(())
    })
}

static S_SKILL_REPO_KEY_0: wire::Schema = wire::Schema::Dyn;

/// `skillRepoKey` over the binary wire.
#[wasm_bindgen(js_name = "skillRepoKeyWire")]
pub fn skill_repo_key_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let repo: Value = wire::read_value(r, &S_SKILL_REPO_KEY_0)?;
        let out = &port::skill_repo_key(repo)?;
        w.string16(out);
        Ok(())
    })
}
