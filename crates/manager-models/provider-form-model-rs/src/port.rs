//! The Rust port of `src/provider-form-model.ts`.

#![forbid(unsafe_code)]

use crate::js;
use manager_common_rs::gateway_shared::split_models;
use manager_common_rs::{
    as_number, es_trim, is_nil, is_plain_object, is_truthy, js_own_keys_order, js_str,
    number_to_string, parse_js_number, spread_object, string_units, to_number, utf16_string,
};
use serde_json::{Map, Value, json};
use std::collections::HashSet;

static UNDEF: std::sync::LazyLock<Value> = std::sync::LazyLock::new(js::undefined);

fn js_spread(v: &Value) -> Map<String, Value> {
    if let Some(units) = string_units(v) {
        let mut map = Map::with_capacity(units.len());
        for (i, &u) in units.iter().enumerate() {
            map.insert(i.to_string(), utf16_string(&[u]));
        }
        return map;
    }
    spread_object(v)
}

fn nil_err(prop: &str) -> String {
    format!("TypeError: Cannot read properties of null/undefined (reading '{prop}')")
}

fn prop<'a>(v: &'a Value, key: &str) -> Result<&'a Value, String> {
    if is_nil(v) {
        return Err(nil_err(key));
    }
    Ok(v.as_object()
        .filter(|_| is_plain_object(v))
        .and_then(|m| m.get(key))
        .unwrap_or(&UNDEF))
}

fn req_str(v: &Value, key: &str, method: &str) -> Result<String, String> {
    let field = prop(v, key)?;
    if is_nil(field) {
        return Err(nil_err(method));
    }
    string_units(field)
        .map(|u| String::from_utf16_lossy(&u))
        .ok_or_else(|| format!("TypeError: {key}.{method} is not a function"))
}

fn js_to_fixed_4_string(value: f64) -> String {
    if !value.is_finite() || value <= 0.0 {
        return "0".into();
    }
    if value >= 1e21 {
        return number_to_string(value);
    }
    if value < 0.00004 {
        return "0".into();
    }
    let bits = value.to_bits();
    let exp_bits = ((bits >> 52) & 0x7ff) as i32;
    let frac = bits & ((1u64 << 52) - 1);
    let mant = u128::from((1u64 << 52) | frac);
    let exp = exp_bits - 1075;
    let scale = 10_000u128;
    let q = if exp >= 0 {
        (mant << (exp as u32)) * scale
    } else {
        let shift = (-exp) as u32;
        let scaled = mant * scale;
        let rem_mask = (1u128 << shift) - 1;
        let half = 1u128 << (shift - 1);
        (scaled >> shift) + u128::from((scaled & rem_mask) >= half)
    };
    let text = format!("{}.{:04}", q / scale, q % scale);
    let num = text.parse::<f64>().unwrap_or(value);
    if num == 0.0 {
        "0".into()
    } else {
        number_to_string(num)
    }
}

fn format_price_val(v: &Value) -> String {
    as_number(v).map_or_else(|| "0".into(), js_to_fixed_4_string)
}

fn price_value(text: &str) -> f64 {
    let val = parse_js_number(es_trim(text));
    if val.is_finite() && val > 0.0 {
        val
    } else {
        0.0
    }
}

fn price_from_row_inner(row: &Value, currency: &str) -> Result<Value, String> {
    if is_nil(row) {
        return Err(nil_err("input"));
    }
    let input = price_value(&req_str(row, "input", "trim")?);
    let cached = price_value(&req_str(row, "cached", "trim")?);
    let output = price_value(&req_str(row, "output", "trim")?);
    if input == 0.0 && cached == 0.0 && output == 0.0 {
        return Ok(Value::Null);
    }
    let curr = if currency.is_empty() { "USD" } else { currency };
    Ok(json!({
        "currency": curr,
        "input": js::number(input),
        "cachedInput": js::number(cached),
        "output": js::number(output),
    }))
}

/// Port of `draftModels` (`src/provider-form-model.ts:16`).
pub fn draft_models(draft: Value) -> Result<Vec<String>, String> {
    let raw = req_str(&draft, "modelList", "split")?;
    split_models(raw)
}

/// Port of `defaultModel` (`src/provider-form-model.ts:21`).
pub fn default_model(draft: Value) -> Result<String, String> {
    let models = draft_models(draft.clone())?;
    let chosen_raw = req_str(&draft, "model", "trim")?;
    let chosen = es_trim(&chosen_raw);
    if !chosen.is_empty() && models.iter().any(|m| m == chosen) {
        return Ok(chosen.to_string());
    }
    Ok(models
        .into_iter()
        .next()
        .unwrap_or_else(|| chosen.to_string()))
}

/// Port of `withModels` (`src/provider-form-model.ts:28`).
pub fn with_models(draft: Value, models: Vec<String>) -> Result<Value, String> {
    let mut seen = HashSet::new();
    let mut unique = Vec::new();
    for model in models {
        let trimmed = es_trim(&model);
        if !trimmed.is_empty() && seen.insert(trimmed.to_string()) {
            unique.push(trimmed.to_string());
        }
    }
    let chosen_raw = req_str(&draft, "model", "trim")?;
    let chosen = es_trim(&chosen_raw);
    let keep = if unique.iter().any(|m| m == chosen) {
        chosen
    } else {
        ""
    };
    let mut out = js_spread(&draft);
    out.insert("modelList".into(), Value::String(unique.join("\n")));
    out.insert("model".into(), Value::String(keep.to_string()));
    Ok(Value::Object(out))
}

/// Port of `addModel` (`src/provider-form-model.ts:34`).
pub fn add_model(draft: Value, model: String) -> Result<Value, String> {
    let mut models = draft_models(draft.clone())?;
    models.extend(split_models(model)?);
    with_models(draft, models)
}

/// Port of `removeModel` (`src/provider-form-model.ts:38`).
pub fn remove_model(draft: Value, model: String) -> Result<Value, String> {
    let models = draft_models(draft.clone())?
        .into_iter()
        .filter(|item| item != &model)
        .collect();
    with_models(draft, models)
}

/// Port of `draftKeys` (`src/provider-form-model.ts:42`).
pub fn draft_keys(draft: Value) -> Result<Vec<String>, String> {
    let raw = req_str(&draft, "keys", "split")?;
    let mut parts: Vec<String> = raw.split('\n').map(str::to_string).collect();
    let count = parts.len().saturating_sub(1);
    for line in parts.iter_mut().take(count) {
        if line.ends_with('\r') {
            line.pop();
        }
    }
    Ok(parts)
}

/// Port of `withKeys` (`src/provider-form-model.ts:47`).
pub fn with_keys(draft: Value, keys: Vec<String>) -> Result<Value, String> {
    let mut out = js_spread(&draft);
    out.insert("keys".into(), Value::String(keys.join("\n")));
    Ok(Value::Object(out))
}

/// Port of `formatPrice` (`src/provider-form-model.ts:52`).
pub fn format_price(value: f64) -> Result<String, String> {
    Ok(js_to_fixed_4_string(value))
}

/// Port of `draftWithPrice` (`src/provider-form-model.ts:57`).
pub fn draft_with_price(draft: Value, r#match: Value) -> Result<Value, String> {
    let curr = prop(&r#match, "currency")?;
    let price_currency = if is_truthy(curr) {
        curr.clone()
    } else {
        Value::String("USD".into())
    };
    let price_input = format_price_val(prop(&r#match, "input")?);
    let price_cached = format_price_val(prop(&r#match, "cachedInput")?);
    let price_output = format_price_val(prop(&r#match, "output")?);
    let mut out = js_spread(&draft);
    out.insert("priceCurrency".into(), price_currency);
    out.insert("priceInput".into(), Value::String(price_input));
    out.insert("priceCached".into(), Value::String(price_cached));
    out.insert("priceOutput".into(), Value::String(price_output));
    Ok(Value::Object(out))
}

/// Port of `hasPrice` (`src/provider-form-model.ts:67`).
pub fn has_price(draft: Value) -> Result<bool, String> {
    if is_nil(&draft) {
        return Err(nil_err("priceInput"));
    }
    Ok(["priceInput", "priceCached", "priceOutput"]
        .into_iter()
        .any(|k| prop(&draft, k).is_ok_and(|v| to_number(v) > 0.0)))
}

/// Port of `priceRowFrom` (`src/provider-form-model.ts:76`).
pub fn price_row_from(price: Value) -> Result<Value, String> {
    if !is_truthy(&price) {
        return Ok(json!({ "input": "", "cached": "", "output": "" }));
    }
    Ok(json!({
        "input": js_str(prop(&price, "input")?),
        "cached": js_str(prop(&price, "cachedInput")?),
        "output": js_str(prop(&price, "output")?),
    }))
}

/// Port of `priceRowFromMatch` (`src/provider-form-model.ts:80`).
pub fn price_row_from_match(r#match: Value) -> Result<Value, String> {
    Ok(json!({
        "input": format_price_val(prop(&r#match, "input")?),
        "cached": format_price_val(prop(&r#match, "cachedInput")?),
        "output": format_price_val(prop(&r#match, "output")?),
    }))
}

/// Port of `priceFromRow` (`src/provider-form-model.ts:90`).
pub fn price_from_row(row: Value, currency: String) -> Result<Value, String> {
    price_from_row_inner(&row, &currency)
}

/// Port of `priceFromMatch` (`src/provider-form-model.ts:95`).
pub fn price_from_match(r#match: Value) -> Result<Value, String> {
    let curr = prop(&r#match, "currency")?;
    let currency = if is_truthy(curr) {
        curr.clone()
    } else {
        Value::String("USD".into())
    };
    Ok(json!({
        "currency": currency,
        "input": prop(&r#match, "input")?,
        "cachedInput": prop(&r#match, "cachedInput")?,
        "output": prop(&r#match, "output")?,
    }))
}

/// Port of `modelPricesFromRows` (`src/provider-form-model.ts:100`).
pub fn model_prices_from_rows(rows: Value, currency: String) -> Result<Value, String> {
    if is_nil(&rows) {
        return Err("TypeError: Cannot convert undefined or null to object".into());
    }
    let mut prices = Map::new();
    match &rows {
        Value::Object(obj) if is_plain_object(&rows) => {
            for model in js_own_keys_order(obj) {
                if let Some(row) = obj.get(model) {
                    let price = price_from_row_inner(row, &currency)?;
                    if !price.is_null() {
                        prices.insert(model.clone(), price);
                    }
                }
            }
        }
        Value::Array(arr) => {
            for (i, row) in arr.iter().enumerate() {
                let price = price_from_row_inner(row, &currency)?;
                if !price.is_null() {
                    prices.insert(i.to_string(), price);
                }
            }
        }
        Value::String(s) => {
            for (i, ch) in s.chars().enumerate() {
                let price = price_from_row_inner(&Value::String(ch.to_string()), &currency)?;
                if !price.is_null() {
                    prices.insert(i.to_string(), price);
                }
            }
        }
        _ => {}
    }
    Ok(Value::Object(prices))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base_draft() -> Value {
        json!({
            "name": "X",
            "baseUrl": "https://x",
            "keys": "",
            "protocol": "chatCompletions",
            "model": "",
            "modelList": "",
            "priceCurrency": "CNY",
            "priceInput": "",
            "priceCached": "",
            "priceOutput": "",
            "proxy": ""
        })
    }

    #[test]
    fn models_keys_and_prices_match_ts_behavior() {
        let mut d = base_draft();
        d["modelList"] = json!("a\nb");
        d["model"] = json!("b");
        assert_eq!(default_model(d.clone()).unwrap(), "b");
        d["model"] = json!("zzz");
        assert_eq!(default_model(d.clone()).unwrap(), "a");

        let added = add_model(base_draft(), "b\na".into()).unwrap();
        assert_eq!(added["modelList"], json!("b\na"));
        let mut with_b = added.clone();
        with_b["model"] = json!("b");
        let removed = remove_model(with_b, "b".into()).unwrap();
        assert_eq!(removed["model"], json!(""));
        assert_eq!(default_model(removed).unwrap(), "a");

        assert_eq!(draft_keys(base_draft()).unwrap(), vec![""]);
        let keyed = with_keys(base_draft(), vec!["k1".into(), "k2".into()]).unwrap();
        assert_eq!(draft_keys(keyed).unwrap(), vec!["k1", "k2"]);

        assert_eq!(format_price(3.0).unwrap(), "3");
        assert_eq!(format_price(0.0030001).unwrap(), "0.003");
        assert_eq!(format_price(0.03125).unwrap(), "0.0313");
        assert_eq!(format_price(f64::NAN).unwrap(), "0");

        let priced = draft_with_price(
            base_draft(),
            json!({ "providerId": "deepseek", "providerName": "DeepSeek", "modelId": "m", "currency": "USD", "input": 0.15, "cachedInput": 0.0030001, "output": 0.6 }),
        )
        .unwrap();
        assert_eq!(priced["priceCurrency"], json!("USD"));
        assert_eq!(priced["priceCached"], json!("0.003"));
        assert!(has_price(priced).unwrap());
        assert!(!has_price(base_draft()).unwrap());

        assert_eq!(
            price_from_row(
                json!({ "input": "0", "cached": "", "output": " " }),
                "USD".into()
            )
            .unwrap(),
            Value::Null
        );
        let map = model_prices_from_rows(
            json!({
                "m1": { "input": "1.5", "cached": "", "output": "3" },
                "m2": { "input": "0", "cached": "0", "output": "0" }
            }),
            "".into(),
        )
        .unwrap();
        assert_eq!(
            map,
            json!({ "m1": { "currency": "USD", "input": 1.5, "cachedInput": 0, "output": 3 } })
        );
    }
}
