//! Port of the Go QuestManager / FilterValid / FilterRedeemable logic.
//! Adds local-input status interpretation and AND/OR group evaluation.
use crate::model::{has_timestamp, invalid_timestamp, ExpiryState, Quest, TaskConfig};
use chrono::{DateTime, Utc};
use serde::Serialize;
use std::thread;

#[derive(Debug)]
pub struct QuestManager {
    quests: Vec<Quest>,
    pub enrollment_blocked_until: Option<String>,
}

impl QuestManager {
    pub fn new(quests: Vec<Quest>, enrollment_blocked_until: Option<String>) -> Self {
        Self {
            quests,
            enrollment_blocked_until,
        }
    }

    pub fn quests(&self) -> &[Quest] {
        &self.quests
    }

    /// Matches the Go manager's intent without making enrollment-block state
    /// a reason to hide already-enrolled or locally inspectable quests.
    pub fn filter_valid(&self, now: DateTime<Utc>) -> Vec<&Quest> {
        self.quests
            .iter()
            .filter(|quest| !quest.is_completed() && quest.expiry_state(now) == ExpiryState::Active)
            .collect()
    }

    pub fn filter_redeemable(&self) -> Vec<&Quest> {
        self.quests
            .iter()
            .filter(|quest| quest.is_completed() && !quest.has_claimed_rewards())
            .collect()
    }

    pub fn assess_all(&self, now: DateTime<Utc>, workers: usize) -> Vec<QuestAssessment> {
        analyze_parallel(&self.quests, now, workers)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Verdict {
    Expired,
    NotStarted,
    NotEnrolled,
    InProgress,
    TasksCompleteQuestUnconfirmed,
    QuestConfirmedUnclaimed,
    RewardClaimed,
    Unsupported,
    Inconsistent,
}

#[derive(Debug, Serialize)]
pub struct TaskAssessment {
    pub task_type: String,
    pub family: &'static str,
    pub goal: Option<f64>,
    pub reported_progress: Option<f64>,
    pub numeric_goal_reached: Option<bool>,
    pub server_task_completed: bool,
    pub application_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct QuestAssessment {
    pub quest_id: String,
    pub name: String,
    pub verdict: Verdict,
    pub join_operator: String,
    pub enrolled: bool,
    pub quest_completed: bool,
    pub rewards_claimed: bool,
    pub tasks: Vec<TaskAssessment>,
    pub warnings: Vec<String>,
}

pub fn task_family(name: &str) -> &'static str {
    match name {
        "WATCH_VIDEO" | "WATCH_VIDEO_ON_MOBILE" => "video",
        "PLAY_ON_DESKTOP" => "desktop_game",
        "STREAM_ON_DESKTOP" => "desktop_stream",
        "PLAY_ACTIVITY" => "embedded_activity",
        "PLAY_ON_XBOX" | "PLAY_ON_PLAYSTATION" => "linked_console",
        "ACHIEVEMENT_IN_GAME" | "ACHIEVEMENT_IN_ACTIVITY" => "publisher_or_activity_achievement",
        "PLAY_ON_DESKTOP_V2" => "historical_deprecated",
        _ => "unknown",
    }
}

fn parse_time(raw: &Option<String>) -> Option<DateTime<Utc>> {
    raw.as_deref()
        .and_then(|time| DateTime::parse_from_rfc3339(time).ok())
        .map(|date| date.with_timezone(&Utc))
}

fn join_operator(group: Option<&TaskConfig>) -> String {
    match group {
        None => "missing".to_owned(),
        Some(g) if g.tasks.len() == 1 => "single".to_owned(),
        Some(g) => g
            .join_operator
            .as_deref()
            .unwrap_or("missing")
            .trim()
            .to_ascii_lowercase(),
    }
}

/// Inspect only previously available JSON. Never sends task progress or claims.
pub fn assess_quest(quest: &Quest, now: &DateTime<Utc>) -> QuestAssessment {
    let group = quest.tasks();
    let status = quest.user_status.as_ref();
    let enrolled = quest.is_enrolled();
    let quest_completed = quest.is_completed();
    let rewards_claimed = quest.has_claimed_rewards();
    let mut warnings = Vec::new();
    let mut tasks = Vec::new();

    if let Some(group) = group {
        for (name, def) in &group.tasks {
            let progress = status.and_then(|s| s.progress.get(name));
            let reported = progress.and_then(|p| p.value);
            let numeric_goal_reached = match (reported, def.target) {
                (Some(value), Some(target))
                    if target > 0.0 && value >= 0.0 && target.is_finite() && value.is_finite() =>
                {
                    Some(value >= target)
                }
                _ => None,
            };
            if def.target.is_some_and(|target| !target.is_finite() || target <= 0.0) {
                warnings.push(format!("Input has invalid target for task {name}."));
            }
            if reported.is_some_and(|value| !value.is_finite() || value < 0.0) {
                warnings.push(format!("Input has invalid progress for task {name}."));
            }
            let server_task_completed = progress.is_some_and(|p| has_timestamp(&p.completed_at));
            let application_id = def
                .applications
                .iter()
                .find(|a| !a.id.is_empty())
                .map(|a| a.id.clone())
                .or_else(|| {
                    (!quest.config.application.id.is_empty())
                        .then(|| quest.config.application.id.clone())
                });

            tasks.push(TaskAssessment {
                task_type: name.clone(),
                family: task_family(name),
                goal: def.target,
                reported_progress: reported,
                numeric_goal_reached,
                server_task_completed,
                application_id,
            });
        }
    }

    if let Some(record) = status {
        for (name, field) in [
            ("enrolled_at", &record.enrolled_at),
            ("completed_at", &record.completed_at),
            ("claimed_at", &record.claimed_at),
        ] {
            if invalid_timestamp(field) {
                warnings.push(format!("Input has invalid {name} timestamp."));
            }
        }
        for (name, progress) in &record.progress {
            if invalid_timestamp(&progress.completed_at) {
                warnings.push(format!("Input has invalid timestamp for task {name}."));
            }
        }
    }

    if tasks
        .iter()
        .any(|task| task.numeric_goal_reached == Some(true) && !task.server_task_completed)
    {
        warnings.push(
            "Numeric task progress reached its target without a task completion timestamp."
                .to_owned(),
        );
    }
    if tasks.iter().any(|task| task.family == "unknown") {
        warnings.push("The Quest contains an unrecognized task type.".to_owned());
    }
    if tasks
        .iter()
        .any(|task| task.family == "historical_deprecated")
    {
        warnings.push("The Quest uses a deprecated desktop-play task identifier.".to_owned());
    }
    if rewards_claimed && !quest_completed {
        warnings.push("Claimed timestamp exists without Quest completion timestamp.".to_owned());
    }
    if !quest_completed
        && tasks.iter().any(|task| {
            matches!(
                task.family,
                "desktop_stream" | "linked_console" | "publisher_or_activity_achievement"
            )
        })
    {
        warnings.push(
            "Some tasks require authentic streaming, console, or application-provider activity."
                .to_owned(),
        );
    }

    let operator = join_operator(group);
    let expiry_state = quest.expiry_state(*now);
    match expiry_state {
        ExpiryState::Missing => warnings.push("Input has missing expires_at; eligibility is unknown.".to_owned()),
        ExpiryState::Invalid => warnings.push("Input has invalid expires_at; eligibility is unknown.".to_owned()),
        _ => {}
    }
    if invalid_timestamp(&quest.config.starts_at) {
        warnings.push("Input has invalid starts_at timestamp.".to_owned());
    }
    let start = parse_time(&quest.config.starts_at);
    let end = parse_time(&quest.config.expires_at);
    let not_started = start.is_some_and(|date| now < &date);
    let inconsistent_id = (!quest.config.id.trim().is_empty() && quest.config.id != quest.id)
        || status.is_some_and(|record| {
            record.quest_id.as_ref().is_some_and(|id| id.trim().is_empty() || id != &quest.id)
        });
    if inconsistent_id {
        warnings.push("Quest ID disagrees with config or user_status Quest ID.".to_owned());
    }
    let invalid_order = start.zip(end).is_some_and(|(start, end)| start > end)
        || status.is_some_and(|record| {
            let enrolled = parse_time(&record.enrolled_at);
            let completed = parse_time(&record.completed_at);
            let claimed = parse_time(&record.claimed_at);
            enrolled.zip(completed).is_some_and(|(a, b)| a > b)
                || completed.zip(claimed).is_some_and(|(a, b)| a > b)
        });
    if invalid_order {
        warnings.push("Input contains contradictory timestamp ordering.".to_owned());
    }

    let verdict = if inconsistent_id || invalid_order || (rewards_claimed && !quest_completed) {
        Verdict::Inconsistent
    } else if quest_completed {
        if rewards_claimed {
            Verdict::RewardClaimed
        } else {
            Verdict::QuestConfirmedUnclaimed
        }
    } else if expiry_state == ExpiryState::Expired {
        Verdict::Expired
    } else if matches!(expiry_state, ExpiryState::Missing | ExpiryState::Invalid)
        || invalid_timestamp(&quest.config.starts_at) {
        Verdict::Unsupported
    } else if not_started {
        Verdict::NotStarted
    } else if !enrolled {
        Verdict::NotEnrolled
    } else if tasks.is_empty() || tasks.iter().any(|t| t.family == "unknown") {
        Verdict::Unsupported
    } else {
        let group_satisfied = match operator.as_str() {
            "single" | "and" => Some(tasks.iter().all(|t| t.server_task_completed)),
            "or" => Some(tasks.iter().any(|t| t.server_task_completed)),
            _ => None,
        };
        match group_satisfied {
            None => {
                warnings.push("Missing/unknown join_operator for multi-task Quest.".to_owned());
                Verdict::Unsupported
            }
            Some(true) => Verdict::TasksCompleteQuestUnconfirmed,
            Some(false) => Verdict::InProgress,
        }
    };

    QuestAssessment {
        quest_id: quest.id.clone(),
        name: quest.name().to_owned(),
        verdict,
        join_operator: operator,
        enrolled,
        quest_completed,
        rewards_claimed,
        tasks,
        warnings,
    }
}

/// Bounded parallel analysis, preserving original input ordering.
/// The original Go code starts one goroutine per Quest; this adapts that
/// orchestration to bounded Rust scoped threads to prevent unlimited concurrency.
pub fn analyze_parallel(
    quests: &[Quest],
    now: DateTime<Utc>,
    workers: usize,
) -> Vec<QuestAssessment> {
    if quests.is_empty() {
        return Vec::new();
    }
    let n = workers.clamp(1, 32).min(quests.len());
    let (sender, receiver) = std::sync::mpsc::channel();
    thread::scope(|scope| {
        for worker_id in 0..n {
            let tx = sender.clone();
            scope.spawn(move || {
                for index in (worker_id..quests.len()).step_by(n) {
                    let assessment = assess_quest(&quests[index], &now);
                    if tx.send((index, assessment)).is_err() {
                        return;
                    }
                }
            });
        }
    });
    drop(sender);
    let mut indexed: Vec<_> = receiver.into_iter().collect();
    indexed.sort_by_key(|(index, _)| *index);
    indexed
        .into_iter()
        .map(|(_, assessment)| assessment)
        .collect()
}
