use crate::cli::TagCommand;
use crate::client::LocoClient;
use crate::output::Output;
use anyhow::Result;

pub async fn run(client: &LocoClient, output: &Output, command: TagCommand) -> Result<()> {
    match command {
        TagCommand::List => list(client, output).await,
        TagCommand::Create { name } => create(client, output, &name).await,
        TagCommand::Rename { old, new } => rename(client, output, &old, &new).await,
        TagCommand::Delete { name, force } => delete(client, output, &name, force).await,
    }
}

async fn list(client: &LocoClient, output: &Output) -> Result<()> {
    let tags = client.list_tags().await?;
    if output.is_json() {
        output.print_json(&tags)?;
        return Ok(());
    }
    output.info(&format!("{} tag(s):", tags.len()));
    for t in &tags {
        output.info(&format!("  {t}"));
    }
    Ok(())
}

async fn create(client: &LocoClient, output: &Output, name: &str) -> Result<()> {
    client.create_tag(name).await?;
    output.success(&format!("Created tag: {name}"));
    Ok(())
}

async fn rename(client: &LocoClient, output: &Output, old: &str, new: &str) -> Result<()> {
    client.rename_tag(old, new).await?;
    output.success(&format!("Renamed tag: {old} -> {new}"));
    Ok(())
}

async fn delete(client: &LocoClient, output: &Output, name: &str, force: bool) -> Result<()> {
    if !output.confirm_or_force(&format!("Delete tag \"{name}\"?"), force) {
        output.info("Aborted");
        return Ok(());
    }
    client.delete_tag(name).await?;
    output.success(&format!("Deleted tag: {name}"));
    Ok(())
}
