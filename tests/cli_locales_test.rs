mod common;

use predicates::prelude::*;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

const TEST_KEY: &str = "test-key-locales";

fn locale_json(code: &str, name: &str) -> serde_json::Value {
    serde_json::json!({
        "code": code,
        "name": name,
    })
}

fn locale_with_progress(
    code: &str,
    name: &str,
    translated: u32,
    untranslated: u32,
) -> serde_json::Value {
    serde_json::json!({
        "code": code,
        "name": name,
        "progress": {
            "num-translated": translated,
            "num-untranslated": untranslated,
            "num-approved": 0,
            "num-pending": 0,
            "translated": translated,
            "untranslated": untranslated,
            "flagged": 0
        }
    })
}

#[tokio::test]
async fn locales_list() {
    let server = common::start_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/locales"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            locale_json("en", "English"),
            locale_json("fr", "French"),
            locale_json("de", "German"),
        ])))
        .expect(1)
        .mount(&server)
        .await;

    common::loco_cmd(&server.uri(), TEST_KEY)
        .args(["locales", "list"])
        .assert()
        .success()
        .stderr(predicate::str::contains("3 locale(s)"))
        .stderr(predicate::str::contains("en"))
        .stderr(predicate::str::contains("fr"))
        .stderr(predicate::str::contains("de"));
}

#[tokio::test]
async fn locales_list_json() {
    let server = common::start_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/locales"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            locale_json("en", "English"),
            locale_json("fr", "French"),
            locale_json("de", "German"),
        ])))
        .expect(1)
        .mount(&server)
        .await;

    let output = common::loco_cmd(&server.uri(), TEST_KEY)
        .args(["--json", "locales", "list"])
        .assert()
        .success();

    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
    let val: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON");
    assert!(val.is_array());
    let arr = val.as_array().unwrap();
    assert_eq!(arr.len(), 3);
    assert_eq!(arr[0]["code"], "en");
    assert_eq!(arr[1]["code"], "fr");
    assert_eq!(arr[2]["code"], "de");
}

#[tokio::test]
async fn locales_get() {
    let server = common::start_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/locales/fr"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(locale_with_progress("fr", "French", 80, 20)),
        )
        .expect(1)
        .mount(&server)
        .await;

    common::loco_cmd(&server.uri(), TEST_KEY)
        .args(["locales", "get", "fr"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Code: fr"))
        .stderr(predicate::str::contains("Translated: 80/100"));
}

#[tokio::test]
async fn locales_get_json() {
    let server = common::start_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/locales/fr"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(locale_with_progress("fr", "French", 80, 20)),
        )
        .expect(1)
        .mount(&server)
        .await;

    let output = common::loco_cmd(&server.uri(), TEST_KEY)
        .args(["--json", "locales", "get", "fr"])
        .assert()
        .success();

    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
    let val: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON");
    assert_eq!(val["code"], "fr");
    assert_eq!(val["progress"]["num-translated"], 80);
    assert_eq!(val["progress"]["num-untranslated"], 20);
}

#[tokio::test]
async fn locales_create() {
    let server = common::start_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/locales"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(locale_json("ja", "Japanese")),
        )
        .expect(1)
        .mount(&server)
        .await;

    common::loco_cmd(&server.uri(), TEST_KEY)
        .args(["locales", "create", "ja"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Created locale: ja"));
}

#[tokio::test]
async fn locales_delete() {
    let server = common::start_mock_server().await;

    Mock::given(method("DELETE"))
        .and(path("/locales/ja"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(serde_json::json!({"status": 200})),
        )
        .expect(1)
        .mount(&server)
        .await;

    common::loco_cmd(&server.uri(), TEST_KEY)
        .args(["locales", "delete", "ja", "--force"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Deleted locale: ja"));
}
