use super::common;
use predicates::prelude::*;

#[tokio::test]
#[serial_test::serial]
async fn e2e_auth_verify() {
    let key = common::skip_without_key!();
    common::e2e_cmd(&key)
        .args(["auth", "verify"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Authenticated"));
}

#[tokio::test]
#[serial_test::serial]
async fn e2e_auth_verify_json() {
    let key = common::skip_without_key!();
    let output = common::e2e_cmd(&key)
        .args(["--json", "auth", "verify"])
        .assert()
        .success();

    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
    let val: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON");
    assert!(
        val["user"]["name"].is_string(),
        "user.name should be a string"
    );
    assert!(
        val["project"]["name"].is_string(),
        "project.name should be a string"
    );
}
