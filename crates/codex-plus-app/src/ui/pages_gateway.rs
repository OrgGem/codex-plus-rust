//! Gateway, Provider Pool, Relay Profile Editor, and Hub Overview views.

#![forbid(unsafe_code)]

use crate::backend::{self, AppSnapshot};
use crate::ui::{Route, render_highlighted_code, tr_text};
use dioxus::prelude::*;
use manager_core_rs::js_only_bridges::{create_relay_profile_at, duplicate_relay_profile_at};
use manager_core_rs::lifted::{
    HighlightLanguage, RequestMetricInput, provider_avatar_text_fast,
    summarize_recent_requests_typed,
};
use manager_core_rs::relay_editing::{build_relay_config_toml, with_generated_relay_files};
use manager_core_rs::static_units::presets;
use serde_json::{Value, json};
use std::time::{SystemTime, UNIX_EPOCH};

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

pub fn render_gateway_routes(
    route: Route,
    snap: &AppSnapshot,
    en: bool,
    mut status_msg: Signal<String>,
    mut refresh_tick: Signal<u64>,
) -> Element {
    match route {
        Route::Home | Route::Overview | Route::Recommendations => {
            render_overview(snap, en, status_msg, refresh_tick)
        }
        Route::Hub | Route::GatewayProviders | Route::Relay | Route::RelayEnvironment => {
            render_providers_and_relay(route, snap, en, status_msg, refresh_tick)
        }
        Route::RequestLog | Route::GatewayUsage => render_request_logs(snap, en),
        Route::Gateway | Route::GatewayRouting | Route::GatewaySettings | Route::MultiLaunch => {
            render_gateway_control(route, snap, en, &mut status_msg, &mut refresh_tick)
        }
        _ => rsx! { div {} },
    }
}

fn render_overview(
    snap: &AppSnapshot,
    en: bool,
    mut status_msg: Signal<String>,
    mut refresh_tick: Signal<u64>,
) -> Element {
    let presets_count = presets().as_array().map_or(0, Vec::len);
    let active_id = snap
        .settings
        .get("activeRelayId")
        .and_then(Value::as_str)
        .unwrap_or("-");
    let profile_count = snap
        .settings
        .get("relayProfiles")
        .and_then(Value::as_array)
        .map_or(0, Vec::len);
    let offline_label = tr_text("Chưa bật (cổng {0})", "Offline (port {0})", en)
        .replace("{0}", &snap.configured_proxy_port.to_string());
    let metric_inputs: Vec<RequestMetricInput<'_>> = snap
        .request_logs
        .iter()
        .map(|item| RequestMetricInput {
            timestamp_ms: item.timestamp_ms as f64,
            duration_ms: item.duration_ms as f64,
            success: item.success,
            model: &item.model,
        })
        .collect();
    let req_summary = summarize_recent_requests_typed(&metric_inputs, now_ms() as f64, None);
    let rate_label = req_summary
        .success_rate
        .map_or_else(|| "-".to_string(), |r| format!("{:.0}%", r * 100.0));
    let top_model_label = req_summary.top_model.unwrap_or_else(|| "-".to_string());

    rsx! {
        div { class: "cx-page-stack",
            div { class: "cx-card",
                div { class: "cx-card-header",
                    h2 { "{tr_text(\"Codex++ Tổng quan (Pure Rust Dioxus)\", \"Codex++ Overview (Pure Rust Dioxus)\", en)}" }
                    span { class: "cx-badge", "v{snap.version}" }
                }
                div { class: "cx-metrics-grid",
                    div { class: "cx-stat-box",
                        div { class: "cx-stat-label", "{tr_text(\"Gateway Helper\", \"Gateway Helper\", en)}" }
                        div { class: "cx-stat-value",
                            if let Some(port) = snap.helper_port {
                                "127.0.0.1:{port} (HTTP {snap.gateway_http_status.unwrap_or(0)})"
                            } else {
                                "{offline_label}"
                            }
                        }
                    }
                    div { class: "cx-stat-box",
                        div { class: "cx-stat-label", "{tr_text(\"Codex Desktop\", \"Codex Desktop\", en)}" }
                        div { class: "cx-stat-value",
                            if let Some(dir) = &snap.codex_app_dir {
                                "{snap.codex_app_version.clone().unwrap_or_else(|| \"installed\".into())} ({dir})"
                            } else {
                                "{tr_text(\"Không tìm thấy cài đặt cục bộ\", \"Not detected locally\", en)}"
                            }
                        }
                    }
                    div { class: "cx-stat-box",
                        div { class: "cx-stat-label", "{tr_text(\"Nhà cung cấp & Presets\", \"Providers & Presets\", en)}" }
                        div { class: "cx-stat-value", "{profile_count} profiles · active: {active_id} · {presets_count} presets" }
                    }
                    div { class: "cx-stat-box",
                        div { class: "cx-stat-label", "{tr_text(\"Phiên & Request 24h\", \"Sessions & 24h Requests\", en)}" }
                        div { class: "cx-stat-value", "{snap.sessions.len()} sessions · 24h: {req_summary.total} ({rate_label}, top: {top_model_label})" }
                    }
                }
                div { class: "cx-btn-row",
                    button {
                        class: "btn btn-primary",
                        onclick: move |_| {
                            spawn(async move {
                                match backend::ensure_gateway_helper_async().await {
                                    Ok(port) => status_msg.set(format!("Gateway đang lắng nghe trên 127.0.0.1:{port}")),
                                    Err(e) => status_msg.set(format!("Lỗi khởi động Gateway: {e:#}")),
                                }
                                let next = *refresh_tick.read() + 1;
                                refresh_tick.set(next);
                            });
                        },
                        "{tr_text(\"Bật / Kiểm tra Gateway\", \"Start / Probe Gateway\", en)}"
                    }
                    button {
                        class: "btn",
                        onclick: move |_| {
                            spawn(async move {
                                match backend::launch_codex_async().await {
                                    Ok(()) => status_msg.set("Đã kích hoạt khởi chạy Codex Desktop".to_string()),
                                    Err(e) => status_msg.set(format!("Khởi chạy Codex Desktop không khả dụng trên môi trường này: {e:#}")),
                                }
                                let next = *refresh_tick.read() + 1;
                                refresh_tick.set(next);
                            });
                        },
                        "{tr_text(\"Khởi chạy Codex Desktop\", \"Launch Codex Desktop\", en)}"
                    }
                }
            }
            if !snap.codex_processes.is_empty() {
                div { class: "cx-card",
                    h3 { "{tr_text(\"Tiến trình Codex đang chạy\", \"Running Codex Processes\", en)}" }
                    ul { class: "cx-list",
                        for proc_item in &snap.codex_processes {
                            li { "PID {proc_item.pid} — {proc_item.exe}" }
                        }
                    }
                }
            }
        }
    }
}

fn render_providers_and_relay(
    route: Route,
    snap: &AppSnapshot,
    en: bool,
    mut status_msg: Signal<String>,
    mut refresh_tick: Signal<u64>,
) -> Element {
    let settings_clone = snap.settings.clone();
    let settings_for_dup = snap.settings.clone();
    let settings_for_preset = snap.settings.clone();
    let profiles = snap
        .settings
        .get("relayProfiles")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let active_id = snap
        .settings
        .get("activeRelayId")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string();
    let active_profile = profiles
        .iter()
        .find(|p| p.get("id").and_then(Value::as_str) == Some(active_id.as_str()))
        .or_else(|| profiles.first())
        .cloned()
        .unwrap_or_else(|| json!({}));

    let toml_preview = build_relay_config_toml(active_profile.clone(), json!({}))
        .unwrap_or_else(|e| format!("# build_relay_config_toml error: {e}"));
    let preset_items = presets().as_array().cloned().unwrap_or_default();

    rsx! {
        div { class: "cx-page-stack",
            div { class: "cx-card",
                div { class: "cx-card-header",
                    h2 { "{route.title(en)}" }
                    div { class: "cx-btn-row",
                        button {
                            class: "btn btn-primary",
                            onclick: move |_| {
                                let base = settings_clone.clone();
                                spawn(async move {
                                    match create_relay_profile_at(&base, now_ms(), "https://api.openai.com/v1", json!({})) {
                                        Ok(new_profile) => {
                                            let mut next_settings = base.clone();
                                            if let Some(obj) = next_settings.as_object_mut() {
                                                let arr = obj
                                                    .entry("relayProfiles")
                                                    .or_insert_with(|| Value::Array(Vec::new()));
                                                if let Some(list) = arr.as_array_mut() {
                                                    list.push(new_profile.clone());
                                                }
                                                if let Some(id) = new_profile.get("id").and_then(Value::as_str) {
                                                    obj.insert("activeRelayId".into(), Value::String(id.to_string()));
                                                }
                                            }
                                            match backend::save_settings_async(next_settings).await {
                                                Ok(_) => status_msg.set("Đã tạo & lưu Relay Profile mới".to_string()),
                                                Err(e) => status_msg.set(format!("Lỗi lưu cài đặt: {e:#}")),
                                            }
                                        }
                                        Err(e) => status_msg.set(format!("Lỗi tạo profile: {e}")),
                                    }
                                    let next = *refresh_tick.read() + 1;
                                    refresh_tick.set(next);
                                });
                            },
                            "{tr_text(\"+ Thêm Provider\", \"+ New Provider\", en)}"
                        }
                        if !active_id.is_empty() {
                            button {
                                class: "btn",
                                onclick: move |_| {
                                    let base = settings_for_dup.clone();
                                    let target_id = active_id.clone();
                                    spawn(async move {
                                        match duplicate_relay_profile_at(&base, &target_id, now_ms()) {
                                            Ok(next_settings) => match backend::save_settings_async(next_settings).await {
                                                Ok(_) => status_msg.set("Đã nhân bản provider đang chọn".to_string()),
                                                Err(e) => status_msg.set(format!("Lỗi lưu nhân bản: {e:#}")),
                                            },
                                            Err(e) => status_msg.set(format!("Lỗi nhân bản: {e}")),
                                        }
                                        let next = *refresh_tick.read() + 1;
                                        refresh_tick.set(next);
                                    });
                                },
                                "{tr_text(\"Nhân bản active\", \"Duplicate Active\", en)}"
                            }
                        }
                    }
                }
                div { class: "cx-profile-grid",
                    for p in &profiles {
                        {
                            let pid = p.get("id").and_then(Value::as_str).unwrap_or("").to_string();
                            let pname = p.get("name").and_then(Value::as_str).unwrap_or(&pid).to_string();
                            let avatar = provider_avatar_text_fast(&pname, Some("CP"));
                            let pmode = p.get("relayMode").and_then(Value::as_str).unwrap_or("official").to_string();
                            let purl = p.get("baseUrl").and_then(Value::as_str).unwrap_or("-").to_string();
                            let is_active = p.get("id").and_then(Value::as_str) == snap.settings.get("activeRelayId").and_then(Value::as_str);
                            let mut next_settings = snap.settings.clone();
                            rsx! {
                                div {
                                    class: if is_active { "cx-profile-card cx-active" } else { "cx-profile-card" },
                                    div { class: "cx-profile-title",
                                        span { class: "cx-avatar", "{avatar}" }
                                        "{pname}"
                                    }
                                    div { class: "cx-profile-meta", "{pmode} · {purl}" }
                                    if !is_active {
                                        button {
                                            class: "btn btn-sm",
                                            onclick: move |_| {
                                                if let Some(obj) = next_settings.as_object_mut() {
                                                    obj.insert("activeRelayId".into(), Value::String(pid.clone()));
                                                }
                                                let payload = next_settings.clone();
                                                spawn(async move {
                                                    match backend::save_settings_async(payload).await {
                                                        Ok(_) => status_msg.set("Đã chuyển & áp dụng nhà cung cấp kích hoạt".to_string()),
                                                        Err(e) => status_msg.set(format!("Lỗi chuyển provider: {e:#}")),
                                                    }
                                                    let next = *refresh_tick.read() + 1;
                                                    refresh_tick.set(next);
                                                });
                                            },
                                            "{tr_text(\"Kích hoạt\", \"Activate\", en)}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            div { class: "cx-card",
                h3 { "{tr_text(\"Mẫu Preset nhanh (23 nhà cung cấp từ manager-core-rs)\", \"Quick Presets (23 providers from manager-core-rs)\", en)}" }
                div { class: "cx-preset-chips",
                    for item in preset_items.iter().take(12) {
                        {
                            let name = item.get("name").and_then(Value::as_str).unwrap_or("Preset").to_string();
                            let base_url = item.get("baseUrl").and_then(Value::as_str).unwrap_or("").to_string();
                            let model = item.get("model").and_then(Value::as_str).unwrap_or("").to_string();
                            let base = settings_for_preset.clone();
                            let preset_name = name.clone();
                            rsx! {
                                button {
                                    class: "btn btn-sm",
                                    onclick: move |_| {
                                        let mut b = base.clone();
                                        let bu = base_url.clone();
                                        let mdl = model.clone();
                                        let nm = preset_name.clone();
                                        spawn(async move {
                                            match create_relay_profile_at(&b, now_ms(), &bu, json!({})) {
                                                Ok(mut prof) => {
                                                    if let Some(pobj) = prof.as_object_mut() {
                                                        pobj.insert("name".into(), Value::String(nm.clone()));
                                                        pobj.insert("model".into(), Value::String(mdl));
                                                        pobj.insert("relayMode".into(), Value::String("pureApi".into()));
                                                    }
                                                    let prof = with_generated_relay_files(prof.clone()).unwrap_or(prof);
                                                    if let Some(bobj) = b.as_object_mut() {
                                                        let arr = bobj
                                                            .entry("relayProfiles")
                                                            .or_insert_with(|| Value::Array(Vec::new()));
                                                        if let Some(list) = arr.as_array_mut() {
                                                            list.push(prof.clone());
                                                        }
                                                        if let Some(id) = prof.get("id").and_then(Value::as_str) {
                                                            bobj.insert("activeRelayId".into(), Value::String(id.to_string()));
                                                        }
                                                    }
                                                    match backend::save_settings_async(b).await {
                                                        Ok(_) => status_msg.set(format!("Đã thêm & áp dụng preset {nm}")),
                                                        Err(e) => status_msg.set(format!("Lỗi lưu preset {nm}: {e:#}")),
                                                    }
                                                }
                                                Err(e) => status_msg.set(format!("Lỗi khởi tạo preset {nm}: {e}")),
                                            }
                                            let next = *refresh_tick.read() + 1;
                                            refresh_tick.set(next);
                                        });
                                    },
                                    "{name}"
                                }
                            }
                        }
                    }
                }
            }
            div { class: "cx-card",
                h3 { "{tr_text(\"Xem trước config.toml sinh tự động (Zero-Copy Highlight)\", \"Generated config.toml Preview (Zero-Copy Highlight)\", en)}" }
                {render_highlighted_code(&toml_preview, HighlightLanguage::Toml)}
            }
        }
    }
}

fn render_request_logs(snap: &AppSnapshot, en: bool) -> Element {
    rsx! {
        div { class: "cx-card",
            h2 { "{tr_text(\"Nhật ký Request gần đây\", \"Recent Request Logs\", en)} ({snap.request_logs.len()})" }
            if snap.request_logs.is_empty() {
                p { class: "cx-muted", "{tr_text(\"Chưa có request log nào trong bộ nhớ trạng thái hiện tại.\", \"No request logs recorded in the current state directory.\", en)}" }
            } else {
                table { class: "cx-table",
                    thead {
                        tr {
                            th { "Time (ms)" }
                            th { "Method / Path" }
                            th { "Model" }
                            th { "Provider" }
                            th { "Status" }
                            th { "Duration" }
                        }
                    }
                    tbody {
                        for item in &snap.request_logs {
                            tr {
                                td { "{item.timestamp_ms}" }
                                td { "{item.method} {item.path}" }
                                td { "{item.model}" }
                                td { "{item.relay_name} ({item.wire_api})" }
                                td { "{item.status_code}" }
                                td { "{item.duration_ms} ms" }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn render_gateway_control(
    route: Route,
    snap: &AppSnapshot,
    en: bool,
    status_msg: &mut Signal<String>,
    refresh_tick: &mut Signal<u64>,
) -> Element {
    let mut sm = *status_msg;
    let mut rt = *refresh_tick;
    let backend_body = snap
        .gateway_status_json
        .as_ref()
        .map(|v| serde_json::to_string_pretty(v).unwrap_or_default())
        .unwrap_or_else(|| "{}".to_string());
    let proxy_body = snap
        .proxy_status_json
        .as_ref()
        .map(|v| serde_json::to_string_pretty(v).unwrap_or_default())
        .unwrap_or_else(|| "{}".to_string());

    rsx! {
        div { class: "cx-page-stack",
            div { class: "cx-card",
                h2 { "{route.title(en)}" }
                p { class: "cx-muted",
                    "Port: {snap.helper_port.unwrap_or(snap.configured_proxy_port)} · Codex Home: {snap.codex_home}"
                }
                div { class: "cx-btn-row",
                    button {
                        class: "btn btn-primary",
                        onclick: move |_| {
                            spawn(async move {
                                match backend::ensure_gateway_helper_async().await {
                                    Ok(port) => sm.set(format!("Gateway active tại 127.0.0.1:{port}")),
                                    Err(e) => sm.set(format!("Lỗi: {e:#}")),
                                }
                                let next = *rt.read() + 1;
                                rt.set(next);
                            });
                        },
                        "{tr_text(\"Khởi động / Đồng bộ Gateway\", \"Start / Sync Gateway\", en)}"
                    }
                    button {
                        class: "btn",
                        onclick: move |_| {
                            spawn(async move {
                                backend::stop_gateway_helper_async().await;
                                sm.set("Đã dừng resident helper".to_string());
                                let next = *rt.read() + 1;
                                rt.set(next);
                            });
                        },
                        "{tr_text(\"Dừng Helper\", \"Stop Helper\", en)}"
                    }
                }
                h4 { "/backend/status" }
                pre { class: "cx-code-block", "{backend_body}" }
                h4 { "/proxy-status" }
                pre { class: "cx-code-block", "{proxy_body}" }
            }
        }
    }
}
