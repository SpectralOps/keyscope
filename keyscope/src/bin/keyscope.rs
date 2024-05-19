mod cli;

use clap::Parser;
use console::style;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

pub const BANNER: &str = r"
    __                                       
   / /_____  __  ________________  ____  ___ 
  / // / _ \/ / / / ___/ ___/ __ \/ __ \/ _ \
 /   </  __/ /_/ (__  ) /__/ /_/ / /_/ /  __/
/_/|_|\___/\__, /____/\___/\____/ .___/\___/ 
          (____/               /_/";

fn main() {
    let args = cli::Cli::parse();

    tracing(args.verbose);

    if args.no_banner {
        let v = format!(
            "{} ({})",
            env!("CARGO_PKG_VERSION"),
            option_env!("BUILD_SHA")
                .or(option_env!("GITHUB_SHA"))
                .unwrap_or("dev")
        );
        println!(
            "{}\n                    {}",
            style(BANNER).magenta(),
            style(v).dim()
        );
    }

    match cli::run(args) {
        Ok(cmd) => cmd,
        Err(err) => cli::exit::CmdResult::error_with_message(err.to_string().as_str()),
    }
    .exit();
}

pub fn tracing(verbose: bool) {
    let level = if verbose {
        LevelFilter::DEBUG
    } else {
        LevelFilter::OFF
    };
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(level.into())
                .from_env_lossy(),
        )
        .with_line_number(true)
        .with_target(true)
        .init();
}
