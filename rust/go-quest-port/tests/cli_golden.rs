//! Stable CLI golden fixtures. Test data and evaluation time are synthetic.
use std::process::Command;

const EXPECTED_TEXT: &str = include_str!("golden/demo.txt");
const EXPECTED_JSON: &str = include_str!("golden/demo.json");
const AT: &str = "2026-11-01T12:00:00Z";

fn run(args: &[&str]) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_go-quest-port"))
        .args(args)
        .output()
        .expect("start offline native CLI");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).expect("valid UTF-8 CLI stdout")
}

fn normalize_line_endings(text: &str) -> String {
    text.replace("\r\n", "\n")
}

#[test]
fn human_readable_report_matches_committed_golden() {
    let result = run(&["demo", "--at", AT]);
    assert_eq!(normalize_line_endings(&result), EXPECTED_TEXT);
}

#[test]
fn json_report_matches_committed_golden() {
    let result = run(&["demo", "--json", "--at", AT]);
    assert_eq!(normalize_line_endings(&result), EXPECTED_JSON);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["schema_version"], 1);
    assert_eq!(parsed["input_trust"], "unverified_local_json");
}

#[test]
fn worker_parallelism_does_not_change_golden_json() {
    for workers in ["1", "2", "4", "32"] {
        let result = run(&["demo", "--json", "--at", AT, "--workers", workers]);
        assert_eq!(
            normalize_line_endings(&result),
            EXPECTED_JSON,
            "Parallelism {workers} changed CLI JSON"
        );
    }
}
