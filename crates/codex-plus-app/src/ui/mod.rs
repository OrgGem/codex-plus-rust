//! Root Dioxus 0.7 Desktop UI component reusing the bundled original 27 CSS stylesheets.

#![forbid(unsafe_code)]

pub mod pages_gateway;
pub mod pages_management;

use crate::backend;
use crate::ui::pages_gateway::render_gateway_routes;
use crate::ui::pages_management::render_management_routes;
use dioxus::prelude::*;
use manager_core_rs::lifted::{HighlightLanguage, ResolvedTheme, tokenize_code_borrowed};
use manager_core_rs::static_units::{EscapeLayerStack, translate_en};

pub const COMBINED_MANAGER_CSS: &str =
    include_str!(concat!(env!("OUT_DIR"), "/combined-manager.css"));

pub const POLISH_CSS: &str = r#"
:root { --cx-bg: #111318; --cx-panel: #181b22; --cx-border: rgba(255,255,255,0.09); --cx-text: #e6e9ef; --cx-muted: #9aa4b2; --cx-accent: #4c8bf5; }
[data-theme="light"] { --cx-bg: #f6f7fb; --cx-panel: #ffffff; --cx-border: rgba(15,23,42,0.10); --cx-text: #111827; --cx-muted: #4b5563; --cx-accent: #2563eb; }
body { margin: 0; font-family: "Segoe UI", -apple-system, BlinkMacSystemFont, sans-serif; background: var(--cx-bg); color: var(--cx-text); }
.cx-shell { display: flex; min-height: 100vh; background: var(--cx-bg); color: var(--cx-text); }
.cx-sidebar { width: 236px; border-right: 1px solid var(--cx-border); padding: 14px 12px; display: flex; flex-direction: column; gap: 10px; background: var(--cx-panel); flex-shrink: 0; }
.cx-main { flex: 1; display: flex; flex-direction: column; min-width: 0; }
.cx-topbar { display: flex; align-items: center; justify-content: space-between; padding: 10px 18px; border-bottom: 1px solid var(--cx-border); background: var(--cx-panel); gap: 8px; }
.cx-body { padding: 18px; overflow-y: auto; flex: 1; }
.cx-nav-group-label { font-size: 11px; text-transform: uppercase; letter-spacing: .06em; color: var(--cx-muted); margin-top: 8px; padding: 0 8px; }
.cx-nav-btn { width: 100%; text-align: left; border: 0; background: transparent; color: var(--cx-text); padding: 7px 10px; border-radius: 8px; cursor: pointer; font-size: 13px; }
.cx-nav-btn:hover { background: rgba(128,128,128,0.12); }
.cx-nav-btn.cx-active { background: var(--cx-accent); color: #fff; font-weight: 600; }
.cx-page-stack { display: flex; flex-direction: column; gap: 14px; }
.cx-card { background: var(--cx-panel); border: 1px solid var(--cx-border); border-radius: 12px; padding: 16px; }
.cx-card-header { display: flex; align-items: center; justify-content: space-between; gap: 12px; margin-bottom: 12px; }
.cx-card-header h2 { margin: 0; font-size: 16px; }
.cx-metrics-grid, .cx-profile-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(220px, 1fr)); gap: 10px; margin-bottom: 12px; }
.cx-stat-box, .cx-profile-card { border: 1px solid var(--cx-border); border-radius: 10px; padding: 10px 12px; }
.cx-profile-card.cx-active { border-color: var(--cx-accent); }
.cx-stat-label, .cx-muted, .cx-profile-meta { font-size: 12px; color: var(--cx-muted); word-break: break-all; }
.cx-stat-value, .cx-profile-title { font-size: 14px; font-weight: 600; margin-top: 4px; }
.cx-btn-row, .cx-preset-chips { display: flex; flex-wrap: wrap; gap: 8px; align-items: center; }
.btn { border: 1px solid var(--cx-border); background: transparent; color: var(--cx-text); border-radius: 8px; padding: 7px 12px; cursor: pointer; font-size: 13px; }
.btn-primary { background: var(--cx-accent); color: #fff; border-color: transparent; }
.btn-sm { padding: 4px 9px; font-size: 12px; }
.cx-code-block { background: rgba(0,0,0,0.24); border: 1px solid var(--cx-border); border-radius: 8px; padding: 10px; font-size: 12px; overflow-x: auto; max-height: 260px; white-space: pre-wrap; word-break: break-all; }
.cx-badge, .cx-badge-warn { font-size: 11px; padding: 2px 8px; border-radius: 999px; border: 1px solid var(--cx-border); display: inline-block; }
.cx-badge-warn { border-color: #f59e0b; color: #fbbf24; margin-top: 6px; }
.cx-table { width: 100%; border-collapse: collapse; font-size: 12px; }
.cx-table th, .cx-table td { border-bottom: 1px solid var(--cx-border); padding: 7px 8px; text-align: left; }
.cx-toggle-list { display: flex; flex-direction: column; gap: 8px; }
.cx-toggle-row { display: flex; align-items: center; gap: 8px; font-size: 13px; cursor: pointer; }
.cx-palette-backdrop { position: fixed; inset: 0; background: rgba(0,0,0,0.55); display: flex; align-items: flex-start; justify-content: center; padding-top: 80px; z-index: 50; }
.cx-palette-box { width: 480px; max-width: 92vw; background: var(--cx-panel); border: 1px solid var(--cx-border); border-radius: 12px; padding: 14px; }
.cx-palette-input { width: 100%; padding: 8px 10px; border-radius: 8px; border: 1px solid var(--cx-border); background: transparent; color: var(--cx-text); margin-bottom: 10px; }
.cx-toast { margin-bottom: 12px; padding: 8px 12px; border-radius: 8px; border: 1px solid var(--cx-accent); font-size: 12px; }
.cx-avatar { display: inline-flex; align-items: center; justify-content: center; width: 24px; height: 24px; border-radius: 6px; background: rgba(76,139,245,0.18); color: var(--cx-accent); font-size: 11px; font-weight: 700; margin-right: 6px; }
.cx-tok-comment { color: #6b7280; font-style: italic; }
.cx-tok-section { color: #c084fc; font-weight: 600; }
.cx-tok-key { color: #60a5fa; }
.cx-tok-string { color: #34d399; }
.cx-tok-number, .cx-tok-date { color: #fbbf24; }
.cx-tok-bool { color: #f472b6; font-weight: 600; }
.cx-tok-op, .cx-tok-punct { color: var(--cx-muted); }
"#;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Route {
    Home,
    Overview,
    Relay,
    MultiLaunch,
    Claude,
    ClaudePatch,
    RelayEnvironment,
    RequestLog,
    Sessions,
    Context,
    Skills,
    Weixin,
    Enhance,
    DreamSkin,
    UserScripts,
    PluginMarket,
    Recommendations,
    AgentCache,
    AgentCollaboration,
    ComputerUse,
    RemoteControl,
    Hub,
    GatewayProviders,
    Gateway,
    GatewayRouting,
    GatewayUsage,
    GatewaySettings,
    Maintenance,
    About,
    Settings,
    AgentSettings,
    Extension,
}

impl Route {
    pub const ALL: &'static [Route] = &[
        Route::Home,
        Route::Overview,
        Route::Relay,
        Route::MultiLaunch,
        Route::Claude,
        Route::ClaudePatch,
        Route::RelayEnvironment,
        Route::RequestLog,
        Route::Sessions,
        Route::Context,
        Route::Skills,
        Route::Weixin,
        Route::Enhance,
        Route::DreamSkin,
        Route::UserScripts,
        Route::PluginMarket,
        Route::Recommendations,
        Route::AgentCache,
        Route::AgentCollaboration,
        Route::ComputerUse,
        Route::RemoteControl,
        Route::Hub,
        Route::GatewayProviders,
        Route::Gateway,
        Route::GatewayRouting,
        Route::GatewayUsage,
        Route::GatewaySettings,
        Route::Maintenance,
        Route::About,
        Route::Settings,
        Route::AgentSettings,
        Route::Extension,
    ];

    pub fn title(self, en: bool) -> String {
        let (vi, zh) = match self {
            Route::Home => ("Trang chủ", "首页"),
            Route::Overview => ("Tổng quan", "概览"),
            Route::Relay => ("Kết nối & Nhà cung cấp", "连接与供应商"),
            Route::MultiLaunch => ("Chạy đa phiên bản", "多开"),
            Route::Claude => ("Tăng cường Desktop", "Desktop 增强"),
            Route::ClaudePatch => ("Bản vá Desktop", "补丁 Desktop"),
            Route::RelayEnvironment => ("Cấu hình Proxy cũ", "旧版代理设置"),
            Route::RequestLog => ("Nhật ký Request", "请求日志"),
            Route::Sessions => ("Phiên cục bộ", "会话"),
            Route::Context => ("MCP", "MCP"),
            Route::Skills => ("Skills", "Skills"),
            Route::Weixin => ("Kết nối WeChat", "微信连接"),
            Route::Enhance => ("Tiện ích tích hợp", "内置拓展"),
            Route::DreamSkin => ("Quản lý giao diện", "皮肤管理"),
            Route::UserScripts => ("Extensions & Scripts", "万物皆拓展"),
            Route::PluginMarket => ("Chợ Plugin", "插件市场"),
            Route::Recommendations => ("Đề xuất", "推荐内容"),
            Route::AgentCache => ("Dọn Cache Agent", "AI Agent 缓存清理"),
            Route::AgentCollaboration => ("Cộng tác Agent", "Agent 协作"),
            Route::ComputerUse => ("Computer Use", "Computer Use"),
            Route::RemoteControl => ("Điều khiển từ xa", "远程控制"),
            Route::Hub => ("Hub Gateway", "总览"),
            Route::GatewayProviders => ("Bể Nhà cung cấp", "供应商池"),
            Route::Gateway => ("Kết nối Agent", "Agent 接入"),
            Route::GatewayRouting => ("Định tuyến", "路由"),
            Route::GatewayUsage => ("Thống kê dung lượng", "用量"),
            Route::GatewaySettings => ("Cài đặt Gateway", "网关设置"),
            Route::Maintenance => ("Bảo trì & Cài đặt", "安装维护"),
            Route::About => ("Giới thiệu", "关于"),
            Route::Settings => ("Cài đặt tổng thể", "整体设置"),
            Route::AgentSettings => ("Cài đặt Agent", "设置"),
            Route::Extension => ("Trang mở rộng", "拓展页面"),
        };
        if en {
            let translated = translate_en(zh);
            if translated == zh {
                vi.to_string()
            } else {
                translated
            }
        } else {
            vi.to_string()
        }
    }
}

pub fn tr_text(vi: &str, en_text: &str, en: bool) -> String {
    if en {
        en_text.to_string()
    } else {
        vi.to_string()
    }
}

pub fn render_highlighted_code(code: &str, lang: HighlightLanguage) -> Element {
    let lines = tokenize_code_borrowed(code, lang);
    rsx! {
        pre { class: "cx-code-block",
            for (line_idx, line) in lines.iter().enumerate() {
                if line_idx > 0 {
                    "\n"
                }
                for span in line {
                    span { class: "{span.kind.css_class()}", "{span.text}" }
                }
            }
        }
    }
}

#[component]
pub fn App() -> Element {
    let mut active_route = use_signal(|| Route::Overview);
    let mut en_lang = use_signal(|| false);
    let mut theme_mode = use_signal(|| ResolvedTheme::Dark);
    let mut palette_open = use_signal(|| false);
    let mut palette_query = use_signal(String::new);
    let status_msg = use_signal(String::new);
    let mut refresh_tick = use_signal(|| 0u64);
    let mut escape_stack = use_signal(EscapeLayerStack::default);

    let snapshot = use_resource(move || {
        let _tick = *refresh_tick.read();
        async move { backend::load_snapshot_with_gateway_probe().await }
    });

    // Trigger one automatic post-startup refresh once background resident helper finishes binding.
    use_future(move || async move {
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        if codex_plus_core::resident_helper::resident_helper_port().is_some() {
            let next = *refresh_tick.read() + 1;
            refresh_tick.set(next);
        }
    });

    let nav_groups: &[(&str, &str, &[Route])] = &[
        (
            "Chính",
            "Main",
            &[
                Route::Home,
                Route::Overview,
                Route::Hub,
                Route::GatewayProviders,
            ],
        ),
        (
            "Nhà cung cấp & Gateway",
            "Providers & Gateway",
            &[
                Route::Relay,
                Route::Gateway,
                Route::GatewayRouting,
                Route::GatewaySettings,
                Route::MultiLaunch,
            ],
        ),
        (
            "Dữ liệu & Lịch sử",
            "Data & History",
            &[
                Route::GatewayUsage,
                Route::RequestLog,
                Route::Sessions,
                Route::Context,
                Route::Skills,
                Route::UserScripts,
            ],
        ),
        (
            "Tiện ích & Hệ thống",
            "Features & System",
            &[
                Route::Enhance,
                Route::DreamSkin,
                Route::PluginMarket,
                Route::RemoteControl,
                Route::ComputerUse,
                Route::AgentCache,
                Route::Maintenance,
                Route::Settings,
                Route::About,
            ],
        ),
    ];

    let en = *en_lang.read();
    let theme_attr = theme_mode.read().as_str();
    let msg_text = status_msg.read().clone();
    let q_lower = palette_query.read().trim().to_lowercase();

    rsx! {
        div {
            class: "cx-shell",
            "data-theme": "{theme_attr}",
            tabindex: 0,
            onkeydown: move |ev: KeyboardEvent| {
                if ev.modifiers().ctrl() && ev.key().to_string().eq_ignore_ascii_case("k") {
                    let next = !*palette_open.read();
                    palette_open.set(next);
                    if next {
                        escape_stack.write().push("palette");
                    } else {
                        escape_stack.write().reset();
                    }
                } else if ev.key() == Key::Escape {
                    let top = escape_stack.write().handle_escape_key("Escape", false);
                    if top.as_deref() == Some("palette") || *palette_open.read() {
                        palette_open.set(false);
                        escape_stack.write().reset();
                    }
                }
            },
            aside { class: "cx-sidebar",
                div { class: "cx-card-header",
                    strong { "Codex++" }
                    span { class: "cx-badge", "Rust UI" }
                }
                for (vi_g, en_g, items) in nav_groups {
                    div { class: "cx-nav-group-label", "{tr_text(vi_g, en_g, en)}" }
                    for r in *items {
                        {
                            let r_val = *r;
                            let is_sel = *active_route.read() == r_val;
                            rsx! {
                                button {
                                    class: if is_sel { "cx-nav-btn cx-active" } else { "cx-nav-btn" },
                                    onclick: move |_| active_route.set(r_val),
                                    "{r_val.title(en)}"
                                }
                            }
                        }
                    }
                }
            }
            main { class: "cx-main",
                header { class: "cx-topbar",
                    strong { "{active_route.read().title(en)}" }
                    div { class: "cx-btn-row",
                        button {
                            class: "btn btn-sm",
                            onclick: move |_| {
                                let next = !*palette_open.read();
                                palette_open.set(next);
                                if next {
                                    escape_stack.write().push("palette");
                                } else {
                                    escape_stack.write().reset();
                                }
                            },
                            "Ctrl+K"
                        }
                        button {
                            class: "btn btn-sm",
                            onclick: move |_| {
                                let cur = *en_lang.read();
                                en_lang.set(!cur);
                            },
                            if en { "EN" } else { "VI / 中" }
                        }
                        button {
                            class: "btn btn-sm",
                            onclick: move |_| {
                                let cur = *theme_mode.read();
                                theme_mode.set(cur.toggled());
                            },
                            if theme_mode.read().is_dark() { "Dark" } else { "Light" }
                        }
                        button {
                            class: "btn btn-sm",
                            onclick: move |_| {
                                let next = *refresh_tick.read() + 1;
                                refresh_tick.set(next);
                            },
                            "{tr_text(\"Làm mới\", \"Refresh\", en)}"
                        }
                    }
                }
                div { class: "cx-body",
                    if !msg_text.is_empty() {
                        div { class: "cx-toast", "{msg_text}" }
                    }
                    match &*snapshot.read() {
                        Some(Ok(snap)) => {
                            let route = *active_route.read();
                            match route {
                                Route::Home
                                | Route::Overview
                                | Route::Recommendations
                                | Route::Hub
                                | Route::GatewayProviders
                                | Route::Relay
                                | Route::RelayEnvironment
                                | Route::RequestLog
                                | Route::GatewayUsage
                                | Route::Gateway
                                | Route::GatewayRouting
                                | Route::GatewaySettings
                                | Route::MultiLaunch => render_gateway_routes(route, snap, en, status_msg, refresh_tick),
                                _ => render_management_routes(route, snap, en, status_msg, refresh_tick),
                            }
                        }
                        Some(Err(err)) => rsx! {
                            div { class: "cx-card",
                                h3 { "Backend Error" }
                                pre { class: "cx-code-block", "{err:#}" }
                            }
                        },
                        None => rsx! {
                            div { class: "cx-card", "{tr_text(\"Đang tải dữ liệu trạng thái từ lõi Rust...\", \"Loading state snapshot from Rust core...\", en)}" }
                        },
                    }
                }
            }
            if *palette_open.read() {
                div {
                    class: "cx-palette-backdrop",
                    onclick: move |_| {
                        palette_open.set(false);
                        escape_stack.write().reset();
                    },
                    div {
                        class: "cx-palette-box",
                        onclick: move |ev| ev.stop_propagation(),
                        input {
                            class: "cx-palette-input",
                            autofocus: true,
                            placeholder: "{tr_text(\"Tìm trang nhanh...\", \"Jump to route...\", en)}",
                            value: "{palette_query}",
                            oninput: move |ev| palette_query.set(ev.value()),
                        }
                        div { class: "cx-toggle-list",
                            for r in Route::ALL {
                                {
                                    let r_val = *r;
                                    let lbl = r_val.title(en);
                                    if q_lower.is_empty() || lbl.to_lowercase().contains(&q_lower) {
                                        rsx! {
                                            button {
                                                class: "cx-nav-btn",
                                                onclick: move |_| {
                                                    active_route.set(r_val);
                                                    palette_open.set(false);
                                                    escape_stack.write().reset();
                                                },
                                                "{lbl}"
                                            }
                                        }
                                    } else {
                                        rsx! {}
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
