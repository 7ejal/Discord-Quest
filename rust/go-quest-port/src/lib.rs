//! Offline Rust port of the Go Discord Quest project's portable components.
//! Deliberately excludes personal-token logins, engagement spoofing and claims.

pub mod manager;
pub mod model;

use model::{AllQuestsResponse, Quest};
use serde_json::Value;

/// Parse a Go-like response with quests, an array of Quests, or one Quest.
/// Never loads accounts or performs network requests.
pub fn parse_quests(input: &str) -> Result<(Vec<Quest>, Option<String>), String> {
    let document: Value = serde_json::from_str(input)
        .map_err(|error| format!("Invalid JSON: {error}"))?;
    match document {
        Value::Array(quests) => {
            let quests: Vec<Quest> = serde_json::from_value(Value::Array(quests))
                .map_err(|error| format!("Invalid Quest array: {error}"))?;
            Ok((quests, None))
        }
        Value::Object(fields) if fields.contains_key("quests") => {
            let decoded: AllQuestsResponse =
                serde_json::from_value(Value::Object(fields))
                    .map_err(|error| format!("Invalid Go-style Quest response: {error}"))?;
            Ok((decoded.quests, decoded.quest_enrollment_blocked_until))
        }
        Value::Object(fields) => {
            let quest: Quest = serde_json::from_value(Value::Object(fields))
                .map_err(|error| format!("Invalid Quest object: {error}"))?;
            Ok((vec![quest], None))
        }
        _ => Err("Expected a Quest object, array of Quests, or {\"quests\":[...]}".to_owned()),
    }
}

#[cfg(test)]
mod tests {
    use super::manager::{assess_quest, analyze_parallel, QuestManager, Verdict};
    use super::model::Quest;
    use super::parse_quests;
    use chrono::{DateTime, Utc};
    use serde_json::{json, Value};

    fn now() -> DateTime<Utc> {
        DateTime::parse_from_rfc3339("2026-11-01T12:00:00Z")
            .unwrap()
            .with_timezone(&Utc)
    }

    fn fixture() -> Value {
        json!({
            "id":"demo-001",
            "config":{
                "messages":{"quest_name":"Offline Test Campaign"},
                "starts_at":"2026-01-01T00:00:00Z",
                "expires_at":"2027-01-01T00:00:00Z",
                "application":{"id":"app-a","name":"Example"},
                "task_config_v2":{
                    "join_operator":"and",
                    "tasks":{
                        "WATCH_VIDEO":{"target":60,"applications":[{"id":"app-v"}]},
                        "PLAY_ACTIVITY":{"target":120}
                    }
                }
            },
            "user_status":{
                "enrolled_at":"2026-11-01T11:00:00Z",
                "completed_at":null,
                "claimed_at":null,
                "progress":{
                    "WATCH_VIDEO":{"value":60,"completed_at":"2026-11-01T11:05:00Z"},
                    "PLAY_ACTIVITY":{"value":0}
                }
            }
        })
    }

    fn decode(q: Value) -> Quest {
        serde_json::from_value(q).unwrap()
    }

    #[test]
    fn parses_wrapped_go_response_and_block_marker() {
        let text = json!({
            "quests":[fixture(),fixture()],
            "quest_enrollment_blocked_until":"2026-11-02T00:00:00Z"
        }).to_string();
        let (quests, block) = parse_quests(&text).unwrap();
        assert_eq!(quests.len(), 2);
        assert!(block.unwrap().starts_with("2026-11-02"));
    }

    #[test]
    fn and_requires_both_tasks() {
        assert_eq!(assess_quest(&decode(fixture()), &now()).verdict, Verdict::InProgress);
    }

    #[test]
    fn all_and_tasks_finished_still_need_quest_confirmation() {
        let mut q = fixture();
        q["user_status"]["progress"]["PLAY_ACTIVITY"]["completed_at"] =
            json!("2026-11-01T11:10:00Z");
        assert_eq!(
            assess_quest(&decode(q), &now()).verdict,
            Verdict::TasksCompleteQuestUnconfirmed
        );
    }

    #[test]
    fn or_one_task_completion_does_not_imply_quest_completion() {
        let mut q = fixture();
        q["config"]["task_config_v2"]["join_operator"] = json!("or");
        assert_eq!(
            assess_quest(&decode(q), &now()).verdict,
            Verdict::TasksCompleteQuestUnconfirmed
        );
    }

    #[test]
    fn number_at_target_is_not_server_confirmed() {
        let mut q = fixture();
        q["user_status"]["progress"]["PLAY_ACTIVITY"]["value"] = json!(120);
        assert_eq!(assess_quest(&decode(q), &now()).verdict, Verdict::InProgress);
    }

    #[test]
    fn verified_quest_and_claim_are_separate() {
        let mut q = fixture();
        q["user_status"]["completed_at"] = json!("2026-11-01T11:15:00Z");
        assert_eq!(assess_quest(&decode(q.clone()), &now()).verdict,
            Verdict::QuestConfirmedUnclaimed);
        q["user_status"]["claimed_at"] = json!("2026-11-01T11:20:00Z");
        assert_eq!(assess_quest(&decode(q), &now()).verdict,
            Verdict::RewardClaimed);
    }

    #[test]
    fn unknown_compound_operator_is_unsupported() {
        let mut q = fixture();
        q["config"]["task_config_v2"]["join_operator"] = json!("xor");
        assert_eq!(assess_quest(&decode(q), &now()).verdict, Verdict::Unsupported);
    }

    #[test]
    fn missing_enrollment_remains_not_enrolled() {
        let mut q = fixture();
        q["user_status"]["enrolled_at"] = Value::Null;
        assert_eq!(assess_quest(&decode(q), &now()).verdict, Verdict::NotEnrolled);
    }

    #[test]
    fn claimed_without_completed_is_inconsistent() {
        let mut q = fixture();
        q["user_status"]["claimed_at"] = json!("2026-11-01T11:20:00Z");
        assert_eq!(assess_quest(&decode(q), &now()).verdict, Verdict::Inconsistent);
    }

    #[test]
    fn first_task_compatibility_does_not_become_verdict() {
        let quest = decode(fixture());
        assert_eq!(quest.original_go_first_task(), Some("WATCH_VIDEO"));
        assert_eq!(assess_quest(&quest, &now()).verdict, Verdict::InProgress);
    }

    #[test]
    fn per_task_application_takes_precedence_over_global() {
        let result = assess_quest(&decode(fixture()), &now());
        let video = result.tasks.iter().find(|task| task.task_type == "WATCH_VIDEO").unwrap();
        assert_eq!(video.application_id.as_deref(), Some("app-v"));
    }

    #[test]
    fn filters_match_go_intent() {
        let mut q = fixture();
        q["user_status"]["completed_at"] = json!("2026-11-01T11:10:00Z");
        let manager = QuestManager::new(vec![decode(fixture()),decode(q)], None);
        assert_eq!(manager.filter_valid(now()).len(), 1);
        assert_eq!(manager.filter_redeemable().len(), 1);
    }

    #[test]
    fn concurrency_preserves_result_order() {
        let mut q1 = fixture();
        let mut q2 = fixture();
        q1["id"] = json!("first");
        q2["id"] = json!("second");
        let quests = vec![decode(q1),decode(q2)];
        let results = analyze_parallel(&quests, now(), 2);
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].quest_id, "first");
        assert_eq!(results[1].quest_id, "second");
    }

    #[test]
    fn old_schema_supported_without_crashing() {
        let mut q = fixture();
        q["config"]["task_config"] = q["config"]["task_config_v2"].clone();
        q["config"]["task_config_v2"] = Value::Null;
        assert_eq!(assess_quest(&decode(q), &now()).verdict, Verdict::InProgress);
    }

    #[test]
    fn empty_and_invalid_quest_documents_are_rejected_or_empty() {
        assert_eq!(parse_quests("[]").unwrap().0.len(), 0);
        assert!(parse_quests("123").is_err());
        assert!(parse_quests("not JSON").is_err());
    }
}
