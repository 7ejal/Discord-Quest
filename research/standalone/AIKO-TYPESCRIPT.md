# Detailed source walkthrough: aiko-chan-ai (TypeScript/Node)

**Revision:** [main @ 4dd4e6b535cc799708f60cfcbff0ecf33eb504ee](https://github.com/aiko-chan-ai/Discord-Quest-Auto-Completion-Selfbot/tree/4dd4e6b535cc799708f60cfcbff0ecf33eb504ee)  
**Review:** 2026-11-01, static code. No live account or Quest simulation, test-suite execution, runtime benchmark or dependency audit.

## The program and its dependencies

The entry point is `bot.ts`, launched by Node.js through `tsx`. `package.json` declares **Node 24+** and dependencies `@discordjs/core`, `@discordjs/rest`, `@discordjs/ws`, `discord-api-types`, `undici` and `tsx`. Discord desktop is **not installed or needed for the Node process to start**. The package uses `tsx --env-file=.env bot.ts` and reads a personal Discord user token from environment configuration. `package-lock.json` is committed, enabling more deterministic dependency resolution than a manifest alone.

**Be aware:** the GitHub README prominently states that personal-account selfbots and simulated Quests violate Discord rules. Its own current code also states streaming is unsupported.

## File responsibilities

| File | What it does |
|---|---|
| [bot.ts](https://github.com/aiko-chan-ai/Discord-Quest-Auto-Completion-Selfbot/blob/4dd4e6b535cc799708f60cfcbff0ecf33eb504ee/bot.ts) | Program entry, READY callback, list Quests, launch concurrently, disconnect |
| [src/client.ts](https://github.com/aiko-chan-ai/Discord-Quest-Auto-Completion-Selfbot/blob/4dd4e6b535cc799708f60cfcbff0ecf33eb504ee/src/client.ts) | Builds Discord.js REST and Gateway client; handles READY, rate-limit notices, webhook |
| [src/questManager.ts](https://github.com/aiko-chan-ai/Discord-Quest-Auto-Completion-Selfbot/blob/4dd4e6b535cc799708f60cfcbff0ecf33eb504ee/src/questManager.ts) | Enrollment, task switching, video/game/activity/achievement logic, optional reward claim |
| [src/quest.ts](https://github.com/aiko-chan-ai/Discord-Quest-Auto-Completion-Selfbot/blob/4dd4e6b535cc799708f60cfcbff0ecf33eb504ee/src/quest.ts) | Object representing a Quest with getters and status updates |
| [src/interface.ts](https://github.com/aiko-chan-ai/Discord-Quest-Auto-Completion-Selfbot/blob/4dd4e6b535cc799708f60cfcbff0ecf33eb504ee/src/interface.ts) | TypeScript interfaces describing the data |
| [src/constants.ts](https://github.com/aiko-chan-ai/Discord-Quest-Auto-Completion-Selfbot/blob/4dd4e6b535cc799708f60cfcbff0ecf33eb504ee/src/constants.ts) | Client metadata |
| [src/utils.ts](https://github.com/aiko-chan-ai/Discord-Quest-Auto-Completion-Selfbot/blob/4dd4e6b535cc799708f60cfcbff0ecf33eb504ee/src/utils.ts) | Request headers, client version lookup, app-specific provider integration, optional webhook |
| [src/captcha.ts](https://github.com/aiko-chan-ai/Discord-Quest-Auto-Completion-Selfbot/blob/4dd4e6b535cc799708f60cfcbff0ecf33eb504ee/src/captcha.ts), [providers/yescaptcha.ts](https://github.com/aiko-chan-ai/Discord-Quest-Auto-Completion-Selfbot/blob/4dd4e6b535cc799708f60cfcbff0ecf33eb504ee/src/providers/yescaptcha.ts) | Optional third-party challenge service; no guarantee that reward claiming works |

## Exact call flow and why Discord.js is used

**Program starts:** `bot.ts` constructs `ClientQuest` with an environment credential. This inherits `@discordjs/core`'s `Client` but uses it with a **personal user credential** rather than an ordinary bot credential. This is an unsupported application of the library.

**REST setup:** `src/client.ts` creates a `@discordjs/rest` REST client with `version: 10` and a custom request function. `Utils.makeHeaders` adjusts request headers for client-like behavior. The library provides request queue/rate-limit infrastructure; the code attaches a `rateLimited` listener for diagnostics.

**Gateway:** The `@discordjs/ws` `WebSocketManager` handles the persistent connection and emits a READY event. The code overrides `WebSocketShard.prototype.send` to modify the Identify payload before it is sent. This modifies internal library behavior globally—a maintenance risk if dependencies change.

**READY handler:** After connection, `bot.ts` calls `fetchQuests`, obtains `QuestManager`, filters candidates, and runs `Promise.allSettled(questsValid.map(...doingQuest))`. It then disconnects. Crucially, `Promise.allSettled` **allows some tasks to fail** while the program continues; the result array is not inspected to present a truthful per-Quest success summary.

Source: [client.ts L11–96](https://github.com/aiko-chan-ai/Discord-Quest-Auto-Completion-Selfbot/blob/4dd4e6b535cc799708f60cfcbff0ecf33eb504ee/src/client.ts#L11-L96), [bot.ts L21–60](https://github.com/aiko-chan-ai/Discord-Quest-Auto-Completion-Selfbot/blob/4dd4e6b535cc799708f60cfcbff0ecf33eb504ee/bot.ts#L21-L60).

## Task and state logic

**Selection:** `doingQuest` checks enrollment, then scans a fixed priority array and takes the **first task present**. It does not evaluate a compound Quest's `join_operator` or a selected desktop/console alternative. There is a risk of wrong task, unfinished AND groups, and task configuration missing a recognized entry. Source [questManager.ts L270–358](https://github.com/aiko-chan-ai/Discord-Quest-Auto-Completion-Selfbot/blob/4dd4e6b535cc799708f60cfcbff0ecf33eb504ee/src/questManager.ts#L270-L358).

**Video:** A time-based progress reporter checks elapsed time since enrollment, advances cached local progress and submits timestamp-shaped updates. When the target is reached, the code logs `completed!` **even when the final server response has not confirmed `completed_at`**. Thus a success message alone is not evidence. [questManager.ts L361–413](https://github.com/aiko-chan-ai/Discord-Quest-Auto-Completion-Selfbot/blob/4dd4e6b535cc799708f60cfcbff0ecf33eb504ee/src/questManager.ts#L361-L413).

**Desktop/console:** `doingPlayOnPlatformQuest` is a shared heartbeat loop. The fact that the code handles Xbox/PlayStation task names via this function is **not evidence of genuine console-linked play**. It updates the Quest object from successful REST responses and uses the Quest completion flag to exit. There is **no hard deadline when progress stalls**. [questManager.ts L415–457](https://github.com/aiko-chan-ai/Discord-Quest-Auto-Completion-Selfbot/blob/4dd4e6b535cc799708f60cfcbff0ecf33eb504ee/src/questManager.ts#L415-L457).

**Embedded activity:** `doingPlayActivityQuest` uses a **hardcoded placeholder stream key** and a similar response loop; comments acknowledge the real voice/channel/user session key is different. This should be labeled **untested/fragile**, not automatically working. [questManager.ts L458–495](https://github.com/aiko-chan-ai/Discord-Quest-Auto-Completion-Selfbot/blob/4dd4e6b535cc799708f60cfcbff0ecf33eb504ee/src/questManager.ts#L458-L495).

**Stream:** In `doingQuest`, the `STREAM_ON_DESKTOP` branch only logs that the feature no longer works in Node and advises using Discord desktop. That is **explicitly unsupported**, regardless of the task name being present in the switch. [questManager.ts L340–347](https://github.com/aiko-chan-ai/Discord-Quest-Auto-Completion-Selfbot/blob/4dd4e6b535cc799708f60cfcbff0ecf33eb504ee/src/questManager.ts#L340-L347).

**Application achievement:** A separate application-specific authorization/provider path exists. It is not equivalent to a game-publisher achievement. The deauthorization step is reached only after all previous steps succeed; an early return after an authorization grant can leave a permission in place. No enclosing `try/finally` ensures cleanup. [questManager.ts L496–584](https://github.com/aiko-chan-ai/Discord-Quest-Auto-Completion-Selfbot/blob/4dd4e6b535cc799708f60cfcbff0ecf33eb504ee/src/questManager.ts#L496-L584).

**Claims:** `redeemQuest` exists, but the entire calling block in the program's main handler is **commented out**. Its challenge branch contains a TODO describing an unresolved claim error. [bot.ts L40–51](https://github.com/aiko-chan-ai/Discord-Quest-Auto-Completion-Selfbot/blob/4dd4e6b535cc799708f60cfcbff0ecf33eb504ee/bot.ts#L40-L51), [questManager.ts L182–267](https://github.com/aiko-chan-ai/Discord-Quest-Auto-Completion-Selfbot/blob/4dd4e6b535cc799708f60cfcbff0ecf33eb504ee/src/questManager.ts#L182-L267).

## Compare with Go rewrite

Go and aiko share the same conceptual layout (entry/client/quest manager/Quest object), the same fixed-priority selection behavior, similar video and platform handlers, similar specialized activity provider and commented-out automatic reward claiming. The Go rewrite adds a claimed streaming handler and dynamic property discovery; those additions do **not** establish independent production correctness.

### Failure modes

- Top-level `Promise.allSettled` results are not used to reconcile failed tasks.
- Fixed task selection ignores multi-task AND/OR.
- Video logs can be falsely optimistic.
- Activity loops can wait indefinitely; placeholder session identifier.
- Stream explicitly unsupported.
- App permissions can remain after early failure.
- Claims are not invoked in the standard program.
- Dependency versions / private client expectations can break when Discord changes.

### Verdict

**Strength:** Best understandable example of separating an independent TypeScript Gateway/REST client, model, manager and task handlers using recognized libraries.  
**Limit:** It is not a verified all-task completer; its Node streaming path is expressly unsupported and multiple other handlers are untested or vulnerable to wrong completion reporting.  
**Security:** Treat the personal user token as full-account-sensitive; third-party services and app authorization add additional trust boundaries.
