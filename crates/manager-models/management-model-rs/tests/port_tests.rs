use management_model_rs::port::{
    matches_management_query, normalize_project_path, session_groups, skill_repo_key,
};
use serde_json::json;

#[test]
fn normalizes_windows_paths_queries_and_groups() {
    assert_eq!(
        normalize_project_path(r"\\?\c:\Users\Demo\repo\".into()).unwrap(),
        "C:/Users/Demo/repo"
    );
    assert_eq!(
        normalize_project_path(r"\\?\UNC\srv\share\dir\".into()).unwrap(),
        "//srv/share/dir"
    );
    assert!(
        matches_management_query(vec!["Codex".into(), "Plus".into()], "plus cod".into()).unwrap()
    );
    assert_eq!(
        skill_repo_key(json!({"owner":"o","name":"n","branch":"main","subdir":"sub"})).unwrap(),
        "o/n@main:sub"
    );
    let groups = session_groups(
        vec![json!({"id":"1","title":"T","cwd":"c:\\a","modelProvider":"openai","archived":false,"updatedAtMs":1732400000000.0,"rolloutPath":"","dbPath":""})],
        "project".into(),
        json!(1732410000000.0),
    )
    .unwrap();
    assert_eq!(groups[0]["key"], "C:/a");
}
