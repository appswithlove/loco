use super::common;
use predicates::prelude::*;

/// Cleanup helper - delete asset, ignore failures.
fn cleanup_asset(key: &str, id: &str) {
    let _ = common::e2e_cmd(key).args(["strings", "delete", id, "--force"]).ok();
}

#[tokio::test]
#[serial_test::serial]
async fn e2e_pull_and_push() {
    let key = common::skip_without_key!();
    let id = common::unique_id("pull");
    let translation_text = "Pull push E2E test";

    let tmp_dir = tempfile::tempdir().expect("create temp dir");
    let tmp_path = tmp_dir.path();
    let path_template = tmp_path.join("{locale}.json");
    let path_template_str = path_template.to_str().unwrap();
    let expected_file = tmp_path.join("en.json");

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        // 1. Create asset and set translation
        common::e2e_cmd(&key)
            .args([
                "strings", "set", &id, &format!("en={translation_text}"),
                "--create",
            ])
            .assert()
            .success();

        // 2. Pull to temp dir
        common::e2e_cmd(&key)
            .args([
                "pull",
                "--format", "json",
                "--locale", "en",
                "--path", path_template_str,
            ])
            .assert()
            .success()
            .stderr(predicate::str::contains("Exported"));

        // 3. Verify file exists and contains the translation
        assert!(expected_file.exists(), "pulled file should exist at {:?}", expected_file);

        let content = std::fs::read_to_string(&expected_file).expect("read pulled file");
        let val: serde_json::Value = serde_json::from_str(&content).expect("valid JSON in pulled file");

        // The export format puts translations as key-value pairs
        assert!(
            val.get(&id).is_some(),
            "pulled JSON should contain key {id}, got: {content}"
        );
        assert_eq!(
            val[&id].as_str(),
            Some(translation_text),
            "translation text should match"
        );

        // 4. Push the file back
        let file_path = expected_file.to_str().unwrap();
        common::e2e_cmd(&key)
            .args(["push", "--file", file_path])
            .assert()
            .success()
            .stderr(predicate::str::contains("Import complete"));
    }));

    // Always clean up asset
    cleanup_asset(&key, &id);
    if let Err(e) = result {
        std::panic::resume_unwind(e);
    }
}

#[tokio::test]
#[serial_test::serial]
async fn e2e_pull_all_locales() {
    let key = common::skip_without_key!();

    let tmp_dir = tempfile::tempdir().expect("create temp dir");
    let path_template = tmp_dir.path().join("{locale}.json");
    let path_template_str = path_template.to_str().unwrap();

    // Pull all locales (no --locale flag)
    common::e2e_cmd(&key)
        .args([
            "pull",
            "--format", "json",
            "--path", path_template_str,
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains("Exported"));

    // At least one file should have been created
    let entries: Vec<_> = std::fs::read_dir(tmp_dir.path())
        .expect("read temp dir")
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "json"))
        .collect();
    assert!(
        !entries.is_empty(),
        "pull should have created at least one locale file"
    );
}
