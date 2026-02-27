mod common;

use predicates::prelude::*;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

const TEST_KEY: &str = "test-key-strings";

fn asset_json(id: &str, asset_type: &str) -> serde_json::Value {
    json!({
        "id": id,
        "type": asset_type,
        "context": "",
        "notes": "",
        "tags": [],
        "progress": { "num-translated": 1, "num-locales": 2 }
    })
}

fn translation_json(locale: &str, text: &str, translated: bool) -> serde_json::Value {
    json!({
        "id": locale,
        "type": "text",
        "translated": translated,
        "flagged": false,
        "status": "translated",
        "translation": text,
        "revision": 1,
        "modified": "2025-01-01T00:00:00Z"
    })
}

// 1. strings list — human output
#[tokio::test]
async fn strings_list() {
    let server = common::start_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/assets"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            asset_json("greeting", "text"),
            asset_json("farewell", "text"),
        ])))
        .expect(1)
        .mount(&server)
        .await;

    common::loco_cmd(&server.uri(), TEST_KEY)
        .args(["strings", "list"])
        .assert()
        .success()
        .stderr(predicate::str::contains("2 string(s)"))
        .stderr(predicate::str::contains("greeting"))
        .stderr(predicate::str::contains("farewell"));
}

// 2. strings list --json
#[tokio::test]
async fn strings_list_json() {
    let server = common::start_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/assets"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            asset_json("greeting", "text"),
            asset_json("farewell", "text"),
        ])))
        .expect(1)
        .mount(&server)
        .await;

    let output = common::loco_cmd(&server.uri(), TEST_KEY)
        .args(["--json", "strings", "list"])
        .assert()
        .success();

    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
    let val: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON");
    assert!(val.is_array());
    assert_eq!(val.as_array().unwrap().len(), 2);
    assert_eq!(val[0]["id"], "greeting");
    assert_eq!(val[1]["id"], "farewell");
}

// 3. strings get <id> — get with all translations
#[tokio::test]
async fn strings_get() {
    let server = common::start_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/assets/mykey.json"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "mykey",
            "type": "text",
            "context": "home screen",
            "notes": "main greeting",
            "tags": ["ui"],
            "progress": { "num-translated": 1, "num-locales": 2 }
        })))
        .expect(1)
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/translations/mykey.json"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            translation_json("en", "Hello", true),
        ])))
        .expect(1)
        .mount(&server)
        .await;

    common::loco_cmd(&server.uri(), TEST_KEY)
        .args(["strings", "get", "mykey"])
        .assert()
        .success()
        .stderr(predicate::str::contains("ID: mykey"))
        .stderr(predicate::str::contains("Type: text"))
        .stderr(predicate::str::contains("Context: home screen"))
        .stderr(predicate::str::contains("Notes: main greeting"))
        .stderr(predicate::str::contains("Tags: ui"))
        .stderr(predicate::str::contains("en"));
}

// 4. strings add
#[tokio::test]
async fn strings_add() {
    let server = common::start_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/assets"))
        .respond_with(ResponseTemplate::new(200).set_body_json(asset_json("newkey", "text")))
        .expect(1)
        .mount(&server)
        .await;

    common::loco_cmd(&server.uri(), TEST_KEY)
        .args(["strings", "add", "newkey"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Created string: newkey"));
}

// 5. strings add with inline translations
#[tokio::test]
async fn strings_add_with_translations() {
    let server = common::start_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/assets"))
        .respond_with(ResponseTemplate::new(200).set_body_json(asset_json("welcome", "text")))
        .expect(1)
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/translations/welcome/en"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(translation_json("en", "Hello", true)),
        )
        .expect(1)
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/translations/welcome/de"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(translation_json("de", "Hallo", true)),
        )
        .expect(1)
        .mount(&server)
        .await;

    common::loco_cmd(&server.uri(), TEST_KEY)
        .args(["strings", "add", "welcome", "en=Hello", "de=Hallo"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Created string: welcome"))
        .stderr(predicate::str::contains("en: Hello"))
        .stderr(predicate::str::contains("de: Hallo"));
}

// 6. strings delete
#[tokio::test]
async fn strings_delete() {
    let server = common::start_mock_server().await;

    Mock::given(method("DELETE"))
        .and(path("/assets/mykey.json"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"status": 200})))
        .expect(1)
        .mount(&server)
        .await;

    common::loco_cmd(&server.uri(), TEST_KEY)
        .args(["strings", "delete", "mykey", "--force"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Deleted string: mykey"));
}

// 7. strings tag
#[tokio::test]
async fn strings_tag() {
    let server = common::start_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/assets/mykey/tags"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"status": 200})))
        .expect(1)
        .mount(&server)
        .await;

    common::loco_cmd(&server.uri(), TEST_KEY)
        .args(["strings", "tag", "mykey", "mytag"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Tagged mykey"));
}

// 8. strings untag
#[tokio::test]
async fn strings_untag() {
    let server = common::start_mock_server().await;

    Mock::given(method("DELETE"))
        .and(path("/assets/mykey/tags/mytag.json"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"status": 200})))
        .expect(1)
        .mount(&server)
        .await;

    common::loco_cmd(&server.uri(), TEST_KEY)
        .args(["strings", "untag", "mykey", "mytag"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Removed tag"));
}

// 9. strings set
#[tokio::test]
async fn strings_set() {
    let server = common::start_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/translations/mykey/en"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(translation_json("en", "Hello world", true)),
        )
        .expect(1)
        .mount(&server)
        .await;

    common::loco_cmd(&server.uri(), TEST_KEY)
        .args(["strings", "set", "mykey", "en=Hello world"])
        .assert()
        .success()
        .stderr(predicate::str::contains("mykey/en: Hello world"));
}

// 10. strings set --create (asset already exists -> 409, then translation succeeds)
#[tokio::test]
async fn strings_set_with_create() {
    let server = common::start_mock_server().await;

    // POST /assets returns 409 (asset already exists)
    Mock::given(method("POST"))
        .and(path("/assets"))
        .respond_with(ResponseTemplate::new(409).set_body_json(json!({
            "status": 409,
            "error": "Asset already exists"
        })))
        .expect(1)
        .mount(&server)
        .await;

    // POST /translations succeeds
    Mock::given(method("POST"))
        .and(path("/translations/mykey/en"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(translation_json("en", "Updated", true)),
        )
        .expect(1)
        .mount(&server)
        .await;

    common::loco_cmd(&server.uri(), TEST_KEY)
        .args(["strings", "set", "mykey", "en=Updated", "--create"])
        .assert()
        .success()
        .stderr(predicate::str::contains("mykey/en: Updated"));
}
