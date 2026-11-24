use overview_model_rs::port::summarize_recent_requests;
use serde_json::json;

#[test]
fn summarizes_recent_window_and_top_model() {
    let now = 10.0 * 86_400_000.0;
    let entries = vec![
        json!({"timestampMs": now - 60_000.0, "durationMs": 1000, "success": true, "model": "a"}),
        json!({"timestampMs": now - 30_000.0, "durationMs": 3000, "success": false, "model": "b"}),
        json!({"timestampMs": now - 10_000.0, "durationMs": 2000, "success": true, "model": "b"}),
        json!({"timestampMs": now - 25.0 * 3_600_000.0, "durationMs": 500, "success": true, "model": "old"}),
    ];
    let out = summarize_recent_requests(entries, now, None).unwrap();
    assert_eq!(out["total"], 3);
    assert_eq!(out["failed"], 1);
    assert_eq!(out["avgDurationMs"], 2000);
    assert_eq!(out["topModel"], "b");
}
