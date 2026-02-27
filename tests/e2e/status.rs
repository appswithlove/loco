use super::common;
use predicates::prelude::*;

#[tokio::test]
#[serial_test::serial]
async fn e2e_status_human() {
    let key = common::skip_without_key!();

    // Status should list locales with progress info
    common::e2e_cmd(&key)
        .args(["status"])
        .assert()
        .success()
        .stderr(predicate::str::contains("locale(s)"));
}

#[tokio::test]
#[serial_test::serial]
async fn e2e_status_json() {
    let key = common::skip_without_key!();

    let output = common::e2e_cmd(&key)
        .args(["--json", "status"])
        .assert()
        .success();

    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
    let val: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON");
    assert!(val.is_array(), "status JSON should be an array of locales");
    let arr = val.as_array().unwrap();
    // Project should have at least one locale
    assert!(!arr.is_empty(), "should have at least one locale");
    // Each entry should have a code field
    for locale in arr {
        assert!(
            locale["code"].is_string(),
            "each locale should have a code field"
        );
    }
}

#[tokio::test]
#[serial_test::serial]
async fn e2e_status_single_locale() {
    let key = common::skip_without_key!();

    // Get a locale code from the project first
    let output = common::e2e_cmd(&key)
        .args(["--json", "locales", "list"])
        .assert()
        .success();

    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
    let val: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON");
    let arr = val.as_array().expect("should be array");
    if arr.is_empty() {
        eprintln!("Skipping: no locales in project");
        return;
    }

    let code = arr[0]["code"].as_str().expect("locale code");

    // Status for single locale
    common::e2e_cmd(&key)
        .args(["status", "--locale", code])
        .assert()
        .success()
        .stderr(predicate::str::contains(code));
}
