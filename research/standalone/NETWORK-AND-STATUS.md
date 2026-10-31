# Networking, Quest models, and source-code terminology — beginner-friendly

**Why this document exists:** All five reviewed programs talk to the Discord *service* from their own process. Understanding the difference between a client connection, Quest progress, and final reward entitlement is essential to reading their source.

## 1. A process without the Discord app

An `.exe`, Python process or Node program can use ordinary sockets to contact an online service. “Discord isn't installed” describes the **operating-system environment**, not a promise that the process performs the activities required by Discord's advertising program.

- Go [client.go](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/client.go): a WebSocket client and HTTP functions.
- TypeScript [aiko client.ts](https://github.com/aiko-chan-ai/Discord-Quest-Auto-Completion-Selfbot/blob/4dd4e6b535cc799708f60cfcbff0ecf33eb504ee/src/client.ts): wraps Discord.js REST and Gateway libraries.
- Python [SUDIN main.py](https://github.com/SUDIN010/discord-auto-quest/blob/c14afa1dbb3c8c59d7f68c90caa305eff8e15b0e/main.py): Requests plus a separate asynchronous WebSocket.
- Python [unconsents main.py](https://github.com/unconsents/auto-quest-completer/blob/6d3b9a085dc78ce42d85dd9748f1636eff111a5a/main.py): discord.py-self for session presence plus aiohttp for HTTP calls.
- TypeScript [lfathh client.ts](https://github.com/lfathh/Auto-Quest-Discord/blob/5fe8bd958571fea5e953d23a211d3472cea3ebce/src/client.ts): older Discord.js client structure.

## 2. REST vs Gateway vs Quest task heartbeat

| Concept | In plain language | Examples in source | What it **doesn't** establish |
|---|---|---|---|
| REST | Ask a server to return/change data | Quest listing, enrollment, progress response, claim attempt | Genuine game/video/streaming conditions |
| Gateway | Long-lived bidirectional session | READY event, client Identify and Gateway heartbeat | Quest task completion |
| Gateway heartbeat | Keep the connection/session alive | Go `sendHeartbeat`, SUDIN `presence_worker.heartbeat` | A Quest's game-playtime credit |
| Quest task heartbeat | Report / check progression for some task families | `doingPlayOnPlatformQuest`, `run_heartbeat` | That required activity was genuinely performed or server credited the objective |
| Video progress | Report video-related state | `doingWatchVideoQuest`, `complete_watch_video` | Actual watched time merely from a locally submitted counter |
| Publisher achievement | External game/activity reports a qualifying achievement | Task identifier and provider-side event | Can be replaced with generic Discord client presence |
| Reward claim | Awarding a benefit when eligibility is satisfied | `RedeemQuest`, `redeemQuest` | That a code was redeemed at the partner site |

**Note:** The Quest-related REST operations described by the projects are **undocumented user endpoints**, not supported public bot API methods. No detailed request replay or authentication instructions are provided here.

## 3. A synthetic Quest object

The following is deliberately invented and suitable for offline test fixtures. It does not include real accounts, credentials, campaign identifiers or request recipes.

~~~json
{
  "config": {
    "config_version": 2,
    "expires_at": "2027-01-01T00:00:00Z",
    "task_config_v2": {
      "join_operator": "and",
      "tasks": {
        "WATCH_VIDEO": { "type": "WATCH_VIDEO", "target": 60 },
        "PLAY_ACTIVITY": { "type": "PLAY_ACTIVITY", "target": 300 }
      }
    }
  },
  "user_status": {
    "enrolled_at": "2026-11-01T12:00:00Z",
    "completed_at": null,
    "claimed_at": null,
    "progress": {
      "WATCH_VIDEO": { "value": 60, "completed_at": "2026-11-01T12:02:00Z" },
      "PLAY_ACTIVITY": { "value": 0, "completed_at": null }
    }
  }
}
~~~

In this **AND** example the video task is finished but the **Quest is not**. A tool that picks only `WATCH_VIDEO`, prints “done” and exits gives a misleading result. For **OR**, one valid completed alternative can be enough. See [Unofficial Quest structures](https://docs.discord.food/resources/quests).

## 4. State-machine vocabulary

~~~text
DISCOVERED -> ELIGIBLE -> ENROLLED -> TASK_PROGRESS
   -> TASK_COMPLETED -> ALL_REQUIRED_TASKS_COMPLETED
   -> QUEST_COMPLETED -> REWARD_CLAIMED -> REWARD_REDEEMED
~~~

- A Quest can be **shown** but not accepted.
- An HTTP request can return success without server-side **Quest-level completion**.
- A task may meet a numeric target while the server has not confirmed `completed_at`.
- A Quest can be **completed** but the reward claim still requires a manual verification step.
- A code can be **claimed** but not yet **redeemed** with the partner.

### Credibility ladder

1. **Local console says done:** almost no proof.
2. **Local counter >= target:** no proof of server acceptance.
3. **HTTP returned 2xx:** server received/accepted a request, not necessarily credited the required engagement.
4. **Returned task progress reached target:** stronger, but may not satisfy AND group or publisher-backed conditions.
5. **Server Quest `completed_at`:** strong account-status evidence.
6. **Server reward `claimed_at`:** separate reward event.
7. **Partner confirms redemption:** separate partner-side event.

## 5. Why a fixed task list can be wrong

Example pattern observed in Go and TypeScript:

~~~text
for each known_task in priority_order:
  if task appears:
    choose task
    stop searching
~~~

It checks **whether a task type is present**, not whether it is genuinely feasible, platform-eligible, complete or part of an AND group. A robust *offline parser* should instead compute the group requirements, selected platform, unknown tasks, and server-confirmed task status. [Go quest.go](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/quest.go#L32-L52), [aiko questManager.ts](https://github.com/aiko-chan-ai/Discord-Quest-Auto-Completion-Selfbot/blob/4dd4e6b535cc799708f60cfcbff0ecf33eb504ee/src/questManager.ts#L290-L307).

## 6. Timeouts, retries, and connection correctness

**Timeout:** Prevents a program from waiting forever for a response or server-confirmed change. Go uses default HTTP clients and some unbounded task loops; SUDIN gives individual HTTP calls a 30-second timeout but lacks a whole-session timeout; unconsents loops indefinitely in its generic heartbeat worker.

**429 rate limit:** The server is asking the client to slow down. Go recursively retries without a cap. SUDIN has a bounded attempt loop. aiko uses discord.js REST infrastructure and logs rate-limit warnings.

**Background concurrency:** Go starts a goroutine per Quest; aiko starts concurrent Promises per Quest; unconsents also starts asynchronous work per account. More concurrency means more outstanding work and possible rate pressure; it says nothing about the validity of progress.

**Reconnect and cleanup:** A client can lose Gateway connection, restart, or close before an operation completes. A reliable implementation distinguishes aborted, failed and completed and does not leak permissions, open sockets, timers or credentials.

## 7. Official vs unofficial assumptions

Discord's [official FAQ](https://support.discord.com/hc/en-us/articles/22225719947543-Discord-Quests-FAQ) requires legitimate video watching, game participation, desktop streaming with a viewer, or publisher-reported game actions. The [unofficial Quest resource](https://docs.discord.food/resources/quests) is valuable for understanding names, task group semantics and status fields but can change without notice.

The official [Platform Manipulation Policy](https://discord.com/safety/platform-manipulation-policy-explainer) disallows scripts and spoofed apps used to simulate Quests instead of completing them authentically.

## 8. What this research does **not** establish

- A local program executing doesn't mean the particular Quest can complete without the official app/game.
- No live Quest task or reward was independently tested.
- No author RAM or throughput claims were benchmarked.
- No general solution was demonstrated for externally verified game achievements or legitimate console progress.
- No project was declared malware-free from a README or limited static review.
