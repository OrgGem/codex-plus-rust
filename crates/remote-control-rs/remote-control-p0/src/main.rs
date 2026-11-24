//! Unified CLI entrypoint for `remote-control-p0` tools and verification runners.

use anyhow::Result;
use clap::{Parser, Subcommand};
use remote_control_p0::asar::{extract_ipc_versions, read_asar_file};
use remote_control_p0::mock_responses::serve_mock_responses;
use remote_control_p0::runners::{run_http_suite_smoke, run_p0, run_poc_dump, run_poc_turn};
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[command(
    name = "remote-control-p0",
    about = "Codex++ remote-control P0..P5 tools & mock server (Rust port)"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Extract IPC method version table from Codex app.asar
    IpcVersions { asar: PathBuf },
    /// Start fake OpenAI Responses API HTTP/SSE server
    MockResponses {
        #[arg(default_value_t = 18765)]
        port: u16,
        #[arg(default_value = ".")]
        work_dir: String,
        #[arg(long)]
        mock_log: Option<PathBuf>,
    },
    /// Headless dump PoC (`poc-headless-dump.mjs`)
    PocDump {
        #[arg(env = "P0_CODEX")]
        codex: String,
        #[arg(env = "P0_HOME")]
        home: String,
        #[arg(default_value = ".")]
        cwd: String,
        #[arg(long, default_value = "headless-notifications.jsonl")]
        out: PathBuf,
    },
    /// Headless single-turn PoC (`poc-headless-turn.mjs`)
    PocTurn {
        #[arg(env = "P0_CODEX")]
        codex: String,
        #[arg(env = "P0_HOME")]
        home: String,
        #[arg(default_value = ".")]
        cwd: String,
    },
    /// Run P0 IPC regression suite
    RunP0 {
        #[arg(env = "P0_HOME")]
        home: String,
        #[arg(env = "P0_WORK")]
        work: String,
        #[arg(env = "P0_CODEX")]
        codex: String,
        #[arg(long, env = "P0_CDP_PORT", default_value_t = 9333)]
        cdp_port: u16,
    },
    /// Run HTTP remote-control regression checks (p1..p5)
    RunSuite {
        suite: String,
        #[arg(long, env = "P1_PORT", default_value_t = 57399)]
        port: u16,
        #[arg(long, env = "P1_TOKEN")]
        token: String,
    },
}

fn run_cli(cli: Cli) -> Result<bool> {
    match cli.command {
        Commands::IpcVersions { asar } => {
            let src = read_asar_file(&asar, ".vite/build/early-bootstrap.js")?;
            let versions = extract_ipc_versions(&src)?;
            println!("{}", serde_json::to_string_pretty(&versions)?);
            Ok(true)
        }
        Commands::MockResponses {
            port,
            work_dir,
            mock_log,
        } => {
            serve_mock_responses(port, work_dir, mock_log, None)?;
            Ok(true)
        }
        Commands::PocDump {
            codex,
            home,
            cwd,
            out,
        } => {
            run_poc_dump(&codex, &home, &cwd, &out)?;
            Ok(true)
        }
        Commands::PocTurn { codex, home, cwd } => {
            let tid = run_poc_turn(&codex, &home, &cwd)?;
            println!("threadId={tid}");
            Ok(true)
        }
        Commands::RunP0 {
            home,
            work,
            codex,
            cdp_port,
        } => {
            let mut all_ok = true;
            for r in run_p0(&home, &work, &codex, cdp_port)? {
                if !r.ok {
                    all_ok = false;
                }
                println!("{} {} {}", if r.ok { "✓" } else { "✗" }, r.name, r.detail);
            }
            Ok(all_ok)
        }
        Commands::RunSuite { suite, port, token } => {
            let mut all_ok = true;
            for r in run_http_suite_smoke(&suite, &format!("http://127.0.0.1:{port}"), &token)? {
                if !r.ok {
                    all_ok = false;
                }
                println!("{} {} {}", if r.ok { "✓" } else { "✗" }, r.name, r.detail);
            }
            Ok(all_ok)
        }
    }
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match run_cli(cli) {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::from(1),
        Err(e) => {
            eprintln!("{e:#}");
            ExitCode::from(1)
        }
    }
}
