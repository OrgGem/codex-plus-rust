//! The Rust port of `src/skill-agent-batch.ts`.

#![forbid(unsafe_code)]

use crate::js;
use manager_common_rs::{is_nil, is_truthy, js_str};
use serde_json::{Map, Value, json};

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

fn raw_agents(skill: &Value) -> Result<Vec<Value>, String> {
    let obj = require_obj(skill, "agents")?;
    match obj.get("agents") {
        Some(v) if !is_nil(v) => match v {
            Value::Array(arr) => Ok(arr.clone()),
            _ => Err("TypeError: skill.agents is not iterable".into()),
        },
        _ if obj.get("enabled").is_some_and(is_truthy) => Ok(vec![Value::String("codex".into())]),
        _ => Ok(Vec::new()),
    }
}

fn has_agent(agents: &[Value], target: &str) -> bool {
    agents
        .iter()
        .any(|v| v.as_str().is_some_and(|s| s == target))
}

fn is_eligible(skill: &Value, agent: &str) -> Result<bool, String> {
    let obj = require_obj(skill, "installed")?;
    let ok = obj.get("installed").is_some_and(is_truthy)
        && !obj.get("bundled").is_some_and(is_truthy)
        && !obj.get("conflict").is_some_and(is_truthy)
        && (agent != "codex" || skill_codex_status_error(skill.clone())?.is_none());
    Ok(ok)
}

/// Port of `skillAgents` (`src/skill-agent-batch.ts:112`).
pub fn skill_agents(skill: Value) -> Result<Vec<String>, String> {
    Ok(raw_agents(&skill)?.iter().map(js_str).collect())
}

/// Port of `skillCodexStatusError` (`src/skill-agent-batch.ts:116`).
pub fn skill_codex_status_error(skill: Value) -> Result<Option<String>, String> {
    let obj = require_obj(&skill, "locations")?;
    let Some(locations) = obj.get("locations").filter(|v| !is_nil(v)) else {
        return Ok(None);
    };
    let Value::Array(arr) = locations else {
        return Err("TypeError: skill.locations.find is not a function".into());
    };
    for loc in arr {
        if let Some(err) = require_obj(loc, "statusError")?.get("statusError")
            && is_truthy(err)
        {
            return Ok(Some(js_str(err)));
        }
    }
    Ok(None)
}

/// Port of `skillBatchTargets` (`src/skill-agent-batch.ts:120`).
pub fn skill_batch_targets(
    skills: Vec<Value>,
    agent: String,
    enabled: bool,
) -> Result<Vec<Value>, String> {
    let mut out = Vec::new();
    for skill in skills {
        if is_eligible(&skill, &agent)? && has_agent(&raw_agents(&skill)?, &agent) != enabled {
            out.push(skill);
        }
    }
    Ok(out)
}

/// Port of `skillBatchState` (`src/skill-agent-batch.ts:125`).
pub fn skill_batch_state(skills: Vec<Value>, agent: String) -> Result<Value, String> {
    let (mut total, mut selected) = (0usize, 0usize);
    for skill in &skills {
        if is_eligible(skill, &agent)? {
            total += 1;
            if has_agent(&raw_agents(skill)?, &agent) {
                selected += 1;
            }
        }
    }
    Ok(
        json!({ "total": total, "checked": total > 0 && selected == total, "mixed": selected > 0 && selected < total }),
    )
}

fn same_vz(a: &Value, b: &Value) -> bool {
    match (js::as_number(a), js::as_number(b)) {
        (Some(na), Some(nb)) => (na.is_nan() && nb.is_nan()) || na == nb,
        _ => a == b,
    }
}

/// Port of `skillBatchHasPending` (`src/skill-agent-batch.ts:132`).
pub fn skill_batch_has_pending(
    operations: Vec<Value>,
    skills: Vec<Value>,
    agent: String,
) -> Result<bool, String> {
    let mut ids: Vec<Value> = Vec::with_capacity(skills.len());
    for skill in &skills {
        let id = require_obj(skill, "id")?
            .get("id")
            .cloned()
            .unwrap_or_else(js::undefined);
        if !ids.iter().any(|x| same_vz(x, &id)) {
            ids.push(id);
        }
    }
    for op in &operations {
        let op_obj = require_obj(op, "agent")?;
        if op_obj.get("agent").and_then(Value::as_str) != Some(agent.as_str()) {
            continue;
        }
        let Some(Value::Array(skill_ids)) = op_obj.get("skillIds") else {
            return Err("TypeError: operation.skillIds.some is not a function".into());
        };
        if skill_ids
            .iter()
            .any(|id| ids.iter().any(|k| same_vz(k, id)))
        {
            return Ok(true);
        }
    }
    Ok(false)
}

fn matches_installed(skill: &Value, id: &str) -> Result<bool, String> {
    let obj = require_obj(skill, "id")?;
    Ok(obj.get("id").and_then(Value::as_str) == Some(id)
        && obj.get("installed").is_some_and(is_truthy)
        && !obj.get("bundled").is_some_and(is_truthy))
}

/// Port of `mergeSkillAgentResult` (`src/skill-agent-batch.ts:138`).
pub fn merge_skill_agent_result(
    current: Vec<Value>,
    result: Vec<Value>,
    id: String,
    agent: String,
) -> Result<Vec<Value>, String> {
    let mut matches = Vec::new();
    for s in &result {
        if matches_installed(s, &id)? {
            matches.push(s);
        }
    }
    let mut existing = Vec::new();
    for s in &current {
        if matches_installed(s, &id)? {
            existing.push(s);
        }
    }
    if matches.len() != 1
        || existing.len() != 1
        || matches[0].get("conflict").is_some_and(is_truthy)
        || existing[0].get("conflict").is_some_and(is_truthy)
    {
        return Ok(current);
    }
    let enabled = has_agent(&raw_agents(matches[0])?, &agent);
    let mut out = Vec::with_capacity(current.len());
    for skill in current {
        if !matches_installed(&skill, &id)? {
            out.push(skill);
            continue;
        }
        let mut next: Vec<Value> = raw_agents(&skill)?
            .into_iter()
            .filter(|v| v.as_str() != Some(agent.as_str()))
            .collect();
        if enabled {
            next.push(Value::String(agent.clone()));
        }
        let mut obj = skill.as_object().cloned().unwrap_or_default();
        obj.insert("agents".into(), Value::Array(next));
        if agent == "codex" {
            obj.insert("enabled".into(), Value::Bool(enabled));
        } else if !obj.contains_key("enabled") {
            obj.insert("enabled".into(), js::undefined());
        }
        out.push(Value::Object(obj));
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn batch_targets_state_and_merge() {
        let items = vec![
            json!({"id": "a", "installed": true, "bundled": false, "enabled": false, "agents": ["claude"]}),
            json!({"id": "b", "installed": true, "bundled": false, "enabled": false, "agents": ["codex"]}),
        ];
        assert_eq!(
            skill_batch_state(items.clone(), "codex".into()).ok(),
            Some(json!({"total": 2, "checked": false, "mixed": true}))
        );
        let merged = merge_skill_agent_result(items, vec![json!({"id": "a", "installed": true, "bundled": false, "enabled": true, "agents": ["codex"]})], "a".into(), "codex".into()).ok().unwrap_or_default();
        assert_eq!(merged[0]["agents"], json!(["claude", "codex"]));
    }
}
