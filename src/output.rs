use console::{style, Term};
use dialoguer::Confirm;

pub struct Output {
    term: Term,
    json_mode: bool,
    quiet: bool,
    verbose: bool,
}

impl Output {
    pub fn new(json_mode: bool, quiet: bool, verbose: bool) -> Self {
        Self {
            term: Term::stderr(),
            json_mode,
            quiet,
            verbose,
        }
    }

    pub fn success(&self, msg: &str) {
        if !self.quiet {
            let _ = self
                .term
                .write_line(&format!("{} {}", style("✓").green().bold(), msg));
        }
    }

    pub fn error(&self, msg: &str) {
        if self.json_mode {
            let val = serde_json::json!({"error": msg});
            let _ = println!("{}", serde_json::to_string_pretty(&val).unwrap_or_default());
            return;
        }
        let _ = self
            .term
            .write_line(&format!("{} {}", style("✗").red().bold(), msg));
    }

    pub fn info(&self, msg: &str) {
        if !self.quiet {
            let _ = self
                .term
                .write_line(&format!("{} {}", style("ℹ").blue().bold(), msg));
        }
    }

    pub fn warn(&self, msg: &str) {
        if !self.quiet {
            let _ = self
                .term
                .write_line(&format!("{} {}", style("⚠").yellow().bold(), msg));
        }
    }

    pub fn print_json<T: serde::Serialize>(&self, value: &T) -> anyhow::Result<()> {
        let json = serde_json::to_string_pretty(value)?;
        println!("{json}");
        Ok(())
    }

    pub fn is_json(&self) -> bool {
        self.json_mode
    }

    pub fn verbose(&self, msg: &str) {
        if self.verbose {
            let _ = self
                .term
                .write_line(&format!("{} {}", style("·").dim(), msg));
        }
    }

    pub fn confirm_or_force(&self, msg: &str, force: bool) -> bool {
        if force {
            return true;
        }
        if !self.term.is_term() {
            self.error("Use --force (-y) to skip confirmation in non-interactive mode");
            return false;
        }
        Confirm::new()
            .with_prompt(msg)
            .default(false)
            .interact()
            .unwrap_or(false)
    }
}
