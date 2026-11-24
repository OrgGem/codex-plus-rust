//! Standalone drop-in replacement for `node assets/native-browser/inspect-service.cjs --stdin`.

use codex_plus_devkit::browser_inspector::{FAILURE_MESSAGE, run_stdin};
use std::io;
use std::process::ExitCode;

fn main() -> ExitCode {
    if !std::env::args_os().any(|a| a == "--stdin") {
        eprintln!("Usage: native-browser-inspector --stdin");
        return ExitCode::from(2);
    }
    let res = std::thread::Builder::new()
        .name("native-browser-inspector".into())
        .stack_size(16 * 1024 * 1024)
        .spawn(|| run_stdin(io::stdin().lock()))
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
            // Never echo service source, parser excerpts or local paths.
            eprintln!("{FAILURE_MESSAGE}");
            ExitCode::from(2)
        }
    }
}
