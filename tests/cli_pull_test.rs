mod common;

use predicates::prelude::*;
use tempfile::TempDir;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, ResponseTemplate};

const TEST_KEY: &str = "pull-test-key";

#[tokio::test]
async fn pull_single_locale() {
    let server = common::start_mock_server().await;
    let dir = TempDir::new().unwrap();

    let translations = serde_json::json!({ "hello": "Hello", "bye": "Goodbye" });

    Mock::given(method("GET"))
        .and(path("/export/locale/en.json"))
        .and(header("Authorization", &format!("Loco {TEST_KEY}")))
        .respond_with(ResponseTemplate::new(200).set_body_json(&translations))
        .expect(1)
        .mount(&server)
        .await;

    let out_path = dir.path().join("{locale}.{format}");

    common::loco_cmd(&server.uri(), TEST_KEY)
        .args([
            "pull",
            "--locale",
            "en",
            "--path",
            out_path.to_str().unwrap(),
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains("Exported"));

    let written = std::fs::read_to_string(dir.path().join("en.json")).unwrap();
    let val: serde_json::Value = serde_json::from_str(&written).unwrap();
    assert_eq!(val["hello"], "Hello");
}

#[tokio::test]
async fn pull_all_locales() {
    let server = common::start_mock_server().await;
    let dir = TempDir::new().unwrap();

    // Mock /locales
    Mock::given(method("GET"))
        .and(path("/locales"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            { "code": "en", "name": "English" },
            { "code": "fr", "name": "French" }
        ])))
        .expect(1)
        .mount(&server)
        .await;

    // Mock exports
    Mock::given(method("GET"))
        .and(path("/export/locale/en.json"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(serde_json::json!({ "hello": "Hello" })),
        )
        .expect(1)
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/export/locale/fr.json"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(serde_json::json!({ "hello": "Bonjour" })),
        )
        .expect(1)
        .mount(&server)
        .await;

    let out_path = dir.path().join("{locale}.{format}");

    common::loco_cmd(&server.uri(), TEST_KEY)
        .args(["pull", "--path", out_path.to_str().unwrap()])
        .assert()
        .success()
        .stderr(predicate::str::contains("Exported 2 locale(s)"));

    assert!(dir.path().join("en.json").exists());
    assert!(dir.path().join("fr.json").exists());

    let fr: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(dir.path().join("fr.json")).unwrap())
            .unwrap();
    assert_eq!(fr["hello"], "Bonjour");
}

#[tokio::test]
async fn pull_custom_path_template() {
    let server = common::start_mock_server().await;
    let dir = TempDir::new().unwrap();

    Mock::given(method("GET"))
        .and(path("/export/locale/de.json"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(serde_json::json!({ "hello": "Hallo" })),
        )
        .expect(1)
        .mount(&server)
        .await;

    let out_path = dir.path().join("translations/{locale}/strings.{format}");

    common::loco_cmd(&server.uri(), TEST_KEY)
        .args([
            "pull",
            "--locale",
            "de",
            "--path",
            out_path.to_str().unwrap(),
        ])
        .assert()
        .success();

    let expected = dir.path().join("translations/de/strings.json");
    assert!(expected.exists(), "file should be at custom path");
}

#[tokio::test]
async fn pull_unicode_content() {
    let server = common::start_mock_server().await;
    let dir = TempDir::new().unwrap();

    let translations = serde_json::json!({
        "greeting": "こんにちは",
        "farewell": "مع السلامة",
        "emoji": "Hello 👋🌍",
        "accented": "Ärger mit Ü und ß",
        "chinese": "你好世界"
    });

    Mock::given(method("GET"))
        .and(path("/export/locale/ja.json"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&translations))
        .expect(1)
        .mount(&server)
        .await;

    let out_path = dir.path().join("{locale}.{format}");

    common::loco_cmd(&server.uri(), TEST_KEY)
        .args([
            "pull",
            "--locale",
            "ja",
            "--path",
            out_path.to_str().unwrap(),
        ])
        .assert()
        .success();

    let written = std::fs::read_to_string(dir.path().join("ja.json")).unwrap();
    let val: serde_json::Value = serde_json::from_str(&written).unwrap();
    assert_eq!(val["greeting"], "こんにちは");
    assert_eq!(val["farewell"], "مع السلامة");
    assert_eq!(val["emoji"], "Hello 👋🌍");
    assert_eq!(val["accented"], "Ärger mit Ü und ß");
    assert_eq!(val["chinese"], "你好世界");
}

#[tokio::test]
async fn pull_missing_api_key() {
    let mut cmd = assert_cmd::Command::cargo_bin("loco-cli").expect("binary exists");
    cmd.env("LOCO_API_URL", "http://127.0.0.1:1")
        .env_remove("LOCO_API_KEY")
        .args([
            "--config",
            "/nonexistent/.loco.toml",
            "pull",
            "--locale",
            "en",
        ])
        .assert()
        .failure()
        .code(3) // EXIT_CONFIG
        .stderr(predicate::str::contains("API key"));
}
