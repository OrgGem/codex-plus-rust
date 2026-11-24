//! Unified CLI for `claude-patcher.exe` (`build-copy`, `sign-copy`, `inject-cn`).

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use claude_patcher_rs::{build_copy, inject_cn, sign_copy};
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[command(
    name = "claude-patcher",
    about = "Pure-Rust Claude Desktop localization & injection tool"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Clone, unlock Electron fuses, localize `ion-dist`, optionally patch Computer Use, or `--check`
    BuildCopy {
        #[arg(long)]
        check: bool,
        #[arg(long)]
        source: Option<PathBuf>,
        #[arg(long)]
        copy: Option<PathBuf>,
        #[arg(long)]
        catalogs: Option<PathBuf>,
        #[arg(long)]
        computer_use: bool,
    },
    /// Bottom-up macOS code signing with entitlement stripping & fixed `untrusted_only` check
    SignCopy {
        source: Option<PathBuf>,
        copy: Option<PathBuf>,
    },
    /// Launch or attach via CDP WebSocket, verify `process.execPath`, evaluate `hook.js`, and close inspector
    InjectCn {
        #[arg(long)]
        app: Option<PathBuf>,
        #[arg(long)]
        hook: Option<PathBuf>,
        #[arg(long, default_value_t = 45)]
        timeout: u64,
        #[arg(long)]
        no_launch: bool,
        #[arg(long)]
        attach: bool,
        #[arg(long)]
        port: Option<u16>,
        #[arg(long)]
        user_data_dir: Option<PathBuf>,
        #[arg(long, default_value = "zh-Hans")]
        locale: String,
        #[arg(long)]
        voice_input: Option<String>,
        #[arg(long)]
        localization: Option<String>,
    },
}

fn run_cli(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::BuildCopy {
            check,
            source,
            copy,
            catalogs,
            computer_use,
        } => {
            let src = source
                .or_else(|| std::env::var_os("CODEX_PLUS_CLAUDE_SOURCE").map(PathBuf::from))
                .context("Missing --source or CODEX_PLUS_CLAUDE_SOURCE")?;
            let dst = copy
                .or_else(|| std::env::var_os("CODEX_PLUS_CLAUDE_COPY").map(PathBuf::from))
                .context("Missing --copy or CODEX_PLUS_CLAUDE_COPY")?;
            let cu = computer_use
                || std::env::var("CODEX_PLUS_CLAUDE_COMPUTER_USE").as_deref() == Ok("1");
            if check {
                build_copy::check_copy(&src, &dst, cu)
            } else {
                let raw_cat = catalogs
                    .unwrap_or_else(|| PathBuf::from("assets/claude-localization/catalogs"));
                let cat = inject_cn::clean_absolute_path(&raw_cat)?;
                build_copy::build_copy(&src, &dst, &cat, cu)
            }
        }
        Commands::SignCopy { source, copy } => {
            sign_copy::sign_copy(source.as_deref(), copy.as_deref())
        }
        Commands::InjectCn {
            app,
            hook,
            timeout,
            no_launch,
            attach,
            port,
            user_data_dir,
            locale,
            voice_input,
            localization,
        } => {
            let app_path = app
                .or_else(|| std::env::var_os("CODEX_PLUS_CLAUDE_COPY").map(PathBuf::from))
                .context("Missing --app or CODEX_PLUS_CLAUDE_COPY")?;
            let raw_hook =
                hook.unwrap_or_else(|| PathBuf::from("assets/claude-localization/hook.js"));
            let hook_path = inject_cn::clean_absolute_path(&raw_hook)?;
            let voice = voice_input
                .or_else(|| std::env::var("CODEX_PLUS_CLAUDE_VOICE_INPUT").ok())
                .unwrap_or_else(|| "1".to_string())
                != "0";
            let loc = localization
                .or_else(|| std::env::var("CODEX_PLUS_CLAUDE_LOCALIZATION").ok())
                .unwrap_or_else(|| "1".to_string())
                != "0";
            let cfg = inject_cn::InjectConfig {
                app: app_path,
                hook: hook_path,
                timeout_secs: timeout,
                no_launch,
                attach,
                port,
                user_data_dir,
                locale,
                voice_input: voice,
                localization: loc,
                bash_timeout_json: std::env::var("CODEX_PLUS_CLAUDE_BASH_TIMEOUT")
                    .ok()
                    .filter(|s| !s.trim().is_empty())
                    .unwrap_or_else(|| "null".to_string()),
                bash_report_file: std::env::var("CODEX_PLUS_CLAUDE_BASH_REPORT_FILE").ok(),
                bash_launch_id: std::env::var("CODEX_PLUS_CLAUDE_BASH_LAUNCH_ID").ok(),
            };
            inject_cn::run_inject(&cfg)
        }
    }
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match run_cli(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e:#}");
            ExitCode::from(1)
        }
    }
}
