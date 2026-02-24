use crate::cli::AuthCommand;
use crate::client::LocoClient;
use crate::output::Output;
use anyhow::Result;

pub async fn run(client: &LocoClient, output: &Output, command: AuthCommand) -> Result<()> {
    match command {
        AuthCommand::Verify => verify(client, output).await,
    }
}

async fn verify(client: &LocoClient, output: &Output) -> Result<()> {
    let auth = client.auth_verify().await?;

    if output.is_json() {
        // AuthResponse doesn't derive Serialize; re-serialize via serde_json::Value
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
