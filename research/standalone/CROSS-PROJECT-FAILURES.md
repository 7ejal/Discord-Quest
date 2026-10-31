# Cross-project deep comparison: failure mechanisms and research priorities

**Source-based assessment on 2026-11-01.** This compares five core standalone programs and two additional no-desktop clients. It does **not** claim live Quest success, provide evasion instructions, certify binary safety, or replace the official Discord product requirements.

## New source-verification corrections (November 1, 2026)\n\nA second line-by-line examination found [SUDIN source details](METHODS-DEEP-DIVE-2026-11-01.md): **(1)** its desktop handler treats numeric progress at target as a local `finished` status even without Quest `completed_at`; **(2)** its video handler can begin `finished=true` from cached numeric progress; **(3)** its Gateway presence is built from `matches[0]` and is not updated when later selected Quests are processed. **SUDIN remains a good compact example of bounded HTTP retries, but not a strict server-confirmation reference.**\n\nThe [offline validation fixtures](METHOD-VALIDATION-FIXTURES.md) reproduce these cases using invented data and mock transport only.\n\n## A. What “best” should mean

A project with nine `case` labels is **not** automatically better than a project with three handlers. Count **server-confirmed outcomes under legitimate task conditions**, not source-code labels. The strongest implementation should detect unknown Quest schemas, treat enrollment and completion as separate states, stop on terminal errors, preserve user credentials, and report uncertainty accurately.

### Comparison matrix — core five

| Engineering property | Go yeweroooo | TypeScript aiko | Python SUDIN | Python unconsents | TypeScript lfathh |
|---|---|---|---|---|---|
| Starts without Discord desktop | Yes | Yes | Yes | Yes | Yes |
| Single native EXE without runtime | Go build output | No, Node normally required | No, Python normally required | No, Python normally required | No, Node normally required |
| Gateway library | gorilla/websocket | discord.js WebSocketManager | websockets | discord.py-self | discord.js WebSocketManager |
| HTTP library | Go net/http | @discordjs/rest + undici | requests | aiohttp | @discordjs/rest |
| Task selection | Fixed first match | Fixed first match | Limited priority/dispatch | Fixed first match | Fixed first match on legacy config |
| Composite AND/OR | Not properly evaluated | Not properly evaluated | Not general | Not properly evaluated | Not properly evaluated |
| Concurrent tasks | Goroutine per Quest | Promise.allSettled | Sequential selected Quests | asyncio per account/tasks | Promise.allSettled |
| HTTP 429 control | Recursive, **uncapped** | Discord.js REST scheduler | **Bounded** request attempts | Generic errors; no dedicated bounded 429 policy | Discord.js REST scheduler |
| Request deadline | No configured overall net/http deadline | Library-specific, review version needed | **30 seconds per HTTP call** | No explicit bounded policy in wrapper | Library-specific |
| Whole-task deadline | No on several handlers | No on several handlers | No on certain stalls | No on generic heartbeat | No on certain stalls |
| Verified final Quest-level status | No consistent final refetch | No consistent final refetch | Some returned completed_at checks | No consistent final refetch | No consistent final refetch |
| Misleading “done” logs | Video path | Video path | Narrower/conditional | Video and job-level logs | Video path |
| Claim function exists | Yes | Yes | No | No | No current general implementation |
| Claims enabled by default | **No** (commented) | **No** (commented) | No | No | No |
| Account credential handling | Environment variable | Environment variable | Environment variable | **Plaintext tokens.txt** | Environment variable |
| Running tests measured in this review | None | None | None | None | None |

**SDK/library additional risks:** [xgg-2](EXTRA-NODE-AND-HOSTED.md) has a swallowed-error false-success path; [SubhuJaat07](EXTRA-NODE-AND-HOSTED.md) stores a raw token in a browser cookie and marks local completion without final server confirmation.

## B. Failure categories, step by step

### 1. Task selection failure

**Observed:** Go `GetTaskName()`, aiko `doingQuest()`, unconsents `get_task()`, and lfathh manager take the first known task name.

**Why wrong:** Quest config can say `join_operator="and"` and require multiple tasks. A project can announce one task completed and exit before the whole Quest is satisfied. An OR campaign can also have a platform preference, or the first task may be unsupported on the current account.

**Offline regression fixture:** Test one AND Quest with a completed video and incomplete Activity, and one OR Quest with a completed alternative. Check that a parser does **not** mark the former done.

### 2. False-success reporting

**Observed:** Go, aiko, lfathh and unconsents can log video completed without a confirmed top-level server status; xgg-2 can return completed after a swallowed solver error; SubhuJaat07 synthesizes a local `completedAt`.

**Why wrong:** A local timer or accepted request may differ from the authoritative account state.

**Offline regression fixture:** Mock a 2xx response with `completed_at=null` and a second task still incomplete. A truthful analyzer reports `pending/unknown` rather than `completed`.

### 3. Indefinite waits / memory and resource use

**Observed:** Go HTTP 429 recursion and status loops, aiko task loops, unconsents heartbeat `while True`, SUDIN gameplay/enrollment wait without total deadlines.

**Why wrong:** A program that never receives progress can remain live forever, leading to wasted memory/CPU, repeated traffic and a misleading “working” indicator.

**Offline regression fixture:** Mock unchanging progress for a long duration and verify explicit `stalled` or `timed_out` state rather than an endless wait. Unit test retry counts, deadlines, cancellation and cleanup.

### 4. Wrong task-specific environment

**Observed:** Fixed or synthesized Activity session keys, gameplay-like handlers reused for streaming, generic desktop methods mapped to console task labels.

**Why wrong:** A real Discord Activity, console game or stream may require contextual evidence that the network request alone does not create. **No all-Quest guarantee** can follow from merely supporting a task string.

**Fixture principle:** Classify `requires_real_stream`, `requires_console_game` and `requires_publisher_achievement` as explicit preconditions, not magic handler types.

### 5. Outdated Quest schema

**Observed:** lfathh standalone uses `task_config` while the modern unofficial reference uses `task_config_v2`. Several standalone projects read a single top-level `config.application.id` and ignore per-task `applications` arrays.

**Why wrong:** The result can be wrong application association, failure, or an outright exception when the expected field is absent.

**Fixture principle:** Parse current and historical versions as distinct data shapes, preserve unknown task types, and fail closed on unsupported schema.

### 6. Enrollment/expiry/claim confusion

**Observed:** Go manager stops on a general enrollment-block marker; unconsents creates a local fallback `enrolled_at` timestamp after an empty response; Go and aiko have claim functions but their main scripts do not invoke them.

**Why wrong:** A shown campaign isn't necessarily enrolled; enrollment isn't a completed Quest; completed isn't a claimed reward; claimed isn't externally redeemed.

**Fixture principle:** Use independent, strongly typed server states.

### 7. Error handling and diagnostics

**Observed:** SUDIN has bounded HTTP retries, but some whole-task loops can stall. Go has no total network deadline and infinite rate-limit recursion. aiko has a mature REST library but doesn't reconcile all `Promise.allSettled` results. Unconsents logs errors but continues.

**Why wrong:** Error handling at the HTTP layer is different from end-to-end success criteria. Both are needed.

### 8. Credential and authorization risks

**Observed:** All five require personal user tokens; unconsents uses plaintext `tokens.txt`, prints a token prefix on login failure; Go and aiko may use third-party challenge services and sensitive app authorization; certain flows can exit before revocation.

**Why wrong:** Personal account tokens provide broad privileges. Third-party hosting, CI, plugins or external services create additional custody and supply-chain risks.

**Privacy-first guideline:** Offline source analysis and synthetic fixtures need **no real user token**. Read-only documentation/testing must never require one.

## C. The best source for each coding concept

| What you want to learn | Best available example |
|---|---|
| Standalone executable / Go concurrency / program structure | yeweroooo |
| Node HTTP/Gateway client design via established libraries | aiko |
| Basic CLI state handling and bounded HTTP retries | SUDIN |
| Async Python multi-account task scheduling (not credential security) | unconsents |
| CLI terminal dashboard formatting and progress presentation | lfathh |
| Standalone extensible solver pattern with an error-propagation counterexample | xgg-2 |
| Hosted queues/state persistence and authentication anti-patterns | SubhuJaat07 |
| Correct AND/OR interpretation and fail-closed claim parsing, though plugin not standalone | [saintordevil](../../audits/SAINTORDEVIL.md) |
| Progress watchdogs and extensive regression tests, though plugin/userscript not standalone | [Orion](../../audits/ORION.md) |

## D. Which existing standalone implementation is best?

**Depends on your objective:**

- **Native executable format:** yeweroooo Go, because compilation creates a self-contained binary with a small dependency list. This is about packaging, **not Quest coverage correctness**.
- **Understandable limited code and error handling:** SUDIN Python, because code distinguishes unsupported tasks and HTTP retries are bounded.
- **Existing Task-type labels:** Go appears widest by claimed count, but several are explicitly untested, one is deprecated, and other families require genuine external validation. Do **not** present it as proven best at completion.
- **TypeScript architecture:** aiko is a more contemporary standalone reference than lfathh but has significant functional limits.
- **Avoid as a credential or completion-truth model:** unconsents (plaintext tokens, error-loop behavior), xgg-2 (false-success semantics), SubhuJaat07 (raw cookie token/local completion).

No legitimate Quest user should infer that any project can complete every Quest without the required video, game, stream or publisher participation. Discord's [official policy](https://discord.com/safety/platform-manipulation-policy-explainer) explicitly prohibits simulated Quest completion.

## E. Scope of this audit and what is still unverified

- Read the actual core source files, task handlers, HTTP/Gateway clients, READMEs, main manifests and configuration at pinned source revisions.
- Did not run any project's script, access a user account, execute a Quest, or benchmark RAM/CPU.
- Did not test all dependency versions, every platform, all binaries, all historical tags, or every third-party service.
- These are **code-level findings**, not promises about Discord production behavior.
- No independent confirmation of live console/streaming/external achievement success.
- Some remaining code paths and transitive dependencies warrant follow-up; do not call this a completed security penetration test.
