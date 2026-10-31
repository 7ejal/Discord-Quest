# Detailed source walkthrough: yeweroooo (Go selfbot)

**Pinned upstream:** [golang @ 352d7f457c9511bc9e7c9b940a71012bfb47572e](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/tree/352d7f457c9511bc9e7c9b940a71012bfb47572e)  
**Reviewed:** 2026-11-01. Code reviewed statically, not run or independently verified against live Discord Quests.

## What kind of program is this?

A compiled Go CLI. The Go toolchain is needed to build, but a compiled binary does not need Python, Node or Discord desktop. The `go.mod` declares Go 1.23 and only two direct dependencies: `gorilla/websocket` (persistent Gateway connection) and `joho/godotenv` (configuration). Standard library packages handle HTTP, JSON, time and concurrency. [go.mod](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/go.mod).

The README advertises substantially lower memory than Node, but those numbers are author-provided **not measured in this review**. No committed Go `*_test.go` tests were found in the inspected project root.

## File map and responsibilities

| File | Main responsibility | Notable code |
|---|---|---|
| [main.go](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/main.go) | Process startup, configuration, orchestration and lifecycle | `main()` |
| [client.go](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/client.go) | REST/Gateway client, Identify/READY, Gateway heartbeats, optional presence, HTTP wrapper | `Connect`, `readLoop`, `Request`, `FetchQuests` |
| [manager.go](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/manager.go) | Filter campaigns, enrollment, task dispatch, outcome notifications and optional claim | `DoingQuest`, `doingWatchVideoQuest`, `doingPlayOnPlatformQuest`, `doingPlayActivityQuest`, `doingStreamOnDesktopQuest`, `doingAchievementInActivityQuest`, `RedeemQuest` |
| [quest.go](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/quest.go) | IsEnrolled/IsCompleted/IsExpired and task selection | `GetTaskName` |
| [types.go](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/types.go) | Typed shapes for Quest/config/user status/Gateway events | `Quest`, `TaskConfigV2`, `QuestUserStatus` |
| [constants.go](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/constants.go) | Client properties and request headers | `MakeHeaders` |
| [utils.go](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/utils.go) | Version lookup, webhooks, app-provider helper | `FetchAllServerProps` |
| [captcha.go](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/captcha.go) | Optional third-party challenge-service integration | `SolveCaptcha` |

## Actual startup call sequence

1. `main()` checks a required user credential and optional webhook/challenge configuration; sets up its client properties.
2. `NewDiscordClient(token)` creates a client that holds the credential and Gateway state.
3. `Connect()` opens a WebSocket to Discord's Gateway; `readLoop()` processes HELLO and READY and starts periodic Gateway heartbeats. This makes the independent Go process look like a connected client to the service. A Gateway heartbeat **is not Quest progress**.
4. The READY callback calls `FetchQuests()`, constructs `QuestManager` and calls `FilterValid()`.
5. For every candidate Quest, `main.go` starts a **goroutine**, waits on `sync.WaitGroup`, and calls `DoingQuest` for each; concurrent work is unbounded by a configured worker limit.
6. `DoingQuest` checks enrollment, reads a single task from `GetTaskName`, and selects one handler.
7. After all goroutines finish, the Gateway connection closes.
8. **Default reward claiming is NOT run:** the `FilterRedeemable` / `RedeemQuest` calls in `main.go` are commented out.

Sources: [main.go L11–89](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/main.go#L11-L89), [client.go L82–207](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/client.go#L82-L207).

## Code logic, explained

**Quest model:** `IsCompleted` tests whether a server-shaped `completed_at` field is non-null; `HasClaimedRewards` checks `claimed_at`. That separation is reasonable. However, `GetTaskName` walks a **hardcoded list** and takes the first name in the task map. It does **not** read/evaluate `join_operator`; an AND campaign might need two tasks, and a chosen task may be unsuitable for the current platform. [quest.go L17–57](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/quest.go#L17-L57).

**Video handler:** Builds timestamp-related progress from elapsed time since enrollment and posts it to a video-progress operation. It advances its own `secondsDone` even when a request fails, and after a final unvalidated attempt prints **“completed!”** even if server `completed_at` was never returned. The precise defect is **the missing final validation**, not the programming language. [manager.go L173–225](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/manager.go#L173-L225).

**Gameplay handler:** Repeatedly submits a heartbeat and updates cached `quest.UserStatus` from a successful response, then checks `IsCompleted`. Desktop, Xbox, PlayStation and deprecated Desktop V2 **share this same handler**. That does **not** demonstrate an Xbox or PlayStation session exists. If the service never confirms completion, the loop has no task-level maximum elapsed time and may keep running. [manager.go L113–146, L227–265](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/manager.go#L227-L265).

**Activity handler:** Similar heartbeat loop but uses a fixed synthetic `call:1:1` session key. No genuine activity/voice session is constructed or verified; accepted request != validated activity. [manager.go L268–308](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/manager.go#L268-L308).

**Streaming handler:** Sends a streaming presence event and uses gameplay-like progress heartbeats. Its own comment admits Discord may require genuine Go Live and a real viewer. “Streaming support” describes a code path, not a validated stream. [manager.go L392–451](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/manager.go#L392-L451).

**Activity achievement:** Separate sensitive application-authorization flow involving an external activity provider. It is *not* generic `ACHIEVEMENT_IN_GAME`. Failure after authorization but before the cleanup call can leave a grant behind; no robust `defer`/finally guarantee wraps the complete authorization flow. [manager.go L310–391](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/manager.go#L310-L391).

**Reward claiming:** A separate function handles claim attempts and optional third-party challenge solving. The default entry point never calls it. It also logs part of a returned challenge token, an inappropriate practice for credentials/challenges. [main.go L70–79](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/main.go#L70-L79), [manager.go L491–559](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/manager.go#L491-L559).

## Network/robustness analysis

- REST is constructed in `client.go` and uses `http.DefaultClient` rather than a per-request context with explicit maximum deadline. A stalled request can stall a goroutine.
- HTTP 429 retries are implemented by **recursively calling the same Request function** after sleeping; the code has no configured maximum retry count. A sufficiently persistent rate limit can consume resources indefinitely.
- Several HTTP helper requests ignore errors or response statuses and a webhook helper constructs raw JSON by string concatenation; this can produce invalid JSON for unescaped text.
- The default manager returns an error on enrollment block, which prevents *even already enrolled* Quest processing when that marker is set.
- The code retrieves client version/build properties dynamically; this reduces reliance on one static build value but creates dependencies on Discord asset formats and external HTTP availability.
- Concurrent goroutines can increase rate-limit pressure without a global request scheduler.

Evidence: [client.go L231–303](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/client.go#L231-L303), [manager.go L23–43](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/manager.go#L23-L43), [utils.go L257–268](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/utils.go#L257-L268).

## Bottom line

**Strength:** Small, separable compiled Go codebase; straightforward native executable distribution; broad task-switch structure.  
**Weakness:** Does not mean all Quest types work: untested console paths, fake activity session, unsupported external game achievements, streaming uncertainty, incorrect AND/OR selection, unbounded retries and false-positive success logs.  
**Assessment:** Best for studying the *standalone Go architecture*; not the best source-backed model of truthful completion outcomes or reliable task selection. **No live benchmark or production completion proof.**
