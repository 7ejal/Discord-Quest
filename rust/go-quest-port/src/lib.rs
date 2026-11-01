//! Offline Rust port of the Go Discord Quest project's portable components.
//! Deliberately excludes personal-token logins, engagement spoofing and claims.

pub mod manager;
pub mod model;

use model::Quest;
use serde_json::Value;

/// Upper bound for each local JSON document, including programmatic callers.
pub const MAX_JSON_BYTES: usize = 8 * 1024 * 1024;
pub const MAX_QUESTS: usize = 10_000;

/// Require an identifiable full Quest before deserializing it. The Go wire
/// structs accept zero values, but silently treating an empty object as a real
/// campaign would produce a misleading status report.
fn parse_full_quest(value: Value, index: usize) -> Result<Quest, String> {
    let obj = value
        .as_object()
        .ok_or_else(|| format!("Quest at index {index} must be an object"))?;
    let id = obj.get("id").and_then(Value::as_str).unwrap_or("");
    if id.trim().is_empty() {
        return Err(format!(
            "Quest at index {index} must have a non-empty string id"
        ));
    }
    if !obj.get("config").is_some_and(Value::is_object) {
        return Err(format!("Quest at index {index} must have an object config"));
    }
    serde_json::from_value(value)
        .map_err(|error| format!("Quest at index {index}: invalid field: {error}"))
}

fn parse_quest_list(values: Vec<Value>) -> Result<Vec<Quest>, String> {
    if values.len() > MAX_QUESTS {
        return Err(format!("Too many Quests: maximum is {MAX_QUESTS}"));
    }
    values
        .into_iter()
        .enumerate()
        .map(|(index, value)| parse_full_quest(value, index))
        .collect()
}

/// Parse a Go-like response with quests, an array of Quests, or one Quest.
/// Never loads accounts or performs network requests.
pub fn parse_quests(input: &str) -> Result<(Vec<Quest>, Option<String>), String> {
    if input.len() > MAX_JSON_BYTES {
        return Err(format!("JSON exceeds {}-byte input limit", MAX_JSON_BYTES));
    }
    let document: Value =
        serde_json::from_str(input).map_err(|error| format!("Invalid JSON: {error}"))?;
    match document {
        Value::Array(quests) => Ok((parse_quest_list(quests)?, None)),
        Value::Object(mut fields) if fields.contains_key("quests") => {
            // Keep wrapper metadata distinct from the Quest records; reject
            // malformed full records rather than accepting zero-filled structs.
            let raw_quests = fields.remove("quests").ok_or("Missing quests array")?;
            let records = raw_quests
                .as_array()
                .ok_or("quests must be an array")?
                .clone();
            let blocked = fields.remove("quest_enrollment_blocked_until");
            let blocked = match blocked {
                None | Some(Value::Null) => None,
                Some(Value::String(s)) => Some(s),
                _ => {
                    return Err("quest_enrollment_blocked_until must be a string or null".to_owned())
                }
            };
            Ok((parse_quest_list(records)?, blocked))
        }
        quest @ Value::Object(_) => Ok((vec![parse_full_quest(quest, 0)?], None)),
        _ => Err("Expected a Quest object, array of Quests, or {\"quests\":[...]}".to_owned()),
    }
}

#[cfg(test)]
mod tests {
    use super::manager::{analyze_parallel, assess_quest, QuestManager, Verdict};
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
        })
        .to_string();
        let (quests, block) = parse_quests(&text).unwrap();
        assert_eq!(quests.len(), 2);
        assert!(block.unwrap().starts_with("2026-11-02"));
    }

    #[test]
    fn invalid_full_quest_objects_are_rejected() {
        for input in [
            "{}",
            r#"{"id":"  ","config":{}}"#,
            r#"{"id":"a"}"#,
            r#"{"id":"a","config":null}"#,
            r#"{"id":1,"config":{}}"#,
        ] {
            assert!(parse_quests(input).is_err(), "Unexpected success: {input}");
        }
    }

    #[test]
    fn malformed_records_are_rejected_with_index() {
        let value = json!({"quests":[fixture(), {}]}).to_string();
        let err = parse_quests(&value).expect_err("must reject blank second quest");
        assert!(err.contains("index 1"), "{err}");
        assert!(parse_quests(r#"{"quests":{}}"#).is_err());
        assert!(parse_quests(r#"{"quests":[12]}"#).is_err());
    }

    #[test]
    fn timestamp_strings_must_be_valid_rfc3339() {
        let mut q = fixture();
        q["user_status"]["completed_at"] = json!("not-a-date");
        q["user_status"]["claimed_at"] = json!("also-not-a-date");
        assert!(!decode(q.clone()).is_completed());
        assert!(!decode(q.clone()).has_claimed_rewards());
        assert_eq!(
            assess_quest(&decode(q), &now()).verdict,
            Verdict::InProgress
        );
    }

    #[test]
    fn invalid_task_timestamp_is_not_confirmed() {
        let mut q = fixture();
        q["user_status"]["progress"]["WATCH_VIDEO"]["completed_at"] = json!("garbage");
        q["config"]["task_config_v2"]["join_operator"] = json!("or");
        let result = assess_quest(&decode(q), &now());
        assert_eq!(result.verdict, Verdict::InProgress);
        assert!(
            !result
                .tasks
                .iter()
                .find(|t| t.task_type == "WATCH_VIDEO")
                .unwrap()
                .server_task_completed
        );
        assert!(result.warnings.iter().any(|w| w.contains("invalid")));
    }

    #[test]
    fn timestamp_offsets_are_accepted() {
        let mut q = fixture();
        q["user_status"]["completed_at"] = json!("2026-11-01T15:15:00+04:00");
        assert!(decode(q.clone()).is_completed());
        assert_eq!(
            assess_quest(&decode(q), &now()).verdict,
            Verdict::QuestConfirmedUnclaimed
        );
    }

    #[test]
    fn excessive_local_json_is_rejected_before_parsing() {
        let too_large = " ".repeat(super::MAX_JSON_BYTES + 1);
        assert!(parse_quests(&too_large)
            .unwrap_err()
            .contains("input limit"));
    }

    #[test]
    fn excessive_quest_count_is_rejected() {
        let sample = json!({"id":"valid","config":{}});
        let values = vec![sample; super::MAX_QUESTS + 1];
        let input = serde_json::to_string(&values).unwrap();
        assert!(parse_quests(&input)
            .unwrap_err()
            .contains("Too many Quests"));
    }

    #[test]
    fn bad_enrollment_date_cannot_establish_enrollment() {
        let mut q = fixture();
        q["user_status"]["enrolled_at"] = json!("nonsense");
        let quest = decode(q);
        assert!(!quest.is_enrolled());
        let report = assess_quest(&quest, &now());
        assert_eq!(report.verdict, Verdict::NotEnrolled);
        assert!(report
            .warnings
            .iter()
            .any(|s| s.contains("invalid enrolled_at")));
    }

    #[test]
    fn and_requires_both_tasks() {
        assert_eq!(
            assess_quest(&decode(fixture()), &now()).verdict,
            Verdict::InProgress
        );
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
        assert_eq!(
            assess_quest(&decode(q), &now()).verdict,
            Verdict::InProgress
        );
    }

    #[test]
    fn verified_quest_and_claim_are_separate() {
        let mut q = fixture();
        q["user_status"]["completed_at"] = json!("2026-11-01T11:15:00Z");
        assert_eq!(
            assess_quest(&decode(q.clone()), &now()).verdict,
            Verdict::QuestConfirmedUnclaimed
        );
        q["user_status"]["claimed_at"] = json!("2026-11-01T11:20:00Z");
        assert_eq!(
            assess_quest(&decode(q), &now()).verdict,
            Verdict::RewardClaimed
        );
    }

    #[test]
    fn unknown_compound_operator_is_unsupported() {
        let mut q = fixture();
        q["config"]["task_config_v2"]["join_operator"] = json!("xor");
        assert_eq!(
            assess_quest(&decode(q), &now()).verdict,
            Verdict::Unsupported
        );
    }

    #[test]
    fn missing_enrollment_remains_not_enrolled() {
        let mut q = fixture();
        q["user_status"]["enrolled_at"] = Value::Null;
        assert_eq!(
            assess_quest(&decode(q), &now()).verdict,
            Verdict::NotEnrolled
        );
    }

    #[test]
    fn claimed_without_completed_is_inconsistent() {
        let mut q = fixture();
        q["user_status"]["claimed_at"] = json!("2026-11-01T11:20:00Z");
        assert_eq!(
            assess_quest(&decode(q), &now()).verdict,
            Verdict::Inconsistent
        );
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
        let video = result
            .tasks
            .iter()
            .find(|task| task.task_type == "WATCH_VIDEO")
            .unwrap();
        assert_eq!(video.application_id.as_deref(), Some("app-v"));
    }

    #[test]
    fn filters_match_go_intent() {
        let mut q = fixture();
        q["user_status"]["completed_at"] = json!("2026-11-01T11:10:00Z");
        let manager = QuestManager::new(vec![decode(fixture()), decode(q)], None);
        assert_eq!(manager.filter_valid(now()).len(), 1);
        assert_eq!(manager.filter_redeemable().len(), 1);
    }

    #[test]
    fn concurrency_preserves_result_order() {
        let mut q1 = fixture();
        let mut q2 = fixture();
        q1["id"] = json!("first");
        q2["id"] = json!("second");
        let quests = vec![decode(q1), decode(q2)];
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
        assert_eq!(
            assess_quest(&decode(q), &now()).verdict,
            Verdict::InProgress
        );
    }

    #[test]
    fn empty_and_invalid_quest_documents_are_rejected_or_empty() {
        assert_eq!(parse_quests("[]").unwrap().0.len(), 0);
        assert!(parse_quests("123").is_err());
        assert!(parse_quests("not JSON").is_err());
    }
}
