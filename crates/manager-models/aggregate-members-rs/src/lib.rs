//! JS bindings for `src/aggregate-members.ts`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;

/// `isAggregateProfile`.
fn is_aggregate_profile_inner(profile: String) -> Result<bool, String> {
    let profile = crate::js::from_json(&profile)?;
    let out = port::is_aggregate_profile(profile)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "isAggregateProfile")]
pub fn is_aggregate_profile(profile: String) -> Result<bool, JsError> {
    is_aggregate_profile_inner(profile).map_err(|e| JsError::new(&e))
}

/// `isApiMemberProfile`.
fn is_api_member_profile_inner(profile: String) -> Result<bool, String> {
    let profile = crate::js::from_json(&profile)?;
    let out = port::is_api_member_profile(profile)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "isApiMemberProfile")]
pub fn is_api_member_profile(profile: String) -> Result<bool, JsError> {
    is_api_member_profile_inner(profile).map_err(|e| JsError::new(&e))
}

/// `authContentsHaveChatgptTokens`.
fn auth_contents_have_chatgpt_tokens_inner(contents: Option<String>) -> Result<bool, String> {
    let out = port::auth_contents_have_chatgpt_tokens(contents)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "authContentsHaveChatgptTokens")]
pub fn auth_contents_have_chatgpt_tokens(contents: Option<String>) -> Result<bool, JsError> {
    auth_contents_have_chatgpt_tokens_inner(contents).map_err(|e| JsError::new(&e))
}

/// `isOfficialLoginMemberProfile`.
fn is_official_login_member_profile_inner(profile: String) -> Result<bool, String> {
    let profile = crate::js::from_json(&profile)?;
    let out = port::is_official_login_member_profile(profile)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "isOfficialLoginMemberProfile")]
pub fn is_official_login_member_profile(profile: String) -> Result<bool, JsError> {
    is_official_login_member_profile_inner(profile).map_err(|e| JsError::new(&e))
}

/// `aggregateMemberCandidates`.
fn aggregate_member_candidates_inner(
    profiles: String,
    aggregate_id: String,
) -> Result<String, String> {
    let profiles = crate::js::from_json(&profiles)?;
    let out = port::aggregate_member_candidates(profiles, aggregate_id)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "aggregateMemberCandidates")]
pub fn aggregate_member_candidates(
    profiles: String,
    aggregate_id: String,
) -> Result<String, JsError> {
    aggregate_member_candidates_inner(profiles, aggregate_id).map_err(|e| JsError::new(&e))
}

/// `normalizeAggregateDefaultRelayId`.
fn normalize_aggregate_default_relay_id_inner(
    value: Option<String>,
    member_ids: String,
) -> Result<String, String> {
    let member_ids = crate::js::from_json(&member_ids)?;
    let out = port::normalize_aggregate_default_relay_id(value, member_ids)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "normalizeAggregateDefaultRelayId")]
pub fn normalize_aggregate_default_relay_id(
    value: Option<String>,
    member_ids: String,
) -> Result<String, JsError> {
    normalize_aggregate_default_relay_id_inner(value, member_ids).map_err(|e| JsError::new(&e))
}

/// `normalizeAggregateMembers`.
fn normalize_aggregate_members_inner(
    members: String,
    profile_ids: String,
) -> Result<String, String> {
    let members = crate::js::from_json(&members)?;
    let profile_ids = crate::js::from_json(&profile_ids)?;
    let out = port::normalize_aggregate_members(members, profile_ids)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "normalizeAggregateMembers")]
pub fn normalize_aggregate_members(
    members: String,
    profile_ids: String,
) -> Result<String, JsError> {
    normalize_aggregate_members_inner(members, profile_ids).map_err(|e| JsError::new(&e))
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

static S_IS_AGGREGATE_PROFILE_0: wire::Schema = wire::Schema::Dyn;

/// `isAggregateProfile` over the binary wire.
#[wasm_bindgen(js_name = "isAggregateProfileWire")]
pub fn is_aggregate_profile_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let profile: serde_json::Value = wire::read_value(r, &S_IS_AGGREGATE_PROFILE_0)?;
        let out = &port::is_aggregate_profile(profile)?;
        w.bool(*out);
        Ok(())
    })
}

static S_IS_API_MEMBER_PROFILE_0: wire::Schema = wire::Schema::Dyn;

/// `isApiMemberProfile` over the binary wire.
#[wasm_bindgen(js_name = "isApiMemberProfileWire")]
pub fn is_api_member_profile_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let profile: serde_json::Value = wire::read_value(r, &S_IS_API_MEMBER_PROFILE_0)?;
        let out = &port::is_api_member_profile(profile)?;
        w.bool(*out);
        Ok(())
    })
}

static S_IS_OFFICIAL_LOGIN_MEMBER_PROFILE_0: wire::Schema = wire::Schema::Dyn;

/// `isOfficialLoginMemberProfile` over the binary wire.
#[wasm_bindgen(js_name = "isOfficialLoginMemberProfileWire")]
pub fn is_official_login_member_profile_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let profile: serde_json::Value =
            wire::read_value(r, &S_IS_OFFICIAL_LOGIN_MEMBER_PROFILE_0)?;
        let out = &port::is_official_login_member_profile(profile)?;
        w.bool(*out);
        Ok(())
    })
}

static S_AGGREGATE_MEMBER_CANDIDATES_0: wire::Schema = wire::Schema::List(&wire::Schema::Dyn);
static S_AGGREGATE_MEMBER_CANDIDATES_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Dyn);

/// `aggregateMemberCandidates` over the binary wire.
#[wasm_bindgen(js_name = "aggregateMemberCandidatesWire")]
pub fn aggregate_member_candidates_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let profiles: Vec<serde_json::Value> =
            r.list(|r| wire::read_value(r, S_AGGREGATE_MEMBER_CANDIDATES_0.inner()))?;
        let aggregate_id: String = r.string16()?;
        let out = &port::aggregate_member_candidates(profiles, aggregate_id)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_AGGREGATE_MEMBER_CANDIDATES_OUT.inner())?;
        }
        Ok(())
    })
}

static S_NORMALIZE_AGGREGATE_DEFAULT_RELAY_ID_1: wire::Schema = wire::Schema::Dyn;

/// `normalizeAggregateDefaultRelayId` over the binary wire.
#[wasm_bindgen(js_name = "normalizeAggregateDefaultRelayIdWire")]
pub fn normalize_aggregate_default_relay_id_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let value: Option<String> = r.opt_js(|r| r.string16())?;
        let member_ids: serde_json::Value =
            wire::read_value(r, &S_NORMALIZE_AGGREGATE_DEFAULT_RELAY_ID_1)?;
        let out = &port::normalize_aggregate_default_relay_id(value, member_ids)?;
        w.string16(out);
        Ok(())
    })
}

static S_NORMALIZE_AGGREGATE_MEMBERS_0: wire::Schema = wire::Schema::Dyn;
static S_NORMALIZE_AGGREGATE_MEMBERS_1: wire::Schema = wire::Schema::Dyn;
static S_NORMALIZE_AGGREGATE_MEMBERS_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("profileId", wire::Schema::Str16),
    ("weight", wire::Schema::Num),
]));

/// `normalizeAggregateMembers` over the binary wire.
#[wasm_bindgen(js_name = "normalizeAggregateMembersWire")]
pub fn normalize_aggregate_members_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let members: serde_json::Value = wire::read_value(r, &S_NORMALIZE_AGGREGATE_MEMBERS_0)?;
        let profile_ids: serde_json::Value = wire::read_value(r, &S_NORMALIZE_AGGREGATE_MEMBERS_1)?;
        let out = &port::normalize_aggregate_members(members, profile_ids)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_NORMALIZE_AGGREGATE_MEMBERS_OUT.inner())?;
        }
        Ok(())
    })
}
