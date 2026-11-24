//! The Rust port of `src/enhancement-catalog.ts`.

#![forbid(unsafe_code)]

use manager_common_rs::{
    enhancement_settings_value, is_nil, is_plain_object, is_truthy, is_undefined, string_units,
    undefined,
};
use serde_json::{Value, json};
use std::collections::HashSet;

fn contains_ascii_word_ci(haystack: &str, word: &str) -> bool {
    let h = haystack.as_bytes();
    let w = word.as_bytes();
    if h.len() < w.len() {
        return false;
    }
    let is_word = |b: u8| b.is_ascii_alphanumeric() || b == b'_';
    for i in 0..=h.len() - w.len() {
        if h[i..i + w.len()].eq_ignore_ascii_case(w) {
            let left_ok = i == 0 || !is_word(h[i - 1]);
            let right_ok = i + w.len() == h.len() || !is_word(h[i + w.len()]);
            if left_ok && right_ok {
                return true;
            }
        }
    }
    false
}

fn resolve_catalog(catalog: &Value) -> Result<&Value, String> {
    if is_undefined(catalog) {
        Ok(enhancement_settings_value())
    } else if catalog.is_null() {
        Err("TypeError: Cannot read properties of null".into())
    } else {
        Ok(catalog)
    }
}

fn as_array_field<'a>(obj: &'a Value, field: &str) -> Result<&'a Vec<Value>, String> {
    if is_nil(obj) {
        return Err(format!(
            "TypeError: Cannot read properties of null/undefined (reading '{field}')"
        ));
    }
    obj.as_object()
        .filter(|_| is_plain_object(obj))
        .and_then(|o| o.get(field))
        .and_then(Value::as_array)
        .ok_or_else(|| {
            format!("TypeError: Cannot read properties of undefined (reading '{field}')")
        })
}

/// Port of `enhancementPlatformFromUserAgent` (`src/enhancement-catalog.ts:92`).
pub fn enhancement_platform_from_user_agent(user_agent: String) -> Result<String, String> {
    if contains_ascii_word_ci(&user_agent, "Windows") {
        Ok("windows".into())
    } else if contains_ascii_word_ci(&user_agent, "Macintosh") {
        Ok("macos".into())
    } else {
        Ok("linux".into())
    }
}

/// Port of `enhancementItem` (`src/enhancement-catalog.ts:98`).
pub fn enhancement_item(key: String, catalog: Value) -> Result<Value, String> {
    let cat = resolve_catalog(&catalog)?;
    let items = as_array_field(cat, "items")?;
    for candidate in items {
        if candidate.get("key").and_then(Value::as_str) == Some(key.as_str()) {
            return Ok(candidate.clone());
        }
    }
    Err(format!("enhancement catalog is missing {key}"))
}

/// Port of `enhancementGroup` (`src/enhancement-catalog.ts:104`).
pub fn enhancement_group(id: String, catalog: Value) -> Result<Value, String> {
    let cat = resolve_catalog(&catalog)?;
    let groups = as_array_field(cat, "groups")?;
    for candidate in groups {
        if candidate.get("id").and_then(Value::as_str) == Some(id.as_str()) {
            return Ok(candidate.clone());
        }
    }
    Err(format!("enhancement catalog is missing group {id}"))
}

fn value_includes_str(arr: &Value, target: &str) -> Result<bool, String> {
    let Some(list) = arr.as_array() else {
        return Err("TypeError: Cannot read properties of undefined (reading 'includes')".into());
    };
    Ok(list.iter().any(|x| x.as_str() == Some(target)))
}

/// Port of `isEnhancementItemVisible` (`src/enhancement-catalog.ts:110`).
pub fn is_enhancement_item_visible(
    item: Value,
    surface: String,
    platform: String,
) -> Result<bool, String> {
    if is_nil(&item) {
        return Err(
            "TypeError: Cannot read properties of null/undefined (reading 'surfaces')".into(),
        );
    }
    let obj = item.as_object().filter(|_| is_plain_object(&item));
    let surfaces = obj.and_then(|o| o.get("surfaces")).ok_or_else(|| {
        "TypeError: Cannot read properties of undefined (reading 'includes')".to_string()
    })?;
    if !value_includes_str(surfaces, &surface)? {
        return Ok(false);
    }
    match obj.and_then(|o| o.get("platforms")) {
        None => Ok(true),
        Some(p) if !is_truthy(p) => Ok(true),
        Some(p) => value_includes_str(p, &platform),
    }
}

/// Port of `managerBuiltinCards` (`src/enhancement-catalog.ts:130`).
pub fn manager_builtin_cards(
    group_id: String,
    platform: String,
    catalog: Value,
) -> Result<Vec<Value>, String> {
    let cat = resolve_catalog(&catalog)?;
    let items = as_array_field(cat, "items")?;
    let mut cards = Vec::new();

    let collect_params = |id: &str| -> Result<Vec<Value>, String> {
        let mut params = Vec::new();
        for item in items {
            if item.get("parent").and_then(Value::as_str) == Some(id)
                && is_enhancement_item_visible(item.clone(), "manager".into(), platform.clone())?
            {
                params.push(item.clone());
            }
        }
        Ok(params)
    };

    for item in items {
        let builtin = item.get("builtin");
        if builtin.is_some_and(is_truthy)
            && item.get("group").and_then(Value::as_str) == Some(group_id.as_str())
            && is_enhancement_item_visible(item.clone(), "manager".into(), platform.clone())?
        {
            let id = builtin.and_then(Value::as_str).unwrap_or("");
            let params = collect_params(id)?;
            cards.push(json!({ "id": builtin, "item": item, "params": params }));
        }
    }

    if let Some(builtins_val) = cat
        .as_object()
        .and_then(|o| o.get("builtins"))
        .filter(|v| !is_nil(v))
    {
        let Some(builtins) = builtins_val.as_array() else {
            return Err("TypeError: builtins.filter is not a function".into());
        };
        for entry in builtins {
            if is_nil(entry) {
                return Err(
                    "TypeError: Cannot read properties of null/undefined (reading 'group')".into(),
                );
            }
            if entry.get("group").and_then(Value::as_str) == Some(group_id.as_str()) {
                let id_val = entry.get("id").cloned().unwrap_or_else(undefined);
                let params = match id_val.as_str() {
                    Some(id) => collect_params(id)?,
                    None => Vec::new(),
                };
                cards.push(json!({ "id": id_val, "entry": entry, "params": params }));
            }
        }
    }

    Ok(cards)
}

/// Port of `builtinSettingValue` (`src/enhancement-catalog.ts:147`).
pub fn builtin_setting_value(settings: Value, path: String) -> Result<Value, String> {
    let mut current = settings;
    for key in path.split('.') {
        let next = match &current {
            Value::Object(map) if is_plain_object(&current) => map.get(key).cloned(),
            Value::Array(arr) => {
                if key == "length" {
                    Some(json!(arr.len()))
                } else {
                    key.parse::<usize>().ok().and_then(|i| arr.get(i).cloned())
                }
            }
            _ => None,
        };
        current = next.unwrap_or_else(undefined);
    }
    Ok(current)
}

/// Port of `managerStandaloneGroup` (`src/enhancement-catalog.ts:151`).
pub fn manager_standalone_group(catalog: Value) -> Result<Value, String> {
    let cat = resolve_catalog(&catalog)?;
    let groups = as_array_field(cat, "groups")?;
    for candidate in groups {
        if candidate.get("managerTab").and_then(Value::as_str) == Some("general")
            && candidate.get("standalone").is_some_and(is_truthy)
        {
            return Ok(candidate.clone());
        }
    }
    Err("enhancement catalog is missing the standalone group".into())
}

/// Port of `isEnhancementItemMasterGated` (`src/enhancement-catalog.ts:157`).
pub fn is_enhancement_item_master_gated(item: Value) -> Result<bool, String> {
    if is_nil(&item) {
        return Err(
            "TypeError: Cannot read properties of null/undefined (reading 'masterGated')".into(),
        );
    }
    Ok(item.get("masterGated") != Some(&Value::Bool(false)))
}

fn push_unique(seen: &mut HashSet<String>, out: &mut Vec<String>, val: Option<&Value>) {
    if let Some(v) = val.filter(|x| is_truthy(x)) {
        let s = match v {
            Value::String(s) => s.clone(),
            other => string_units(other).map_or_else(
                || manager_common_rs::js_str(other),
                |u| String::from_utf16_lossy(&u),
            ),
        };
        if seen.insert(s.clone()) {
            out.push(s);
        }
    }
}

/// Port of `enhancementCatalogStrings` (`src/enhancement-catalog.ts:162`).
pub fn enhancement_catalog_strings(catalog: Value) -> Result<Vec<String>, String> {
    let cat = resolve_catalog(&catalog)?;
    let groups = as_array_field(cat, "groups")?;
    let items = as_array_field(cat, "items")?;
    let mut seen = HashSet::new();
    let mut out = Vec::new();

    for group in groups {
        if is_nil(group) {
            return Err(
                "TypeError: Cannot read properties of null/undefined (reading 'title')".into(),
            );
        }
        push_unique(&mut seen, &mut out, group.get("title"));
        push_unique(&mut seen, &mut out, group.get("description"));
    }
    for item in items {
        if is_nil(item) {
            return Err(
                "TypeError: Cannot read properties of null/undefined (reading 'title')".into(),
            );
        }
        push_unique(&mut seen, &mut out, item.get("title"));
        push_unique(&mut seen, &mut out, item.get("description"));
        push_unique(
            &mut seen,
            &mut out,
            item.get("descriptionWhenEnhancementsOn"),
        );
        if let Some(opts) = item
            .get("options")
            .filter(|v| !is_nil(v))
            .and_then(Value::as_array)
        {
            for option in opts {
                if is_nil(option) {
                    return Err(
                        "TypeError: Cannot read properties of null/undefined (reading 'label')"
                            .into(),
                    );
                }
                push_unique(&mut seen, &mut out, option.get("label"));
            }
        }
    }
    if let Some(builtins) = cat
        .as_object()
        .and_then(|o| o.get("builtins"))
        .filter(|v| !is_nil(v))
    {
        let Some(list) = builtins.as_array() else {
            return Err("TypeError: builtins is not iterable".into());
        };
        for builtin in list {
            if is_nil(builtin) {
                return Err(
                    "TypeError: Cannot read properties of null/undefined (reading 'title')".into(),
                );
            }
            push_unique(&mut seen, &mut out, builtin.get("title"));
            push_unique(&mut seen, &mut out, builtin.get("description"));
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn platform_and_lookup_work() {
        assert_eq!(
            enhancement_platform_from_user_agent("Mozilla/5.0 (Windows NT 10.0)".into()).unwrap(),
            "windows"
        );
        assert_eq!(
            enhancement_platform_from_user_agent("Mozilla/5.0 (Macintosh; Intel)".into()).unwrap(),
            "macos"
        );
        assert_eq!(
            enhancement_platform_from_user_agent("mywindows".into()).unwrap(),
            "linux"
        );
        let item = enhancement_item("codexAppNativeFastUnlock".into(), undefined()).unwrap();
        assert_eq!(item["title"], "原生 Fast 强制解锁");
        assert!(enhancement_item("missing-x".into(), undefined()).is_err());
        assert_eq!(
            manager_standalone_group(undefined()).unwrap()["id"],
            "standalone"
        );
        assert_eq!(
            manager_builtin_cards("standalone".into(), "macos".into(), undefined())
                .unwrap()
                .len(),
            1
        );
        assert!(!enhancement_catalog_strings(undefined()).unwrap().is_empty());
    }
}
