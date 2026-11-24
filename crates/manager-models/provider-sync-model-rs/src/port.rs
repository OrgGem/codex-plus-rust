//! The Rust port of `src/provider-sync-model.ts`.

#![forbid(unsafe_code)]

use crate::js::undefined;
use manager_common_rs::{as_number, is_nil, is_truthy, is_undefined, string_units, utf16_string};
use serde_json::{Value, json};

fn js_trim(s: &str) -> &str {
    s.trim_matches(|c: char| c != '\u{0085}' && (c.is_whitespace() || c == '\u{FEFF}'))
}

fn require_str(v: Option<&Value>) -> Result<String, String> {
    match v {
        Some(Value::String(s)) => Ok(s.clone()),
        Some(val) if let Some(u) = string_units(val) => Ok(String::from_utf16_lossy(&u)),
        Some(val) if is_nil(val) => {
            Err("TypeError: Cannot read properties of null/undefined".into())
        }
        None => Err("TypeError: Cannot read properties of undefined".into()),
        Some(_) => Err("TypeError: model.trim is not a function".into()),
    }
}

fn set_has(v: &Value, needle: &Value) -> Result<bool, String> {
    let set = v
        .as_object()
        .filter(|o| o.len() == 1)
        .and_then(|o| o.get("$set"))
        .and_then(Value::as_array)
        .ok_or("TypeError: set.has is not a function")?;
    Ok(set.iter().any(|item| {
        if let (Some(a), Some(b)) = (as_number(item), as_number(needle)) {
            (a.is_nan() && b.is_nan()) || a == b
        } else {
            (is_undefined(item) && is_undefined(needle)) || item == needle
        }
    }))
}

/// Port of `modelSlug` (`src/provider-sync-model.ts:18`).
pub fn model_slug(model: String) -> Result<String, String> {
    let t = js_trim(&model);
    let body = if t.ends_with(']')
        && let Some(i) = t.rfind('[')
        && !t[i + 1..t.len() - 1].contains(']')
    {
        &t[..i]
    } else {
        t
    };
    Ok(js_trim(body).to_lowercase())
}

/// Port of `sameModel` (`src/provider-sync-model.ts:22`).
pub fn same_model(left: String, right: String) -> Result<bool, String> {
    Ok(model_slug(left)? == model_slug(right)?)
}

/// Port of `syncOutcome` (`src/provider-sync-model.ts:28`).
pub fn sync_outcome(preview: Value, remove: Value, add: Value) -> Result<Value, String> {
    if is_nil(&preview) {
        return Err(
            "TypeError: Cannot read properties of null/undefined (reading 'current')".into(),
        );
    }
    let current = preview
        .get("current")
        .and_then(Value::as_array)
        .ok_or("TypeError: Cannot read properties of undefined (reading 'filter')")?;
    let mut models = Vec::with_capacity(current.len());
    for model in current {
        if !set_has(&remove, model)? {
            models.push(model.clone());
        }
    }
    let added_val = preview
        .get("added")
        .ok_or("TypeError: preview.added is not iterable")?;
    let owned_chars;
    let added: &[Value] = if let Some(arr) = added_val.as_array() {
        arr
    } else if let Some(units) = string_units(added_val) {
        owned_chars = units
            .into_iter()
            .map(|u| utf16_string(&[u]))
            .collect::<Vec<_>>();
        &owned_chars
    } else {
        return Err("TypeError: preview.added is not iterable".into());
    };
    for model in added {
        if set_has(&add, model)? {
            let mut dup = false;
            for item in &models {
                if same_model(require_str(Some(item))?, require_str(Some(model))?)? {
                    dup = true;
                    break;
                }
            }
            if !dup {
                models.push(model.clone());
            }
        }
    }
    let default_val = preview.get("defaultModel");
    let mut kept: Option<Value> = None;
    for model in &models {
        if same_model(require_str(Some(model))?, require_str(default_val)?)? {
            kept = Some(model.clone());
            break;
        }
    }
    let default_model = kept
        .as_ref()
        .filter(|v| !is_nil(v))
        .cloned()
        .or_else(|| models.first().filter(|v| !is_nil(v)).cloned())
        .unwrap_or_else(|| Value::String(String::new()));
    let default_replaced =
        default_val.is_some_and(is_truthy) && !kept.as_ref().is_some_and(is_truthy);
    Ok(
        json!({ "models": models, "defaultModel": default_model, "defaultReplaced": default_replaced }),
    )
}

/// Port of `pickerRows` (`src/provider-sync-model.ts:55`).
pub fn picker_rows(existing: Vec<String>, fetched: Vec<String>) -> Result<Vec<Value>, String> {
    let fetched_slugs: Vec<String> = fetched
        .iter()
        .map(|m| model_slug(m.clone()))
        .collect::<Result<_, _>>()?;
    let mut rows = Vec::with_capacity(existing.len() + fetched.len());
    let mut seen_slugs = Vec::with_capacity(existing.len() + fetched.len());
    for model in existing {
        let slug = model_slug(model.clone())?;
        let upstream = fetched_slugs.iter().any(|f| f == &slug);
        rows.push(json!({ "model": model, "inList": true, "upstream": upstream }));
        seen_slugs.push(slug);
    }
    for (model, slug) in fetched.into_iter().zip(fetched_slugs) {
        if !seen_slugs.iter().any(|s| s == &slug) {
            rows.push(json!({ "model": model, "inList": false, "upstream": true }));
            seen_slugs.push(slug);
        }
    }
    Ok(rows)
}

/// Port of `pickerOutcome` (`src/provider-sync-model.ts:64`).
pub fn picker_outcome(rows: Vec<Value>, selected: Value) -> Result<Value, String> {
    let (mut models, mut added, mut removed) = (Vec::new(), 0usize, 0usize);
    for row in rows {
        if is_nil(&row) {
            return Err(
                "TypeError: Cannot read properties of null/undefined (reading 'model')".into(),
            );
        }
        let model = row.get("model").cloned().unwrap_or_else(undefined);
        let in_list = row.get("inList").is_some_and(is_truthy);
        let picked = set_has(&selected, &model)?;
        if picked {
            models.push(model);
            if !in_list {
                added += 1;
            }
        } else if in_list {
            removed += 1;
        }
    }
    Ok(json!({ "models": models, "added": added, "removed": removed }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slug_sync_and_picker_match_ts_behavior() {
        assert_eq!(
            model_slug(" Claude-Opus-4-6[1M] ".into()).unwrap(),
            "claude-opus-4-6"
        );
        assert_eq!(model_slug("a[1] bar]".into()).unwrap(), "a[1] bar]");
        assert!(same_model("big[1M]".into(), "BIG".into()).unwrap());
        let preview = json!({
            "upstream": ["a", "b", "new-1", "new-2"], "current": ["b", "a", "old", "older"],
            "defaultModel": "b", "added": ["new-1", "new-2"], "removed": ["old", "older"],
        });
        assert_eq!(
            sync_outcome(
                preview,
                json!({ "$set": ["old"] }),
                json!({ "$set": ["new-2"] })
            )
            .unwrap(),
            json!({ "models": ["b", "a", "older", "new-2"], "defaultModel": "b", "defaultReplaced": false })
        );
        let rows = picker_rows(
            vec!["a".into(), "custom".into(), "big[1M]".into()],
            vec!["A".into(), "BIG".into(), "new-1".into()],
        )
        .unwrap();
        assert_eq!(rows.len(), 4);
        assert_eq!(
            picker_outcome(rows, json!({ "$set": ["a", "big[1M]", "new-1"] })).unwrap(),
            json!({ "models": ["a", "big[1M]", "new-1"], "added": 1, "removed": 1 })
        );
        assert!(sync_outcome(Value::Null, json!({ "$set": [] }), json!({ "$set": [] })).is_err());
    }
}
