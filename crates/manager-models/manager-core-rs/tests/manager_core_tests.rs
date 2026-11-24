use manager_core_rs::common::gateway_shared::mask_account_name;
use manager_core_rs::js_only_bridges::{
    OpenPopoverRegistry, ProviderBalanceStore, UpdateNotificationsState, busy_links,
    create_aggregate_relay_profile_at, create_relay_api_key_at, create_relay_profile_at,
    default_dream_skin_theme_for_ua, default_picked, duplicate_relay_profile_at, hub_providers,
    is_version_skipped, returning_links, route_records_by_entry, unlisted_models,
};
use manager_core_rs::relay_editing::{
    build_relay_auth_json, build_relay_config_toml, derive_relay_profile_from_files,
    normalize_relay_profile,
};
use manager_core_rs::static_units::{
    BACKEND_SETTINGS_CHANGED_EVENT, BRAND_ICON_KEYS, EscapeLayerStack, FlowProviderColorStore,
    MAC_TITLEBAR_CLASS, SkillRepoRef, SkillSourceScanState, SkillSourceScanStatus,
    assign_provider_slot, brand_icon_asset_filename, brand_icon_asset_map, cn_classes, en_dict,
    get_i18n_keys_counts, mcp_preset_by_id, mcp_preset_toml_body, mcp_presets, presets,
    resolve_relative_index, skill_source_repo_key, translate_en, translate_template_en,
    uses_mac_overlay_titlebar, vlm_test_translation_zh,
};
use serde_json::{Value, json};
use std::collections::HashMap;
use std::process::Command;
use std::sync::OnceLock;

const FIXED_NOW_MS: u64 = 1_700_000_000_000;

fn run_node_oracle() -> &'static Value {
    static ORACLE: OnceLock<Value> = OnceLock::new();
    ORACLE.get_or_init(|| {
        let loader_url = "file:///D:/Git/codex-plus/rust-port/crates/manager-models/ts-loader.mjs";
        let oracle_script = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/node_oracle.mjs");
        let output = Command::new("node")
            .current_dir(r"d:\Git\codex-plus\apps\codex-plus-manager")
            .arg(format!("--import={loader_url}"))
            .arg(oracle_script)
            .output()
            .expect("node 22 should run oracle script");
        if !output.status.success() {
            panic!(
                "node oracle failed ({}):\nstdout: {}\nstderr: {}",
                output.status,
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
        }
        serde_json::from_slice(&output.stdout).expect("oracle output must be valid JSON")
    })
}

#[test]
fn i18n_en_and_presets_and_mcp_match_ts_exactly() {
    let oracle = run_node_oracle();

    // 1. presets.ts
    assert_eq!(presets(), &oracle["presets"]);

    // 2. i18n-en.ts (exact key counts and full dictionary comparison)
    let counts = get_i18n_keys_counts();
    assert_eq!(
        counts.get("plain").copied(),
        oracle["i18nCounts"]["plain"].as_u64().map(|n| n as usize)
    );
    assert_eq!(
        counts.get("template").copied(),
        oracle["i18nCounts"]["template"]
            .as_u64()
            .map(|n| n as usize)
    );
    assert_eq!(
        counts.get("backend").copied(),
        oracle["i18nCounts"]["backend"].as_u64().map(|n| n as usize)
    );
    assert_eq!(
        counts.get("backendPatterns").copied(),
        oracle["i18nCounts"]["patterns"]
            .as_u64()
            .map(|n| n as usize)
    );

    let dict = en_dict();
    let plain_obj = oracle["i18nPlain"].as_object().expect("i18nPlain map");
    for (k, v) in plain_obj {
        let expected = v.as_str().expect("string translation");
        assert_eq!(
            dict.plain.get(k.as_str()).map(String::as_str),
            Some(expected),
            "plain i18n mismatch for {k}"
        );
        assert_eq!(translate_en(k), expected);
    }

    let tpl_obj = oracle["i18nTemplate"]
        .as_object()
        .expect("i18nTemplate map");
    for (k, v) in tpl_obj {
        let expected = v.as_str().expect("template string");
        assert_eq!(
            dict.template.get(k.as_str()).map(String::as_str),
            Some(expected),
            "template i18n mismatch for {k}"
        );
    }
    assert_eq!(
        translate_template_en("供应商 {0}", &[json!(3)]),
        "Provider 3"
    );

    let backend_obj = oracle["i18nBackend"].as_object().expect("i18nBackend map");
    for (k, v) in backend_obj {
        let expected = v.as_str().expect("backend string");
        assert_eq!(
            dict.backend.get(k.as_str()).map(String::as_str),
            Some(expected),
            "backend i18n mismatch for {k}"
        );
    }

    let samples = oracle["backendSamples"].as_array().expect("samples");
    let translated = oracle["backendTranslated"].as_array().expect("translated");
    for (src, exp) in samples.iter().zip(translated.iter()) {
        assert_eq!(
            translate_en(src.as_str().expect("src")),
            exp.as_str().expect("exp"),
            "backend pattern mismatch for {src}"
        );
    }

    // 3. mcp-presets.ts
    let rust_mcp: Value = serde_json::to_value(mcp_presets()).expect("serialize mcp_presets");
    assert_eq!(rust_mcp, oracle["mcpPresets"]);
    for row in oracle["mcpTomls"].as_array().expect("mcpTomls") {
        let id = row["id"].as_str().expect("id");
        assert!(mcp_preset_by_id(id).is_some());
        assert_eq!(
            mcp_preset_toml_body(id, false).as_deref(),
            row["unix"].as_str(),
            "unix toml mismatch for {id}"
        );
        assert_eq!(
            mcp_preset_toml_body(id, true).as_deref(),
            row["windows"].as_str(),
            "windows toml mismatch for {id}"
        );
    }
    assert!(mcp_preset_by_id("unknown-id").is_none());
    assert!(mcp_preset_toml_body("unknown-id", false).is_none());
}

#[test]
fn static_dom_and_utility_units_match_ts() {
    let oracle = run_node_oracle();

    assert_eq!(
        BACKEND_SETTINGS_CHANGED_EVENT,
        oracle["settingsEventName"].as_str().expect("event name")
    );
    assert_eq!(
        MAC_TITLEBAR_CLASS,
        oracle["macChecks"]["classConst"]
            .as_str()
            .expect("mac class")
    );

    // vlm-test-translation.ts
    let vlm_exp: Vec<&str> = oracle["vlmCases"]
        .as_array()
        .expect("vlmCases")
        .iter()
        .map(|v| v.as_str().expect("str"))
        .collect();
    let vlm_actual = vec![
        vlm_test_translation_zh("ok", None, 345.0),
        vlm_test_translation_zh("http_error", Some(401), 120.0),
        vlm_test_translation_zh("http_error", Some(404), 100.0),
        vlm_test_translation_zh("http_error", Some(500), 1850.0),
        vlm_test_translation_zh("timeout", None, 0.0),
    ];
    assert_eq!(vlm_actual, vlm_exp);

    // flow-provider-colors.ts
    let mut reg = Vec::new();
    let s1 = assign_provider_slot(&mut reg, "p-alpha");
    let s2 = assign_provider_slot(&mut reg, "p-beta");
    let s3 = assign_provider_slot(&mut reg, "p-gamma");
    let s4 = assign_provider_slot(&mut reg, "p-delta");
    let s1_again = assign_provider_slot(&mut reg, "p-alpha");
    let s_empty = assign_provider_slot(&mut reg, "");
    assert_eq!(
        json!([s1, s2, s3, s4, s1_again, s_empty]),
        oracle["colorSlots"]
    );
    let serialized = FlowProviderColorStore { slots: reg }.to_json();
    let reloaded = FlowProviderColorStore::from_json(&serialized);
    assert_eq!(reloaded.slots.len(), 4);

    // skill-source-scan.ts
    let repo = SkillRepoRef {
        owner: "anthropics".into(),
        name: "skills".into(),
        branch: "main".into(),
        subdir: "sub".into(),
        enabled: true,
    };
    assert_eq!(
        skill_source_repo_key(&repo),
        oracle["skillRepoKeySample"].as_str().expect("repo key")
    );
    let mut scan = SkillSourceScanState::default();
    assert_eq!(scan.status, SkillSourceScanStatus::Idle);
    scan.ensure(vec![repo.clone()], &[]);
    assert_eq!(scan.status, SkillSourceScanStatus::Scanning);
    scan.pause();
    assert_eq!(scan.status, SkillSourceScanStatus::Paused);
    scan.resume();
    assert_eq!(scan.status, SkillSourceScanStatus::Scanning);
    scan.candidates.push(json!({ "localId": "skill-1" }));
    scan.consume(&["skill-1".into()]);
    assert!(scan.candidates.is_empty());
    scan.dispose();
    assert_eq!(scan.status, SkillSourceScanStatus::Idle);

    // escape-layer.ts
    let mut esc = EscapeLayerStack::default();
    let tok_a = esc.push("layer-a");
    let tok_b = esc.push("layer-b");
    assert_eq!(
        esc.handle_escape_key("Escape", false).as_deref(),
        Some("layer-b")
    );
    assert_eq!(esc.handle_escape_key("Escape", true), None);
    assert_eq!(esc.handle_escape_key("Enter", false), None);
    esc.unregister(tok_b);
    assert_eq!(
        esc.handle_escape_key("Escape", false).as_deref(),
        Some("layer-a")
    );
    esc.unregister(tok_a);
    assert!(esc.is_empty());
    assert_eq!(oracle["escapeChecks"]["escHandled1"], Value::Bool(true));
    assert_eq!(oracle["escapeChecks"]["escHandledComp"], Value::Bool(false));
    assert_eq!(oracle["escapeChecks"]["escHandled2"], Value::Bool(true));

    // mac-titlebar.ts
    assert_eq!(
        uses_mac_overlay_titlebar("?macTitlebar=1", false, "Win32", ""),
        oracle["macChecks"]["macQueryOn"].as_bool().expect("bool")
    );
    assert_eq!(
        uses_mac_overlay_titlebar("", false, "MacIntel", "Mozilla/5.0"),
        oracle["macChecks"]["macNoTauri"].as_bool().expect("bool")
    );
    assert_eq!(
        uses_mac_overlay_titlebar("", true, "MacIntel", "Mozilla/5.0"),
        oracle["macChecks"]["macTauriDarwin"]
            .as_bool()
            .expect("bool")
    );
    assert!(!uses_mac_overlay_titlebar(
        "",
        true,
        "Win32",
        "Windows NT 10.0"
    ));

    // provider-brand-icons.ts
    let expected_icons: Vec<&str> = oracle["brandIconKeys"]
        .as_array()
        .expect("brandIconKeys")
        .iter()
        .map(|v| v.as_str().expect("icon key"))
        .collect();
    assert_eq!(BRAND_ICON_KEYS, expected_icons.as_slice());
    let icon_map = brand_icon_asset_map();
    assert_eq!(icon_map.len(), BRAND_ICON_KEYS.len());
    assert_eq!(
        brand_icon_asset_filename("openai").as_deref(),
        Some("assets/providers/openai.svg")
    );
    assert_eq!(brand_icon_asset_filename("not-a-brand"), None);

    // lib/utils.ts (`cn` pure whitespace/token dedup) & legacy-webkit.ts (`Array.prototype.at`)
    assert_eq!(
        cn_classes(&["px-2 py-1", "", "px-2 text-sm"]),
        "py-1 px-2 text-sm"
    );
    assert_eq!(resolve_relative_index(3, -1.0), Some(2));
    assert_eq!(resolve_relative_index(3, 1.9), Some(1));
    assert_eq!(resolve_relative_index(3, 3.0), None);
    assert_eq!(resolve_relative_index(3, -4.0), None);
    assert_eq!(resolve_relative_index(0, 0.0), None);
    assert_eq!(resolve_relative_index(3, f64::NAN), Some(0));
}

#[test]
fn js_only_bridges_match_ts() {
    let oracle = run_node_oracle();

    let payload = json!({
        "gateway": { "autoModels": true },
        "groups": [
            { "id": "g1", "disabled": false, "hideMembers": true, "members": [{ "model": "gpt-4o-mini" }, { "model": "group/x" }, { "model": "claude-*" }] },
            { "id": "g2", "disabled": true, "hideMembers": true, "members": [{ "model": "secret-disabled" }] },
        ],
        "autoGroups": [
            { "id": "ag1", "hideMembers": true, "hidden": false, "members": [{ "model": "Auto-Hidden-1" }] },
            { "id": "ag2", "hideMembers": true, "hidden": true, "members": [{ "model": "ignored-hidden" }] },
        ],
        "pool": [
            { "id": "p1", "name": "alpha@example.com", "baseUrl": "https://a.test/v1", "enabled": true, "keyCount": 2 },
            { "id": "p2", "name": "disabled", "baseUrl": "https://b.test/v1", "enabled": false, "keyCount": 1 },
        ],
        "health": { "providers": [], "keys": [] },
        "active": { "relayId": "p1" },
        "activity": {
            "agents": [
                { "agent": "codex", "inFlight": [], "last": { "finishedAtMs": 1_700_000_002_000_u64, "relayId": "p1", "model": "gpt-5" } }
            ]
        }
    });

    let mut rust_unlisted: Vec<String> = unlisted_models(&payload).into_iter().collect();
    rust_unlisted.sort();
    assert_eq!(json!(rust_unlisted), oracle["unlisted"]);

    let meta_preview = json!({
        "windows": [{ "model": "gpt-5", "manual": false }, { "model": "o3", "manual": true }],
        "reasoning": [{ "model": "gpt-5", "manual": false }, { "model": "o1", "manual": true }]
    });
    let mut win_picked: Vec<String> = default_picked(&meta_preview, "windows")
        .into_iter()
        .collect();
    win_picked.sort();
    assert_eq!(json!(win_picked), oracle["metaPickedWin"]);
    let mut r_picked: Vec<String> = default_picked(&meta_preview, "reasoning")
        .into_iter()
        .collect();
    r_picked.sort();
    assert_eq!(json!(r_picked), oracle["metaPickedReason"]);

    let flow_entries = vec![
        json!({ "id": "e1", "agent": "codex", "status": "running", "phase": "generating", "relayId": "p1", "timestampMs": 1_700_000_000_500_u64 }),
        json!({ "id": "e2", "agent": "codex", "status": "running", "phase": "waiting", "relayId": "p2", "timestampMs": 1_700_000_000_900_u64 }),
        json!({ "id": "e3", "agent": "codex", "status": "done", "relayId": "p1", "timestampMs": 1_700_000_001_000_u64 }),
    ];
    let mut rust_busy: Vec<String> = busy_links(&flow_entries).into_iter().collect();
    rust_busy.sort();
    assert_eq!(json!(rust_busy), oracle["busy"]);

    let mut rust_returning: Vec<String> = returning_links(&flow_entries).into_iter().collect();
    rust_returning.sort();
    assert_eq!(json!(rust_returning), oracle["returning"]);

    let rust_routes = route_records_by_entry(&payload, &flow_entries);
    assert_eq!(json!(rust_routes), oracle["routeRecs"]);

    let calls_map = HashMap::from([("p1".to_string(), 42.0)]);
    let rust_hub_provs =
        hub_providers(&payload, Some(&calls_map), FIXED_NOW_MS as f64).expect("hub_providers");
    assert_eq!(Value::Array(rust_hub_provs), oracle["hubProvs"]);

    let created_key = create_relay_api_key_at(
        &[json!({ "id": "key-loy3v6tc", "name": "Existing" })],
        FIXED_NOW_MS,
    );
    assert_eq!(created_key, oracle["createdKey"]);

    let created_profile = create_relay_profile_at(
        &json!({ "relayProfiles": [{ "id": "p0" }] }),
        FIXED_NOW_MS,
        "https://relay.default/v1",
        json!({ "window": true }),
    )
    .expect("create_relay_profile_at");
    assert_eq!(created_profile, oracle["createdProfile"]);

    let mut base_draft = created_profile.clone();
    base_draft["id"] = json!("relay-1");
    base_draft["name"] = json!("One");
    base_draft["model"] = json!("gpt-5");
    base_draft["relayMode"] = json!("official");
    base_draft["apiKey"] = json!("k1");
    base_draft["baseUrl"] = json!("https://u1.test/v1");
    base_draft["upstreamBaseUrl"] = json!("https://u1.test/v1");
    let base_relay =
        normalize_relay_profile(base_draft, manager_core_rs::common::js_rt::undefined())
            .expect("base_relay");

    let created_agg = create_aggregate_relay_profile_at(
        &json!({
            "relayProfiles": [base_relay.clone()]
        }),
        FIXED_NOW_MS,
        json!({ "window": true }),
    )
    .expect("create_aggregate_relay_profile_at");
    assert_eq!(created_agg, oracle["createdAgg"]);

    let duplicated = duplicate_relay_profile_at(
        &json!({
            "activeRelayId": "relay-1",
            "relayProfiles": [base_relay]
        }),
        "relay-1",
        FIXED_NOW_MS,
    )
    .expect("duplicate_relay_profile_at");
    assert_eq!(duplicated, oracle["duplicated"]);

    // update-notification.ts
    let skip_actual = vec![
        is_version_skipped("0.12.0", "0.12.0"),
        is_version_skipped("v0.11.9", "0.12.0"),
        is_version_skipped("0.13.0", "0.12.0"),
        is_version_skipped("0.12.1", "v0.12.1-beta"),
        is_version_skipped("", "0.12.0"),
        is_version_skipped("0.12.0", ""),
    ];
    assert_eq!(json!(skip_actual), oracle["updSkipCases"]);
    let mut upd = UpdateNotificationsState::new();
    upd.set_preferences(true, "0.12.0", false);
    assert!(!upd.should_remind("ok", true, Some("0.12.0"), FIXED_NOW_MS as f64));
    assert!(upd.should_remind("ok", true, Some("0.12.1"), FIXED_NOW_MS as f64));
    upd.mute_today_at(FIXED_NOW_MS as f64);
    assert!(!upd.should_remind("ok", true, Some("0.12.1"), FIXED_NOW_MS as f64));
    assert!(upd.should_prompt("ok", true, Some("0.12.1"), true, FIXED_NOW_MS as f64));

    // provider-balances, popover-layer, dream-skin
    let mut bal = ProviderBalanceStore::default();
    assert!(!bal.is_fresh(1_000.0, Some(500.0)));
    bal.update(vec![json!({ "id": "p1", "remaining": 10 })], 900.0);
    assert!(bal.is_fresh(1_000.0, Some(500.0)));
    assert!(!bal.is_fresh(1_500.0, Some(500.0)));
    bal.reset();
    assert!(!bal.is_fresh(1_000.0, None));

    let mut pops = OpenPopoverRegistry::default();
    pops.register("pop-a");
    let closed = pops.register("pop-b");
    assert_eq!(closed, vec!["pop-a"]);
    assert_eq!(
        Value::from(pops.count()),
        oracle["popChecks"]["popCountAfterB"]
    );
    pops.release("pop-b");
    assert_eq!(
        Value::from(pops.count()),
        oracle["popChecks"]["popCountAfterRelease"]
    );

    assert_eq!(
        default_dream_skin_theme_for_ua(Some("Mozilla/5.0 (Windows NT 10.0)")),
        oracle["dreamThemeWin"]
    );
    assert_eq!(
        default_dream_skin_theme_for_ua(Some("Mozilla/5.0 (Macintosh)")),
        oracle["dreamThemeMac"]
    );
}

#[test]
fn renderer_inject_bundle_matches_relay_editing_and_gateway_shared() {
    let oracle = run_node_oracle();
    let mut sample_profile = create_relay_profile_at(
        &json!({ "relayProfiles": [{ "id": "p0" }] }),
        FIXED_NOW_MS,
        "https://relay.default/v1",
        json!({ "window": true }),
    )
    .expect("sample_profile base");
    sample_profile["id"] = json!("relay-sample");
    sample_profile["name"] = json!("Sample");
    sample_profile["baseUrl"] = json!("https://api.example.com/v1");
    sample_profile["upstreamBaseUrl"] = json!("https://api.example.com/v1");
    sample_profile["apiKey"] = json!("sk-test-123");
    sample_profile["model"] = json!("gpt-5");
    sample_profile["protocol"] = json!("responses");
    sample_profile["relayMode"] = json!("third_party");
    sample_profile["sessionProvider"] = json!("custom");
    sample_profile["customHeaders"] = json!([{ "name": "X-Custom", "value": "abc" }]);

    let undef = manager_core_rs::common::js_rt::undefined();
    let cfg = build_relay_config_toml(sample_profile.clone(), json!({}))
        .expect("build_relay_config_toml");
    let auth = build_relay_auth_json(sample_profile.clone()).expect("build_relay_auth_json");
    let norm =
        normalize_relay_profile(sample_profile.clone(), undef).expect("normalize_relay_profile");
    let mut with_files = sample_profile;
    with_files["configContents"] = Value::String(cfg.clone());
    with_files["authContents"] = Value::String(auth.clone());
    let derived = derive_relay_profile_from_files(with_files, json!("config"))
        .expect("derive_relay_profile_from_files");
    let masked = mask_account_name("alice@example.com".into()).expect("mask_account_name");

    assert_eq!(
        cfg,
        oracle["bundleChecks"]["configToml"].as_str().expect("cfg")
    );
    assert_eq!(
        auth,
        oracle["bundleChecks"]["authJson"].as_str().expect("auth")
    );
    assert_eq!(norm, oracle["bundleChecks"]["normalized"]);
    assert_eq!(derived, oracle["bundleChecks"]["derived"]);
    assert_eq!(
        masked,
        oracle["bundleChecks"]["maskAccountName"]
            .as_str()
            .expect("mask")
    );
    assert!(oracle["bundleChecks"]["exportCount"].as_u64().unwrap_or(0) >= 85);
}
