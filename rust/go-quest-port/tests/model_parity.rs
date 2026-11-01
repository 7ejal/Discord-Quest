//! Regression tests for Go's portable task, reward, and heartbeat wire models.
//! All fixtures are synthetic; no Discord account or network access.
use chrono::{DateTime, Utc};
use go_quest_port::{
    manager::{assess_quest, Verdict},
    model::Quest,
};
use serde_json::{json, Value};

fn time() -> DateTime<Utc> {
    DateTime::parse_from_rfc3339("2026-11-01T12:00:00Z")
        .unwrap()
        .with_timezone(&Utc)
}

fn record() -> Value {
    json!({
        "id":"test-quest",
        "config":{
            "id":"test-quest",
            "starts_at":"2026-01-01T00:00:00Z",
            "expires_at":"2027-01-01T00:00:00Z",
            "task_config_v2":{"tasks":{
                "WATCH_VIDEO":{"type":"WATCH_VIDEO", "target":30,"event_name":"VIDEO_WATCH"}
            }},
            "rewards_config":{
                "assignment_method":1,
                "rewards_expire_at":"2027-02-01T00:00:00Z",
                "platforms":[0,1],
                "rewards":[{"type":0,"sku_id":"synthetic-sku",
                   "asset":"image.png","asset_video":"preview.mp4",
                   "messages":{"name":"Test reward","name_with_article":"a Test reward"}}]
            }
        },
        "user_status":{
            "quest_id":"test-quest",
            "enrolled_at":"2026-11-01T11:00:00Z",
            "progress":{"WATCH_VIDEO":{
                "event_name":"VIDEO_WATCH",
                "value":15,"updated_at":"2026-11-01T11:30:00Z",
                "heartbeat":{"last_beat_at":"2026-11-01T11:30:00Z",
                             "expires_at":"2026-11-01T11:45:00Z"}
            }}
        }
    })
}

fn decode(value: Value) -> Quest {
    serde_json::from_value(value).unwrap()
}

#[test]
fn exact_go_reward_fields_remain_available() {
    let quest = decode(record());
    let config = &quest.config.rewards_config;
    assert_eq!(config.assignment_method, Some(1));
    assert_eq!(config.platforms, vec![0, 1]);
    assert_eq!(
        config.rewards_expire_at.as_deref(),
        Some("2027-02-01T00:00:00Z")
    );
    let item = &config.rewards[0];
    assert_eq!(item.sku_id.as_deref(), Some("synthetic-sku"));
    assert_eq!(item.asset.as_deref(), Some("image.png"));
    assert_eq!(item.asset_video.as_deref(), Some("preview.mp4"));
    assert_eq!(item.messages.as_ref().unwrap().name_with_article, "a Test reward");
}

#[test]
fn matching_task_type_and_event_remain_valid() {
    let quest = decode(record());
    let result = assess_quest(&quest, &time());
    assert_eq!(result.verdict, Verdict::InProgress);
    assert_eq!(
        quest.tasks().unwrap().tasks["WATCH_VIDEO"]
            .task_type
            .as_deref(),
        Some("WATCH_VIDEO")
    );
    assert!(!result
        .warnings
        .iter()
        .any(|w| w.contains("mismatch") || w.contains("conflicting")));
}

#[test]
fn conflicting_task_type_never_reports_a_claimed_reward() {
    let mut input = record();
    input["config"]["task_config_v2"]["tasks"]["WATCH_VIDEO"]["type"] = json!("PLAY_ON_DESKTOP");
    input["user_status"]["completed_at"] = json!("2026-11-01T11:40:00Z");
    input["user_status"]["claimed_at"] = json!("2026-11-01T11:50:00Z");
    let result = assess_quest(&decode(input), &time());
    assert_eq!(result.verdict, Verdict::Inconsistent);
    assert!(result
        .warnings
        .iter()
        .any(|w| w.contains("conflicting type")));
}

#[test]
fn unrecognized_declared_type_is_flagged() {
    let mut input = record();
    input["config"]["task_config_v2"]["tasks"]["WATCH_VIDEO"]["type"] = json!("UNSEEN_TYPE");
    let result = assess_quest(&decode(input), &time());
    assert_eq!(result.verdict, Verdict::Inconsistent);
    assert!(result
        .warnings
        .iter()
        .any(|w| w.contains("unrecognized declared type")));
}

#[test]
fn conflicting_event_names_are_inconsistent() {
    let mut input = record();
    input["user_status"]["progress"]["WATCH_VIDEO"]["event_name"] = json!("OTHER_EVENT");
    let result = assess_quest(&decode(input), &time());
    assert_eq!(result.verdict, Verdict::Inconsistent);
    assert!(result
        .warnings
        .iter()
        .any(|w| w.contains("mismatched event_name")));
}

#[test]
fn input_heartbeat_and_freshness_are_preserved_not_fabricated() {
    let quest = decode(record());
    let result = assess_quest(&quest, &time());
    let task = &result.tasks[0];
    assert_eq!(task.reported_event_name.as_deref(), Some("VIDEO_WATCH"));
    assert_eq!(
        task.progress_updated_at.as_deref(),
        Some("2026-11-01T11:30:00Z")
    );
    assert_eq!(
        task.heartbeat_last_beat_at.as_deref(),
        Some("2026-11-01T11:30:00Z")
    );
    assert_eq!(
        task.heartbeat_expires_at.as_deref(),
        Some("2026-11-01T11:45:00Z")
    );
    assert!(!task.server_task_completed);
    assert!(result
        .warnings
        .iter()
        .any(|w| w.contains("expired reported heartbeat")));
}

#[test]
fn invalid_progress_time_is_reported() {
    let mut input = record();
    input["user_status"]["progress"]["WATCH_VIDEO"]["updated_at"] = json!("invalid");
    input["user_status"]["progress"]["WATCH_VIDEO"]["heartbeat"]["last_beat_at"] = json!("???");
    let result = assess_quest(&decode(input), &time());
    assert!(result
        .warnings
        .iter()
        .any(|w| w.contains("invalid updated_at")));
    assert!(result
        .warnings
        .iter()
        .any(|w| w.contains("invalid heartbeat.last_beat_at")));
}

#[test]
fn null_optional_reward_metadata_is_safe() {
    let mut input = record();
    input["config"]["rewards_config"]["rewards"][0]["asset"] = Value::Null;
    input["config"]["rewards_config"]["rewards"][0]["asset_video"] = Value::Null;
    input["config"]["rewards_config"]["rewards"][0]["messages"] = Value::Null;
    input["config"]["rewards_config"]["rewards_expire_at"] = Value::Null;
    let quest = decode(input);
    let item = &quest.config.rewards_config.rewards[0];
    assert!(item.asset.is_none());
    assert!(item.asset_video.is_none());
    assert!(item.messages.is_none());
    assert!(quest.config.rewards_config.rewards_expire_at.is_none());
}

#[test]
fn blank_application_ids_are_skipped_and_reported() {
    let mut input = record();
    input["config"]["application"]["id"] = json!("global-application");
    input["config"]["task_config_v2"]["tasks"]["WATCH_VIDEO"]["applications"] =
        json!([{"id":" "},{"id":"task-specific-application"}]);
    let result = assess_quest(&decode(input), &time());
    assert_eq!(result.tasks[0].application_id.as_deref(), Some("task-specific-application"));
    assert!(result.warnings.iter().any(|w| w.contains("blank application ID")));
}

#[test]
fn blank_task_application_ids_fall_back_to_nonblank_campaign_id() {
    let mut input = record();
    input["config"]["application"]["id"] = json!("campaign-application");
    input["config"]["task_config_v2"]["tasks"]["WATCH_VIDEO"]["applications"] =
        json!([{"id":"   "}]);
    let result = assess_quest(&decode(input), &time());
    assert_eq!(result.tasks[0].application_id.as_deref(), Some("campaign-application"));
    assert!(result.warnings.iter().any(|w| w.contains("blank application ID")));
}
