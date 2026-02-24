use clap::{Parser, Subcommand};
use clap_complete::Shell;

#[derive(Parser, Debug)]
#[command(
    name = "loco-cli",
    about = "CLI for the localise.biz translation management API",
    version,
    propagate_version = true
)]
pub struct Cli {
    /// Loco API key (overrides config/env)
    #[arg(long = "key", global = true, env = "LOCO_API_KEY")]
    pub api_key: Option<String>,

    /// Path to config file
    #[arg(long = "config", global = true)]
    pub config_path: Option<String>,

    /// Suppress non-essential output
    #[arg(long, global = true)]
    pub quiet: bool,

    /// Enable verbose output
    #[arg(long, global = true)]
    pub verbose: bool,

    /// Disable colored output
    #[arg(long, global = true)]
    pub no_color: bool,

    /// Output as JSON
    #[arg(long, global = true)]
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

    /// Manage translation assets
    Assets {
        #[command(subcommand)]
        command: AssetCommand,
    },

    /// Manage project locales
    Locales {
        #[command(subcommand)]
        command: LocaleCommand,
    },

    /// Manage translations
    Translations {
        #[command(subcommand)]
        command: TranslationCommand,
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
    },
}

// --- Pull / Push / Status ---

#[derive(Parser, Debug)]
pub struct PullArgs {
    /// Export format (json, po, xlf, strings, yml, etc.)
    #[arg(long)]
    pub format: Option<String>,

    /// Single locale code to export (default: all)
    #[arg(long)]
    pub locale: Option<String>,

    /// Output path template ({locale} placeholder)
    #[arg(long)]
    pub path: Option<String>,

    /// Filter by tag
    #[arg(long)]
    pub filter: Option<String>,

    /// Filter by translation status
    #[arg(long)]
    pub status: Option<String>,
}

#[derive(Parser, Debug)]
pub struct PushArgs {
    /// File to upload
    #[arg(long, required = true)]
    pub file: String,

    /// Locale of the file
    #[arg(long)]
    pub locale: Option<String>,

    /// Format hint (e.g. json, po, xlf)
    #[arg(long)]
    pub format: Option<String>,

    /// Tag new assets with this tag
    #[arg(long)]
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
    #[arg(long)]
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

// --- Assets ---

#[derive(Subcommand, Debug)]
pub enum AssetCommand {
    /// List all assets
    List {
        /// Filter by tag
        #[arg(long)]
        filter: Option<String>,
    },

    /// Get a single asset by ID
    Get {
        /// Asset ID
        id: String,
    },

    /// Create a new asset
    Create {
        /// Asset ID (key)
        id: String,

        /// Source text
        #[arg(long)]
        text: Option<String>,

        /// Asset type
        #[arg(long, name = "type")]
        asset_type: Option<String>,

        /// Context hint
        #[arg(long)]
        context: Option<String>,

        /// Developer notes
        #[arg(long)]
        notes: Option<String>,
    },

    /// Delete an asset
    Delete {
        /// Asset ID
        id: String,
    },

    /// Add a tag to an asset
    Tag {
        /// Asset ID
        id: String,
        /// Tag name
        tag: String,
    },

    /// Remove a tag from an asset
    Untag {
        /// Asset ID
        id: String,
        /// Tag name
        tag: String,
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
    },
}

// --- Translations ---

#[derive(Subcommand, Debug)]
pub enum TranslationCommand {
    /// List translations for an asset
    List {
        /// Asset ID
        asset_id: String,
    },

    /// Get a specific translation
    Get {
        /// Asset ID
        asset_id: String,
        /// Locale code
        locale: String,
    },

    /// Set a translation value
    Set {
        /// Asset ID
        asset_id: String,
        /// Locale code
        locale: String,
        /// Translation text
        #[arg(long)]
        text: String,
    },

    /// Delete a translation
    Delete {
        /// Asset ID
        asset_id: String,
        /// Locale code
        locale: String,
    },

    /// Flag a translation
    Flag {
        /// Asset ID
        asset_id: String,
        /// Locale code
        locale: String,
        /// Flag value
        #[arg(long)]
        flag: Option<String>,
    },

    /// Unflag a translation
    Unflag {
        /// Asset ID
        asset_id: String,
        /// Locale code
        locale: String,
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
    },
}
