//! Read-only, offline Quest status analysis.
//!
//! This library NEVER connects to Discord, uses personal account credentials,
//! or submits Quest progress. Its results are diagnostic, not reward claims.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Debug, Default, Deserialize)]
pub struct Quest {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub config: QuestConfig,
    #[serde(default)]
    pub user_status: Option<UserStatus>,
}

#[derive(Debug, Default, Deserialize)]
pub struct QuestConfig {
    #[serde(default)]
    pub messages: QuestMessages,
    #[serde(default)]
    pub task_config_v2: Option<TaskGroup>,
    #[serde(default)]
    pub task_config: Option<TaskGroup>,
}

#[derive(Debug, Default, Deserialize)]
pub struct QuestMessages {
    #[serde(default)]
    pub quest_name: String,
}

#[derive(Debug, Default, Deserialize)]
pub struct TaskGroup {
    #[serde(default)]
    pub join_operator: Option<String>,
    #[serde(default)]
    pub tasks: BTreeMap<String, TaskDefinition>,
}

#[derive(Debug, Default, Deserialize)]
pub struct TaskDefinition {
    #[serde(default)]
    pub target: Option<f64>,
}

#[derive(Debug, Default, Deserialize)]
pub struct UserStatus {
    #[serde(default)]
    pub enrolled_at: Option<String>,
    #[serde(default)]
    pub completed_at: Option<String>,
    #[serde(default)]
    pub claimed_at: Option<String>,
    #[serde(default)]
    pub progress: BTreeMap<String, TaskProgress>,
}

#[derive(Debug, Default, Deserialize)]
pub struct TaskProgress {
    #[serde(default)]
    pub value: Option<f64>,
    #[serde(default)]
    pub completed_at: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Verdict {
    NotEnrolled,
    InProgress,
    TasksConfirmedQuestPending,
    QuestConfirmedUnclaimed,
    RewardClaimed,
    Unsupported,
    Inconsistent,
}

#[derive(Debug, Serialize)]
pub struct TaskAssessment {
    pub task_type: String,
    pub type_classification: &'static str,
    pub target: Option<f64>,
    pub reported_value: Option<f64>,
    pub numeric_target_reached: Option<bool>,
    pub server_task_completed: bool,
}

#[derive(Debug, Serialize)]
pub struct Assessment {
    pub quest_id: String,
    pub quest_name: String,
    pub verdict: Verdict,
    pub join_operator: String,
    pub tasks: Vec<TaskAssessment>,
    pub notes: Vec<String>,
}

fn present(value: &Option<String>) -> bool {
    value.as_ref().is_some_and(|s| !s.trim().is_empty())
}

fn classify_task(name: &str) -> &'static str {
    match name {
        "WATCH_VIDEO" | "WATCH_VIDEO_ON_MOBILE" | "PLAY_ON_DESKTOP"
        | "STREAM_ON_DESKTOP" | "PLAY_ACTIVITY"
        | "ACHIEVEMENT_IN_ACTIVITY" | "ACHIEVEMENT_IN_GAME"
        | "PLAY_ON_XBOX" | "PLAY_ON_PLAYSTATION" => "recognized",
        "PLAY_ON_DESKTOP_V2" => "historical",
        _ => "unknown",
    }
}

/// Interpret a server-shaped record without causing any network action.
/// Only a Quest-level completed_at establishes server-confirmed completion.
/// A task's progress or timestamp alone does not.
pub fn assess_quest(quest: &Quest) -> Assessment {
    let status = quest.user_status.as_ref();
    let completed = status.is_some_and(|s| present(&s.completed_at));
    let claimed = status.is_some_and(|s| present(&s.claimed_at));
    let enrolled = status.is_some_and(|s| present(&s.enrolled_at));

    let group = quest.config.task_config_v2.as_ref()
        .or(quest.config.task_config.as_ref());
    let mut tasks = Vec::new();
    let mut notes = Vec::new();

    if let Some(group) = group {
        for (name, definition) in &group.tasks {
            let progress = status.and_then(|s| s.progress.get(name));
            let value = progress.and_then(|p| p.value);
            let numeric_target_reached = match (value, definition.target) {
                (Some(v), Some(t)) if v.is_finite() && t.is_finite() && t > 0.0 => {
                    Some(v >= t)
                }
                _ => None,
            };
            tasks.push(TaskAssessment {
                task_type: name.clone(),
                type_classification: classify_task(name),
                target: definition.target,
                reported_value: value,
                numeric_target_reached,
                server_task_completed: progress.is_some_and(|p| present(&p.completed_at)),
            });
        }
    }

    let operator = match group {
        None => "missing".to_owned(),
        Some(g) if g.tasks.len() == 1 => "single".to_owned(),
        Some(g) => g.join_operator.as_ref()
            .map(|op| op.trim().to_ascii_lowercase())
            .unwrap_or_else(|| "missing".to_owned()),
    };

    let unknown_types = tasks.iter().any(|t| t.type_classification == "unknown");
    if unknown_types {
        notes.push("One or more unrecognized task types; no handler support is implied.".into());
    }
    if tasks.iter().any(|t| t.numeric_target_reached == Some(true) && !t.server_task_completed) {
        notes.push("A numeric target was reached without server task completion confirmation.".into());
    }
    if claimed && !completed {
        notes.push("Reward marked claimed but Quest completed_at is missing.".into());
    }

    let verdict = if claimed && !completed {
        Verdict::Inconsistent
    } else if completed {
        if claimed { Verdict::RewardClaimed } else { Verdict::QuestConfirmedUnclaimed }
    } else if !enrolled {
        Verdict::NotEnrolled
    } else if group.is_none() || tasks.is_empty() || unknown_types {
        notes.push("Task schema is absent, empty, or contains unknown task types.".into());
        Verdict::Unsupported
    } else {
        let complete = match operator.as_str() {
            "single" | "and" => tasks.iter().all(|t| t.server_task_completed),
            "or" => tasks.iter().any(|t| t.server_task_completed),
            _ => {
                notes.push("Unknown multi-task join_operator; refusing to infer success.".into());
                return Assessment {
                    quest_id: quest.id.clone(),
                    quest_name: quest_name(quest),
                    verdict: Verdict::Unsupported,
                    join_operator: operator,
                    tasks,
                    notes,
                };
            }
        };
        if complete {
            Verdict::TasksConfirmedQuestPending
        } else {
            Verdict::InProgress
        }
    };

    Assessment {
        quest_id: quest.id.clone(),
        quest_name: quest_name(quest),
        verdict,
        join_operator: operator,
        tasks,
        notes,
    }
}

fn quest_name(quest: &Quest) -> String {
    if quest.config.messages.quest_name.trim().is_empty() {
        quest.id.clone()
    } else {
        quest.config.messages.quest_name.clone()
    }
}

/// Accept a single Quest object, an array, or an object with a quests array.
pub fn assess_document(document: Value) -> Result<Vec<Assessment>, String> {
    let quest_values = match document {
        Value::Array(quests) => quests,
        Value::Object(mut fields) if fields.contains_key("quests") => {
            match fields.remove("quests") {
                Some(Value::Array(quests)) => quests,
                _ => return Err("The quests field must be an array".into()),
            }
        }
        Value::Object(fields) => vec![Value::Object(fields)],
        _ => return Err("Expected a Quest object, array, or object with quests array".into()),
    };
    let mut assessments = Vec::with_capacity(quest_values.len());
    for (index, raw) in quest_values.into_iter().enumerate() {
        let quest: Quest = serde_json::from_value(raw)
            .map_err(|error| format!("Quest at index {index} has invalid JSON fields: {error}"))?;
        assessments.push(assess_quest(&quest));
    }
    Ok(assessments)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn base() -> Value {
        json!({
            "id": "synthetic-001",
            "config": {
                "messages": { "quest_name": "Mock Quest" },
                "task_config_v2": {
                    "join_operator": "and",
                    "tasks": {
                        "WATCH_VIDEO": { "target": 60 },
                        "PLAY_ACTIVITY": { "target": 120 }
                    }
                }
            },
            "user_status": {
                "enrolled_at": "2026-11-01T00:00:00Z",
                "completed_at": null,
                "claimed_at": null,
                "progress": {
                    "WATCH_VIDEO": {
                        "value": 60, "completed_at": "2026-11-01T00:01:00Z"
                    },
                    "PLAY_ACTIVITY": {
                        "value": 0, "completed_at": null
                    }
                }
            }
        })
    }

    fn verdict(value: Value) -> Verdict {
        let quest: Quest = serde_json::from_value(value).expect("synthetic quest JSON");
        assess_quest(&quest).verdict
    }

    #[test]
    fn and_does_not_complete_when_one_task_is_missing() {
        assert_eq!(verdict(base()), Verdict::InProgress);
    }

    #[test]
    fn and_all_tasks_remain_pending_quest_confirmation() {
        let mut q = base();
        q["user_status"]["progress"]["PLAY_ACTIVITY"]["completed_at"] =
            json!("2026-11-01T00:02:00Z");
        assert_eq!(verdict(q), Verdict::TasksConfirmedQuestPending);
    }

    #[test]
    fn or_one_task_does_not_imply_quest_completed() {
        let mut q = base();
        q["config"]["task_config_v2"]["join_operator"] = json!("or");
        assert_eq!(verdict(q), Verdict::TasksConfirmedQuestPending);
    }

    #[test]
    fn numeric_target_without_task_completed_at_is_insufficient() {
        let mut q = base();
        q["user_status"]["progress"]["WATCH_VIDEO"]["completed_at"] = Value::Null;
        q["user_status"]["progress"]["PLAY_ACTIVITY"]["value"] = json!(120);
        assert_eq!(verdict(q), Verdict::InProgress);
    }

    #[test]
    fn quest_level_completion_is_distinct_from_claiming() {
        let mut q = base();
        q["user_status"]["completed_at"] = json!("2026-11-01T00:02:00Z");
        assert_eq!(verdict(q.clone()), Verdict::QuestConfirmedUnclaimed);
        q["user_status"]["claimed_at"] = json!("2026-11-01T00:03:00Z");
        assert_eq!(verdict(q), Verdict::RewardClaimed);
    }

    #[test]
    fn claimed_without_completed_is_inconsistent() {
        let mut q = base();
        q["user_status"]["claimed_at"] = json!("2026-11-01T00:03:00Z");
        assert_eq!(verdict(q), Verdict::Inconsistent);
    }

    #[test]
    fn unknown_join_operator_fails_closed() {
        let mut q = base();
        q["config"]["task_config_v2"]["join_operator"] = json!("xor");
        assert_eq!(verdict(q), Verdict::Unsupported);
    }

    #[test]
    fn no_enrollment_means_not_enrolled() {
        let mut q = base();
        q["user_status"]["enrolled_at"] = Value::Null;
        assert_eq!(verdict(q), Verdict::NotEnrolled);
    }

    #[test]
    fn legacy_schema_can_be_read() {
        let mut q = base();
        q["config"]["task_config"] = q["config"]["task_config_v2"].clone();
        q["config"]["task_config_v2"] = Value::Null;
        assert_eq!(verdict(q), Verdict::InProgress);
    }

    #[test]
    fn unknown_task_fails_closed_without_quest_completion() {
        let mut q = base();
        q["config"]["task_config_v2"]["tasks"] =
            json!({"NEW_TASK_ID": {"target": 1}});
        assert_eq!(verdict(q), Verdict::Unsupported);
    }

    #[test]
    fn absent_task_config_is_unsupported() {
        let mut q = base();
        q["config"]["task_config_v2"] = Value::Null;
        assert_eq!(verdict(q), Verdict::Unsupported);
    }

    #[test]
    fn document_supports_wrapped_quest_list() {
        let result = assess_document(json!({"quests": [base(), base()]}))
            .expect("valid synthetic list");
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].verdict, Verdict::InProgress);
    }

    #[test]
    fn wrapped_field_rejects_non_array() {
        let result = assess_document(json!({"quests": "not an array"}));
        assert!(result.is_err());
    }

    #[test]
    fn empty_status_does_not_invent_enrollment() {
        let mut q = base();
        q["user_status"] = Value::Null;
        assert_eq!(verdict(q), Verdict::NotEnrolled);
    }
}
