use crate::cli::LocaleCommand;
use crate::client::LocoClient;
use crate::output::Output;
use anyhow::Result;

pub async fn run(client: &LocoClient, output: &Output, command: LocaleCommand) -> Result<()> {
    match command {
        LocaleCommand::List => list(client, output).await,
        LocaleCommand::Get { code } => get(client, output, &code).await,
        LocaleCommand::Create { code } => create(client, output, &code).await,
        LocaleCommand::Delete { code, force } => delete(client, output, &code, force).await,
    }
}

async fn list(client: &LocoClient, output: &Output) -> Result<()> {
    let locales = client.list_locales().await?;
    if output.is_json() {
        output.print_json(&locales)?;
        return Ok(());
    }
    output.info(&format!("{} locale(s):", locales.len()));
    for l in &locales {
        output.info(&format!("  {} - {}", l.code, l.name));
    }
    Ok(())
}

async fn get(client: &LocoClient, output: &Output, code: &str) -> Result<()> {
    let locale = client.get_locale(code).await?;
    if output.is_json() {
        output.print_json(&locale)?;
        return Ok(());
    }
    let (translated, total) = match &locale.progress {
        Some(p) => {
            let t = p.num_translated.unwrap_or(0);
            let u = p.num_untranslated.unwrap_or(0);
            (t, t + u)
        }
        None => (0, 0),
    };
    output.info(&format!("Code: {}", locale.code));
    output.info(&format!("Name: {}", locale.name));
    output.info(&format!("Translated: {}/{}", translated, total));
    Ok(())
}

async fn create(client: &LocoClient, output: &Output, code: &str) -> Result<()> {
    let locale = client.create_locale(code).await?;
    if output.is_json() {
        output.print_json(&locale)?;
        return Ok(());
    }
    output.success(&format!(
        "Created locale: {} ({})",
        locale.code, locale.name
    ));
    Ok(())
}

async fn delete(client: &LocoClient, output: &Output, code: &str, force: bool) -> Result<()> {
    if !output.confirm_or_force(&format!("Delete locale \"{code}\"?"), force) {
        output.info("Aborted");
        return Ok(());
    }
    client.delete_locale(code).await?;
    output.success(&format!("Deleted locale: {code}"));
    Ok(())
}
