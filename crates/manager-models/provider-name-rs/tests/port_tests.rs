use provider_name_rs::port::{
    provider_avatar_text, resolve_provider_name, toml_section_string_value,
};

#[test]
fn section_lookup_and_avatar_rules() {
    let cfg = "[model_providers.custom]\nname = 'My \\'Relay\\''\n";
    assert_eq!(
        resolve_provider_name(cfg.into(), "custom".into()).unwrap(),
        "My 'Relay'"
    );
    assert_eq!(
        toml_section_string_value(
            "name = \"root\"\n".into(),
            "model_providers.custom".into(),
            "name".into()
        )
        .unwrap(),
        ""
    );
    assert_eq!(provider_avatar_text("DeepSeek".into(), None).unwrap(), "DS");
    assert_eq!(
        provider_avatar_text("供应商 4".into(), None).unwrap(),
        "供4"
    );
    assert_eq!(
        provider_avatar_text("!!!".into(), Some("?".into())).unwrap(),
        "?"
    );
}
