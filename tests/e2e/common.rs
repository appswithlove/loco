use assert_cmd::Command;
use uuid::Uuid;

/// Returns the API key from LOCO_TEST_API_KEY, or None to skip.
pub fn get_e2e_key() -> Option<String> {
    std::env::var("LOCO_TEST_API_KEY").ok()
}

/// Build a Command that hits the real API.
pub fn e2e_cmd(api_key: &str) -> Command {
    let mut cmd = Command::cargo_bin("loco").expect("binary exists");
    cmd.env("LOCO_API_KEY", api_key);
    cmd.env("NO_COLOR", "1");
    // Don't set LOCO_API_URL — use real API
    cmd
}

/// Generate a unique ID for test isolation.
pub fn unique_id(prefix: &str) -> String {
    let short = &Uuid::new_v4().to_string()[..8];
    format!("loco_e2e_{prefix}_{short}")
}

macro_rules! skip_without_key {
    () => {
        match $crate::e2e::common::get_e2e_key() {
            Some(key) => key,
            None => {
                eprintln!("Skipping: LOCO_TEST_API_KEY not set");
                return;
            }
        }
    };
}
pub(crate) use skip_without_key;
