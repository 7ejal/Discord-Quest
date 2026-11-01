# Additional source-level gap audit — November 1, 2026

This audit compares the **actual repository state**, not the previous prose claims. Source baseline: [Go `352d7f4`](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/tree/352d7f457c9511bc9e7c9b940a71012bfb47572e). Rust target: [current project source](../src). **No live Discord account or Quest completion was tested.**

**Follow-up:** [Pre-code source audit, 126 tasks](PRE-CODE-REVIEW-2026-11-01.md) adds key/type mismatch, reward-model correction, JSON provenance trust, terminal escape handling, cross-Quest ID consistency, freshness metadata and clean Windows executable testing. `STATE-005` is now *partial*, not done, because real AND/OR schema compatibility is unproven.

## Highest-impact discrepancies, with proof and task IDs

| Priority | Source-backed finding | Why important | Tracker task |
|---|---|---|---|
| P0 | `parse_quests` treats an object without `quests` as a Quest. `Quest` has `#[serde(default)]` fields. Thus `{}` deserializes successfully with blank ID and empty config. | Invalid input can look like a normal un-enrolled Quest instead of an invalid record. | `PARSE-001` |
| P0 | Original Go `NewQuestManager` collects Quests in `map[string]*Quest`, keyed by ID. Rust `QuestManager` keeps `Vec<Quest>` and no deduplication. | The same Quest appearing twice is counted twice; conflicting duplicate statuses are ambiguous. | `PARSE-002`, `AUD-002` |
| P0 | Rust `has_timestamp` checks only nonempty string. `is_completed`/`is_enrolled`/`has_claimed_rewards` accept a malformed nonempty string. | Invalid timestamps can establish false confirmed status. | `STATUS-010`, `STATUS-011` |
| P0 | Rust `Quest::is_expired` returns false when expires_at is malformed. `assess_quest` likewise treats missing parse as “not expired.” | Unknown expiry is silently treated as unexpired; eligibility reports can mislead. | `STATUS-012` |
| P0 | `parse_quests` parses a Go `AllQuestsResponse` but returns only `quests` and the blocked-until marker. It discards `excluded_quests`. Rust `QuestPartial` only has `id`. | Original wrapper evidence is lost, preventing correct excluded/preview reporting. | `PARSE-003`, `MODEL-003` |
| P0 | Rust has a sibling `rust/quest-audit-cli` crate containing `Cargo.toml` and `src/lib.rs`, but no `main.rs`. | Agents may edit the wrong project, run tests in wrong directory, or claim a binary that cannot exist there. | `AUD-003` |
| P0 | Rust `src/main.rs` reads entire file with `fs::read_to_string`, `serde_json::from_str` and no explicit bytes/depth/count resource budget. | Malicious/accidental gigantic local JSON may exhaust memory. | `PARSE-005` |
| P0 | Cargo manifest exists, but **`Cargo.lock` is not in repository tree**; workflow does not use `--locked`. | Future builds may use different transitive versions, so historical CI output is not reproducible. | `ENV-005`, `REL-006` |
| P1 | CLI only implements `demo`, `inspect`, `--json`, `--workers`. `--version`, `--verbose`, stable exit codes, output schema version, and deterministic `--at` are absent. | Difficult downstream automation, reproducibility and debugging. | `CLI-002`, `STATUS-016`, `UX-001`, `UX-002`, `UX-004` |
| P1 | Original Go `TaskConfigV2` is a simple task map. Rust assumes `join_operator` as `Option<String>` without schema-fidelity fixtures. | Some real variants could be rejected or misinterpreted; need sanitized schema evidence rather than guesses. | `AUD-001`, `PARSE-009` |
| P1 | Rust `task_family` recognizes ten task identifiers, but `data/task-types.json` is a separate catalog. | Identifiers and deprecated classifications can drift between documentation and code. | `AUD-005` |
| P1 | Rust `MODEL` omits many original Go assets, video metadata, user heartbeat details, gradients, partial Quest metadata and raw extension fields. | Data loss and partial type parity. | `MODEL-002`, `MODEL-003`, `PARSE-010` |
| P1 | Original Go `NewQuestManager` rejects enrollment-block marker at construction; Rust preserves it as diagnostics. | Intentional safer behavior difference must be covered by parity fixture, not described as exact 1:1. | `STATUS-017`, `AUD-002` |
| P1 | `PROGRESS.md` and `progress.json` originally required hand-adding a row for any new task; checker only changed known status cells. | New work can be omitted from the visible progress board. | `OPS-001`, `OPS-002` |
| P1 | Prior CI passed **15 tests** and Linux/Windows CLI smoke checks, but these do not establish full models, malformed inputs, full artifact lifecycle or current-commit verification. | Passing tests must not be described as complete port. | `TEST-002`, `QA-001` through `QA-008`, `REL-002` |
| P1 | Artifacts from Actions can expire; there is no tagged durable release, SHA256 manifest or license provenance bundle. | Users cannot verify a lasting published deliverable. | `REL-002`, `REL-003`, `REL-007` |
| P1 | Root README contains literal backslash-n escapes at some section boundaries. | Broken documentation rendering and navigation. | `AUD-007` |

## Findings that remain true, not new defects

- **Go method names != fully ported behavior.** Offline `QuestManager` provides correct filtering/inspection but not the original personal-account user login or engagement handlers.
- **Rust runs natively.** An earlier [GitHub Actions run](https://github.com/7ejal/Discord-Quest/actions/runs/41256646888) passed Linux's 15 tests and a Windows EXE smoke test.
- **Bounded parallel analysis exists**, using `std::thread::scope`, not Tokio. It is not a Quest execution scheduler.
- **Server-shaped top-level status is kept separate** from task local progress. This is good, but timestamp parsing and contradictory states need extra tests.
- The original Go source claims wide Quest coverage, while source-level logic includes console paths without independent console evidence and achievement flows specific to applications. **No universal completion proof exists.**

## Working approach for AI

1. Confirm the exact defect in the current source (do not rely only on this dated snapshot).
2. Select the linked canonical task ID from [progress.json](progress.json).
3. Implement a minimal test that fails with the old behavior.
4. Make the fix and rerun all associated Rust/CI checks.
5. Update tracker status and evidence with current commit/run; regenerate [PROGRESS.md](PROGRESS.md).
6. If a finding no longer applies, note the resolving commit rather than deleting its history.

This is an audit of **known gaps**, not a claim that no future bugs or upstream changes exist.
