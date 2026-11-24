//! The Rust port of `src/provider-pool-layout.ts`.

#![forbid(unsafe_code)]

use manager_common_rs::domain::es_trim;
use manager_common_rs::gateway_shared::provider_enabled;
use manager_common_rs::js_rt::{
    as_number, is_nil, is_plain_object, is_undefined, string_units, undefined, utf16_string,
};
use serde_json::{Value, json};
use std::sync::LazyLock;

static UNDEF: LazyLock<Value> = LazyLock::new(undefined);

fn field_or_undef<'a>(v: &'a Value, key: &str) -> &'a Value {
    v.as_object()
        .filter(|_| is_plain_object(v))
        .and_then(|m| m.get(key))
        .unwrap_or(&UNDEF)
}

fn require_str_lower(v: &Value, prop: &str) -> Result<String, String> {
    if is_nil(v) {
        return Err(
            "TypeError: Cannot read properties of null/undefined (reading 'toLowerCase')".into(),
        );
    }
    match v {
        Value::String(s) => Ok(s.to_lowercase()),
        _ => match string_units(v) {
            Some(units) => Ok(String::from_utf16_lossy(&units).to_lowercase()),
            None => Err(format!("TypeError: {prop}.toLowerCase is not a function")),
        },
    }
}

fn same_vz(a: &Value, b: &Value) -> bool {
    match (as_number(a), as_number(b)) {
        (Some(x), Some(y)) => (x.is_nan() && y.is_nan()) || x == y,
        _ => match (string_units(a), string_units(b)) {
            (Some(ua), Some(ub)) => ua == ub,
            _ => (is_undefined(a) && is_undefined(b)) || a == b,
        },
    }
}

fn set_contains(set: &[Value], item: &Value) -> bool {
    set.iter().any(|x| same_vz(x, item))
}

fn push_unique(set: &mut Vec<Value>, item: Value) {
    if !set_contains(set, &item) {
        set.push(item);
    }
}

fn utf16_codepoints(units: &[u16]) -> Vec<Value> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < units.len() {
        let u = units[i];
        if (0xD800..=0xDBFF).contains(&u)
            && i + 1 < units.len()
            && (0xDC00..=0xDFFF).contains(&units[i + 1])
        {
            out.push(utf16_string(&units[i..i + 2]));
            i += 2;
        } else {
            out.push(utf16_string(&[u]));
            i += 1;
        }
    }
    out
}

/// Port of `providerPoolSections` (`src/provider-pool-layout.ts:4`).
pub fn provider_pool_sections(pool: Value, query: String) -> Result<Value, String> {
    if is_nil(&pool) {
        return Err(
            "TypeError: Cannot read properties of null/undefined (reading 'filter')".into(),
        );
    }
    let arr = pool
        .as_array()
        .ok_or_else(|| "TypeError: pool.filter is not a function".to_string())?;
    let needle = es_trim(&query).to_lowercase();

    let mut visible: Vec<&Value> = Vec::new();
    for entry in arr {
        if needle.is_empty() {
            visible.push(entry);
            continue;
        }
        if is_nil(entry) {
            return Err(
                "TypeError: Cannot read properties of null/undefined (reading 'name')".into(),
            );
        }
        if require_str_lower(field_or_undef(entry, "name"), "entry.name")?.contains(&needle) {
            visible.push(entry);
            continue;
        }
        if require_str_lower(field_or_undef(entry, "baseUrl"), "entry.baseUrl")?.contains(&needle) {
            visible.push(entry);
            continue;
        }
        let models_val = field_or_undef(entry, "models");
        if is_nil(models_val) {
            return Err(
                "TypeError: Cannot read properties of null/undefined (reading 'some')".into(),
            );
        }
        let models = models_val
            .as_array()
            .ok_or_else(|| "TypeError: entry.models.some is not a function".to_string())?;
        let mut matched = false;
        for model in models {
            if require_str_lower(model, "model")?.contains(&needle) {
                matched = true;
                break;
            }
        }
        if matched {
            visible.push(entry);
        }
    }

    let mut active = Vec::new();
    let mut inactive = Vec::new();
    for entry in visible {
        if provider_enabled((*entry).clone())? {
            active.push((*entry).clone());
        } else {
            inactive.push((*entry).clone());
        }
    }

    Ok(json!({ "active": active, "inactive": inactive }))
}

/// Port of `reorderProviderSlots` (`src/provider-pool-layout.ts:15`).
pub fn reorder_provider_slots(order: Value, group_order: Value) -> Result<Value, String> {
    let mut members: Vec<Value> = Vec::new();
    if !is_nil(&group_order) {
        if let Some(arr) = group_order.as_array() {
            for item in arr {
                push_unique(&mut members, item.clone());
            }
        } else if let Some(units) = string_units(&group_order) {
            for ch in utf16_codepoints(&units) {
                push_unique(&mut members, ch);
            }
        } else if let Some(set_items) = group_order
            .as_object()
            .filter(|o| o.len() == 1 && o.contains_key("$set"))
            .and_then(|o| o.get("$set"))
            .and_then(Value::as_array)
        {
            for item in set_items {
                push_unique(&mut members, item.clone());
            }
        } else {
            return Err("TypeError: groupOrder is not iterable".into());
        }
    }

    if is_nil(&order) {
        return Err(
            "TypeError: Cannot read properties of null/undefined (reading 'filter')".into(),
        );
    }
    let order_arr = order
        .as_array()
        .ok_or_else(|| "TypeError: order.filter is not a function".to_string())?;

    let slots_len = order_arr
        .iter()
        .filter(|id| set_contains(&members, id))
        .count();

    if is_nil(&group_order) {
        return Err(
            "TypeError: Cannot read properties of null/undefined (reading 'length')".into(),
        );
    }

    let group_items: Vec<Value> = if let Some(arr) = group_order.as_array() {
        arr.clone()
    } else if let Some(units) = string_units(&group_order) {
        units.into_iter().map(|u| utf16_string(&[u])).collect()
    } else {
        return Ok(order);
    };

    if members.len() != group_items.len() || slots_len != group_items.len() {
        return Ok(order);
    }

    let mut index = 0usize;
    let mut out = Vec::with_capacity(order_arr.len());
    for id in order_arr {
        if set_contains(&members, id) {
            out.push(group_items[index].clone());
            index += 1;
        } else {
            out.push(id.clone());
        }
    }
    Ok(Value::Array(out))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(id: &str, enabled: Option<bool>) -> Value {
        let mut obj = json!({
            "id": id,
            "name": id,
            "baseUrl": format!("https://{id}.example.com/v1"),
            "protocol": "responses",
            "official": false,
            "models": [format!("{id}-model")],
            "keyCount": 1,
        });
        if let Some(en) = enabled {
            obj["enabled"] = json!(en);
        }
        obj
    }

    #[test]
    fn splits_and_filters_pool_sections() {
        let pool = json!([
            entry("alpha", None),
            entry("paused", Some(false)),
            entry("beta", Some(true)),
            entry("offline", Some(false)),
        ]);
        let all = provider_pool_sections(pool.clone(), "".into()).unwrap();
        assert_eq!(all["active"].as_array().unwrap().len(), 2);
        assert_eq!(all["active"][0]["id"], json!("alpha"));
        assert_eq!(all["active"][1]["id"], json!("beta"));
        assert_eq!(all["inactive"][0]["id"], json!("paused"));
        assert_eq!(all["inactive"][1]["id"], json!("offline"));

        for q in [" PAUSED ", "paused.example.com", "paused-model"] {
            let filtered = provider_pool_sections(pool.clone(), q.into()).unwrap();
            assert_eq!(filtered["active"].as_array().unwrap().len(), 0);
            assert_eq!(filtered["inactive"].as_array().unwrap().len(), 1);
            assert_eq!(filtered["inactive"][0]["id"], json!("paused"));
        }
        assert!(provider_pool_sections(Value::Null, "".into()).is_err());
    }

    #[test]
    fn reorders_provider_slots_in_place() {
        let order = json!(["alpha", "paused", "beta", "offline"]);
        assert_eq!(
            reorder_provider_slots(order.clone(), json!(["beta", "alpha"])).unwrap(),
            json!(["beta", "paused", "alpha", "offline"])
        );
        assert_eq!(
            reorder_provider_slots(order.clone(), json!(["offline", "paused"])).unwrap(),
            json!(["alpha", "offline", "beta", "paused"])
        );
        assert_eq!(
            reorder_provider_slots(order.clone(), json!([])).unwrap(),
            order
        );
        assert_eq!(
            reorder_provider_slots(order.clone(), json!(["alpha", "alpha"])).unwrap(),
            order
        );
        assert_eq!(
            reorder_provider_slots(order.clone(), json!(["missing"])).unwrap(),
            order
        );
        assert!(reorder_provider_slots(order, Value::Null).is_err());
    }
}
