//! JS bindings for `src/skill-groups.ts`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;
pub mod utf16;

/// `customSkillGroupKey`.
fn custom_skill_group_key_inner(name: String) -> Result<String, String> {
    let out = port::custom_skill_group_key(name)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "customSkillGroupKey")]
pub fn custom_skill_group_key(name: String) -> Result<String, JsError> {
    custom_skill_group_key_inner(name).map_err(|e| JsError::new(&e))
}

/// `repositorySkillGroupKey`.
fn repository_skill_group_key_inner(name: String) -> Result<String, String> {
    let out = port::repository_skill_group_key(name)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "repositorySkillGroupKey")]
pub fn repository_skill_group_key(name: String) -> Result<String, JsError> {
    repository_skill_group_key_inner(name).map_err(|e| JsError::new(&e))
}

/// `skillIdentityKey`.
fn skill_identity_key_inner(skill: String) -> Result<String, String> {
    let skill = crate::js::from_json(&skill)?;
    let out = port::skill_identity_key(skill)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "skillIdentityKey")]
pub fn skill_identity_key(skill: String) -> Result<String, JsError> {
    skill_identity_key_inner(skill).map_err(|e| JsError::new(&e))
}

/// `skillGroupIdentityKey`.
fn skill_group_identity_key_inner(skill: String) -> Result<String, String> {
    let skill = crate::js::from_json(&skill)?;
    let out = port::skill_group_identity_key(skill)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "skillGroupIdentityKey")]
pub fn skill_group_identity_key(skill: String) -> Result<String, JsError> {
    skill_group_identity_key_inner(skill).map_err(|e| JsError::new(&e))
}

/// `aggregateSkillGroupItems`.
fn aggregate_skill_group_items_inner(skills: String) -> Result<String, String> {
    let skills = crate::js::from_json(&skills)?;
    let out = port::aggregate_skill_group_items(skills)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "aggregateSkillGroupItems")]
pub fn aggregate_skill_group_items(skills: String) -> Result<String, JsError> {
    aggregate_skill_group_items_inner(skills).map_err(|e| JsError::new(&e))
}

/// `migrateSkillInstanceAssignments`.
fn migrate_skill_instance_assignments_inner(
    assignments_by_identity: String,
    assignments: String,
    skills: String,
    custom_names: Vec<String>,
) -> Result<String, String> {
    let assignments_by_identity = crate::js::from_json(&assignments_by_identity)?;
    let assignments = crate::js::from_json(&assignments)?;
    let skills = crate::js::from_json(&skills)?;
    let out = port::migrate_skill_instance_assignments(
        assignments_by_identity,
        assignments,
        skills,
        custom_names,
    )?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "migrateSkillInstanceAssignments")]
pub fn migrate_skill_instance_assignments(
    assignments_by_identity: String,
    assignments: String,
    skills: String,
    custom_names: Vec<String>,
) -> Result<String, JsError> {
    migrate_skill_instance_assignments_inner(
        assignments_by_identity,
        assignments,
        skills,
        custom_names,
    )
    .map_err(|e| JsError::new(&e))
}

/// `migrateSkillGroupAssignments`.
fn migrate_skill_group_assignments_inner(
    assignments: String,
    changes: String,
) -> Result<String, String> {
    let assignments = crate::js::from_json(&assignments)?;
    let changes = crate::js::from_json(&changes)?;
    let out = port::migrate_skill_group_assignments(assignments, changes)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "migrateSkillGroupAssignments")]
pub fn migrate_skill_group_assignments(
    assignments: String,
    changes: String,
) -> Result<String, JsError> {
    migrate_skill_group_assignments_inner(assignments, changes).map_err(|e| JsError::new(&e))
}

/// `skillRepositoryName`.
fn skill_repository_name_inner(repo_key: String) -> Result<String, String> {
    let out = port::skill_repository_name(repo_key)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "skillRepositoryName")]
pub fn skill_repository_name(repo_key: String) -> Result<String, JsError> {
    skill_repository_name_inner(repo_key).map_err(|e| JsError::new(&e))
}

/// `defaultSkillGroupKey`.
fn default_skill_group_key_inner(skill: String) -> Result<String, String> {
    let skill = crate::js::from_json(&skill)?;
    let out = port::default_skill_group_key(skill)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "defaultSkillGroupKey")]
pub fn default_skill_group_key(skill: String) -> Result<String, JsError> {
    default_skill_group_key_inner(skill).map_err(|e| JsError::new(&e))
}

/// `legacySkillCustomGroupName`.
fn legacy_skill_custom_group_name_inner(
    value: String,
    custom_names: Vec<String>,
) -> Result<String, String> {
    let out = port::legacy_skill_custom_group_name(value, custom_names)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "legacySkillCustomGroupName")]
pub fn legacy_skill_custom_group_name(
    value: String,
    custom_names: Vec<String>,
) -> Result<String, JsError> {
    legacy_skill_custom_group_name_inner(value, custom_names).map_err(|e| JsError::new(&e))
}

/// `skillGroupAssignment`.
fn skill_group_assignment_inner(
    skill: String,
    custom_names: Vec<String>,
    assignments: String,
    assignments_by_identity: String,
) -> Result<String, String> {
    let skill = crate::js::from_json(&skill)?;
    let assignments = crate::js::from_json(&assignments)?;
    let assignments_by_identity = crate::js::from_json(&assignments_by_identity)?;
    let out =
        port::skill_group_assignment(skill, custom_names, assignments, assignments_by_identity)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "skillGroupAssignment")]
pub fn skill_group_assignment(
    skill: String,
    custom_names: Vec<String>,
    assignments: String,
    assignments_by_identity: String,
) -> Result<String, JsError> {
    skill_group_assignment_inner(skill, custom_names, assignments, assignments_by_identity)
        .map_err(|e| JsError::new(&e))
}

/// `readSkillGroupStorage`.
fn read_skill_group_storage_inner(value: Option<String>) -> Result<String, String> {
    let out = port::read_skill_group_storage(value)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "readSkillGroupStorage")]
pub fn read_skill_group_storage(value: Option<String>) -> Result<String, JsError> {
    read_skill_group_storage_inner(value).map_err(|e| JsError::new(&e))
}

/// `sortSkillGroupsByName`.
fn sort_skill_groups_by_name_inner(groups: String, direction: String) -> Result<String, String> {
    let groups = crate::js::from_json(&groups)?;
    let out = port::sort_skill_groups_by_name(groups, direction)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "sortSkillGroupsByName")]
pub fn sort_skill_groups_by_name(groups: String, direction: String) -> Result<String, JsError> {
    sort_skill_groups_by_name_inner(groups, direction).map_err(|e| JsError::new(&e))
}

/// `sortSkillGroupKeys`.
fn sort_skill_group_keys_inner(groups: String, direction: String) -> Result<Vec<String>, String> {
    let groups = crate::js::from_json(&groups)?;
    let out = port::sort_skill_group_keys(groups, direction)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "sortSkillGroupKeys")]
pub fn sort_skill_group_keys(groups: String, direction: String) -> Result<Vec<String>, JsError> {
    sort_skill_group_keys_inner(groups, direction).map_err(|e| JsError::new(&e))
}

/// `nextSkillGroupSortDirection`.
fn next_skill_group_sort_direction_inner(direction: Option<String>) -> Result<String, String> {
    let out = port::next_skill_group_sort_direction(direction)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "nextSkillGroupSortDirection")]
pub fn next_skill_group_sort_direction(direction: Option<String>) -> Result<String, JsError> {
    next_skill_group_sort_direction_inner(direction).map_err(|e| JsError::new(&e))
}

/// `moveSkillGroup`.
fn move_skill_group_inner(groups: String, from: String, to: String) -> Result<Vec<String>, String> {
    let groups = crate::js::from_json(&groups)?;
    let out = port::move_skill_group(groups, from, to)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "moveSkillGroup")]
pub fn move_skill_group(groups: String, from: String, to: String) -> Result<Vec<String>, JsError> {
    move_skill_group_inner(groups, from, to).map_err(|e| JsError::new(&e))
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

static S_CUSTOM_SKILL_GROUP_KEY_OUT: wire::Schema = wire::Schema::Dyn;

/// `customSkillGroupKey` over the binary wire.
#[wasm_bindgen(js_name = "customSkillGroupKeyWire")]
pub fn custom_skill_group_key_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let name: String = r.string16()?;
        let out = &port::custom_skill_group_key(name)?;
        wire::write_value(w, out, &S_CUSTOM_SKILL_GROUP_KEY_OUT)?;
        Ok(())
    })
}

static S_REPOSITORY_SKILL_GROUP_KEY_OUT: wire::Schema = wire::Schema::Dyn;

/// `repositorySkillGroupKey` over the binary wire.
#[wasm_bindgen(js_name = "repositorySkillGroupKeyWire")]
pub fn repository_skill_group_key_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let name: String = r.string16()?;
        let out = &port::repository_skill_group_key(name)?;
        wire::write_value(w, out, &S_REPOSITORY_SKILL_GROUP_KEY_OUT)?;
        Ok(())
    })
}

static S_SKILL_IDENTITY_KEY_0: wire::Schema = wire::Schema::Dyn;
static S_SKILL_IDENTITY_KEY_OUT: wire::Schema = wire::Schema::Dyn;

/// `skillIdentityKey` over the binary wire.
#[wasm_bindgen(js_name = "skillIdentityKeyWire")]
pub fn skill_identity_key_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let skill: Value = wire::read_value(r, &S_SKILL_IDENTITY_KEY_0)?;
        let out = &port::skill_identity_key(skill)?;
        wire::write_value(w, out, &S_SKILL_IDENTITY_KEY_OUT)?;
        Ok(())
    })
}

static S_SKILL_GROUP_IDENTITY_KEY_0: wire::Schema = wire::Schema::Dyn;
static S_SKILL_GROUP_IDENTITY_KEY_OUT: wire::Schema = wire::Schema::Dyn;

/// `skillGroupIdentityKey` over the binary wire.
#[wasm_bindgen(js_name = "skillGroupIdentityKeyWire")]
pub fn skill_group_identity_key_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let skill: Value = wire::read_value(r, &S_SKILL_GROUP_IDENTITY_KEY_0)?;
        let out = &port::skill_group_identity_key(skill)?;
        wire::write_value(w, out, &S_SKILL_GROUP_IDENTITY_KEY_OUT)?;
        Ok(())
    })
}

static S_AGGREGATE_SKILL_GROUP_ITEMS_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("repoKey", wire::Schema::Str16),
    ("bundled", wire::Schema::Bool),
    ("instanceKey", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("conflict", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("conflictKind", wire::Schema::OptJs(&wire::Schema::Str16)),
    (
        "displayGroupRepoKey",
        wire::Schema::OptJs(&wire::Schema::Str16),
    ),
    ("installed", wire::Schema::Bool),
    ("enabled", wire::Schema::Bool),
    (
        "agents",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Str16)),
    ),
    ("repoPath", wire::Schema::OptJs(&wire::Schema::Str16)),
    (
        "locations",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Obj(&[(
            "rootKind",
            wire::Schema::Str16,
        )]))),
    ),
    (
        "safeDisableAgents",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Str16)),
    ),
]));
static S_AGGREGATE_SKILL_GROUP_ITEMS_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("repoKey", wire::Schema::Str16),
    ("bundled", wire::Schema::Bool),
    ("instanceKey", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("conflict", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("conflictKind", wire::Schema::OptJs(&wire::Schema::Str16)),
    (
        "displayGroupRepoKey",
        wire::Schema::OptJs(&wire::Schema::Str16),
    ),
    ("installed", wire::Schema::Bool),
    ("enabled", wire::Schema::Bool),
    (
        "agents",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Str16)),
    ),
    ("repoPath", wire::Schema::OptJs(&wire::Schema::Str16)),
    (
        "locations",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Obj(&[(
            "rootKind",
            wire::Schema::Str16,
        )]))),
    ),
    (
        "safeDisableAgents",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Str16)),
    ),
    (
        "displayInstances",
        wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Obj(&[
            ("id", wire::Schema::Str16),
            ("repoKey", wire::Schema::Str16),
            ("bundled", wire::Schema::Bool),
            ("instanceKey", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("conflict", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("conflictKind", wire::Schema::OptJs(&wire::Schema::Str16)),
            (
                "displayGroupRepoKey",
                wire::Schema::OptJs(&wire::Schema::Str16),
            ),
            ("installed", wire::Schema::Bool),
            ("enabled", wire::Schema::Bool),
            (
                "agents",
                wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Str16)),
            ),
            ("repoPath", wire::Schema::OptJs(&wire::Schema::Str16)),
            (
                "locations",
                wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Dyn)),
            ),
            (
                "safeDisableAgents",
                wire::Schema::OptJs(&wire::Schema::List(&wire::Schema::Str16)),
            ),
        ]))),
    ),
]));

/// `aggregateSkillGroupItems` over the binary wire.
#[wasm_bindgen(js_name = "aggregateSkillGroupItemsWire")]
pub fn aggregate_skill_group_items_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let skills: Vec<Value> =
            r.list(|r| wire::read_value(r, S_AGGREGATE_SKILL_GROUP_ITEMS_0.inner()))?;
        let out = &port::aggregate_skill_group_items(skills)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_AGGREGATE_SKILL_GROUP_ITEMS_OUT.inner())?;
        }
        Ok(())
    })
}

static S_MIGRATE_SKILL_INSTANCE_ASSIGNMENTS_0: wire::Schema = wire::Schema::Dyn;
static S_MIGRATE_SKILL_INSTANCE_ASSIGNMENTS_1: wire::Schema = wire::Schema::Dyn;
static S_MIGRATE_SKILL_INSTANCE_ASSIGNMENTS_2: wire::Schema = wire::Schema::Dyn;
static S_MIGRATE_SKILL_INSTANCE_ASSIGNMENTS_OUT: wire::Schema =
    wire::Schema::Map16(&wire::Schema::Str16);

/// `migrateSkillInstanceAssignments` over the binary wire.
#[wasm_bindgen(js_name = "migrateSkillInstanceAssignmentsWire")]
pub fn migrate_skill_instance_assignments_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let assignments_by_identity: Value =
            wire::read_value(r, &S_MIGRATE_SKILL_INSTANCE_ASSIGNMENTS_0)?;
        let assignments: Value = wire::read_value(r, &S_MIGRATE_SKILL_INSTANCE_ASSIGNMENTS_1)?;
        let skills: Value = wire::read_value(r, &S_MIGRATE_SKILL_INSTANCE_ASSIGNMENTS_2)?;
        let custom_names: Vec<String> = r.list(|r| r.string16())?;
        let out = &port::migrate_skill_instance_assignments(
            assignments_by_identity,
            assignments,
            skills,
            custom_names,
        )?;
        wire::write_value(w, out, &S_MIGRATE_SKILL_INSTANCE_ASSIGNMENTS_OUT)?;
        Ok(())
    })
}

static S_MIGRATE_SKILL_GROUP_ASSIGNMENTS_0: wire::Schema = wire::Schema::Dyn;
static S_MIGRATE_SKILL_GROUP_ASSIGNMENTS_1: wire::Schema = wire::Schema::Dyn;
static S_MIGRATE_SKILL_GROUP_ASSIGNMENTS_OUT: wire::Schema =
    wire::Schema::Map16(&wire::Schema::Str16);

/// `migrateSkillGroupAssignments` over the binary wire.
#[wasm_bindgen(js_name = "migrateSkillGroupAssignmentsWire")]
pub fn migrate_skill_group_assignments_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let assignments: Value = wire::read_value(r, &S_MIGRATE_SKILL_GROUP_ASSIGNMENTS_0)?;
        let changes: Value = wire::read_value(r, &S_MIGRATE_SKILL_GROUP_ASSIGNMENTS_1)?;
        let out = &port::migrate_skill_group_assignments(assignments, changes)?;
        wire::write_value(w, out, &S_MIGRATE_SKILL_GROUP_ASSIGNMENTS_OUT)?;
        Ok(())
    })
}

static S_SKILL_REPOSITORY_NAME_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Str16);

/// `skillRepositoryName` over the binary wire.
#[wasm_bindgen(js_name = "skillRepositoryNameWire")]
pub fn skill_repository_name_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let repo_key: String = r.string16()?;
        let out = &port::skill_repository_name(repo_key)?;
        wire::write_value(w, out, &S_SKILL_REPOSITORY_NAME_OUT)?;
        Ok(())
    })
}

static S_DEFAULT_SKILL_GROUP_KEY_0: wire::Schema = wire::Schema::Dyn;

/// `defaultSkillGroupKey` over the binary wire.
#[wasm_bindgen(js_name = "defaultSkillGroupKeyWire")]
pub fn default_skill_group_key_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let skill: Value = wire::read_value(r, &S_DEFAULT_SKILL_GROUP_KEY_0)?;
        let out = &port::default_skill_group_key(skill)?;
        w.string16(out);
        Ok(())
    })
}

static S_LEGACY_SKILL_CUSTOM_GROUP_NAME_OUT: wire::Schema =
    wire::Schema::OptJs(&wire::Schema::Str16);

/// `legacySkillCustomGroupName` over the binary wire.
#[wasm_bindgen(js_name = "legacySkillCustomGroupNameWire")]
pub fn legacy_skill_custom_group_name_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let value: String = r.string16()?;
        let custom_names: Vec<String> = r.list(|r| r.string16())?;
        let out = &port::legacy_skill_custom_group_name(value, custom_names)?;
        wire::write_value(w, out, &S_LEGACY_SKILL_CUSTOM_GROUP_NAME_OUT)?;
        Ok(())
    })
}

static S_SKILL_GROUP_ASSIGNMENT_0: wire::Schema = wire::Schema::Dyn;
static S_SKILL_GROUP_ASSIGNMENT_2: wire::Schema = wire::Schema::Dyn;
static S_SKILL_GROUP_ASSIGNMENT_3: wire::Schema = wire::Schema::Dyn;

/// `skillGroupAssignment` over the binary wire.
#[wasm_bindgen(js_name = "skillGroupAssignmentWire")]
pub fn skill_group_assignment_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let skill: Value = wire::read_value(r, &S_SKILL_GROUP_ASSIGNMENT_0)?;
        let custom_names: Vec<String> = r.list(|r| r.string16())?;
        let assignments: Value = wire::read_value(r, &S_SKILL_GROUP_ASSIGNMENT_2)?;
        let assignments_by_identity: Value = wire::read_value(r, &S_SKILL_GROUP_ASSIGNMENT_3)?;
        let out = &port::skill_group_assignment(
            skill,
            custom_names,
            assignments,
            assignments_by_identity,
        )?;
        w.string16(out);
        Ok(())
    })
}

static S_READ_SKILL_GROUP_STORAGE_OUT: wire::Schema = wire::Schema::Map16(&wire::Schema::Dyn);

/// `readSkillGroupStorage` over the binary wire.
#[wasm_bindgen(js_name = "readSkillGroupStorageWire")]
pub fn read_skill_group_storage_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let value: Option<String> = r.opt_js(|r| r.string16())?;
        let out = &port::read_skill_group_storage(value)?;
        wire::write_value(w, out, &S_READ_SKILL_GROUP_STORAGE_OUT)?;
        Ok(())
    })
}

static S_SORT_SKILL_GROUPS_BY_NAME_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("key", wire::Schema::Str16),
    ("label", wire::Schema::Str16),
    ("kind", wire::Schema::Str16),
    ("items", wire::Schema::List(&wire::Schema::Dyn)),
]));
static S_SORT_SKILL_GROUPS_BY_NAME_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("key", wire::Schema::Str16),
    ("label", wire::Schema::Str16),
    ("kind", wire::Schema::Str16),
    ("items", wire::Schema::List(&wire::Schema::Dyn)),
]));

/// `sortSkillGroupsByName` over the binary wire.
#[wasm_bindgen(js_name = "sortSkillGroupsByNameWire")]
pub fn sort_skill_groups_by_name_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let groups: Vec<Value> =
            r.list(|r| wire::read_value(r, S_SORT_SKILL_GROUPS_BY_NAME_0.inner()))?;
        let direction: String = r.string16()?;
        let out = &port::sort_skill_groups_by_name(groups, direction)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_SORT_SKILL_GROUPS_BY_NAME_OUT.inner())?;
        }
        Ok(())
    })
}

static S_SORT_SKILL_GROUP_KEYS_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("key", wire::Schema::Str16),
    ("label", wire::Schema::Str16),
    ("kind", wire::Schema::Str16),
    ("items", wire::Schema::List(&wire::Schema::Dyn)),
]));

/// `sortSkillGroupKeys` over the binary wire.
#[wasm_bindgen(js_name = "sortSkillGroupKeysWire")]
pub fn sort_skill_group_keys_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let groups: Vec<Value> =
            r.list(|r| wire::read_value(r, S_SORT_SKILL_GROUP_KEYS_0.inner()))?;
        let direction: String = r.string16()?;
        let out = &port::sort_skill_group_keys(groups, direction)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            w.string16(x0);
        }
        Ok(())
    })
}

static S_MOVE_SKILL_GROUP_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("key", wire::Schema::Str16),
    ("label", wire::Schema::Str16),
    ("kind", wire::Schema::Str16),
    ("items", wire::Schema::List(&wire::Schema::Dyn)),
]));

/// `moveSkillGroup` over the binary wire.
#[wasm_bindgen(js_name = "moveSkillGroupWire")]
pub fn move_skill_group_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let groups: Vec<Value> = r.list(|r| wire::read_value(r, S_MOVE_SKILL_GROUP_0.inner()))?;
        let from: String = r.string16()?;
        let to: String = r.string16()?;
        let out = &port::move_skill_group(groups, from, to)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            w.string16(x0);
        }
        Ok(())
    })
}
