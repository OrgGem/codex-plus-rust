//! The Rust port of `src/skill-groups.ts`.

#![forbid(unsafe_code)]

use crate::js;
use indexmap::IndexMap;
use manager_common_rs::{is_nil, is_plain_object, is_truthy, js_str, spread_object};
use serde_json::{Map, Value};
use std::cmp::Ordering;
use std::collections::HashSet;

pub const BUILTIN_SKILL_GROUP: &str = "builtin";
pub const USER_SKILL_GROUP: &str = "user";

fn is_js_ws(c: char) -> bool {
    c != '\u{0085}' && (c.is_whitespace() || c == '\u{FEFF}')
}

fn js_trim(s: &str) -> &str {
    s.trim_matches(is_js_ws)
}

fn empty_map() -> &'static Map<String, Value> {
    static EMPTY: std::sync::LazyLock<Map<String, Value>> = std::sync::LazyLock::new(Map::new);
    &EMPTY
}

fn require_obj<'a>(v: &'a Value, prop: &str) -> Result<&'a Map<String, Value>, String> {
    if is_nil(v) {
        return Err(format!(
            "TypeError: Cannot read properties of null/undefined (reading '{prop}')"
        ));
    }
    Ok(v.as_object().unwrap_or(empty_map()))
}

fn require_has_own_target<'a>(v: &'a Value, label: &str) -> Result<&'a Map<String, Value>, String> {
    if is_nil(v) {
        return Err(format!(
            "TypeError: Cannot convert undefined or null to object ({label})"
        ));
    }
    Ok(v.as_object().unwrap_or(empty_map()))
}

fn js_json_array(items: &[&Value]) -> Result<String, String> {
    let mapped: Vec<Value> = items
        .iter()
        .map(|v| {
            if js::is_undefined(v) {
                Value::Null
            } else {
                (*v).clone()
            }
        })
        .collect();
    serde_json::to_string(&mapped).map_err(|e| format!("TypeError: {e}"))
}

fn skill_identity_key_str(skill: &Value) -> Result<String, String> {
    let obj = require_obj(skill, "instanceKey")?;
    let id = obj.get("id").unwrap_or(&Value::Null);
    let id_missing = !obj.contains_key("id");
    let undef = js::undefined();
    let id_ref = if id_missing { &undef } else { id };

    if let Some(inst) = obj.get("instanceKey")
        && is_truthy(inst)
        && inst != id_ref
    {
        return js_json_array(&[&Value::String("instance".into()), id_ref, inst]);
    }
    let repo_ref = obj.get("repoKey").unwrap_or(&undef);
    js_json_array(&[repo_ref, id_ref])
}

fn skill_group_identity_key_str(skill: &Value) -> Result<String, String> {
    let obj = require_obj(skill, "conflictKind")?;
    if obj.get("conflictKind").and_then(Value::as_str) == Some("equal-independent") {
        let undef = js::undefined();
        let id_ref = obj.get("id").unwrap_or(&undef);
        return js_json_array(&[&Value::String("aggregate".into()), id_ref]);
    }
    skill_identity_key_str(skill)
}

fn parse_repo_name(repo_key: &str) -> Option<&str> {
    let (owner_repo, after_at) = repo_key.split_once('@')?;
    let (owner, repo) = owner_repo.split_once('/')?;
    if owner.is_empty() || repo.is_empty() {
        return None;
    }
    if owner.contains(['/', '@', ':']) || repo.contains(['/', '@', ':']) {
        return None;
    }
    if after_at.is_empty() || after_at.starts_with(':') {
        return None;
    }
    if let Some((_, subpath)) = after_at.split_once(':')
        && subpath.contains(['\n', '\r', '\u{2028}', '\u{2029}'])
    {
        return None;
    }
    Some(owner_repo)
}

fn aggregate_legacy_assignment(
    skill: &Value,
    assignments_by_identity: &Map<String, Value>,
) -> Result<Option<Option<String>>, String> {
    let existing = skill_identity_key_str(skill)?;
    if let Some(val) = assignments_by_identity.get(&existing) {
        return Ok(Some(Some(js_str(val))));
    }
    let skill_id = skill
        .as_object()
        .and_then(|o| o.get("id"))
        .cloned()
        .unwrap_or_else(js::undefined);
    let mut values: Vec<Value> = Vec::new();
    for (key, val) in assignments_by_identity {
        if let Ok(Value::Array(tuple)) = serde_json::from_str::<Value>(key)
            && tuple.len() == 2
            && tuple[1] == skill_id
            && tuple[0] != Value::String("aggregate".into())
        {
            values.push(val.clone());
        }
    }
    if values.is_empty() {
        return Ok(None);
    }
    let first = &values[0];
    if values.iter().all(|v| v == first) {
        Ok(Some(Some(js_str(first))))
    } else {
        Ok(Some(None))
    }
}

fn string_record(input: Option<&Value>) -> Map<String, Value> {
    let mut out = Map::new();
    let Some(val) = input else {
        return out;
    };
    if is_plain_object(val)
        && let Some(obj) = val.as_object()
    {
        for (k, v) in obj {
            if let Value::String(s) = v {
                out.insert(k.clone(), Value::String(s.clone()));
            }
        }
    }
    out
}

/// Primary, secondary (accent), and tertiary (case) weights matching Node default `en-US` `localeCompare`.
fn char_weights(c: char, out: &mut Vec<(u32, u16, u16)>) {
    if matches!(c, '\u{FEFF}' | '\u{0300}'..='\u{036F}') {
        if ('\u{0300}'..='\u{036F}').contains(&c) {
            out.push((0, (c as u16) - 0x0300 + 1, 0));
        }
        return;
    }
    if c == 'ß' {
        if let Some(w) = ascii_primary('s') {
            out.push((w, 1, 1));
            out.push((w, 1, 1));
            out.push((0, 1, 2));
        }
        return;
    }
    let decomposed: Option<(char, u16, u16)> = match c {
        'é' => Some(('e', 2, 1)),
        'É' => Some(('e', 2, 2)),
        'è' => Some(('e', 3, 1)),
        'È' => Some(('e', 3, 2)),
        'ê' => Some(('e', 4, 1)),
        'Ê' => Some(('e', 4, 2)),
        'ë' => Some(('e', 5, 1)),
        'Ë' => Some(('e', 5, 2)),
        'İ' => Some(('i', 3, 2)),
        'ï' => Some(('i', 2, 1)),
        'Ï' => Some(('i', 2, 2)),
        'ì' | 'í' | 'î' => Some(('i', 2, 1)),
        'ü' => Some(('u', 2, 1)),
        'Ü' => Some(('u', 2, 2)),
        'ù' | 'ú' | 'û' => Some(('u', 2, 1)),
        'ö' => Some(('o', 2, 1)),
        'Ö' => Some(('o', 2, 2)),
        'ò' | 'ó' | 'ô' | 'õ' | 'ø' => Some(('o', 2, 1)),
        'ä' | 'à' | 'á' | 'â' | 'ã' | 'å' => Some(('a', 2, 1)),
        'Ä' | 'À' | 'Á' | 'Â' | 'Ã' | 'Å' => Some(('a', 2, 2)),
        'ñ' => Some(('n', 2, 1)),
        'Ñ' => Some(('n', 2, 2)),
        'ç' => Some(('c', 2, 1)),
        'Ç' => Some(('c', 2, 2)),
        _ => None,
    };
    if let Some((base, sec, tert)) = decomposed {
        if let Some(w) = ascii_primary(base) {
            out.push((w, sec, tert));
        }
        return;
    }
    if c.is_ascii() {
        if let Some(w) = ascii_primary(c)
            && w > 0
        {
            let tert = if c.is_ascii_uppercase() { 2 } else { 1 };
            out.push((w, 1, tert));
        }
        return;
    }
    if matches!(c, '\u{2000}'..='\u{2BFF}' | '\u{1F000}'..='\u{1FAFF}') {
        out.push((38, 1, 1));
        out.push((0x10_0000 + (c as u32), 1, 1));
        return;
    }
    let lower = c.to_lowercase().next().unwrap_or(c);
    let (base_cp, sec) = match lower {
        'ά' => ('α' as u32, 2),
        'έ' => ('ε' as u32, 2),
        'ή' => ('η' as u32, 2),
        'ί' => ('ι' as u32, 2),
        'ό' => ('ο' as u32, 2),
        'ύ' => ('υ' as u32, 2),
        'ώ' => ('ω' as u32, 2),
        other => (other as u32, 1),
    };
    let tert = if c != lower { 2 } else { 1 };
    out.push((0x20_0000 + base_cp, sec, tert));
}

fn ascii_primary(ch: char) -> Option<u32> {
    if !ch.is_ascii() {
        return None;
    }
    let b = ch as u8;
    if matches!(b, 0x00..=0x08 | 0x0E..=0x1F | 0x7F) {
        return Some(0);
    }
    const ORDER: &[u8] =
        b"\t\n\r _-,;:!?.'\"()[]{}@*/\\&#%`^+<=>|~$0123456789abcdefghijklmnopqrstuvwxyz";
    let lower = b.to_ascii_lowercase();
    ORDER
        .iter()
        .position(|&x| x == lower)
        .map(|pos| (pos as u32) + 1)
}

fn locale_compare_default(a: &str, b: &str) -> Ordering {
    let mut wa = Vec::with_capacity(a.len());
    let mut wb = Vec::with_capacity(b.len());
    for c in a.chars() {
        char_weights(c, &mut wa);
    }
    for c in b.chars() {
        char_weights(c, &mut wb);
    }
    // Primary pass
    let pa: Vec<u32> = wa.iter().filter(|w| w.0 > 0).map(|w| w.0).collect();
    let pb: Vec<u32> = wb.iter().filter(|w| w.0 > 0).map(|w| w.0).collect();
    if let ord @ (Ordering::Less | Ordering::Greater) = pa.cmp(&pb) {
        return ord;
    }
    // Secondary pass
    let sa: Vec<u16> = wa.iter().map(|w| w.1).collect();
    let sb: Vec<u16> = wb.iter().map(|w| w.1).collect();
    if let ord @ (Ordering::Less | Ordering::Greater) = sa.cmp(&sb) {
        return ord;
    }
    // Tertiary pass
    let ta: Vec<u16> = wa.iter().map(|w| w.2).collect();
    let tb: Vec<u16> = wb.iter().map(|w| w.2).collect();
    ta.cmp(&tb)
}

/// Port of `customSkillGroupKey` (`src/skill-groups.ts:24`).
pub fn custom_skill_group_key(name: String) -> Result<Value, String> {
    Ok(Value::String(format!("custom:{name}")))
}

/// Port of `repositorySkillGroupKey` (`src/skill-groups.ts:25`).
pub fn repository_skill_group_key(name: String) -> Result<Value, String> {
    Ok(Value::String(format!("repo:{name}")))
}

/// Port of `skillIdentityKey` (`src/skill-groups.ts:26`).
pub fn skill_identity_key(skill: Value) -> Result<Value, String> {
    Ok(Value::String(skill_identity_key_str(&skill)?))
}

/// Port of `skillGroupIdentityKey` (`src/skill-groups.ts:29`).
pub fn skill_group_identity_key(skill: Value) -> Result<Value, String> {
    Ok(Value::String(skill_group_identity_key_str(&skill)?))
}

/// Port of `aggregateSkillGroupItems` (`src/skill-groups.ts:33`).
pub fn aggregate_skill_group_items(skills: Vec<Value>) -> Result<Vec<Value>, String> {
    let mut candidates: IndexMap<String, Vec<Value>> = IndexMap::new();
    for skill in &skills {
        let obj = require_obj(skill, "installed")?;
        if !obj.get("installed").is_some_and(is_truthy) || obj.get("bundled").is_some_and(is_truthy)
        {
            continue;
        }
        let id = obj.get("id").map_or_else(|| "undefined".into(), js_str);
        candidates.entry(id).or_default().push(skill.clone());
    }

    let mut merged: IndexMap<String, Value> = IndexMap::new();
    for (id, instances) in candidates {
        if instances.len() < 2 {
            continue;
        }
        let mut missing_locations = false;
        for item in &instances {
            let len = item
                .as_object()
                .and_then(|o| o.get("locations"))
                .and_then(Value::as_array)
                .map_or(0, Vec::len);
            if len == 0 {
                missing_locations = true;
                break;
            }
        }
        if missing_locations {
            continue;
        }

        let mut owners: HashSet<String> = HashSet::new();
        let mut overlaps = false;
        for item in &instances {
            let locs = item
                .as_object()
                .and_then(|o| o.get("locations"))
                .and_then(Value::as_array);
            let mut item_agents: Vec<String> = Vec::new();
            if let Some(locs) = locs {
                for loc in locs {
                    let loc_obj = require_obj(loc, "rootKind")?;
                    let rk = loc_obj
                        .get("rootKind")
                        .map_or_else(|| "undefined".into(), js_str);
                    let agent = if rk == "shared" { "codex".into() } else { rk };
                    if !item_agents.contains(&agent) {
                        item_agents.push(agent);
                    }
                }
            }
            if item_agents.iter().any(|a| owners.contains(a)) {
                overlaps = true;
            }
            owners.extend(item_agents);
        }

        let mut sources: HashSet<String> = HashSet::new();
        for item in &instances {
            let repo_key = item
                .as_object()
                .and_then(|o| o.get("repoKey"))
                .map_or_else(|| "undefined".into(), js_str);
            if parse_repo_name(&repo_key).is_some() {
                sources.insert(repo_key);
            }
        }
        if overlaps || sources.len() > 1 {
            continue;
        }

        let mut agents: Vec<String> = Vec::new();
        let mut all_locations: Vec<Value> = Vec::new();
        let mut safe_disable_agents: Vec<Value> = Vec::new();
        for item in &instances {
            let item_obj = item.as_object();
            match item_obj
                .and_then(|o| o.get("agents"))
                .filter(|v| !is_nil(v))
            {
                Some(Value::Array(arr)) => {
                    for a in arr {
                        let s = js_str(a);
                        if !agents.contains(&s) {
                            agents.push(s);
                        }
                    }
                }
                _ => {
                    if item_obj
                        .and_then(|o| o.get("enabled"))
                        .is_some_and(is_truthy)
                        && !agents.iter().any(|a| a == "codex")
                    {
                        agents.push("codex".into());
                    }
                }
            }
            if let Some(Value::Array(locs)) = item_obj.and_then(|o| o.get("locations")) {
                all_locations.extend(locs.iter().cloned());
            }
            if let Some(Value::Array(safe)) = item_obj.and_then(|o| o.get("safeDisableAgents")) {
                for s in safe {
                    if !safe_disable_agents.contains(s) {
                        safe_disable_agents.push(s.clone());
                    }
                }
            }
        }

        let mut base = instances[0].as_object().cloned().unwrap_or_default();
        let enabled = agents.iter().any(|a| a == "codex");
        base.insert("repoKey".into(), Value::String(String::new()));
        base.insert("repoPath".into(), Value::String(String::new()));
        base.insert("originalDir".into(), Value::Null);
        base.insert("updateOwner".into(), Value::String(String::new()));
        base.insert("updateAvailable".into(), Value::Bool(false));
        base.insert(
            "agents".into(),
            Value::Array(agents.into_iter().map(Value::String).collect()),
        );
        base.insert("enabled".into(), Value::Bool(enabled));
        base.insert("locations".into(), Value::Array(all_locations));
        base.insert(
            "safeDisableAgents".into(),
            Value::Array(safe_disable_agents),
        );
        base.insert("displayInstances".into(), Value::Array(instances));
        merged.insert(id, Value::Object(base));
    }

    let mut emitted: HashSet<String> = HashSet::new();
    let mut out = Vec::with_capacity(skills.len());
    for skill in skills {
        let obj = skill.as_object();
        let eligible = obj.is_some_and(|o| {
            o.get("installed").is_some_and(is_truthy) && !o.get("bundled").is_some_and(is_truthy)
        });
        let id = obj
            .and_then(|o| o.get("id"))
            .map_or_else(|| "undefined".into(), js_str);
        if eligible && let Some(item) = merged.get(&id) {
            if emitted.insert(id) {
                out.push(item.clone());
            }
        } else {
            out.push(skill);
        }
    }
    Ok(out)
}

fn iter_js_items(v: &Value, name: &str) -> Result<Vec<Value>, String> {
    if let Value::Array(items) = v {
        return Ok(items.clone());
    }
    js::string_units(v)
        .map(|us| us.into_iter().map(|u| js::utf16_string(&[u])).collect())
        .ok_or_else(|| format!("TypeError: {name} is not iterable"))
}

/// Port of `migrateSkillInstanceAssignments` (`src/skill-groups.ts:89`).
pub fn migrate_skill_instance_assignments(
    assignments_by_identity: Value,
    assignments: Value,
    skills: Value,
    custom_names: Vec<String>,
) -> Result<Value, String> {
    let items = iter_js_items(&skills, "skills")?;
    let mut migrated_opt: Option<Map<String, Value>> = None;

    for skill in &items {
        let obj = require_obj(skill, "bundled")?;
        if obj.get("bundled").is_some_and(is_truthy) {
            continue;
        }
        let Some(inst) = obj.get("instanceKey").filter(|v| is_truthy(v)) else {
            continue;
        };
        let undef = js::undefined();
        let id_val = obj.get("id").unwrap_or(&undef);
        if inst == id_val {
            continue;
        }

        let orig_map = require_has_own_target(&assignments_by_identity, "assignmentsByIdentity")?;
        let key = skill_group_identity_key_str(skill)?;
        let current_map = migrated_opt.as_ref().unwrap_or(orig_map);
        if current_map.contains_key(&key) {
            continue;
        }

        let id_str = js_str(id_val);
        if obj.get("conflictKind").and_then(Value::as_str) == Some("equal-independent") {
            let agg = aggregate_legacy_assignment(skill, current_map)?;
            match agg {
                Some(Some(val)) => {
                    let target = migrated_opt.get_or_insert_with(|| orig_map.clone());
                    target.insert(key, Value::String(val));
                }
                Some(None) => {}
                None => {
                    let assign_obj = require_has_own_target(&assignments, "assignments")?;
                    if let Some(legacy_val) = assign_obj.get(&id_str) {
                        let legacy = match legacy_val {
                            Value::String(s) => s.clone(),
                            _ => return Err("TypeError: value.startsWith is not a function".into()),
                        };
                        let name = legacy_custom_name(&legacy, &custom_names);
                        let next_val = match name {
                            None => legacy,
                            Some(n) => format!("custom:{n}"),
                        };
                        let target = migrated_opt.get_or_insert_with(|| orig_map.clone());
                        target.insert(key, Value::String(next_val));
                    }
                }
            }
            continue;
        }

        let repo_ref = obj.get("repoKey").unwrap_or(&undef);
        let mut previous_keys = vec![
            js_json_array(&[repo_ref, id_val, inst])?,
            js_json_array(&[&Value::String(String::new()), id_val, inst])?,
        ];
        let has_conflict = obj.get("conflict").is_some_and(is_truthy);
        if !has_conflict {
            previous_keys.push(js_json_array(&[repo_ref, id_val])?);
        }

        let found_prev = previous_keys
            .iter()
            .find_map(|pk| current_map.get(pk).cloned());
        let next_val: Value = if let Some(v) = found_prev {
            v
        } else if !has_conflict {
            let assign_obj = require_has_own_target(&assignments, "assignments")?;
            if let Some(legacy_val) = assign_obj.get(&id_str) {
                let legacy = match legacy_val {
                    Value::String(s) => s.clone(),
                    _ => return Err("TypeError: value.startsWith is not a function".into()),
                };
                let name = legacy_custom_name(&legacy, &custom_names);
                Value::String(match name {
                    None => legacy,
                    Some(n) => format!("custom:{n}"),
                })
            } else {
                continue;
            }
        } else {
            continue;
        };

        let target = migrated_opt.get_or_insert_with(|| orig_map.clone());
        target.insert(key, next_val);
    }

    Ok(match migrated_opt {
        Some(m) => Value::Object(m),
        None => assignments_by_identity,
    })
}

fn spread_js_object(v: &Value) -> Map<String, Value> {
    if let Value::String(s) = v {
        let mut m = Map::new();
        for (i, u) in s.encode_utf16().enumerate() {
            m.insert(i.to_string(), js::utf16_string(&[u]));
        }
        return m;
    }
    spread_object(v)
}

/// Port of `migrateSkillGroupAssignments` (`src/skill-groups.ts:132`).
pub fn migrate_skill_group_assignments(
    assignments: Value,
    changes: Value,
) -> Result<Value, String> {
    let mut migrated = spread_js_object(&assignments);
    for change in &iter_js_items(&changes, "changes")? {
        let obj = require_obj(change, "id")?;
        let undef = js::undefined();
        let id_ref = obj.get("id").unwrap_or(&undef);
        let from_repo = obj.get("fromRepoKey").unwrap_or(&undef);
        let to_repo = obj.get("toRepoKey").unwrap_or(&undef);
        let from = js_json_array(&[from_repo, id_ref])?;
        let to = js_json_array(&[to_repo, id_ref])?;
        if from == to || !migrated.contains_key(&from) {
            continue;
        }
        if !migrated.contains_key(&to)
            && let Some(val) = migrated.get(&from).cloned()
        {
            migrated.insert(to, val);
        }
        migrated.shift_remove(&from);
    }
    Ok(Value::Object(migrated))
}

/// Port of `skillRepositoryName` (`src/skill-groups.ts:149`).
pub fn skill_repository_name(repo_key: String) -> Result<Value, String> {
    Ok(parse_repo_name(&repo_key).map_or(Value::Null, |s| Value::String(s.to_string())))
}

/// Port of `defaultSkillGroupKey` (`src/skill-groups.ts:154`).
pub fn default_skill_group_key(skill: Value) -> Result<String, String> {
    let obj = require_obj(&skill, "bundled")?;
    if obj.get("bundled").is_some_and(is_truthy) {
        return Ok(BUILTIN_SKILL_GROUP.into());
    }
    let repo_key = obj
        .get("repoKey")
        .map_or_else(|| "undefined".into(), js_str);
    let display_key = obj
        .get("displayGroupRepoKey")
        .filter(|v| is_truthy(v))
        .map(js_str)
        .unwrap_or_default();
    let repo = parse_repo_name(&repo_key).or_else(|| parse_repo_name(&display_key));
    Ok(match repo {
        Some(r) => format!("repo:{r}"),
        None => USER_SKILL_GROUP.into(),
    })
}

fn legacy_custom_name(value: &str, custom_names: &[String]) -> Option<String> {
    if custom_names.iter().any(|n| n == value) {
        return Some(value.to_string());
    }
    let name = value.strip_prefix("custom:").unwrap_or("");
    custom_names.iter().find(|n| n.as_str() == name).cloned()
}

/// Port of `legacySkillCustomGroupName` (`src/skill-groups.ts:160`).
pub fn legacy_skill_custom_group_name(
    value: String,
    custom_names: Vec<String>,
) -> Result<Value, String> {
    Ok(legacy_custom_name(&value, &custom_names).map_or(Value::Null, Value::String))
}

/// Port of `skillGroupAssignment` (`src/skill-groups.ts:166`).
pub fn skill_group_assignment(
    skill: Value,
    custom_names: Vec<String>,
    assignments: Value,
    assignments_by_identity: Value,
) -> Result<String, String> {
    let obj = require_obj(&skill, "conflictKind")?;
    let key = skill_group_identity_key_str(&skill)?;
    let by_id = require_has_own_target(&assignments_by_identity, "assignmentsByIdentity")?;
    if let Some(val) = by_id.get(&key) {
        return Ok(js_str(val));
    }
    let conflict_kind = obj.get("conflictKind").and_then(Value::as_str);
    if conflict_kind == Some("equal-independent")
        && let Some(opt_val) = aggregate_legacy_assignment(&skill, by_id)?
    {
        return Ok(opt_val.unwrap_or_default());
    }
    if obj.get("conflict").is_some_and(is_truthy) && conflict_kind != Some("equal-independent") {
        return Ok(String::new());
    }
    let undef = js::undefined();
    let repo_ref = obj.get("repoKey").unwrap_or(&undef);
    let id_ref = obj.get("id").unwrap_or(&undef);
    let previous_key = js_json_array(&[repo_ref, id_ref])?;
    if let Some(val) = by_id.get(&previous_key) {
        return Ok(js_str(val));
    }
    let assign_obj = require_has_own_target(&assignments, "assignments")?;
    let id_str = js_str(id_ref);
    let legacy = match assign_obj.get(&id_str) {
        Some(Value::String(s)) => s.clone(),
        Some(v) if !is_nil(v) => return Err("TypeError: value.startsWith is not a function".into()),
        _ => String::new(),
    };
    let custom_name = legacy_custom_name(&legacy, &custom_names);
    Ok(match custom_name {
        None => legacy,
        Some(n) => format!("custom:{n}"),
    })
}

/// Port of `readSkillGroupStorage` (`src/skill-groups.ts:188`).
pub fn read_skill_group_storage(value: Option<String>) -> Result<Value, String> {
    let raw = value.unwrap_or_default();
    let text = if raw.is_empty() { "{}" } else { raw.as_str() };
    let stored: Map<String, Value> = match serde_json::from_str::<Value>(text) {
        Ok(Value::Object(map)) => map,
        _ => Map::new(),
    };

    let names = match stored.get("names") {
        Some(Value::Array(arr)) => Value::Array(
            arr.iter()
                .filter_map(|v| match v {
                    Value::String(s) if !js_trim(s).is_empty() => Some(Value::String(s.clone())),
                    _ => None,
                })
                .collect(),
        ),
        _ => Value::Array(Vec::new()),
    };
    let order = match stored.get("order") {
        Some(Value::Array(arr)) => Value::Array(
            arr.iter()
                .filter_map(|v| match v {
                    Value::String(s) => Some(Value::String(s.clone())),
                    _ => None,
                })
                .collect(),
        ),
        _ => Value::Array(Vec::new()),
    };
    let sort_direction = match stored.get("sortDirection").and_then(Value::as_str) {
        Some("asc") => Value::String("asc".into()),
        Some("desc") => Value::String("desc".into()),
        _ => Value::Null,
    };

    let mut out = stored.clone();
    out.insert("names".into(), names);
    out.insert(
        "assignments".into(),
        Value::Object(string_record(stored.get("assignments"))),
    );
    out.insert(
        "assignmentsByIdentity".into(),
        Value::Object(string_record(stored.get("assignmentsByIdentity"))),
    );
    out.insert("order".into(), order);
    out.insert("sortDirection".into(), sort_direction);
    out.insert(
        "aliases".into(),
        Value::Object(string_record(stored.get("aliases"))),
    );
    Ok(Value::Object(out))
}

/// Port of `sortSkillGroupsByName` (`src/skill-groups.ts:269`).
pub fn sort_skill_groups_by_name(
    groups: Vec<Value>,
    direction: String,
) -> Result<Vec<Value>, String> {
    let mut pinned = Vec::new();
    let mut movable = Vec::new();
    for group in groups {
        let obj = require_obj(&group, "kind")?;
        let kind = obj.get("kind").and_then(Value::as_str);
        if matches!(kind, Some("builtin") | Some("user")) {
            pinned.push(group);
        } else if matches!(kind, Some("repo") | Some("custom")) {
            movable.push(group);
        }
    }
    for group in &movable {
        if !matches!(group.get("label"), Some(Value::String(_))) {
            return Err("TypeError: first.label.localeCompare is not a function".into());
        }
    }
    let asc = direction == "asc";
    movable.sort_by(|first, second| {
        let fl = first.get("label").and_then(Value::as_str).unwrap_or("");
        let sl = second.get("label").and_then(Value::as_str).unwrap_or("");
        let ord = locale_compare_default(fl, sl);
        if asc { ord } else { ord.reverse() }
    });
    pinned.extend(movable);
    Ok(pinned)
}

/// Port of `sortSkillGroupKeys` (`src/skill-groups.ts:276`).
pub fn sort_skill_group_keys(groups: Vec<Value>, direction: String) -> Result<Vec<String>, String> {
    let sorted = sort_skill_groups_by_name(groups, direction)?;
    Ok(sorted
        .into_iter()
        .filter(|g| {
            matches!(
                g.get("kind").and_then(Value::as_str),
                Some("repo") | Some("custom")
            )
        })
        .map(|g| {
            g.as_object()
                .and_then(|o| o.get("key"))
                .map_or_else(|| "undefined".into(), js_str)
        })
        .collect())
}

/// Port of `nextSkillGroupSortDirection` (`src/skill-groups.ts:281`).
pub fn next_skill_group_sort_direction(direction: Option<String>) -> Result<String, String> {
    Ok(if direction.as_deref() == Some("asc") {
        "desc".into()
    } else {
        "asc".into()
    })
}

/// Port of `moveSkillGroup` (`src/skill-groups.ts:285`).
pub fn move_skill_group(
    groups: Vec<Value>,
    from: String,
    to: String,
) -> Result<Vec<String>, String> {
    let mut movable = Vec::new();
    for group in &groups {
        let obj = require_obj(group, "kind")?;
        if matches!(
            obj.get("kind").and_then(Value::as_str),
            Some("repo") | Some("custom")
        ) {
            let key = obj.get("key").map_or_else(|| "undefined".into(), js_str);
            movable.push(key);
        }
    }
    let Some(source) = movable.iter().position(|k| k == &from) else {
        return Ok(movable);
    };
    let Some(destination) = movable.iter().position(|k| k == &to) else {
        return Ok(movable);
    };
    if source == destination {
        return Ok(movable);
    }
    let item = movable.remove(source);
    movable.insert(destination, item);
    Ok(movable)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn builds_keys_and_migrates_assignments() {
        let first = json!({"id": "b:c", "repoKey": "a", "bundled": false});
        let second = json!({"id": "c", "repoKey": "a:b", "bundled": false});
        assert_ne!(
            skill_identity_key(first.clone()).ok(),
            skill_identity_key(second).ok()
        );

        let first_key = skill_identity_key(first.clone())
            .ok()
            .and_then(|v| v.as_str().map(str::to_string))
            .unwrap_or_default();
        assert_eq!(
            skill_group_assignment(
                first,
                vec!["Legacy".into(), "Other".into()],
                json!({"__proto__": "Legacy"}),
                json!({ first_key: "custom:Other" }),
            )
            .ok()
            .as_deref(),
            Some("custom:Other")
        );
        assert_eq!(
            skill_repository_name("owner/repo@main:skills".into()).ok(),
            Some(Value::String("owner/repo".into()))
        );
        assert_eq!(
            skill_repository_name("local".into()).ok(),
            Some(Value::Null)
        );
        assert_eq!(
            legacy_skill_custom_group_name("nope".into(), vec![]).ok(),
            Some(Value::Null)
        );
        assert_eq!(
            migrate_skill_group_assignments(json!({"a": "b"}), json!("x")).ok(),
            Some(json!({"a": "b"}))
        );
        assert!(
            migrate_skill_instance_assignments(json!({}), json!({}), json!(0), vec![]).is_err()
        );
    }

    #[test]
    fn reads_storage_and_sorts_groups() {
        let restored = read_skill_group_storage(Some(
            r#"{"names":["Empty","",4],"assignments":{"id":"Empty","bad":3},"order":["custom:Empty",false],"sortDirection":"desc","future":{"keep":true}}"#.into(),
        ))
        .ok()
        .unwrap_or_default();
        assert_eq!(restored["names"], json!(["Empty"]));
        assert_eq!(restored["sortDirection"], json!("desc"));
        assert_eq!(restored["future"], json!({"keep": true}));

        let groups = vec![
            json!({"key": "builtin", "label": "Built-in", "kind": "builtin", "items": []}),
            json!({"key": "user", "label": "User", "kind": "user", "items": []}),
            json!({"key": "repo:z/repo", "label": "z/repo", "kind": "repo", "items": []}),
            json!({"key": "custom:middle", "label": "middle", "kind": "custom", "items": []}),
            json!({"key": "repo:a/repo", "label": "a/repo", "kind": "repo", "items": []}),
        ];
        assert_eq!(
            sort_skill_group_keys(groups.clone(), "asc".into()).ok(),
            Some(vec![
                "repo:a/repo".into(),
                "custom:middle".into(),
                "repo:z/repo".into()
            ])
        );
        assert_eq!(
            move_skill_group(groups, "repo:a/repo".into(), "repo:z/repo".into()).ok(),
            Some(vec![
                "repo:a/repo".into(),
                "repo:z/repo".into(),
                "custom:middle".into(),
            ])
        );
    }
}
