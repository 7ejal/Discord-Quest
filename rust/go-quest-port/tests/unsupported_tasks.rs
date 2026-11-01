//! Unknown future Quest tasks must remain clearly unsupported even when a
//! local JSON file claims the overall Quest is completed or reward claimed.
use chrono::{DateTime, Utc};
use go_quest_port::{manager::{assess_quest, Verdict}, model::Quest};
use serde_json::{json, Value};

fn at() -> DateTime<Utc> {
    DateTime::parse_from_rfc3339("2026-11-01T12:00:00Z")
        .unwrap()
        .with_timezone(&Utc)
}

fn document(completed: bool, claimed: bool) -> Value {
    let mut data = json!({
        "id": "unknown-task-test",
        "config": {
            "expires_at": "2027-01-01T00:00:00Z",
            "task_config_v2": {"tasks": {
                "FUTURE_UNSUPPORTED_TYPE": {"type":"FUTURE_UNSUPPORTED_TYPE","target": 3}
            }}
        },
        "user_status": {"enrolled_at": "2026-11-01T10:00:00Z"}
    });
    if completed {
        data["user_status"]["completed_at"] = json!("2026-11-01T11:30:00Z");
    }
    if claimed {
        data["user_status"]["claimed_at"] = json!("2026-11-01T11:40:00Z");
    }
    data
}

#[test]
fn unsupported_task_does_not_become_known_from_claimed_quest_record() {
    let quest: Quest = serde_json::from_value(document(true, true)).unwrap();
    let result = assess_quest(&quest, &at());
    assert_eq!(result.verdict, Verdict::RewardClaimed);
    assert_eq!(result.tasks.len(), 1);
    assert_eq!(result.tasks[0].family, "unknown");
    assert!(result.warnings.iter().any(|warning| warning.contains("unrecognized task type")));
}

#[test]
fn unknown_task_remains_visible_on_reported_completed_quest() {
    let quest: Quest = serde_json::from_value(document(true, false)).unwrap();
    let result = assess_quest(&quest, &at());
    assert_eq!(result.verdict, Verdict::QuestConfirmedUnclaimed);
    assert_eq!(result.tasks[0].family, "unknown");
    assert!(!result.warnings.is_empty());
}

#[test]
fn unknown_task_without_quest_level_completion_does_not_claim_success() {
    let quest: Quest = serde_json::from_value(document(false, false)).unwrap();
    let result = assess_quest(&quest, &at());
    assert_eq!(result.verdict, Verdict::Unsupported);
    assert_eq!(result.tasks[0].family, "unknown");
}
