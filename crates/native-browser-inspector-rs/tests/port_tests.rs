use inspect_service_rs::port::{inspect_entry, inspect_service};
use serde_json::json;

const SPECIMEN: &str =
    include_str!("../../../../assets/native-browser/fixtures/service-specimen.mjs");

#[test]
fn verifies_specimen_entry_and_utf8_offsets() {
    assert!(
        inspect_entry(json!(
            "import * as cua from '@oai/cua-repl'; await cua.launch();"
        ))
        .is_ok()
    );
    assert!(inspect_entry(json!("import { launch } from '@oai/cua-repl';")).is_err());

    let res = inspect_service(json!(SPECIMEN)).unwrap();
    assert_eq!(res["start"], json!(2209));
    assert_eq!(res["end"], json!(2215));
    assert_eq!(res["policy"], json!("policy"));
    assert_eq!(res["metadata"], json!("metadata"));
    assert_eq!(
        res["contractSha"],
        json!("eae1b49427aebf3ed3d1119de1c303125c4f78b3b0ca644f055ed07ec2c2be30")
    );

    let prefixed = format!("// 浏览器契约 🚀\n{SPECIMEN}");
    let p_res = inspect_service(json!(prefixed)).unwrap();
    assert_eq!(p_res["start"], json!(2209 + "// 浏览器契约 🚀\n".len()));
    assert_eq!(p_res["contractSha"], res["contractSha"]);

    let adaptive = SPECIMEN.replace(
        "async sendSessionRequest(r,n) {",
        "async sendSessionRequest(r,n) { void 0;",
    );
    assert!(inspect_service(json!(adaptive)).is_ok());
}
