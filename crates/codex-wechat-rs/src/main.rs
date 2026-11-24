//! CLI entrypoint for `codex-wechat` (`run` and `login` subcommands).

use anyhow::Result;
use clap::{Parser, Subcommand};
use codex_wechat_rs::bridge::{poll_loop_with, qr_login};
use codex_wechat_rs::config::load_config;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "codex-wechat", about = "Codex WeChat Claw PoC (Rust port)")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 启动 iLink -> codex app-server/exec -> iLink 转发
    Run {
        #[arg(long)]
        config: PathBuf,
    },
    /// 扫码获取 iLink token 并写入配置
    Login {
        #[arg(long)]
        output: PathBuf,
        #[arg(long, default_value = "3")]
        bot_type: String,
        #[arg(long, default_value_t = 480)]
        timeout: u64,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Run { config } => {
            let cfg = load_config(&config)?;
            poll_loop_with(&cfg, None)
        }
        Commands::Login {
            output,
            bot_type,
            timeout,
        } => qr_login(&output, &bot_type, timeout),
    }
}
