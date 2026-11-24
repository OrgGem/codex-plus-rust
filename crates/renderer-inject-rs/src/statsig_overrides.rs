//! 1:1 port of pure override logic in `assets/inject/renderer-inject/34-statsig-overrides.js`.

#![forbid(unsafe_code)]

use indexmap::IndexMap;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};
use std::sync::OnceLock;

const GATE_LOG_LIMIT: usize = 500;

fn gate_id_pattern() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"^\d{1,12}$").expect("gate id regex must compile"))
}

fn value_id_str(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => String::new(),
        other => other.to_string(),
    }
}

/// Filters `config.gateCatalog` to valid objects carrying a 1..12 digit numeric ID.
pub fn statsig_gate_catalog(config: &Value) -> Vec<Value> {
    let Some(items) = config.get("gateCatalog").and_then(Value::as_array) else {
        return Vec::new();
    };
    items
        .iter()
        .filter(|entry| {
            entry.is_object()
                && gate_id_pattern()
                    .is_match(&value_id_str(entry.get("id").unwrap_or(&Value::Null)))
        })
        .cloned()
        .collect()
}

/// Extracts `codexAppStatsigGateOverrides` keeping only numeric IDs with `"on"|"off"|"native"`.
pub fn statsig_user_gate_overrides(backend_settings: &Value) -> IndexMap<String, String> {
    let mut out = IndexMap::new();
    let Some(raw) = backend_settings
        .get("codexAppStatsigGateOverrides")
        .and_then(Value::as_object)
    else {
        return out;
    };
    for (id, choice) in raw {
        if let Some(choice_str) = choice.as_str()
            && gate_id_pattern().is_match(id)
            && matches!(choice_str, "on" | "off" | "native")
        {
            out.insert(id.clone(), choice_str.to_string());
        }
    }
    out
}

/// Computes effective boolean gates map (`defaults` overlaid by `user` choices).
pub fn statsig_effective_gates(
    defaults: &Value,
    user: &IndexMap<String, String>,
) -> IndexMap<String, bool> {
    let mut effective = IndexMap::new();
    if let Some(obj) = defaults.as_object() {
        for (k, v) in obj {
            effective.insert(k.clone(), v == &Value::Bool(true));
        }
    }
    for (id, choice) in user {
        match choice.as_str() {
            "on" => {
                effective.insert(id.clone(), true);
            }
            "off" => {
                effective.insert(id.clone(), false);
            }
            "native" => {
                effective.shift_remove(id);
            }
            _ => {}
        }
    }
    effective
}

/// Clones a Statsig dynamicConfig/experiment/layer object and merges listed override keys
/// without mutating the memoized input (`codexPlusStatsigMergeObjectValue`).
pub fn statsig_merge_object_value(target: &Value, overrides: &Value) -> Value {
    let Some(target_obj) = target.as_object() else {
        return target.clone();
    };
    let Some(ov_obj) = overrides.as_object() else {
        return target.clone();
    };

    let base_map = target_obj
        .get("value")
        .and_then(Value::as_object)
        .or_else(|| target_obj.get("__value").and_then(Value::as_object));
    let Some(base) = base_map else {
        return target.clone();
    };

    let mut next_val = base.clone();
    for (k, ov) in ov_obj {
        if let Some(ov_map) = ov.as_object()
            && let Some(cur_map) = next_val.get(k).and_then(Value::as_object)
        {
            let mut nested = cur_map.clone();
            for (nk, nv) in ov_map {
                nested.insert(nk.clone(), nv.clone());
            }
            next_val.insert(k.clone(), Value::Object(nested));
        } else {
            next_val.insert(k.clone(), ov.clone());
        }
    }

    let next_val_enum = Value::Object(next_val);
    let mut result = target_obj.clone();
    result.insert("value".into(), next_val_enum.clone());
    if result.contains_key("__value") {
        result.insert("__value".into(), next_val_enum.clone());
    }
    if let Some(eval_obj) = result.get("__evaluation").and_then(Value::as_object) {
        let mut next_eval = eval_obj.clone();
        next_eval.insert("value".into(), next_val_enum);
        next_eval.insert("is_user_in_experiment".into(), Value::Bool(true));
        next_eval.insert("is_experiment_active".into(), Value::Bool(true));
        result.insert("__evaluation".into(), Value::Object(next_eval));
    }
    Value::Object(result)
}

/// Computes canonical signature matching `codexPlusStatsigOverridesSignature`.
pub fn statsig_overrides_signature(
    enabled: bool,
    config: &Value,
    backend_settings: &Value,
) -> String {
    if !enabled {
        return String::new();
    }
    let user = statsig_user_gate_overrides(backend_settings);
    let gates = statsig_effective_gates(config.get("gates").unwrap_or(&Value::Null), &user);
    let mut gate_pairs: Vec<(&str, Value)> = gates
        .iter()
        .map(|(k, v)| (k.as_str(), Value::Bool(*v)))
        .collect();
    gate_pairs.sort_by(|a, b| a.0.cmp(b.0));

    let sorted_section = |name: &str| -> Vec<(String, Value)> {
        let mut pairs: Vec<(String, Value)> = config
            .get(name)
            .and_then(Value::as_object)
            .map(|m| m.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
            .unwrap_or_default();
        pairs.sort_by(|a, b| a.0.cmp(&b.0));
        pairs
    };

    let serialize_pairs = |pairs: &[(String, Value)]| -> Value {
        Value::Array(pairs.iter().map(|(k, v)| json!([k, v])).collect())
    };
    let gate_json = Value::Array(gate_pairs.iter().map(|(k, v)| json!([k, v])).collect());

    Value::Array(vec![
        gate_json,
        serialize_pairs(&sorted_section("dynamicConfigs")),
        serialize_pairs(&sorted_section("experiments")),
        serialize_pairs(&sorted_section("layers")),
    ])
    .to_string()
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatsigGateLogEntry {
    pub id: String,
    pub native: Option<bool>,
    pub count: u64,
    pub last_at: u64,
}

/// Bounded insertion-order LRU recorder (`limit = 500`) mirroring `codexPlusStatsigRecordGate`.
#[derive(Debug, Default)]
pub struct StatsigGateRecorder {
    entries: IndexMap<String, StatsigGateLogEntry>,
}

impl StatsigGateRecorder {
    pub fn record(&mut self, id: &str, native_value: Option<bool>, now_ms: u64) -> bool {
        if !gate_id_pattern().is_match(id) {
            return false;
        }
        let mut entry = self
            .entries
            .shift_remove(id)
            .unwrap_or(StatsigGateLogEntry {
                id: id.to_string(),
                native: None,
                count: 0,
                last_at: 0,
            });
        if let Some(val) = native_value {
            entry.native = Some(val);
        }
        entry.count += 1;
        entry.last_at = now_ms;
        self.entries.insert(id.to_string(), entry);
        while self.entries.len() > GATE_LOG_LIMIT {
            self.entries.shift_remove_index(0);
        }
        true
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn snapshot(&self) -> Vec<&StatsigGateLogEntry> {
        self.entries.values().collect()
    }

    pub fn to_json_map(&self) -> Map<String, Value> {
        let mut map = Map::new();
        for (k, v) in &self.entries {
            if let Ok(val) = serde_json::to_value(v) {
                map.insert(k.clone(), val);
            }
        }
        map
    }
}
