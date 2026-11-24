use goals_config_rs::port::{
    codex_goals_feature_state, codex_goals_feature_value, set_codex_goals_feature_in_config,
};

#[test]
fn reads_and_sets_goals_only_in_features_table() {
    assert_eq!(
        codex_goals_feature_value("[features]\ngoals = true\n".into()).unwrap(),
        Some(true)
    );
    assert_eq!(
        codex_goals_feature_value("[features.other]\ngoals = true\n".into()).unwrap(),
        None
    );

    let state =
        codex_goals_feature_state("".into(), "[features]\ngoals = true\n".into(), true).unwrap();
    assert_eq!(state["enabled"], true);
    assert_eq!(state["inherited"], true);

    let updated = set_codex_goals_feature_in_config("model = \"gpt-5\"\n".into(), false).unwrap();
    assert_eq!(updated, "model = \"gpt-5\"\n\n[features]\ngoals = false\n");
}
