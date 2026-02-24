use crate::cli::PullArgs;
use crate::client::export::ExportParams;
use crate::client::LocoClient;
use crate::config::ResolvedConfig;
use crate::output::Output;
use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::Path;

pub async fn run(
    client: &LocoClient,
    output: &Output,
    config: &ResolvedConfig,
    args: PullArgs,
) -> Result<()> {
    let format = args
        .format
        .as_deref()
        .or(config.pull.format.as_deref())
        .unwrap_or("json");

    let path_template = args
        .path
        .as_deref()
        .or(config.pull.path.as_deref())
        .unwrap_or("./{locale}.{format}");

    let params = ExportParams {
        format: Some(format.to_string()),
        filter: args.filter.clone(),
        status: args.status.clone(),
        index: None,
    };

    if let Some(ref locale) = args.locale {
        export_single(client, output, locale, format, path_template, &params).await?;
    } else {
        export_all(client, output, format, path_template, &params).await?;
    }

    Ok(())
}

async fn export_single(
    client: &LocoClient,
    output: &Output,
    locale: &str,
    format: &str,
    path_template: &str,
    params: &ExportParams,
) -> Result<()> {
    let bytes = client.export_locale(locale, format, params).await?;
    let dest = resolve_path(path_template, locale, format);
    write_file(&dest, &bytes)?;
    output.success(&format!("Exported {locale} -> {dest}"));
    Ok(())
}

async fn export_all(
    client: &LocoClient,
    output: &Output,
    format: &str,
    path_template: &str,
    params: &ExportParams,
) -> Result<()> {
    let locales = client.list_locales().await?;

    let pb = ProgressBar::new(locales.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:30}] {pos}/{len} {msg}")
            .expect("valid template")
            .progress_chars("=> "),
    );

    let mut count = 0u64;
    for locale in &locales {
        pb.set_message(locale.code.clone());
        let bytes = client.export_locale(&locale.code, format, params).await?;
        let dest = resolve_path(path_template, &locale.code, format);
        write_file(&dest, &bytes)?;
        count += 1;
        pb.inc(1);
    }

    pb.finish_and_clear();
    output.success(&format!("Exported {count} locale(s)"));
    Ok(())
}

fn resolve_path(template: &str, locale: &str, format: &str) -> String {
    template
        .replace("{locale}", locale)
        .replace("{format}", format)
}

fn write_file(path: &str, data: &[u8]) -> Result<()> {
    if let Some(parent) = Path::new(path).parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, data)?;
    Ok(())
}
