use crate::cli::{parse_translation, StringCommand};
use crate::client::LocoClient;
use crate::models::CreateAssetRequest;
use crate::output::Output;
use anyhow::Result;
use console::Term;
use dialoguer::Input;

pub async fn run(client: &LocoClient, output: &Output, command: StringCommand) -> Result<()> {
    match command {
        StringCommand::List { filter } => list(client, output, filter).await,
        StringCommand::Get { id, locale: None } => get_all(client, output, &id).await,
        StringCommand::Get {
            id,
            locale: Some(locale),
        } => get_one(client, output, &id, &locale).await,
        StringCommand::Add {
            id,
            translations,
            asset_type,
            context,
            notes,
            update,
        } => {
            add(
                client,
                output,
                id,
                translations,
                asset_type,
                context,
                notes,
                update,
            )
            .await
        }
        StringCommand::Delete { id, force } => delete(client, output, &id, force).await,
        StringCommand::Set {
            id,
            translations,
            create,
        } => set(client, output, &id, translations, create).await,
        StringCommand::Rm { id, locale } => rm(client, output, &id, &locale).await,
        StringCommand::Tag { id, tag } => tag_string(client, output, &id, &tag).await,
        StringCommand::Untag { id, tag } => untag_string(client, output, &id, &tag).await,
        StringCommand::Flag { id, locale, flag } => {
            flag_string(client, output, &id, &locale, flag).await
        }
        StringCommand::Unflag { id, locale } => unflag_string(client, output, &id, &locale).await,
    }
}

async fn list(client: &LocoClient, output: &Output, filter: Option<String>) -> Result<()> {
    let assets = client.list_assets(filter.as_deref()).await?;
    if output.is_json() {
        output.print_json(&assets)?;
        return Ok(());
    }
    output.info(&format!("{} string(s):", assets.len()));
    for a in &assets {
        let t = a.asset_type.as_deref().unwrap_or("unknown");
        output.info(&format!("  {} [{}]", a.id, t));
    }
    Ok(())
}

async fn get_all(client: &LocoClient, output: &Output, id: &str) -> Result<()> {
    let asset = client.get_asset(id).await?;
    let translations = client.get_translations(id).await.unwrap_or_default();

    if output.is_json() {
        let val = serde_json::json!({
            "asset": asset,
            "translations": translations.iter().map(|t| {
                serde_json::json!({"locale": t.id, "text": t.translation, "translated": t.translated})
            }).collect::<Vec<_>>(),
        });
        output.print_json(&val)?;
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
    if !translations.is_empty() {
        output.info("Translations:");
        for t in &translations {
            let status = if t.translated { "✓" } else { "·" };
            output.info(&format!("  {status} {}: {}", t.id, t.translation));
        }
    }
    Ok(())
}

async fn get_one(client: &LocoClient, output: &Output, id: &str, locale: &str) -> Result<()> {
    let t = client.get_translation(id, locale).await?;
    if output.is_json() {
        let val = serde_json::json!({
            "id": t.id,
            "translation": t.translation,
            "translated": t.translated,
            "flagged": t.flagged,
            "status": t.status,
            "revision": t.revision,
            "modified": t.modified,
        });
        output.print_json(&val)?;
        return Ok(());
    }
    output.info(&format!("String: {id}"));
    output.info(&format!("Locale: {locale}"));
    output.info(&format!(
        "Status: {}",
        t.status.as_deref().unwrap_or("unknown")
    ));
    output.info(&format!(
        "Flagged: {}",
        t.flagged.as_deref().unwrap_or("none")
    ));
    output.info(&format!("Text: {}", t.translation));
    Ok(())
}

async fn add(
    client: &LocoClient,
    output: &Output,
    id: String,
    mut translations: Vec<(String, String)>,
    asset_type: Option<String>,
    context: Option<String>,
    notes: Option<String>,
    update: bool,
) -> Result<()> {
    // Interactive mode: no translations provided and TTY
    if translations.is_empty() && Term::stderr().is_term() {
        let locales = client.list_locales().await?;
        let source_code = locales
            .first()
            .map(|l| l.code.clone())
            .unwrap_or_else(|| "en".to_string());

        let source_text: String = Input::new()
            .with_prompt(format!("Source text ({source_code})"))
            .interact_text()?;

        if !source_text.is_empty() {
            translations.push((source_code, source_text));
        }

        output.info("Add translations (LOCALE=TEXT, empty to finish):");
        loop {
            let input: String = Input::new()
                .with_prompt(">")
                .allow_empty(true)
                .interact_text()?;

            if input.is_empty() {
                break;
            }

            match parse_translation(&input) {
                Ok(pair) => translations.push(pair),
                Err(e) => output.warn(&format!("Skipped: {e}")),
            }
        }
    }

    // Use first translation's text as source text for asset creation
    let source_text = translations.first().map(|(_, text)| text.clone());

    let req = CreateAssetRequest {
        id: id.clone(),
        text: source_text.clone(),
        context,
        notes,
        asset_type,
    };

    match client.create_asset(&req).await {
        Ok(asset) => {
            output.success(&format!("Created string: {}", asset.id));
        }
        Err(crate::error::LocoError::Api { status: 409, .. }) if update => {
            output.info(&format!(
                "String {id} already exists, updating translations"
            ));
        }
        Err(e) => return Err(e.into()),
    }

    for (locale, val) in &translations {
        client.set_translation(&id, locale, val).await?;
        output.success(&format!("  {locale}: {val}"));
    }

    // If --update with source text, set it on the source locale
    if update {
        if let Some(ref t) = source_text {
            let locales = client.list_locales().await?;
            if let Some(source) = locales.first() {
                // Only set if not already in the translations list
                if !translations.iter().any(|(l, _)| l == &source.code) {
                    client.set_translation(&id, &source.code, t).await?;
                }
            }
        }
    }

    Ok(())
}

async fn delete(client: &LocoClient, output: &Output, id: &str, force: bool) -> Result<()> {
    if !output.confirm_or_force(
        &format!("Delete string \"{id}\" and all its translations?"),
        force,
    ) {
        output.info("Aborted");
        return Ok(());
    }
    client.delete_asset(id).await?;
    output.success(&format!("Deleted string: {id}"));
    Ok(())
}

async fn set(
    client: &LocoClient,
    output: &Output,
    id: &str,
    translations: Vec<(String, String)>,
    create: bool,
) -> Result<()> {
    if create {
        let req = CreateAssetRequest {
            id: id.to_string(),
            text: None,
            context: None,
            notes: None,
            asset_type: None,
        };
        match client.create_asset(&req).await {
            Ok(_) => output.success(&format!("Created string: {id}")),
            Err(crate::error::LocoError::Api { status: 409, .. }) => {}
            Err(e) => return Err(e.into()),
        }
    }
    let mut results = Vec::new();
    for (locale, text) in &translations {
        let t = client.set_translation(id, locale, text).await?;
        results.push(t);
    }
    if output.is_json() {
        let vals: Vec<_> = results
            .iter()
            .map(|t| {
                serde_json::json!({
                    "id": t.id,
                    "translation": t.translation,
                    "translated": t.translated,
                })
            })
            .collect();
        output.print_json(&vals)?;
        return Ok(());
    }
    for (locale, text) in &translations {
        output.success(&format!("{id}/{locale}: {text}"));
    }
    Ok(())
}

async fn rm(client: &LocoClient, output: &Output, id: &str, locale: &str) -> Result<()> {
    client.delete_translation(id, locale).await?;
    output.success(&format!("Removed translation {id}/{locale}"));
    Ok(())
}

async fn tag_string(client: &LocoClient, output: &Output, id: &str, tag: &str) -> Result<()> {
    client.tag_asset(id, tag).await?;
    output.success(&format!("Tagged {id} with \"{tag}\""));
    Ok(())
}

async fn untag_string(client: &LocoClient, output: &Output, id: &str, tag: &str) -> Result<()> {
    client.untag_asset(id, tag).await?;
    output.success(&format!("Removed tag \"{tag}\" from {id}"));
    Ok(())
}

async fn flag_string(
    client: &LocoClient,
    output: &Output,
    id: &str,
    locale: &str,
    flag: Option<String>,
) -> Result<()> {
    client.flag_translation(id, locale, flag.as_deref()).await?;
    output.success(&format!("Flagged {id}/{locale}"));
    Ok(())
}

async fn unflag_string(client: &LocoClient, output: &Output, id: &str, locale: &str) -> Result<()> {
    client.unflag_translation(id, locale).await?;
    output.success(&format!("Unflagged {id}/{locale}"));
    Ok(())
}
