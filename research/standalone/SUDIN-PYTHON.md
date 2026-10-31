# Detailed source walkthrough: SUDIN010 (Python CLI)

**Pinned upstream:** [main @ c14afa1dbb3c8c59d7f68c90caa305eff8e15b0e](https://github.com/SUDIN010/discord-auto-quest/tree/c14afa1dbb3c8c59d7f68c90caa305eff8e15b0e)  
**Reviewed:** 2026-11-01, static source only; no script execution, live completion, performance benchmark or full dependency analysis.

## Program type

One Python module, [main.py](https://github.com/SUDIN010/discord-auto-quest/blob/c14afa1dbb3c8c59d7f68c90caa305eff8e15b0e/main.py), approximately 546 lines, which contains both the network client and CLI. It does not require Discord desktop for the Python process to start.

Direct dependencies in [requirements.txt](https://github.com/SUDIN010/discord-auto-quest/blob/c14afa1dbb3c8c59d7f68c90caa305eff8e15b0e/requirements.txt): `requests>=2.31.0` (HTTP), `websockets>=12.0` (Gateway WebSocket) and `python-dotenv>=1.0.0` (configuration). `argparse`, `asyncio`, `logging`, `json`, `time`, `random` etc. are Python standard library.

## Key code units explained

| Function / class | What it means |
|---|---|
| `QuestClient` L76–142 | Owns an HTTP Session, common headers, error/retry policy, and operations for user, Quest, enrollment and task state |
| `QuestClient.request` L81–116 | Core sync HTTP wrapper, fixed timeout, bounded retries and error messages |
| `quest_tasks`, `quest_app_id` L145–158 | Get task map and target application ID |
| `enrolled`, `completed`, `task_progress`, `expired`, `not_started` L181–212 | Interpret Quest user state and timing |
| `task_priority`, `select_quests` L229–236, L418–435 | Sort and interactively choose work |
| `presence_worker` L250–315 | Separate async Gateway connection and periodic Gateway heartbeat; publishes a user presence |
| `wait_for_enrollment` L317–325 | Polls to detect a Quest accepted from the actual server |
| `complete_play_on_desktop` L326–359 | Attempts desktop-progress handler and checks returned Quest status |
| `complete_watch_video` L361–398 | Attempts timestamp-related video progress and checks completion state |
| `complete_quest` L401–409 | Dispatches only supported task families; logs explicit unsupported types |
| `run` L436–509 | CLI orchestration, user selection, optional enrollment, async Gateway worker and cleanup |
| `parse_args`, `main` L511–545 | Command-line argument parsing and process entry |

## Control flow from start to finish

1. `main()` parses optional CLI flags and calls `run(args)`.
2. `run` reads a personal credential from `DISCORD_TOKEN`, creates `QuestClient` and validates the user with `me()`.
3. It retrieves and filters campaigns, reports expired/not-yet-started/eligible states and optionally selects a subset.
4. For pending campaigns it attempts enrollment and then may wait for server acceptance.
5. It establishes a separate async Gateway presence worker, while regular HTTP requests are executed through a wrapper from the async context.
6. For each chosen Quest, `complete_quest` chooses **desktop gameplay** or **video/mobile video**. Unsupported types are explicitly rejected in the log.
7. The handler follows returned server progress, outputs a final status, clears presence and exits.
8. Reward claiming is **manual only**; no automatic claiming is implemented.

Read [main.py L436–545](https://github.com/SUDIN010/discord-auto-quest/blob/c14afa1dbb3c8c59d7f68c90caa305eff8e15b0e/main.py#L436-L545).

## Critical distinction: its two network connections

**The Requests Session** sends ordinary HTTP requests and parses JSON responses with a 30-second timeout. On 429, it waits according to the response; on 5xx and connection failure it retries with backoff, with a configured `MAX_RETRIES = 5`. On a permanent error it raises `QuestError` rather than ignoring it. This design is clearer and more bounded than the Go client discussed in [GO.md](GO.md).

**The Gateway WebSocket** provides an asynchronous connected-client presence while the process runs, with its own heartbeat. It is **not the same as a Quest heartbeat** and does not independently prove that a game is running.

See [main.py L17–29, L76–116, L250–315](https://github.com/SUDIN010/discord-auto-quest/blob/c14afa1dbb3c8c59d7f68c90caa305eff8e15b0e/main.py).

## Quest logic

**Discovery:** The program uses a small priority list `PLAY_ON_DESKTOP`, `WATCH_VIDEO`, `WATCH_VIDEO_ON_MOBILE`. It does not have task implementations for STREAM, ACTIVITY, achievement or linked console categories. This narrower scope is more transparent than projects that advertise every task type because they recognize a string.

**Gameplay handler:** Repeats a request and reads the returned `completed_at` or progress. If the response lacks expected progress, it stops with a warning. It distinguishes success from “stopped before completion.” **Residual flaw:** If the server continues reporting unchanged but valid-looking progress, the loop can run without an overall deadline. [main.py L326–359](https://github.com/SUDIN010/discord-auto-quest/blob/c14afa1dbb3c8c59d7f68c90caa305eff8e15b0e/main.py#L326-L359).

**Video handler:** Uses elapsed-time accounting and a requested progress value, but reports success only if a returned server response contains a completion marker. If that never arrives it prints that the task was not marked complete yet, which is more honest than an unconditional success log. This does not establish live functionality. [main.py L361–398](https://github.com/SUDIN010/discord-auto-quest/blob/c14afa1dbb3c8c59d7f68c90caa305eff8e15b0e/main.py#L361-L398).

**Task schema:** `quest_app_id` and task mapping should be compared with the current per-task application data model. No general AND/OR task satisfaction algorithm is implemented. A Quest requiring multiple actions is not guaranteed to complete even if its one selected handler finishes.

**Enrollment:** `wait_for_enrollment` can keep polling forever if the campaign never becomes enrolled. The main program has cleanup of presence on exit, but a never-ending enrollment wait delays that exit. [main.py L317–325](https://github.com/SUDIN010/discord-auto-quest/blob/c14afa1dbb3c8c59d7f68c90caa305eff8e15b0e/main.py#L317-L325).

## Why this is an interesting reference

- **Explicit error handling** with a dedicated `QuestError` type, rather than blanket ignore.
- **Per-HTTP-call timeout and bounded retries**.
- **Clear unsupported-task scope** and manual reward claiming.
- **User-facing CLI** with readable output and selection.
- **Separate async Gateway work** even though the Requests client is synchronous.

## What remains incomplete

- No general AND/OR task support or platform preference handling.
- No real verification for streams, activities, consoles or publisher achievements.
- Potential indefinite progress/enrollment loops.
- No unified per-task time budget.
- No automated test suite detected in the examined root.
- Runtime/CPU/RAM and successful Quest completion were not tested.
- Personal-token use and simulation still violate Discord's rules.

**Verdict:** The **clearest narrow standalone Python codebase for learning control flow and HTTP reliability** among these five, not the tool with widest task coverage.
