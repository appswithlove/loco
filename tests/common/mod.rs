use assert_cmd::Command;
use std::io::Write;
use tempfile::NamedTempFile;
use wiremock::MockServer;

/// Start a wiremock server and return it.
pub async fn start_mock_server() -> MockServer {
    MockServer::start().await
}

/// Create a temp .loco.toml config file with the given API key.
/// Returns the NamedTempFile (keeps it alive) so the path remains valid.
pub fn create_config_file(api_key: &str) -> NamedTempFile {
    let mut f = NamedTempFile::new().expect("create temp config");
    writeln!(
        f,
        r#"[api]
key = "{api_key}"
"#
    )
    .expect("write config");
    f
}

/// Build an assert_cmd Command for loco-cli pointing at the mock server.
/// Sets LOCO_API_URL and LOCO_API_KEY env vars.
pub fn loco_cmd(server_url: &str, api_key: &str) -> Command {
    let mut cmd = Command::cargo_bin("loco-cli").expect("binary exists");
    cmd.env("LOCO_API_URL", server_url);
    cmd.env("LOCO_API_KEY", api_key);
    // Prevent picking up a real .loco.toml from the filesystem
    cmd.env("NO_COLOR", "1");
    cmd
}
