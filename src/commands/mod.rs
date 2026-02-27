pub mod auth;
pub mod init;
pub mod locales;
pub mod pull;
pub mod push;
pub mod status;
pub mod strings;
pub mod tags;

use crate::cli::{Cli, Command};
use crate::client::LocoClient;
use crate::config::ResolvedConfig;
use crate::error::LocoError;
use crate::output::Output;
use clap::CommandFactory;
use clap_complete::generate;
use clap_complete::Shell;

/// Dispatch CLI commands to handlers.
pub async fn run(cli: Cli) -> anyhow::Result<()> {
    let output = Output::new(cli.json, cli.quiet, cli.verbose);

    match cli.command {
        Command::Init => {
            init::run(&output).await?;
        }

        Command::Completions { shell, install } => {
            let mut cmd = crate::cli::Cli::command();
            if install {
                install_completions(&output, shell, &mut cmd)?;
            } else {
                generate(shell, &mut cmd, "loco", &mut std::io::stdout());
            }
        }

        Command::Auth { command } => {
            auth::run(&cli.api_key, &cli.config_path, &output, command).await?;
        }

        Command::Pull(args) => {
            let config = ResolvedConfig::load(cli.api_key.as_deref(), cli.config_path.as_deref())?;
            log_config(&output, &config);
            let client = LocoClient::new(&config.api_key, &config.base_url)?;
            pull::run(&client, &output, &config, args).await?;
        }

        Command::Push(args) => {
            let config = ResolvedConfig::load(cli.api_key.as_deref(), cli.config_path.as_deref())?;
            log_config(&output, &config);
            let client = LocoClient::new(&config.api_key, &config.base_url)?;
            push::run(&client, &output, &config, args).await?;
        }

        Command::Status(args) => {
            let (config, client) = build_client(&cli.api_key, &cli.config_path)?;
            log_config(&output, &config);
            status::run(&client, &output, args).await?;
        }

        Command::Strings { command } => {
            let (config, client) = build_client(&cli.api_key, &cli.config_path)?;
            log_config(&output, &config);
            strings::run(&client, &output, command).await?;
        }

        Command::Locales { command } => {
            let (config, client) = build_client(&cli.api_key, &cli.config_path)?;
            log_config(&output, &config);
            locales::run(&client, &output, command).await?;
        }

        Command::Tags { command } => {
            let (config, client) = build_client(&cli.api_key, &cli.config_path)?;
            log_config(&output, &config);
            tags::run(&client, &output, command).await?;
        }
    }

    Ok(())
}

fn log_config(output: &Output, config: &ResolvedConfig) {
    match &config.config_path {
        Some(p) => output.verbose(&format!("Config: {}", p.display())),
        None => output.verbose("Config: none found, using defaults"),
    }
}

fn install_completions(
    output: &Output,
    shell: Shell,
    cmd: &mut clap::Command,
) -> anyhow::Result<()> {
    let home = std::env::var("HOME").unwrap_or_else(|_| "~".to_string());

    let dest = match shell {
        Shell::Bash => format!("{home}/.bash_completion.d/loco"),
        Shell::Zsh => format!("{home}/.zfunc/_loco"),
        Shell::Fish => format!("{home}/.config/fish/completions/loco.fish"),
        Shell::PowerShell => {
            output.warn("PowerShell has no standard completions directory.");
            output.info("Pipe output manually: loco completions powershell > _loco.ps1");
            return Ok(());
        }
        _ => {
            output.warn(&format!(
                "No known install path for {shell:?}. Printing to stdout instead."
            ));
            generate(shell, cmd, "loco", &mut std::io::stdout());
            return Ok(());
        }
    };

    let path = std::path::Path::new(&dest);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let mut file = std::fs::File::create(path)?;
    generate(shell, cmd, "loco", &mut file);

    output.success(&format!("Installed completions to {dest}"));

    match shell {
        Shell::Zsh => {
            output.info("Ensure ~/.zfunc is in your fpath. Add to ~/.zshrc:");
            output.info("  fpath=(~/.zfunc $fpath)");
            output.info("  autoload -Uz compinit && compinit");
        }
        Shell::Bash => {
            output.info("Ensure ~/.bash_completion.d is sourced. Add to ~/.bashrc:");
            output.info("  for f in ~/.bash_completion.d/*; do source \"$f\"; done");
        }
        _ => {}
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
