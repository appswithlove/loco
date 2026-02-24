use console::{style, Term};

pub struct Output {
    term: Term,
    json_mode: bool,
    quiet: bool,
}

impl Output {
    pub fn new(json_mode: bool, quiet: bool) -> Self {
        Self {
            term: Term::stderr(),
            json_mode,
            quiet,
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
}
