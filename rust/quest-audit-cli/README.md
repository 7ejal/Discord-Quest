> **SUPERSEDED RESEARCH PROTOTYPE — DO NOT CONTINUE DEVELOPMENT HERE.** The active native Rust CLI is [`rust/go-quest-port`](../go-quest-port/README.md). All tasks, work status, tests, release criteria, and agent instructions are tracked in [`rust/go-quest-port/migration/START-HERE.md`](../go-quest-port/migration/START-HERE.md) and [`progress.json`](../go-quest-port/migration/progress.json). This older library is retained only as historical reference. It has **no `src/main.rs`** and is not a verified runnable CLI.

# Rust Quest status analyzer — library foundation (WIP)

**Status (2026-11-01):** Rust library source committed and unit tests authored. This is **not yet a runnable CLI executable**: the `src/main.rs` binary entry point has not been published. Compilation/tests have **not been executed** in this research environment.

This is a **read-only, offline** source-quality project. It does not connect to Discord, authenticate user accounts, submit progress, simulate gameplay/streaming, or claim any reward. Discord's [Platform Manipulation Policy](https://discord.com/safety/platform-manipulation-policy-explainer) prohibits simulated Quest completion.

## What exists

- [Cargo.toml](Cargo.toml): Rust package, with Serde and serde_json dependencies.
- [src/lib.rs](src/lib.rs): typed Quest JSON structures, support for both current v2 and legacy task config, basic classification for 10 task IDs, AND/OR task groups, separate numeric progress vs task completion vs Quest completion vs reward claimed.
- **14 synthetic unit tests** inside `src/lib.rs` for AND/OR, numeric progress false positives, claimed-but-uncompleted inconsistency, unknown schema, legacy config and document wrappers.

## Intended workflow

1. Read and parse synthetic or *locally supplied* Quest-shaped JSON.
2. Assess progress without inventing missing enrollment or successful completion.
3. Report `in_progress`, `tasks_confirmed_quest_pending`, `quest_confirmed_unclaimed`, `reward_claimed`, `unsupported`, `inconsistent` and `not_enrolled` distinctly.
4. Test correctness using fake data and known anti-patterns.

### Build/testing status

The library is designed for a standard Rust toolchain (`cargo test` in this directory), **but has not been compiled or tested**, and its current status must not be presented as passing.

There is **no runnable binary** in this folder yet.

## What we still need to validate

- Rust compiler and test suite execution on Windows/Linux.
- The supported task data model against current, legitimately obtained, sanitized fixtures.
- Correct handling of missing/invalid nested fields and unusual task configurations.
- Whether diagnostics can be made useful without collecting account tokens.
- A standalone command-line interface for *offline analysis only*.

For engineering research, see [the standalone method deep dive](../../research/standalone/METHODS-DEEP-DIVE-2026-11-01.md) and [offline fixture plan](../../research/standalone/METHOD-VALIDATION-FIXTURES.md).
