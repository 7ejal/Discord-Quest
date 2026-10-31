# Source audit: yeweroooo Go selfbot

- **Upstream:** https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot
- **Pinned branch:** [golang @ 352d7f457c9511bc9e7c9b940a71012bfb47572e](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/tree/352d7f457c9511bc9e7c9b940a71012bfb47572e)
- **Review:** 2026-11-01, static source inspection. **No live Quest test, full dependency audit or runnable benchmarking.**
- **Verdict:** Most advertised standalone task identifiers; unreliable as a universal Quest completer. Avoid equating “working” in its README with evidence.

## Architecture

`main.go` loads environment credentials, initializes optional CAPTCHA and webhook support, gets dynamic client build metadata, connects to Discord's Gateway, retrieves quests, builds a manager and starts **one Go goroutine per valid quest**; waits for all before disconnecting. `client.go` maintains gateway Identify/heartbeat/presence and a REST wrapper. `quest.go` models enrollment/completion state and task priority. `manager.go` has separate video, heartbeat, activity, streaming, achievement and reward paths. `utils.go` handles version discovery, webhook and an application-specific activity provider; `captcha.go` calls a third-party challenge service.

## Source-backed findings

| Severity | Observation | Evidence |
|---|---|---|
| **High** | `GetTaskName` uses a fixed ordered list and takes **one task**. No semantic evaluation of AND/OR task combinations, completion of other branches or platform preference. | [quest.go L32–52](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/quest.go#L32-L52) |
| **High** | Xbox/PlayStation/desktop/V2 all use the **same** gameplay heartbeat function. Code handling an Xbox task does not prove linked-console gameplay validation. | [manager.go L113–146](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/manager.go#L113-L146), [L227–264](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/manager.go#L227-L264) |
| **High** | Streaming sends Gateway streaming presence plus a regular gameplay-like heartbeat; code comments acknowledge a genuine stream and voice viewer may still be required. | [manager.go L392–450](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/manager.go#L392-L450) |
| **High** | Activities send a **fixed** `call:1:1` session identifier, not an observed live activity session. | [manager.go L269–317](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/manager.go#L269-L317) |
| **High** | 429 retries recurse without an overall attempt cap. Persistently rejected progress in gameplay/activity/stream loops has no hard timeout. This can run indefinitely. | [client.go L270–289](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/client.go#L270-L289), [manager.go L227–317](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/manager.go#L227-L317) |
| **High** | Reward-claiming function exists, but the processing block is **commented out** in main, so it does not run in the standard execution path. | [main.go L70–79](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/main.go#L70-L79), [manager.go L486–565](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/manager.go#L486-L565) |
| **High** | Activity achievement authorization can exit before the later deauthorize step after a failed post-authorization flow; this may leave an application grant. | [manager.go L320–391](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/manager.go#L320-L391) |
| **Medium** | Claim function logs a partial solved CAPTCHA response; avoid recording authentication/challenge artifacts even partially. | [manager.go L530–548](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/manager.go#L530-L548) |
| **Medium** | Per-task application IDs are not used; common config.application.id may be incorrect for certain newer Quest layouts. | [manager.go L117–145](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/manager.go#L117-L145), [Quest config reference](https://docs.discord.food/resources/quests) |
| **Medium** | Dynamic version-discovery depends on Discord's asset format and can break. README's speed/RAM values were not independently benchmarked. | [utils.go](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/utils.go), [README](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot) |
| **Positive** | Separates REST/Gateway, quest logic and application-specific components; reads user_status.completed_at as an end condition; handles basic 429 delay. | [client.go](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/client.go), [quest.go](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/quest.go) |

## Task assessment

- Video + mobile video: handler exists; production credit not independently tested.
- Desktop + legacy V2: handler exists; V2 is deprecated; not a guarantee of current behavior.
- Stream: handler exists, real-stream/viewer validation unresolved.
- Play Activity: handler exists but uses a synthetic fixed session key.
- Achievement in Activity: specific external application method, high authorization risk.
- Xbox, PlayStation: **README itself says untested**; shared desktop heartbeat does not validate true platform support.
- Achievement in Game: no general handler.
- Claiming: not invoked in default main program.

## Risk and testing gaps

No Go test files were found at the repository root in this review. Concurrency does not equal reliable progress or rate safety. Personal user-token credentials, optional webhook data, and optional third-party challenge solving increase exposure. This is **not** evidence of malicious intent.

**Research recommendation:** Valuable for learning standalone Go Gateway/REST structure, not the strongest reliability baseline. See [overall ranking](../research/RECOMMENDATION.md).
