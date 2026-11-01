# AI agent entry point — Discord Quest Research

**This repository is a source-backed research and Rust engineering project, not a verified universal Discord Quest completer.** Do not use real personal Discord account tokens or develop inauthentic reward-earning features.

## Current Linux-only override

Run Rust tests and package artifacts **only on `main-server`**, following [the Linux-only contract](rust/go-quest-port/migration/LINUX-ONLY.md). Do not run Windows tests or GitHub-hosted Actions. Keep the CLI offline; no GUI.

## Required starting point for a fresh AI

**[rust/go-quest-port/migration/START-HERE.md](rust/go-quest-port/migration/START-HERE.md)** and [the final pre-code audit](rust/go-quest-port/migration/PRE-CODE-REVIEW-2026-11-01.md) gives exact source, working directory, priority order, commands, evidence requirements and the out-of-scope boundary. The source-backed [gap audit](rust/go-quest-port/migration/GAP-AUDIT.md), [decision record](rust/go-quest-port/migration/DECISIONS.md), [38-case test plan](rust/go-quest-port/migration/TEST-MATRIX.md), and [release gates](rust/go-quest-port/migration/RELEASE-GATES.md) are also mandatory reading. **Only rust/go-quest-port is active; rust/quest-audit-cli is historical.**

## Where the active Rust work is tracked

- **[Go-to-Rust progress dashboard](rust/go-quest-port/migration/PROGRESS.md)** — current completed, partial, unstarted and excluded tasks.
- **[Canonical machine tracker](rust/go-quest-port/migration/progress.json)** — task IDs, status, acceptance, dependencies, evidence, source file links.
- **[Full implementation specification](rust/go-quest-port/migration/SPEC.md)** — scope, architecture, Definition of Done and tests.
- **[Every original Go file mapped](rust/go-quest-port/migration/GO-FILE-MAP.md)** — original 16 root entries and function clusters.
- **[Agent workflow](rust/go-quest-port/migration/AI-WORKFLOW.md)** — read/check/implement/test/update/commit protocol.
- **[Rust project instructions](rust/go-quest-port/AGENTS.md)**.

## Non-negotiable progress rules

Read the tracker before any repository edit. Pick exact task IDs; update the JSON and Markdown together. Mark `done` only after the task's acceptance test actually passes and evidence is recorded. Keep `excluded` separate from `todo`. Do not reuse historical CI runs as proof for newer code changes.

Use `python3 rust/go-quest-port/migration/check_progress.py --check` for tracker consistency. For Rust code, follow the project README's Cargo commands and verify CI on the current commit.

The Rust application currently handles **offline** Quest state inspection. It does not impersonate user clients, forge video/game/stream progress, solve CAPTCHA, or claim rewards. Do not change that scope based on upstream feature names.

For broader research, read [research/standalone](research/standalone), [audits](audits), and [docs](docs).
