use crate::cli::PushArgs;
use crate::client::import::ImportParams;
use crate::client::LocoClient;
use crate::config::ResolvedConfig;
use crate::output::Output;
use anyhow::{bail, Result};
use indicatif::{ProgressBar, ProgressStyle};
use std::path::Path;
use std::time::Duration;

pub async fn run(
    client: &LocoClient,
    output: &Output,
    config: &ResolvedConfig,
    args: PushArgs,
) -> Result<()> {
    let file_path = Path::new(&args.file);
    if !file_path.exists() {
        bail!("File not found: {}", args.file);
    }

    let ext = args
        .format
        .as_deref()
        .or_else(|| file_path.extension().and_then(|e| e.to_str()))
        .unwrap_or("json");

    let body = std::fs::read(&args.file)?;

    let index = args.index.as_deref().or(config.push.index.as_deref());

    let result = client
        .import_file(ImportParams {
            ext,
            body,
            locale: args.locale.as_deref(),
            index,
            format: args.format.as_deref(),
            tag_new: args.tag_new.as_deref(),
            is_async: args.is_async,
        })
        .await?;

    if args.is_async {
        output.info("Import started asynchronously, polling for progress...");
        poll_progress(client, output, &result.message).await?;
    } else if output.is_json() {
        let val = serde_json::json!({
            "message": result.message,
            "status": result.status,
        });
        output.print_json(&val)?;
    } else {
        output.success(&format!("Import complete: {}", result.message));
    }

    Ok(())
}

async fn poll_progress(client: &LocoClient, output: &Output, job_id: &str) -> Result<()> {
    let pb = ProgressBar::new(100);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:30}] {pos}% {msg}")
            .expect("valid template")
            .progress_chars("=> "),
    );

    loop {
        let progress = client.import_progress(job_id).await?;
        pb.set_position(progress.progress as u64);

        if progress.progress >= 100 {
            pb.finish_and_clear();
            if let Some(ref err) = progress.error {
                output.error(&format!("Import failed: {err}"));
            } else {
                output.success("Import complete");
            }
            break;
        }

        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    Ok(())
}
