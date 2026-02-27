use clap::{Parser, Subcommand};
use clap_complete::Shell;

pub fn parse_translation(s: &str) -> Result<(String, String), String> {
    let (locale, text) = s
        .split_once('=')
        .ok_or_else(|| format!("expected LOCALE=TEXT, got: {s}"))?;
    if locale.is_empty() {
        return Err("locale cannot be empty".to_string());
    }
    Ok((locale.to_string(), text.to_string()))
}

#[derive(Parser, Debug)]
#[command(
    name = "loco",
    about = "CLI for the localise.biz translation management API",
    version,
    propagate_version = true
)]
pub struct Cli {
    /// Loco API key (overrides config/env)
    #[arg(short = 'k', long = "key", global = true, env = "LOCO_API_KEY")]
    pub api_key: Option<String>,

    /// Path to config file
    #[arg(short = 'c', long = "config", global = true)]
    pub config_path: Option<String>,

    /// Suppress non-essential output
    #[arg(short = 'q', long, global = true)]
    pub quiet: bool,

    /// Enable verbose output
    #[arg(short = 'v', long, global = true)]
    pub verbose: bool,

    /// Disable colored output
    #[arg(long, global = true)]
    pub no_color: bool,

    /// Output as JSON
    #[arg(short = 'j', long, global = true)]
    pub json: bool,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Interactive project setup
    Init,

    /// Export translations to local files
    Pull(PullArgs),

    /// Import local translation files
    Push(PushArgs),

    /// Show translation progress
    Status(StatusArgs),

    /// Authentication commands
    Auth {
        #[command(subcommand)]
        command: AuthCommand,
    },

    /// Manage translatable strings
    Strings {
        #[command(subcommand)]
        command: StringCommand,
    },

    /// Manage project locales
    Locales {
        #[command(subcommand)]
        command: LocaleCommand,
    },

    /// Manage tags
    Tags {
        #[command(subcommand)]
        command: TagCommand,
    },

    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,

        /// Install completions to the standard location
        #[arg(long)]
        install: bool,
    },
}

// --- Pull / Push / Status ---

#[derive(Parser, Debug)]
pub struct PullArgs {
    /// Export format (json, po, xlf, strings, yml, etc.)
    #[arg(short = 'f', long)]
    pub format: Option<String>,

    /// Single locale code to export (default: all)
    #[arg(short = 'l', long)]
    pub locale: Option<String>,

    /// Output path template ({locale} placeholder)
    #[arg(short = 'p', long)]
    pub path: Option<String>,

    /// Filter by tag
    #[arg(long)]
    pub filter: Option<String>,

    /// Filter by translation status
    #[arg(short = 's', long)]
    pub status: Option<String>,
}

#[derive(Parser, Debug)]
pub struct PushArgs {
    /// File to upload
    #[arg(long, required = true)]
    pub file: String,

    /// Locale of the file
    #[arg(short = 'l', long)]
    pub locale: Option<String>,

    /// Format hint (e.g. json, po, xlf)
    #[arg(short = 'f', long)]
    pub format: Option<String>,

    /// Tag new assets with this tag
    #[arg(short = 't', long)]
    pub tag_new: Option<String>,

    /// Run import asynchronously
    #[arg(long = "async")]
    pub is_async: bool,

    /// Key mapping: "id" or "text"
    #[arg(long)]
    pub index: Option<String>,
}

#[derive(Parser, Debug)]
pub struct StatusArgs {
    /// Show progress for a specific locale
    #[arg(short = 'l', long)]
    pub locale: Option<String>,
}

// --- Auth ---

#[derive(Subcommand, Debug)]
pub enum AuthCommand {
    /// Verify your API key
    Verify,
    /// Set up and save your API key
    Init,
}

// --- Strings ---

#[derive(Subcommand, Debug)]
pub enum StringCommand {
    /// List all strings
    List {
        /// Filter by tag
        #[arg(long)]
        filter: Option<String>,
    },

    /// Get string details (optionally for a single locale)
    Get {
        /// String ID
        id: String,
        /// Locale code (omit to show all translations)
        locale: Option<String>,
    },

    /// Add a new string with translations (interactive prompt if no translations given)
    Add {
        /// String ID (key)
        id: String,

        /// Translations as LOCALE=TEXT (e.g. en=Hello de=Hallo)
        #[arg(value_parser = parse_translation)]
        translations: Vec<(String, String)>,

        /// Asset type
        #[arg(long, name = "type")]
        asset_type: Option<String>,

        /// Context hint
        #[arg(long)]
        context: Option<String>,

        /// Developer notes
        #[arg(long)]
        notes: Option<String>,

        /// Update if string already exists instead of failing
        #[arg(long)]
        update: bool,
    },

    /// Delete a string and all its translations
    Delete {
        /// String ID
        id: String,
        /// Skip confirmation prompt
        #[arg(short = 'y', long)]
        force: bool,
    },

    /// Set translations for a string
    Set {
        /// String ID
        id: String,
        /// Translations as LOCALE=TEXT (e.g. en=Hello de=Hallo)
        #[arg(value_parser = parse_translation, required = true)]
        translations: Vec<(String, String)>,
        /// Create the string if it doesn't exist
        #[arg(long)]
        create: bool,
    },

    /// Remove a single translation
    Rm {
        /// String ID
        id: String,
        /// Locale code
        locale: String,
    },

    /// Add a tag to a string
    Tag {
        /// String ID
        id: String,
        /// Tag name
        tag: String,
    },

    /// Remove a tag from a string
    Untag {
        /// String ID
        id: String,
        /// Tag name
        tag: String,
    },

    /// Flag a translation
    Flag {
        /// String ID
        id: String,
        /// Locale code
        locale: String,
        /// Flag value
        #[arg(long)]
        flag: Option<String>,
    },

    /// Unflag a translation
    Unflag {
        /// String ID
        id: String,
        /// Locale code
        locale: String,
    },
}

// --- Locales ---

#[derive(Subcommand, Debug)]
pub enum LocaleCommand {
    /// List all locales
    List,

    /// Get locale details
    Get {
        /// Locale code (e.g. en, fr-FR)
        code: String,
    },

    /// Create a new locale
    Create {
        /// Locale code
        code: String,
    },

    /// Delete a locale
    Delete {
        /// Locale code
        code: String,
        /// Skip confirmation prompt
        #[arg(short = 'y', long)]
        force: bool,
    },
}

// --- Tags ---

#[derive(Subcommand, Debug)]
pub enum TagCommand {
    /// List all tags
    List,

    /// Create a tag
    Create {
        /// Tag name
        name: String,
    },

    /// Rename a tag
    Rename {
        /// Current name
        old: String,
        /// New name
        new: String,
    },

    /// Delete a tag
    Delete {
        /// Tag name
        name: String,
        /// Skip confirmation prompt
        #[arg(short = 'y', long)]
        force: bool,
    },
}
