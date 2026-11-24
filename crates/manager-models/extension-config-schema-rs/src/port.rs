//! The Rust port of `src/extension-config-schema.ts`.

#![forbid(unsafe_code)]

use manager_common_rs::{
    is_nil, is_plain_object, is_truthy, js_str, string_units, undefined, utf16_string,
};
use serde_json::{Map, Value, json};

fn obj_of(v: Option<&Value>) -> Option<&Map<String, Value>> {
    v.filter(|x| is_plain_object(x)).and_then(Value::as_object)
}

fn schemastery_field(key: &str, schema: Option<&Value>) -> Result<Option<Value>, String> {
    let Some(s) = schema.filter(|v| is_truthy(v)) else {
        return Ok(None);
    };
    let s_obj = obj_of(Some(s));
    let meta = obj_of(s_obj.and_then(|o| o.get("meta")).filter(|v| !is_nil(v)));
    if meta.and_then(|m| m.get("hidden")).is_some_and(is_truthy) {
        return Ok(None);
    }

    let mget = |k: &str| meta.and_then(|m| m.get(k));
    let mut out = Map::new();
    out.insert("key".into(), json!(key));
    out.insert("title".into(), json!(key));
    out.insert(
        "description".into(),
        mget("description")
            .filter(|v| !is_nil(v))
            .cloned()
            .unwrap_or_else(|| json!("")),
    );
    out.insert(
        "default".into(),
        mget("default").cloned().unwrap_or_else(undefined),
    );
    out.insert("options".into(), json!([]));
    out.insert("secret".into(), json!(false));

    match s_obj.and_then(|o| o.get("type")).and_then(Value::as_str) {
        Some("string") => {
            let role = mget("role").and_then(Value::as_str);
            out.insert("secret".into(), json!(role == Some("secret")));
            out.insert(
                "type".into(),
                json!(if role == Some("textarea") {
                    "text"
                } else {
                    "string"
                }),
            );
            Ok(Some(Value::Object(out)))
        }
        Some(ty @ ("number" | "natural" | "percent")) => {
            let min = mget("min")
                .filter(|v| !is_nil(v))
                .cloned()
                .unwrap_or_else(|| {
                    if ty == "natural" {
                        json!(0)
                    } else {
                        undefined()
                    }
                });
            let max = mget("max")
                .filter(|v| !is_nil(v))
                .cloned()
                .unwrap_or_else(|| {
                    if ty == "percent" {
                        json!(1)
                    } else {
                        undefined()
                    }
                });
            out.insert("type".into(), json!("number"));
            out.insert("min".into(), min);
            out.insert("max".into(), max);
            Ok(Some(Value::Object(out)))
        }
        Some("boolean") => {
            out.insert("type".into(), json!("boolean"));
            Ok(Some(Value::Object(out)))
        }
        Some("union") => {
            let list = match s_obj.and_then(|o| o.get("list")).filter(|v| !is_nil(v)) {
                None => return Ok(None),
                Some(Value::Array(arr)) if !arr.is_empty() => arr,
                Some(Value::Array(_)) => return Ok(None),
                Some(_) => return Err("TypeError: list.every is not a function".into()),
            };
            let mut options = Vec::with_capacity(list.len());
            for opt in list {
                if is_nil(opt) {
                    return Err(
                        "TypeError: Cannot read properties of null/undefined (reading 'type')"
                            .into(),
                    );
                }
                if opt.get("type").and_then(Value::as_str) != Some("const") {
                    return Ok(None);
                }
                let val = opt.get("value").cloned().unwrap_or_else(undefined);
                let desc = opt
                    .get("meta")
                    .and_then(Value::as_object)
                    .and_then(|m| m.get("description"));
                let label = desc
                    .filter(|v| string_units(v).is_some())
                    .cloned()
                    .unwrap_or_else(|| json!(js_str(&val)));
                options.push(json!({ "value": val, "label": label }));
            }
            out.insert("type".into(), json!("select"));
            out.insert("options".into(), Value::Array(options));
            Ok(Some(Value::Object(out)))
        }
        Some("array") => {
            let Some(items) = schemastery_field("item", s_obj.and_then(|o| o.get("inner")))? else {
                return Ok(None);
            };
            if items.get("type").and_then(Value::as_str) == Some("object") {
                return Ok(None);
            }
            out.insert(
                "default".into(),
                mget("default")
                    .filter(|v| !is_nil(v))
                    .cloned()
                    .unwrap_or_else(|| json!([])),
            );
            out.insert("type".into(), json!("array"));
            out.insert("items".into(), items);
            Ok(Some(Value::Object(out)))
        }
        Some("object") => {
            let props = schemastery_fields(Some(s.clone()))?;
            if props.is_empty() {
                return Ok(None);
            }
            out.insert("type".into(), json!("object"));
            out.insert("properties".into(), Value::Array(props));
            Ok(Some(Value::Object(out)))
        }
        _ => Ok(None),
    }
}

/// Port of `schemasteryFields` (`src/extension-config-schema.ts:34`).
pub fn schemastery_fields(schema: Option<Value>) -> Result<Vec<Value>, String> {
    let Some(obj) = obj_of(schema.as_ref().filter(|v| is_truthy(v))) else {
        return Ok(Vec::new());
    };
    if obj.get("type").and_then(Value::as_str) != Some("object") {
        return Ok(Vec::new());
    }
    let Some(dict) = obj.get("dict").filter(|v| is_truthy(v)) else {
        return Ok(Vec::new());
    };
    let entries: Vec<(String, Value)> = match dict {
        Value::Object(map) if is_plain_object(dict) => {
            map.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
        }
        Value::Array(arr) => arr
            .iter()
            .enumerate()
            .map(|(i, v)| (i.to_string(), v.clone()))
            .collect(),
        _ => string_units(dict)
            .map(|u| {
                u.into_iter()
                    .enumerate()
                    .map(|(i, c)| (i.to_string(), utf16_string(&[c])))
                    .collect()
            })
            .unwrap_or_default(),
    };
    let mut fields = Vec::new();
    for (key, child) in entries {
        if let Some(field) = schemastery_field(&key, Some(&child))? {
            fields.push(field);
        }
    }
    Ok(fields)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_schemastery_and_rejects_invalid() {
        let fields = schemastery_fields(Some(json!({
            "type": "object",
            "dict": {
                "text": { "type": "string", "meta": { "description": "人设", "role": "textarea" } },
                "key": { "type": "string", "meta": { "role": "secret" } },
                "n": { "type": "number", "meta": { "default": 2, "min": 1, "max": 5 } },
                "level": { "type": "union", "list": [{ "type": "const", "value": "a" }, { "type": "const", "value": "b", "meta": { "description": "B 档" } }] },
                "tags": { "type": "array", "inner": { "type": "string" } },
                "nested": { "type": "object", "dict": { "on": { "type": "boolean", "meta": { "default": true } } } },
                "hidden": { "type": "string", "meta": { "hidden": true } },
                "any": { "type": "any" }
            }
        })))
        .unwrap();
        assert_eq!(fields.len(), 6);
        assert_eq!(fields[0]["type"], "text");
        assert_eq!(fields[1]["secret"], true);
        assert_eq!(fields[3]["options"][1]["label"], "B 档");
        assert!(schemastery_fields(None).unwrap().is_empty());
        assert!(
            schemastery_fields(Some(
                json!({ "type": "object", "dict": { "u": { "type": "union", "list": [null] } } })
            ))
            .is_err()
        );
    }
}
