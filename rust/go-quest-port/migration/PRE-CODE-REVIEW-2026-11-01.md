# Pre-code verification review — November 1, 2026

**Decision:** **GO for implementation of the highest-priority, approved-scope Rust fixes. NO-GO for calling this a finished 1:1 selfbot, universal Quest completer, or release-ready app.**

**Current canonical tracker:** [progress.json](progress.json) · [Generated dashboard](PROGRESS.md) · [Fresh agent handoff](START-HERE.md) · [Release gates](RELEASE-GATES.md).

## What was double-checked

1. Compared [the pinned original Go source tree at `352d7f4`](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/tree/352d7f457c9511bc9e7c9b940a71012bfb47572e), including `types.go`, `quest.go`, `main.go`, `manager.go`, `client.go`, `constants.go`, and the remaining root-level entries, against active Rust files under `rust/go-quest-port/src`.
2. Reviewed all 114 previous migration task IDs, their acceptance criteria, status/evidence definitions, release gating, CI workflow and AI instructions.
3. Confirmed the older sibling `rust/quest-audit-cli` is *not* the active executable crate and cannot be assumed to build a CLI.
4. Checked current crate's top-level structs, JSON behavior, task-type matching, filtering, threading, CLI reporting and 15 prior Rust tests.
5. Cross-referenced the Go original's public documentation and static source with the Rust project's `GO-FILE-MAP.md`, `GAP-AUDIT.md`, `CAPABILITY-CONTRACT.md` and `TEST-MATRIX.md`.
6. Checked historical GitHub Actions results and whether successful CI covers the **same commit** as the source being reviewed.

**No live Discord Quest, account token, challenged account, game presence or reward-claim tests were performed.** The original Go's advertised task breadth does not establish actual completion rates.

## Newly identified evidence-backed findings

| Task ID | What current source reveals | Verification / required behavior |
|---|---|---|
| `MODEL-010` | Go `TaskEntry` includes `type` while Rust `TaskEntry` currently contains only `target`, `event_name`, and `applications`. | Preserve task's own type and validate against task map key; test conflicting/missing values. |
| `MODEL-011` | Go `QuestReward` has `sku_id`, `asset`, `asset_video`, and messages; current Rust `Reward` instead includes quantities not present in pinned Go type. | Model the real source schema and treat additional fields only as separately evidenced extensions. |
| `STATE-019` | Go status contains an optional `quest_id` while Rust omits it. | Cross-check outer `id`, `config.id`, and `user_status.quest_id` when supplied. |
| `STATE-020` | Go progress includes `updated_at` and a heartbeat with `last_beat_at`, `expires_at`; Rust currently loses both. | Preserve and characterize observed freshness without fabricating sessions/heartbeats. |
| `STATE-021` | Both task configuration and reported task progress can contain `event_name`; current Rust progress omits it. | Validate/reveal mismatched reported event types. |
| `SEC-003` | Rust CLI prints untrusted local `quest_name` and IDs directly, with no terminal-control filtering. | Escape ANSI/OSC controls, newlines, carriage returns, bidi override and spoofed status text; test. |
| `SEC-004` | Imported JSON may contain personal/user identifiers; current output policy is not fully defined. | Minimize/redact sensitive identifiers and ensure debug/errors do not leak raw input. |
| `QA-009` | Current Rust tests are synthetic; there is no executable differential test of **pure, non-network** Go helper behavior. | Test matching/informational differences of Go and Rust pure status/filter functions offline. |
| `QA-010` | Bounded Rust worker threads exist, but panic/failure propagation has not been independently tested. | Ensure incomplete worker output cannot be mistaken for successful/complete results. |
| `REL-009` | Windows EXE smoke-tested on GitHub runner, but absence of *all runtime DLL prerequisites* is unproven. | Test clean-system launch and document OS/DLL dependencies instead of claiming guaranteed static portability. |
| `DOC-005` | Rust reads **local, untrusted** JSON yet status names/comments can suggest server-verified completion. | State provenance limits, safe sanitized fixtures and permitted input workflows clearly. |
| `DOC-006` | Current README shows Cargo commands but lacks comprehensive native-end-user troubleshooting. | Document architecture compatibility, download trust, CLI failures and artifact expiration. |

## Critical corrections to previous “done” claims

- **`STATE-005` changed from `done` to `partial`.** Rust's AND/OR evaluator works on synthetic strings, but the pinned Go `TaskConfigV2` contains a `tasks` map **without `join_operator`**. Its real API wire-format support is **not validated**. Source: [Go types.go](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/types.go), [Rust manager.rs](../src/manager.rs). Related `AUD-001` and `STATUS-015` remain open.
- **`QA-008` changed from `done` to `partial`.** The existing tracker checker has tests, but its acceptance calls for unit coverage of stale Markdown, invalid status and missing evidence files that is incomplete. Do not confuse a successful full CI check with exhaustive test coverage of the checker.
- **`OPS-001` remains done**, but its acceptance wording was corrected: generated `PROGRESS.md` is fully overwritten; curated prose belongs in separate docs rather than being “preserved” by the generator.
- **`TEST-004` remains done only for the historical cross-platform build/smoke**. Cargo.lock and reproducible/tagged builds remain unimplemented `ENV-005`, `REL-002`, `REL-006`.
- **`STATUS-010` now explicitly includes task-level `progress.completed_at`**, not just Quest-level fields.
- **`STATUS-013` is clarified:** even a syntactically valid `completed_at` field **inside a local file cannot be authenticated as genuine server state by this offline program**. Verdict wording and evidence provenance must reflect that limitation.

## Model coverage fact check

- Pinned upstream [`types.go`](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/types.go) declares **26 Go struct types**.
- The inspected Rust [`src/model.rs`](../src/model.rs) declares **12 Rust struct types**.
- These are **not directly interchangeable counts** because some Go types cover excluded user-token login/CAPTCHA structures, and some Rust types merge Go structures. A field-by-field coverage and exclusion ledger is still required under `PARSE-010` before anyone calls it model parity.
- The Go file inventory remains 16 root entries, fully listed by [GO-FILE-MAP.md](GO-FILE-MAP.md).

## Source-truthfulness reminders

**Five separate things are often conflated:** an upstream source handler; locally recognized task IDs; actual eligible game/stream/activity participation; *authenticated* server progress; reward claim/redemption. The offline Rust program currently does **the first two at analysis level only**. Locally supplied data cannot authenticate origin, and no game reward execution was verified.

Use the [capability contract](CAPABILITY-CONTRACT.md) to prevent ambiguous wording such as “all Quests supported” or “real completion verified.”

## Snapshot after audit

| Task status | Count |
|---|---:|
| Done | 38 |
| Partial | 13 |
| Todo | 72 |
| Intentionally excluded | 4 |
| **Total** | **127** |

**Approved offline-scope required tasks:** 108, of which **72 remain open**. These totals describe engineering work inventory, **not** percent completion of a personal-account selfbot.

## Final dependency-ordering finding

`OPS-009` was added after cross-checking the task DAG: some P0 release blockers depend on P1/P2 tasks that have not been completed. The existing generated dashboard sorts by priority but does not distinguish immediately actionable from blocked tasks. Until it is fixed, agents must check dependency statuses before selecting work. With this addition, the canonical inventory is **127 tasks**, including 13 new tasks beyond the previous 114.

## GO/NO-GO pre-code instructions

**GO** when the next agent:
1. Opens [START-HERE.md](START-HERE.md) and the latest canonical [progress.json](progress.json), not an older chat summary.
2. Checks working tree, active branch, CI **head_sha**, current active crate and upstream pinned SHA.
3. Selects specific unblocked P0 tasks (start with `AUD-001`, `PARSE-001`, `STATUS-010`, `MODEL-010`, `SEC-003`, plus the dependencies needed).
4. Writes failing synthetic fixtures, implements fixes, and runs Cargo/checker/CI on the changed commit.
5. Updates JSON evidence and regenerates Markdown. No task becomes done on source existence alone.
6. Keeps current user-token selfbot/spoofing/captcha/reward functions **excluded**.

**NO-GO** if the AI starts changing the wrong Rust crate, assumes multi-task schema is proved, treats local JSON as authenticated, omits prior CI failures, claims successful tests without running them, changes `excluded` scope, or claims a native all-game completer.

## Can we guarantee there will be no further tasks?

**No.** This audit covers the pinned source, existing Rust tree, known fixture/release requirements and tracker at a definite revision. Dependencies, compiler changes, newly encountered input samples and testing may reveal new issues. The agent must create new task IDs with acceptance/evidence and regenerate the progress board when a real gap is found. The goal is a **complete traceable process**, not a static promise of exhaustiveness.

**Source authority:** pinned original Go code; current GitHub Rust source; CI runs (only for their exact head SHA). All conclusions are static or mocked unless explicitly labeled otherwise.
