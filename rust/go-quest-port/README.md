# Go Discord Quest project → Rust port (offline, read-only)

**Status:** **Built and tested successfully in GitHub Actions on November 1, 2026.** The Linux job passed all **15 Rust unit tests** and a CLI smoke test. The Windows job compiled `go-quest-port.exe`, passed a Windows CLI smoke test, and uploaded its executable artifact. [Successful CI run](https://github.com/7ejal/Discord-Quest/actions/runs/41256646888).

**Upstream:** [yeweroooo/Discord-Quest-Auto-Completion-Selfbot](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot), commit [352d7f457c9511bc9e7c9b940a71012bfb47572e](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/tree/352d7f457c9511bc9e7c9b940a71012bfb47572e).

This is a Rust port of the **reusable Go Quest data model, campaign filtering, classification, status evaluation and concurrent processing**. It is **not an operational selfbot** and does **not** copy the Go project's personal-account login, synthetic engagement/video/streaming methods, automated challenge solving, or reward-claiming behavior. Discord [does not permit applications that simulate Quest activity to earn rewards](https://discord.com/safety/platform-manipulation-policy-explainer).

## Active Rust project — start here

This is the **only active Rust CLI** in this repository. The separate `rust/quest-audit-cli` is a superseded non-runnable research library; never implement features there. For each task, start at [migration/START-HERE.md](migration/START-HERE.md), [migration/GAP-AUDIT.md](migration/GAP-AUDIT.md), and [migration/TEST-MATRIX.md](migration/TEST-MATRIX.md). The generated [progress dashboard](migration/PROGRESS.md) must agree with the canonical JSON and pass CI.

## AI progress tracker (read this before editing)

**[Master migration progress checklist](migration/PROGRESS.md)** · **[Machine-readable task IDs, evidence and acceptance](migration/progress.json)** · **[Full Go-to-Rust spec](migration/SPEC.md)** · **[All original Go files/functions mapped](migration/GO-FILE-MAP.md)**.

AI coding agents should follow [AGENTS.md](AGENTS.md) and [migration/AI-WORKFLOW.md](migration/AI-WORKFLOW.md). Every code change must identify task IDs, pass the relevant tests, and update both trackers. Run `python3 migration/check_progress.py --check`. **Excluded** tasks are intentional, not unfinished features.

## What works in the source

- Parse Go-compatible Quest JSON supplied from a **local file**: a single Quest, array, or `{"quests":[...]}` wrapper.
- Preserve Go methods' intent: `is_expired`, `is_enrolled`, `is_completed`, `has_claimed_rewards`, `is_android_video_only`, `original_go_first_task`.
- Discover and classify all 10 documented current/historical task names.
- Parse either current `task_config_v2` or historical `task_config`; evaluate `and` / `or` groups, and mark unknown multi-task operators unsupported.
- Inspect per-task application IDs, falling back to campaign-level ID only when missing.
- Distinguish **local numeric goal**, **server task completed**, **server Quest completed**, and **reward claimed**.
- Print a readable status report or JSON output.
- Bounded parallel evaluation using Rust scoped threads, preserving original Quest order.
- Synthetic fixtures and unit tests that target the specific false-success problems in the Go and Python projects.
- **No network calls or credentials**.

## Windows setup

Install a stable Rust toolchain using [rustup.rs](https://rustup.rs/), open PowerShell in this directory and run:

```powershell
cargo test
cargo run --release -- demo
cargo run --release -- inspect .\examples\sample-quests.json
cargo run --release -- inspect .\examples\sample-quests.json --json
cargo build --release
```

The built executable is in `target\release\go-quest-port.exe` on Windows. **[Download the verified Windows EXE artifact from GitHub Actions](https://github.com/7ejal/Discord-Quest/actions/runs/41256646888/artifacts/13063362600)** (GitHub sign-in may be required; artifacts may expire). You can create a `.bat` wrapper to launch that executable; `.bat` is only a launcher.

On Linux/macOS: `cargo run --release -- demo`.

## CLI options

```text
go-quest-port demo [--json] [--workers N]
go-quest-port inspect <local-quests.json> [--json] [--workers N]
```

- `demo`: uses bundled **fictional synthetic** Quests. No outside access.
- `inspect`: reads an existing local JSON file. This does **not** fetch live account data.
- `--json`: machine-readable report.
- `--workers N`: between 1 and 32 analysis threads (default 4).

## Original Go → Rust mapping

| Go file/function | Rust equivalent | Notes |
|---|---|---|
| `go.mod` | `Cargo.toml` | Rust native binary; Serde JSON and chrono |
| `types.go` | `src/model.rs` | Typed Quest models, optional statuses, per-task app IDs |
| `quest.go:IsExpired` | `Quest::is_expired` | RFC3339 parsing through chrono |
| `quest.go:IsCompleted/IsEnrolled/HasClaimedRewards` | `Quest::is_completed/is_enrolled/has_claimed_rewards` | Server status timestamps only |
| `quest.go:GetTaskName` | `Quest::original_go_first_task` | **For comparison only**; it is flawed for AND/OR |
| `quest.go:IsAndroidQuest` | `Quest::is_android_video_only` | Classifies mobile-video-only campaigns |
| `manager.go:FilterValid` | `QuestManager::filter_valid` | Noncompleted and nonexpired |
| `manager.go:FilterRedeemable` | `QuestManager::filter_redeemable` | Confirmed completion, reward not claimed |
| `main.go` Quest goroutines + WaitGroup | `analyze_parallel` | Bounded scoped Rust threads, no network side effects |
| `main.go` | `src/main.rs` | Offline `demo` and `inspect` CLI |
| Go engagement-specific task handlers | **Not ported** | No simulation of rewarded activity |
| `client.go` personal-account WebSocket/REST/Identify | **Not ported** | No credential-based client automation |
| `captcha.go`, OAuth achievement and claiming logic | **Not ported** | Account safety and reward integrity |

**Important engineering improvements** over original Go:

1. Correctly evaluate `AND/OR`; never treat the first matching task as entire Quest completion.
2. Check the **server Quest completion timestamp** instead of printing success after a task-local timer.
3. Distinguish status from claim and redemption; completed does not mean claimed.
4. Preserve unknown/deprecated task type distinctions, with explicit warnings.
5. Use bounded concurrency rather than a new unbounded goroutine for every campaign.
6. Keep the source local and avoid handling personal account credentials.

## Project files

```text
rust/go-quest-port/
├── Cargo.toml
├── README.md
├── examples/sample-quests.json
└── src/
    ├── lib.rs          # Parser and 15 unit tests
    ├── main.rs         # CLI entry point
    ├── manager.rs      # QuestManager, task group evaluation, parallel analysis
    └── model.rs        # Typed Go-compatible JSON models
```

## Testing/verification status

- Source file presence verified in the GitHub repository.
- **15 unit tests passed** using invented offline JSON (Linux GitHub Actions).
- **Linux CLI smoke test passed.**
- **Windows EXE release build and CLI smoke test passed**, with [artifact uploaded](https://github.com/7ejal/Discord-Quest/actions/runs/41256646888/artifacts/13063362600).
- [Full successful workflow](https://github.com/7ejal/Discord-Quest/actions/runs/41256646888).
- No Rust CPU/RAM performance measurements have been taken.
- No Discord Quest has been executed, and no universal Quest completion claim is made.

For research and evidence: [detailed Go audit](../../audits/GO-YEWEROOOO.md) and [method comparison](../../research/standalone/METHODS-DEEP-DIVE-2026-11-01.md).
