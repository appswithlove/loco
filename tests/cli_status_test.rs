mod common;

use predicates::prelude::*;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

const TEST_KEY: &str = "status-test-key";

fn locale_json(code: &str, name: &str, translated: u32, untranslated: u32) -> serde_json::Value {
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
async fn status_all_locales() {
    let server = common::start_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/locales"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            locale_json("en", "English", 100, 0),
            locale_json("fr", "French", 80, 20),
        ])))
        .expect(1)
        .mount(&server)
        .await;

    common::loco_cmd(&server.uri(), TEST_KEY)
        .args(["status"])
        .assert()
        .success()
        .stderr(predicate::str::contains("2 locale(s)"))
        .stderr(predicate::str::contains("en"))
        .stderr(predicate::str::contains("fr"));
}

#[tokio::test]
async fn status_single_locale() {
    let server = common::start_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/locales/fr"))
        .respond_with(ResponseTemplate::new(200).set_body_json(locale_json("fr", "French", 80, 20)))
        .expect(1)
        .mount(&server)
        .await;

    common::loco_cmd(&server.uri(), TEST_KEY)
        .args(["status", "--locale", "fr"])
        .assert()
        .success()
        .stderr(predicate::str::contains("fr"))
        .stderr(predicate::str::contains("80"));
}

#[tokio::test]
async fn status_json_output() {
    let server = common::start_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/locales"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!([locale_json("en", "English", 50, 50),])),
        )
        .expect(1)
        .mount(&server)
        .await;

    let output = common::loco_cmd(&server.uri(), TEST_KEY)
        .args(["--json", "status"])
        .assert()
        .success();

    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
    let val: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON");
    assert!(val.is_array());
    assert_eq!(val[0]["code"], "en");
}
