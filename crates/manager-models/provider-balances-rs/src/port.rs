//! The Rust port of `src/provider-balances.ts`.

#![forbid(unsafe_code)]

use manager_common_rs::js_rt::{is_nil, is_plain_object, is_undefined, string_units, to_number};
use serde_json::Value;

const PROVIDER_BALANCE_TTL_MS: f64 = 5.0 * 60_000.0;

/// Port of `providerBalancesFresh` (`src/provider-balances.ts:28`).
pub fn provider_balances_fresh(entry: Value, now: f64, ttl: Value) -> Result<bool, String> {
    if is_nil(&entry) {
        return Err(
            "TypeError: Cannot read properties of null/undefined (reading 'balances')".into(),
        );
    }
    let balances = entry
        .as_object()
        .filter(|_| is_plain_object(&entry))
        .and_then(|o| o.get("balances"));
    if matches!(balances, Some(Value::Null)) {
        return Ok(false);
    }
    let at = entry
        .as_object()
        .filter(|_| is_plain_object(&entry))
        .and_then(|o| o.get("at"))
        .map_or(f64::NAN, to_number);
    let ttl_ms = if is_undefined(&ttl) {
        PROVIDER_BALANCE_TTL_MS
    } else {
        to_number(&ttl)
    };
    Ok((now - at) < ttl_ms)
}

/// Port of `providerBalanceFor` (`src/provider-balances.ts:33`).
pub fn provider_balance_for(balances: Value, id: String) -> Result<Value, String> {
    if is_nil(&balances) {
        return Ok(Value::Null);
    }
    let list = balances
        .as_array()
        .ok_or("TypeError: balances.find is not a function")?;
    let needle: Vec<u16> = id.encode_utf16().collect();
    for item in list {
        if is_nil(item) {
            return Err(
                "TypeError: Cannot read properties of null/undefined (reading 'providerId')".into(),
            );
        }
        let matches = item
            .as_object()
            .filter(|_| is_plain_object(item))
            .and_then(|o| o.get("providerId"))
            .and_then(string_units)
            .is_some_and(|u| u == needle);
        if matches {
            return Ok(item.clone());
        }
    }
    Ok(Value::Null)
}

#[cfg(test)]
mod tests {
    use super::*;
    use manager_common_rs::js_rt::undefined;
    use serde_json::json;

    #[test]
    fn freshness_and_lookup_match_ts_behavior() {
        let deepseek = json!({
            "providerId": "deepseek",
            "provider": "DeepSeek",
            "baseUrl": "https://api.deepseek.com",
            "amount": 14.6,
            "currency": "CNY",
            "error": null
        });
        assert_eq!(
            provider_balance_for(json!([deepseek]), "deepseek".into()).unwrap(),
            deepseek
        );
        assert_eq!(
            provider_balance_for(json!([deepseek]), "kimi".into()).unwrap(),
            Value::Null
        );
        assert_eq!(
            provider_balance_for(Value::Null, "deepseek".into()).unwrap(),
            Value::Null
        );
        assert_eq!(
            provider_balance_for(undefined(), "deepseek".into()).unwrap(),
            Value::Null
        );
        assert!(
            provider_balances_fresh(
                json!({ "balances": [], "at": 1000 }),
                1000.0 + PROVIDER_BALANCE_TTL_MS - 1.0,
                undefined()
            )
            .unwrap()
        );
        assert!(
            !provider_balances_fresh(
                json!({ "balances": [], "at": 1000 }),
                1000.0 + PROVIDER_BALANCE_TTL_MS,
                undefined()
            )
            .unwrap()
        );
        assert!(
            !provider_balances_fresh(json!({ "balances": null, "at": 1000 }), 1000.0, undefined())
                .unwrap()
        );
        assert!(
            !provider_balances_fresh(json!({ "balances": [], "at": 1000 }), 1000.0, Value::Null)
                .unwrap()
        );
    }
}
