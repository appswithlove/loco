mod common;

use predicates::prelude::*;
use std::io::Write;
use tempfile::NamedTempFile;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, ResponseTemplate};

const TEST_KEY: &str = "push-test-key";

#[tokio::test]
async fn push_file_success() {
    let server = common::start_mock_server().await;

    // Create a temp JSON file to push
    let mut f = NamedTempFile::with_suffix(".json").unwrap();
    writeln!(f, r#"{{"hello":"world"}}"#).unwrap();

    Mock::given(method("POST"))
        .and(path("/import/json"))
        .and(header("Authorization", &format!("Loco {TEST_KEY}")))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "message": "1 translation imported",
            "status": 200
        })))
        .expect(1)
        .mount(&server)
        .await;

    common::loco_cmd(&server.uri(), TEST_KEY)
        .args(["push", "--file", f.path().to_str().unwrap()])
        .assert()
        .success()
        .stderr(predicate::str::contains("Import complete"));
}

#[tokio::test]
async fn push_unicode_content() {
    let server = common::start_mock_server().await;

    let mut f = NamedTempFile::with_suffix(".json").unwrap();
    write!(
        f,
        r#"{{"greeting":"こんにちは","emoji":"👋🌍","german":"Ärger mit ß"}}"#
    )
    .unwrap();

    Mock::given(method("POST"))
        .and(path("/import/json"))
        .and(header("Authorization", &format!("Loco {TEST_KEY}")))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "message": "3 translations imported",
            "status": 200
        })))
        .expect(1)
        .mount(&server)
        .await;

    common::loco_cmd(&server.uri(), TEST_KEY)
        .args(["push", "--file", f.path().to_str().unwrap()])
        .assert()
        .success()
        .stderr(predicate::str::contains("Import complete"));
}

#[tokio::test]
async fn push_file_not_found() {
    let server = common::start_mock_server().await;

    common::loco_cmd(&server.uri(), TEST_KEY)
        .args(["push", "--file", "/nonexistent/missing.json"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("File not found"));
}

#[tokio::test]
async fn push_with_async_flag() {
    let server = common::start_mock_server().await;

    let mut f = NamedTempFile::with_suffix(".json").unwrap();
    writeln!(f, r#"{{"key":"val"}}"#).unwrap();

    // Initial import returns job id
    Mock::given(method("POST"))
        .and(path("/import/json"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "message": "job-123",
            "status": 200
        })))
        .expect(1)
        .mount(&server)
        .await;

    // Progress poll returns 100% immediately
    Mock::given(method("GET"))
        .and(path("/import/progress/job-123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "progress": 100,
            "success": "Import complete"
        })))
        .expect(1)
        .mount(&server)
        .await;

    common::loco_cmd(&server.uri(), TEST_KEY)
        .args(["push", "--file", f.path().to_str().unwrap(), "--async"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Import"));
}
