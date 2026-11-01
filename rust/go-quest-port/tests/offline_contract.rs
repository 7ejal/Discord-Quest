//! Bounded synthetic parser, privacy, CLI and resource checks.
use go_quest_port::{
    manager::{analyze_parallel, assess_quest, Verdict},
    model::TaskKind,
    parse_document,
};
use serde_json::{json, Value};
use std::{
    io::Write,
    process::{Command, Stdio},
};

fn at() -> chrono::DateTime<chrono::Utc> {
    "2026-11-01T12:00:00Z".parse().unwrap()
}
fn fixture() -> Value {
    json!({"id":"synthetic", "config":{"expires_at":"2027-01-01T00:00:00Z",
        "task_config_v2":{"tasks":{"WATCH_VIDEO":{"target":60}}}},
        "user_status":{"enrolled_at":"2026-11-01T10:00:00Z","progress":{}}})
}
fn cli(input: &[u8], options: &[&str]) -> std::process::Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_go-quest-port"))
        .args(["inspect", "-"])
        .args(options)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(input).unwrap();
    child.wait_with_output().unwrap()
}

#[test]
fn metadata_and_nested_extensions_survive_library_round_trip() {
    let mut q = fixture();
    q["config"]["features"] = json!({"future": [1,2]});
    q["config"]["assets"] = json!({"hero":"image","hero_video":null,"extension":{"nested":true}});
    q["config"]["colors"] = json!({"primary":"#fff","secondary":"#000"});
    q["config"]["video_metadata"] = json!({"messages":{"video_title":"synthetic"},"assets":{"video_player_video":"offline.mp4"}});
    q["config"]["cosponsor_metadata"] =
        json!({"name":"sample","redemption_instructions":"read-only"});
    q["config"]["application"] = json!({"id":"app","link":"example.invalid","future":{"new":1}});
    q["config"]["messages"] = json!({"game_publisher":"synthetic"});
    q["user_status"]["user_id"] = json!("PRIVATE_USER_CANARY");
    let parsed = parse_document(&q.to_string()).unwrap();
    let roundtrip = serde_json::to_value(&parsed.quests[0]).unwrap();
    for pointer in [
        "/config/features",
        "/config/assets",
        "/config/colors",
        "/config/video_metadata",
        "/config/cosponsor_metadata",
        "/config/application/future",
        "/config/messages/game_publisher",
        "/user_status/user_id",
    ] {
        assert_eq!(roundtrip.pointer(pointer), q.pointer(pointer), "{pointer}");
    }
    let again = parse_document(&roundtrip.to_string()).unwrap();
    assert_eq!(again.quests.len(), 1);
}

#[test]
fn missing_and_null_status_have_explicit_separate_diagnostics() {
    let mut q = fixture();
    q.as_object_mut().unwrap().remove("user_status");
    let missing = parse_document(&q.to_string()).unwrap();
    assert!(missing.quests[0]
        .field_warnings
        .iter()
        .any(|s| s.contains("missing user_status")));
    q["user_status"] = Value::Null;
    let null = parse_document(&q.to_string()).unwrap();
    assert!(null.quests[0]
        .field_warnings
        .iter()
        .any(|s| s.contains("null user_status")));
}

#[test]
fn unknown_task_kind_retains_its_identifier_without_claiming_support() {
    assert_eq!(
        TaskKind::from("FUTURE_TASK"),
        TaskKind::Unknown("FUTURE_TASK".to_owned())
    );
    assert_eq!(TaskKind::from("FUTURE_TASK").family(), "unknown");
}

#[test]
fn milestones_have_local_provenance_and_independent_redemption() {
    let mut q = fixture();
    for claimed in [false, true] {
        q["user_status"]["completed_at"] = json!("2026-11-01T11:00:00Z");
        if claimed {
            q["user_status"]["claimed_at"] = json!("2026-11-01T11:01:00Z");
        }
        let result = cli(
            q.to_string().as_bytes(),
            &["--json", "--at", "2026-11-01T12:00:00Z"],
        );
        assert!(result.status.success());
        let report: Value = serde_json::from_slice(&result.stdout).unwrap();
        let a = &report["assessments"][0];
        assert_eq!(a["input_trust"], "unverified_local_json");
        assert_eq!(a["tasks"][0]["input_trust"], "unverified_local_json");
        assert_eq!(a["external_redemption"], "unknown");
        assert_eq!(
            a["reward_claim_availability"],
            if claimed {
                "reported_claimed"
            } else {
                "reported_completed_unclaimed_eligibility_unverified"
            }
        );
    }
}

#[test]
fn summaries_do_not_echo_user_targeting_or_unknown_private_fields() {
    let mut q = fixture();
    q["user_status"]["user_id"] = json!("PRIVATE_USER_CANARY");
    q["token"] = json!("PRIVATE_TOKEN_CANARY");
    q["targeted_content"] = json!({"authorization":"PRIVATE_TARGET_CANARY"});
    q["config"]["video_metadata"] = json!({"token":"PRIVATE_VIDEO_CANARY"});
    for options in [vec!["--json"], vec!["--verbose"]] {
        let result = cli(q.to_string().as_bytes(), &options);
        assert!(result.status.success());
        let text = format!(
            "{}{}",
            String::from_utf8_lossy(&result.stdout),
            String::from_utf8_lossy(&result.stderr)
        );
        assert!(!text.contains("PRIVATE_"));
    }
}

#[test]
fn malformed_json_never_echoes_input_in_errors() {
    for bytes in [
        b"{\"PRIVATE_SECRET_CANARY\":".as_slice(),
        b"{\"quests\":[\"PRIVATE_SECRET_CANARY\"]}",
        b"{\"id\":\"q\",\"config\":{\"config_version\":\"PRIVATE_SECRET_CANARY\"}}",
    ] {
        let result = cli(bytes, &["--json", "--verbose"]);
        assert_eq!(result.status.code(), Some(65));
        assert!(result.stdout.is_empty());
        assert!(!String::from_utf8_lossy(&result.stderr).contains("PRIVATE_SECRET_CANARY"));
    }
}

#[test]
fn limits_apply_before_any_text_or_json_is_emitted() {
    for format in [
        vec!["--json", "--max-output-bytes", "16"],
        vec!["--max-output-bytes", "16"],
    ] {
        let result = cli(fixture().to_string().as_bytes(), &format);
        assert!(!result.status.success());
        assert!(result.stdout.is_empty());
    }
    let result = cli(b"[]", &["--max-input-bytes", "1"]);
    assert_eq!(result.status.code(), Some(65));
}

#[test]
fn flags_fail_before_stdin_read_and_do_not_echo_tokens() {
    for options in [
        vec!["demo", "--token", "PRIVATE_CANARY"],
        vec!["demo", "--claim"],
        vec!["demo", "--captcha"],
        vec!["demo", "--workers"],
        vec!["demo", "--workers", "0"],
        vec!["demo", "--workers", "33"],
        vec!["--version", "PRIVATE_CANARY"],
        vec!["demo", "--max-input-bytes", "999999999"],
    ] {
        let result = Command::new(env!("CARGO_BIN_EXE_go-quest-port"))
            .args(options)
            .output()
            .unwrap();
        assert_eq!(result.status.code(), Some(64));
        assert!(result.stdout.is_empty());
        assert!(!String::from_utf8_lossy(&result.stderr).contains("PRIVATE_CANARY"));
    }
}

#[test]
fn help_version_empty_input_and_verbose_have_stable_streams() {
    let help = Command::new(env!("CARGO_BIN_EXE_go-quest-port"))
        .arg("--help")
        .output()
        .unwrap();
    assert!(help.status.success());
    assert!(String::from_utf8_lossy(&help.stdout).contains("--verbose"));
    assert!(help.stderr.is_empty());
    let result = cli(b"[]", &["--json", "--verbose"]);
    assert!(result.status.success());
    assert_eq!(
        serde_json::from_slice::<Value>(&result.stdout).unwrap()["analyzed"],
        0
    );
    assert!(String::from_utf8_lossy(&result.stderr).contains("Offline analysis"));
    let io = Command::new(env!("CARGO_BIN_EXE_go-quest-port"))
        .args(["inspect", "/does-not-exist/PRIVATE_CANARY"])
        .output()
        .unwrap();
    assert_eq!(io.status.code(), Some(74));
    assert!(!String::from_utf8_lossy(&io.stderr).contains("PRIVATE_CANARY"));
}

#[cfg(target_os = "linux")]
#[test]
fn linux_non_utf8_filename_is_accepted_without_path_disclosure() {
    use std::os::unix::ffi::OsStringExt;
    let path = std::env::temp_dir().join(std::ffi::OsString::from_vec(
        format!("quest-{}-", std::process::id())
            .into_bytes()
            .into_iter()
            .chain([255])
            .collect(),
    ));
    std::fs::write(&path, b"[]").unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_go-quest-port"))
        .arg("inspect")
        .arg(&path)
        .arg("--json")
        .output()
        .unwrap();
    std::fs::remove_file(path).unwrap();
    assert!(result.status.success());
}

#[test]
fn deeply_nested_and_hundred_thousand_task_inputs_are_bounded() {
    let deep = format!("{}0{}", "[".repeat(65), "]".repeat(65));
    assert!(parse_document(&deep).unwrap_err().contains("depth"));
    let mut q = fixture();
    let tasks = (0..100_001)
        .map(|i| (format!("FUTURE_{i}"), json!({"target":1})))
        .collect::<serde_json::Map<_, _>>();
    q["config"]["task_config_v2"]["tasks"] = Value::Object(tasks);
    assert!(parse_document(&q.to_string())
        .unwrap_err()
        .contains("entries exceed"));
    assert!(parse_document(&format!("[{}]", " ".repeat(go_quest_port::MAX_JSON_BYTES))).is_err());
}

#[test]
fn duplicate_empty_and_large_lists_preserve_order_and_fail_conflicts() {
    assert!(parse_document("[]").unwrap().quests.is_empty());
    let q = fixture();
    assert_eq!(
        parse_document(&json!([q, q]).to_string())
            .unwrap()
            .quests
            .len(),
        1
    );
    let mut conflicting = q.clone();
    conflicting["preview"] = json!(true);
    assert!(parse_document(&json!([q, conflicting]).to_string()).is_err());
    let values: Vec<_> = (0..1000)
        .map(|i| {
            let mut v = fixture();
            v["id"] = json!(format!("q-{i}"));
            v
        })
        .collect();
    let parsed = parse_document(&json!(values).to_string()).unwrap();
    for workers in [0, 1, 2, 32, usize::MAX] {
        let results = analyze_parallel(&parsed.quests, at(), workers);
        assert_eq!(results.len(), 1000);
        assert!(results
            .iter()
            .enumerate()
            .all(|(i, r)| r.quest_id == format!("q-{i}")));
    }
}

#[test]
fn deterministic_mutation_corpus_never_panics_or_confirms_completion() {
    // Fixed seed, 1024 bounded mutations; rerun during every main-server test.
    let mut seed = 0x5eed_u64;
    for i in 0..1024 {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        let mut q = fixture();
        match seed % 7 {
            0 => q["user_status"]["completed_at"] = json!(seed.to_string()),
            1 => q["config"]["task_config_v2"]["join_operator"] = json!(seed),
            2 => q["user_status"]["progress"]["WATCH_VIDEO"] = json!({"value":seed}),
            3 => q["config"]["expires_at"] = json!("malformed"),
            4 => q["id"] = Value::Null,
            5 => q["user_status"] = Value::Null,
            _ => q["config"]["task_config_v2"]["tasks"] = json!({"FUTURE_TASK":{"target":1}}),
        }
        if let Ok(parsed) = parse_document(&q.to_string()) {
            let result = assess_quest(&parsed.quests[0], &at());
            assert!(!result.quest_completed, "mutation {i}");
            assert!(!matches!(
                result.verdict,
                Verdict::QuestConfirmedUnclaimed | Verdict::RewardClaimed
            ));
        }
        let bytes: Vec<_> = (0..(seed as usize % 64))
            .map(|n| b"{}[],:012x \""[n % 12])
            .collect();
        if let Ok(text) = std::str::from_utf8(&bytes) {
            let _ = parse_document(text);
        }
    }
}

#[test]
fn application_identity_is_kept_per_quest() {
    let mut first = fixture();
    first["id"] = json!("first");
    first["config"]["task_config_v2"]["tasks"]["WATCH_VIDEO"]["applications"] =
        json!([{"id":"first-app"}]);
    let mut second = fixture();
    second["id"] = json!("second");
    second["config"]["task_config_v2"]["tasks"]["WATCH_VIDEO"]["applications"] =
        json!([{"id":"second-app"}]);
    let parsed = parse_document(&json!([first, second]).to_string()).unwrap();
    let results = analyze_parallel(&parsed.quests, at(), 2);
    assert_eq!(
        results[0].tasks[0].application_id.as_deref(),
        Some("first-app")
    );
    assert_eq!(
        results[1].tasks[0].application_id.as_deref(),
        Some("second-app")
    );
    assert_eq!(results[0].tasks[0].application_id_source, "task_config");
}
