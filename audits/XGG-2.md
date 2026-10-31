# Source audit: xgg-2 discord-quests (Node/TypeScript library)

**Upstream:** https://github.com/xgg-2/discord-quests  
**Pinned revision:** [6d166090474cd831d7d7bf39104b2881406407e7](https://github.com/xgg-2/discord-quests/tree/6d166090474cd831d7d7bf39104b2881406407e7)  
**Reviewed:** 2026-11-01, static code only.

## Architecture

A reusable `DiscordQuests` class with token validation, configuration parsing, enrollment, solver registry, event emitters, optional proxies, and an Axios request wrapper. Five named task identifiers map to **only three built-in solver implementations**: `WatchVideoSolver`, `PlayOnDesktopSolver`, and `PlayActivitySolver`. `src/utils/axios.ts` has an HTTP timeout and caps automatic retries at five.

## Source-confirmed issues

| Severity | Finding | Evidence |
|---|---|---|
| **Critical for result correctness** | `BaseSolver.start()` **catches** errors and emits an event but does not rethrow; `DiscordQuests.solve()` then returns `status: "completed"` when `start()` resolves, even though the task could have errored. This is a realistic false-success path. | [BaseSolver.ts L33–47](https://github.com/xgg-2/discord-quests/blob/6d166090474cd831d7d7bf39104b2881406407e7/src/solvers/BaseSolver.ts#L33-L47), [DiscordQuests.ts L191–221](https://github.com/xgg-2/discord-quests/blob/6d166090474cd831d7d7bf39104b2881406407e7/src/DiscordQuests.ts#L191-L221) |
| **High** | `STREAM_ON_DESKTOP` dispatches to the **same** `PlayOnDesktopSolver` as gameplay, without establishing an authentic active streaming session. | [DiscordQuests.ts L177–186](https://github.com/xgg-2/discord-quests/blob/6d166090474cd831d7d7bf39104b2881406407e7/src/DiscordQuests.ts#L177-L186) |
| **High** | `isSolvable` only tests whether **any** task has a registered solver; `pickBestTask` picks one task by highest percent. It does not enforce AND/OR semantics for a composite Quest. | [DiscordQuests.ts L266–277](https://github.com/xgg-2/discord-quests/blob/6d166090474cd831d7d7bf39104b2881406407e7/src/DiscordQuests.ts#L266-L277) |
| **Medium** | A parsed task can be treated completed by percent threshold without a task `completed_at` timestamp. `solve()` also does not refetch the Quest after solving to confirm top-level completion. | [DiscordQuests.ts L278–318](https://github.com/xgg-2/discord-quests/blob/6d166090474cd831d7d7bf39104b2881406407e7/src/DiscordQuests.ts#L278-L318) |
| **Medium** | Game/activity solvers use constructed session identifiers; no guarantee of a real desktop or Discord activity state. A code path's existence is not successful server credit. | [PlayOnDesktop.ts](https://github.com/xgg-2/discord-quests/blob/6d166090474cd831d7d7bf39104b2881406407e7/src/solvers/PlayOnDesktop.ts), [PlayActivity.ts](https://github.com/xgg-2/discord-quests/blob/6d166090474cd831d7d7bf39104b2881406407e7/src/solvers/PlayActivity.ts) |
| **Positive** | Axios wrapper provides request timeout, limited retries, rate-limit bucket handling and proxy error handling. Solver abstraction is understandable and extensible. | [axios.ts](https://github.com/xgg-2/discord-quests/blob/6d166090474cd831d7d7bf39104b2881406407e7/src/utils/axios.ts) |

## Coverage and verdict

Built-in handlers: `WATCH_VIDEO`, `WATCH_VIDEO_ON_MOBILE`, `PLAY_ON_DESKTOP`, `STREAM_ON_DESKTOP` and `PLAY_ACTIVITY`. No built-in console/achievement solvers. The stream claim is not substantiated by dedicated streaming logic. **Not recommended as a baseline for reliable result reporting until false-completion semantics are corrected and independently verified.** No live test performed.
