//! End-to-end CLI tests: input is an untrusted local JSON file, never live Discord.
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

fn sample_file(quest_name: &str, quest_id: &str) -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time")
        .as_nanos();
    let path = std::env::temp_dir().join(format!(
        "rust-quest-cli-fixture-{}-{stamp}.json",
        std::process::id()
    ));
    let data = serde_json::json!({
        "id": quest_id,
        "config": {
            "expires_at": "2030-01-01T00:00:00Z",
            "messages": {"quest_name": quest_name}
        }
    });
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)
        .expect("open synthetic test file");
    write!(file, "{}", data).expect("write synthetic test file");
    path
}

fn run_cli(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_go-quest-port"))
        .args(args)
        .output()
        .expect("launch native CLI")
}

#[test]
fn terminal_report_escapes_input_sequences_and_preserves_unicode() {
    let file = sample_file(
        "Visible \u{001B}[2J\u{000D}\u{000A}\u{202E} مرحباً",
        "id-\u{001B}]0;fake-title\u{0007}",
    );
    let result = run_cli(&["inspect", file.to_str().unwrap()]);
    fs::remove_file(file).unwrap();
    assert!(result.status.success());
    let stdout = String::from_utf8(result.stdout).unwrap();
    assert!(stdout.contains("Visible"));
    assert!(stdout.contains("مرحباً"));
    assert!(stdout.contains("\\u{001B}"));
    assert!(stdout.contains("\\u{202E}"));
    assert!(!stdout.contains('\u{001B}'));
    assert!(!stdout.contains('\u{000D}'));
    assert!(!stdout.contains('\u{202E}'));
}

#[test]
fn malformed_argument_does_not_echo_token_like_secret() {
    let secret = "--fake-token=SENSITIVE_EXAMPLE_STRING";
    let result = run_cli(&["demo", secret]);
    assert!(!result.status.success());
    let stderr = String::from_utf8(result.stderr).unwrap();
    assert!(!stderr.contains("SENSITIVE_EXAMPLE_STRING"));
    assert!(stderr.contains("Unknown argument"));
}

#[test]
fn json_report_labels_untrusted_source_and_has_version() {
    let file = sample_file("Ordinary campaign", "quest-local-1");
    let result = run_cli(&["inspect", file.to_str().unwrap(), "--json"]);
    fs::remove_file(file).unwrap();
    assert!(result.status.success());
    let report: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(report["schema_version"], 1);
    assert_eq!(report["input_trust"], "unverified_local_json");
    assert_eq!(report["analyzed"], 1);
}
