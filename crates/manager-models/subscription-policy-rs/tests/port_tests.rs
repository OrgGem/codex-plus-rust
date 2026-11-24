use serde_json::json;
use subscription_policy_rs::port::{
    default_subscription_policy, normalize_subscription_policy, subscription_percent_limit,
    with_subscription_window_limit,
};

#[test]
fn limit_validation_and_window_updates() {
    let def = default_subscription_policy().unwrap();
    assert_eq!(def["maxAgeSeconds"], 600);
    assert_eq!(subscription_percent_limit("".into()).unwrap(), json!(null));
    assert_eq!(
        subscription_percent_limit("100".into()).unwrap(),
        json!(null)
    );
    assert_eq!(subscription_percent_limit("80".into()).unwrap(), json!(80));
    assert_eq!(
        subscription_percent_limit("101".into()).unwrap(),
        json!({"$undefined": true})
    );

    let norm = normalize_subscription_policy(None).unwrap();
    let updated = with_subscription_window_limit(norm, "300".into(), "75".into()).unwrap();
    assert_eq!(updated["windowUsedPercentLimits"]["300"], 75);
    let cleared = with_subscription_window_limit(updated, "300".into(), "100".into()).unwrap();
    assert!(cleared["windowUsedPercentLimits"].get("300").is_none());
}
