use auto_compact_rs::port::{
    auto_compact_options, is_valid_auto_compact_percent, normalize_auto_compact_editing,
    normalize_auto_compact_percent,
};

#[test]
fn presets_validation_and_editing() {
    assert!(is_valid_auto_compact_percent("".into()).unwrap());
    assert!(is_valid_auto_compact_percent("87.5%".into()).unwrap());
    assert!(!is_valid_auto_compact_percent("0".into()).unwrap());
    assert!(!is_valid_auto_compact_percent("100.1%".into()).unwrap());
    assert_eq!(
        normalize_auto_compact_percent(" 80 ".into()).unwrap(),
        "80%"
    );
    assert_eq!(
        auto_compact_options("87.5".into()).unwrap().last().unwrap(),
        "87.5%"
    );
    assert_eq!(
        normalize_auto_compact_editing("9%2".into(), None).unwrap(),
        "92%"
    );
}
