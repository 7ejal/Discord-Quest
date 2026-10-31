# Source audit: Orion (nyxxbit)

**Source:** https://github.com/nyxxbit/discord-quest-completer  
**Pinned revision:** [7b21ac365dd38686d25933dc84d46e2bdb2c2b2a](https://github.com/nyxxbit/discord-quest-completer/tree/7b21ac365dd38686d25933dc84d46e2bdb2c2b2a)  
**Reviewed:** November 1, 2026. Static inspection only; tests identified but not executed.

## Engineering assessment

**Strongest observed architecture for error handling, progress confidence, test organization and documentation.** This is a userscript/Vencord integration, not a standalone Go/Python/Node selfbot, and it does not support every task.

- `questConfig.ts`: current/legacy configuration fields, task selection, source application IDs, unsupported types.
- `tasks.ts`: task progress state, server feedback, 25-minute maximum task duration in several flows and failure thresholds.
- `heartbeatWatchdog.ts`: makes lack of accepted progress observable rather than letting tasks wait indefinitely.
- `traffic.ts`: request coordination and retry classification.
- `oauthLifecycle.ts`: application-authorization lifecycle management; sensitive permissions should not be granted to untrusted software.
- `questRewards.ts`: reward handling helper logic.
- `tests/`: twelve separately named test files covering quest configuration, rewards, heartbeat watchdogs, task selection, identity, lifecycle and scheduler behavior. Tests were **not run** in this audit.

## Source evidence

| Claim | Evidence |
|---|---|
| Client config parsing and unsupported task rules | [questConfig.ts](https://github.com/nyxxbit/discord-quest-completer/blob/7b21ac365dd38686d25933dc84d46e2bdb2c2b2a/questConfig.ts) |
| Explicit server progress checks, timeouts and error callbacks | [tasks.ts lines 90–97, 350–610](https://github.com/nyxxbit/discord-quest-completer/blob/7b21ac365dd38686d25933dc84d46e2bdb2c2b2a/tasks.ts#L350-L610) |
| Dedicated heartbeat failure watchdog | [heartbeatWatchdog.ts](https://github.com/nyxxbit/discord-quest-completer/blob/7b21ac365dd38686d25933dc84d46e2bdb2c2b2a/heartbeatWatchdog.ts) |
| Internal streaming metadata alone was not sufficient for a genuine credited stream | [architecture discussion](https://github.com/nyxxbit/discord-quest-completer/blob/7b21ac365dd38686d25933dc84d46e2bdb2c2b2a/docs/ARCHITECTURE.md#L105-L139) |
| Multiple targeted regression suites | [tests/](https://github.com/nyxxbit/discord-quest-completer/tree/7b21ac365dd38686d25933dc84d46e2bdb2c2b2a/tests) |

## Limitations

- **Streaming cannot be completed by its simulation alone** according to maintainer analysis; authentic Go Live and an additional voice viewer remain important.
- Publisher-controlled `ACHIEVEMENT_IN_GAME` is not a generic client progress task.
- External activity achievements involve additional app-controlled validation and account authorization; do not assume general support.
- The userscript depends on Discord internal modules and may not work in unmodified Stable clients.
- Source tests and maintainer observations are not independent live verification.
- Discord explicitly prohibits falsified Quest progress and unauthorized selfbots/client modifications.

**Verdict:** Best to study for state-machine correctness, watchdogs, failure categories, architecture and test discipline. Not an account-safe recommendation to execute.
