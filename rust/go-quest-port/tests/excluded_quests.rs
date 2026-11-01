//! Ensure partial/excluded Go records are retained without leaking targeting data.
use go_quest_port::parse_document;
use serde_json::{json, Value};
use std::{path::PathBuf, process::Command};

const SAMPLE: &str = include_str!("fixtures/excluded_demo.json");

#[test]
fn wrapped_excluded_records_are_preserved_separately() {
    let parsed = parse_document(SAMPLE).unwrap();
    assert_eq!(parsed.quests.len(), 1);
    assert_eq!(parsed.quests[0].id, "full-demo");
    assert_eq!(parsed.excluded_quests.len(), 1);
    let excluded = &parsed.excluded_quests[0];
    assert_eq!(excluded.id, "partial-demo");
    assert!(excluded.preview);
    assert_eq!(excluded.config.as_ref().unwrap().id, "partial-demo");
    assert_eq!(
        excluded.traffic_metadata_raw.as_deref(),
        Some("HIDDEN_TRAFFIC_SECRET")
    );
    assert_eq!(
        excluded.targeted_content.as_ref().unwrap()["restricted"],
        "HIDDEN_TARGETING_SECRET"
    );
}

#[test]
fn malformed_or_conflicting_excluded_records_fail_closed() {
    for invalid in [
        json!({"quests":[],"excluded_quests":{}}),
        json!({"quests":[],"excluded_quests":[{}]}),
        json!({"quests":[],"excluded_quests":[123]}),
        json!({"quests":[],"excluded_quests":[{"id":"same","preview":false},{"id":"same","preview":true}]}),
    ] {
        assert!(parse_document(&invalid.to_string()).is_err(), "{invalid}");
    }
}

#[test]
fn identical_excluded_ids_deduplicate() {
    let record = json!({"id":"same","preview":true});
    let input = json!({"quests":[],"excluded_quests":[record.clone(),record]});
    let parsed = parse_document(&input.to_string()).unwrap();
    assert_eq!(parsed.excluded_quests.len(), 1);
}

#[test]
fn cli_exposes_only_safe_excluded_metadata() {
    let input_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/excluded_demo.json");
    let binary = env!("CARGO_BIN_EXE_go-quest-port");
    let output = Command::new(binary)
        .arg("inspect")
        .arg(&input_path)
        .args(["--json", "--at", "2026-11-01T12:00:00Z"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let text = String::from_utf8(output.stdout).unwrap();
    let report: Value = serde_json::from_str(&text).unwrap();
    assert_eq!(report["analyzed"], 1);
    assert_eq!(report["excluded_count"], 1);
    assert_eq!(report["excluded_quests"][0]["id"], "partial-demo");
    assert_eq!(report["excluded_quests"][0]["preview"], true);
    for secret in [
        "HIDDEN_TARGETING_SECRET",
        "HIDDEN_TRAFFIC_SECRET",
        "HIDDEN_SEALED_SECRET",
    ] {
        assert!(
            !text.contains(secret),
            "Sensitive targeting metadata leaked"
        );
    }

    let text_output = Command::new(binary)
        .arg("inspect")
        .arg(&input_path)
        .args(["--at", "2026-11-01T12:00:00Z"])
        .output()
        .unwrap();
    assert!(text_output.status.success());
    let displayed = String::from_utf8(text_output.stdout).unwrap();
    assert!(displayed.contains("partial-demo"));
    for secret in [
        "HIDDEN_TARGETING_SECRET",
        "HIDDEN_TRAFFIC_SECRET",
        "HIDDEN_SEALED_SECRET",
    ] {
        assert!(
            !displayed.contains(secret),
            "Sensitive targeting metadata leaked"
        );
    }
}
