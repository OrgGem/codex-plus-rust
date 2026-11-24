//! The Rust port of `src/gateway-auto-compact-model.ts`.
#![forbid(unsafe_code)]

use manager_common_rs::{
    es_trim, is_nil, js_number, js_own_keys_order, spread_object, string_units, utf16_string,
};
use serde_json::{Map, Value};

fn spread(v: &Value) -> Map<String, Value> {
    let mut m = spread_object(v);
    if let Some(u) = string_units(v) {
        u.iter().enumerate().for_each(|(i, &c)| {
            m.insert(i.to_string(), utf16_string(&[c]));
        });
    }
    let ks: Vec<_> = js_own_keys_order(&m).into_iter().cloned().collect();
    ks.into_iter()
        .filter_map(|k| m.remove(&k).map(|x| (k, x)))
        .collect()
}

/// Port of `parseCompactPercent` (`src/gateway-auto-compact-model.ts:6`).
pub fn parse_compact_percent(text: String) -> Result<Value, String> {
    let raw = es_trim(&text);
    if raw.is_empty() {
        return Ok(Value::Null);
    }
    let t = es_trim(raw.strip_suffix('%').unwrap_or(raw));
    let (i, f) = t.split_once('.').unwrap_or((t, "0"));
    let fmt = !i.is_empty()
        && (1..=2).contains(&f.len())
        && !f.contains('.')
        && t.bytes().all(|b| b.is_ascii_digit() || b == b'.');
    let v = if fmt {
        t.parse().unwrap_or(f64::NAN)
    } else {
        f64::NAN
    };
    Ok(js_number(if (1.0..=100.0).contains(&v) {
        v
    } else {
        f64::NAN
    }))
}

/// Port of `withUnlistedAutoCompact` (`src/gateway-auto-compact-model.ts:18`).
pub fn with_unlisted_auto_compact(
    entry: Value,
    shown: Value,
    auto_compact: Value,
) -> Result<Value, String> {
    if is_nil(&entry) {
        return Err("TypeError: entry is null/undefined".into());
    }
    let src = entry
        .get("autoCompact")
        .filter(|v| !is_nil(v))
        .and_then(|v| v.get("models"))
        .unwrap_or(&Value::Null);
    let mut kept = Map::new();
    for (m, v) in spread(src) {
        let low = m.to_lowercase();
        let arr = shown
            .as_array()
            .ok_or("TypeError: shown.some is not a function")?;
        let mut hit = false;
        for s in arr {
            let u = string_units(s).ok_or("TypeError: item.toLowerCase is not a function")?;
            if String::from_utf16_lossy(&u).to_lowercase() == low {
                hit = true;
                break;
            }
        }
        if !hit {
            kept.insert(m, v);
        }
    }
    if is_nil(&auto_compact) {
        return Err("TypeError: autoCompact is null/undefined".into());
    }
    kept.extend(spread(auto_compact.get("models").unwrap_or(&Value::Null)));
    let mut out = spread(&auto_compact);
    out.insert("models".into(), Value::Object(spread(&Value::Object(kept))));
    Ok(Value::Object(spread(&Value::Object(out))))
}

#[cfg(test)]
mod tests {
    use super::*;
    use manager_common_rs::js_rt::as_number;
    use serde_json::json;
    #[test]
    fn percent_and_merge() {
        assert_eq!(parse_compact_percent(" ".into()).unwrap(), Value::Null);
        assert_eq!(parse_compact_percent("84.5%".into()).unwrap(), json!(84.5));
        assert!(
            as_number(&parse_compact_percent("100.5".into()).unwrap()).is_some_and(f64::is_nan)
        );
        let e = json!({"autoCompact": {"models": {"h": 90, "s": 70}}});
        let out = with_unlisted_auto_compact(
            e,
            json!(["S"]),
            json!({"models": {"s": 80}, "fallback": 50}),
        )
        .unwrap();
        assert_eq!(out, json!({"models": {"h": 90, "s": 80}, "fallback": 50}));
        assert!(with_unlisted_auto_compact(Value::Null, json!([]), json!({})).is_err());
    }
}
