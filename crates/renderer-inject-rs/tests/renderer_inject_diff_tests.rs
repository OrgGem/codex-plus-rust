use indexmap::IndexMap;
use renderer_inject_rs::{
    EXTENSION_ALLOWED_ROUTES, StatsigGateRecorder, apply_model_metadata_to_descriptor,
    build_model_descriptor, catalog_belongs_to_profile, escape_css_ident, escape_html,
    extension_coalesce_window_ms, is_extension_route_allowed, model_catalog_identity,
    model_reasoning_efforts, model_response_host_id, model_settings_key, model_settings_source,
    parse_picker_group_suffix, prompt_args_expand, prompt_args_parse_command,
    prompt_args_placeholders, prompt_args_split, sanitize_external_http_url, sanitize_svg_icon,
    statsig_effective_gates, statsig_gate_catalog, statsig_merge_object_value,
    statsig_overrides_signature, statsig_user_gate_overrides, unique_model_values,
};
use serde_json::{Value, json};
use std::process::Command;
use std::sync::OnceLock;

fn run_inject_oracle() -> &'static Value {
    static ORACLE: OnceLock<Value> = OnceLock::new();
    ORACLE.get_or_init(|| {
        let oracle_script = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/node_inject_oracle.mjs");
        let output = Command::new("node")
            .arg(oracle_script)
            .output()
            .expect("node 22 should run node_inject_oracle.mjs");
        if !output.status.success() {
            panic!(
                "node_inject_oracle failed ({}):\nstdout: {}\nstderr: {}",
                output.status,
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
        }
        serde_json::from_slice(&output.stdout).expect("oracle JSON output must parse")
    })
}

#[test]
fn prompt_args_matches_js_shard_1_to_1() {
    let oracle = run_inject_oracle();
    let split_inputs = [
        r#"a  b	"c d" 'e "f"' "g \"h\"" """#,
        r#"KEY="x y" z"#,
        "   ",
        r#""unclosed trailing"#,
        r#"plain1 plain2 "tiếng việt" '中文'"#,
    ];
    let rust_splits: Vec<Vec<String>> = split_inputs.iter().map(|s| prompt_args_split(s)).collect();
    assert_eq!(json!(rust_splits), oracle["promptArgs"]["splitResults"]);

    let placeholder_inputs = [
        "Fix $1 in $2 ($9) costs $$5",
        "All: $ARGUMENTS!",
        "Review $BRANCH against $BASE and $BRANCH",
        "No placeholders at all",
        "Mixed $HOME and $1 with $USER_ID",
    ];
    let rust_placeholders: Vec<Value> = placeholder_inputs
        .iter()
        .map(|s| serde_json::to_value(prompt_args_placeholders(s)).expect("serialize"))
        .collect();
    assert_eq!(
        Value::Array(rust_placeholders),
        oracle["promptArgs"]["placeholderResults"]
    );

    let expand_cases = [
        ("p", "Fix $1 in $2 ($9) costs $$5", r#"bug "src/a b.ts""#),
        ("p", "All: $ARGUMENTS!", r#"one "two three""#),
        ("p", "Review $BRANCH against $BASE", "BRANCH=feat BASE=main"),
        ("p", "Review $BRANCH against $BASE", "BRANCH=feat"),
        ("p", "Review $BRANCH", "BRANCH=feat stray"),
        ("p", "$BRANCH $1", "BRANCH=x"),
        ("p", "Run in $HOME: $1", "build"),
        ("p", "Explain this.", "  foo bar "),
    ];
    let rust_expands: Vec<Value> = expand_cases
        .iter()
        .map(|(id, content, args)| {
            serde_json::to_value(prompt_args_expand(id, content, args)).expect("serialize")
        })
        .collect();
    assert_eq!(
        Value::Array(rust_expands),
        oracle["promptArgs"]["expandResults"]
    );

    let parsed = prompt_args_parse_command(r#"  /prompts:fix  arg1 "b c" "#)
        .map(|(id, args)| json!([id, args]))
        .unwrap_or(Value::Null);
    assert_eq!(parsed, oracle["promptArgs"]["cmdMatchValid"]);
    assert_eq!(prompt_args_parse_command("/prompts:fix"), None);
    assert!(oracle["promptArgs"]["cmdMatchBare"].is_null());
}

#[test]
fn statsig_overrides_and_model_catalog_match_js_shards() {
    let oracle = run_inject_oracle();
    let cfg = json!({
        "enabled": true,
        "gates": { "12345": true, "555": true, "777": false },
        "dynamicConfigs": { "cfg1": { "timeout": 5000, "nested": { "a": 1 } } },
        "experiments": { "exp1": { "enabled": true } },
        "layers": { "lyr1": { "v": 2 } },
        "gateCatalog": [
            { "id": "12345", "title": "Gate A" },
            { "id": 99999, "title": "Num Gate" },
            { "id": "not-numeric", "title": "Skip" }
        ]
    });
    let settings = json!({
        "codexAppStatsigGateOverrides": {
            "12345": "off",
            "99999": "on",
            "555": "native",
            "bad_id": "on",
            "111": "invalid"
        }
    });

    assert_eq!(
        json!(statsig_gate_catalog(&cfg)),
        oracle["statsig"]["gateCatalog"]
    );
    let user_gates = statsig_user_gate_overrides(&settings);
    assert_eq!(json!(user_gates), oracle["statsig"]["userGates"]);
    assert_eq!(
        json!(statsig_effective_gates(&cfg["gates"], &user_gates)),
        oracle["statsig"]["effectiveGates"]
    );
    assert_eq!(
        statsig_overrides_signature(true, &cfg, &settings),
        oracle["statsig"]["signatureOn"].as_str().expect("sig")
    );
    assert_eq!(statsig_overrides_signature(false, &cfg, &settings), "");

    let target1 = json!({
        "name": "cfg1",
        "value": { "timeout": 1000, "keep": "orig", "nested": { "a": 0, "b": 2 } },
        "__evaluation": { "value": { "timeout": 1000 }, "is_user_in_experiment": false, "is_experiment_active": false }
    });
    assert_eq!(
        statsig_merge_object_value(&target1, &cfg["dynamicConfigs"]["cfg1"]),
        oracle["statsig"]["merged1"]
    );
    // Input must remain unmutated (memoized object safety).
    assert_eq!(target1["value"]["timeout"], json!(1000));

    let target2 = json!({ "name": "lyr1", "__value": { "v": 1, "stay": true } });
    assert_eq!(
        statsig_merge_object_value(&target2, &cfg["layers"]["lyr1"]),
        oracle["statsig"]["merged2"]
    );

    // LRU cap test for recorder (500 cap)
    let mut rec = StatsigGateRecorder::default();
    assert!(!rec.record("non-numeric", Some(true), 1));
    for i in 0..505u64 {
        assert!(rec.record(&i.to_string(), Some(i % 2 == 0), i));
    }
    assert_eq!(rec.len(), 500);

    // 70-model-catalog & 71-model-picker-groups
    let unique_in = vec![
        json!("a"),
        json!(""),
        json!("  "),
        json!("b"),
        json!("a"),
        Value::Null,
        json!(123),
        json!("c"),
    ];
    assert_eq!(
        json!(unique_model_values(&unique_in)),
        oracle["modelCatalog"]["unique"]
    );

    let backend_settings = json!({
        "relayProfilesEnabled": true,
        "activeRelayId": "p1",
        "activeAggregateRelayId": "agg1",
        "activeRelayCodexProvider": "custom",
        "modelCatalogSourceIdentity": "src-1",
        "relayProfiles": [
            { "id": "p1", "name": "P1", "relayMode": "pureApi", "protocol": "responses", "model": "gpt-5", "modelList": "gpt-5,o3" },
            { "id": "p2", "name": "P2", "relayMode": "official", "model": "gpt-5" }
        ],
        "aggregateRelayProfiles": [
            { "id": "agg1", "members": [{ "relayId": "p1" }, { "relayId": "p2" }] }
        ]
    });
    assert_eq!(
        model_settings_source(&backend_settings),
        oracle["modelCatalog"]["settingsSource"]
            .as_str()
            .expect("source")
    );
    assert_eq!(
        model_settings_key(&backend_settings),
        oracle["modelCatalog"]["settingsKey"].as_str().expect("key")
    );
    assert_eq!(
        model_catalog_identity(&backend_settings),
        oracle["modelCatalog"]["identity"].as_str().expect("id")
    );

    let sample_catalog = json!({
        "status": "ok",
        "model": "gpt-5.4",
        "default_model": "gpt-5.4",
        "model_provider": "p1",
        "provider_name": "Custom Relay",
        "sources": [{ "id": "relay-profile:p1" }],
        "modelMetadata": {
            "gpt-5.4": {
                "displayName": "GPT 5.4",
                "description": "Flagship",
                "defaultReasoningEffort": "high",
                "inputModalities": ["text", "image"],
                "supportsImageDetailOriginal": true,
                "supportedReasoningEfforts": [
                    { "reasoningEffort": "low", "description": "low d" },
                    { "reasoningEffort": "high", "description": "high d" }
                ]
            },
            "custom-small": {
                "supportedReasoningEfforts": [{ "reasoningEffort": "low", "description": "only low" }]
            }
        }
    });
    assert_eq!(
        catalog_belongs_to_profile(&sample_catalog, &json!({ "id": "p1" })),
        oracle["modelCatalog"]["belongsTrue"].as_bool().expect("b")
    );
    assert_eq!(
        catalog_belongs_to_profile(&sample_catalog, &json!({ "id": "p999" })),
        oracle["modelCatalog"]["belongsFalse"].as_bool().expect("b")
    );
    assert_eq!(
        json!(model_reasoning_efforts(&sample_catalog, "gpt-5.4")),
        oracle["modelCatalog"]["effortsGpt54"]
    );
    assert_eq!(
        json!(model_reasoning_efforts(&sample_catalog, "custom-small")),
        oracle["modelCatalog"]["effortsCustomSmall"]
    );
    assert_eq!(
        json!(model_reasoning_efforts(&sample_catalog, "unknown-model")),
        oracle["modelCatalog"]["effortsUnknown"]
    );
    assert_eq!(
        build_model_descriptor(&sample_catalog, "gpt-5.4"),
        oracle["modelCatalog"]["descGpt54"]
    );

    let mut applied =
        json!({ "model": "gpt-5.4", "displayName": "old", "supportedReasoningEfforts": [] });
    let changed = apply_model_metadata_to_descriptor(&mut applied, &sample_catalog, "GPT-5.4");
    assert_eq!(
        changed,
        oracle["modelCatalog"]["appliedChanged"]
            .as_bool()
            .expect("b")
    );
    assert_eq!(applied, oracle["modelCatalog"]["appliedDesc"]);

    assert_eq!(
        model_response_host_id(&[json!(""), json!({ "hostId": "local" })]),
        oracle["modelCatalog"]["hostLocal"].as_str().expect("host")
    );
    assert_eq!(
        model_response_host_id(&[json!("local"), json!({ "host_id": "ssh-1" })]),
        oracle["modelCatalog"]["hostRemote"].as_str().expect("host")
    );

    let picker_samples = [
        "DeepSeek-V4-Pro（DeepSeek）",
        "Claude Sonnet 4 (Anthropic)",
        "gpt-5.4",
        "Very-Long-Model (ProviderNameThatExceedsTwentyFourCharactersLimitHere)",
        "  spaced   name  ( TeamA ) ",
    ];
    let rust_picker: Vec<Value> = picker_samples
        .iter()
        .map(|s| {
            let (name, provider) = parse_picker_group_suffix(s);
            json!({ "name": name, "provider": provider })
        })
        .collect();
    assert_eq!(Value::Array(rust_picker), oracle["pickerResults"]);
}

#[test]
fn extension_allowlist_and_xss_sanitizers_enforce_hardening() {
    let oracle = run_inject_oracle();
    let expected_routes: Vec<&str> = oracle["extApi"]["routes"]
        .as_array()
        .expect("routes")
        .iter()
        .map(|v| v.as_str().expect("route"))
        .collect();
    assert_eq!(EXTENSION_ALLOWED_ROUTES, expected_routes.as_slice());

    for route in &expected_routes {
        assert!(is_extension_route_allowed(route));
    }
    // C-3 hardening: dangerous backend execution & write routes blocked.
    for blocked in [
        "/hooks/test-run",
        "/hooks/file/write",
        "/settings/set",
        "/delete",
    ] {
        assert!(!is_extension_route_allowed(blocked));
    }

    let expected_coalesce = oracle["extApi"]["coalesce"]
        .as_object()
        .expect("coalesce map");
    for (route, ms) in expected_coalesce {
        assert_eq!(extension_coalesce_window_ms(route), ms.as_u64());
    }

    // H-5 DOM XSS sanitizers
    assert_eq!(
        escape_html(r#"<img src=x onerror="alert('xss')">"#),
        "&lt;img src=x onerror=&quot;alert(&#39;xss&#39;)&quot;&gt;"
    );
    assert_eq!(
        escape_css_ident("a\"][onclick=alert(1)]"),
        r#"a\"\]\[onclick\=alert\(1\)\]"#
    );
    assert!(sanitize_svg_icon(r#"<svg viewBox="0 0 16 16"><path d="M0 0"/></svg>"#).is_some());
    assert!(
        sanitize_svg_icon(
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" aria-hidden="true"><defs><linearGradient id="grad-1"><stop offset="0%" stop-color="#fff"/></linearGradient><clipPath id="clip_0"><rect x="0" y="0" width="24" height="24"/></clipPath></defs><title>Safe &amp; Clean</title><g clip-path="url(#clip_0)" clip-rule="evenodd"><path d="M0 0h24v24z" fill="url(#grad-1)"/></g></svg>"##
        )
        .is_some()
    );
    assert!(sanitize_svg_icon(r#"<svg onload="alert(1)"></svg>"#).is_none());
    assert!(sanitize_svg_icon(r#"<svg></svg><script>alert(1)</script>"#).is_none());
    assert!(sanitize_svg_icon(r#"<svg></svg ><div><img src=x onerror=alert(1)></div>"#).is_none());
    assert!(
        sanitize_svg_icon(r#"<svg></svg ><div><script>alert(1)</script></div></svg>"#).is_none()
    );
    assert!(sanitize_svg_icon(r#"<svg><foreignObject/></svg>"#).is_none());
    assert!(
        sanitize_svg_icon(r#"<svg><set attributeName="onload" to="alert(1)"/></svg>"#).is_none()
    );
    assert!(
        sanitize_svg_icon(r#"<svg><a href="&#x6a;avascript:alert(1)"><path d="M0 0"/></a></svg>"#)
            .is_none()
    );
    assert!(sanitize_svg_icon(r#"<svg fill="&#x6a;avascript:alert(1)"></svg>"#).is_none());
    assert!(sanitize_svg_icon(r#"<svg fill="java&colon;script"></svg>"#).is_none());
    assert!(
        sanitize_svg_icon(
            r#"<svg><path d="M0 0" fill="url(https://evil.example/x.svg#a)"/></svg>"#
        )
        .is_none()
    );
    assert!(sanitize_svg_icon(r#"<svg><!-- comment --></svg>"#).is_none());
    assert!(sanitize_svg_icon(r#"<svg><![CDATA[payload]]></svg>"#).is_none());
    assert!(sanitize_svg_icon(r#"<?xml version="1.0"?><svg></svg>"#).is_none());
    assert_eq!(
        sanitize_external_http_url("https://example.com/a?x=1").as_deref(),
        Some("https://example.com/a?x=1")
    );
    assert!(sanitize_external_http_url("javascript:alert(1)").is_none());
    assert!(sanitize_external_http_url("data:text/html,<script>").is_none());
    assert!(sanitize_external_http_url("https://openai.com:443@evil.example/").is_none());
    assert!(sanitize_external_http_url("https://user@evil.example/").is_none());
    assert!(sanitize_external_http_url("https://:secret@evil.example/").is_none());
    assert!(sanitize_external_http_url("https://example.com/a\x07b").is_none());
    assert!(sanitize_external_http_url("https://example.com/a\nb").is_none());
    assert_eq!(EXTENSION_ALLOWED_ROUTES.len(), 25);

    // Ensure `IndexMap` helper type works cleanly without unused warnings.
    let mut dummy = IndexMap::new();
    dummy.insert("1".to_string(), "on".to_string());
    assert!(!dummy.is_empty());
}

#[test]
fn renderer_inject_wasm_bridge_preserves_prototypes_and_nullish_coercion() {
    let bridge_url = format!(
        "file:///{}",
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/bridge/renderer-inject-wasm-bridge.js"
        )
        .replace('\\', "/")
    );
    let merged_cfg_sample = statsig_merge_object_value(
        &json!({
            "value": { "timeout": 1000, "keep": "orig" },
            "__evaluation": { "value": { "timeout": 1000 }, "is_user_in_experiment": false, "is_experiment_active": false }
        }),
        &json!({ "timeout": 5000, "zeroVal": 0 }),
    );
    let mut desc_sample = json!({ "model": "gpt-5.4", "displayName": "old" });
    let sample_catalog = json!({
        "modelMetadata": {
            "gpt-5.4": { "displayName": "GPT 5.4", "description": "Flagship" }
        }
    });
    let changed = apply_model_metadata_to_descriptor(&mut desc_sample, &sample_catalog, "GPT-5.4");
    assert!(changed);

    let script = format!(
        r#"
        import {{ installRendererInjectWasmBridge }} from {bridge_url:?};
        const calls = [];
        const wasmMock = {{
          promptArgsSplitJson: (s) => {{ calls.push(["split", s]); return JSON.stringify(s ? [s] : []); }},
          promptArgsExpandJson: (id, c, a) => {{ calls.push(["expand", id, c, a]); return JSON.stringify({{ id, c, a }}); }},
          statsigEffectiveGatesJson: (d, b) => JSON.stringify({{}}),
          statsigMergeObjectJson: (targetJson, ovJson) => {{
            calls.push(["mergeIn", JSON.parse(targetJson), JSON.parse(ovJson)]);
            return {merged_str:?};
          }},
          modelCatalogIdentityJson: (s) => "id",
          buildModelDescriptorJson: (cat, m) => {{ calls.push(["buildDesc", m]); return JSON.stringify({{ model: m }}); }},
          applyModelMetadataJson: (descJson, catJson, m) => {{
            const parsed = JSON.parse(descJson);
            calls.push(["applyMeta", m]);
            if (parsed.displayName === "GPT 5.4") {{
              return JSON.stringify({{ changed: false, descriptor: parsed }});
            }}
            return JSON.stringify({{ changed: true, descriptor: {desc_str} }});
          }},
          parsePickerGroupSuffixJson: (l) => {{ calls.push(["picker", l]); return JSON.stringify({{ name: l, provider: "" }}); }},
          isExtensionRouteAllowed: (r) => {{ calls.push(["route", r]); return r === "/proxy-status"; }},
          escapeHtml: (s) => s,
          escapeCssIdent: (s) => s,
          sanitizeSvgIcon: (s) => s || undefined,
          sanitizeExternalHttpUrl: (s) => s || undefined,
        }};
        const api = installRendererInjectWasmBridge(wasmMock);

        // 1. Primitive/null targets returned unchanged
        if (api.statsigMergeObject(null, {{ a: 1 }}) !== null) throw new Error("null target");
        if (api.statsigMergeObject("raw", {{ a: 1 }}) !== "raw") throw new Error("string target");

        // 2. Prototype, methods, non-JSON fields, and get() preserved on Statsig instance
        class StatsigDynamicConfig {{
          constructor() {{
            this.name = "cfg1";
            this.value = {{ timeout: 1000, keep: "orig" }};
            this.__evaluation = {{ value: {{ timeout: 1000 }}, is_user_in_experiment: false, is_experiment_active: false }};
            this.unserializableFn = () => "alive";
          }}
          ruleName() {{ return `rule:${{this.name}}`; }}
          get(k, fb) {{ return Object.prototype.hasOwnProperty.call(this.value, k) ? this.value[k] : fb; }}
        }}
        const inst = new StatsigDynamicConfig();
        const out = api.statsigMergeObject(inst, {{ timeout: 5000, zeroVal: 0 }});
        if (out !== inst) throw new Error("must mutate and return target in place");
        if (!(out instanceof StatsigDynamicConfig)) throw new Error("prototype lost");
        if (out.ruleName() !== "rule:cfg1") throw new Error("prototype method broken");
        if (out.unserializableFn() !== "alive") throw new Error("instance method clobbered");
        if (out.get("timeout", -1) !== 5000) throw new Error("get overridden failed");
        if (out.get("zeroVal", 99) !== 0) throw new Error("get falsy 0 failed");
        if (out.get("missing", "fb") !== "fb") throw new Error("get fallback failed");
        const mergeSentKeys = Object.keys(calls.find((c) => c[0] === "mergeIn")[1]);
        if (mergeSentKeys.includes("name")) throw new Error("should serialize only value/__value/__evaluation");

        // 3. applyModelMetadata mutates descriptor in place and returns boolean
        if (api.applyModelMetadata(null, {{}}, "gpt-5.4") !== false) throw new Error("null descriptor");
        if (api.applyModelMetadata("str", {{}}, "gpt-5.4") !== false) throw new Error("str descriptor");
        const desc = {{ model: "gpt-5.4", displayName: "old", helper: () => 42 }};
        const ch1 = api.applyModelMetadata(desc, {{}}, "GPT-5.4");
        if (ch1 !== true) throw new Error("expected true on first apply");
        if (desc.displayName !== "GPT 5.4" || desc.description !== "Flagship") throw new Error("descriptor not updated");
        if (desc.helper() !== 42) throw new Error("descriptor helper lost");
        const ch2 = api.applyModelMetadata(desc, {{}}, "GPT-5.4");
        if (ch2 !== false) throw new Error("expected false when unchanged");

        // 4. ?? "" preserves falsy non-nullish values like 0 / false while turning null/undefined into ""
        api.promptArgsSplit(0);
        api.promptArgsSplit(null);
        api.promptArgsSplit(undefined);
        api.parsePickerGroupSuffix(0);
        api.isExtensionRouteAllowed(false);
        const splitZero = calls.find((c) => c[0] === "split" && c[1] === "0");
        const splitEmpty = calls.filter((c) => c[0] === "split" && c[1] === "");
        if (!splitZero || splitEmpty.length !== 2) throw new Error("nullish coercion mismatch: " + JSON.stringify(calls));
        console.log("OK");
        "#,
        merged_str = serde_json::to_string(&merged_cfg_sample).unwrap(),
        desc_str = serde_json::to_string(&desc_sample).unwrap(),
    );

    let output = Command::new("node")
        .args(["--input-type=module", "-e", &script])
        .output()
        .expect("run node bridge contract test");
    assert!(
        output.status.success(),
        "bridge contract test failed:\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}
