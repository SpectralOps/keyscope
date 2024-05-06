use std::path::PathBuf;

use clap::{ArgAction, Parser, Subcommand};
use keyscope::{
    config::{Config, Definitions},
    out,
};
use service_policy_kit::runner::RunOptions;

use super::{exit::CmdResult, output};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Don't show the banner
    #[arg(global = true, short, long, action = ArgAction::SetFalse)]
    pub no_banner: bool,

    /// Log level
    #[arg(global = true, short, long, action = ArgAction::SetTrue)]
    pub verbose: bool,

    /// Path to configuration file
    #[arg(global = true, short, long)]
    pub config: Option<PathBuf>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Validate keys
    Validate {
        /// Specify the provider to validate for
        #[arg(index = 1)]
        provider: String,

        /// Dry run with examples given in `EXAMPLE_KEY`
        #[arg(short, long, action = ArgAction::SetTrue)]
        dry_run: bool,

        /// Reporter to use
        #[arg(short, long, value_enum, default_value = "stdout")]
        reporter: out::Reporter,

        /// Flip the meaning of success
        #[arg(short, long, action = ArgAction::SetTrue)]
        flip: bool,

        /// List of parameters
        #[arg(short, long)]
        params: Vec<String>,
    },
    /// Show provider requirements (params)
    Requirements {
        /// Specify the provider to validate for
        #[arg(index = 1)]
        provider: String,
    },
    /// Show provider list
    Providers {},
}

#[allow(clippy::unused_async)]
// TODO:: Remove anyhow
pub fn run(args: Cli) -> anyhow::Result<CmdResult> {
    let config: Config = match args.config {
        Some(config_file) => {
            let def: Definitions = serde_yaml::from_reader(std::fs::File::open(config_file)?)?;
            def.into()
        }
        None => Config::default(),
    };

    match args.command {
        Commands::Validate {
            provider,
            dry_run,
            reporter,
            flip,
            params,
        } => {
            // check if provider exists
            let provider = match config.get_providers(&provider) {
                Ok(p) => p,
                Err(err) => {
                    tracing::debug!(provider = provider, error = ?err, "provider not exists");
                    return Ok(CmdResult::error_with_message(output::provider_not_found()));
                }
            };

            let opts: RunOptions = RunOptions::build(
                if dry_run {
                    Some("example".to_string())
                } else {
                    None
                },
                flip,
                Some(reporter.to_string()),
                args.verbose,
            );

            let res = provider.key_validate_with_opts(&params, &opts)?;

            if res.ok {
                Ok(CmdResult::ok())
            } else {
                Ok(CmdResult::error())
            }
        }
        Commands::Requirements { provider } => {
            let provider = match config.get_providers(&provider) {
                Ok(p) => p,
                Err(err) => {
                    tracing::debug!(provider = provider, error = ?err, "provider not exists");
                    return Ok(CmdResult::error_with_message(output::provider_not_found()));
                }
            };
            let Some(params) = provider.get_validation_request_params()? else {
                return Ok(CmdResult::error_with_message("params not found"));
            };

            Ok(CmdResult::ok_with_message(&output::requirements_results(
                provider, params,
            )))
        }
        Commands::Providers {} => Ok(CmdResult::ok_with_message(&output::supported_providers(
            &config.providers,
        ))),
    }
}
