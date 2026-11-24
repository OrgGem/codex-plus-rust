//! The Rust port of `native-history.mjs`.

#![forbid(unsafe_code)]

use crate::js;
use serde_json::Value;

fn is_nil(v: &Value) -> bool {
    v.is_null() || js::is_undefined(v)
}

fn js_str(v: &Value) -> String {
    let elem = |x: &Value| if is_nil(x) { "".into() } else { js_str(x) };
    match v {
        Value::Null => "null".into(),
        Value::Bool(b) => b.to_string(),
        Value::String(s) => s.clone(),
        Value::Array(a) => a.iter().map(elem).collect::<Vec<_>>().join(","),
        v if js::is_undefined(v) => "undefined".into(),
        v => js::as_number(v).map_or_else(|| "[object Object]".into(), js::number_to_string),
    }
}

fn is_non_nl(u: u16) -> bool {
    !matches!(u, 0x0A | 0x0D | 0x2028 | 0x2029)
}

fn ascii_eq_fold(u: u16, b: u8) -> bool {
    u == u16::from(b) || u == u16::from(b.to_ascii_uppercase()) || (b == b's' && u == 0x017F)
}

fn starts_lit(s: &[u16], pat: &[u8]) -> bool {
    s.len() >= pat.len() && pat.iter().enumerate().all(|(i, &b)| ascii_eq_fold(s[i], b))
}

fn has_bounded_pair(u: &[u16], head: &[u8], max_gap: usize, tail: &[u8]) -> bool {
    let min_len = head.len() + tail.len();
    for i in 0..=u.len().saturating_sub(min_len) {
        if !starts_lit(&u[i..], head) {
            continue;
        }
        let after = i + head.len();
        for g in 0..=max_gap {
            let end = after + g;
            if end + tail.len() > u.len() {
                break;
            }
            if starts_lit(&u[end..], tail) {
                return true;
            }
            if !is_non_nl(u[end]) {
                break;
            }
        }
    }
    false
}

/// Port of `isExportSizeError` (`native-history.mjs:3`).
pub fn is_export_size_error(message: Value) -> Result<Value, String> {
    let u: Vec<u16> =
        js::string_units(&message).unwrap_or_else(|| js_str(&message).encode_utf16().collect());
    let cn = u
        .windows(2)
        .position(|w| w == [0x8D85, 0x8FC7])
        .is_some_and(|pos| {
            let mut p = pos + 2;
            while p + 1 < u.len() {
                if matches!([u[p], u[p + 1]], [0x5927, 0x5C0F] | [0x9650, 0x5236]) {
                    return true;
                }
                if !is_non_nl(u[p]) {
                    return false;
                }
                p += 1;
            }
            false
        });
    let matched = cn
        || u.windows(9).any(|w| starts_lit(w, b"too large"))
        || u.windows(7).any(|w| starts_lit(w, b"too big"))
        || has_bounded_pair(&u, b"size", 30, b"limit")
        || has_bounded_pair(&u, b"maximum", 20, b"size");
    Ok(Value::Bool(matched))
}
