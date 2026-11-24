use dictation_settings_rs::port::{default_dictation_settings, normalize_dictation_settings};
use serde_json::json;

#[test]
fn defaults_and_legacy_sensevoice_normalization() {
    let def = default_dictation_settings().unwrap();
    assert_eq!(def["provider"], "api");
    assert_eq!(def["timeoutSeconds"], 120);

    let legacy = normalize_dictation_settings(Some(json!({
        "baseUrl": "LOCAL://SenseVoice/",
        "timeoutSeconds": 9999
    })))
    .unwrap();
    assert_eq!(legacy["provider"], "sensevoice");
    assert_eq!(legacy["baseUrl"], "LOCAL://SenseVoice");
    assert_eq!(legacy["timeoutSeconds"], 600);
}
