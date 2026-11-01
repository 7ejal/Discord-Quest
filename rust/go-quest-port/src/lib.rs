//! Offline Rust port of the Go Discord Quest project's portable components.
//! Deliberately excludes personal-token logins, engagement spoofing and claims.

pub mod manager;
pub mod model;
pub mod output;

use model::{Quest, QuestPartial};
use serde_json::Value;
use std::collections::BTreeMap;

/// Upper bound for each local JSON document, including programmatic callers.
pub const MAX_JSON_BYTES: usize = 8 * 1024 * 1024;
pub const MAX_QUESTS: usize = 10_000;
pub const MAX_TASKS: usize = 100_000;
pub const MAX_DEPTH: usize = 64;

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
    if obj
        .get("config")
        .and_then(Value::as_object)
        .is_some_and(|config| config.is_empty())
    {
        return Err(format!("Quest at index {index} has an empty config"));
    }
    if let Some(Value::Object(status)) = obj.get("user_status") {
        if !status.is_empty()
            && ![
                "quest_id",
                "user_id",
                "enrolled_at",
                "completed_at",
                "claimed_at",
                "progress",
            ]
            .iter()
            .any(|key| status.contains_key(*key))
        {
            return Err(format!(
                "Quest at index {index}: unrecognized user_status shape"
            ));
        }
    }
    let mut field_warnings = Vec::new();
    for (field, state) in [
        ("user_status", obj.get("user_status")),
        (
            "progress",
            obj.get("user_status").and_then(|s| s.get("progress")),
        ),
        (
            "enrolled_at",
            obj.get("user_status").and_then(|s| s.get("enrolled_at")),
        ),
        (
            "completed_at",
            obj.get("user_status").and_then(|s| s.get("completed_at")),
        ),
        (
            "claimed_at",
            obj.get("user_status").and_then(|s| s.get("claimed_at")),
        ),
    ] {
        if state.is_none() {
            field_warnings.push(format!("Input has missing {field}; no milestone inferred."));
        } else if state == Some(&Value::Null) {
            field_warnings.push(format!("Input has null {field}; no milestone inferred."));
        }
    }
    let mut quest: Quest = serde_json::from_value(value)
        .map_err(|_| format!("Quest at index {index}: invalid field type"))?;
    quest.field_warnings = field_warnings;
    Ok(quest)
}

fn parse_quest_list(values: Vec<Value>) -> Result<Vec<Quest>, String> {
    if values.len() > MAX_QUESTS {
        return Err(format!("Too many Quests: maximum is {MAX_QUESTS}"));
    }
    let mut seen = BTreeMap::<String, Value>::new();
    let mut unique = Vec::with_capacity(values.len());
    for (index, value) in values.into_iter().enumerate() {
        let quest = parse_full_quest(value.clone(), index)?;
        if let Some(previous) = seen.get(&quest.id) {
            if previous != &value {
                return Err(format!(
                    "Conflicting records for duplicate Quest ID at index {index}"
                ));
            }
            continue;
        }
        seen.insert(quest.id.clone(), value);
        unique.push(quest);
    }
    Ok(unique)
}

/// Preserve the complete and partial Quest records separately. A partial
/// excluded Quest must never be treated as a full, eligible campaign.
#[derive(Debug)]
pub struct ParsedDocument {
    pub quests: Vec<Quest>,
    pub excluded_quests: Vec<QuestPartial>,
    pub enrollment_blocked_until: Option<String>,
}

fn parse_excluded(values: Vec<Value>) -> Result<Vec<QuestPartial>, String> {
    if values.len() > MAX_QUESTS {
        return Err(format!("Too many excluded Quests: maximum is {MAX_QUESTS}"));
    }
    let mut seen = BTreeMap::<String, Value>::new();
    let mut unique = Vec::with_capacity(values.len());
    for (index, value) in values.into_iter().enumerate() {
        let object = value
            .as_object()
            .ok_or_else(|| format!("Excluded Quest at index {index} must be an object"))?;
        let id = object.get("id").and_then(Value::as_str).unwrap_or("");
        if id.trim().is_empty() {
            return Err(format!(
                "Excluded Quest at index {index} must have a non-empty id"
            ));
        }
        let partial: QuestPartial = serde_json::from_value(value.clone())
            .map_err(|_| format!("Excluded Quest at index {index}: invalid field type"))?;
        if let Some(previous) = seen.get(&partial.id) {
            if previous != &value {
                return Err(format!(
                    "Conflicting excluded records for ID at index {index}"
                ));
            }
            continue;
        }
        seen.insert(partial.id.clone(), value);
        unique.push(partial);
    }
    Ok(unique)
}

/// Parse one full Quest, a full Quest array or a Go-style wrapped response.
/// No account access, authentication or network requests are performed.
pub fn parse_document(input: &str) -> Result<ParsedDocument, String> {
    if input.len() > MAX_JSON_BYTES {
        return Err(format!("JSON exceeds {}-byte input limit", MAX_JSON_BYTES));
    }
    // A UTF-8 BOM is common in Windows-edited local JSON fixtures.
    // Accept it only at the very start of the document.
    let normalized = input.strip_prefix('\u{feff}').unwrap_or(input);
    // Scan structure before serde allocates nested maps. Ignore delimiters in strings.
    let mut depth = 0usize;
    let mut in_string = false;
    let mut escaped = false;
    for byte in normalized.bytes() {
        if in_string {
            if escaped {
                escaped = false;
            } else if byte == b'\\' {
                escaped = true;
            } else if byte == b'"' {
                in_string = false;
            }
        } else {
            match byte {
                b'"' => in_string = true,
                b'{' | b'[' => {
                    depth += 1;
                    if depth > MAX_DEPTH {
                        return Err("JSON nesting exceeds depth limit".to_owned());
                    }
                }
                b'}' | b']' => depth = depth.saturating_sub(1),
                _ => {}
            }
        }
    }
    let document: Value = serde_json::from_str(normalized).map_err(|error| {
        format!(
            "Invalid JSON at line {}, column {}",
            error.line(),
            error.column()
        )
    })?;
    let mut pending = vec![&document];
    let mut task_count = 0usize;
    while let Some(value) = pending.pop() {
        match value {
            Value::Object(fields) => {
                for (key, child) in fields {
                    if key == "tasks" || key == "progress" {
                        task_count =
                            task_count.saturating_add(child.as_object().map_or(0, |m| m.len()));
                        if task_count > MAX_TASKS {
                            return Err("Task/progress entries exceed document limit".to_owned());
                        }
                    }
                    pending.push(child);
                }
            }
            Value::Array(values) => pending.extend(values),
            _ => {}
        }
    }
    match document {
        Value::Array(records) => Ok(ParsedDocument {
            quests: parse_quest_list(records)?,
            excluded_quests: Vec::new(),
            enrollment_blocked_until: None,
        }),
        Value::Object(mut fields) if fields.contains_key("quests") => {
            let raw_quests = fields.remove("quests").ok_or("Missing quests array")?;
            let records = raw_quests
                .as_array()
                .ok_or("quests must be an array")?
                .clone();
            let excluded = match fields.remove("excluded_quests") {
                None | Some(Value::Null) => Vec::new(),
                Some(Value::Array(values)) => parse_excluded(values)?,
                _ => return Err("excluded_quests must be an array or null".to_owned()),
            };
            let blocked = match fields.remove("quest_enrollment_blocked_until") {
                None | Some(Value::Null) => None,
                Some(Value::String(s)) => Some(s),
                _ => {
                    return Err("quest_enrollment_blocked_until must be a string or null".to_owned())
                }
            };
            Ok(ParsedDocument {
                quests: parse_quest_list(records)?,
                excluded_quests: excluded,
                enrollment_blocked_until: blocked,
            })
        }
        quest @ Value::Object(_) => Ok(ParsedDocument {
            quests: vec![parse_full_quest(quest, 0)?],
            excluded_quests: Vec::new(),
            enrollment_blocked_until: None,
        }),
        _ => Err("Expected a Quest object, array of Quests, or {\"quests\":[...]}".to_owned()),
    }
}

/// Compatibility helper: callers needing only full campaigns can keep using
/// the historical tuple API; CLI uses parse_document to retain excluded data.
pub fn parse_quests(input: &str) -> Result<(Vec<Quest>, Option<String>), String> {
    let document = parse_document(input)?;
    Ok((document.quests, document.enrollment_blocked_until))
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
        assert_eq!(quests.len(), 1); // Identical IDs deduplicate.
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
    fn leading_utf8_bom_is_accepted() {
        let (records, _) = parse_quests("\u{feff}{\"id\":\"bom-1\",\"config\":{}}").unwrap();
        assert_eq!(records[0].id, "bom-1");
    }

    #[test]
    fn excessive_json_nesting_is_rejected_without_panicking() {
        let deep = format!("{}0{}", "[".repeat(140), "]".repeat(140));
        assert!(parse_quests(&deep).is_err());
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
    fn duplicate_quest_ids_are_not_double_counted() {
        let repeated = json!([fixture(), fixture()]);
        let (result, _) = parse_quests(&repeated.to_string()).unwrap();
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn conflicting_duplicate_quest_ids_fail_closed() {
        let a = fixture();
        let mut b = fixture();
        b["user_status"]["completed_at"] = json!("2026-11-01T11:30:00Z");
        let err = parse_quests(&json!([a, b]).to_string()).unwrap_err();
        assert!(err.contains("Conflicting records"));
        assert!(err.contains("index 1"));
    }

    #[test]
    fn missing_or_invalid_expiration_is_not_eligible() {
        for expiry in [Value::Null, json!("invalid-expiry")] {
            let mut q = fixture();
            q["config"]["expires_at"] = expiry;
            let quest = decode(q);
            assert_eq!(assess_quest(&quest, &now()).verdict, Verdict::Unsupported);
            assert_eq!(
                QuestManager::new(vec![quest], None)
                    .filter_valid(now())
                    .len(),
                0
            );
        }
    }

    #[test]
    fn contradictory_milestone_ordering_is_inconsistent() {
        for (first, second) in [
            ("enrolled_at", "completed_at"),
            ("completed_at", "claimed_at"),
        ] {
            let mut q = fixture();
            q["user_status"][first] = json!("2026-11-01T12:00:00Z");
            q["user_status"][second] = json!("2026-11-01T11:00:00Z");
            assert_eq!(
                assess_quest(&decode(q), &now()).verdict,
                Verdict::Inconsistent
            );
        }
        let mut q = fixture();
        q["config"]["starts_at"] = json!("2027-12-01T00:00:00Z");
        assert_eq!(
            assess_quest(&decode(q), &now()).verdict,
            Verdict::Inconsistent
        );
    }

    #[test]
    fn conflicting_status_or_config_quest_id_is_inconsistent() {
        let mut q = fixture();
        q["config"]["id"] = json!("wrong-id");
        q["user_status"]["completed_at"] = json!("2026-11-01T11:30:00Z");
        assert_eq!(
            assess_quest(&decode(q), &now()).verdict,
            Verdict::Inconsistent
        );
        let mut q = fixture();
        q["user_status"]["quest_id"] = json!("other-id");
        assert_eq!(
            assess_quest(&decode(q), &now()).verdict,
            Verdict::Inconsistent
        );
    }

    #[test]
    fn invalid_numeric_progress_or_target_has_no_numeric_confirmation() {
        let mut q = fixture();
        q["config"]["task_config_v2"]["tasks"]["WATCH_VIDEO"]["target"] = json!(-5);
        q["user_status"]["progress"]["PLAY_ACTIVITY"]["value"] = json!(-10);
        let outcome = assess_quest(&decode(q), &now());
        assert!(outcome
            .tasks
            .iter()
            .all(|task| task.numeric_goal_reached != Some(true)));
        assert!(outcome
            .warnings
            .iter()
            .any(|w| w.contains("invalid target")));
        assert!(outcome
            .warnings
            .iter()
            .any(|w| w.contains("invalid progress")));
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
