//! Release-mode microbenchmark comparing:
//! 1. Lifted idiomatic Pure-Rust (`rlib`, zero-copy / typed structs)
//! 2. Generated `serde_json::Value` + JSON IPC / WASM serialization boundary

use std::hint::black_box;
use std::time::Instant;

use manager_core_rs::lifted::{
    HighlightLanguage, RequestMetricInput, summarize_recent_requests_typed, tokenize_code_borrowed,
    toml_section_string_value_fast,
};
use manager_core_rs::relay_editing::build_relay_config_toml;
use serde_json::{Value, json};

fn main() {
    let sample_toml = r#"# Codex++ Managed Configuration
model = "gpt-5.4"
model_provider = "openrouter"
model_context_window = 1_000_000
approval_policy = "on-request"
enabled = true
updated_at = 2026-11-24T16:20:00Z

[model_providers.openrouter]
name = "OpenRouter 网关"
base_url = "https://openrouter.ai/api/v1"
wire_api = "responses"
inline = { retries = 3, ratio = 0.85, hex = 0xff_00, oct = 0o755, bin = 0b1010 }
"#;

    let n = 10_000usize;

    // 1. Code Highlighter: Zero-Copy Lifted vs JSON IPC Bridge
    let t0 = Instant::now();
    for _ in 0..n {
        let lines = tokenize_code_borrowed(black_box(sample_toml), HighlightLanguage::Toml);
        black_box(lines.len());
    }
    let hl_lifted_ns = t0.elapsed().as_nanos() as f64 / n as f64;

    let t1 = Instant::now();
    for _ in 0..n {
        let req = serde_json::to_string(black_box(sample_toml)).unwrap();
        let arg: String = serde_json::from_str(&req).unwrap();
        let out = code_highlight_rs::port::tokenize_code(arg, "toml".to_string()).unwrap();
        let wire = serde_json::to_string(&out).unwrap();
        black_box(wire.len());
    }
    let hl_bridge_ns = t1.elapsed().as_nanos() as f64 / n as f64;

    // 2. TOML Section Lookup: Zero-Regex Slice Scanner vs Per-Call fancy_regex + JSON Bridge
    let t2 = Instant::now();
    for _ in 0..n {
        let v = toml_section_string_value_fast(
            black_box(sample_toml),
            "model_providers.openrouter",
            "name",
        );
        black_box(v);
    }
    let sec_lifted_ns = t2.elapsed().as_nanos() as f64 / n as f64;

    let t3 = Instant::now();
    for _ in 0..n {
        let v = provider_name_rs::port::toml_section_string_value(
            black_box(sample_toml).to_string(),
            "model_providers.openrouter".to_string(),
            "name".to_string(),
        )
        .unwrap();
        let wire = serde_json::to_string(&v).unwrap();
        black_box(wire.len());
    }
    let sec_bridge_ns = t3.elapsed().as_nanos() as f64 / n as f64;

    // 3. Overview Request Aggregation (64 log entries): Typed O(N) vs Value + JSON IPC
    let now = 1_700_000_000_000.0;
    let typed_logs: Vec<RequestMetricInput<'_>> = (0..64)
        .map(|i| RequestMetricInput {
            timestamp_ms: now - (i as f64) * 1_000.0,
            duration_ms: 150.0 + (i % 9) as f64 * 45.0,
            success: i % 7 != 0,
            model: if i % 3 == 0 {
                "gpt-5.4"
            } else if i % 3 == 1 {
                "deepseek-r1"
            } else {
                "claude-sonnet-4"
            },
        })
        .collect();
    let json_logs: Vec<Value> = typed_logs
        .iter()
        .map(|item| {
            json!({
                "timestampMs": item.timestamp_ms,
                "durationMs": item.duration_ms,
                "success": item.success,
                "model": item.model,
            })
        })
        .collect();
    let json_logs_wire = serde_json::to_string(&json_logs).unwrap();

    let m = 5_000usize;
    let t4 = Instant::now();
    for _ in 0..m {
        let s = summarize_recent_requests_typed(black_box(&typed_logs), now, None);
        black_box(s.total);
    }
    let ov_lifted_ns = t4.elapsed().as_nanos() as f64 / m as f64;

    let t5 = Instant::now();
    for _ in 0..m {
        let parsed: Vec<Value> = serde_json::from_str(black_box(&json_logs_wire)).unwrap();
        let out = overview_model_rs::port::summarize_recent_requests(parsed, now, None).unwrap();
        let wire = serde_json::to_string(&out).unwrap();
        black_box(wire.len());
    }
    let ov_bridge_ns = t5.elapsed().as_nanos() as f64 / m as f64;

    // 4. Relay Config TOML Generation: Direct in-process call vs JSON IPC payload round-trip
    let mut profile = manager_core_rs::js_only_bridges::create_relay_profile_at(
        &json!({ "relayProfiles": [] }),
        1_700_000_000_000,
        "https://openrouter.ai/api/v1",
        json!({}),
    )
    .unwrap();
    profile["name"] = json!("OpenRouter");
    profile["model"] = json!("gpt-5.4");
    profile["modelList"] = json!("gpt-5.4[1M]\ndeepseek-r1[128K]");
    let profile_wire = serde_json::to_string(&profile).unwrap();

    let t6 = Instant::now();
    for _ in 0..m {
        let out = build_relay_config_toml(black_box(profile.clone()), json!({})).unwrap();
        black_box(out.len());
    }
    let relay_inproc_ns = t6.elapsed().as_nanos() as f64 / m as f64;

    let t7 = Instant::now();
    for _ in 0..m {
        let decoded: Value = serde_json::from_str(black_box(&profile_wire)).unwrap();
        let out = build_relay_config_toml(decoded, json!({})).unwrap();
        let resp_wire = serde_json::to_string(&json!({ "ok": true, "toml": out })).unwrap();
        black_box(resp_wire.len());
    }
    let relay_ipc_ns = t7.elapsed().as_nanos() as f64 / m as f64;

    let report = json!({
        "tokenize_code": {
            "lifted_zero_copy_ns": (hl_lifted_ns * 10.0).round() / 10.0,
            "json_ipc_bridge_ns": (hl_bridge_ns * 10.0).round() / 10.0,
            "speedup_x": ((hl_bridge_ns / hl_lifted_ns) * 100.0).round() / 100.0
        },
        "toml_section_lookup": {
            "lifted_slice_scan_ns": (sec_lifted_ns * 10.0).round() / 10.0,
            "regex_json_bridge_ns": (sec_bridge_ns * 10.0).round() / 10.0,
            "speedup_x": ((sec_bridge_ns / sec_lifted_ns) * 100.0).round() / 100.0
        },
        "overview_64_requests": {
            "lifted_typed_on_ns": (ov_lifted_ns * 10.0).round() / 10.0,
            "value_json_ipc_ns": (ov_bridge_ns * 10.0).round() / 10.0,
            "speedup_x": ((ov_bridge_ns / ov_lifted_ns) * 100.0).round() / 100.0
        },
        "relay_config_toml": {
            "in_process_rlib_ns": (relay_inproc_ns * 10.0).round() / 10.0,
            "json_ipc_roundtrip_ns": (relay_ipc_ns * 10.0).round() / 10.0,
            "speedup_x": ((relay_ipc_ns / relay_inproc_ns) * 100.0).round() / 100.0
        }
    });
    println!("{}", serde_json::to_string_pretty(&report).unwrap());
}
