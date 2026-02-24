use thiserror::Error;

#[derive(Error, Debug)]
pub enum LocoError {
    #[error("Authentication failed — check your LOCO_API_KEY")]
    Unauthorized,

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Rate limited — wait and retry")]
    RateLimited,

    #[error("API error ({status}): {message}")]
    Api { status: u16, message: String },

    #[error("Config error: {0}")]
    Config(String),

    #[error("{0}")]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    Http(#[from] reqwest::Error),

    #[error("{0}")]
    Json(#[from] serde_json::Error),
}

// Exit codes
pub const EXIT_OK: i32 = 0;
pub const EXIT_ERROR: i32 = 1;
pub const EXIT_AUTH: i32 = 2;
pub const EXIT_CONFIG: i32 = 3;
