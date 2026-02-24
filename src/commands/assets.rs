use crate::cli::AssetCommand;
use crate::client::LocoClient;
use crate::models::CreateAssetRequest;
use crate::output::Output;
use anyhow::Result;

pub async fn run(client: &LocoClient, output: &Output, command: AssetCommand) -> Result<()> {
    match command {
        AssetCommand::List { filter } => list(client, output, filter).await,
        AssetCommand::Get { id } => get(client, output, &id).await,
        AssetCommand::Create {
            id,
            text,
            asset_type,
            context,
            notes,
            translate,
        } => create(client, output, id, text, asset_type, context, notes, translate).await,
        AssetCommand::Delete { id } => delete(client, output, &id).await,
        AssetCommand::Tag { id, tag } => tag_asset(client, output, &id, &tag).await,
        AssetCommand::Untag { id, tag } => untag_asset(client, output, &id, &tag).await,
    }
}

async fn list(client: &LocoClient, output: &Output, filter: Option<String>) -> Result<()> {
    let assets = client.list_assets(filter.as_deref()).await?;
    if output.is_json() {
        output.print_json(&assets)?;
        return Ok(());
    }
    output.info(&format!("{} asset(s):", assets.len()));
    for a in &assets {
        let t = a.asset_type.as_deref().unwrap_or("unknown");
        output.info(&format!("  {} [{}]", a.id, t));
    }
    Ok(())
}

async fn get(client: &LocoClient, output: &Output, id: &str) -> Result<()> {
    let asset = client.get_asset(id).await?;
    if output.is_json() {
        output.print_json(&asset)?;
        return Ok(());
    }
    output.info(&format!("ID: {}", asset.id));
    output.info(&format!(
        "Type: {}",
        asset.asset_type.as_deref().unwrap_or("unknown")
    ));
    if !asset.context.is_empty() {
        output.info(&format!("Context: {}", asset.context));
    }
    if !asset.notes.is_empty() {
        output.info(&format!("Notes: {}", asset.notes));
    }
    if !asset.tags.is_empty() {
        output.info(&format!("Tags: {}", asset.tags.join(", ")));
    }
    Ok(())
}

async fn create(
    client: &LocoClient,
    output: &Output,
    id: String,
    text: Option<String>,
    asset_type: Option<String>,
    context: Option<String>,
    notes: Option<String>,
    translate: Vec<(String, String)>,
) -> Result<()> {
    let req = CreateAssetRequest {
        id: id.clone(),
        text,
        context,
        notes,
        asset_type,
    };
    let asset = client.create_asset(&req).await?;
    output.success(&format!("Created asset: {}", asset.id));

    for (locale, text) in &translate {
        client.set_translation(&asset.id, locale, text).await?;
        output.success(&format!("  {locale}: {text}"));
    }

    if output.is_json() {
        output.print_json(&asset)?;
    }
    Ok(())
}

async fn delete(client: &LocoClient, output: &Output, id: &str) -> Result<()> {
    client.delete_asset(id).await?;
    output.success(&format!("Deleted asset: {id}"));
    Ok(())
}

async fn tag_asset(client: &LocoClient, output: &Output, id: &str, tag: &str) -> Result<()> {
    client.tag_asset(id, tag).await?;
    output.success(&format!("Tagged {id} with \"{tag}\""));
    Ok(())
}

async fn untag_asset(client: &LocoClient, output: &Output, id: &str, tag: &str) -> Result<()> {
    client.untag_asset(id, tag).await?;
    output.success(&format!("Removed tag \"{tag}\" from {id}"));
    Ok(())
}
