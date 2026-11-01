# AI START HERE — next-session handoff

**Read this page first.** This file is meant for an AI agent without previous conversation context.

## Mandatory final audit gate

**[Pre-code review (127 source-backed tasks)](PRE-CODE-REVIEW-2026-11-01.md)** supersedes the old assumption that 114 tasks are exhaustive. The audit downgraded `STATE-005` and `QA-008` to partial and added model parity, untrusted-data provenance, terminal escaping, native runtime and differential verification tasks. Never start by assuming a Go `join_operator` field exists; the pinned Go TaskConfigV2 has no such field. If code changed after this audit, repeat the source check.

**Current platform rule:** [LINUX-ONLY.md](LINUX-ONLY.md): Linux x86_64 CLI only, main-server verification exclusively, no hosted/Windows CI. This supersedes stale platform examples below.

## Mission

Complete the **supported native Rust offline Quest inspection** application under `rust/go-quest-port`. Do not create another sibling crate. Do not build an operational personal-account selfbot, synthetic rewarded engagement, CAPTCHA bypass, or automatic reward claims. **No Discord installation or personal token is needed for the supported CLI.**

**Source Go reference:** [`yeweroooo/Discord-Quest-Auto-Completion-Selfbot` pinned commit `352d7f457c9511bc9e7c9b940a71012bfb47572e`](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/tree/352d7f457c9511bc9e7c9b940a71012bfb47572e). Compare code to pinned revision, do not silently trust newest HEAD.

## Dependency-aware tasks (live generated queue)

Open [PROGRESS.md](PROGRESS.md): **Ready to start** lists release-gating tasks whose dependencies are all marked DONE; **Blocked release gates** lists unfinished prerequisites with their statuses and priorities. The queue is regenerated from [progress.json](progress.json) with `python3 migration/check_progress.py --write`. Do not rely on static lists from earlier discussions.

## Required main-server test step

After Rust code changes, use the user-authorized [main-server validation workflow](MAIN-SERVER-WORKFLOW.md), not GitHub Actions alone. Work in the dedicated isolated checkout, use fast-forward Git import, cap CPU use, record tested SHA, and then package the Linux release candidate on main-server. Do not run untrusted PR code on production.

## Every session, follow this order

1. Read [`../AGENTS.md`](../AGENTS.md), [SPEC](SPEC.md), [GO-FILE-MAP](GO-FILE-MAP.md), [GAP-AUDIT](GAP-AUDIT.md), [DECISIONS](DECISIONS.md), [RELEASE-GATES](RELEASE-GATES.md).
2. Read canonical [progress.json](progress.json), then generated [PROGRESS.md](PROGRESS.md). The JSON task IDs, **priority**, **required_for_release**, dependencies, acceptance and evidence govern work. Never “mark complete” from a README statement.
3. Inspect actual `src/{model,manager,lib,main}.rs`, `Cargo.toml`, and latest CI. Record current branch and commit.
4. Pick at most a few unblocked **P0 release-gating** tasks. Start by writing a regression test that fails for the known defect.
5. Implement it correctly, with no placeholder no-op code. Keep code simple and maintainable.
6. Run test, fmt, Clippy, CLI smoke and tracker checker. Confirm the **current commit** passes Linux main-server checks before saying verified.
7. Update progress.json evidence; regenerate Markdown; update GO-FILE-MAP or decisions when semantics change.
8. Commit/test/docs/manifest updates together. Report exact task IDs and remaining risks.

## Current source coverage

See [model/behavior ledger](MODEL-COVERAGE.md), [acceptance evidence](ACCEPTANCE-EVIDENCE.md)
and [continuation verification](MAIN-SERVER-COMPLETION-2026-11-01.md). Do not
repeat historical completed phases; consult the generated queue. Modern schema
evidence and public publication remain open. Windows/GUI work is outside scope.

## New requested live HTTP milestone — not implemented

The user requested Go-style raw `Authorization` header login plus real account/Quest tests. The [requested feature backlog](GO-LIVE-HTTP-REQUEST.md) and dedicated tasks in `progress.json` record this separately from the supported offline Linux CLI. First read `GO-HTTP-AUTH-AUDIT-2026-11-01.md`, `GO-LIVE-PARITY-DECISION-2026-11-01.md` and the remaining network tasks; do not build against unverifiable private APIs, collect personal tokens in chat, or mark a live HTTP feature done until actual permission/schema and mock-test evidence exists. The former incomplete `src/account.rs` was removed, and `main.rs` still provides only `demo` / `inspect`.

## Immediate engineering order (avoid chasing optional features)

**Phase 1 — Parser and truthfulness:** `PARSE-001` reject blank objects, `PARSE-002` duplicate-ID policy, `STATUS-010` validate timestamps, `STATUS-012` missing/invalid expiry, `PARSE-005` bound input memory, `AUD-001` verify schema variants.

**Phase 2 — Models:** `MODEL-002/003`, `PARSE-003` excluded Quest data, `PARSE-010` field coverage, `MODEL-007` typed task variants and unknowns.

**Phase 3 — CLI and tests:** `UX-002` output schema; `STATUS-016` deterministic clock; `TEST-002/003`, `QA-003/005`; format/Clippy and lockfile.

**Phase 4 — Release:** `ENV-009` upstream MIT notice; `REL-001/002/003/006/007` tagged Linux builds, checksums, stable release, evidence.

**Parallel docs/agent safety:** `AUD-003` explicitly separate incomplete older Rust crate. Keep [CAPABILITY-CONTRACT](CAPABILITY-CONTRACT.md) accurate.

Do not treat this static priority list as more current than JSON. New blockers may appear during testing.

## Local commands (inside rust/go-quest-port)

```sh
python3 migration/check_progress.py --check
CARGO_BUILD_JOBS=2 nice -n 10 bash scripts/test-linux.sh
CARGO_BUILD_JOBS=2 nice -n 10 bash scripts/package-linux.sh
```

Once `ENV-005/006` are completed, also require `--locked`, `cargo fmt --check`, and strict Clippy in CI. This project no longer targets Windows. Do not test or build on GitHub-hosted runners; use `scripts/test-linux.sh` and `scripts/package-linux.sh` on main-server.

## Update manifest example

```json
{
  "id": "PARSE-001",
  "status": "partial",
  "priority": "P0",
  "required_for_release": true,
  "evidence": ["src/lib.rs", "tests/parser_validation.rs"],
  "notes": "Missing required fields still need a failing test"
}
```

This is **an illustrative subset of a task**. Never overwrite the entire task collection with this example.

**Rules:** `done` requires acceptance and evidence. `partial` means real work exists but criteria remain. `todo` unimplemented. `excluded` outside scope. When adding a new task ID, do so in JSON and regenerate Markdown. CI must reject a desynchronized tracker.

## Handoff response after completing an assigned task

```text
Tasks: <IDs>
Changed: <exact paths>
Verified: <tests run and pass/fail>
CI: <current-commit URL or "not verified">
Tracker: <before -> after states>
Go parity: <same / intentionally different / excluded>
Remaining P0 release blockers: <IDs>
Risks / warnings: <specific>
```

**Historical known-good build:** [GitHub Actions #41256646888](https://github.com/7ejal/Discord-Quest/actions/runs/41256646888), 15 Rust tests and Windows EXE smoke test. It is NOT proof for future commits.

**Canonical home:** [PROGRESS.md](PROGRESS.md); **not** a story about a 1:1 universal Quest completer.
