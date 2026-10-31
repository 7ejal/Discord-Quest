# Two additional no-Discord-desktop projects: Node library and hosted dashboard

These are included for completeness because they can run without installing Discord, but they **do not match a simple local CLI/executable in the same sense as the five core projects**.

## xgg-2/discord-quests — reusable Node.js library

**Upstream revision:** [6d166090474cd831d7d7bf39104b2881406407e7](https://github.com/xgg-2/discord-quests/tree/6d166090474cd831d7d7bf39104b2881406407e7).  
**Main code:** [src/DiscordQuests.ts](https://github.com/xgg-2/discord-quests/blob/6d166090474cd831d7d7bf39104b2881406407e7/src/DiscordQuests.ts), [solvers](https://github.com/xgg-2/discord-quests/tree/6d166090474cd831d7d7bf39104b2881406407e7/src/solvers), [Axios wrapper](https://github.com/xgg-2/discord-quests/blob/6d166090474cd831d7d7bf39104b2881406407e7/src/utils/axios.ts).

### What it is

Unlike a ready-made executable or interactive Python CLI, this is a TypeScript library a developer imports into another Node program. `DiscordQuests` wraps token format checking, Quest listing and enrollment, selection, solver construction and asynchronous result reporting. It uses **Axios** rather than raw fetch and has a configurable proxy layer, a 30-second request timeout and a maximum number of retry attempts.

### Classes explained

- `DiscordQuests`: top-level API facade, holds a user token and solver registry.
- `BaseSolver`: shared event emitter and progress interface.
- `WatchVideoSolver`: video task handler.
- `PlayOnDesktopSolver`: gameplay-style handler; **also used for stream tasks**, with no separate real-stream session implementation.
- `PlayActivitySolver`: activity-style handler.
- `createAxiosInstance`: common request policy, rate-limit buckets and retries.

### Two important bugs/limitations

**Critical source-code error propagation bug:** `BaseSolver.start()` catches an error and emits an `error` event but does not rethrow or return a failure status. `DiscordQuests.solve()` awaits `start()` and then returns a `completed` result. Therefore a failed solver can yield an apparent top-level success (or an EventEmitter unhandled error if no listener). Do not use its completion result as server-confirmed evidence. [BaseSolver.ts L33–47](https://github.com/xgg-2/discord-quests/blob/6d166090474cd831d7d7bf39104b2881406407e7/src/solvers/BaseSolver.ts#L33-L47), [DiscordQuests.ts L191–221](https://github.com/xgg-2/discord-quests/blob/6d166090474cd831d7d7bf39104b2881406407e7/src/DiscordQuests.ts#L191-L221).

**Coverage limitation:** The library exposes five task names but only **three built-in solvers**. Both `STREAM_ON_DESKTOP` and `PLAY_ON_DESKTOP` dispatch to the same solver. `isSolvable` returns true if **any** task is handled, and `pickBestTask` picks the highest percent rather than resolving an AND/OR group. [DiscordQuests.ts L175–187](https://github.com/xgg-2/discord-quests/blob/6d166090474cd831d7d7bf39104b2881406407e7/src/DiscordQuests.ts#L175-L187), [L266–291](https://github.com/xgg-2/discord-quests/blob/6d166090474cd831d7d7bf39104b2881406407e7/src/DiscordQuests.ts#L266-L291).

**Assessment:** Interesting modular SDK design with useful HTTP-policy code, but its correctness contract for “completed” is currently weaker than advertised. Standalone runnable script requires a separate embedding application.

## SubhuJaat07/discord-quest-completor — hosted Next.js dashboard

**Upstream revision:** [e2097af7c0ea23e3a6b85c979eb3438c9570f1eb](https://github.com/SubhuJaat07/discord-quest-completor/tree/e2097af7c0ea23e3a6b85c979eb3438c9570f1eb).

### What it is

A web server with API routes, browser frontend, Quest/session manager, queued jobs, optional monitoring and persistent state. It does **not** need Discord installed on the server, but **is not a simple .exe/.bat local CLI**. It has more deployment and credential-custody complexity than the five core projects.

### Main files

- [src/lib/quest-engine.ts](https://github.com/SubhuJaat07/discord-quest-completor/blob/e2097af7c0ea23e3a6b85c979eb3438c9570f1eb/src/lib/quest-engine.ts): Quest state, video/heartbeat work loops, queue and claim attempts.
- [src/lib/session.ts](https://github.com/SubhuJaat07/discord-quest-completor/blob/e2097af7c0ea23e3a6b85c979eb3438c9570f1eb/src/lib/session.ts): login cookie and process memory session handling.
- [src/lib/monitor.ts](https://github.com/SubhuJaat07/discord-quest-completor/blob/e2097af7c0ea23e3a6b85c979eb3438c9570f1eb/src/lib/monitor.ts): scheduled checks.
- [src/lib/state-store.ts](https://github.com/SubhuJaat07/discord-quest-completor/blob/e2097af7c0ea23e3a6b85c979eb3438c9570f1eb/src/lib/state-store.ts): encrypted monitor persistence to disk and optionally Supabase.
- [scripts](https://github.com/SubhuJaat07/discord-quest-completor/tree/e2097af7c0ea23e3a6b85c979eb3438c9570f1eb/scripts): mock Discord/API tests and integration test scripts.

### Main issues

**Credential safety:** `session.ts` sets a browser cookie whose **value is the plaintext personal Discord token**. The cookie is HttpOnly, but HttpOnly means JavaScript cannot ordinarily read it; it is **not encryption or token minimization**. The encrypted AES-GCM storage in `state-store.ts` protects *another persistence surface*, not this raw login cookie. [session.ts L133–162](https://github.com/SubhuJaat07/discord-quest-completor/blob/e2097af7c0ea23e3a6b85c979eb3438c9570f1eb/src/lib/session.ts#L133-L162), [state-store.ts L64–94](https://github.com/SubhuJaat07/discord-quest-completor/blob/e2097af7c0ea23e3a6b85c979eb3438c9570f1eb/src/lib/state-store.ts#L64-L94).

**False local completion:** `finishQuest` sends a best-effort terminal message, ignores its error, then sets the local state to `completed` and creates a new local `completedAt` timestamp. There is **no final Quest-status re-fetch and validation** in this function. A dashboard showing “completed” is not proof Discord credited it. [quest-engine.ts L662–693](https://github.com/SubhuJaat07/discord-quest-completor/blob/e2097af7c0ea23e3a6b85c979eb3438c9570f1eb/src/lib/quest-engine.ts#L662-L693).

**Coverage:** Five task names collapse to just two main handler families (video vs heartbeat); real streaming and genuine embedded activity conditions are not demonstrated merely by a session key. There are HTTP error thresholds, but these guard program flow—not authenticity of the Quest. [quest-engine.ts](https://github.com/SubhuJaat07/discord-quest-completor/blob/e2097af7c0ea23e3a6b85c979eb3438c9570f1eb/src/lib/quest-engine.ts).

**Assessment:** Valuable source for persistent queues and a dashboard; much higher user-credential exposure and potentially misleading local success. Not an advisable credential custodian for a personal account, and its mock tests do not verify production outcomes.

## Bottom line for the user's format constraint

If the requirement is strictly **standalone .exe, terminal CLI or .bat that launches an executable**, prioritize the [five core walkthroughs](README.md) instead of treating a Node SDK or Next.js web service as equivalent.
