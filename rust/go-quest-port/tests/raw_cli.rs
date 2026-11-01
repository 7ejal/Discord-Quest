use std::{
    io::Write,
    process::{Command, Stdio},
};

// All credentials here are synthetic; invalid input must fail before networking.
#[test]
fn synthetic_raw_cli_rejects_credentials_without_echo() {
    for input in [
        b"".as_slice(),
        b"Bearer SECRET_CANARY",
        b"Bot SECRET_CANARY",
        b"SECRET_CANARY\r\nX: injection",
        b"SECRET_CANARY\n\n",
        &[255],
        &[b'x'; 4097],
    ] {
        let mut child = Command::new(env!("CARGO_BIN_EXE_go-quest-port"))
            .args(["raw-http-check", "--token-stdin", "--json"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        child.stdin.take().unwrap().write_all(input).unwrap();
        let output = child.wait_with_output().unwrap();
        assert_eq!(output.status.code(), Some(69));
        assert!(output.stdout.is_empty());
        assert!(!String::from_utf8_lossy(&output.stderr).contains("SECRET_CANARY"));
    }
}

#[test]
fn synthetic_raw_cli_requires_stdin_mode_and_rejects_secret_arguments() {
    for args in [
        vec!["raw-http-check"],
        vec!["raw-http-check", "--token", "SECRET_CANARY"],
        vec!["raw-http-check", "--token-stdin", "--json", "--json"],
    ] {
        let output = Command::new(env!("CARGO_BIN_EXE_go-quest-port"))
            .args(args)
            .stdin(Stdio::null())
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(64));
        assert!(output.stdout.is_empty());
        assert!(!String::from_utf8_lossy(&output.stderr).contains("SECRET_CANARY"));
    }
}
