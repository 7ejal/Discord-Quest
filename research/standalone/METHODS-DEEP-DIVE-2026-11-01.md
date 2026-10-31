# Which standalone Quest method is best? — deeper source-level comparison

**Research date:** 2026-11-01  
**Scope:** Five standalone CLI/EXE projects, plus two no-desktop architectural comparators (Node SDK and hosted REST service). The exact HEAD revisions below were rechecked on GitHub on 2026-11-01.  
**Method:** static source review + official product documentation. **No live Discord Quest automation, credential access, task credit, performance benchmark or test-suite run.**

> **Important:** Discord [prohibits scripts and spoofed applications that simulate Quest completion without the required activity](https://discord.com/safety/platform-manipulation-policy-explainer). This document explains what public code does, why certain strategies are fragile, and how to evaluate source-level correctness—not how to evade Discord's engagement checks.

## Quick answer

**No independent code-only method has been demonstrated to complete every task.** The meaningful engineering answer is conditional:

- **Best *delivery format* for a native Windows CLI EXE:** Go, specifically [yeweroooo](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot). A Go build can be distributed as a standalone executable. **This does not make its Quest logic best.**
- **Best existing lightweight CLI for *HTTP retry policy and readable network flow*:** [SUDIN010 Python](https://github.com/SUDIN010/discord-auto-quest). It has capped retries and a 30-second request timeout and understands per-task desktop application identifiers. **New finding: it still has false-success conditions and stale presence across multiple selected Quests.**
- **Best *general architecture* (not currently realized end-to-end by these projects):** typed Quest schema + correct AND/OR evaluation + task-specific authentic activity context + bounded network/task deadlines + **server-authoritative Quest-level completion** + explicit reward state. Prefer separate read-only/offline verification tools over credential-using selfbots.
- **Most advertised code branches:** Go yeweroooo, but several represent deprecated, untested or externally controlled tasks.
- **Worst method to infer success:** a local timestamp, a plain HTTP 2xx, or a hardcoded session key treated as authoritative completion.

All code links below pin specific upstream revisions so they cannot silently change after this report.

## 1. Five standalone architectures (what runs, which process does what)

| Project | Pinned revision | Local runtime | HTTP mechanism | Gateway mechanism | Task scheduler |
|---|---|---|---|---|---|
| [Go yeweroooo](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/tree/352d7f457c9511bc9e7c9b940a71012bfb47572e) | `352d7f4` | Compiled Go EXE | `net/http` + handwritten request wrapper | `gorilla/websocket`, own HELLO/READY/heartbeat handling | Goroutine per Quest; `sync.WaitGroup` |
| [TS aiko](https://github.com/aiko-chan-ai/Discord-Quest-Auto-Completion-Selfbot/tree/4dd4e6b535cc799708f60cfcbff0ecf33eb504ee) | `4dd4e6b` | Node 24+, `tsx` | `@discordjs/rest` + `undici` | `@discordjs/ws` and `@discordjs/core`, modified Identify | `Promise.allSettled` |
| [Python SUDIN](https://github.com/SUDIN010/discord-auto-quest/tree/c14afa1dbb3c8c59d7f68c90caa305eff8e15b0e) | `c14afa1` | Python CLI | Synchronous `requests.Session` called from async flow | `websockets` presence worker | Sequential selected Quests |
| [Python unconsents](https://github.com/unconsents/auto-quest-completer/tree/6d3b9a085dc78ce42d85dd9748f1636eff111a5a) | `6d3b9a0` | Python CLI | `aiohttp` | `discord.py-self` for multiple user-account Gateway sessions | `asyncio.gather` for accounts and tasks |
| [TS lfathh](https://github.com/lfathh/Auto-Quest-Discord/tree/5fe8bd958571fea5e953d23a211d3472cea3ebce) | `5fe8bd9` | Node 18+, `tsx` | Older `@discordjs/rest` | Older `@discordjs/ws`; modified Identify | `Promise.allSettled` + CLI dashboard |

**Go is the only one of these five that naturally produces a self-contained native binary without bundling the language runtime.** A `.bat` wrapper for Node/Python starts the interpreter; it does not improve task coverage.

### A. Pure HTTP vs HTTP + WebSocket

**HTTP (REST)** is request/response: a client asks for Quest state, enrollment status or progress and examines JSON returned by the server. Most task-handling code uses HTTP.  
**Gateway (WebSocket)** is a persistent session for READY, presence and connection health; it has its own keep-alive heartbeat, unrelated to task-credit heartbeats.  
**Crucial:** No Gateway client library intrinsically creates real game processes, a stream viewer or linked console gameplay.

| Design option | Benefit | Trade-off | Relevant code |
|---|---|---|---|
| **Handwritten Go HTTP + Gateway** | Minimal direct dependencies, small portable binary, easy profiling | Must correctly implement timeouts, retries, reconnect and client state; Go version fails some | [Go client.go](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/client.go) |
| **Discord.js REST + Gateway** | Reuses mature transport scheduling and WebSocket lifecycle | These libraries target supported bot/application workflows; personal-account use and custom Identify patching are unsupported and version-fragile | [aiko client.ts L11–96](https://github.com/aiko-chan-ai/Discord-Quest-Auto-Completion-Selfbot/blob/4dd4e6b535cc799708f60cfcbff0ecf33eb504ee/src/client.ts#L11-L96) |
| **Python requests + independent presence worker** | Straightforward call graph, bounded HTTP attempts, visible logs | Mixed sync/async complexity; one initial presence does not necessarily match later selected Quests | [SUDIN main.py L250–315, L436–510](https://github.com/SUDIN010/discord-auto-quest/blob/c14afa1dbb3c8c59d7f68c90caa305eff8e15b0e/main.py#L436-L510) |
| **Python discord.py-self + aiohttp** | Concurrent sessions and async network I/O | More user-token exposure and concurrency; generalized task loops hide type-specific preconditions | [unconsents main.py](https://github.com/unconsents/auto-quest-completer/blob/6d3b9a085dc78ce42d85dd9748f1636eff111a5a/main.py) |
| **REST-only Node SDK** (not one of five CLIs) | No Gateway connection; reusable solver interface | Cannot infer actual voice/game context; one solver handles both gameplay and streaming in xgg-2 | [xgg-2 DiscordQuests.ts L175–186](https://github.com/xgg-2/discord-quests/blob/6d166090474cd831d7d7bf39104b2881406407e7/src/DiscordQuests.ts#L175-L186) |

**Verdict:** REST-only is simpler for *listing and inspecting state*, not necessarily for fulfilling a task. Gateway + REST is more expressive for client-state observations but still is **not a universal completion mechanism**. In either design, server-confirmed Quest state is more important than request transport.

## 2. Method-by-method task comparison

### Method 1 — Video timestamp/progress reporting

**What the code actually does:** Implements a local time-based counter, reads an existing progress value (if available), submits a video-related progress request, and uses the response to infer a result. Video-watching Quests officially require watching the video; **an HTTP request does not prove that happened**.

| Project | Implementation choice | Source-backed failure |
|---|---|---|
| Go | Handler uses enrollment-relative elapsed time, advances local `secondsDone` | Finishes with “Quest completed!” even if no server `completed_at` was confirmed; increments local value even after failed request ([manager.go L173–225](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/manager.go#L173-L225)) |
| aiko | Similar elapsed-time implementation | Same unconditional success-report flaw ([questManager.ts L361–413](https://github.com/aiko-chan-ai/Discord-Quest-Auto-Completion-Selfbot/blob/4dd4e6b535cc799708f60cfcbff0ecf33eb504ee/src/questManager.ts#L361-L413)) |
| SUDIN | Checks returned `completed_at` after sending progress | **Important qualification:** it initializes `finished` from progress already at target, even without `completed_at`; it is more conservative than Go's unconditional log but is **not strict proof** ([main.py L361–399](https://github.com/SUDIN010/discord-auto-quest/blob/c14afa1dbb3c8c59d7f68c90caa305eff8e15b0e/main.py#L361-L399)) |
| unconsents | Local count with HTTP responses; final call then success log | Logs video complete even if server never confirmed it ([main.py L169–194](https://github.com/unconsents/auto-quest-completer/blob/6d3b9a085dc78ce42d85dd9748f1636eff111a5a/main.py#L169-L194)) |
| lfathh | Similar client timestamp routine | Logs complete without required final proof ([questManager.ts L139–188](https://github.com/lfathh/Auto-Quest-Discord/blob/5fe8bd958571fea5e953d23a211d3472cea3ebce/src/questManager.ts#L139-L188)) |

**Best *verification* approach:** Trust real Quest-level completion returned by the service after authentic participation. A locally computed video percentage is a **display estimate**, not a completion verdict. None of the five guarantees strict final server re-fetch for all cases.

### Method 2 — Desktop-game presence vs task heartbeats

These are **two different concepts**:

- **Presence/Gateway:** a client announces an activity name/application over an active WebSocket.
- **Quest task progress:** a separate task-specific progress check/action. A presence value does not make a game executable exist or prove game playtime.

**Go:** Shares one platform heartbeat handler between desktop, Xbox, PlayStation and historical V2; it uses top-level `config.application.id`. It does *not* establish platform-specific console sessions. [manager.go L113–145, L228–265](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/manager.go#L228-L265).

**aiko:** Uses the same general desktop/console handler pattern, including `config.application.id`, not the task-specific `applications` array. [questManager.ts L290–338, L415–457](https://github.com/aiko-chan-ai/Discord-Quest-Auto-Completion-Selfbot/blob/4dd4e6b535cc799708f60cfcbff0ecf33eb504ee/src/questManager.ts#L415-L457).

**SUDIN:** Reads a **per-task application ID first**, falling back to the top-level application. This is an architectural improvement for modern `task_config_v2`. It also opens an explicit Gateway presence worker. **New issue:** `run()` builds `activity` from **`matches[0]` just once** and then iterates over all pending Quests; that Gateway presence is never changed to represent a later-selected game's application. This can make user presence inconsistent with the task being processed. [main.py L145–158](https://github.com/SUDIN010/discord-auto-quest/blob/c14afa1dbb3c8c59d7f68c90caa305eff8e15b0e/main.py#L145-L158), [L474–505](https://github.com/SUDIN010/discord-auto-quest/blob/c14afa1dbb3c8c59d7f68c90caa305eff8e15b0e/main.py#L474-L505).

**unconsents:** Uses one broad generic heartbeat loop for desktop, stream and activity tasks, not a real game-process detector. The Gateway session created by discord.py-self doesn't change that. [main.py L98–102, L195–211](https://github.com/unconsents/auto-quest-completer/blob/6d3b9a085dc78ce42d85dd9748f1636eff111a5a/main.py#L195-L211).

**lfathh:** Has an older desktop HTTP loop; its config assumption may not match current Quest definitions. [questManager.ts L125–135, L189–223](https://github.com/lfathh/Auto-Quest-Discord/blob/5fe8bd958571fea5e953d23a211d3472cea3ebce/src/questManager.ts#L189-L223).

**Best architecture:** Correctly parse *the task-specific application* and observe authentic game state separately from HTTP status. **No reviewed standalone demonstrates a general, authentic substitute for the Discord desktop game-detection requirement.**

### Method 3 — Streaming

A real desktop stream Quest is **not** equivalent to “some stream-shaped JSON exists.” The official [FAQ](https://support.discord.com/hc/en-us/articles/22225719947543-Discord-Quests-FAQ) requires streaming to at least one other person in voice. [Orion's code investigation](https://github.com/nyxxbit/discord-quest-completer/blob/7b21ac365dd38686d25933dc84d46e2bdb2c2b2a/docs/ARCHITECTURE.md#L105-L139) reports that faking stream application metadata without a **real active Go Live and viewer** did not begin valid stream heartbeats.

- **Go:** Has a distinct “stream” branch, but presence/event handling and a generic HTTP loop are not proof that a legitimate stream exists. [manager.go L394–450](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/manager.go#L394-L450).
- **aiko:** The Node branch explicitly says streaming is unsupported. [questManager.ts L340–347](https://github.com/aiko-chan-ai/Discord-Quest-Auto-Completion-Selfbot/blob/4dd4e6b535cc799708f60cfcbff0ecf33eb504ee/src/questManager.ts#L340-L347).
- **SUDIN:** No stream handler.
- **unconsents:** Reuses generic heartbeat; no independent proof of stream-session validity.
- **lfathh:** The standalone TypeScript branch simply displays unsupported (even though a separate console script has stream-related logic). [questManager.ts L223–241](https://github.com/lfathh/Auto-Quest-Discord/blob/5fe8bd958571fea5e953d23a211d3472cea3ebce/src/questManager.ts#L223-L241).
- **xgg-2 SDK:** Dispatches streaming and desktop gameplay to the same `PlayOnDesktopSolver`; not a separate real streaming implementation. [DiscordQuests.ts L175–187](https://github.com/xgg-2/discord-quests/blob/6d166090474cd831d7d7bf39104b2881406407e7/src/DiscordQuests.ts#L175-L187).

**Best evidenced fulfillment method:** actual eligible gameplay streaming with a real viewer; a headless CLI method is **not independently verified**.

### Method 4 — Embedded Activity session

An embedded Discord Activity can have a **real application instance and voice context**. Several selfbots collapse it into the same heartbeat code as desktop gameplay.

- **Go:** fixed placeholder `call:1:1`, not a dynamically observed real session ([manager.go L268–309](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/manager.go#L268-L309)).
- **aiko:** same placeholder noted as TODO in source ([questManager.ts L458–495](https://github.com/aiko-chan-ai/Discord-Quest-Auto-Completion-Selfbot/blob/4dd4e6b535cc799708f60cfcbff0ecf33eb504ee/src/questManager.ts#L458-L495)).
- **unconsents:** synthetic session marker in generic loop ([main.py L195–211](https://github.com/unconsents/auto-quest-completer/blob/6d3b9a085dc78ce42d85dd9748f1636eff111a5a/main.py#L195-L211)).
- **SUDIN and lfathh standalone:** no implemented Activity completion handler.
- **xgg-2:** `PlayActivitySolver` has a separate file but also constructs a synthetic session key ([PlayActivity.ts](https://github.com/xgg-2/discord-quests/blob/6d166090474cd831d7d7bf39104b2881406407e7/src/solvers/PlayActivity.ts)).

**Best architectural principle:** The task's authoritative progress should come from actual application/session participation; neither a placeholder session identifier nor a registered handler proves that the embedded Activity ran.

### Method 5 — Publisher/game achievements and consoles

The [unofficial Quest schema](https://docs.discord.food/resources/quests) says `ACHIEVEMENT_IN_GAME` and `ACHIEVEMENT_IN_ACTIVITY` are application-tracked. **`ACHIEVEMENT_IN_GAME` is not a timed desktop progress task.**

- **Go and aiko:** have an **application-specific authorization/provider flow for Activity achievements**, involving additional permissions. That does not generalize to all game/publisher achievements. Their cleanup can be skipped on early failure. [Go manager.go L310–392](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/manager.go#L310-L392), [aiko questManager.ts L496–585](https://github.com/aiko-chan-ai/Discord-Quest-Auto-Completion-Selfbot/blob/4dd4e6b535cc799708f60cfcbff0ecf33eb504ee/src/questManager.ts#L496-L585).
- **Go and aiko:** Xbox/PlayStation task names map to the generic desktop-style handler. In their documentation, console support is **untested**. There is no proof of actual linked-console gameplay.
- **Other three CLIs:** no credible general publisher-achievement or console-game replacement path.

**Best evidenced method:** actual publisher/game-controlled achievement reporting and real account-linked console play. The strongest client-side code cannot override that source of truth.

## 3. Best network and reliability strategy

| Criterion | Go yew | TS aiko | Python SUDIN | Python unconsents | TS lfathh |
|---|---|---|---|---|---|
| **Per-HTTP-call timeout** | Uses Go `http.DefaultClient` without explicit global deadline | Depends on discord.js request layer | **30 s** explicit | No explicit customized timeout in helper (aiohttp defaults still apply) | Depends on older discord.js library |
| **HTTP 429 handling** | Recursive retry, **no attempt cap** | Discord.js REST scheduler and logging | **Maximum 5 attempts**, retries use response delay | Generic HTTP error, no specialized bounded policy | Discord.js REST scheduler |
| **Whole task can stall** | Yes | Yes | Yes | Yes | Yes |
| **Concurrent Quest work** | Goroutines, no bounded worker pool | Concurrent `Promise.allSettled` | Sequential | `asyncio.gather` accounts/tasks | Concurrent `Promise.allSettled` |
| **Task application config** | Top-level only in reviewed handler | Top-level only in reviewed handler | **Task-specific preferred** | General first-matching task | Older task config |
| **General AND/OR evaluator** | No | No | No | No | No |
| **Server Quest re-fetch before claiming success** | Not consistently | Not consistently | Not consistently | Not consistently | Not consistently |
| **Safe OAuth cleanup guarantee** | Not demonstrated | Not demonstrated | Not applicable | Not applicable | Not applicable |
| **Default automatic reward claim** | **No, block commented out** | **No, block commented out** | No | No | No |

**Why this matters:** Bounded retries protect resource usage, not Quest validity. A tool can have excellent error handling and **still announce a Quest completed incorrectly**. Correct data models and explicit server status are at least as important as HTTP performance.

### Particularly important newly verified issues

**SUDIN gameplay completion may be overstated.** It sets `finished = true` if task progress `>= target` **or** `status.completed_at` exists, then logs “Quest completed” after its final call if **either** `finished` or the final response has `completed_at`. This conflates numeric task progress with top-level Quest completion. [main.py L326–359](https://github.com/SUDIN010/discord-auto-quest/blob/c14afa1dbb3c8c59d7f68c90caa305eff8e15b0e/main.py#L326-L359). This qualifies older documentation that described SUDIN as strictly completion-verified.

**SUDIN video completion has a similar edge case.** It initializes `finished` using locally available progress `>= target`; if so, it may log success without fetching a fresh Quest-level completion state. [main.py L361–399](https://github.com/SUDIN010/discord-auto-quest/blob/c14afa1dbb3c8c59d7f68c90caa305eff8e15b0e/main.py#L361-L399).

**SUDIN presence stays tied to the first selected campaign.** The single `activity` is calculated from `matches[0]`, then all pending Quests are run sequentially. [main.py L473–507](https://github.com/SUDIN010/discord-auto-quest/blob/c14afa1dbb3c8c59d7f68c90caa305eff8e15b0e/main.py#L473-L507).

**xgg-2 solver false success is conditional on error handling.** `BaseSolver.start()` emits error without rethrowing. If an error listener is attached, `solve()` can continue to return `completed`. Without an error listener, Node's `EventEmitter` can instead throw from the unhandled `error` event; therefore the precise bug is **dependent on listener wiring**. [BaseSolver.ts L33–47](https://github.com/xgg-2/discord-quests/blob/6d166090474cd831d7d7bf39104b2881406407e7/src/solvers/BaseSolver.ts#L33-L47), [DiscordQuests.ts L191–221](https://github.com/xgg-2/discord-quests/blob/6d166090474cd831d7d7bf39104b2881406407e7/src/DiscordQuests.ts#L191-L221).

**Hosted REST dashboard has two distinct failures:** SubhuJaat07 stores a raw personal user token in an HttpOnly cookie and generates its own completed timestamp after a best-effort terminal request, without verifying Quest-level completion. Its separate encrypted monitor storage does not make the cookie value encrypted. [session.ts L133–163](https://github.com/SubhuJaat07/discord-quest-completor/blob/e2097af7c0ea23e3a6b85c979eb3438c9570f1eb/src/lib/session.ts#L133-L163), [quest-engine.ts L662–695](https://github.com/SubhuJaat07/discord-quest-completor/blob/e2097af7c0ea23e3a6b85c979eb3438c9570f1eb/src/lib/quest-engine.ts#L662-L695).

## 4. Decision: what's the best *method* rather than repository?

### A. Best for independent, local executable packaging
**Go** has the simplest runtime packaging path: compiled binary, two direct external modules in `go.mod` (`gorilla/websocket` and `godotenv`), and native `net/http`. Its actual network error semantics need substantial refactoring before it could be a reliable *status observer*. Native code is not automatically faster for network-bound Quests, and no RAM/CPU benchmark has been run.

### B. Best for readable and robust local HTTP client
**SUDIN's `QuestClient.request`** is a stronger small example than the Go handwritten HTTP wrapper for timeouts/limited retries. But **its task validation is NOT the best**, due to numeric-progress false positives and stale per-Quest presence. Compare [SUDIN request](https://github.com/SUDIN010/discord-auto-quest/blob/c14afa1dbb3c8c59d7f68c90caa305eff8e15b0e/main.py#L76-L116) with [Go request](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/client.go#L231-L303).

### C. Best for understanding a reusable client abstraction
**aiko's Node/TypeScript client** cleanly exposes `ClientQuest`, `QuestManager`, and object models; Discord.js supplies transport machinery. Its unverified task assumptions mean a clean abstraction cannot be mistaken for successful Discord task fulfillment.

### D. Best for accurate status and multi-task logic
**None of these five is best.** As a source-code reference, [saintordevil's independent `resolveTask` logic](https://github.com/saintordevil/questCompleter/blob/3e032d4e4110cc7619ace4858451c374e3570e85/logic.ts#L130-L174) is stronger for AND/OR and unsupported tasks, while [Orion's watchdog](https://github.com/nyxxbit/discord-quest-completer/blob/7b21ac365dd38686d25933dc84d46e2bdb2c2b2a/heartbeatWatchdog.ts) provides a model for stalled-progress handling. These are **plugins/userscripts, not CLI choices**; the lesson is how to build a trustworthy *offline parser and diagnostic state machine*, not to ship their spoofing paths.

### E. Best method for real Quest fulfillment
The **official task route**: watch the video, play the game, stream with a real viewer, use the actual embedded Activity, or let the connected publisher/console report the action. A no-Discord-install CLI may be suitable for examining public code or offline fixtures; **there is no independently established code-only universal substitute for all these actions**.

## 5. Evaluating claimed compatibility correctly

A defensible evaluation has **four separate yes/no gates** for every task:

1. **Source implementation:** Is there a handler, with actual logic rather than a message saying unsupported?
2. **Precondition:** Can the real required activity/context exist in the chosen environment? E.g., a genuine viewer/stream, console game, or publisher-reported achievement.
3. **Server evidence:** Is there a verified Quest-level completed state after the required activity? Local percentage is insufficient.
4. **Reward evidence:** Is it claimable/claimed/redeemed? These are separate checks.

**Never collapse all four into “Working”.** In particular, “Working (untested)” must be reported as **untested**.

### Offline verification plan (without real Discord tokens)

Use synthetic config fixtures and mock transport, rather than real-Quest engagement simulation:

- `AND` two tasks where only one reached target: must stay incomplete.
- `OR` one legitimate complete alternative: can become satisfied.
- Progress = target but Quest `completed_at = null`: state must remain **not server-confirmed**.
- Response 2xx with missing progress: status **unknown**, not success.
- HTTP 429 repeated: finite retries, then rate-limited/paused.
- Unchanged progress: bounded stalled state and cancellation.
- Unknown task ID, old config, absent application ID: **unsupported** with explanation.
- Multiple selected apps: a reporter must not reuse one app's display/context for unrelated tasks.
- Claim challenge or refusal: objective completion does **not** imply reward claimed.
- Error in an event-emitter solver with and without subscribed listeners: status must never turn to completed without server evidence.
- Secret handling: no credential prefix or raw token in logs, cookies or test fixtures.

See [offline verification cases](METHOD-VALIDATION-FIXTURES.md) for expanded acceptance criteria.

## 6. Limitations of this research

- **Source-derived facts:** exact pinned files, architecture, branch/switch logic, retry and status conditions.
- **Not established:** which methods currently earn actual server credit under illegitimate conditions, total real-world success rate, malicious intent, RAM/CPU, server-side anti-abuse behavior, all historical Discord versions, and completeness of every external dependency.
- **No tests run:** a source code test file is not a passing test result. A mock test is not independent production validation.
- The undocumented Quest protocol can change; Discord's [public FAQ](https://support.discord.com/hc/en-us/articles/22225719947543-Discord-Quests-FAQ) and [platform rules](https://discord.com/safety/platform-manipulation-policy-explainer) have higher authority for user-facing requirements.

**Bottom line:** For packaging choose **Go**; for clear bounded HTTP code choose **SUDIN**; for understanding Node transport architecture choose **aiko**; for correct Quest-state methods study **typed AND/OR parsing and server confirmation**, which none of the five standalone programs consistently achieves. No verified universal Quest-completion method exists.
