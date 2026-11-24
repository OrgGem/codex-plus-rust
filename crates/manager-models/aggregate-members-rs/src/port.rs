//! The Rust port of `src/aggregate-members.ts`.

#![forbid(unsafe_code)]

use manager_common_rs::js_rt::{is_js_ws, is_nil, is_truthy, js_str, js_trim, spread_object};
use serde_json::Value;
use std::collections::HashSet;

fn check_profile(p: &Value, prop: &str) -> Result<(), String> {
    if is_nil(p) {
        Err(format!(
            "TypeError: Cannot read properties of null/undefined (reading '{prop}')"
        ))
    } else {
        Ok(())
    }
}

fn opt_trimmed_str<'a>(v: Option<&'a Value>, prop: &str) -> Result<&'a str, String> {
    match v {
        None => Ok(""),
        Some(val) if is_nil(val) => Ok(""),
        Some(Value::String(s)) => Ok(js_trim(s)),
        Some(_) => Err(format!(
            "TypeError: ({prop} ?? \"\").trim is not a function"
        )),
    }
}

fn has_json_key_colon(haystack: &str, key: &str) -> bool {
    let pat = format!("\"{key}\"");
    let mut start = 0;
    while let Some(pos) = haystack[start..].find(&pat) {
        let after = &haystack[start + pos + pat.len()..];
        if after.trim_start_matches(is_js_ws).starts_with(':') {
            return true;
        }
        start += pos + pat.len();
    }
    false
}

fn set_has_str(set_val: &Value, id: &str) -> Result<bool, String> {
    let arr = set_val
        .as_object()
        .filter(|o| o.len() == 1)
        .and_then(|o| o.get("$set"))
        .and_then(Value::as_array)
        .ok_or("TypeError: set.has is not a function")?;
    Ok(arr.iter().any(|v| v.as_str() == Some(id)))
}

/// Port of `isAggregateProfile` (`src/aggregate-members.ts:20`).
pub fn is_aggregate_profile(profile: Value) -> Result<bool, String> {
    check_profile(&profile, "relayMode")?;
    Ok(
        profile.get("relayMode").and_then(Value::as_str) == Some("aggregate")
            || profile.get("aggregate").is_some_and(is_truthy),
    )
}

/// Port of `isApiMemberProfile` (`src/aggregate-members.ts:25`).
pub fn is_api_member_profile(profile: Value) -> Result<bool, String> {
    check_profile(&profile, "baseUrl")?;
    Ok(
        !opt_trimmed_str(profile.get("baseUrl"), "baseUrl")?.is_empty()
            && !opt_trimmed_str(profile.get("apiKey"), "apiKey")?.is_empty(),
    )
}

/// Port of `authContentsHaveChatgptTokens` (`src/aggregate-members.ts:30`).
pub fn auth_contents_have_chatgpt_tokens(contents: Option<String>) -> Result<bool, String> {
    let trimmed = contents.as_deref().map(js_trim).unwrap_or("");
    if trimmed.is_empty() {
        return Ok(false);
    }
    if let Ok(parsed) = serde_json::from_str::<Value>(trimmed) {
        let tokens = parsed
            .as_object()
            .and_then(|o| o.get("tokens"))
            .and_then(Value::as_object);
        return Ok(tokens.is_some_and(|t| {
            ["access_token", "refresh_token"].iter().any(|k| {
                t.get(*k)
                    .and_then(Value::as_str)
                    .is_some_and(|s| !js_trim(s).is_empty())
            })
        }));
    }
    Ok(has_json_key_colon(trimmed, "tokens")
        && (has_json_key_colon(trimmed, "access_token")
            || has_json_key_colon(trimmed, "refresh_token")))
}

/// Port of `isOfficialLoginMemberProfile` (`src/aggregate-members.ts:46`).
pub fn is_official_login_member_profile(profile: Value) -> Result<bool, String> {
    check_profile(&profile, "relayMode")?;
    if profile.get("relayMode").and_then(Value::as_str) != Some("official")
        || profile.get("officialMixApiKey").is_some_and(is_truthy)
    {
        return Ok(false);
    }
    let proto = profile.get("protocol").filter(|v| !is_nil(v));
    if proto.is_some_and(|v| v.as_str() != Some("responses")) {
        return Ok(false);
    }
    let auth = match profile.get("authContents") {
        None => None,
        Some(v) if is_nil(v) => None,
        Some(Value::String(s)) => Some(s.clone()),
        Some(_) => return Err("TypeError: (contents ?? \"\").trim is not a function".into()),
    };
    auth_contents_have_chatgpt_tokens(auth)
}

/// Port of `aggregateMemberCandidates` (`src/aggregate-members.ts:56`).
pub fn aggregate_member_candidates(
    profiles: Vec<Value>,
    aggregate_id: String,
) -> Result<Vec<Value>, String> {
    let mut out = Vec::new();
    for p in profiles {
        check_profile(&p, "id")?;
        if p.get("id").and_then(Value::as_str) == Some(aggregate_id.as_str())
            || is_aggregate_profile(p.clone())?
        {
            continue;
        }
        if is_api_member_profile(p.clone())? || is_official_login_member_profile(p.clone())? {
            out.push(p);
        }
    }
    Ok(out)
}

/// Port of `normalizeAggregateDefaultRelayId` (`src/aggregate-members.ts:72`).
pub fn normalize_aggregate_default_relay_id(
    value: Option<String>,
    member_ids: Value,
) -> Result<String, String> {
    let id = value.as_deref().map(js_trim).unwrap_or("");
    if id.is_empty() || !set_has_str(&member_ids, id)? {
        Ok(String::new())
    } else {
        Ok(id.to_string())
    }
}

/// Port of `normalizeAggregateMembers` (`src/aggregate-members.ts:101`).
pub fn normalize_aggregate_members(
    members: Value,
    profile_ids: Value,
) -> Result<Vec<Value>, String> {
    let list = match &members {
        v if is_nil(v) => &[] as &[Value],
        Value::Array(arr) => arr.as_slice(),
        Value::String(_) => return Ok(Vec::new()),
        _ => return Err("TypeError: members is not iterable".into()),
    };
    let filter_ids = (!is_nil(&profile_ids) && is_truthy(&profile_ids)).then_some(&profile_ids);
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    for m in list {
        let raw_id = m
            .get("profileId")
            .filter(|v| !is_nil(v))
            .map(js_str)
            .unwrap_or_default();
        let pid = js_trim(&raw_id);
        if pid.is_empty() || seen.contains(pid) {
            continue;
        }
        if let Some(ids) = filter_ids
            && !set_has_str(ids, pid)?
        {
            continue;
        }
        seen.insert(pid.to_string());
        let mut obj = spread_object(m);
        obj.insert("profileId".into(), Value::String(pid.to_string()));
        out.push(Value::Object(obj));
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn member_candidates_and_normalization() {
        let auth = json!({"tokens": {"access_token": "tok"}}).to_string();
        let api =
            json!({"id": "api", "relayMode": "pureApi", "baseUrl": "https://a", "apiKey": "k"});
        let off = json!({"id": "off", "relayMode": "official", "authContents": auth});
        let agg = json!({"id": "agg-1", "relayMode": "aggregate", "aggregate": {"members": []}});
        let cands = aggregate_member_candidates(vec![agg, api, off], "agg-1".into()).unwrap();
        assert_eq!(cands.len(), 2);
        let set = json!({"$set": ["api", "off"]});
        assert_eq!(
            normalize_aggregate_default_relay_id(Some("  api ".into()), set.clone()).unwrap(),
            "api"
        );
        assert_eq!(
            normalize_aggregate_default_relay_id(Some("gone".into()), set).unwrap(),
            ""
        );
        let kept = normalize_aggregate_members(
            json!([{"profileId": " api ", "weight": 1}, {"profileId": "ghost", "weight": 2}]),
            json!({"$set": ["api"]}),
        )
        .unwrap();
        assert_eq!(kept, vec![json!({"profileId": "api", "weight": 1})]);
        assert!(
            auth_contents_have_chatgpt_tokens(Some("{\"tokens\":{\"access_token\":\"a\"".into()))
                .unwrap()
        );
        assert!(is_aggregate_profile(Value::Null).is_err());
    }
}
