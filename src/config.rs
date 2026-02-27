use crate::error::LocoError;
use serde::Deserialize;
use std::path::PathBuf;

const CONFIG_FILENAME: &str = ".loco.toml";
const DEFAULT_BASE_URL: &str = "https://localise.biz/api";

#[derive(Debug, Deserialize, Default)]
pub struct ConfigFile {
    #[serde(default)]
    pub api: ApiConfig,
    #[serde(default)]
    pub pull: PullConfig,
    #[serde(default)]
    pub push: PushConfig,
}

#[derive(Debug, Deserialize, Default)]
pub struct ApiConfig {
    pub key: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct PullConfig {
    pub format: Option<String>,
    pub path: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct PushConfig {
    pub index: Option<String>,
}

#[derive(Debug)]
pub struct ResolvedConfig {
    pub api_key: String,
    pub base_url: String,
    pub pull: PullConfig,
    pub push: PushConfig,
    pub config_path: Option<PathBuf>,
}

impl ResolvedConfig {
    /// Load config by merging: CLI flags > env vars > config file > defaults
    pub fn load(cli_key: Option<&str>, cli_config_path: Option<&str>) -> Result<Self, LocoError> {
        let (config_file, config_path) = load_config_file(cli_config_path)?;

        let api_key = cli_key
            .map(String::from)
            .or_else(|| std::env::var("LOCO_API_KEY").ok())
            .or(config_file.api.key)
            .ok_or_else(|| {
                LocoError::Config(
                    "No API key found. Set LOCO_API_KEY env var, use --key, or add to .loco.toml"
                        .into(),
                )
            })?;

        let base_url =
            std::env::var("LOCO_API_URL").unwrap_or_else(|_| DEFAULT_BASE_URL.to_string());

        Ok(Self {
            api_key,
            base_url,
            pull: config_file.pull,
            push: config_file.push,
            config_path,
        })
    }
}

fn load_config_file(
    explicit_path: Option<&str>,
) -> Result<(ConfigFile, Option<PathBuf>), LocoError> {
    let path = if let Some(p) = explicit_path {
        Some(PathBuf::from(p))
    } else {
        find_config_file()
    };

    let Some(path) = path else {
        return Ok((ConfigFile::default(), None));
    };

    if !path.exists() {
        return Ok((ConfigFile::default(), None));
    }

    let content = std::fs::read_to_string(&path)
        .map_err(|e| LocoError::Config(format!("Failed to read {}: {e}", path.display())))?;

    let config = toml::from_str(&content)
        .map_err(|e| LocoError::Config(format!("Invalid config in {}: {e}", path.display())))?;

    Ok((config, Some(path)))
}

/// Walk up from cwd to find .loco.toml
fn find_config_file() -> Option<PathBuf> {
    let mut dir = std::env::current_dir().ok()?;
    loop {
        let candidate = dir.join(CONFIG_FILENAME);
        if candidate.exists() {
            return Some(candidate);
        }
        if !dir.pop() {
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    /// Serialize tests that touch env vars / cwd to avoid races.
    static ENV_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn load_with_env_var() {
        let _lock = ENV_LOCK.lock().unwrap();
        let key = "env-key-12345";
        std::env::set_var("LOCO_API_KEY", key);
        let cfg = ResolvedConfig::load(None, Some("/nonexistent/.loco.toml"));
        std::env::remove_var("LOCO_API_KEY");
        let cfg = cfg.unwrap();
        assert_eq!(cfg.api_key, key);
    }

    #[test]
    fn load_with_config_file() {
        let _lock = ENV_LOCK.lock().unwrap();
        std::env::remove_var("LOCO_API_KEY");

        let path = {
            let f = tempfile::NamedTempFile::new().unwrap();
            let p = f.path().to_path_buf();
            // Keep temp file alive by persisting it
            f.persist(&p).unwrap();
            p
        };
        std::fs::write(&path, "[api]\nkey = \"file-key-abc\"\n").unwrap();

        let cfg = ResolvedConfig::load(None, Some(path.to_str().unwrap())).unwrap();
        std::fs::remove_file(&path).ok();
        assert_eq!(cfg.api_key, "file-key-abc");
    }

    #[test]
    fn load_cli_key_takes_precedence() {
        let _lock = ENV_LOCK.lock().unwrap();
        std::env::set_var("LOCO_API_KEY", "env-key");
        let cfg = ResolvedConfig::load(Some("cli-key"), Some("/nonexistent/.loco.toml"));
        std::env::remove_var("LOCO_API_KEY");
        let cfg = cfg.unwrap();
        assert_eq!(cfg.api_key, "cli-key");
    }

    #[test]
    fn load_missing_key_returns_error() {
        let _lock = ENV_LOCK.lock().unwrap();
        std::env::remove_var("LOCO_API_KEY");
        let result = ResolvedConfig::load(None, Some("/nonexistent/.loco.toml"));
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, LocoError::Config(_)));
    }

    #[test]
    fn find_config_file_walks_up() {
        let _lock = ENV_LOCK.lock().unwrap();
        let dir = tempfile::TempDir::new().unwrap();
        let child = dir.path().join("a/b/c");
        std::fs::create_dir_all(&child).unwrap();

        std::fs::write(dir.path().join(".loco.toml"), "[api]\nkey = \"found\"\n").unwrap();

        let original = std::env::current_dir().unwrap();
        std::env::set_current_dir(&child).unwrap();

        let found = find_config_file();

        std::env::set_current_dir(original).unwrap();

        assert!(found.is_some());
        let content = std::fs::read_to_string(found.unwrap()).unwrap();
        assert!(content.contains("found"));
    }
}
