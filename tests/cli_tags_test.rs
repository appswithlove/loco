mod common;

use predicates::prelude::*;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

const TEST_KEY: &str = "test-key-tags";

// 1. tags list — human output
#[tokio::test]
async fn tags_list() {
    let server = common::start_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/tags"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(serde_json::json!(["v1", "v2", "v3"])),
        )
        .expect(1)
        .mount(&server)
        .await;

    common::loco_cmd(&server.uri(), TEST_KEY)
        .args(["tags", "list"])
        .assert()
        .success()
        .stderr(predicate::str::contains("3 tag(s)"))
        .stderr(predicate::str::contains("v1"))
        .stderr(predicate::str::contains("v2"))
        .stderr(predicate::str::contains("v3"));
}

// 2. tags list --json
#[tokio::test]
async fn tags_list_json() {
    let server = common::start_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/tags"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(serde_json::json!(["v1", "v2", "v3"])),
        )
        .expect(1)
        .mount(&server)
        .await;

    let output = common::loco_cmd(&server.uri(), TEST_KEY)
        .args(["--json", "tags", "list"])
        .assert()
        .success();

    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
    let val: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON");
    assert!(val.is_array());
    let arr = val.as_array().unwrap();
    assert_eq!(arr.len(), 3);
    assert_eq!(arr[0], "v1");
    assert_eq!(arr[1], "v2");
    assert_eq!(arr[2], "v3");
}

// 3. tags create
#[tokio::test]
async fn tags_create() {
    let server = common::start_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/tags"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&server)
        .await;

    common::loco_cmd(&server.uri(), TEST_KEY)
        .args(["tags", "create", "release"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Created tag"));
}

// 4. tags rename
#[tokio::test]
async fn tags_rename() {
    let server = common::start_mock_server().await;

    Mock::given(method("PATCH"))
        .and(path("/tags/old-name.json"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&server)
        .await;

    common::loco_cmd(&server.uri(), TEST_KEY)
        .args(["tags", "rename", "old-name", "new-name"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Renamed tag"));
}

// 5. tags delete
#[tokio::test]
async fn tags_delete() {
    let server = common::start_mock_server().await;

    Mock::given(method("DELETE"))
        .and(path("/tags/obsolete.json"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&server)
        .await;

    common::loco_cmd(&server.uri(), TEST_KEY)
        .args(["tags", "delete", "obsolete", "--force"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Deleted tag"));
}
