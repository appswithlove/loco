use super::common;
use predicates::prelude::*;

/// Cleanup helper - delete string, ignore failures.
fn cleanup_string(key: &str, id: &str) {
    let _ = common::e2e_cmd(key)
        .args(["strings", "delete", id, "--force"])
        .ok();
}

#[tokio::test]
#[serial_test::serial]
async fn e2e_string_lifecycle() {
    let key = common::skip_without_key!();
    let id = common::unique_id("str");

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        // 1. Add string
        common::e2e_cmd(&key)
            .args(["strings", "add", &id])
            .assert()
            .success()
            .stderr(predicate::str::contains("Created string"));

        // 2. Get string, verify it exists
        common::e2e_cmd(&key)
            .args(["strings", "get", &id])
            .assert()
            .success()
            .stderr(predicate::str::contains(&format!("ID: {id}")));

        // 3. List strings (JSON), verify it appears
        let output = common::e2e_cmd(&key)
            .args(["--json", "strings", "list"])
            .assert()
            .success();

        let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
        let val: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON");
        let arr = val.as_array().expect("should be array");
        assert!(
            arr.iter().any(|a| a["id"].as_str() == Some(&id)),
            "created string should appear in list"
        );

        // 4. Tag string
        let tag_name = common::unique_id("tag");
        common::e2e_cmd(&key)
            .args(["strings", "tag", &id, &tag_name])
            .assert()
            .success()
            .stderr(predicate::str::contains("Tagged"));

        // 5. Set translation
        common::e2e_cmd(&key)
            .args(["strings", "set", &id, "en=hello e2e"])
            .assert()
            .success()
            .stderr(predicate::str::contains(&format!("{id}/en")));

        // 6. Verify translation was set via get (JSON)
        let output = common::e2e_cmd(&key)
            .args(["--json", "strings", "get", &id])
            .assert()
            .success();

        let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
        let val: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON");
        let translations = val["translations"].as_array().expect("translations array");
        assert!(
            translations
                .iter()
                .any(|t| t["text"].as_str() == Some("hello e2e")),
            "translation with text 'hello e2e' should appear"
        );

        // 7. Untag string
        common::e2e_cmd(&key)
            .args(["strings", "untag", &id, &tag_name])
            .assert()
            .success()
            .stderr(predicate::str::contains("Removed tag"));

        // 8. Delete string
        common::e2e_cmd(&key)
            .args(["strings", "delete", &id, "--force"])
            .assert()
            .success()
            .stderr(predicate::str::contains("Deleted string"));
    }));

    if result.is_err() {
        cleanup_string(&key, &id);
        std::panic::resume_unwind(result.unwrap_err());
    }
}

#[tokio::test]
#[serial_test::serial]
async fn e2e_string_add_with_translations() {
    let key = common::skip_without_key!();
    let id = common::unique_id("str");

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        // Add string with inline translations
        common::e2e_cmd(&key)
            .args(["strings", "add", &id, "en=hello from add"])
            .assert()
            .success();

        // Verify via JSON get
        let output = common::e2e_cmd(&key)
            .args(["--json", "strings", "get", &id])
            .assert()
            .success();

        let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
        let val: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON");
        assert_eq!(val["asset"]["id"].as_str(), Some(id.as_str()));
    }));

    cleanup_string(&key, &id);
    if let Err(e) = result {
        std::panic::resume_unwind(e);
    }
}

#[tokio::test]
#[serial_test::serial]
async fn e2e_string_set_with_create_flag() {
    let key = common::skip_without_key!();
    let id = common::unique_id("str");

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        // Use `strings set --create` which creates the string if missing
        common::e2e_cmd(&key)
            .args(["strings", "set", &id, "en=auto-created", "--create"])
            .assert()
            .success();

        // Verify string exists
        common::e2e_cmd(&key)
            .args(["strings", "get", &id])
            .assert()
            .success()
            .stderr(predicate::str::contains(&format!("ID: {id}")));
    }));

    cleanup_string(&key, &id);
    if let Err(e) = result {
        std::panic::resume_unwind(e);
    }
}

#[tokio::test]
#[serial_test::serial]
async fn e2e_translation_lifecycle() {
    let key = common::skip_without_key!();
    let id = common::unique_id("str");

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        // 1. Add string
        common::e2e_cmd(&key)
            .args(["strings", "add", &id])
            .assert()
            .success();

        // 2. Set translation
        common::e2e_cmd(&key)
            .args(["strings", "set", &id, "de-CH=Hallo E2E"])
            .assert()
            .success()
            .stderr(predicate::str::contains(&format!("{id}/de-CH")));

        // 3. Get single translation, verify text
        let output = common::e2e_cmd(&key)
            .args(["--json", "strings", "get", &id, "de-CH"])
            .assert()
            .success();

        let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
        let val: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON");
        assert_eq!(val["translation"].as_str(), Some("Hallo E2E"));
        assert_eq!(val["translated"], true);

        // 4. Flag translation
        common::e2e_cmd(&key)
            .args(["strings", "flag", &id, "de-CH"])
            .assert()
            .success()
            .stderr(predicate::str::contains(&format!("Flagged {id}/de-CH")));

        std::thread::sleep(std::time::Duration::from_secs(1));

        // 5. Verify flagged
        let output = common::e2e_cmd(&key)
            .args(["--json", "strings", "get", &id, "de-CH"])
            .assert()
            .success();

        let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
        let val: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON");
        assert!(
            !val["flagged"].is_null(),
            "translation should be flagged, got: {}",
            val["flagged"]
        );

        // 6. Unflag translation
        common::e2e_cmd(&key)
            .args(["strings", "unflag", &id, "de-CH"])
            .assert()
            .success()
            .stderr(predicate::str::contains(&format!("Unflagged {id}/de-CH")));

        std::thread::sleep(std::time::Duration::from_secs(1));

        // 7. Verify unflagged
        let output = common::e2e_cmd(&key)
            .args(["--json", "strings", "get", &id, "de-CH"])
            .assert()
            .success();

        let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
        let val: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON");
        assert!(
            val["flagged"].is_null(),
            "translation should not be flagged, got: {}",
            val["flagged"]
        );

        // 8. Remove translation
        common::e2e_cmd(&key)
            .args(["strings", "rm", &id, "de-CH"])
            .assert()
            .success()
            .stderr(predicate::str::contains(&format!(
                "Removed translation {id}/de-CH"
            )));
    }));

    cleanup_string(&key, &id);
    if let Err(e) = result {
        std::panic::resume_unwind(e);
    }
}
