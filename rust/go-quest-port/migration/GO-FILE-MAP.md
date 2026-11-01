# Exhaustive Go file/function → Rust destination map

**Source baseline:** [yeweroooo/Discord-Quest-Auto-Completion-Selfbot @ `352d7f457c9511bc9e7c9b940a71012bfb47572e`](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/tree/352d7f457c9511bc9e7c9b940a71012bfb47572e), branch `golang`. Reviewed November 1, 2026.

**Interpretation of statuses:** ✅ implemented in the supported offline Rust application; ◐ partial; ⬜ not yet implemented; ⛔ intentionally excluded. The original Go program is a user-account selfbot, while the supported Rust destination is a native **read-only** Quest-state analyzer. Source mapping is **not** an assertion of behavioral parity for engagement simulation or account verification.

## Every top-level original repository entry

| Go source entry | Go role | Rust location/replacement | Status | Required work / reason |
|---|---|---|---|---|
| [`main.go`](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/main.go) | Startup, session lifecycle, goroutines, filtering | [`src/main.rs`](../src/main.rs), [`src/manager.rs`](../src/manager.rs) | ◐ | Offline CLI, manager and parallel analysis done. Personal user login and reward spoofing intentionally excluded. Add CLI errors, flags and cancellation tests. |
| [`types.go`](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/types.go) | Many Quest, task, reward, Gateway and application data structs | [`src/model.rs`](../src/model.rs) | ◐ | Core task/type, reward fields, status and heartbeat data now covered with offline tests; assets, gradients, video/cosponsor metadata, challenge and Gateway wire models remain incomplete. |
| [`quest.go`](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/quest.go) | Expired, enrolled, completed, claimed, task priority, Android classification | [`src/model.rs`](../src/model.rs), [`src/manager.rs`](../src/manager.rs) | ✅ | Methods migrated; flawed first-match behavior exists for audit parity only; Rust also evaluates AND/OR correctly. |
| [`manager.go`](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/manager.go) | QuestManager, filters, enrollment, task dispatch, task “completion”, OAuth and claim | [`src/manager.rs`](../src/manager.rs) | ◐ | Filtering/classification/status analysis done. Enrollment and reward simulation handlers excluded. Offline transports/mocks and better states still needed. |
| [`client.go`](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/client.go) | User-token REST/Gateway client, Identify, presence, callbacks, 429 handling | Proposed `src/client/http.rs`, `src/client/gateway.rs` | ⬜/⛔ | Offline fixture-based HTTP/Gateway frame parsers are not done. User-token selfbot client and client impersonation intentionally excluded. |
| [`constants.go`](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/constants.go) | Desktop/Android client properties and request header construction | Proposed `src/constants.rs`, `src/config.rs` | ⛔/⬜ | Desktop/device impersonation excluded. Safe local configuration and recognized task constants may be added. |
| [`utils.go`](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/utils.go) | Client version/build scraping, user-agent formatting, webhooks, third-party activity authorization/progress | Proposed `src/utils/metadata.rs`, `src/notifications/` | ⛔/⬜ | Safe local metadata formatting and optional authorized notifications can be developed. Client spoofing/version harvesting and third-party achievement simulation not ported. |
| [`helpers.go`](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/helpers.go) | UUID generation and formatting | Proposed `src/utils/identifiers.rs` | ⬜ | Use Rust `uuid` crate rather than `math/rand`-based UUID bytes. Include determinism-independent format tests. |
| [`captcha.go`](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/captcha.go) | YesCaptcha request models and external solving | Optional `src/models/challenge.rs` model only | ⛔ | Solving/bypass excluded. If needed, parse/read challenge status without sending credentials to third parties. |
| [`go.mod`](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/go.mod) | Go module dependencies | [`Cargo.toml`](../Cargo.toml) | ✅ | Cargo package exists, with dependencies appropriate for **offline** functionality. |
| [`go.sum`](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/go.sum) | Dependency checksums | [`Cargo.lock`](../Cargo.lock) | ✅ | Lockfile committed; the main-server Linux scripts use `--locked`. |
| [`.env.example`](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/.env.example) | User token, webhook, external solver key | Optional safe `.env.example` | ⛔/⬜ | No personal-user-token configuration; only optional legitimate local settings (logs/output) may be documented. |
| [`.gitignore`](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/.gitignore) | Ignore binary artifacts and secret files | [`.gitignore`](../.gitignore) | ✅ | Exists and excludes target, .env and exe. |
| [`README.md`](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/README.md) | Setup, feature claims, caveats | [`README.md`](../README.md), [`SPEC.md`](SPEC.md) | ◐ | Offline use documented; port completion must not claim unsupported Go features. |
| [`LICENSE`](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/LICENSE) | MIT license, Copyright (c) 2026 yeweroooo | Proposed `LICENSE` + attribution/NOTICE | ⬜ | Upstream is **MIT** and requires retaining its copyright/permission notice when using substantial portions. Review provenance and add notices. |
| [`.github/`](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/tree/352d7f457c9511bc9e7c9b940a71012bfb47572e/.github) | Automation workflows | [`scripts/test-linux.sh`](../scripts/test-linux.sh), [`scripts/package-linux.sh`](../scripts/package-linux.sh) | ◐ | Windows and hosted GitHub runners retired per Linux-only request. Tests and release archives are built exclusively on main-server; distribution remains separate. |

## Latest source-parity additions (verified November 1, 2026)

The [main-server model validation report](MAIN-SERVER-CI-2026-11-01-MODEL-BATCH.md) records 52 passing offline Rust tests and historical cross-platform CI (no longer the active test pipeline). Go `TaskEntry.type` is now parsed and checked against task-map keys; reported `event_name`, `updated_at`, optional heartbeat dates, reward SKU/assets/messages and reward configuration fields are present. Excluded partial Quests retain optional source metadata internally while only safe summary fields are printed. **This is model/parsing coverage, not actual game/Quest completion support.**

## Go function mapping (every named operational cluster)

### `main.go`

| Go code | Rust code/status | Notes |
|---|---|---|
| `main` config/startup/shutdown | ✅ `src/main.rs::run`, `main` for offline CLI | Different supported input source: local JSON instead of live user-account login |
| `NewDiscordClient`, READY callback | ⛔ | Personal user-token gateway session not ported |
| `FetchQuests` | ⬜ Local fixture/mock reader proposed | Do not imply live account access |
| `NewQuestManager`, `FilterValid` | ✅ `QuestManager::new/filter_valid` | Diagnostic-only |
| Goroutines, `WaitGroup` | ✅ `analyze_parallel` using scoped bounded threads | Preserves output ordering; not Tokio |
| Default commented-out reward loop | ⛔ | It was not active in original standard flow |

### `quest.go`

| Go function | Rust function | Status |
|---|---|---|
| `IsExpired` | `Quest::is_expired` | ✅ |
| `IsCompleted` | `Quest::is_completed` | ✅ |
| `IsEnrolled` | `Quest::is_enrolled` | ✅ |
| `HasClaimedRewards` | `Quest::has_claimed_rewards` | ✅ |
| `GetTaskName` | `Quest::original_go_first_task` | ✅ compatibility only; known wrong for AND |
| `IsAndroidQuest` | `Quest::is_android_video_only` | ✅ |

### `manager.go`

| Go function | Rust function/status | Evidence or limitation |
|---|---|---|
| `NewQuestManager` | ✅ `QuestManager::new` | Holds local Quest list and blocked marker |
| `addExcludedQuest` | ⬜ partial model only | Excluded campaign lookup via personal token not supported |
| `FilterValid` | ✅ `QuestManager::filter_valid` | Existing test |
| `FilterRedeemable` | ✅ `QuestManager::filter_redeemable` | Existing test |
| `DoingQuest` | ◐ `assess_quest` | Classification, no engagement execution |
| `acceptQuest` | ⛔ | User-account enrollment action excluded |
| `doingWatchVideoQuest` | ⛔ | Simulated video progress excluded |
| `doingPlayOnPlatformQuest` | ⛔ | Simulated gameplay/console progress excluded |
| `doingPlayActivityQuest` | ⛔ | Fabricated activity session excluded |
| `doingAchievementInActivityQuest` | ⛔ | Special external authorization/achievement simulation excluded |
| `doingStreamOnDesktopQuest` | ⛔ | Simulated stream presence excluded |
| `deauthorizeApp` | ⛔ | Coupled to excluded authorization flow |
| `RedeemQuest` | ⛔ | Reward claiming/challenge solving excluded |
| `cutPrefix` | ⬜ optional | Low-level utility, only port if legitimate code needs it |

### `client.go`

| Go function | Rust status |
|---|---|
| `NewDiscordClient`, `SetOnReady`, `Connect`, `Destroy` | ⛔ personal user-account selfbot; offline mock lifecycle can be implemented |
| `readLoop`, `handleHello`, `handleDispatch`, `sendHeartbeat`, `heartbeatLoop` | ⬜ offline protocol/frame analysis possible; no spoofed Identify |
| `sendIdentify`, `SendPresenceUpdate` | ⛔ user-account/client impersonation |
| `sendJSON`, `Post`, `PostWithHeaders`, `Get`, `Request` | ⬜ credential-free mock transport, bounded retry/timeout tests |
| `FetchQuests` | ⬜ local fixture provider can mimic response *data structure* only |
| `EmitQuestCompleted`, `SetupWebhook` | ⬜ optional sanitized report notifications, not spoofed success |

### `constants.go`, `utils.go`, `helpers.go`, `captcha.go`

| Go function cluster | Rust status |
|---|---|
| `Properties`, `AndroidProperties`, Base64 variants, custom mobile/desktop headers | ⛔ impersonating platform clients |
| `ApplyDesktopHeaders`, `MakeHeaders` | ⛔ as user-token client emulation |
| `splitMajor`, `BuildUserAgent` | ⬜ optional safe version-format helpers |
| `FetchAllServerProps`, `fetchClientVersion`, `fetchNativeBuildNumber`, `fetchBuildNumberFromJS` | ⛔ client-impersonation scraping; no need in offline port |
| `ExtractWebhookInfo`, `SendWebhookMessage` | ⬜ optional authorized notification adapter with escaping |
| `GetProxyTicket`, `AuthorizeDiscordSays`, `ProgressDiscordSays` | ⛔ excluded achievement simulation |
| `randomUUID`, `formatUUID` | ⬜ use `uuid` crate if needed |
| `InitCaptcha`, `SolveCaptcha` | ⛔ third-party challenge bypass/external solver |

## Functionality genuinely present today

- [`src/model.rs`](../src/model.rs): core models, Quest status methods, first-task comparison, v2/legacy config, Android-only classification.
- [`src/manager.rs`](../src/manager.rs): filters, task-family classification, AND/OR support, server-status evidence, per-task app IDs and bounded thread processing.
- [`src/lib.rs`](../src/lib.rs): JSON parser and **32 library unit tests**; additional CLI/model/excluded integration tests bring suite to 52..
- [`src/main.rs`](../src/main.rs): `demo`, `inspect`, `--json`, `--workers`.
- [`examples/sample-quests.json`](../examples/sample-quests.json): fabricated test data.

Source provenance is recorded here, not a claim that *every Go function was copied or that every game's Quests can be completed*. The canonical task-by-task record is [progress.json](progress.json).
