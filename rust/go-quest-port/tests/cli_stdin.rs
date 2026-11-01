//! Pipeline tests with synthetic offline JSON. No Discord connection or token.
use serde_json::Value;
use std::{
    io::Write,
    process::{Command, Stdio},
};

fn invoke_stdin(data: &[u8], extra: &[&str]) -> std::process::Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_go-quest-port"))
        .args(["inspect", "-"])
        .args(extra)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn CLI");

    child
        .stdin
        .take()
        .expect("capture stdin")
        .write_all(data)
        .expect("send input");

    child.wait_with_output().expect("wait for CLI")
}

#[test]
fn piped_quest_json_reports_source_and_evaluation() {
    let data = br#"{"id":"piped-1","config":{"expires_at":"2030-01-01T00:00:00Z"}}"#;
    let result = invoke_stdin(data, &["--json", "--at", "2026-11-01T12:00:00Z"]);
    assert!(result.status.success(), "{:?}", result.stderr);
    let parsed: Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(parsed["input_trust"], "unverified_local_json");
    assert_eq!(parsed["analyzed"], 1);
    assert_eq!(parsed["assessments"][0]["quest_id"], "piped-1");
    assert_eq!(parsed["evaluated_at"], "2026-11-01T12:00:00+00:00");
}

#[test]
fn malformed_piped_input_exits_cleanly_without_json_fragment() {
    let result = invoke_stdin(br#"{"quests":[ "#, &["--json"]);
    assert!(!result.status.success());
    assert!(result.stdout.is_empty());
    assert!(String::from_utf8(result.stderr)
        .unwrap()
        .contains("Invalid JSON"));
}

#[test]
fn non_utf8_piped_input_fails_without_printing_bytes() {
    let result = invoke_stdin(&[0xff, 0xfe], &["--json"]);
    assert!(!result.status.success());
    assert!(result.stdout.is_empty());
}

#[test]
fn oversized_piped_json_is_rejected_without_partial_output() {
    let payload = vec![b' '; go_quest_port::MAX_JSON_BYTES + 1];
    let result = invoke_stdin(&payload, &["--json"]);
    assert!(!result.status.success());
    assert!(result.stdout.is_empty());
    assert!(String::from_utf8(result.stderr)
        .unwrap()
        .contains("input limit"));
}
