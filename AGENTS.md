# AI agent entry point — Discord Quest Research

**This repository is a source-backed research and Rust engineering project, not a verified universal Discord Quest completer.** Do not use real personal Discord account tokens or develop inauthentic reward-earning features.

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
