//! The Rust port of `follower.mjs` (`applyPatches`).

#![forbid(unsafe_code)]

use crate::js;
use serde_json::{Value, json};

fn is_nil(v: &Value) -> bool {
    v.is_null() || js::is_undefined(v)
}

fn seg_str(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        v if js::is_undefined(v) => "undefined".into(),
        Value::Null => "null".into(),
        Value::Bool(b) => b.to_string(),
        v => js::as_number(v).map_or_else(|| "[object Object]".into(), js::number_to_string),
    }
}

fn arr_idx(v: &Value) -> Option<usize> {
    if let Some(n) = js::as_number(v).filter(|n| n.is_finite() && *n >= 0.0 && n.fract() == 0.0) {
        return Some(n as usize);
    }
    v.as_str()?.parse::<usize>().ok()
}

/// Port of `applyPatches` (`follower.mjs:9`).
pub fn apply_patches(mut state: Value, patches: Value) -> Result<Value, String> {
    let Value::Array(ops) = patches else {
        return Err("TypeError: patches is not iterable".into());
    };
    for patch in ops {
        if is_nil(&patch) {
            return Err("TypeError: Cannot destructure property 'op' of null/undefined".into());
        }
        let op = patch.get("op").and_then(Value::as_str).unwrap_or("");
        let Some(Value::Array(path)) = patch.get("path") else {
            return Err("TypeError: Cannot read properties of undefined (reading 'length')".into());
        };
        let value = patch.get("value").cloned().unwrap_or_else(js::undefined);
        if path.is_empty() {
            if op != "remove" {
                state = value;
            }
            continue;
        }
        let mut parent = &mut state;
        for i in 0..path.len() - 1 {
            let next_is_arr = js::as_number(&path[i + 1]).is_some();
            let fill = || if next_is_arr { json!([]) } else { json!({}) };
            match parent {
                Value::Array(a) => {
                    let idx = arr_idx(&path[i]).ok_or("TypeError: invalid array index")?;
                    while a.len() <= idx {
                        a.push(Value::Null);
                    }
                    if is_nil(&a[idx]) {
                        a[idx] = fill();
                    }
                    parent = &mut a[idx];
                }
                Value::Object(o) => {
                    let k = seg_str(&path[i]);
                    let entry = o.entry(k).or_insert_with(fill);
                    if is_nil(entry) {
                        *entry = fill();
                    }
                    parent = entry;
                }
                _ => return Err("TypeError: cannot traverse non-container".into()),
            }
        }
        let last = &path[path.len() - 1];
        match parent {
            Value::Array(a) => {
                if seg_str(last) == "length" && op != "remove" {
                    let n = js::as_number(&value).unwrap_or(0.0).max(0.0) as usize;
                    a.resize(n, Value::Null);
                } else {
                    let idx = arr_idx(last).unwrap_or(a.len());
                    if op == "remove" {
                        if idx < a.len() {
                            a.remove(idx);
                        }
                    } else if op == "add" {
                        let pos = idx.min(a.len());
                        a.insert(pos, value);
                    } else {
                        while a.len() <= idx {
                            a.push(Value::Null);
                        }
                        a[idx] = value;
                    }
                }
            }
            Value::Object(o) => {
                let k = seg_str(last);
                if op == "remove" {
                    o.shift_remove(&k);
                } else {
                    o.insert(k, value);
                }
            }
            _ => return Err("TypeError: cannot set property on non-container".into()),
        }
    }
    Ok(state)
}
