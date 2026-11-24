//! Sessions, MCP Config, Extensions (with C-3 risks), Enhancements, Config Doctor,
//! Redacted Public Settings Viewer (C-2), and System Diagnostics views.

#![forbid(unsafe_code)]

use crate::backend::{self, AppSnapshot};
use crate::security;
use crate::ui::{Route, render_highlighted_code, tr_text};
use dioxus::prelude::*;
use manager_core_rs::lifted::{
    HighlightLanguage, auto_compact_options_vec, normalize_auto_compact_percent_cow,
};
use manager_core_rs::static_units::mcp_presets;
use serde_json::{Value, json};

pub fn render_management_routes(
    route: Route,
    snap: &AppSnapshot,
    en: bool,
    status_msg: Signal<String>,
    refresh_tick: Signal<u64>,
) -> Element {
    match route {
        Route::Sessions => render_sessions(snap, en, status_msg, refresh_tick),
        Route::Context => render_mcp(snap, en, status_msg, refresh_tick),
        Route::UserScripts | Route::Extension => render_extensions(snap, en),
        Route::Enhance
        | Route::DreamSkin
        | Route::PluginMarket
        | Route::Weixin
        | Route::RemoteControl
        | Route::Claude
        | Route::ClaudePatch
        | Route::ComputerUse
        | Route::AgentCollaboration
        | Route::AgentCache
        | Route::Skills => render_features(route, snap, en, status_msg, refresh_tick),
        Route::Maintenance | Route::Settings | Route::AgentSettings | Route::About => {
            render_system_and_security(route, snap, en)
        }
        _ => rsx! { div {} },
    }
}

fn render_sessions(
    snap: &AppSnapshot,
    en: bool,
    mut status_msg: Signal<String>,
    mut refresh_tick: Signal<u64>,
) -> Element {
    rsx! {
        div { class: "cx-card",
            div { class: "cx-card-header",
                h2 { "{tr_text(\"Quản lý Phiên cục bộ (C-4 Canonical Path Guard)\", \"Local Sessions (C-4 Canonical Path Guard)\", en)} ({snap.sessions.len()})" }
            }
            if snap.sessions.is_empty() {
                p { class: "cx-muted", "{tr_text(\"Không tìm thấy phiên SQLite nào trong Codex Home hiện tại.\", \"No SQLite sessions found under current Codex Home.\", en)}" }
            } else {
                table { class: "cx-table",
                    thead {
                        tr {
                            th { "Title" }
                            th { "Provider" }
                            th { "CWD" }
                            th { "Action" }
                        }
                    }
                    tbody {
                        for sess in &snap.sessions {
                            {
                                let sid = sess.id.clone();
                                let dbp = sess.db_path.clone();
                                let rlp = (!sess.rollout_path.is_empty()).then(|| sess.rollout_path.clone());
                                rsx! {
                                    tr {
                                        td { "{sess.title}" }
                                        td { "{sess.model_provider}" }
                                        td { "{sess.cwd}" }
                                        td {
                                            button {
                                                class: "btn btn-sm",
                                                onclick: move |_| {
                                                    let sid = sid.clone();
                                                    let dbp = dbp.clone();
                                                    let rlp = rlp.clone();
                                                    spawn(async move {
                                                        match backend::delete_session_guarded_async(sid, dbp, rlp).await {
                                                            Ok(msg) => status_msg.set(msg),
                                                            Err(e) => status_msg.set(format!("Đã chặn/lỗi xóa phiên: {e:#}")),
                                                        }
                                                        let next = *refresh_tick.read() + 1;
                                                        refresh_tick.set(next);
                                                    });
                                                },
                                                "{tr_text(\"Xóa an toàn\", \"Safe Delete\", en)}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn render_mcp(
    snap: &AppSnapshot,
    en: bool,
    mut status_msg: Signal<String>,
    mut refresh_tick: Signal<u64>,
) -> Element {
    let presets = mcp_presets();
    rsx! {
        div { class: "cx-page-stack",
            div { class: "cx-card",
                h2 { "MCP Presets (manager-core-rs)" }
                div { class: "cx-profile-grid",
                    for preset in &presets {
                        {
                            let id = preset.id.clone();
                            rsx! {
                                div { class: "cx-profile-card",
                                    div { class: "cx-profile-title", "{preset.name}" }
                                    p { class: "cx-muted", "{preset.description}" }
                                    button {
                                        class: "btn btn-sm btn-primary",
                                        onclick: move |_| {
                                            let id = id.clone();
                                            spawn(async move {
                                                match backend::append_mcp_preset_async(id).await {
                                                    Ok(msg) => status_msg.set(msg),
                                                    Err(e) => status_msg.set(format!("Lỗi thêm MCP: {e:#}")),
                                                }
                                                let next = *refresh_tick.read() + 1;
                                                refresh_tick.set(next);
                                            });
                                        },
                                        "{tr_text(\"Thêm vào config.toml\", \"Add to config.toml\", en)}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
            div { class: "cx-card",
                h3 { "{snap.codex_home}/config.toml" }
                if snap.config_toml_exists {
                    {render_highlighted_code(&snap.config_toml_preview, HighlightLanguage::Toml)}
                } else {
                    {render_highlighted_code("# (config.toml chưa tồn tại)", HighlightLanguage::Toml)}
                }
            }
        }
    }
}

fn render_extensions(snap: &AppSnapshot, en: bool) -> Element {
    let prompt_demo = renderer_inject_rs::prompt_args_expand(
        "review",
        "Review $BRANCH vs $BASE: $1",
        "BRANCH=feat BASE=main quick",
    );
    let prompt_demo_text = prompt_demo.text.unwrap_or_default();
    let sample_route_ok = security::is_renderer_bridge_route_permitted("/backend/status");
    let dangerous_blocked = !security::is_renderer_bridge_route_permitted("/hooks/test-run");

    rsx! {
        div { class: "cx-page-stack",
            div { class: "cx-card",
                h2 { "{tr_text(\"Tiện ích & Chính sách Bridge (C-3 Hardened)\", \"Extensions & Bridge Policy (C-3 Hardened)\", en)}" }
                p { class: "cx-muted",
                    "/backend/status allowed={sample_route_ok} · /hooks/test-run blocked={dangerous_blocked} · prompt_args_expand demo: {prompt_demo_text}"
                }
                if snap.user_scripts.is_empty() {
                    p { class: "cx-muted", "{tr_text(\"Chưa cài user script cộng đồng nào.\", \"No community user scripts installed.\", en)}" }
                } else {
                    for ext in &snap.user_scripts {
                        div { class: "cx-profile-card",
                            div { class: "cx-profile-title",
                                "{ext.name} "
                                if ext.enabled { "[enabled]" } else { "[disabled]" }
                            }
                            if !ext.risks.is_empty() {
                                div { class: "cx-badge-warn", "{ext.risks.join(\" | \")}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn render_features(
    route: Route,
    snap: &AppSnapshot,
    en: bool,
    mut status_msg: Signal<String>,
    mut refresh_tick: Signal<u64>,
) -> Element {
    let toggles = [
        (
            "enhancementsEnabled",
            "Tổng công tắc Tiện ích nâng cao",
            "Master Enhancements",
        ),
        (
            "codexAppModelWhitelistUnlock",
            "Mở khóa danh sách model tùy chỉnh",
            "Model Whitelist Unlock",
        ),
        (
            "codexAppStatsigOverrides",
            "Ghi đè Statsig client flags",
            "Statsig Client Overrides",
        ),
        (
            "codexAppPromptArgs",
            "Mở rộng tham số /prompts:<tên>",
            "Custom Prompt Args Expansion",
        ),
        (
            "codexAppProxyRequestStatus",
            "Thanh trạng thái Proxy /proxy-status",
            "Proxy Status Line",
        ),
        (
            "codexAppUsageStatusLine",
            "Thanh trạng thái Token Usage",
            "Usage Status Line",
        ),
    ];

    rsx! {
        div { class: "cx-card",
            h2 { "{route.title(en)}" }
            div { class: "cx-toggle-list",
                for (key, zh_label, en_label) in toggles {
                    {
                        let checked = snap.settings.get(key).and_then(Value::as_bool).unwrap_or(false);
                        rsx! {
                            label { class: "cx-toggle-row",
                                input {
                                    r#type: "checkbox",
                                    checked: checked,
                                    onchange: move |ev| {
                                        let on = ev.checked();
                                        let patch = json!({ key: on });
                                        spawn(async move {
                                            match backend::patch_settings_async(patch).await {
                                                Ok(_) => status_msg.set(format!("Đã cập nhật `{key}` = {on}")),
                                                Err(e) => status_msg.set(format!("Lỗi lưu: {e:#}")),
                                            }
                                            let next = *refresh_tick.read() + 1;
                                            refresh_tick.set(next);
                                        });
                                    }
                                }
                                span { "{tr_text(zh_label, en_label, en)}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn render_system_and_security(route: Route, snap: &AppSnapshot, en: bool) -> Element {
    let public_dto = serde_json::to_string_pretty(&snap.public_settings).unwrap_or_default();
    let sample_sha_ok = security::verify_release_asset_url(
        "https://github.com/BigPizzaV3/CodexPlusPlus/releases/download/v2.8.0/CodexPlusPlus.exe",
    )
    .is_ok();
    let active_relay_id = snap
        .settings
        .get("activeRelayId")
        .and_then(Value::as_str)
        .unwrap_or("");
    let auto_compact_raw = snap
        .settings
        .get("relayProfiles")
        .and_then(Value::as_array)
        .and_then(|arr| {
            arr.iter()
                .find(|p| p.get("id").and_then(Value::as_str) == Some(active_relay_id))
                .or_else(|| arr.first())
        })
        .and_then(|p| p.get("autoCompactLimit"))
        .and_then(Value::as_str)
        .unwrap_or("");
    let auto_compact_norm = normalize_auto_compact_percent_cow(auto_compact_raw);
    let auto_compact_presets = auto_compact_options_vec(auto_compact_raw).join(", ");

    rsx! {
        div { class: "cx-page-stack",
            div { class: "cx-card",
                h2 { "{route.title(en)}" }
                ul { class: "cx-list",
                    li { "Version: {snap.version}" }
                    li { "State Dir: {snap.state_dir}" }
                    li { "Settings Path: {snap.settings_path}" }
                    li { "Codex Home: {snap.codex_home}" }
                    li { "Native Browser Patch Status: {snap.native_browser_summary}" }
                    li { "Auto-Compact Policy: current='{auto_compact_norm}' · options=[{auto_compact_presets}]" }
                    li { "C-1 HTTPS Updater Check: {sample_sha_ok} · Strict CSP active" }
                }
            }
            div { class: "cx-card",
                h3 { "Config Doctor ({snap.doctor_findings.len()})" }
                if snap.doctor_findings.is_empty() {
                    p { class: "cx-muted", "{tr_text(\"Không phát hiện cảnh báo cấu hình.\", \"No configuration issues found.\", en)}" }
                } else {
                    for f in &snap.doctor_findings {
                        div { class: "cx-profile-card",
                            div { class: "cx-profile-title", "[{f.level}] {f.title}" }
                            p { class: "cx-muted", "{f.detail}" }
                        }
                    }
                }
            }
            div { class: "cx-card",
                h3 { "{tr_text(\"Public Settings DTO (Đã lọc bí mật theo C-2 — Zero-Copy Highlight)\", \"Public Settings DTO (C-2 Secret-Redacted Allowlist — Zero-Copy Highlight)\", en)}" }
                {render_highlighted_code(&public_dto, HighlightLanguage::Json)}
            }
        }
    }
}
