# Native Go → Rust migration: full specification and acceptance criteria

**Project:** [`7ejal/Discord-Quest/rust/go-quest-port`](../)  
**Source:** [`yeweroooo/Discord-Quest-Auto-Completion-Selfbot` @ `352d7f457c9511bc9e7c9b940a71012bfb47572e`](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/tree/352d7f457c9511bc9e7c9b940a71012bfb47572e)  
**Specification:** November 1, 2026  
**Current platform rule:** [Linux-only/main-server testing](LINUX-ONLY.md) (supersedes historical cross-platform requirements)  
**Machine progress:** [`progress.json`](progress.json) · [Readable checklist](PROGRESS.md) · [Original Go file/function map](GO-FILE-MAP.md)

## 0. Objective and precise scope

Produce a **maintainable, native Rust application** that preserves the useful, legitimate behavior of the Go implementation: portable Quest data structures, local configuration, task taxonomy, campaign filtering, data inspection, progress-status interpretation, concurrent analysis, error handling, documentation, and Linux x86_64 packaging.

**A “native, buildable application” is achievable and already partially present. A “literal 1:1 operational selfbot” is not the same deliverable.** The source's personal-account client impersonation, synthetic rewarded engagement, external challenge-solving, OAuth reward simulation, and automated claims are **intentionally excluded**. This is not an invitation to reimplement those behaviors later; scope changes require separate approval and policy review.

All task results must be **evidence-based**. A local timer, HTTP 2xx, numeric progress = target, or printed message does not establish Quest-level server completion. The program must not promise to complete every game or Quest.

## 1. Source inventory and evidence

The Go repository has these **16 root entries** at the pinned revision:

```text
.env.example
.github/
.gitignore
LICENSE
README.md
captcha.go
client.go
constants.go
go.mod
go.sum
helpers.go
main.go
manager.go
quest.go
types.go
utils.go
```

For every original file, class, method, and unported action, use [GO-FILE-MAP.md](GO-FILE-MAP.md). Track implementation as `done`, `partial`, `todo`, or `excluded` using the ID in [progress.json](progress.json); do not equate “files copied” with functionally verified parity.

## 2. Platform and distribution

- Target a **Linux x86_64** native CLI executable only. No Go/Node/Python/Discord installation to launch.
- Use stable Rust 2021 edition, Cargo, and a locked dependency graph. The current project already compiles using `serde`, `serde_json`, and `chrono`; additional crates should be introduced only for actual features.
- Keep application startup simple: `demo` with bundled synthetic data; `inspect <local-path>` with a local JSON document; optional `--json`, `--workers`, `--verbose`, `--version`, help.
- Build release artifacts and record provenance and checksums.
- Run all validation exclusively on connected **main-server** using [LINUX-ONLY.md](LINUX-ONLY.md); hosted CI and Windows are no longer supported.

### Recommended crates **only when justified**

| Area | Choice | Current status |
|---|---|---|
| Typed Quest JSON | `serde`, `serde_json` | **Already used** |
| Date/time and expiry | `chrono` | **Already used** |
| Better CLI parsing | `clap` | **Not yet added**; current CLI uses std `env::args` |
| Structured errors | `thiserror`, `anyhow` | Not added |
| Structured logging | `tracing`, `tracing-subscriber` | Not added |
| UUIDs | `uuid` | Not added |
| Cancellation / concurrency for an I/O client | `tokio`, `tokio-util` | Not added; current parallel evaluation uses **scoped std threads** |
| Authorized HTTP transport / mocks | `reqwest` + injection abstraction | Not added; no live Discord calls in current app |
| WebSocket frame parsing | `tokio-tungstenite` or a simple offline JSON parser | Not added; **no personal user-token Gateway impersonation** |
| Secret redaction | `secrecy` | Not added; no secrets are required in current app |

An AI must never mark a crate “already integrated” simply because this design mentions it.

## 3. Exact Rust architecture

Current source (verified in repository):

```text
rust/go-quest-port/
├── Cargo.toml
├── .gitignore
├── README.md
├── src/
│   ├── lib.rs           # JSON parser, 15 unit tests
│   ├── main.rs          # offline demo/inspect CLI
│   ├── manager.rs       # filters, task classification, status, concurrent analysis
│   └── model.rs         # core Quest and task/reward models
├── examples/
│   └── sample-quests.json
└── migration/
    ├── SPEC.md
    ├── GO-FILE-MAP.md
    ├── PROGRESS.md
    ├── progress.json
    └── AI-WORKFLOW.md
```

Proposed additional modules (not implemented merely by appearing here):

```text
src/
├── config.rs
├── errors.rs
├── constants.rs
├── models/
│   ├── assets.rs
│   ├── reward.rs
│   ├── gateway.rs
│   └── challenge.rs
├── client/
│   ├── http.rs          # injectable fake/authorized transport
│   └── gateway.rs       # offline frame inspection or supported app protocol
├── task/
│   └── classification.rs
├── utils/
│   ├── identifiers.rs
│   └── metadata.rs
├── notifications/
│   └── mod.rs
└── output/
    └── json.rs
tests/
├── json_parsing.rs
├── status_truth.rs
├── task_group.rs
├── workflow.rs
├── cancellation.rs
└── cli_golden.rs
```

Use a modest module split; do not create empty placeholder modules to inflate parity.

## 4. Data-model requirements

Preserve and safely deserialize all legitimate Go wire model families from `types.go`:

- `AllQuestsResponse`, `QuestPartial`, `Quest`, `QuestConfig`
- `TaskConfigV2`, `TaskEntry`, `QuestApplication`
- `QuestAssets`, `QuestGradient`, `QuestMessages`
- `QuestRewardsConfig`, `QuestReward`, `QuestRewardMessages`
- `QuestVideoMetadata`, `QuestVideoAssets`, `QuestVideoMessages`, `QuestCosponsorMeta`
- `QuestUserStatus`, `QuestTaskProgress`, `QuestTaskHeartbeat`
- `GatewayPayload`, `HelloData`, `IdentifyData`, `ReadyData`, `ApplicationPublicData`
- `CaptchaData` **for read-only status classification**, never challenge bypass.

**Acceptance:** Fields that may be null/missing are `Option<T>` or defaults as appropriate; data with future/unknown fields can be preserved or clearly reported; invalid JSON and absent/invalid timestamps are not silently treated as successful completion. Test both legacy and current Quest configuration. Preserve **task-specific** application IDs before campaign-level fallback.

## 5. Quest status/selection requirements

Implement or preserve these Go `quest.go` concepts:

| Go concept | Rust equivalent | Requirement |
|---|---|---|
| `IsExpired` | `Quest::is_expired` | RFC3339 parse; explicit handling for malformed dates |
| `IsCompleted` | `Quest::is_completed` | Only server-shaped Quest `completed_at` |
| `IsEnrolled` | `Quest::is_enrolled` | Only server-shaped `enrolled_at` |
| `HasClaimedRewards` | `Quest::has_claimed_rewards` | Only `claimed_at` |
| `GetTaskName` | `original_go_first_task` | **Compatibility only**; never infer Quest completion |
| `IsAndroidQuest` | `is_android_video_only` | Differentiate mobile-video-only task map |

Preserve `FilterValid` and `FilterRedeemable` as read-only operations. A general enrollment-block marker must not mutate already enrolled data or hide inspectable campaigns.

**AND/OR:** Multi-task groups must use the declared join operator. For example, video finished + activity unfinished with `and` remains **in progress**. With `or`, one task-level confirmed item can satisfy the *group* but still must not imply **Quest** `completed_at` exists. Unknown operators fail closed.

**Status vocabulary:** not_started, expired, not_enrolled, in_progress, tasks_complete_quest_unconfirmed, quest_confirmed_unclaimed, reward_claimed, inconsistent, unsupported. External redemption remains an independent unknown state unless verified separately.

## 6. Task types and external prerequisites

The Rust project must parse/describe all ten documented current/historical identifiers, without falsely implying that it carries out each engagement:

| Identifier | Classification / condition |
|---|---|
| `WATCH_VIDEO` | Video watch progress |
| `WATCH_VIDEO_ON_MOBILE` | Mobile-specific video progress |
| `PLAY_ON_DESKTOP` | Authentic game process/playtime requirement |
| `PLAY_ON_DESKTOP_V2` | Historical/deprecated identifier |
| `STREAM_ON_DESKTOP` | Actual desktop stream and viewer conditions |
| `PLAY_ACTIVITY` | Real embedded Activity participation |
| `PLAY_ON_XBOX` | Linked console/game activity |
| `PLAY_ON_PLAYSTATION` | Linked console/game activity |
| `ACHIEVEMENT_IN_ACTIVITY` | External Activity application reports achievement |
| `ACHIEVEMENT_IN_GAME` | Publisher-controlled game achievement |

Distinct labels: recognized, historical, unknown, task-level observed, Quest-level confirmed, externally dependent. Source `switch` branches are not real-world success proof.

## 7. Client architecture / what not to port

The Go client's key clusters (`client.go`, `constants.go`, `utils.go`) are documented in [GO-FILE-MAP.md](GO-FILE-MAP.md). It manually implements a user-token Gateway session and undocumented account actions.

For the Rust **offline** application, prioritize:

- An interface that accepts synthetic/local responses from an injected mock transport.
- Timeouts, bounded retries, HTTP 429/5xx recovery and cancellation tested **entirely offline**.
- Optional authorized, supported application integrations only after scope/permissions review.
- JSON/frame parsing without user-token client Identify spoofing.
- Resource cleanup tests, no secret printing and deterministic failure outcomes.

Do **not** copy or implement personal-account Identify spoofing, synthetic video/game/stream/activity requests, fake console/achievement reporting, CAPTCHA solvers, third-party authorization to fabricate milestones, or reward claiming based on falsified engagement. Those components are **excluded** and should stay that way in progress tracking.

## 8. Concurrency and lifecycle

The original Go entry point starts a goroutine for each valid Quest and waits via `sync.WaitGroup`. The current Rust program instead uses **bounded scoped threads** (1–32), preserving output order. This is already implemented.

Additional acceptance:
- Saturated work does not create unbounded threads.
- Cancellation or shutdown releases resources.
- One invalid input never marks other tasks successful.
- Failures propagate into typed results.
- No 429 infinite recursion, indefinite progress polling, or fake local completed marker.
- Introduce Tokio only if a genuine supported asynchronous I/O use case warrants it.

## 9. Security and licensing

- Do not solicit, store, log, or commit a personal Discord user token.
- No CAPTCHA outsourcing, key leaking, or simulated rewarded engagement.
- Use only synthetic test fixtures or legitimately supplied, sanitized local JSON.
- Review dependencies and log redaction.
- Original `LICENSE` is **MIT**, Copyright (c) 2026 yeweroooo. If copying substantial original material, preserve upstream copyright and license terms and document source attribution before release.
- Never use shell commands, network requests or CI secrets to collect real users' credentials.

## 10. CLI acceptance

Existing commands (verified in source and prior CI):

```text
go-quest-port demo [--json] [--workers N]
go-quest-port inspect <local-quests.json> [--json] [--workers N]
```

New tasks: add a stable `--version`, optional `--verbose`, clearer help/error codes, strict input validation, limits for oversized inputs, and consistent stdout/stderr behavior. A `.bat` launcher is optional and provides **no additional capabilities**.

## 11. Testing and verification plan

The **15 existing unit tests** passed in [run 41256646888](https://github.com/7ejal/Discord-Quest/actions/runs/41256646888); the Windows executable also passed a smoke test. That run is a historical fact, not confirmation of all future commits.

Before claiming the extended application is finished:
- Unit tests for every supported model and null/unknown field.
- Numeric goal met, Quest `completed_at = null` => **never confirmed**.
- AND/OR group validity and unknown operator.
- Enrollment, expiry, not-yet-started, claim and inconsistent status.
- Different application IDs and deprecated task IDs.
- Identity-preserving concurrency and deterministic ordering.
- Mock 429, 5xx, malformed JSON, timeout, stalled response and cancellation.
- CLI integration/golden-output tests.
- `cargo fmt --check`, strict Clippy, `cargo test --all-targets`, `cargo build --release --locked` on Linux x86_64 main-server only.
- CI archives binaries with file hashes, provenance and limitations.
- Optional reproducible CPU/RAM benchmark with machine/inputs disclosed.

The [22 research test fixtures](../../../research/standalone/METHOD-VALIDATION-FIXTURES.md) are **ideas/documentation**, not yet 22 passing Rust integration tests.

## 12. Definition of done (DoD)

This **supported migration** is complete only when:

1. All supported `progress.json` task IDs have status `done`, or are clearly labeled optional/deferred with justification.
2. `partial`/ `todo` are never hidden; every excluded source path is explicitly documented.
3. Compilation and tests pass for the **current commit** on Linux x86_64 main-server only.
4. Actual artifacts are present and verified, not inferred from a filename.
5. README, [PROGRESS.md](PROGRESS.md), and [progress.json](progress.json) match actual behavior and current tests.
6. The program reports truthful statuses and does not offer user-account spoofing or reward claiming.
7. The final migration report includes implementation evidence, tests, known limitations, dependency notices and file mapping.

Do not state **“1:1 original selfbot parity”** because the deliberately excluded live-user automation paths are not part of this implementation.

## 13. AI delegation and progress-update policy

- The authoritative record is [progress.json](progress.json); stable IDs and `status` values `done` / `partial` / `todo` / `excluded`.
- Each implementation PR/commit cites task IDs, files changed, tests run and exact evidence.
- Never mark “done” solely from generated code or a code comment. Require the task's `acceptance` condition and passing evidence.
- Update [PROGRESS.md](PROGRESS.md) in the same change as JSON. Rerun tracking validation.
- Keep [GO-FILE-MAP.md](GO-FILE-MAP.md) current if functions/files move.
- The next coding agent starts with [AI-WORKFLOW.md](AI-WORKFLOW.md) and [../AGENTS.md](../AGENTS.md).
- No background work, imaginary benchmark, or future update promises. Only report completed work actually verified.

## 14. Expected deliverables

- Clean Rust source and documented modules.
- Native Linux x86_64 executable and checksum-verified archive.
- `Cargo.lock`, license/attribution, deterministic CI and releases.
- Expanded unit/integration test suite.
- Genuine file/function mapping and feature-parity/exclusion statement.
- This specification, a canonical JSON task tracker and human-readable status view.
- Clear evidence that **offline analysis works** and clear acknowledgment that **reward completion is outside scope**.

**Next actions:** follow the priority queue at the top of [PROGRESS.md](PROGRESS.md), rather than generating another architectural plan.
