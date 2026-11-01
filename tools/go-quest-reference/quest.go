package main

import "time"

// IsExpired returns true if the quest has expired.
func (q *Quest) IsExpired() bool {
	expiresAt, err := time.Parse(time.RFC3339, q.Config.ExpiresAt)
	if err != nil {
		expiresAt, err = time.Parse("2006-01-02T15:04:05.000000+00:00", q.Config.ExpiresAt)
		if err != nil {
			return false
		}
	}
	return time.Now().After(expiresAt)
}

// IsCompleted returns true if the quest has been completed.
func (q *Quest) IsCompleted() bool {
	return q.UserStatus != nil && q.UserStatus.CompletedAt != nil
}

// IsEnrolled returns true if the user has enrolled in the quest.
func (q *Quest) IsEnrolled() bool {
	return q.UserStatus != nil && q.UserStatus.EnrolledAt != nil
}

// HasClaimedRewards returns true if the user has claimed rewards.
func (q *Quest) HasClaimedRewards() bool {
	return q.UserStatus != nil && q.UserStatus.ClaimedAt != nil
}

// GetTaskName finds the first matching task type in the quest config.
func (q *Quest) GetTaskName() string {
	taskOrder := []string{
		"WATCH_VIDEO",
		"PLAY_ON_DESKTOP",
		"PLAY_ON_DESKTOP_V2",
		"PLAY_ON_XBOX",
		"PLAY_ON_PLAYSTATION",
		"STREAM_ON_DESKTOP",
		"PLAY_ACTIVITY",
		"WATCH_VIDEO_ON_MOBILE",
		"ACHIEVEMENT_IN_ACTIVITY",
	}
	for _, name := range taskOrder {
		if _, ok := q.Config.TaskConfigV2.Tasks[name]; ok {
			return name
		}
	}
	return ""
}

// IsAndroidQuest returns true if the quest should be done via Android path.
func (q *Quest) IsAndroidQuest() bool {
	_, hasMobile := q.Config.TaskConfigV2.Tasks["WATCH_VIDEO_ON_MOBILE"]
	_, hasDesktop := q.Config.TaskConfigV2.Tasks["WATCH_VIDEO"]
	return hasMobile && !hasDesktop
}

