//! Standalone Pure-Rust entry point for `codex-plus-app`.
//! Handles MCP stdio subcommands, hooks, `--smoke-check`, and Dioxus Desktop GUI.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![forbid(unsafe_code)]

use codex_plus_app::{backend, security, ui};
use dioxus::desktop::{Config, LogicalSize, WindowBuilder};
use std::io::Write;

fn print_help() {
    let _ = writeln!(
        std::io::stdout(),
        "codex-plus-app {} (Pure-Rust Dioxus Desktop Manager)\n\
         \n\
         FLAGS:\n\
         \t--smoke-check [out.json]  Start resident Gateway helper on loopback, probe HTTP + Codex Desktop\n\
         \t--version                 Print version\n\
         \t--help                    Print this help\n\
         \t--codex-plus-cu-mcp / --codex-plus-browser-mcp / ... compatible CLI hooks",
        codex_plus_core::version::VERSION
    );
}

fn try_handle_cli_submodes() -> Option<i32> {
    let args: Vec<_> = std::env::args_os().collect();
    let first = args.get(1).and_then(|a| a.to_str()).unwrap_or("");
    match first {
        "--help" | "-h" => {
            print_help();
            return Some(0);
        }
        "--version" | "-V" => {
            let _ = writeln!(
                std::io::stdout(),
                "codex-plus-app {}",
                codex_plus_core::version::VERSION
            );
            return Some(0);
        }
        "--smoke-check" => {
            let out_file = args.get(2).map(std::path::PathBuf::from);
            let rt = match tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
            {
                Ok(rt) => rt,
                Err(e) => {
                    let _ = writeln!(std::io::stderr(), "smoke-check tokio init failed: {e:#}");
                    return Some(1);
                }
            };
            let code = rt.block_on(async {
                match backend::run_smoke_check(true).await {
                    Ok(report) => {
                        let ok = report.ok;
                        let body = serde_json::to_string_pretty(&report).unwrap_or_default();
                        if let Some(path) = out_file {
                            let _ = std::fs::write(path, &body);
                        }
                        let _ = writeln!(std::io::stdout(), "{body}");
                        if ok { 0 } else { 1 }
                    }
                    Err(e) => {
                        let _ = writeln!(std::io::stderr(), "smoke-check failed: {e:#}");
                        1
                    }
                }
            });
            return Some(code);
        }
        "--codex-plus-cu-mcp" => {
            return Some(
                if let Err(e) = codex_plus_core::computer_use_proxy::serve() {
                    eprintln!("Computer Use MCP failed: {e:#}");
                    1
                } else {
                    0
                },
            );
        }
        "--codex-plus-browser-mcp" => {
            return Some(if let Err(e) = codex_plus_core::browser_use::serve() {
                eprintln!("Browser Use MCP failed: {e:#}");
                1
            } else {
                0
            });
        }
        "--codex-plus-browser-turn-end" => {
            codex_plus_core::claude_hooks::record_turn_end(
                args.get(2).map(std::ffi::OsString::as_os_str),
            );
            return Some(0);
        }
        "--codex-plus-uninstall-cleanup" => {
            let home = codex_plus_core::codex_home::default_codex_home_dir();
            let _ = codex_plus_core::relay_config::clear_managed_proxy_config_for_uninstall(&home);
            return Some(0);
        }
        flag if flag == codex_plus_core::remote_control::jiajia::MCP_FLAG => {
            return Some(
                if let Err(e) = codex_plus_core::remote_control::jiajia::mcp::serve() {
                    eprintln!("Jiajia MCP failed: {e:#}");
                    1
                } else {
                    0
                },
            );
        }
        flag if flag == codex_plus_core::dsh_plugins::MCP_FLAG => {
            return Some(match codex_plus_core::dsh_plugins::run_mcp(&args[2..]) {
                Ok(code) => code,
                Err(e) => {
                    eprintln!("dsh plugin MCP failed: {e:#}");
                    1
                }
            });
        }
        flag if flag == codex_plus_core::extension_hooks::HOOK_FLAG => {
            let event = args.get(2).and_then(|a| a.to_str()).unwrap_or("");
            let manager = codex_plus_core::user_scripts::default_user_script_manager();
            let rt = match tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
            {
                Ok(rt) => rt,
                Err(e) => {
                    eprintln!("extension hook runtime init failed: {e:#}");
                    return Some(1);
                }
            };
            return Some(rt.block_on(codex_plus_core::extension_hooks::run_cli(&manager, event)));
        }
        flag if flag == codex_plus_core::extension_backend::HOST_CALL_FLAG => {
            let manager = codex_plus_core::user_scripts::default_user_script_manager();
            let rt = match tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
            {
                Ok(rt) => rt,
                Err(e) => {
                    eprintln!("extension host-call runtime init failed: {e:#}");
                    return Some(1);
                }
            };
            return Some(
                rt.block_on(codex_plus_core::extension_backend::run_host_call_cli(
                    &manager,
                )),
            );
        }
        _ => {}
    }
    None
}

fn main() {
    if let Some(code) = try_handle_cli_submodes() {
        std::process::exit(code);
    }

    let guard_port = codex_plus_core::ports::manager_guard_port();
    let Ok(_instance_guard) =
        codex_plus_core::ports::acquire_resilient_loopback_port_guard(guard_port)
    else {
        // Another instance already owns this guard port; exit cleanly instead of racing on WebView2 profile.
        std::process::exit(0);
    };

    let state_dir = codex_plus_core::paths::default_app_state_dir();
    let webview_data_dir = state_dir.join("webview2-dioxus");
    let _ = std::fs::create_dir_all(&webview_data_dir);

    // Ensure Gateway resident helper in background without blocking window appearance (`H-4.2.4`).
    std::thread::spawn(|| {
        let port = codex_plus_core::protocol_proxy::protocol_proxy_port();
        if let Ok(settings) = codex_plus_core::settings::SettingsStore::default().load() {
            codex_plus_core::launcher::set_http_body_limit_bytes(
                settings.helper_max_request_body_bytes(),
            );
        }
        let _ = codex_plus_core::resident_helper::ensure_resident_helper(port);
    });

    let head = format!(
        r#"<meta http-equiv="Content-Security-Policy" content="{}"><style>{}</style><style>{}</style>"#,
        security::STRICT_WEBVIEW_CSP,
        ui::COMBINED_MANAGER_CSS,
        ui::POLISH_CSS
    );

    let window = WindowBuilder::new()
        .with_title("Codex++ (Rust Desktop)")
        .with_inner_size(LogicalSize::new(1180.0, 820.0))
        .with_min_inner_size(LogicalSize::new(960.0, 720.0))
        .with_always_on_top(false);

    let cfg = Config::new()
        .with_menu(None)
        .with_custom_head(head)
        .with_data_directory(webview_data_dir)
        .with_window(window)
        .with_navigation_handler(security::is_safe_webview_navigation);

    dioxus::LaunchBuilder::desktop()
        .with_cfg(cfg)
        .launch(ui::App);

    codex_plus_core::resident_helper::stop_resident_helper();
}
