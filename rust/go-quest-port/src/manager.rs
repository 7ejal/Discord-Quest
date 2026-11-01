//! Port of the Go QuestManager / FilterValid / FilterRedeemable logic.
//! Adds local-input status interpretation and AND/OR group evaluation.
use crate::model::{has_timestamp, invalid_timestamp, ExpiryState, Quest, TaskConfig, TaskKind};
use chrono::{DateTime, Utc};
use serde::Serialize;
use std::{sync::atomic::{AtomicBool, Ordering}, thread};

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
    pub input_trust: &'static str,
    pub task_type: String,
    pub family: &'static str,
    pub goal: Option<f64>,
    pub reported_progress: Option<f64>,
    pub numeric_goal_reached: Option<bool>,
    pub server_task_completed: bool,
    pub application_id: Option<String>,
    /// Values below are reported by the supplied local file, not authenticated.
    pub reported_event_name: Option<String>,
    pub progress_updated_at: Option<String>,
    pub heartbeat_last_beat_at: Option<String>,
    pub heartbeat_expires_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct QuestAssessment {
    pub input_trust: &'static str,
    pub external_redemption: &'static str,
    pub reward_claim_availability: &'static str,
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
    TaskKind::from(name).family()
}

fn parse_time(raw: &Option<String>) -> Option<DateTime<Utc>> {
    raw.as_deref()
        .and_then(|time| DateTime::parse_from_rfc3339(time).ok())
        .map(|date| date.with_timezone(&Utc))
}

fn join_operator(group: Option<&TaskConfig>) -> String {
    match group {
        None => "missing".to_owned(),
        Some(g) if g.tasks.len() == 1 && g.join_operator.is_none() => "single".to_owned(),
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
    let mut warnings = quest.field_warnings.clone();
    let mut invalid_numbers = false;
    let mut tasks = Vec::new();
    let mut task_type_mismatch = false;
    let mut event_mismatch = false;

    if let Some(group) = group {
        for (name, def) in &group.tasks {
            if let Some(declared_type) = def.task_type.as_deref() {
                if !declared_type.is_empty() && declared_type != name {
                    warnings.push(format!("Task {name} declares a conflicting type."));
                    task_type_mismatch = true;
                }
                if !declared_type.is_empty() && task_family(declared_type) == "unknown" {
                    // Future task identifiers may be valid but unknown to this
                    // offline version. Warn without calling matching wire keys
                    // contradictory. A differing map key is inconsistent above.
                    warnings.push(format!("Task {name} has an unrecognized declared type."));
                }
            }
            let progress = status.and_then(|s| s.progress.get(name));
            if let (Some(config_event), Some(progress_event)) = (
                def.event_name.as_deref(),
                progress.and_then(|p| p.event_name.as_deref()),
            ) {
                if !config_event.is_empty()
                    && !progress_event.is_empty()
                    && config_event != progress_event
                {
                    warnings.push(format!("Task {name} has mismatched event_name values."));
                    event_mismatch = true;
                }
            }
            let reported = progress.and_then(|p| p.value);
            let numeric_goal_reached = match (reported, def.target) {
                (Some(value), Some(target))
                    if valid_number(target, false) && valid_number(value, true) =>
                {
                    Some(value >= target)
                }
                _ => None,
            };
            if def
                .target
                .is_some_and(|target| !valid_number(target, false))
            {
                warnings.push(format!("Input has invalid target for task {name}."));
                invalid_numbers = true;
            }
            if reported.is_some_and(|value| !valid_number(value, true)) {
                warnings.push(format!("Input has invalid progress for task {name}."));
                invalid_numbers = true;
            }
            let server_task_completed = progress.is_some_and(|p| has_timestamp(&p.completed_at))
                && def.target.is_some_and(|v| valid_number(v, false))
                && reported.is_none_or(|v| valid_number(v, true));
            if def.applications.iter().any(|app| app.id.trim().is_empty()) {
                warnings.push(format!("Task {name} includes a blank application ID."));
            }
            let application_id = def
                .applications
                .iter()
                .find(|a| !a.id.trim().is_empty())
                .map(|a| a.id.clone())
                .or_else(|| {
                    (!quest.config.application.id.trim().is_empty())
                        .then(|| quest.config.application.id.clone())
                });

            tasks.push(TaskAssessment {
                input_trust: "unverified_local_json",
                task_type: name.clone(),
                family: task_family(name),
                goal: def.target,
                reported_progress: reported,
                numeric_goal_reached,
                server_task_completed,
                application_id,
                reported_event_name: progress.and_then(|p| p.event_name.clone()),
                progress_updated_at: progress.and_then(|p| p.updated_at.clone()),
                heartbeat_last_beat_at: progress
                    .and_then(|p| p.heartbeat.as_ref())
                    .and_then(|h| h.last_beat_at.clone()),
                heartbeat_expires_at: progress
                    .and_then(|p| p.heartbeat.as_ref())
                    .and_then(|h| h.expires_at.clone()),
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
            for (field, timestamp) in [
                ("completed_at", &progress.completed_at),
                ("updated_at", &progress.updated_at),
            ] {
                if invalid_timestamp(timestamp) {
                    warnings.push(format!("Task {name} has invalid {field} timestamp."));
                }
            }
            if let Some(heartbeat) = &progress.heartbeat {
                for (field, timestamp) in [
                    ("heartbeat.last_beat_at", &heartbeat.last_beat_at),
                    ("heartbeat.expires_at", &heartbeat.expires_at),
                ] {
                    if invalid_timestamp(timestamp) {
                        warnings.push(format!("Task {name} has invalid {field} timestamp."));
                    }
                }
                if let Some(expiry) = parse_time(&heartbeat.expires_at) {
                    if expiry <= *now {
                        warnings.push(format!("Task {name} has an expired reported heartbeat."));
                    }
                    if parse_time(&heartbeat.last_beat_at).is_some_and(|beat| beat > expiry) {
                        warnings.push(format!(
                            "Task {name} reports heartbeat expiry before its last beat."
                        ));
                    }
                }
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
        ExpiryState::Missing => {
            warnings.push("Input has missing expires_at; eligibility is unknown.".to_owned())
        }
        ExpiryState::Invalid => {
            warnings.push("Input has invalid expires_at; eligibility is unknown.".to_owned())
        }
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
            record
                .quest_id
                .as_ref()
                .is_some_and(|id| id.trim().is_empty() || id != &quest.id)
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
                || [enrolled, completed, claimed].into_iter().flatten().any(|date| date > *now)
                || record.progress.values().any(|p| parse_time(&p.completed_at).is_some_and(|date| date > *now || enrolled.is_some_and(|e| date < e)))
        });
    if invalid_order {
        warnings.push("Input contains contradictory timestamp ordering.".to_owned());
    }

    let malformed_status = status.is_some_and(|s| {
        [&s.enrolled_at, &s.completed_at, &s.claimed_at].into_iter().any(invalid_timestamp)
            || s.progress.values().any(|p| invalid_timestamp(&p.completed_at))
    });
    if quest.config.task_config_v2.is_some() && quest.config.task_config.is_some() {
        warnings.push("Both task configs supplied; v2 takes precedence without merging.".to_owned());
    }
    if !matches!(operator.as_str(), "single" | "and" | "or") {
        warnings.push("Missing/unknown join_operator for task group; group is unsupported.".to_owned());
    }
    let verdict = if inconsistent_id
        || invalid_order
        || task_type_mismatch
        || event_mismatch
        || invalid_numbers
        || (rewards_claimed && !quest_completed)
    {
        Verdict::Inconsistent
    } else if malformed_status {
        Verdict::Unsupported
    } else if quest_completed {
        if rewards_claimed {
            Verdict::RewardClaimed
        } else {
            Verdict::QuestConfirmedUnclaimed
        }
    } else if expiry_state == ExpiryState::Expired {
        Verdict::Expired
    } else if matches!(expiry_state, ExpiryState::Missing | ExpiryState::Invalid)
        || invalid_timestamp(&quest.config.starts_at)
    {
        Verdict::Unsupported
    } else if not_started {
        Verdict::NotStarted
    } else if !enrolled {
        Verdict::NotEnrolled
    } else if tasks.is_empty() || tasks.iter().any(|t| t.family == "unknown" || t.family == "historical_deprecated") {
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
        input_trust: "unverified_local_json",
        external_redemption: "unknown",
        reward_claim_availability: if verdict == Verdict::QuestConfirmedUnclaimed { "reported_completed_unclaimed_eligibility_unverified" } else if verdict == Verdict::RewardClaimed { "reported_claimed" } else { "unknown" },
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

/// Source integer values outside IEEE-754's exact range cannot be compared reliably.
fn valid_number(value: f64, allow_zero: bool) -> bool {
    value.is_finite() && value.fract() == 0.0 && value <= 9_007_199_254_740_991.0
        && if allow_zero { value >= 0.0 } else { value > 0.0 }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnalysisError {
    Cancelled,
    WorkerPanicked,
}

/// All workers are joined before any result is returned, including error paths.
fn analyze_with<T: Send, F: Fn(usize) -> Result<T, AnalysisError> + Sync>(
    count: usize, workers: usize, evaluate: F,
) -> Result<Vec<T>, AnalysisError> {
    if count == 0 { return Ok(Vec::new()); }
    let workers = workers.clamp(1, 32).min(count);
    let batches = thread::scope(|scope| {
        let mut handles = Vec::new();
        for worker in 0..workers {
            let evaluate = &evaluate;
            handles.push(scope.spawn(move || {
                (worker..count).step_by(workers).map(|index|
                    evaluate(index).map(|value| (index, value))
                ).collect::<Result<Vec<_>, _>>()
            }));
        }
        handles.into_iter().map(|handle| handle.join()).collect::<Vec<_>>()
    });
    let mut indexed = Vec::with_capacity(count);
    for batch in batches {
        indexed.extend(batch.map_err(|_| AnalysisError::WorkerPanicked)??);
    }
    indexed.sort_by_key(|(index, _)| *index);
    Ok(indexed.into_iter().map(|(_, value)| value).collect())
}

pub fn try_analyze_parallel(
    quests: &[Quest], now: DateTime<Utc>, workers: usize, cancelled: &AtomicBool,
) -> Result<Vec<QuestAssessment>, AnalysisError> {
    if cancelled.load(Ordering::Relaxed) { return Err(AnalysisError::Cancelled); }
    analyze_with(quests.len(), workers, |index| {
        if cancelled.load(Ordering::Relaxed) { return Err(AnalysisError::Cancelled); }
        Ok(assess_quest(&quests[index], &now))
    })
}

/// Compatibility API: a failed worker panics rather than returning partial success.
pub fn analyze_parallel(quests: &[Quest], now: DateTime<Utc>, workers: usize) -> Vec<QuestAssessment> {
    try_analyze_parallel(quests, now, workers, &AtomicBool::new(false))
        .expect("offline analysis failed")
}

#[cfg(test)]
mod worker_tests {
    use super::*;
    use std::sync::atomic::AtomicUsize;

    #[test]
    fn worker_panic_returns_no_partial_success_and_joins_other_workers() {
        let finished = AtomicUsize::new(0);
        let result = analyze_with(20, 2, |index| {
            if index == 0 { panic!("synthetic worker failure"); }
            finished.fetch_add(1, Ordering::Relaxed);
            Ok(index)
        });
        assert_eq!(result, Err(AnalysisError::WorkerPanicked));
        assert_eq!(finished.load(Ordering::Relaxed), 10);
    }

    #[test]
    fn worker_error_returns_no_partial_success() {
        let result = analyze_with(20, 2, |index| {
            if index == 3 { return Err(AnalysisError::Cancelled); }
            Ok(index)
        });
        assert_eq!(result, Err(AnalysisError::Cancelled));
    }

    #[test]
    fn cancellation_is_observed_before_scheduling() {
        assert!(matches!(try_analyze_parallel(&[], Utc::now(), 32, &AtomicBool::new(true)), Err(AnalysisError::Cancelled)));
    }
}

