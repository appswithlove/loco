use crate::cli::StatusArgs;
use crate::client::LocoClient;
use crate::output::Output;
use anyhow::Result;
use console::Style;

pub async fn run(client: &LocoClient, output: &Output, args: StatusArgs) -> Result<()> {
    if let Some(ref code) = args.locale {
        show_single(client, output, code).await
    } else {
        show_all(client, output).await
    }
}

async fn show_single(client: &LocoClient, output: &Output, code: &str) -> Result<()> {
    let locale = client.get_locale(code).await?;

    if output.is_json() {
        output.print_json(&locale)?;
        return Ok(());
    }

    let (translated, total) = progress_numbers(&locale.progress);
    let pct = if total > 0 {
        translated as f64 / total as f64 * 100.0
    } else {
        0.0
    };
    let style = pct_style(pct);
    let untranslated = total.saturating_sub(translated);
    output.info(&format!(
        "{}: {} ({} translated, {} untranslated)",
        locale.code,
        style.apply_to(format!("{pct:.0}%")),
        translated,
        untranslated,
    ));
    Ok(())
}

async fn show_all(client: &LocoClient, output: &Output) -> Result<()> {
    let locales = client.list_locales().await?;

    if output.is_json() {
        output.print_json(&locales)?;
        return Ok(());
    }

    output.info(&format!("{} locale(s):", locales.len()));
    for locale in &locales {
        let (translated, total) = progress_numbers(&locale.progress);
        let pct = if total > 0 {
            translated as f64 / total as f64 * 100.0
        } else {
            0.0
        };
        let style = pct_style(pct);
        output.info(&format!(
            "  {:<10} {} ({}/{})",
            locale.code,
            style.apply_to(format!("{pct:>5.1}%")),
            translated,
            total,
        ));
    }
    Ok(())
}

fn progress_numbers(progress: &Option<crate::models::LocaleProgress>) -> (u32, u32) {
    match progress {
        Some(p) => {
            let translated = p.num_translated.or(p.translated).unwrap_or(0);
            let untranslated = p.num_untranslated.or(p.untranslated).unwrap_or(0);
            (translated, translated + untranslated)
        }
        None => (0, 0),
    }
}

fn pct_style(pct: f64) -> Style {
    if pct >= 80.0 {
        Style::new().green()
    } else if pct >= 50.0 {
        Style::new().yellow()
    } else {
        Style::new().red()
    }
}
