package main

import "encoding/json"

// AllQuestsResponse is the response from GET /quests/@me
type AllQuestsResponse struct {
	Quests                      []Quest        `json:"quests"`
	ExcludedQuests              []QuestPartial `json:"excluded_quests"`
	QuestEnrollmentBlockedUntil *string        `json:"quest_enrollment_blocked_until"`
}

// QuestPartial is a partial quest (for excluded quests)
type QuestPartial struct {
	ID                string                 `json:"id"`
	Config            *QuestConfig           `json:"config,omitempty"`
	UserStatus        *QuestUserStatus       `json:"user_status,omitempty"`
	TargetedContent   json.RawMessage        `json:"targeted_content"`
	Preview           bool                   `json:"preview"`
	TrafficMetaRaw    *string                `json:"traffic_metadata_raw,omitempty"`
	TrafficMetaSealed *string                `json:"traffic_metadata_sealed,omitempty"`
}

// Quest represents a full quest object
type Quest struct {
	ID                string           `json:"id"`
	Config            QuestConfig      `json:"config"`
	UserStatus        *QuestUserStatus `json:"user_status"`
	TargetedContent   json.RawMessage        `json:"targeted_content"`
	Preview           bool             `json:"preview"`
	TrafficMetaRaw    *string          `json:"traffic_metadata_raw,omitempty"`
	TrafficMetaSealed *string          `json:"traffic_metadata_sealed,omitempty"`
}

type QuestConfig struct {
	ID              string               `json:"id"`
	ConfigVersion   int                  `json:"config_version"`
	StartsAt        string               `json:"starts_at"`
	ExpiresAt       string               `json:"expires_at"`
	Features        json.RawMessage      `json:"features"`
	Application     QuestApplication     `json:"application"`
	Assets          QuestAssets          `json:"assets"`
	Colors          QuestGradient        `json:"colors"`
	Messages        QuestMessages        `json:"messages"`
	RewardsConfig   QuestRewardsConfig   `json:"rewards_config"`
	VideoMetadata   *QuestVideoMetadata  `json:"video_metadata,omitempty"`
	CosponsorMeta   *QuestCosponsorMeta  `json:"cosponsor_metadata,omitempty"`
	TaskConfigV2    TaskConfigV2         `json:"task_config_v2"`
}

type TaskConfigV2 struct {
	Tasks map[string]TaskEntry `json:"tasks"`
}

type TaskEntry struct {
	Type         string   `json:"type"`
	Target       int      `json:"target"`
	EventName    string   `json:"event_name,omitempty"`
	Applications []struct {
		ID string `json:"id"`
	} `json:"applications,omitempty"`
}

type QuestApplication struct {
	ID   string `json:"id"`
	Name string `json:"name"`
	Link string `json:"link"`
}

type QuestAssets struct {
	Hero                 string  `json:"hero"`
	HeroVideo            *string `json:"hero_video"`
	QuestBarHero         string  `json:"quest_bar_hero"`
	QuestBarHeroVideo    *string `json:"quest_bar_hero_video"`
	GameTile             string  `json:"game_tile"`
	Logotype             string  `json:"logotype"`
}

type QuestGradient struct {
	Primary   string `json:"primary"`
	Secondary string `json:"secondary"`
}

type QuestMessages struct {
	QuestName      string `json:"quest_name"`
	GameTitle      string `json:"game_title"`
	GamePublisher  string `json:"game_publisher"`
}

type QuestRewardsConfig struct {
	AssignmentMethod int             `json:"assignment_method"`
	Rewards          []QuestReward   `json:"rewards"`
	RewardsExpireAt  *string         `json:"rewards_expire_at"`
	Platforms        []int           `json:"platforms"`
}

type QuestReward struct {
	Type        int                  `json:"type"`
	SkuID       string               `json:"sku_id"`
	Asset       *string              `json:"asset,omitempty"`
	AssetVideo  *string              `json:"asset_video,omitempty"`
	Messages    QuestRewardMessages  `json:"messages"`
}

type QuestRewardMessages struct {
	Name           string `json:"name"`
	NameWithArticle string `json:"name_with_article"`
}

type QuestVideoMetadata struct {
	Messages QuestVideoMessages `json:"messages"`
	Assets   QuestVideoAssets   `json:"assets"`
}

type QuestVideoAssets struct {
	VideoPlayerVideoHls       *string `json:"video_player_video_hls"`
	VideoPlayerVideo          string  `json:"video_player_video"`
	VideoPlayerThumbnail      *string `json:"video_player_thumbnail"`
	VideoPlayerVideoLowRes    string  `json:"video_player_video_low_res"`
	VideoPlayerCaption        string  `json:"video_player_caption"`
	VideoPlayerTranscript     string  `json:"video_player_transcript"`
	QuestBarPreviewVideo      *string `json:"quest_bar_preview_video"`
	QuestBarPreviewThumbnail  *string `json:"quest_bar_preview_thumbnail"`
	QuestHomeVideo            *string `json:"quest_home_video"`
}

type QuestVideoMessages struct {
	VideoTitle          string `json:"video_title"`
	VideoEndCtaTitle    string `json:"video_end_cta_title"`
	VideoEndCtaSubtitle string `json:"video_end_cta_subtitle"`
	VideoEndCtaButton   string `json:"video_end_cta_button_label"`
}

type QuestCosponsorMeta struct {
	Name                  string `json:"name"`
	Logotype              string `json:"logotype"`
	RedemptionInstructions string `json:"redemption_instructions"`
}

type QuestUserStatus struct {
	UserID       string                         `json:"user_id"`
	QuestID      *string                        `json:"quest_id,omitempty"`
	EnrolledAt   *string                        `json:"enrolled_at"`
	CompletedAt  *string                        `json:"completed_at"`
	ClaimedAt    *string                        `json:"claimed_at"`
	Progress     map[string]QuestTaskProgress   `json:"progress"`
}

type QuestTaskProgress struct {
	EventName   string              `json:"event_name"`
	Value       int                 `json:"value"`
	UpdatedAt   string              `json:"updated_at"`
	CompletedAt *string             `json:"completed_at"`
	Heartbeat   *QuestTaskHeartbeat `json:"heartbeat,omitempty"`
}

type QuestTaskHeartbeat struct {
	LastBeatAt string  `json:"last_beat_at"`
	ExpiresAt  *string `json:"expires_at"`
}


