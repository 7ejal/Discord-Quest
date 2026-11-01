//! Typed Rust equivalents of the relevant Go Quest data structures.
//! Only local JSON parsing. No personal account authentication or network calls.
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

/// Recognition only: these variants never execute engagement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskKind {
    WatchVideo,
    WatchVideoOnMobile,
    PlayOnDesktop,
    PlayOnDesktopV2,
    StreamOnDesktop,
    PlayActivity,
    PlayOnXbox,
    PlayOnPlaystation,
    AchievementInActivity,
    AchievementInGame,
    Unknown(String),
}

impl From<&str> for TaskKind {
    fn from(name: &str) -> Self {
        match name {
            "WATCH_VIDEO" => Self::WatchVideo,
            "WATCH_VIDEO_ON_MOBILE" => Self::WatchVideoOnMobile,
            "PLAY_ON_DESKTOP" => Self::PlayOnDesktop,
            "PLAY_ON_DESKTOP_V2" => Self::PlayOnDesktopV2,
            "STREAM_ON_DESKTOP" => Self::StreamOnDesktop,
            "PLAY_ACTIVITY" => Self::PlayActivity,
            "PLAY_ON_XBOX" => Self::PlayOnXbox,
            "PLAY_ON_PLAYSTATION" => Self::PlayOnPlaystation,
            "ACHIEVEMENT_IN_ACTIVITY" => Self::AchievementInActivity,
            "ACHIEVEMENT_IN_GAME" => Self::AchievementInGame,
            _ => Self::Unknown(name.to_owned()),
        }
    }
}

impl TaskKind {
    pub fn family(&self) -> &'static str {
        match self {
            Self::WatchVideo | Self::WatchVideoOnMobile => "video",
            Self::PlayOnDesktop => "desktop_game",
            Self::PlayOnDesktopV2 => "historical_deprecated",
            Self::StreamOnDesktop => "desktop_stream",
            Self::PlayActivity => "embedded_activity",
            Self::PlayOnXbox | Self::PlayOnPlaystation => "linked_console",
            Self::AchievementInActivity | Self::AchievementInGame => {
                "publisher_or_activity_achievement"
            }
            Self::Unknown(_) => "unknown",
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AllQuestsResponse {
    #[serde(default)]
    pub quests: Vec<Quest>,
    #[serde(default)]
    pub excluded_quests: Vec<QuestPartial>,
    #[serde(default)]
    pub quest_enrollment_blocked_until: Option<String>,
    /// Retained for library users; never included in CLI summaries.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
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
    /// Retained for library users; never included in CLI summaries.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Quest {
    #[serde(skip)]
    pub field_warnings: Vec<String>,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub config: QuestConfig,
    #[serde(default)]
    pub user_status: Option<QuestUserStatus>,
    #[serde(default)]
    pub preview: bool,
    /// Retained for library users; never included in CLI summaries.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QuestConfig {
    #[serde(default)]
    pub features: Option<Value>,
    #[serde(default)]
    pub assets: Option<Value>,
    #[serde(default)]
    pub colors: Option<Value>,
    #[serde(default)]
    pub video_metadata: Option<Value>,
    #[serde(default, rename = "cosponsor_metadata")]
    pub cosponsor_metadata: Option<Value>,
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
    /// Retained for library users; never included in CLI summaries.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QuestMessages {
    #[serde(default)]
    pub quest_name: String,
    #[serde(default)]
    pub game_title: String,
    /// Retained for library users; never included in CLI summaries.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QuestApplication {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    /// Retained for library users; never included in CLI summaries.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaskConfig {
    #[serde(default)]
    pub join_operator: Option<String>,
    #[serde(default)]
    pub tasks: BTreeMap<String, TaskEntry>,
    /// Retained for library users; never included in CLI summaries.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
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
    /// Retained for library users; never included in CLI summaries.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RewardsConfig {
    #[serde(default)]
    pub assignment_method: Option<i64>,
    #[serde(default)]
    pub rewards: Vec<Reward>,
    #[serde(default)]
    pub rewards_expire_at: Option<String>,
    #[serde(default)]
    pub platforms: Vec<i64>,
    /// Retained for library users; never included in CLI summaries.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
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
    pub messages: Option<RewardMessages>,
    /// These optional quantity keys are extensions, not fields in pinned Go QuestReward.
    #[serde(default)]
    pub orb_quantity: Option<i64>,
    #[serde(default)]
    pub premium_orb_quantity: Option<i64>,
    /// Retained for library users; never included in CLI summaries.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RewardMessages {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub name_with_article: String,
    /// Retained for library users; never included in CLI summaries.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
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
    /// Retained for library users; never included in CLI summaries.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
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
    /// Retained for library users; never included in CLI summaries.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaskHeartbeat {
    #[serde(default)]
    pub last_beat_at: Option<String>,
    #[serde(default)]
    pub expires_at: Option<String>,
    /// Retained for library users; never included in CLI summaries.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
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
