use crate::cli::AuthCommand;
use crate::client::LocoClient;
use crate::config::ResolvedConfig;
use crate::output::Output;
use anyhow::Result;
use dialoguer::Input;

const DEFAULT_BASE_URL: &str = "https://localise.biz/api";
const CONFIG_FILE: &str = ".loco.toml";

pub async fn run(
    cli_key: &Option<String>,
    cli_config: &Option<String>,
    output: &Output,
    command: AuthCommand,
) -> Result<()> {
    match command {
        AuthCommand::Verify => {
            let config = ResolvedConfig::load(cli_key.as_deref(), cli_config.as_deref())?;
            let client = LocoClient::new(&config.api_key, &config.base_url)?;
            verify(&client, output).await
        }
        AuthCommand::Init => init_key(output).await,
    }
}

async fn verify(client: &LocoClient, output: &Output) -> Result<()> {
    let auth = client.auth_verify().await?;

    if output.is_json() {
        let val = serde_json::json!({
            "user": {
                "id": auth.user.id,
                "name": auth.user.name,
                "email": auth.user.email,
            },
            "project": {
                "id": auth.project.id,
                "name": auth.project.name,
                "url": auth.project.url,
            }
        });
        output.print_json(&val)?;
        return Ok(());
    }

    output.success("Authenticated successfully");
    output.info(&format!(
        "Project: {} ({})",
        auth.project.name, auth.project.id
    ));
    output.info(&format!("User: {} <{}>", auth.user.name, auth.user.email));

    Ok(())
}

async fn init_key(output: &Output) -> Result<()> {
    let api_key: String = Input::new()
        .with_prompt("API key")
        .interact_text()?;

    if api_key.is_empty() {
        anyhow::bail!("No API key provided");
    }

    // Verify
    output.info("Verifying API key...");
    let base_url = std::env::var("LOCO_API_URL").unwrap_or_else(|_| DEFAULT_BASE_URL.to_string());
    match LocoClient::new(&api_key, &base_url) {
        Ok(client) => match client.auth_verify().await {
            Ok(auth) => {
                output.success(&format!(
                    "Authenticated: {} ({})",
                    auth.project.name, auth.user.email
                ));
            }
            Err(e) => {
                output.warn(&format!("Could not verify key: {e}"));
                output.warn("Saving anyway — you can fix this later.");
            }
        },
        Err(e) => {
            output.warn(&format!("Client error: {e}"));
        }
    }

    // Save to config — merge with existing if present
    save_key_to_config(&api_key)?;
    output.success(&format!("API key saved to {CONFIG_FILE}"));
    output.warn("Add .loco.toml to .gitignore to avoid leaking secrets");
    Ok(())
}

/// Upsert the `[api] key` in .loco.toml, preserving other sections.
fn save_key_to_config(api_key: &str) -> Result<()> {
    let content = std::fs::read_to_string(CONFIG_FILE).unwrap_or_default();
    let mut doc: toml::Table = if content.is_empty() {
        toml::Table::new()
    } else {
        content.parse()?
    };

    let api = doc
        .entry("api")
        .or_insert_with(|| toml::Value::Table(toml::Table::new()));
    if let toml::Value::Table(ref mut t) = api {
        t.insert("key".to_string(), toml::Value::String(api_key.to_string()));
    }

    std::fs::write(CONFIG_FILE, toml::to_string_pretty(&doc)?)?;
    Ok(())
}
