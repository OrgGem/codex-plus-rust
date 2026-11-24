//! Shared JS value coercion, UTF-16 helpers, FNV-1a hashing, and local date helpers.

#![forbid(unsafe_code)]

use chrono::{Local, TimeZone};
use serde_json::{Map, Value, json};

pub fn is_js_ws(c: char) -> bool {
    c.is_whitespace() || c == '\u{FEFF}'
}

pub fn js_trim(s: &str) -> &str {
    s.trim_matches(is_js_ws)
}

pub fn undefined() -> Value {
    json!({ "$undefined": true })
}

pub fn is_undefined(v: &Value) -> bool {
    v.as_object()
        .is_some_and(|o| o.len() == 1 && o.get("$undefined") == Some(&Value::Bool(true)))
}

pub fn is_nil(v: &Value) -> bool {
    v.is_null() || is_undefined(v)
}

pub fn is_plain_object(v: &Value) -> bool {
    v.as_object().is_some_and(|o| {
        !(o.len() == 1
            && (o.contains_key("$undefined")
                || o.contains_key("$num")
                || o.contains_key("$utf16")
                || o.contains_key("$set")
                || o.contains_key("$map")))
    })
}

pub fn spread_object(v: &Value) -> Map<String, Value> {
    let mut map = Map::new();
    match v {
        Value::Object(o) if is_plain_object(v) => {
            for (k, val) in o {
                map.insert(k.clone(), val.clone());
            }
        }
        Value::Array(arr) => {
            for (i, val) in arr.iter().enumerate() {
                map.insert(i.to_string(), val.clone());
            }
        }
        _ => {}
    }
    map
}

pub fn as_number(v: &Value) -> Option<f64> {
    match v {
        Value::Number(n) => n.as_f64(),
        Value::Object(o) if o.len() == 1 => match o.get("$num")?.as_str()? {
            "NaN" => Some(f64::NAN),
            "Infinity" => Some(f64::INFINITY),
            "-Infinity" => Some(f64::NEG_INFINITY),
            "-0" => Some(-0.0),
            _ => None,
        },
        _ => None,
    }
}

pub fn js_number(x: f64) -> Value {
    if x.is_nan() {
        json!({ "$num": "NaN" })
    } else if x.is_infinite() {
        json!({ "$num": if x > 0.0 { "Infinity" } else { "-Infinity" } })
    } else if x == 0.0 && x.is_sign_negative() {
        json!({ "$num": "-0" })
    } else if x.fract() == 0.0 && x.abs() < 9_007_199_254_740_992.0 {
        Value::from(x as i64)
    } else {
        Value::from(x)
    }
}

pub fn number_to_string(x: f64) -> String {
    if x.is_nan() {
        return "NaN".into();
    }
    if x.is_infinite() {
        return if x > 0.0 { "Infinity" } else { "-Infinity" }.into();
    }
    if x == 0.0 {
        return "0".into();
    }
    if x.fract() == 0.0 && x.abs() < 9_007_199_254_740_992.0 {
        return (x as i64).to_string();
    }
    let json = serde_json::to_string(&x).unwrap_or_default();
    let (sign, json) = match json.strip_prefix('-') {
        Some(rest) => ("-", rest),
        None => ("", json.as_str()),
    };
    let (mantissa, exponent) = match json.find(['e', 'E']) {
        Some(i) => (&json[..i], json[i + 1..].parse::<i64>().unwrap_or(0)),
        None => (json, 0),
    };
    let (int, frac) = mantissa.split_once('.').unwrap_or((mantissa, ""));
    let all = format!("{int}{frac}");
    let lead = all.len() - all.trim_start_matches('0').len();
    let digits = all[lead..].trim_end_matches('0');
    let k = digits.len() as i64;
    let n = int.len() as i64 - lead as i64 + exponent;
    let mut out = String::with_capacity(digits.len() + 8);
    out.push_str(sign);
    if k <= n && n <= 21 {
        out.push_str(digits);
        out.extend(std::iter::repeat_n('0', (n - k) as usize));
    } else if 0 < n && n <= 21 {
        out.push_str(&digits[..n as usize]);
        out.push('.');
        out.push_str(&digits[n as usize..]);
    } else if -6 < n && n <= 0 {
        out.push_str("0.");
        out.extend(std::iter::repeat_n('0', (-n) as usize));
        out.push_str(digits);
    } else {
        let e = n - 1;
        out.push_str(&digits[..1]);
        if k > 1 {
            out.push('.');
            out.push_str(&digits[1..]);
        }
        out.push('e');
        out.push(if e < 0 { '-' } else { '+' });
        out.push_str(&e.abs().to_string());
    }
    out
}

pub fn string_units(v: &Value) -> Option<Vec<u16>> {
    match v {
        Value::String(s) => Some(s.encode_utf16().collect()),
        Value::Object(o) if o.len() == 1 => o
            .get("$utf16")?
            .as_array()?
            .iter()
            .map(|u| u.as_u64().and_then(|u| u16::try_from(u).ok()))
            .collect(),
        _ => None,
    }
}

pub fn utf16_string(units: &[u16]) -> Value {
    match String::from_utf16(units) {
        Ok(s) => Value::String(s),
        Err(_) => json!({ "$utf16": units }),
    }
}

pub fn is_truthy(v: &Value) -> bool {
    !is_nil(v)
        && v != &Value::Bool(false)
        && !string_units(v).is_some_and(|u| u.is_empty())
        && !as_number(v).is_some_and(|n| n == 0.0 || n.is_nan())
}

pub fn to_number(v: &Value) -> f64 {
    if let Some(n) = as_number(v) {
        return n;
    }
    match v {
        Value::Null | Value::Bool(false) => 0.0,
        Value::Bool(true) => 1.0,
        v if is_undefined(v) => f64::NAN,
        Value::String(s) => parse_js_number(s),
        Value::Array(a) => match a.len() {
            0 => 0.0,
            1 => to_number(&a[0]),
            _ => f64::NAN,
        },
        _ => f64::NAN,
    }
}

pub fn parse_js_number(raw: &str) -> f64 {
    let s = js_trim(raw);
    if s.is_empty() {
        return 0.0;
    }
    if s == "Infinity" || s == "+Infinity" {
        return f64::INFINITY;
    }
    if s == "-Infinity" {
        return f64::NEG_INFINITY;
    }
    if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        return u64::from_str_radix(hex, 16)
            .map(|n| n as f64)
            .unwrap_or(f64::NAN);
    }
    if let Some(oct) = s.strip_prefix("0o").or_else(|| s.strip_prefix("0O")) {
        return u64::from_str_radix(oct, 8)
            .map(|n| n as f64)
            .unwrap_or(f64::NAN);
    }
    if let Some(bin) = s.strip_prefix("0b").or_else(|| s.strip_prefix("0B")) {
        return u64::from_str_radix(bin, 2)
            .map(|n| n as f64)
            .unwrap_or(f64::NAN);
    }
    if !s
        .bytes()
        .all(|b| b.is_ascii_digit() || matches!(b, b'.' | b'+' | b'-' | b'e' | b'E'))
    {
        return f64::NAN;
    }
    s.parse::<f64>().unwrap_or(f64::NAN)
}

pub fn js_str_u16(v: &Value) -> Vec<u16> {
    if let Some(u) = string_units(v) {
        return u;
    }
    let s = match v {
        Value::Null => "null".into(),
        Value::Bool(b) => b.to_string(),
        Value::Array(a) => a
            .iter()
            .map(|x| {
                if is_nil(x) {
                    String::new()
                } else {
                    String::from_utf16_lossy(&js_str_u16(x))
                }
            })
            .collect::<Vec<_>>()
            .join(","),
        v if is_undefined(v) => "undefined".into(),
        v => as_number(v).map_or_else(|| "[object Object]".into(), number_to_string),
    };
    s.encode_utf16().collect()
}

pub fn js_str(v: &Value) -> String {
    String::from_utf16_lossy(&js_str_u16(v))
}

/// FNV-1a 32-bit hash formatted in base 36, matching JS `((hash >>> 0).toString(36))`.
pub fn fnv1a_36(text: &str) -> String {
    let mut hash: u32 = 0x811c_9dc5;
    for unit in text.encode_utf16() {
        hash ^= u32::from(unit);
        hash = hash.wrapping_mul(0x0100_0193);
    }
    to_radix_36(hash)
}

pub fn to_radix_36(mut n: u32) -> String {
    if n == 0 {
        return "0".into();
    }
    const DIGITS: &[u8; 36] = b"0123456789abcdefghijklmnopqrstuvwxyz";
    let mut buf = Vec::with_capacity(8);
    while n > 0 {
        buf.push(DIGITS[(n % 36) as usize]);
        n /= 36;
    }
    buf.reverse();
    String::from_utf8(buf).unwrap_or_else(|_| "0".into())
}

pub fn u64_to_radix_36(mut n: u64) -> String {
    if n == 0 {
        return "0".into();
    }
    const DIGITS: &[u8; 36] = b"0123456789abcdefghijklmnopqrstuvwxyz";
    let mut buf = Vec::with_capacity(14);
    while n > 0 {
        buf.push(DIGITS[(n % 36) as usize]);
        n /= 36;
    }
    buf.reverse();
    String::from_utf8(buf).unwrap_or_else(|_| "0".into())
}

pub fn now_ms() -> f64 {
    Local::now().timestamp_millis() as f64
}

pub fn local_datetime_from_ms(ms: f64) -> Option<chrono::DateTime<Local>> {
    if !ms.is_finite() || ms.abs() > 8.64e15 {
        return None;
    }
    let truncated = ms.trunc();
    let secs = (truncated / 1000.0).floor() as i64;
    let rem_ms = truncated - (secs as f64) * 1000.0;
    let ns = (rem_ms * 1_000_000.0).clamp(0.0, 999_999_999.0) as u32;
    Local.timestamp_opt(secs, ns).single()
}

pub fn format_en_us_2dec(value: f64) -> String {
    if value.is_nan() {
        return "NaN".into();
    }
    if value.is_infinite() {
        return if value.is_sign_negative() {
            "-∞"
        } else {
            "∞"
        }
        .into();
    }
    let rounded = if (value * 100.0).round() == 0.0 {
        0.0
    } else {
        value
    };
    let s = format!("{rounded:.2}");
    let (neg, body) = match s.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, s.as_str()),
    };
    let (int_part, frac_part) = body.split_once('.').unwrap_or((body, "00"));
    let mut grouped = String::with_capacity(int_part.len() + int_part.len() / 3 + 4);
    if neg {
        grouped.push('-');
    }
    for (i, ch) in int_part.bytes().enumerate() {
        if i > 0 && (int_part.len() - i).is_multiple_of(3) {
            grouped.push(',');
        }
        grouped.push(ch as char);
    }
    grouped.push('.');
    grouped.push_str(frac_part);
    grouped
}

/// JS `Math.round`: halves toward +infinity, preserves -0 for small negatives.
pub fn math_round(x: f64) -> f64 {
    if !x.is_finite() || x == x.trunc() {
        return x;
    }
    let floor = x.floor();
    let r = if x - floor >= 0.5 { floor + 1.0 } else { floor };
    if r == 0.0 && x < 0.0 { -0.0 } else { r }
}

fn merge_sort_indices<T, F>(idx: &mut [usize], scratch: &mut [usize], items: &[T], compare: &mut F)
where
    F: FnMut(&T, &T) -> f64,
{
    let len = idx.len();
    if len < 2 {
        return;
    }
    let mid = len / 2;
    merge_sort_indices(&mut idx[..mid], &mut scratch[..mid], items, compare);
    merge_sort_indices(&mut idx[mid..], &mut scratch[mid..], items, compare);
    let (mut i, mut j, mut k) = (0, mid, 0);
    while i < mid && j < len {
        if compare(&items[idx[i]], &items[idx[j]]) > 0.0 {
            scratch[k] = idx[j];
            j += 1;
        } else {
            scratch[k] = idx[i];
            i += 1;
        }
        k += 1;
    }
    if i < mid {
        scratch[k..k + (mid - i)].copy_from_slice(&idx[i..mid]);
    }
    if j < len {
        scratch[k..k + (len - j)].copy_from_slice(&idx[j..len]);
    }
    idx.copy_from_slice(&scratch[..len]);
}

/// Matches JS `Array.prototype.sort`: binary-insertion on small runs (`< 64`) and
/// deterministic `NaN`-safe stable merge sort (`cmp > 0.0` swaps, `NaN` / `<= 0.0` preserves order)
/// on `>= 64` items without triggering `driftsort` total-order panics.
pub fn js_sort<T>(items: &mut [T], mut compare: impl FnMut(&T, &T) -> f64) {
    let n = items.len();
    if n < 2 {
        return;
    }
    if n >= 64 {
        let mut perm: Vec<usize> = (0..n).collect();
        let mut scratch = vec![0usize; n];
        merge_sort_indices(&mut perm, &mut scratch, items, &mut compare);
        for start in 0..n {
            if perm[start] == start {
                continue;
            }
            let mut cur = start;
            while perm[cur] != start {
                let next = perm[cur];
                items.swap(cur, next);
                perm[cur] = cur;
                cur = next;
            }
            perm[cur] = cur;
        }
        return;
    }
    let mut run = 2;
    let descending = compare(&items[1], &items[0]) < 0.0;
    while run < n {
        let order = compare(&items[run], &items[run - 1]);
        if (descending && order >= 0.0) || (!descending && order < 0.0) {
            break;
        }
        run += 1;
    }
    if descending {
        items[..run].reverse();
    }
    for start in run..n {
        let (mut left, mut right) = (0, start);
        while left < right {
            let mid = left + ((right - left) >> 1);
            if compare(&items[start], &items[mid]) < 0.0 {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        items[left..=start].rotate_right(1);
    }
}
