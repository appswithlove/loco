use super::common;
use predicates::prelude::*;
use std::time::Duration;

/// Cleanup helper - delete tag, ignore failures.
fn cleanup_tag(key: &str, name: &str) {
    let _ = common::e2e_cmd(key).args(["tags", "delete", name, "--force"]).ok();
}

#[tokio::test]
#[serial_test::serial]
async fn e2e_tag_lifecycle() {
    let key = common::skip_without_key!();
    let tag_name = common::unique_id("tag");
    let renamed = format!("{tag_name}_renamed");

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        // 1. Create tag
        common::e2e_cmd(&key)
            .args(["tags", "create", &tag_name])
            .assert()
            .success()
            .stderr(predicate::str::contains("Created tag"));

        // 2. List tags (JSON), verify it appears
        let output = common::e2e_cmd(&key)
            .args(["--json", "tags", "list"])
            .assert()
            .success();

        let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
        let val: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON");
        let arr = val.as_array().expect("should be array");
        assert!(
            arr.iter().any(|t| t.as_str() == Some(&tag_name)),
            "created tag should appear in list"
        );

        // 3. Rename tag
        common::e2e_cmd(&key)
            .args(["tags", "rename", &tag_name, &renamed])
            .assert()
            .success()
            .stderr(predicate::str::contains("Renamed tag"));

        // Small delay for API consistency
        std::thread::sleep(Duration::from_secs(1));

        // 4. List tags (JSON), verify new name appears
        let output = common::e2e_cmd(&key)
            .args(["--json", "tags", "list"])
            .assert()
            .success();

        let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
        let val: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON");
        let arr = val.as_array().expect("should be array");
        assert!(
            arr.iter().any(|t| t.as_str() == Some(renamed.as_str())),
            "renamed tag should appear in list"
        );

        // 5. Delete tag
        common::e2e_cmd(&key)
            .args(["tags", "delete", &renamed, "--force"])
            .assert()
            .success()
            .stderr(predicate::str::contains("Deleted tag"));
    }));

    // Always clean up both possible names
    if result.is_err() {
        cleanup_tag(&key, &tag_name);
        cleanup_tag(&key, &renamed);
        std::panic::resume_unwind(result.unwrap_err());
    }
}
