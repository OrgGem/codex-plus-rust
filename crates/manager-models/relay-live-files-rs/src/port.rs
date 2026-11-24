//! The Rust port of `src/relay-live-files.ts`.

#![forbid(unsafe_code)]

use manager_common_rs::{is_nil, js_trim};
use serde_json::Value;

/// Port of `shouldBackfillRelayProfileBeforeSwitch` (`src/relay-live-files.ts:1`).
pub fn should_backfill_relay_profile_before_switch(
    prev: String,
    next: String,
) -> Result<bool, String> {
    let p = js_trim(&prev);
    Ok(!p.is_empty() && p != js_trim(&next))
}

/// Port of `relayAuthForLiveDraft` (`src/relay-live-files.ts:19`).
pub fn relay_auth_for_live_draft(
    profile: Value,
    live_auth_contents: String,
) -> Result<String, String> {
    if is_nil(&profile) {
        return Err(
            "TypeError: Cannot read properties of null/undefined (reading 'relayMode')".into(),
        );
    }
    if profile.get("relayMode").and_then(Value::as_str) == Some("pureApi") {
        Ok(profile
            .get("authContents")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .into())
    } else {
        Ok(live_auth_contents)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn backfill_and_live_draft_auth() {
        assert!(should_backfill_relay_profile_before_switch("a".into(), "b".into()).unwrap());
        assert!(!should_backfill_relay_profile_before_switch("a".into(), "a".into()).unwrap());
        assert!(!should_backfill_relay_profile_before_switch(" ".into(), "a".into()).unwrap());
        let p = |m: &str| json!({"relayMode": m, "authContents": "stored"});
        assert_eq!(
            relay_auth_for_live_draft(p("pureApi"), "live".into()).unwrap(),
            "stored"
        );
        assert_eq!(
            relay_auth_for_live_draft(p("official"), "live".into()).unwrap(),
            "live"
        );
        assert!(relay_auth_for_live_draft(Value::Null, "live".into()).is_err());
    }
}
