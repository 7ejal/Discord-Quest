# Source audit: SubhuJaat07 hosted Quest server

**Upstream:** https://github.com/SubhuJaat07/discord-quest-completor  
**Pinned revision:** [e2097af7c0ea23e3a6b85c979eb3438c9570f1eb](https://github.com/SubhuJaat07/discord-quest-completor/tree/e2097af7c0ea23e3a6b85c979eb3438c9570f1eb)  
**Reviewed:** 2026-11-01, static code inspection. No server deployed or integration test run.

## Architecture

Next.js web UI, API routes, `src/lib/quest-engine.ts` (~1,019 lines) for Quest discovery, enrollment, video and heartbeat sessions, an in-memory job/queue map, `monitor.ts` for scheduled checks, `session.ts` for login sessions, and `state-store.ts` for encrypted on-disk/optional cloud monitor records. It includes mock-server and end-to-end test scripts in `scripts/`.

## Findings

| Severity | Finding | Evidence |
|---|---|---|
| **Critical credential exposure** | The login session stores the user's **raw Discord token directly as the value of an HttpOnly cookie**, with a 7-day lifetime. HttpOnly is beneficial against ordinary JS reads, but does **not** encrypt cookie content or prevent the server/browser from possessing the token. | [session.ts L133–162](https://github.com/SubhuJaat07/discord-quest-completor/blob/e2097af7c0ea23e3a6b85c979eb3438c9570f1eb/src/lib/session.ts#L133-L162) |
| **High correctness** | `finishQuest` marks the local session `completed` after a best-effort terminal request and assigns its own local completion timestamp, **without fetching and validating authoritative server Quest status**. | [quest-engine.ts L662–695](https://github.com/SubhuJaat07/discord-quest-completor/blob/e2097af7c0ea23e3a6b85c979eb3438c9570f1eb/src/lib/quest-engine.ts#L662-L695) |
| **High correctness** | If video progress response lacks a task progress record, the loop accepts its *proposed* timestamp as local credited progress. This can lead to a misleading progress display, even after an HTTP success. | [quest-engine.ts L621–651](https://github.com/SubhuJaat07/discord-quest-completor/blob/e2097af7c0ea23e3a6b85c979eb3438c9570f1eb/src/lib/quest-engine.ts#L621-L651) |
| **High task semantics** | Five supported task names share only two generic methods (video vs heartbeat); source routing does not establish real stream/game/Activity requirements. | [quest-engine.ts L157–165](https://github.com/SubhuJaat07/discord-quest-completor/blob/e2097af7c0ea23e3a6b85c979eb3438c9570f1eb/src/lib/quest-engine.ts#L157-L165), [L480–490](https://github.com/SubhuJaat07/discord-quest-completor/blob/e2097af7c0ea23e3a6b85c979eb3438c9570f1eb/src/lib/quest-engine.ts#L480-L490) |
| **Medium** | Video errors fall back to a generic heartbeat path after repeated rejections. A fallback's existence does not prove equivalence for every campaign/task. | [quest-engine.ts L590–623](https://github.com/SubhuJaat07/discord-quest-completor/blob/e2097af7c0ea23e3a6b85c979eb3438c9570f1eb/src/lib/quest-engine.ts#L590-L623) |
| **Positive** | 30-second HTTP timeout, limited 429 retry count, hard-error detection and stuck-progress warnings. Session queues and monitoring support persistent workflows. | [quest-engine.ts L175–208](https://github.com/SubhuJaat07/discord-quest-completor/blob/e2097af7c0ea23e3a6b85c979eb3438c9570f1eb/src/lib/quest-engine.ts#L175-L208), [L500–575](https://github.com/SubhuJaat07/discord-quest-completor/blob/e2097af7c0ea23e3a6b85c979eb3438c9570f1eb/src/lib/quest-engine.ts#L500-L575) |
| **Positive but limited scope** | **Monitor persistence** encrypts tokens at rest with AES-256-GCM before writing disk/cloud state. This does **not** change the raw-token browser cookie issue above. | [state-store.ts L1–94](https://github.com/SubhuJaat07/discord-quest-completor/blob/e2097af7c0ea23e3a6b85c979eb3438c9570f1eb/src/lib/state-store.ts#L1-L94) |
| **Mock tests** | Scripts include a mock Discord server and end-to-end test script. These do not prove actual Discord production acceptance or account safety. | [scripts/](https://github.com/SubhuJaat07/discord-quest-completor/tree/e2097af7c0ea23e3a6b85c979eb3438c9570f1eb/scripts) |

## Assessment

Designed for a persistent remote UI rather than a lightweight local CLI. The risks of entrusting a **personal account token** to a hosted service should be weighed separately from claimed task support. A policy-compliant research dashboard should not collect Discord user tokens.

**Verdict:** Useful architecture for job queues and mocks; **not** the best reference for credential safety or authoritative completion reporting in its current inspected source.
