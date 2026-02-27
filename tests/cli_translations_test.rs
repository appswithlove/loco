mod common;

use predicates::prelude::*;
use wiremock::matchers::{body_string, header, method, path};
use wiremock::{Mock, ResponseTemplate};

const TEST_KEY: &str = "test-key-translations";

fn translation_json(locale: &str, text: &str, translated: bool) -> serde_json::Value {
    serde_json::json!({
        "id": locale,
        "type": "text",
        "translated": translated,
        "flagged": false,
        "status": "translated",
        "translation": text,
        "revision": 1,
        "modified": "2026-01-15 12:00:00"
    })
}

#[tokio::test]
async fn strings_get_locale() {
    let server = common::start_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/translations/welcome.title/en"))
        .and(header("Authorization", &format!("Loco {TEST_KEY}")))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(translation_json("en", "Welcome", true)),
        )
        .expect(1)
        .mount(&server)
        .await;

    common::loco_cmd(&server.uri(), TEST_KEY)
        .args(["strings", "get", "welcome.title", "en"])
        .assert()
        .success()
        .stderr(predicate::str::contains("welcome.title"))
        .stderr(predicate::str::contains("en"))
        .stderr(predicate::str::contains("Welcome"));
}

#[tokio::test]
async fn strings_get_locale_json() {
    let server = common::start_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/translations/welcome.title/en"))
        .and(header("Authorization", &format!("Loco {TEST_KEY}")))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(translation_json("en", "Welcome", true)),
        )
        .expect(1)
        .mount(&server)
        .await;

    let output = common::loco_cmd(&server.uri(), TEST_KEY)
        .args(["--json", "strings", "get", "welcome.title", "en"])
        .assert()
        .success();

    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
    let val: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON");
    assert_eq!(val["id"], "en");
    assert_eq!(val["translation"], "Welcome");
    assert_eq!(val["translated"], true);
    assert_eq!(val["revision"], 1);
}

#[tokio::test]
async fn strings_set_translation() {
    let server = common::start_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/translations/welcome.title/fr"))
        .and(header("Authorization", &format!("Loco {TEST_KEY}")))
        .and(body_string("Bienvenue"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(translation_json("fr", "Bienvenue", true)),
        )
        .expect(1)
        .mount(&server)
        .await;

    common::loco_cmd(&server.uri(), TEST_KEY)
        .args(["strings", "set", "welcome.title", "fr=Bienvenue"])
        .assert()
        .success()
        .stderr(predicate::str::contains("welcome.title/fr: Bienvenue"));
}

#[tokio::test]
async fn strings_set_translation_json() {
    let server = common::start_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/translations/welcome.title/fr"))
        .and(header("Authorization", &format!("Loco {TEST_KEY}")))
        .and(body_string("Bienvenue"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(translation_json("fr", "Bienvenue", true)),
        )
        .expect(1)
        .mount(&server)
        .await;

    let output = common::loco_cmd(&server.uri(), TEST_KEY)
        .args(["--json", "strings", "set", "welcome.title", "fr=Bienvenue"])
        .assert()
        .success();

    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
    let val: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON");
    assert!(val.is_array());
    assert_eq!(val[0]["id"], "fr");
    assert_eq!(val[0]["translation"], "Bienvenue");
    assert_eq!(val[0]["translated"], true);
}

#[tokio::test]
async fn strings_rm() {
    let server = common::start_mock_server().await;

    Mock::given(method("DELETE"))
        .and(path("/translations/welcome.title/fr"))
        .and(header("Authorization", &format!("Loco {TEST_KEY}")))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({"status": 200})),
        )
        .expect(1)
        .mount(&server)
        .await;

    common::loco_cmd(&server.uri(), TEST_KEY)
        .args(["strings", "rm", "welcome.title", "fr"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Removed translation welcome.title/fr"));
}

#[tokio::test]
async fn strings_flag() {
    let server = common::start_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/translations/welcome.title/en/flag"))
        .and(header("Authorization", &format!("Loco {TEST_KEY}")))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({"status": 200})),
        )
        .expect(1)
        .mount(&server)
        .await;

    common::loco_cmd(&server.uri(), TEST_KEY)
        .args(["strings", "flag", "welcome.title", "en"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Flagged welcome.title/en"));
}

#[tokio::test]
async fn strings_flag_with_value() {
    let server = common::start_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/translations/welcome.title/en/flag"))
        .and(header("Authorization", &format!("Loco {TEST_KEY}")))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({"status": 200})),
        )
        .expect(1)
        .mount(&server)
        .await;

    common::loco_cmd(&server.uri(), TEST_KEY)
        .args([
            "strings",
            "flag",
            "welcome.title",
            "en",
            "--flag",
            "needs-review",
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains("Flagged welcome.title/en"));
}

#[tokio::test]
async fn strings_unflag() {
    let server = common::start_mock_server().await;

    Mock::given(method("DELETE"))
        .and(path("/translations/welcome.title/en/flag"))
        .and(header("Authorization", &format!("Loco {TEST_KEY}")))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({"status": 200})),
        )
        .expect(1)
        .mount(&server)
        .await;

    common::loco_cmd(&server.uri(), TEST_KEY)
        .args(["strings", "unflag", "welcome.title", "en"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Unflagged welcome.title/en"));
}
