use std::process::{Command, Stdio};

fn cli() -> Command {
    Command::new(env!("CARGO_BIN_EXE_go-quest-port"))
}

#[test]
fn help_lists_live_commands_with_clear_limits() {
    let output = cli().arg("--help").output().unwrap();
    assert!(output.status.success());
    let text = String::from_utf8(output.stdout).unwrap();
    assert!(text.contains("quest-config <public-quest-id>"));
    assert!(text.contains("oauth-check --oauth-token-stdin"));
    assert!(!text.contains(r"\n\"));
    assert!(text.contains("do not read personal Quests"));
}

#[test]
fn malformed_id_fails_without_network_or_echoing_content() {
    let output = cli()
        .args(["quest-config", "12/../../SECRET_CANARY", "--json"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(69));
    assert!(output.stdout.is_empty());
    let err = String::from_utf8(output.stderr).unwrap();
    assert!(!err.contains("SECRET_CANARY"));
    assert!(err.contains("ASCII digits"));
}

#[test]
fn empty_piped_oauth_token_fails_without_account_request() {
    let output = cli()
        .args(["oauth-check", "--oauth-token-stdin", "--json"])
        .stdin(Stdio::null())
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(69));
    assert!(output.stdout.is_empty());
}

#[test]
fn missing_oauth_mode_argument_rejected_before_stdin() {
    let output = cli()
        .arg("oauth-check")
        .stdin(Stdio::null())
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(64));
    assert!(output.stdout.is_empty());
}
