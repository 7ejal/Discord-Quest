# Pinned Go model and behavior coverage

Source: `352d7f457c9511bc9e7c9b940a71012bfb47572e`, reviewed against active Rust source.
This is offline wire-data coverage, never account-client or engagement parity.

## Field ledger

26 named Go structs reviewed. Typed scalar fields are deserialized by the corresponding
Rust model; additional keys are retained in its flattened `extra` map. Asset,
gradient, video and sponsor trees are retained as JSON values without executing
URLs or interpreting opaque metadata. Library serialization retains their nested
extensions; it normalizes missing/default typed fields, so it is not a byte-identical archival format.
CLI reports are summaries and never serialize the source models or raw metadata.

| Go struct | JSON field | Rust destination / decision |
|---|---|---|
| `AllQuestsResponse` | `quests` | `AllQuestsResponse.quests` (typed field or preserved `extra`) |
| `AllQuestsResponse` | `excluded_quests` | `AllQuestsResponse.excluded_quests` (typed field or preserved `extra`) |
| `AllQuestsResponse` | `quest_enrollment_blocked_until` | `AllQuestsResponse.quest_enrollment_blocked_until` (typed field or preserved `extra`) |
| `QuestPartial` | `id` | `QuestPartial.id` (typed field or preserved `extra`) |
| `QuestPartial` | `config` | `QuestPartial.config` (typed field or preserved `extra`) |
| `QuestPartial` | `user_status` | `QuestPartial.user_status` (typed field or preserved `extra`) |
| `QuestPartial` | `targeted_content` | `QuestPartial.targeted_content` (typed field or preserved `extra`) |
| `QuestPartial` | `preview` | `QuestPartial.preview` (typed field or preserved `extra`) |
| `QuestPartial` | `traffic_metadata_raw` | `QuestPartial.traffic_metadata_raw` (typed field or preserved `extra`) |
| `QuestPartial` | `traffic_metadata_sealed` | `QuestPartial.traffic_metadata_sealed` (typed field or preserved `extra`) |
| `Quest` | `id` | `Quest.id` (typed field or preserved `extra`) |
| `Quest` | `config` | `Quest.config` (typed field or preserved `extra`) |
| `Quest` | `user_status` | `Quest.user_status` (typed field or preserved `extra`) |
| `Quest` | `targeted_content` | `Quest.targeted_content` (typed field or preserved `extra`) |
| `Quest` | `preview` | `Quest.preview` (typed field or preserved `extra`) |
| `Quest` | `traffic_metadata_raw` | `Quest.traffic_metadata_raw` (typed field or preserved `extra`) |
| `Quest` | `traffic_metadata_sealed` | `Quest.traffic_metadata_sealed` (typed field or preserved `extra`) |
| `QuestConfig` | `id` | `QuestConfig.id` (typed field or preserved `extra`) |
| `QuestConfig` | `config_version` | `QuestConfig.config_version` (typed field or preserved `extra`) |
| `QuestConfig` | `starts_at` | `QuestConfig.starts_at` (typed field or preserved `extra`) |
| `QuestConfig` | `expires_at` | `QuestConfig.expires_at` (typed field or preserved `extra`) |
| `QuestConfig` | `features` | `QuestConfig.features` (typed field or preserved `extra`) |
| `QuestConfig` | `application` | `QuestConfig.application` (typed field or preserved `extra`) |
| `QuestConfig` | `assets` | `QuestConfig.assets` (typed field or preserved `extra`) |
| `QuestConfig` | `colors` | `QuestConfig.colors` (typed field or preserved `extra`) |
| `QuestConfig` | `messages` | `QuestConfig.messages` (typed field or preserved `extra`) |
| `QuestConfig` | `rewards_config` | `QuestConfig.rewards_config` (typed field or preserved `extra`) |
| `QuestConfig` | `video_metadata` | `QuestConfig.video_metadata` (typed field or preserved `extra`) |
| `QuestConfig` | `cosponsor_metadata` | `QuestConfig.cosponsor_metadata` (typed field or preserved `extra`) |
| `QuestConfig` | `task_config_v2` | `QuestConfig.task_config_v2` (typed field or preserved `extra`) |
| `TaskConfigV2` | `tasks` | `TaskConfig.tasks` (typed field or preserved `extra`) |
| `TaskEntry` | `type` | `TaskEntry.type` (typed field or preserved `extra`) |
| `TaskEntry` | `target` | `TaskEntry.target` (typed field or preserved `extra`) |
| `TaskEntry` | `event_name` | `TaskEntry.event_name` (typed field or preserved `extra`) |
| `TaskEntry` | `id` | `TaskEntry.id` (typed field or preserved `extra`) |
| `TaskEntry` | `applications` | `TaskEntry.applications` (typed field or preserved `extra`) |
| `QuestApplication` | `id` | `QuestApplication.id` (typed field or preserved `extra`) |
| `QuestApplication` | `name` | `QuestApplication.name` (typed field or preserved `extra`) |
| `QuestApplication` | `link` | `QuestApplication.link` (typed field or preserved `extra`) |
| `QuestAssets` | `hero` | `QuestConfig.assets.hero` (preserved JSON) |
| `QuestAssets` | `hero_video` | `QuestConfig.assets.hero_video` (preserved JSON) |
| `QuestAssets` | `quest_bar_hero` | `QuestConfig.assets.quest_bar_hero` (preserved JSON) |
| `QuestAssets` | `quest_bar_hero_video` | `QuestConfig.assets.quest_bar_hero_video` (preserved JSON) |
| `QuestAssets` | `game_tile` | `QuestConfig.assets.game_tile` (preserved JSON) |
| `QuestAssets` | `logotype` | `QuestConfig.assets.logotype` (preserved JSON) |
| `QuestGradient` | `primary` | `QuestConfig.colors.primary` (preserved JSON) |
| `QuestGradient` | `secondary` | `QuestConfig.colors.secondary` (preserved JSON) |
| `QuestMessages` | `quest_name` | `QuestMessages.quest_name` (typed field or preserved `extra`) |
| `QuestMessages` | `game_title` | `QuestMessages.game_title` (typed field or preserved `extra`) |
| `QuestMessages` | `game_publisher` | `QuestMessages.game_publisher` (typed field or preserved `extra`) |
| `QuestRewardsConfig` | `assignment_method` | `RewardsConfig.assignment_method` (typed field or preserved `extra`) |
| `QuestRewardsConfig` | `rewards` | `RewardsConfig.rewards` (typed field or preserved `extra`) |
| `QuestRewardsConfig` | `rewards_expire_at` | `RewardsConfig.rewards_expire_at` (typed field or preserved `extra`) |
| `QuestRewardsConfig` | `platforms` | `RewardsConfig.platforms` (typed field or preserved `extra`) |
| `QuestReward` | `type` | `Reward.type` (typed field or preserved `extra`) |
| `QuestReward` | `sku_id` | `Reward.sku_id` (typed field or preserved `extra`) |
| `QuestReward` | `asset` | `Reward.asset` (typed field or preserved `extra`) |
| `QuestReward` | `asset_video` | `Reward.asset_video` (typed field or preserved `extra`) |
| `QuestReward` | `messages` | `Reward.messages` (typed field or preserved `extra`) |
| `QuestRewardMessages` | `name` | `RewardMessages.name` (typed field or preserved `extra`) |
| `QuestRewardMessages` | `name_with_article` | `RewardMessages.name_with_article` (typed field or preserved `extra`) |
| `QuestVideoMetadata` | `messages` | `QuestConfig.video_metadata.messages` (preserved JSON) |
| `QuestVideoMetadata` | `assets` | `QuestConfig.video_metadata.assets` (preserved JSON) |
| `QuestVideoAssets` | `video_player_video_hls` | `QuestConfig.video_metadata.assets.video_player_video_hls` (preserved JSON) |
| `QuestVideoAssets` | `video_player_video` | `QuestConfig.video_metadata.assets.video_player_video` (preserved JSON) |
| `QuestVideoAssets` | `video_player_thumbnail` | `QuestConfig.video_metadata.assets.video_player_thumbnail` (preserved JSON) |
| `QuestVideoAssets` | `video_player_video_low_res` | `QuestConfig.video_metadata.assets.video_player_video_low_res` (preserved JSON) |
| `QuestVideoAssets` | `video_player_caption` | `QuestConfig.video_metadata.assets.video_player_caption` (preserved JSON) |
| `QuestVideoAssets` | `video_player_transcript` | `QuestConfig.video_metadata.assets.video_player_transcript` (preserved JSON) |
| `QuestVideoAssets` | `quest_bar_preview_video` | `QuestConfig.video_metadata.assets.quest_bar_preview_video` (preserved JSON) |
| `QuestVideoAssets` | `quest_bar_preview_thumbnail` | `QuestConfig.video_metadata.assets.quest_bar_preview_thumbnail` (preserved JSON) |
| `QuestVideoAssets` | `quest_home_video` | `QuestConfig.video_metadata.assets.quest_home_video` (preserved JSON) |
| `QuestVideoMessages` | `video_title` | `QuestConfig.video_metadata.messages.video_title` (preserved JSON) |
| `QuestVideoMessages` | `video_end_cta_title` | `QuestConfig.video_metadata.messages.video_end_cta_title` (preserved JSON) |
| `QuestVideoMessages` | `video_end_cta_subtitle` | `QuestConfig.video_metadata.messages.video_end_cta_subtitle` (preserved JSON) |
| `QuestVideoMessages` | `video_end_cta_button_label` | `QuestConfig.video_metadata.messages.video_end_cta_button_label` (preserved JSON) |
| `QuestCosponsorMeta` | `name` | `QuestConfig.cosponsor_metadata.name` (preserved JSON) |
| `QuestCosponsorMeta` | `logotype` | `QuestConfig.cosponsor_metadata.logotype` (preserved JSON) |
| `QuestCosponsorMeta` | `redemption_instructions` | `QuestConfig.cosponsor_metadata.redemption_instructions` (preserved JSON) |
| `QuestUserStatus` | `user_id` | `QuestUserStatus.user_id` (typed field or preserved `extra`) |
| `QuestUserStatus` | `quest_id` | `QuestUserStatus.quest_id` (typed field or preserved `extra`) |
| `QuestUserStatus` | `enrolled_at` | `QuestUserStatus.enrolled_at` (typed field or preserved `extra`) |
| `QuestUserStatus` | `completed_at` | `QuestUserStatus.completed_at` (typed field or preserved `extra`) |
| `QuestUserStatus` | `claimed_at` | `QuestUserStatus.claimed_at` (typed field or preserved `extra`) |
| `QuestUserStatus` | `progress` | `QuestUserStatus.progress` (typed field or preserved `extra`) |
| `QuestTaskProgress` | `event_name` | `TaskProgress.event_name` (typed field or preserved `extra`) |
| `QuestTaskProgress` | `value` | `TaskProgress.value` (typed field or preserved `extra`) |
| `QuestTaskProgress` | `updated_at` | `TaskProgress.updated_at` (typed field or preserved `extra`) |
| `QuestTaskProgress` | `completed_at` | `TaskProgress.completed_at` (typed field or preserved `extra`) |
| `QuestTaskProgress` | `heartbeat` | `TaskProgress.heartbeat` (typed field or preserved `extra`) |
| `QuestTaskHeartbeat` | `last_beat_at` | `TaskHeartbeat.last_beat_at` (typed field or preserved `extra`) |
| `QuestTaskHeartbeat` | `expires_at` | `TaskHeartbeat.expires_at` (typed field or preserved `extra`) |
| `CaptchaData` | `captcha_key` | Excluded endpoint/session/challenge wire family; no corresponding CLI input or network action |
| `CaptchaData` | `captcha_sitekey` | Excluded endpoint/session/challenge wire family; no corresponding CLI input or network action |
| `CaptchaData` | `captcha_service` | Excluded endpoint/session/challenge wire family; no corresponding CLI input or network action |
| `CaptchaData` | `captcha_session_id` | Excluded endpoint/session/challenge wire family; no corresponding CLI input or network action |
| `CaptchaData` | `captcha_rqdata` | Excluded endpoint/session/challenge wire family; no corresponding CLI input or network action |
| `CaptchaData` | `captcha_rqtoken` | Excluded endpoint/session/challenge wire family; no corresponding CLI input or network action |
| `GatewayPayload` | `op` | Excluded endpoint/session/challenge wire family; no corresponding CLI input or network action |
| `GatewayPayload` | `d` | Excluded endpoint/session/challenge wire family; no corresponding CLI input or network action |
| `GatewayPayload` | `s` | Excluded endpoint/session/challenge wire family; no corresponding CLI input or network action |
| `GatewayPayload` | `t` | Excluded endpoint/session/challenge wire family; no corresponding CLI input or network action |
| `HelloData` | `heartbeat_interval` | Excluded endpoint/session/challenge wire family; no corresponding CLI input or network action |
| `IdentifyData` | `token` | Excluded endpoint/session/challenge wire family; no corresponding CLI input or network action |
| `IdentifyData` | `capabilities` | Excluded endpoint/session/challenge wire family; no corresponding CLI input or network action |
| `IdentifyData` | `properties` | Excluded endpoint/session/challenge wire family; no corresponding CLI input or network action |
| `IdentifyData` | `presence` | Excluded endpoint/session/challenge wire family; no corresponding CLI input or network action |
| `IdentifyData` | `compress` | Excluded endpoint/session/challenge wire family; no corresponding CLI input or network action |
| `IdentifyData` | `client_state` | Excluded endpoint/session/challenge wire family; no corresponding CLI input or network action |
| `ReadyData` | `id` | Excluded endpoint/session/challenge wire family; no corresponding CLI input or network action |
| `ReadyData` | `username` | Excluded endpoint/session/challenge wire family; no corresponding CLI input or network action |
| `ReadyData` | `user` | Excluded endpoint/session/challenge wire family; no corresponding CLI input or network action |
| `ApplicationPublicData` | `id` | Excluded endpoint/session/challenge wire family; no corresponding CLI input or network action |
| `ApplicationPublicData` | `name` | Excluded endpoint/session/challenge wire family; no corresponding CLI input or network action |
| `ApplicationPublicData` | `icon` | Excluded endpoint/session/challenge wire family; no corresponding CLI input or network action |
| `ApplicationPublicData` | `description` | Excluded endpoint/session/challenge wire family; no corresponding CLI input or network action |
| `ApplicationPublicData` | `os` | Excluded endpoint/session/challenge wire family; no corresponding CLI input or network action |
| `ApplicationPublicData` | `name` | Excluded endpoint/session/challenge wire family; no corresponding CLI input or network action |
| `ApplicationPublicData` | `is_launcher` | Excluded endpoint/session/challenge wire family; no corresponding CLI input or network action |
| `ApplicationPublicData` | `executables` | Excluded endpoint/session/challenge wire family; no corresponding CLI input or network action |

For anonymous nested Go structs (task applications, READY user, executable lists),
the fields are included under their owning row. Application IDs are typed; public
application extension fields remain opaque data; READY/session data is excluded.
Unknown top-level wrapper extensions are not part of the inspection summary or
library Quest round trip; full original documents should be retained separately.

## Schema rules

- The pinned Go `TaskConfigV2` has only `tasks`. It declares **no join operator or default**.
- Rust accepts a string `join_operator` as an explicit synthetic/offline extension:
  `and` and `or` after trimming/lowercasing. A lone task with no operator is `single`.
  Missing multi-task operators, unknown strings and empty groups fail closed.
  Wrong-type operators are input errors; there is no guessed AND/OR default.
- Non-null v2 takes precedence over legacy `task_config`, including an empty v2 map.
  Configs are never merged. Null/absent v2 permits the legacy extension.
- Modern authenticated/sanitized Discord samples have not been supplied.
  `AUD-001` and `STATE-005` remain partial for modern-schema verification.
  Synthetic coverage does not satisfy that evidence requirement.
- Model fields use strict types. Missing/null optional timestamps indicate no
  milestone and are distinguished in parser diagnostics. Wrong types fail
  without raw JSON values in the error. Null task maps/progress maps are rejected.
- Targets/progress follow the source integer domain, capped at the exact f64
  integer range (2^53-1); invalid values cannot satisfy a task group.
- Unknown task IDs are retained by `TaskKind::Unknown`, reported as unknown,
  and cannot establish task support even when the local Quest reports completion.

## Pure-helper and dispatcher comparison

`tools/go-quest-reference` contains copied pinned model/Quest helpers and only
the original pure filter loops. It cannot link the upstream client. The Go
reference uses its original wall clock; tests choose distant past/future expiry
and compare Rust at the same current time. Exact-boundary behavior is tested
separately with Rust's fixed clock.

| Original method | Rust behavior | Executable evidence |
|---|---|---|
| IsExpired | Same for valid distant dates; missing/invalid expiry is unknown; equality is expired | `tests/go_parity.rs`; `tests/cli_safety.rs` |
| IsCompleted/IsEnrolled/HasClaimedRewards | Same for valid timestamps; unlike Go non-null pointers, invalid/blank strings are not milestones | `tests/go_parity.rs`; `src/lib.rs` |
| GetTaskName | Same nine-item priority, compatibility information only | `tests/go_parity.rs`; `src/lib.rs` |
| IsAndroidQuest | Same mobile-present/desktop-video-absent rule; never platform execution | `tests/go_parity.rs` |
| NewQuestManager | Preserve block marker for inspection; identical duplicates deduplicated, conflicts rejected; original Go aborts/overwrites | `tests/go_parity.rs`; `tests/offline_contract.rs` |
| FilterValid | Intentional Go-style uncompleted/unexpired selection, including previews/future starts; assessment independently reports starts/unsupported states | `tests/go_parity.rs`; `tests/offline_regressions.rs` |
| FilterRedeemable | Report completed/unclaimed local records, never prove claim eligibility or external redemption | `tests/go_parity.rs`; `tests/offline_contract.rs` |
| DoingQuest dispatcher | Classify all ten catalog IDs, preserve per-task application identity; no task execution | `tests/task_catalog.rs`; `tests/model_parity.rs` |
| addExcludedQuest | Keep supplied partial metadata; no network fetch | `tests/excluded_quests.rs` |
| acceptQuest, all doing* methods, deauthorizeApp, RedeemQuest | Excluded actions; no enrollment, engagement, OAuth, CAPTCHA or claim implementation | `migration/CAPABILITY-CONTRACT.md`; process-local network-isolation checks |

## Input and output privacy

User IDs, targeting, sealed traffic metadata, unknown extensions, reward payloads
and raw documents stay out of CLI summaries. Names, Quest/application IDs, task
identifiers and reported event/freshness fields are intentionally visible
inspection data. Sanitize these fields before sharing reports; the inspector
cannot identify arbitrary secrets deliberately embedded in a campaign name.
Errors never include source values or filenames. Verbose diagnostics contain
only record counts, worker limits and source trust.
