//! JS bindings for `src/skill-agent-batch.ts`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;
pub mod utf16;

/// `skillAgents`.
fn skill_agents_inner(skill: String) -> Result<Vec<String>, String> {
    let skill = crate::js::from_json(&skill)?;
    let out = port::skill_agents(skill)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "skillAgents")]
pub fn skill_agents(skill: String) -> Result<Vec<String>, JsError> {
    skill_agents_inner(skill).map_err(|e| JsError::new(&e))
}

/// `skillCodexStatusError`.
fn skill_codex_status_error_inner(skill: String) -> Result<Option<String>, String> {
    let skill = crate::js::from_json(&skill)?;
    let out = port::skill_codex_status_error(skill)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "skillCodexStatusError")]
pub fn skill_codex_status_error(skill: String) -> Result<Option<String>, JsError> {
    skill_codex_status_error_inner(skill).map_err(|e| JsError::new(&e))
}

/// `skillBatchTargets`.
fn skill_batch_targets_inner(
    skills: String,
    agent: String,
    enabled: bool,
) -> Result<String, String> {
    let skills = crate::js::from_json(&skills)?;
    let out = port::skill_batch_targets(skills, agent, enabled)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "skillBatchTargets")]
pub fn skill_batch_targets(
    skills: String,
    agent: String,
    enabled: bool,
) -> Result<String, JsError> {
    skill_batch_targets_inner(skills, agent, enabled).map_err(|e| JsError::new(&e))
}

/// `skillBatchState`.
fn skill_batch_state_inner(skills: String, agent: String) -> Result<String, String> {
    let skills = crate::js::from_json(&skills)?;
    let out = port::skill_batch_state(skills, agent)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "skillBatchState")]
pub fn skill_batch_state(skills: String, agent: String) -> Result<String, JsError> {
    skill_batch_state_inner(skills, agent).map_err(|e| JsError::new(&e))
}

/// `skillBatchHasPending`.
fn skill_batch_has_pending_inner(
    operations: String,
    skills: String,
    agent: String,
) -> Result<bool, String> {
    let operations = crate::js::from_json(&operations)?;
    let skills = crate::js::from_json(&skills)?;
    let out = port::skill_batch_has_pending(operations, skills, agent)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "skillBatchHasPending")]
pub fn skill_batch_has_pending(
    operations: String,
    skills: String,
    agent: String,
) -> Result<bool, JsError> {
    skill_batch_has_pending_inner(operations, skills, agent).map_err(|e| JsError::new(&e))
}

/// `mergeSkillAgentResult`.
fn merge_skill_agent_result_inner(
    current: String,
    result: String,
    id: String,
    agent: String,
) -> Result<String, String> {
    let current = crate::js::from_json(&current)?;
    let result = crate::js::from_json(&result)?;
    let out = port::merge_skill_agent_result(current, result, id, agent)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "mergeSkillAgentResult")]
pub fn merge_skill_agent_result(
    current: String,
    result: String,
    id: String,
    agent: String,
) -> Result<String, JsError> {
    merge_skill_agent_result_inner(current, result, id, agent).map_err(|e| JsError::new(&e))
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

static S_SKILL_AGENTS_0: wire::Schema = wire::Schema::Dyn;

/// `skillAgents` over the binary wire.
#[wasm_bindgen(js_name = "skillAgentsWire")]
pub fn skill_agents_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let skill: serde_json::Value = wire::read_value(r, &S_SKILL_AGENTS_0)?;
        let out = &port::skill_agents(skill)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            w.string16(x0);
        }
        Ok(())
    })
}

static S_SKILL_CODEX_STATUS_ERROR_0: wire::Schema = wire::Schema::Dyn;

/// `skillCodexStatusError` over the binary wire.
#[wasm_bindgen(js_name = "skillCodexStatusErrorWire")]
pub fn skill_codex_status_error_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let skill: serde_json::Value = wire::read_value(r, &S_SKILL_CODEX_STATUS_ERROR_0)?;
        let out = &port::skill_codex_status_error(skill)?;
        match out {
            Some(x0) => {
                w.tag(1);
                w.string16(x0);
            }
            None => w.tag(0),
        }
        Ok(())
    })
}

static S_SKILL_BATCH_TARGETS_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("installed", wire::Schema::Bool),
    ("bundled", wire::Schema::Bool),
    ("enabled", wire::Schema::Bool),
    (
        "agents",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Str16)),
    ),
    ("conflict", wire::Schema::OptJs(&wire::Schema::Str16)),
    (
        "locations",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Obj(&[(
            "statusError",
            wire::Schema::OptJs(&wire::Schema::Str16),
        )]))),
    ),
]));
static S_SKILL_BATCH_TARGETS_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("installed", wire::Schema::Bool),
    ("bundled", wire::Schema::Bool),
    ("enabled", wire::Schema::Bool),
    (
        "agents",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Str16)),
    ),
    ("conflict", wire::Schema::OptJs(&wire::Schema::Str16)),
    (
        "locations",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Obj(&[(
            "statusError",
            wire::Schema::OptJs(&wire::Schema::Str16),
        )]))),
    ),
]));

/// `skillBatchTargets` over the binary wire.
#[wasm_bindgen(js_name = "skillBatchTargetsWire")]
pub fn skill_batch_targets_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let skills: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_SKILL_BATCH_TARGETS_0.inner()))?;
        let agent: String = r.string16()?;
        let enabled: bool = r.bool()?;
        let out = &port::skill_batch_targets(skills, agent, enabled)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_SKILL_BATCH_TARGETS_OUT.inner())?;
        }
        Ok(())
    })
}

static S_SKILL_BATCH_STATE_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("installed", wire::Schema::Bool),
    ("bundled", wire::Schema::Bool),
    ("enabled", wire::Schema::Bool),
    (
        "agents",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Str16)),
    ),
    ("conflict", wire::Schema::OptJs(&wire::Schema::Str16)),
    (
        "locations",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Obj(&[(
            "statusError",
            wire::Schema::OptJs(&wire::Schema::Str16),
        )]))),
    ),
]));
static S_SKILL_BATCH_STATE_OUT: wire::Schema = wire::Schema::Dyn;

/// `skillBatchState` over the binary wire.
#[wasm_bindgen(js_name = "skillBatchStateWire")]
pub fn skill_batch_state_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let skills: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_SKILL_BATCH_STATE_0.inner()))?;
        let agent: String = r.string16()?;
        let out = &port::skill_batch_state(skills, agent)?;
        wire::write_value(w, out, &S_SKILL_BATCH_STATE_OUT)?;
        Ok(())
    })
}

static S_SKILL_BATCH_HAS_PENDING_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("groupKey", wire::Schema::Str16),
    ("agent", wire::Schema::Str16),
    ("enabled", wire::Schema::Bool),
    ("skillIds", wire::Schema::List(&wire::Schema::Str16)),
]));
static S_SKILL_BATCH_HAS_PENDING_1: wire::Schema =
    wire::Schema::List(&wire::Schema::Obj(&[("id", wire::Schema::Str16)]));

/// `skillBatchHasPending` over the binary wire.
#[wasm_bindgen(js_name = "skillBatchHasPendingWire")]
pub fn skill_batch_has_pending_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let operations: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_SKILL_BATCH_HAS_PENDING_0.inner()))?;
        let skills: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_SKILL_BATCH_HAS_PENDING_1.inner()))?;
        let agent: String = r.string16()?;
        let out = &port::skill_batch_has_pending(operations, skills, agent)?;
        w.bool(*out);
        Ok(())
    })
}

static S_MERGE_SKILL_AGENT_RESULT_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("installed", wire::Schema::Bool),
    ("bundled", wire::Schema::Bool),
    ("enabled", wire::Schema::Bool),
    (
        "agents",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Str16)),
    ),
    ("conflict", wire::Schema::OptJs(&wire::Schema::Str16)),
    (
        "locations",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Obj(&[(
            "statusError",
            wire::Schema::OptJs(&wire::Schema::Str16),
        )]))),
    ),
    ("id", wire::Schema::Str16),
]));
static S_MERGE_SKILL_AGENT_RESULT_1: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("installed", wire::Schema::Bool),
    ("bundled", wire::Schema::Bool),
    ("enabled", wire::Schema::Bool),
    (
        "agents",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Str16)),
    ),
    ("conflict", wire::Schema::OptJs(&wire::Schema::Str16)),
    (
        "locations",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Obj(&[(
            "statusError",
            wire::Schema::OptJs(&wire::Schema::Str16),
        )]))),
    ),
    ("id", wire::Schema::Str16),
]));
static S_MERGE_SKILL_AGENT_RESULT_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("installed", wire::Schema::Bool),
    ("bundled", wire::Schema::Bool),
    ("enabled", wire::Schema::Bool),
    (
        "agents",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Str16)),
    ),
    ("conflict", wire::Schema::OptJs(&wire::Schema::Str16)),
    (
        "locations",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Obj(&[(
            "statusError",
            wire::Schema::OptJs(&wire::Schema::Str16),
        )]))),
    ),
    ("id", wire::Schema::Str16),
]));

/// `mergeSkillAgentResult` over the binary wire.
#[wasm_bindgen(js_name = "mergeSkillAgentResultWire")]
pub fn merge_skill_agent_result_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let current: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_MERGE_SKILL_AGENT_RESULT_0.inner()))?;
        let result: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_MERGE_SKILL_AGENT_RESULT_1.inner()))?;
        let id: String = r.string16()?;
        let agent: String = r.string16()?;
        let out = &port::merge_skill_agent_result(current, result, id, agent)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_MERGE_SKILL_AGENT_RESULT_OUT.inner())?;
        }
        Ok(())
    })
}
