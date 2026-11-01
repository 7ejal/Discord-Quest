//! Synthetic state regressions; no engagement handlers or network requests.
use chrono::{DateTime, Utc};
use go_quest_port::{manager::{assess_quest, Verdict}, parse_document};
use serde_json::{json, Value};

fn at() -> DateTime<Utc> { "2026-11-01T12:00:00Z".parse().unwrap() }
fn record() -> Value {
    json!({"id":"synthetic", "config":{
        "starts_at":"2026-01-01T00:00:00Z", "expires_at":"2027-01-01T00:00:00Z",
        "task_config_v2":{"tasks":{"WATCH_VIDEO":{"target":60}}}
    }, "user_status":{"enrolled_at":"2026-11-01T10:00:00Z",
        "progress":{"WATCH_VIDEO":{"value":60,"completed_at":"2026-11-01T11:00:00Z"}}}})
}
fn verdict(value: Value) -> Verdict {
    let parsed = parse_document(&value.to_string()).unwrap();
    assess_quest(&parsed.quests[0], &at()).verdict
}

#[test]
fn explicit_unknown_operator_on_single_task_fails_closed() {
    let mut q = record();
    q["config"]["task_config_v2"]["join_operator"] = json!("xor");
    assert_eq!(verdict(q), Verdict::Unsupported);
}

#[test]
fn unsafe_numeric_values_cannot_satisfy_task_group() {
    for field in ["target", "value"] {
        for invalid in [-1.0, 0.5, 9_007_199_254_740_992.0] {
            let mut q = record();
            if field == "target" { q["config"]["task_config_v2"]["tasks"]["WATCH_VIDEO"][field] = json!(invalid); }
            else { q["user_status"]["progress"]["WATCH_VIDEO"][field] = json!(invalid); }
            assert_eq!(verdict(q), Verdict::Inconsistent, "{field}={invalid}");
        }
    }
    let mut q = record();
    q["config"]["task_config_v2"]["tasks"]["WATCH_VIDEO"]["target"] = json!(0);
    assert_eq!(verdict(q), Verdict::Inconsistent);
}

#[test]
fn future_milestone_never_becomes_reported_completion() {
    let mut q = record();
    q["user_status"]["completed_at"] = json!("2027-01-01T00:00:00Z");
    assert_eq!(verdict(q), Verdict::Inconsistent);
}

#[test]
fn malformed_claim_does_not_look_like_unclaimed_reward() {
    let mut q = record();
    q["user_status"]["completed_at"] = json!("2026-11-01T11:00:00Z");
    q["user_status"]["claimed_at"] = json!("invalid");
    assert_eq!(verdict(q), Verdict::Unsupported);
}

#[test]
fn group_operator_and_missing_progress_table() {
    for operator in ["and", "or", "missing", "xor"] {
        for bits in 0..4 {
            let mut q = record();
            q["config"]["task_config_v2"]["tasks"]["PLAY_ACTIVITY"] = json!({"target":60});
            if operator != "missing" { q["config"]["task_config_v2"]["join_operator"] = json!(operator); }
            q["user_status"]["progress"] = json!({});
            for (bit, name) in [(1,"WATCH_VIDEO"),(2,"PLAY_ACTIVITY")] {
                if bits & bit != 0 {
                    q["user_status"]["progress"][name] = json!({"completed_at":"2026-11-01T11:00:00Z"});
                }
            }
            let expected = match operator {
                "missing" | "xor" => Verdict::Unsupported,
                "and" if bits == 3 => Verdict::TasksCompleteQuestUnconfirmed,
                "or" if bits != 0 => Verdict::TasksCompleteQuestUnconfirmed,
                _ => Verdict::InProgress,
            };
            assert_eq!(verdict(q), expected, "{operator} bits={bits}");
        }
    }
}

#[test]
fn v2_precedence_never_merges_or_falls_back_from_empty_map() {
    let mut q = record();
    q["config"]["task_config"] = q["config"]["task_config_v2"].clone();
    q["config"]["task_config_v2"]["tasks"] = json!({});
    assert_eq!(verdict(q.clone()), Verdict::Unsupported);
    q["config"]["task_config_v2"] = Value::Null;
    assert_eq!(verdict(q), Verdict::TasksCompleteQuestUnconfirmed);
}

#[test]
fn wrong_nested_types_fail_with_sanitized_diagnostics() {
    for path in ["/user_status", "/user_status/progress", "/user_status/enrolled_at",
        "/config/task_config_v2/join_operator", "/user_status/progress/WATCH_VIDEO/value"] {
        let mut q = record();
        if path.ends_with("join_operator") {
            q["config"]["task_config_v2"]["join_operator"] = json!({"token":"DO_NOT_ECHO_CANARY"});
        } else {
            *q.pointer_mut(path).unwrap() = json!({"token":"DO_NOT_ECHO_CANARY"});
        }
        let error = parse_document(&q.to_string()).unwrap_err();
        assert!(!error.contains("DO_NOT_ECHO_CANARY"));
    }
    assert!(parse_document("{\"quests\":[],\"quest_enrollment_blocked_until\":123}").is_err());
}

#[test]
fn all_ten_identifiers_are_classification_only() {
    for name in ["WATCH_VIDEO", "WATCH_VIDEO_ON_MOBILE", "PLAY_ON_DESKTOP", "PLAY_ON_DESKTOP_V2",
        "STREAM_ON_DESKTOP", "PLAY_ACTIVITY", "PLAY_ON_XBOX", "PLAY_ON_PLAYSTATION",
        "ACHIEVEMENT_IN_ACTIVITY", "ACHIEVEMENT_IN_GAME"] {
        let mut q = record();
        q["config"]["task_config_v2"]["tasks"] = json!({name:{"target":60}});
        q["user_status"]["progress"] = json!({name:{"value":60}});
        let result = assess_quest(&parse_document(&q.to_string()).unwrap().quests[0], &at());
        assert_ne!(result.tasks[0].family, "unknown");
        assert!(!result.quest_completed);
        assert!(!result.tasks[0].server_task_completed);
    }
}

#[test]
fn unsupported_tasks_keep_warnings_even_when_claim_is_reported() {
    for name in ["FUTURE_TASK", "PLAY_ON_DESKTOP_V2"] {
        let mut q = record();
        q["config"]["task_config_v2"]["tasks"] = json!({name:{"target":60}});
        q["user_status"]["completed_at"] = json!("2026-11-01T11:00:00Z");
        q["user_status"]["claimed_at"] = json!("2026-11-01T11:01:00Z");
        let result = assess_quest(&parse_document(&q.to_string()).unwrap().quests[0], &at());
        assert_eq!(result.verdict, Verdict::RewardClaimed);
        assert!(!result.warnings.is_empty());
    }
}

#[test]
fn expired_and_not_started_have_distinct_verdicts() {
    let mut q = record();
    q["config"]["starts_at"] = json!("2026-12-01T00:00:00Z");
    assert_eq!(verdict(q.clone()), Verdict::NotStarted);
    q["config"]["starts_at"] = json!("2026-01-01T00:00:00Z");
    q["config"]["expires_at"] = json!("2026-11-01T12:00:00Z");
    assert_eq!(verdict(q), Verdict::Expired);
}

#[test]
fn unknown_task_in_or_group_never_proves_group_support() {
    let mut q = record();
    q["config"]["task_config_v2"]["tasks"]["FUTURE_TASK"] = json!({"target":60});
    q["config"]["task_config_v2"]["join_operator"] = json!("or");
    assert_eq!(verdict(q), Verdict::Unsupported);
}

