use super::common;
use predicates::prelude::*;
use std::time::Duration;

/// Cleanup helper - delete locale, ignore failures.
fn cleanup_locale(key: &str, code: &str) {
    let _ = common::e2e_cmd(key).args(["locales", "delete", code, "--force"]).ok();
}

#[tokio::test]
#[serial_test::serial]
async fn e2e_locales_list() {
    let key = common::skip_without_key!();
    common::e2e_cmd(&key)
        .args(["locales", "list"])
        .assert()
        .success()
        .stderr(predicate::str::contains("locale(s)"));
}

#[tokio::test]
#[serial_test::serial]
async fn e2e_locale_lifecycle() {
    let key = common::skip_without_key!();
    let locale_code = "vo"; // Volapuk - rare, unlikely to exist

    // Clean up in case a previous run left it behind
    cleanup_locale(&key, locale_code);
    std::thread::sleep(Duration::from_secs(1));

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        // 1. Create locale
        common::e2e_cmd(&key)
            .args(["locales", "create", locale_code])
            .assert()
            .success()
            .stderr(predicate::str::contains("Created locale"));

        // 2. Get locale
        common::e2e_cmd(&key)
            .args(["locales", "get", locale_code])
            .assert()
            .success()
            .stderr(predicate::str::contains(&format!("Code: {locale_code}")));

        // 3. List locales (JSON), verify it appears
        let output = common::e2e_cmd(&key)
            .args(["--json", "locales", "list"])
            .assert()
            .success();

        let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
        let val: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON");
        let arr = val.as_array().expect("should be array");
        assert!(
            arr.iter().any(|l| l["code"].as_str() == Some(locale_code)),
            "created locale should appear in list"
        );

        // 4. Delete locale
        common::e2e_cmd(&key)
            .args(["locales", "delete", locale_code, "--force"])
            .assert()
            .success()
            .stderr(predicate::str::contains("Deleted locale"));

        // Small delay for API consistency
        std::thread::sleep(Duration::from_secs(1));

        // 5. Verify deletion via list
        let output = common::e2e_cmd(&key)
            .args(["--json", "locales", "list"])
            .assert()
            .success();

        let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
        let val: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON");
        let arr = val.as_array().expect("should be array");
        assert!(
            !arr.iter()
                .any(|l| l["code"].as_str() == Some(locale_code)),
            "deleted locale should not appear in list"
        );
    }));

    // Always clean up
    if result.is_err() {
        cleanup_locale(&key, locale_code);
        std::panic::resume_unwind(result.unwrap_err());
    }
}
