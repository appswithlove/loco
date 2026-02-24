mod common;

use predicates::prelude::*;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, ResponseTemplate};

const TEST_KEY: &str = "test-key-abc123";

#[tokio::test]
async fn auth_verify_success() {
    let server = common::start_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/auth/verify"))
        .and(header("Authorization", &format!("Loco {TEST_KEY}")))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "user": { "id": 1, "name": "Alice", "email": "alice@example.com" },
            "project": { "id": 42, "name": "My Project", "url": "https://localise.biz/projects/42" }
        })))
        .expect(1)
        .mount(&server)
        .await;

    common::loco_cmd(&server.uri(), TEST_KEY)
        .args(["auth", "verify"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Authenticated"))
        .stderr(predicate::str::contains("My Project"));
}

#[tokio::test]
async fn auth_verify_invalid_key() {
    let server = common::start_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/auth/verify"))
        .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
            "status": 401,
            "error": "Invalid key"
        })))
        .expect(1)
        .mount(&server)
        .await;

    common::loco_cmd(&server.uri(), "bad-key")
        .args(["auth", "verify"])
        .assert()
        .failure()
        .code(2) // EXIT_AUTH
        .stderr(predicate::str::contains("Authentication failed"));
}

#[tokio::test]
async fn auth_verify_json_output() {
    let server = common::start_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/auth/verify"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "user": { "id": 1, "name": "Alice", "email": "alice@example.com" },
            "project": { "id": 42, "name": "My Project", "url": "https://localise.biz/projects/42" }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let output = common::loco_cmd(&server.uri(), TEST_KEY)
        .args(["--json", "auth", "verify"])
        .assert()
        .success();

    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
    let val: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON output");
    assert_eq!(val["user"]["name"], "Alice");
    assert_eq!(val["project"]["name"], "My Project");
}
