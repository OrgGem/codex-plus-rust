use std::hint::black_box;
use std::time::Instant;

use manager_core_rs::lifted::{
    HighlightLanguage, RequestMetricInput, ResolvedTheme, ThemePreference,
    auto_compact_options_vec, is_valid_auto_compact_percent_str,
    normalize_auto_compact_percent_cow, provider_avatar_text_fast, resolve_provider_name_fast,
    summarize_recent_requests_typed, tokenize_code_borrowed, toml_section_string_value_fast,
};
use serde_json::{Value, json};

#[test]
fn zero_copy_highlighter_matches_bridge_port_1_to_1() {
    let samples = [
        (
            "toml",
            r#"# Codex++ Managed Configuration
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
multiline = """
line 1
line 2 "quoted"
"""
"#,
        ),
        (
            "json",
            r#"{
  // strict public DTO
  "activeRelayId": "relay-1",
  "count": 42,
  "ratio": -3.5e2,
  "enabled": true,
  "disabled": false,
  "empty": null,
  /* block
     comment */
  "unicode": "Xin chào 供应商 🚀"
}"#,
        ),
    ];

    for (lang, src) in samples {
        let port_json =
            code_highlight_rs::port::tokenize_code(src.to_string(), lang.to_string()).unwrap();
        let lifted_lines = tokenize_code_borrowed(src, HighlightLanguage::parse(lang));
        let lifted_as_json: Vec<Vec<Value>> = lifted_lines
            .iter()
            .map(|line| {
                line.iter()
                    .map(|span| json!({ "kind": span.kind.as_str(), "text": span.text }))
                    .collect()
            })
            .collect();
        assert_eq!(lifted_as_json, port_json, "mismatch for language={lang}");
    }
}

#[test]
fn theme_and_auto_compact_match_unit_ports_1_to_1() {
    for raw in [
        None,
        Some("dark"),
        Some("light"),
        Some("system"),
        Some("other"),
    ] {
        let lifted = ThemePreference::parse(raw);
        let port = theme_mode_rs::port::parse_theme_preference(raw.map(str::to_string)).unwrap();
        assert_eq!(lifted.as_str(), port);

        for sys_dark in [false, true] {
            let res_lifted = lifted.resolve(sys_dark);
            let res_port = theme_mode_rs::port::resolve_theme(port.clone(), sys_dark).unwrap();
            assert_eq!(res_lifted.as_str(), res_port);
        }
    }
    assert_eq!(
        ResolvedTheme::Dark.toggled().as_str(),
        theme_mode_rs::port::toggled_theme_preference("dark".into()).unwrap()
    );
    assert_eq!(
        ResolvedTheme::Light.toggled().as_str(),
        theme_mode_rs::port::toggled_theme_preference("light".into()).unwrap()
    );

    for val in ["", "90%", " 85 ", "72.5%", "0%", "105%", "abc", "100%"] {
        assert_eq!(
            is_valid_auto_compact_percent_str(val),
            auto_compact_rs::port::is_valid_auto_compact_percent(val.to_string()).unwrap(),
            "validity mismatch on {val:?}"
        );
        assert_eq!(
            normalize_auto_compact_percent_cow(val).as_ref(),
            auto_compact_rs::port::normalize_auto_compact_percent(val.to_string()).unwrap(),
            "normalize mismatch on {val:?}"
        );
        assert_eq!(
            auto_compact_options_vec(val),
            auto_compact_rs::port::auto_compact_options(val.to_string()).unwrap(),
            "options mismatch on {val:?}"
        );
    }
}

#[test]
fn provider_name_and_overview_match_unit_ports_1_to_1() {
    let toml = r#"
[model_providers.deepseek]
name = "DeepSeek 官方" # comment
base_url = "https://api.deepseek.com/v1"

[model_providers.openrouter]
name = 'OpenRouter \"Gateway\"'
"#;

    for (sec, key) in [
        ("model_providers.deepseek", "name"),
        ("model_providers.openrouter", "name"),
        ("model_providers.missing", "name"),
    ] {
        let fast = toml_section_string_value_fast(toml, sec, key).unwrap_or_default();
        let port = provider_name_rs::port::toml_section_string_value(
            toml.to_string(),
            sec.to_string(),
            key.to_string(),
        )
        .unwrap();
        assert_eq!(fast, port, "section lookup mismatch for {sec}.{key}");
    }

    for prov in ["deepseek", "openrouter", "custom_id"] {
        assert_eq!(
            resolve_provider_name_fast(toml, prov).as_ref(),
            provider_name_rs::port::resolve_provider_name(toml.to_string(), prov.to_string())
                .unwrap()
        );
    }

    for name in [
        "OpenAI",
        "DeepSeek 官方",
        "Claude 3.7",
        " silicon-flow ",
        "智谱清言",
        "---",
        "X",
    ] {
        assert_eq!(
            provider_avatar_text_fast(name, None),
            provider_name_rs::port::provider_avatar_text(name.to_string(), None).unwrap(),
            "avatar mismatch for {name:?}"
        );
    }

    let now = 1_700_000_000_000.0;
    let typed = [
        RequestMetricInput {
            timestamp_ms: now - 5_000.0,
            duration_ms: 420.0,
            success: true,
            model: "gpt-5.4",
        },
        RequestMetricInput {
            timestamp_ms: now - 2_000.0,
            duration_ms: 810.0,
            success: false,
            model: "deepseek-r1",
        },
        RequestMetricInput {
            timestamp_ms: now - 1_000.0,
            duration_ms: 330.0,
            success: true,
            model: "deepseek-r1",
        },
    ];
    let json_entries: Vec<Value> = typed
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

    let lifted_summary =
        serde_json::to_value(summarize_recent_requests_typed(&typed, now, None)).unwrap();
    let port_summary =
        overview_model_rs::port::summarize_recent_requests(json_entries, now, None).unwrap();
    assert_eq!(lifted_summary, port_summary);
}

#[test]
fn lifted_zero_copy_is_measurably_faster_than_json_value_bridge() {
    let sample_toml = r#"# Managed by Codex++
model = "gpt-5.4"
model_provider = "openrouter"
model_context_window = 1000000

[model_providers.openrouter]
name = "OpenRouter Pro"
base_url = "https://openrouter.ai/api/v1"
wire_api = "responses"
"#;

    let iterations = 2_000usize;

    // Warmup
    let _ = tokenize_code_borrowed(sample_toml, HighlightLanguage::Toml);
    let _ = code_highlight_rs::port::tokenize_code(sample_toml.to_string(), "toml".to_string());

    let t0 = Instant::now();
    for _ in 0..iterations {
        let spans = tokenize_code_borrowed(black_box(sample_toml), HighlightLanguage::Toml);
        black_box(spans.len());
    }
    let lifted_ns = t0.elapsed().as_nanos() as f64 / iterations as f64;

    let t1 = Instant::now();
    for _ in 0..iterations {
        let serialized_in = serde_json::to_string(black_box(sample_toml)).unwrap();
        let decoded_in: String = serde_json::from_str(&serialized_in).unwrap();
        let val = code_highlight_rs::port::tokenize_code(decoded_in, "toml".to_string()).unwrap();
        let wire_out = serde_json::to_string(&val).unwrap();
        black_box(wire_out.len());
    }
    let bridge_ns = t1.elapsed().as_nanos() as f64 / iterations as f64;

    println!(
        "LIFT_BENCH tokenize_code: lifted_zero_copy={lifted_ns:.1}ns vs json_bridge={bridge_ns:.1}ns (speedup={:.2}x)",
        bridge_ns / lifted_ns.max(1.0)
    );
    assert!(
        lifted_ns < bridge_ns,
        "Expected zero-copy lifted tokenizer ({lifted_ns:.1}ns) to beat JSON bridge ({bridge_ns:.1}ns)"
    );
}

#[test]
fn regression_utf8_auth_nan_sort_toml_sections_and_relative_index() {
    use manager_core_rs::common::gateway_shared::filter_sessions;
    use manager_core_rs::relay_editing::{
        codex_requires_open_ai_auth_from_config, codex_top_level_int_from_config, toml_string,
    };
    use manager_core_rs::static_units::resolve_relative_index;

    // 1. Multi-byte UTF-8 after `requires_openai_auth =` must never slice mid-char
    assert!(
        !codex_requires_open_ai_auth_from_config("requires_openai_auth = 中文\n".into()).unwrap()
    );
    assert!(
        !codex_requires_open_ai_auth_from_config(
            "[model_providers.custom]\nrequires_openai_auth = 中文\n".into()
        )
        .unwrap()
    );

    // 2. Regex key escaping & TOML control-char escaping
    assert_eq!(
        codex_top_level_int_from_config("a+b[0]( = 42\n".into(), "a+b[0](".into()).unwrap(),
        "42"
    );
    assert_eq!(
        toml_string("a\"b\\c\n\r\t\x08\x0C\x00\x1f\x7f".into()).unwrap(),
        r#"a\"b\\c\n\r\t\b\f\u0000\u001f\u007f"#
    );

    // 3. 70-element `filter_sessions` with missing `updatedAtMs` (`NaN`) must never panic
    let sessions: Vec<Value> = (0..70)
        .map(|i| {
            if i % 2 == 0 {
                json!({
                    "id": format!("sess-{i}"),
                    "agent": "codex",
                    "title": format!("Session {i}"),
                    "cwd": "/repo"
                })
            } else {
                json!({
                    "id": format!("sess-{i}"),
                    "agent": "codex",
                    "title": format!("Session {i}"),
                    "cwd": "/repo",
                    "updatedAtMs": 1_700_000_000_000_u64 + (i as u64) * 1000
                })
            }
        })
        .collect();
    let filtered = filter_sessions(Value::Array(sessions), json!("all"), String::new())
        .expect("filter_sessions 70 items with NaN updatedAtMs");
    assert_eq!(filtered.len(), 70);

    // 4. `[a]\n[[b]]\nname = "wrong"` and trailing header comments must terminate `[a]`
    assert_eq!(
        toml_section_string_value_fast("[a]\n[[b]]\nname = \"wrong\"\n", "a", "name"),
        None
    );
    assert_eq!(
        toml_section_string_value_fast(
            "[a]\n[other] # trailing comment\nname = \"wrong\"\n",
            "a",
            "name"
        ),
        None
    );
    assert_eq!(
        toml_section_string_value_fast("[a] # section comment\nname = \"right\"\n", "a", "name")
            .as_deref(),
        Some("right")
    );

    // 5. `resolve_relative_index` with `f64::INFINITY` / `NEG_INFINITY` / negative offsets
    assert_eq!(resolve_relative_index(3, f64::INFINITY), None);
    assert_eq!(resolve_relative_index(3, f64::NEG_INFINITY), None);
    assert_eq!(resolve_relative_index(3, -1.0), Some(2));
    assert_eq!(resolve_relative_index(3, -3.0), Some(0));
    assert_eq!(resolve_relative_index(3, -4.0), None);
    assert_eq!(resolve_relative_index(3, -1e30), None);

    // 6. `js_rt::math_round` precision on 0.5 predecessor (`0.49999999999999994`)
    let pred_half = f64::from_bits(0.5_f64.to_bits() - 1);
    let summary = summarize_recent_requests_typed(
        &[RequestMetricInput {
            timestamp_ms: 1_000.0,
            duration_ms: pred_half,
            success: true,
            model: "gpt-5.4",
        }],
        2_000.0,
        None,
    );
    assert_eq!(summary.avg_duration_ms, Some(0));
}
