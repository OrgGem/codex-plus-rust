use provider_editing_i18n_shim_rs::port::{t, tf};
use serde_json::json;

#[test]
fn formats_placeholders_and_preserves_missing() {
    assert_eq!(t(json!("供应商")).unwrap(), json!("供应商"));
    assert_eq!(
        tf(
            json!("共 {0} 项，剩余 {1}，跳过 {2}"),
            json!(["3", 5, null])
        )
        .unwrap(),
        json!("共 3 项，剩余 5，跳过 {2}")
    );
    assert_eq!(tf(json!("纯文本"), json!(null)).unwrap(), json!("纯文本"));
    assert!(tf(json!("需要 {0}"), json!(null)).is_err());
}
