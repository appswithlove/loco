use crate::cli::TranslationCommand;
use crate::client::LocoClient;
use crate::output::Output;
use anyhow::Result;

pub async fn run(client: &LocoClient, output: &Output, command: TranslationCommand) -> Result<()> {
    match command {
        TranslationCommand::List { asset_id } => list(client, output, &asset_id).await,
        TranslationCommand::Get { asset_id, locale } => {
            get(client, output, &asset_id, &locale).await
        }
        TranslationCommand::Set {
            asset_id,
            locale,
            text,
        } => set(client, output, &asset_id, &locale, &text).await,
        TranslationCommand::Delete { asset_id, locale } => {
            delete(client, output, &asset_id, &locale).await
        }
        TranslationCommand::Flag {
            asset_id,
            locale,
            flag,
        } => flag_cmd(client, output, &asset_id, &locale, flag).await,
        TranslationCommand::Unflag { asset_id, locale } => {
            unflag(client, output, &asset_id, &locale).await
        }
    }
}

async fn list(client: &LocoClient, output: &Output, asset_id: &str) -> Result<()> {
    let translations = client.get_translations(asset_id).await?;
    if output.is_json() {
        // Translation doesn't derive Serialize; build json manually
        let vals: Vec<serde_json::Value> = translations
            .iter()
            .map(|t| {
                serde_json::json!({
                    "id": t.id,
                    "translation": t.translation,
                    "translated": t.translated,
                    "flagged": t.flagged,
                    "status": t.status,
                })
            })
            .collect();
        output.print_json(&vals)?;
        return Ok(());
    }
    output.info(&format!(
        "{} translation(s) for {asset_id}:",
        translations.len()
    ));
    for t in &translations {
        let flag_str = if t.flagged.is_some() {
            " [flagged]"
        } else {
            ""
        };
        output.info(&format!(
            "  {} - {}{}",
            t.id,
            truncate(&t.translation, 60),
            flag_str,
        ));
    }
    Ok(())
}

async fn get(client: &LocoClient, output: &Output, asset_id: &str, locale: &str) -> Result<()> {
    let t = client.get_translation(asset_id, locale).await?;
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
    output.info(&format!("Asset: {asset_id}"));
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

async fn set(
    client: &LocoClient,
    output: &Output,
    asset_id: &str,
    locale: &str,
    text: &str,
) -> Result<()> {
    let t = client.set_translation(asset_id, locale, text).await?;
    if output.is_json() {
        let val = serde_json::json!({
            "id": t.id,
            "translation": t.translation,
            "translated": t.translated,
        });
        output.print_json(&val)?;
        return Ok(());
    }
    output.success(&format!("Updated {asset_id}/{locale}"));
    Ok(())
}

async fn delete(client: &LocoClient, output: &Output, asset_id: &str, locale: &str) -> Result<()> {
    client.delete_translation(asset_id, locale).await?;
    output.success(&format!("Deleted translation {asset_id}/{locale}"));
    Ok(())
}

async fn flag_cmd(
    client: &LocoClient,
    output: &Output,
    asset_id: &str,
    locale: &str,
    flag: Option<String>,
) -> Result<()> {
    client
        .flag_translation(asset_id, locale, flag.as_deref())
        .await?;
    output.success(&format!("Flagged {asset_id}/{locale}"));
    Ok(())
}

async fn unflag(client: &LocoClient, output: &Output, asset_id: &str, locale: &str) -> Result<()> {
    client.unflag_translation(asset_id, locale).await?;
    output.success(&format!("Unflagged {asset_id}/{locale}"));
    Ok(())
}

fn truncate(s: &str, max: usize) -> &str {
    if s.len() <= max {
        s
    } else {
        &s[..max]
    }
}
