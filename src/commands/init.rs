use crate::client::LocoClient;
use crate::output::Output;
use anyhow::Result;
use dialoguer::{Input, Select};

const DEFAULT_BASE_URL: &str = "https://localise.biz/api";
const CONFIG_FILE: &str = ".loco.toml";

pub async fn run(output: &Output) -> Result<()> {
    output.info("Loco CLI setup wizard");

    let api_key: String = Input::new()
        .with_prompt("API key (or set LOCO_API_KEY env var)")
        .interact_text()?;

    // Verify key
    output.info("Verifying API key...");
    match LocoClient::new(&api_key, DEFAULT_BASE_URL) {
        Ok(client) => match client.auth_verify().await {
            Ok(auth) => {
                output.success(&format!(
                    "Authenticated: {} ({})",
                    auth.project.name, auth.user.email
                ));
            }
            Err(e) => {
                output.warn(&format!("Could not verify key: {e}"));
                output.warn("Continuing anyway -- you can fix this later.");
            }
        },
        Err(e) => {
            output.warn(&format!("Client error: {e}"));
        }
    }

    let formats = &["json", "po", "xlf", "strings", "yml", "xml", "csv"];
    let format_idx = Select::new()
        .with_prompt("Default export format")
        .items(formats)
        .default(0)
        .interact()?;
    let format = formats[format_idx];

    let path: String = Input::new()
        .with_prompt("Output path pattern")
        .default(format!("./locales/{{locale}}.{format}"))
        .interact_text()?;

    let config_content = format!(
        r#"# Loco CLI configuration
# See: https://localise.biz/api

[pull]
format = "{format}"
path = "{path}"

[push]
# index = "id"
"#
    );

    std::fs::write(CONFIG_FILE, &config_content)?;
    output.success(&format!("Config written to {CONFIG_FILE}"));
    output.info("Tip: add your API key as LOCO_API_KEY env var or pass --key");
    Ok(())
}
