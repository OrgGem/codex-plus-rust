use code_highlight_rs::port::tokenize_code;

#[test]
fn lossless_and_line_preserving_for_toml_and_json() {
    let toml = "[features]\ngoals = true # note\nurl = \"https://x/#a\"\n";
    let lines = tokenize_code(toml.into(), "toml".into()).unwrap();
    assert_eq!(lines.len(), 4);
    let rebuilt: String = lines
        .iter()
        .map(|l| {
            l.iter()
                .map(|t| t["text"].as_str().unwrap())
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n");
    assert_eq!(rebuilt, toml);

    let json_lines = tokenize_code("{\"a\": \"😀\", \"b\": null}".into(), "json".into()).unwrap();
    assert_eq!(json_lines.len(), 1);
    assert_eq!(json_lines[0][1]["kind"], "key");
    assert_eq!(json_lines[0][4]["text"], "\"😀\"");
}
