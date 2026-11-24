//! Unified CLI entrypoint for `codex-plus-devkit`.

use clap::{Parser, Subcommand};
use codex_plus_devkit::{
    browser_inspector, local_builds, plugin_market, relaunch, release, release_meta,
    renderer_inject, sherpa_cache, sync_models, whale_runtime,
};
use std::io;
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[command(
    name = "codex-plus-devkit",
    about = "Pure-Rust devkit CLI for CodexPlusPlus"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Validate `inspect-service` contract over stdin JSON (`--stdin`)
    InspectBrowserService {
        #[arg(long)]
        stdin: bool,
    },
    /// Release version consistency & CHANGELOG section helpers
    ReleaseMeta {
        #[command(subcommand)]
        action: ReleaseMetaCmd,
    },
    /// Automated version bump, commit, tag, and atomic push
    Release {
        version: String,
        #[arg(long)]
        dry_run: bool,
        #[arg(long, default_value = ".")]
        root: PathBuf,
    },
    /// Validate or quarantine prebuilt sherpa-onnx static libraries
    PrepareSherpaCache {
        #[arg(long)]
        target_dir: Option<PathBuf>,
        #[arg(long)]
        temp_dir: Option<PathBuf>,
    },
    /// Safely move old `dist/local-build*` directories to Trash
    PruneLocalBuilds {
        #[arg(long)]
        apply: bool,
        #[arg(long, default_value_t = 0)]
        keep: usize,
        #[arg(long, default_value_t = 30)]
        min_age_minutes: u64,
        #[arg(long, default_value = ".")]
        root: PathBuf,
    },
    /// Build, replace, and relaunch the local Codex++ instance (macOS & Windows)
    RelaunchLocal {
        #[arg(long)]
        skip_build: bool,
        #[arg(long)]
        no_open: bool,
        #[arg(long)]
        dry_run: bool,
        /// Build & launch Pure-Rust Dioxus Desktop `codex-plus-app` (zero Node/npm/Vite dependency)
        #[arg(long)]
        pure_rust: bool,
        #[arg(long, default_value = ".")]
        root: PathBuf,
    },
    /// Assemble `assets/inject/renderer-inject/` shards into `renderer-inject.js`
    AssembleRendererInject {
        #[arg(long)]
        check: bool,
        #[arg(long, default_value = ".")]
        root: PathBuf,
    },
    /// Split `assets/inject/renderer-inject.js` into manifest-aligned shard files
    SplitRendererInject {
        #[arg(long, default_value = ".")]
        root: PathBuf,
    },
    /// Assemble or verify `full-widget-codex.js` from upstream whale widget
    AssembleWhaleRuntime {
        #[arg(long)]
        check: bool,
        #[arg(long, default_value = ".")]
        root: PathBuf,
    },
    /// Build compact JSON index for on-demand plugin market
    BuildPluginMarketIndex {
        cache_repo: PathBuf,
        mode: String,
        output: PathBuf,
    },
    /// Sync visible upstream official models into `assets/codex-models.json`
    SyncOfficialModels {
        #[arg(long)]
        check: bool,
        #[arg(long, default_value = sync_models::DEFAULT_SOURCE_URL)]
        source: String,
        #[arg(long)]
        asset: Option<PathBuf>,
        #[arg(long, default_value = ".")]
        root: PathBuf,
    },
}

#[derive(Subcommand)]
enum ReleaseMetaCmd {
    Check {
        version: String,
        #[arg(long, default_value = ".")]
        root: PathBuf,
    },
    Notes {
        version: String,
        #[arg(long, default_value = ".")]
        root: PathBuf,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match cli.command {
        Commands::InspectBrowserService { stdin } => {
            if !stdin {
                eprintln!("Usage: codex-plus-devkit inspect-browser-service --stdin");
                return ExitCode::from(2);
            }
            let res = std::thread::Builder::new()
                .name("inspect-browser-service".into())
                .stack_size(16 * 1024 * 1024)
                .spawn(|| browser_inspector::run_stdin(io::stdin().lock()))
                .and_then(|h| {
                    h.join()
                        .map_err(|_| io::Error::other("inspector thread panicked"))
                });
            match res {
                Ok(Ok(out)) => {
                    println!("{out}");
                    ExitCode::SUCCESS
                }
                _ => {
                    eprintln!("{}", browser_inspector::FAILURE_MESSAGE);
                    ExitCode::from(2)
                }
            }
        }
        Commands::ReleaseMeta { action } => match action {
            ReleaseMetaCmd::Check { version, root } => {
                match release_meta::run_check(&root, &version) {
                    Ok(code) => ExitCode::from(code as u8),
                    Err(e) => {
                        eprintln!("{e:#}");
                        ExitCode::from(1)
                    }
                }
            }
            ReleaseMetaCmd::Notes { version, root } => {
                match release_meta::run_notes(&root, &version) {
                    Ok(code) => ExitCode::from(code as u8),
                    Err(e) => {
                        eprintln!("{e:#}");
                        ExitCode::from(1)
                    }
                }
            }
        },
        Commands::Release {
            version,
            dry_run,
            root,
        } => match release::run_release(&root, &version, dry_run) {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("\n✗ {e:#}");
                ExitCode::from(1)
            }
        },
        Commands::PrepareSherpaCache {
            target_dir,
            temp_dir,
        } => {
            let target = target_dir
                .or_else(|| std::env::var_os("CARGO_TARGET_DIR").map(PathBuf::from))
                .unwrap_or_else(|| PathBuf::from("target"));
            let temp = temp_dir
                .or_else(|| std::env::var_os("RUNNER_TEMP").map(PathBuf::from))
                .unwrap_or_else(std::env::temp_dir);
            match sherpa_cache::prepare_sherpa_cache(&target, &temp) {
                Ok(_) => ExitCode::SUCCESS,
                Err(e) => {
                    eprintln!("{e:#}");
                    ExitCode::from(1)
                }
            }
        }
        Commands::PruneLocalBuilds {
            apply,
            keep,
            min_age_minutes,
            root,
        } => {
            let opts = local_builds::PruneOptions {
                apply,
                keep,
                min_age_minutes,
            };
            match local_builds::prune_local_builds(&root, &opts) {
                Ok(_) => ExitCode::SUCCESS,
                Err(e) => {
                    eprintln!("{e:#}");
                    ExitCode::from(1)
                }
            }
        }
        Commands::RelaunchLocal {
            skip_build,
            no_open,
            dry_run,
            pure_rust,
            root,
        } => {
            let opts = relaunch::RelaunchOptions {
                skip_build,
                no_open,
                dry_run,
                pure_rust,
            };
            match relaunch::relaunch_local(&root, &opts) {
                Ok(()) => ExitCode::SUCCESS,
                Err(e) => {
                    eprintln!("{e:#}");
                    ExitCode::from(1)
                }
            }
        }
        Commands::AssembleRendererInject { check, root } => {
            match renderer_inject::assemble_renderer_inject(&root, check) {
                Ok(_) => ExitCode::SUCCESS,
                Err(e) => {
                    eprintln!("{e:#}");
                    ExitCode::from(1)
                }
            }
        }
        Commands::SplitRendererInject { root } => {
            match renderer_inject::split_renderer_inject(&root) {
                Ok(_) => ExitCode::SUCCESS,
                Err(e) => {
                    eprintln!("{e:#}");
                    ExitCode::from(1)
                }
            }
        }
        Commands::AssembleWhaleRuntime { check, root } => {
            match whale_runtime::assemble_whale_runtime(&root, check) {
                Ok(_) => ExitCode::SUCCESS,
                Err(e) => {
                    eprintln!("{e:#}");
                    ExitCode::from(1)
                }
            }
        }
        Commands::BuildPluginMarketIndex {
            cache_repo,
            mode,
            output,
        } => match plugin_market::build_plugin_market_index(&cache_repo, &mode, &output) {
            Ok(_) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("{e:#}");
                ExitCode::from(1)
            }
        },
        Commands::SyncOfficialModels {
            check,
            source,
            asset,
            root,
        } => {
            let asset_path = asset.unwrap_or_else(|| root.join("assets/codex-models.json"));
            match sync_models::sync_official_models(&asset_path, &source, check) {
                Ok(true) => ExitCode::SUCCESS,
                Ok(false) => ExitCode::from(1),
                Err(e) => {
                    eprintln!("{e:#}");
                    ExitCode::from(1)
                }
            }
        }
    }
}
