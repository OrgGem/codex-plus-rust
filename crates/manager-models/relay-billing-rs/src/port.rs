//! The Rust port of `src/relay-billing.ts`.

#![forbid(unsafe_code)]

use crate::js;
use manager_common_rs::{
    as_number, fnv1a_36, format_en_us_2dec, is_nil, is_truthy, js_str, number_to_string, to_number,
};
use serde_json::{Map, Value, json};

const RELAY_BILLING_POSITIVE_TTL_MS: f64 = 5.0 * 60_000.0;
const RELAY_BILLING_FAILURE_TTL_MS: f64 = 5.0 * 60_000.0;

fn is_js_ws(c: char) -> bool {
    c != '\u{0085}' && (c.is_whitespace() || c == '\u{FEFF}')
}

fn js_trim(s: &str) -> &str {
    s.trim_matches(is_js_ws)
}

fn require_obj<'a>(v: &'a Value, prop: &str) -> Result<&'a Map<String, Value>, String> {
    if is_nil(v) {
        return Err(format!(
            "TypeError: Cannot read properties of null/undefined (reading '{prop}')"
        ));
    }
    Ok(v.as_object().unwrap_or_else(|| empty_map_ref()))
}

fn empty_map_ref() -> &'static Map<String, Value> {
    static EMPTY: std::sync::LazyLock<Map<String, Value>> = std::sync::LazyLock::new(Map::new);
    &EMPTY
}

fn require_str_trim(obj: &Map<String, Value>, prop: &str) -> Result<String, String> {
    match obj.get(prop) {
        Some(Value::String(s)) => Ok(js_trim(s).to_string()),
        Some(v) if is_nil(v) => {
            Err("TypeError: Cannot read properties of null/undefined (reading 'trim')".into())
        }
        None => Err("TypeError: Cannot read properties of undefined (reading 'trim')".into()),
        _ => Err(format!("TypeError: {prop}.trim is not a function")),
    }
}

fn opt_trimmed_str_val(v: Option<&Value>) -> Result<String, String> {
    match v {
        Some(Value::String(s)) => Ok(js_trim(s).to_string()),
        Some(v) if is_nil(v) => {
            Err("TypeError: Cannot read properties of null/undefined (reading 'trim')".into())
        }
        None => Err("TypeError: Cannot read properties of undefined (reading 'trim')".into()),
        _ => Err("TypeError: value.trim is not a function".into()),
    }
}

fn upstream_url(obj: &Map<String, Value>) -> Result<String, String> {
    let up = require_str_trim(obj, "upstreamBaseUrl")?;
    let chosen = if up.is_empty() {
        require_str_trim(obj, "baseUrl")?
    } else {
        up
    };
    Ok(chosen.trim_end_matches('/').to_string())
}

fn active_api_key(obj: &Map<String, Value>) -> Result<String, String> {
    let keys = match obj.get("apiKeys") {
        None => &[][..],
        Some(v) if is_nil(v) => &[][..],
        Some(Value::Array(arr)) => arr.as_slice(),
        Some(_) => return Err("TypeError: keys.find is not a function".into()),
    };
    let active_id = obj.get("activeApiKeyId");
    let mut selected: Option<&Value> = None;
    for entry in keys {
        if is_nil(entry) {
            return Err(
                "TypeError: Cannot read properties of null/undefined (reading 'id')".into(),
            );
        }
        let entry_id = entry.as_object().and_then(|m| m.get("id"));
        let matches = match (entry_id, active_id) {
            (None, None) => true,
            (None, Some(b)) if js::is_undefined(b) => true,
            (Some(a), None) if js::is_undefined(a) => true,
            (Some(a), Some(b)) => a == b,
            _ => false,
        };
        if matches {
            selected = Some(entry);
            break;
        }
    }
    let chosen_entry = selected.or_else(|| keys.first());
    let selected_key = chosen_entry
        .and_then(|e| e.as_object())
        .and_then(|m| m.get("apiKey"))
        .filter(|v| !is_nil(v));
    let raw_key = selected_key.or_else(|| obj.get("apiKey"));
    opt_trimmed_str_val(raw_key)
}

fn round_to_4_js(value: f64) -> f64 {
    if !value.is_finite() || value == 0.0 || value.abs() >= 1e16 {
        return if value == 0.0 { 0.0 } else { value };
    }
    let neg = value.is_sign_negative();
    let bits = value.abs().to_bits();
    let exp = ((bits >> 52) & 0x7ff) as i32;
    let frac = bits & ((1u64 << 52) - 1);
    let (m, e) = if exp == 0 {
        (u128::from(frac), -1074_i32)
    } else {
        (u128::from((1u64 << 52) | frac), exp - 1075)
    };
    if e >= 0 {
        return value;
    }
    let shift = (-e) as u32;
    if shift >= 68 {
        return 0.0;
    }
    let scaled = m * 10_000;
    let half = 1u128 << (shift - 1);
    let rounded = (scaled + half) >> shift;
    let whole = rounded / 10_000;
    let dec = rounded % 10_000;
    let formatted = format!("{whole}.{dec:04}");
    let parsed = formatted.parse::<f64>().unwrap_or(value.abs());
    if parsed == 0.0 {
        0.0
    } else if neg {
        -parsed
    } else {
        parsed
    }
}

/// Port of `relayPlatformLabel` (`src/relay-billing.ts:53`).
pub fn relay_platform_label(platform: String) -> Result<String, String> {
    Ok(if platform == "newapi" {
        "new-api".into()
    } else {
        "sub2api".into()
    })
}

/// Port of `normalizeRelayBillingProbe` (`src/relay-billing.ts:77`).
pub fn normalize_relay_billing_probe(result: Value) -> Result<Value, String> {
    let obj = require_obj(&result, "platform")?;
    let platform = match obj.get("platform").and_then(Value::as_str) {
        Some("sub2api") => Some("sub2api"),
        Some("newapi") => Some("newapi"),
        _ => None,
    };
    let pick_nullish = |key: &str, fallback: Value| -> Value {
        match obj.get(key) {
            Some(v) if !is_nil(v) => v.clone(),
            _ => fallback,
        }
    };
    let outcome = if platform.is_some() {
        "recognized"
    } else if obj.get("outcome").and_then(Value::as_str) == Some("unavailable") {
        "unavailable"
    } else {
        "notRelay"
    };
    let newapi_access = if platform == Some("newapi") {
        pick_nullish("newapiAccess", Value::Null)
    } else {
        Value::Null
    };
    Ok(json!({
        "platform": platform.map(Value::from).unwrap_or(Value::Null),
        "rate": pick_nullish("rate", Value::Null),
        "usage": pick_nullish("usage", Value::Null),
        "detail": pick_nullish("detail", Value::String(String::new())),
        "outcome": outcome,
        "newapiAccess": newapi_access,
    }))
}

/// Port of `newApiAccessHint` (`src/relay-billing.ts:100`).
pub fn new_api_access_hint(probe: Value) -> Result<Option<Value>, String> {
    let obj = require_obj(&probe, "platform")?;
    if obj.get("platform").and_then(Value::as_str) != Some("newapi") {
        return Ok(None);
    }
    let usage_kind = obj
        .get("usage")
        .filter(|v| !is_nil(v))
        .and_then(Value::as_object)
        .and_then(|u| u.get("kind"))
        .and_then(Value::as_str);
    if usage_kind != Some("unlimited") {
        return Ok(None);
    }
    let access = obj.get("newapiAccess").filter(|v| !is_nil(v));
    let configured = access
        .and_then(Value::as_object)
        .and_then(|a| a.get("configured"))
        .is_some_and(is_truthy);
    if configured {
        let error = access
            .and_then(Value::as_object)
            .and_then(|a| a.get("error"));
        if let Some(err) = error
            && is_truthy(err)
        {
            return Ok(Some(json!({ "text": err, "failed": true })));
        }
        return Ok(None);
    }
    Ok(Some(json!({ "text": "", "failed": false })))
}

/// Port of `mergeRelayBillingResult` (`src/relay-billing.ts:123`).
pub fn merge_relay_billing_result(
    previous: Option<Value>,
    probe: Value,
    now: f64,
) -> Result<Value, String> {
    let obj = require_obj(&probe, "outcome")?;
    let outcome = obj.get("outcome").and_then(Value::as_str);
    if outcome == Some("recognized") {
        return Ok(json!({
            "entry": { "probe": probe, "at": js::number(now), "failure": null },
            "negative": "clear",
        }));
    }
    if outcome == Some("notRelay") {
        return Ok(json!({
            "entry": null,
            "negative": "remember",
        }));
    }
    let prev_obj = previous
        .as_ref()
        .filter(|v| !is_nil(v))
        .and_then(Value::as_object);
    let prev_probe = prev_obj
        .and_then(|p| p.get("probe"))
        .filter(|v| !is_nil(v))
        .cloned()
        .unwrap_or(Value::Null);
    let prev_at = prev_obj
        .and_then(|p| p.get("at"))
        .filter(|v| !is_nil(v))
        .cloned()
        .unwrap_or_else(|| json!(0));
    let detail = obj.get("detail").cloned().unwrap_or_else(js::undefined);
    Ok(json!({
        "entry": {
            "probe": prev_probe,
            "at": prev_at,
            "failure": { "at": js::number(now), "detail": detail },
        },
        "negative": "keep",
    }))
}

/// Port of `relayBillingCacheUsable` (`src/relay-billing.ts:137`).
pub fn relay_billing_cache_usable(entry: Option<Value>, now: f64) -> Result<bool, String> {
    let Some(entry) = entry else {
        return Ok(false);
    };
    if !is_truthy(&entry) {
        return Ok(false);
    }
    let Some(obj) = entry.as_object() else {
        // Non-object truthy entry: failure is undefined, probe is undefined !== null, at is undefined -> NaN
        return Ok(false);
    };
    if let Some(failure) = obj.get("failure")
        && is_truthy(failure)
    {
        let failure_at = failure
            .as_object()
            .and_then(|f| f.get("at"))
            .map_or(f64::NAN, to_number);
        return Ok(now - failure_at < RELAY_BILLING_FAILURE_TTL_MS);
    }
    let probe_non_null = obj.get("probe") != Some(&Value::Null);
    let at = obj.get("at").map_or(f64::NAN, to_number);
    Ok(probe_non_null && (now - at) < RELAY_BILLING_POSITIVE_TTL_MS)
}

/// Port of `relayBillingProbeEligible` (`src/relay-billing.ts:154`).
pub fn relay_billing_probe_eligible(profile: Value) -> Result<bool, String> {
    let obj = require_obj(&profile, "relayMode")?;
    let relay_mode = obj.get("relayMode").and_then(Value::as_str);
    let no_auth = obj.get("noAuth").is_some_and(is_truthy);
    if relay_mode == Some("aggregate") || no_auth {
        return Ok(false);
    }
    let official_mix = obj.get("officialMixApiKey").is_some_and(is_truthy);
    if relay_mode == Some("official") && !official_mix {
        return Ok(false);
    }
    let url = upstream_url(obj)?;
    if url.is_empty() {
        return Ok(false);
    }
    let key = active_api_key(obj)?;
    Ok(!key.is_empty())
}

/// Port of `relayBillingCacheKey` (`src/relay-billing.ts:171`).
pub fn relay_billing_cache_key(profile: Value) -> Result<String, String> {
    let obj = require_obj(&profile, "id")?;
    let id = obj.get("id").map_or_else(|| "undefined".into(), js_str);
    let url = upstream_url(obj)?;
    let key = active_api_key(obj)?;
    Ok(format!("{id}|{}", fnv1a_36(&format!("{url}\n{key}"))))
}

/// Port of `relayBillingGatewayCacheKey` (`src/relay-billing.ts:177`).
pub fn relay_billing_gateway_cache_key(entry: Value) -> Result<String, String> {
    let obj = require_obj(&entry, "account")?;
    if obj.get("account").is_some_and(is_truthy)
        || obj.get("official").is_some_and(is_truthy)
        || obj.get("balanceApi").is_some_and(is_truthy)
    {
        return Ok(String::new());
    }
    let key_count_val = obj.get("keyCount");
    let key_count = key_count_val.map_or(f64::NAN, to_number);
    if key_count <= 0.0 {
        return Ok(String::new());
    }
    let base_trimmed = require_str_trim(obj, "baseUrl")?;
    if base_trimmed.is_empty() {
        return Ok(String::new());
    }
    let url = base_trimmed.trim_end_matches('/');
    let access = if let Some(acc) = obj.get("newapiAccess")
        && is_truthy(acc)
    {
        let acc_obj = acc.as_object();
        let masked = acc_obj
            .and_then(|m| m.get("masked"))
            .map_or_else(|| "undefined".into(), js_str);
        let user_id = acc_obj
            .and_then(|m| m.get("userId"))
            .map_or_else(|| "undefined".into(), js_str);
        format!("\n{masked}\n{user_id}")
    } else {
        String::new()
    };
    let id = obj.get("id").map_or_else(|| "undefined".into(), js_str);
    let key_count_str = key_count_val.map_or_else(|| "undefined".into(), js_str);
    let proxy = match obj.get("proxy") {
        Some(v) if !is_nil(v) => js_str(v),
        _ => String::new(),
    };
    let hash = fnv1a_36(&format!("{url}\n{key_count_str}\n{proxy}{access}"));
    Ok(format!("gateway:{id}|{hash}"))
}

/// Port of `formatRelayMultiplier` (`src/relay-billing.ts:225`).
pub fn format_relay_multiplier(value: f64) -> Result<String, String> {
    let rounded = round_to_4_js(value);
    Ok(format!("{}x", number_to_string(rounded)))
}

fn format_large_en_us_2dec(value: f64) -> String {
    let raw = number_to_string(value.abs());
    let (digits, exp): (String, usize) = if let Some((mant, e_str)) = raw.split_once('e') {
        let e = e_str.trim_start_matches('+').parse::<usize>().unwrap_or(0);
        (mant.replace('.', ""), e)
    } else {
        let int_len = raw.len();
        (raw, int_len.saturating_sub(1))
    };
    let total_int = exp + 1;
    let mut int_str = digits;
    while int_str.len() < total_int {
        int_str.push('0');
    }
    let mut grouped = String::with_capacity(int_str.len() + int_str.len() / 3 + 4);
    if value.is_sign_negative() {
        grouped.push('-');
    }
    for (i, ch) in int_str.bytes().enumerate() {
        if i > 0 && (int_str.len() - i).is_multiple_of(3) {
            grouped.push(',');
        }
        grouped.push(ch as char);
    }
    grouped.push_str(".00");
    grouped
}

/// Port of `formatRelayAmount` (`src/relay-billing.ts:229`).
pub fn format_relay_amount(value: f64, unit: String) -> Result<String, String> {
    let amount = if value.is_finite() && value.abs() >= 1e21 {
        format_large_en_us_2dec(value)
    } else {
        let base = format_en_us_2dec(value);
        if value.is_sign_negative() && base == "0.00" {
            "-0.00".into()
        } else {
            base
        }
    };
    let trimmed = js_trim(&unit);
    let normalized = trimmed.to_uppercase();
    if normalized.is_empty() || normalized == "USD" {
        return Ok(format!("${amount}"));
    }
    if normalized == "CNY" || normalized == "RMB" {
        return Ok(format!("¥{amount}"));
    }
    Ok(format!("{amount} {trimmed}"))
}

/// Port of `relayAppliedPeak` (`src/relay-billing.ts:238`).
pub fn relay_applied_peak(rate: Option<Value>) -> Result<Option<f64>, String> {
    let Some(rate) = rate.filter(|v| !is_nil(v)) else {
        return Ok(None);
    };
    let Some(peak) = rate
        .as_object()
        .and_then(|o| o.get("appliedPeakMultiplier"))
        .and_then(as_number)
    else {
        return Ok(None);
    };
    if peak.is_finite() && peak != 1.0 {
        Ok(Some(peak))
    } else {
        Ok(None)
    }
}

/// Port of `relayBalanceBadge` (`src/relay-billing.ts:244`).
pub fn relay_balance_badge(usage: Value) -> Result<Option<Value>, String> {
    let obj = require_obj(&usage, "remaining")?;
    let remaining_val = obj.get("remaining");
    if remaining_val == Some(&Value::Null) {
        return Ok(None);
    }
    let remaining_num = match remaining_val {
        None => {
            return Err(
                "TypeError: Cannot read properties of undefined (reading 'toLocaleString')".into(),
            );
        }
        Some(v) if js::is_undefined(v) => {
            return Err(
                "TypeError: Cannot read properties of undefined (reading 'toLocaleString')".into(),
            );
        }
        Some(v) => to_number(v),
    };

    let kind = obj.get("kind").and_then(Value::as_str);
    let quota_limit_val = obj.get("quotaLimit");
    let ratio = if kind == Some("keyQuota")
        && quota_limit_val.is_some_and(is_truthy)
        && quota_limit_val.map_or(f64::NAN, to_number) > 0.0
    {
        let limit = quota_limit_val.map_or(f64::NAN, to_number);
        let raw = remaining_num / limit;
        if raw.is_nan() {
            Some(f64::NAN)
        } else {
            Some(raw.clamp(0.0, 1.0))
        }
    } else {
        None
    };

    let is_valid = obj.get("isValid").is_some_and(is_truthy);
    let rate_limits = match obj.get("rateLimits") {
        Some(Value::Array(arr)) => arr,
        _ => return Err("TypeError: usage.rateLimits.some is not a function".into()),
    };
    let mut window_exhausted = false;
    for w in rate_limits {
        let w_obj = require_obj(w, "remaining")?;
        if w_obj.get("remaining").map_or(f64::NAN, to_number) <= 0.0 {
            window_exhausted = true;
            break;
        }
    }
    let exhausted = remaining_num <= 0.0 || !is_valid || window_exhausted;
    let tone = if exhausted || ratio.is_some_and(|r| r <= 0.1) {
        "low"
    } else if ratio.is_some_and(|r| r <= 0.3) {
        "mid"
    } else {
        "ok"
    };

    let unit = match obj.get("unit") {
        Some(Value::String(s)) => s.clone(),
        Some(v) if is_nil(v) => {
            return Err(
                "TypeError: Cannot read properties of null/undefined (reading 'trim')".into(),
            );
        }
        None => {
            return Err("TypeError: Cannot read properties of undefined (reading 'trim')".into());
        }
        _ => return Err("TypeError: unit.trim is not a function".into()),
    };
    let amount = format_relay_amount(remaining_num, unit)?;
    Ok(Some(json!({
        "amount": amount,
        "ratio": ratio.map(js::number).unwrap_or(Value::Null),
        "tone": tone,
    })))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_profile() -> Value {
        json!({
            "id": "relay-1",
            "baseUrl": "https://relay.example.com/v1",
            "upstreamBaseUrl": "",
            "apiKey": "sk-a",
            "relayMode": "mixedApi",
            "officialMixApiKey": false,
            "noAuth": false,
        })
    }

    #[test]
    fn checks_eligibility_and_cache_keys() {
        assert_eq!(
            relay_billing_probe_eligible(sample_profile()).ok(),
            Some(true)
        );
        let mut official = sample_profile();
        official["relayMode"] = json!("official");
        assert_eq!(relay_billing_probe_eligible(official).ok(), Some(false));

        let k1 = relay_billing_cache_key(sample_profile())
            .ok()
            .unwrap_or_default();
        assert!(!k1.contains("sk-a"));
        let mut slash = sample_profile();
        slash["baseUrl"] = json!("https://relay.example.com/v1/");
        assert_eq!(
            relay_billing_cache_key(slash).ok().as_deref(),
            Some(k1.as_str())
        );
        assert!(relay_billing_cache_key(Value::Null).is_err());
    }

    #[test]
    fn formats_multiplier_amount_and_badge() {
        assert_eq!(format_relay_multiplier(0.9).ok().as_deref(), Some("0.9x"));
        assert_eq!(
            format_relay_multiplier(1.234567).ok().as_deref(),
            Some("1.2346x")
        );
        assert_eq!(
            format_relay_multiplier(0.03125).ok().as_deref(),
            Some("0.0313x")
        );
        assert_eq!(
            format_relay_amount(12.345, "USD".into()).ok().as_deref(),
            Some("$12.35")
        );
        assert_eq!(
            format_relay_amount(97505834.07, "USD".into())
                .ok()
                .as_deref(),
            Some("$97,505,834.07")
        );
        assert_eq!(
            format_relay_amount(3.0, "CNY".into()).ok().as_deref(),
            Some("¥3.00")
        );
        assert_eq!(
            format_relay_amount(3.0, "credits".into()).ok().as_deref(),
            Some("3.00 credits")
        );

        let badge = relay_balance_badge(json!({
            "endpoint": "",
            "kind": "keyQuota",
            "isValid": true,
            "planName": null,
            "remaining": 10,
            "quotaLimit": 50,
            "quotaUsed": 40,
            "unit": "USD",
            "rateLimits": [],
        }))
        .ok()
        .flatten();
        assert_eq!(
            badge,
            Some(json!({"amount": "$10.00", "ratio": 0.2, "tone": "mid"}))
        );
    }
}
