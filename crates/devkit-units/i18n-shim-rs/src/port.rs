//! The Rust port of `provider-editing-i18n-shim.mjs`.

#![forbid(unsafe_code)]

use crate::js;
use serde_json::Value;

fn js_str(v: &Value) -> String {
    let elem = |x: &Value| {
        if x.is_null() || js::is_undefined(x) {
            "".into()
        } else {
            js_str(x)
        }
    };
    match v {
        Value::Null => "null".into(),
        Value::Bool(b) => b.to_string(),
        Value::String(s) => s.clone(),
        Value::Array(a) => a.iter().map(elem).collect::<Vec<_>>().join(","),
        v if js::is_undefined(v) => "undefined".into(),
        v => js::as_number(v).map_or_else(|| "[object Object]".into(), js::number_to_string),
    }
}

/// Port of `t` (`provider-editing-i18n-shim.mjs:2`).
pub fn t(zh: Value) -> Result<Value, String> {
    Ok(zh)
}

/// Port of `tf` (`provider-editing-i18n-shim.mjs:6`).
pub fn tf(key: Value, args: Value) -> Result<Value, String> {
    let text = js_str(&key);
    let b = text.as_bytes();
    let (mut out, mut i) = (String::with_capacity(text.len()), 0);
    while i < b.len() {
        if b[i] == b'{' {
            let mut j = i + 1;
            while j < b.len() && b[j].is_ascii_digit() {
                j += 1;
            }
            if j > i + 1 && j < b.len() && b[j] == b'}' {
                if args.is_null() || js::is_undefined(&args) {
                    return Err("TypeError: Cannot read properties of null/undefined".into());
                }
                let idx_s = &text[i + 1..j];
                let idx = idx_s.parse::<usize>().ok();
                let picked = match &args {
                    Value::Array(a) => idx
                        .and_then(|k| a.get(k))
                        .filter(|v| !v.is_null() && !js::is_undefined(v))
                        .map(js_str),
                    Value::String(s) => idx
                        .and_then(|k| s.encode_utf16().nth(k))
                        .and_then(|u| char::from_u32(u32::from(u)))
                        .map(String::from),
                    Value::Object(o) => idx
                        .and_then(|k| o.get(&k.to_string()))
                        .filter(|v| !v.is_null() && !js::is_undefined(v))
                        .map(js_str),
                    _ => None,
                };
                out.push_str(picked.as_deref().unwrap_or(&text[i..=j]));
                i = j + 1;
                continue;
            }
        }
        let ch = text[i..].chars().next().ok_or("invalid utf8")?;
        out.push(ch);
        i += ch.len_utf8();
    }
    Ok(Value::String(out))
}
