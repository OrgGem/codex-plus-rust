use serde_json::json;
use whale_settings_rs::port::{
    default_whale_balance_settings, normalize_whale_balance_settings, whale_balance_settings_issue,
};

#[test]
fn normalizes_and_validates_custom_endpoint() {
    let def = default_whale_balance_settings().unwrap();
    assert_eq!(def["codexAppWhaleBalanceCurrency"], "USD");

    let norm = normalize_whale_balance_settings(json!({
        "codexAppWhaleBalanceProtocol": "custom",
        "codexAppWhaleBalancePath": " /v1/balance ",
        "codexAppWhaleBalanceField": "data.items[0].amount",
        "codexAppWhaleBalanceCurrency": " usd ",
        "codexAppWhaleBalanceScale": 0.01
    }))
    .unwrap();
    assert_eq!(
        whale_balance_settings_issue(norm.clone(), true).unwrap(),
        None
    );

    let mut bad = norm;
    bad["codexAppWhaleBalancePath"] = json!("//evil.example/api");
    assert_eq!(
        whale_balance_settings_issue(bad, true).unwrap(),
        Some("path".into())
    );
}
