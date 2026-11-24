use canvas_organize_rs::port::{organization_prompt, parse_organization, validate_organization};
use serde_json::json;

#[test]
fn prompts_validates_and_parses_task_tree() {
    let msgs = json!([
        {"id": "m1", "role": "user", "text": "goal"},
        {"id": "m2", "role": "assistant", "text": "done"}
    ]);
    let prompt = organization_prompt(msgs.clone(), json!("req-1")).unwrap();
    assert!(prompt.as_str().unwrap().contains("req-1"));

    let valid_json = json!({
        "requestId": "req-1",
        "currentNodeId": "n1",
        "nodes": [{
            "id": "n1",
            "parent": null,
            "lane": "main",
            "title": "Goal",
            "summary": "Summary",
            "description": "Details",
            "status": "已确认",
            "sources": ["m1", "m1", "m2"]
        }]
    });
    let tree = validate_organization(valid_json.clone(), msgs.clone(), json!("req-1")).unwrap();
    assert_eq!(tree["nodes"][0]["sources"].as_array().unwrap().len(), 2);

    let fenced = format!("```json\n{valid_json}\n```");
    assert!(parse_organization(json!(fenced), msgs.clone(), json!("req-1")).is_ok());
    assert!(parse_organization(json!("invalid"), msgs, json!("req-1")).is_err());
}
