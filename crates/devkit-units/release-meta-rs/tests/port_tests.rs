use release_meta_rs::port::{changelog_section, compare_versions};
use serde_json::json;

#[test]
fn parses_changelog_sections_and_compares_semver() {
    let md = "## [v2.8.0] - 2026-05-01\n- a\n- b\n## 2.7.0\n- old";
    assert_eq!(
        changelog_section(json!("2.8.0"), json!(md)).unwrap(),
        json!("- a\n- b")
    );
    assert_eq!(
        changelog_section(json!("9.9.9"), json!(md)).unwrap(),
        json!("")
    );
    assert_eq!(
        compare_versions(json!("v2.8.0"), json!("2.8.0-rc.1")).unwrap(),
        json!(1)
    );
    assert_eq!(
        compare_versions(json!("2.8.0-alpha"), json!("2.8.0-beta")).unwrap(),
        json!(-1)
    );
    assert_eq!(
        compare_versions(json!("2.8.0"), json!("2.8")).unwrap(),
        json!(0)
    );
}
