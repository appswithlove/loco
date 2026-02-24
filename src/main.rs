mod cli;
mod client;
mod commands;
mod config;
mod error;
mod models;
mod output;

use clap::Parser;

use crate::error::{EXIT_AUTH, EXIT_CONFIG, EXIT_ERROR, EXIT_OK};
use crate::output::Output;

#[tokio::main]
async fn main() {
    let cli = cli::Cli::parse();
    let output = Output::new(cli.json, cli.quiet);

    if let Err(e) = commands::run(cli).await {
        let code = exit_code_for(&e);
        output.error(&format!("{e:#}"));
        std::process::exit(code);
    }

    std::process::exit(EXIT_OK);
}

fn exit_code_for(err: &anyhow::Error) -> i32 {
    if let Some(loco_err) = err.downcast_ref::<crate::error::LocoError>() {
        match loco_err {
            crate::error::LocoError::Unauthorized => EXIT_AUTH,
            crate::error::LocoError::Config(_) => EXIT_CONFIG,
            _ => EXIT_ERROR,
        }
    } else {
        EXIT_ERROR
    }
}
