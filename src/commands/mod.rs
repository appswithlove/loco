pub mod assets;
pub mod auth;
pub mod init;
pub mod locales;
pub mod pull;
pub mod push;
pub mod status;
pub mod tags;
pub mod translations;

use crate::cli::{Cli, Command};
use crate::client::LocoClient;
use crate::config::ResolvedConfig;
use crate::error::LocoError;
use crate::output::Output;
use clap::CommandFactory;
use clap_complete::generate;

/// Dispatch CLI commands to handlers.
pub async fn run(cli: Cli) -> anyhow::Result<()> {
    let output = Output::new(cli.json, cli.quiet);

    match cli.command {
        Command::Init => {
            init::run(&output).await?;
        }

        Command::Completions { shell } => {
            let mut cmd = crate::cli::Cli::command();
            generate(shell, &mut cmd, "loco-cli", &mut std::io::stdout());
        }

        Command::Auth { command } => {
            auth::run(&cli.api_key, &cli.config_path, &output, command).await?;
        }

        Command::Pull(args) => {
            let config = ResolvedConfig::load(cli.api_key.as_deref(), cli.config_path.as_deref())?;
            let client = LocoClient::new(&config.api_key, &config.base_url)?;
            pull::run(&client, &output, &config, args).await?;
        }

        Command::Push(args) => {
            let config = ResolvedConfig::load(cli.api_key.as_deref(), cli.config_path.as_deref())?;
            let client = LocoClient::new(&config.api_key, &config.base_url)?;
            push::run(&client, &output, &config, args).await?;
        }

        Command::Status(args) => {
            let (_, client) = build_client(&cli.api_key, &cli.config_path)?;
            status::run(&client, &output, args).await?;
        }

        Command::Assets { command } => {
            let (_, client) = build_client(&cli.api_key, &cli.config_path)?;
            assets::run(&client, &output, command).await?;
        }

        Command::Locales { command } => {
            let (_, client) = build_client(&cli.api_key, &cli.config_path)?;
            locales::run(&client, &output, command).await?;
        }

        Command::Translations { command } => {
            let (_, client) = build_client(&cli.api_key, &cli.config_path)?;
            translations::run(&client, &output, command).await?;
        }

        Command::Tags { command } => {
            let (_, client) = build_client(&cli.api_key, &cli.config_path)?;
            tags::run(&client, &output, command).await?;
        }
    }

    Ok(())
}

fn build_client(
    api_key: &Option<String>,
    config_path: &Option<String>,
) -> Result<(ResolvedConfig, LocoClient), LocoError> {
    let config = ResolvedConfig::load(api_key.as_deref(), config_path.as_deref())?;
    let client = LocoClient::new(&config.api_key, &config.base_url)?;
    Ok((config, client))
}
