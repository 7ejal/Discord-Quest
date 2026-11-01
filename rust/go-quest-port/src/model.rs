//! Typed Rust equivalents of the relevant Go Quest data structures.
//! Only local JSON parsing. No personal account authentication or network calls.
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct AllQuestsResponse {
    #[serde(default)]
    pub quests: Vec<Quest>,
    #[serde(default)]
    pub excluded_quests: Vec<QuestPartial>,
    #[serde(default)]
    pub quest_enrollment_blocked_until: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct QuestPartial {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub config: Option<QuestConfig>,
    #[serde(default)]
    pub user_status: Option<QuestUserStatus>,
    #[serde(default)]
    pub targeted_content: Option<Value>,
    #[serde(default)]
    pub preview: bool,
    /// Potentially sensitive targeting metadata; never echo it in CLI reports.
    #[serde(default)]
    pub traffic_metadata_raw: Option<String>,
    #[serde(default)]
    pub traffic_metadata_sealed: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Quest {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub config: QuestConfig,
    #[serde(default)]
    pub user_status: Option<QuestUserStatus>,
    #[serde(default)]
    pub preview: bool,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct QuestConfig {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub config_version: Option<i64>,
    #[serde(default)]
    pub starts_at: Option<String>,
    #[serde(default)]
    pub expires_at: Option<String>,
    #[serde(default)]
    pub messages: QuestMessages,
    #[serde(default)]
    pub application: QuestApplication,
    #[serde(default)]
    pub task_config_v2: Option<TaskConfig>,
    #[serde(default)]
    pub task_config: Option<TaskConfig>,
    #[serde(default)]
    pub rewards_config: RewardsConfig,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct QuestMessages {
    #[serde(default)]
    pub quest_name: String,
    #[serde(default)]
    pub game_title: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct QuestApplication {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct TaskConfig {
    #[serde(default)]
    pub join_operator: Option<String>,
    #[serde(default)]
    pub tasks: BTreeMap<String, TaskEntry>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct TaskEntry {
    /// The Go source carries its own task type in addition to the task-map key.
    #[serde(default, rename = "type")]
    pub task_type: Option<String>,
    #[serde(default)]
    pub target: Option<f64>,
    #[serde(default)]
    pub event_name: Option<String>,
    #[serde(default)]
    pub applications: Vec<QuestApplication>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct RewardsConfig {
    #[serde(default)]
    pub assignment_method: Option<i64>,
    #[serde(default)]
    pub rewards: Vec<Reward>,
    #[serde(default)]
    pub rewards_expire_at: Option<String>,
    #[serde(default)]
    pub platforms: Vec<i64>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Reward {
    #[serde(default)]
    pub r#type: Option<i64>,
    #[serde(default)]
    pub sku_id: Option<String>,
    #[serde(default)]
    pub asset: Option<String>,
    #[serde(default)]
    pub asset_video: Option<String>,
    #[serde(default)]
    pub messages: RewardMessages,
    /// These optional quantity keys are extensions, not fields in pinned Go QuestReward.
    #[serde(default)]
    pub orb_quantity: Option<i64>,
    #[serde(default)]
    pub premium_orb_quantity: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct RewardMessages {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub name_with_article: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct QuestUserStatus {
    /// Optional Quest ID on source status; must agree with the parent Quest.
    #[serde(default)]
    pub quest_id: Option<String>,
    #[serde(default)]
    pub enrolled_at: Option<String>,
    #[serde(default)]
    pub completed_at: Option<String>,
    #[serde(default)]
    pub claimed_at: Option<String>,
    #[serde(default)]
    pub progress: BTreeMap<String, TaskProgress>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct TaskProgress {
    #[serde(default)]
    pub event_name: Option<String>,
    #[serde(default)]
    pub value: Option<f64>,
    #[serde(default)]
    pub updated_at: Option<String>,
    #[serde(default)]
    pub completed_at: Option<String>,
    #[serde(default)]
    pub heartbeat: Option<TaskHeartbeat>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct TaskHeartbeat {
    #[serde(default)]
    pub last_beat_at: Option<String>,
    #[serde(default)]
    pub expires_at: Option<String>,
}

/// True only for a syntactically valid RFC3339 timestamp supplied in the
/// input record. This does NOT authenticate the JSON's source.
pub fn has_timestamp(timestamp: &Option<String>) -> bool {
    timestamp
        .as_deref()
        .is_some_and(|raw| DateTime::parse_from_rfc3339(raw).is_ok())
}

/// Warn about non-empty but malformed date strings rather than treating them
/// as server-confirmed milestone evidence.
pub fn invalid_timestamp(timestamp: &Option<String>) -> bool {
    timestamp
        .as_ref()
        .is_some_and(|_| !has_timestamp(timestamp))
}

/// Expiration cannot be assumed valid when the supplied file is incomplete.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpiryState {
    Missing,
    Invalid,
    Active,
    Expired,
}

impl Quest {
    pub fn name(&self) -> &str {
        if !self.config.messages.quest_name.trim().is_empty() {
            &self.config.messages.quest_name
        } else {
            &self.id
        }
    }

    pub fn tasks(&self) -> Option<&TaskConfig> {
        self.config
            .task_config_v2
            .as_ref()
            .or(self.config.task_config.as_ref())
    }

    pub fn is_enrolled(&self) -> bool {
        self.user_status
            .as_ref()
            .is_some_and(|s| has_timestamp(&s.enrolled_at))
    }

    pub fn is_completed(&self) -> bool {
        self.user_status
            .as_ref()
            .is_some_and(|s| has_timestamp(&s.completed_at))
    }

    pub fn has_claimed_rewards(&self) -> bool {
        self.user_status
            .as_ref()
            .is_some_and(|s| has_timestamp(&s.claimed_at))
    }

    /// Missing/invalid expiry is unknown rather than silently treated as active.
    pub fn expiry_state(&self, now: DateTime<Utc>) -> ExpiryState {
        match self.config.expires_at.as_deref() {
            None => ExpiryState::Missing,
            Some(raw) => match DateTime::parse_from_rfc3339(raw) {
                Err(_) => ExpiryState::Invalid,
                Ok(expiry) if now >= expiry.with_timezone(&Utc) => ExpiryState::Expired,
                Ok(_) => ExpiryState::Active,
            },
        }
    }

    pub fn is_expired(&self, now: DateTime<Utc>) -> bool {
        self.expiry_state(now) == ExpiryState::Expired
    }

    pub fn is_android_video_only(&self) -> bool {
        self.tasks().is_some_and(|group| {
            group.tasks.contains_key("WATCH_VIDEO_ON_MOBILE")
                && !group.tasks.contains_key("WATCH_VIDEO")
        })
    }

    /// For comparison ONLY: replicates the original Go first-match heuristic.
    /// A reliable group evaluation MUST NOT use this as a Quest completion check.
    pub fn original_go_first_task(&self) -> Option<&'static str> {
        const PRIORITY: [&str; 9] = [
            "WATCH_VIDEO",
            "PLAY_ON_DESKTOP",
            "PLAY_ON_DESKTOP_V2",
            "PLAY_ON_XBOX",
            "PLAY_ON_PLAYSTATION",
            "STREAM_ON_DESKTOP",
            "PLAY_ACTIVITY",
            "WATCH_VIDEO_ON_MOBILE",
            "ACHIEVEMENT_IN_ACTIVITY",
        ];
        let tasks = &self.tasks()?.tasks;
        PRIORITY.into_iter().find(|task| tasks.contains_key(*task))
    }
}
