# Source audit: aiko-chan-ai TypeScript selfbot

- **Upstream:** https://github.com/aiko-chan-ai/Discord-Quest-Auto-Completion-Selfbot
- **Pinned branch:** [main @ 4dd4e6b535cc799708f60cfcbff0ecf33eb504ee](https://github.com/aiko-chan-ai/Discord-Quest-Auto-Completion-Selfbot/tree/4dd4e6b535cc799708f60cfcbff0ecf33eb504ee)
- **Review:** 2026-11-01; static source inspection only.
- **Verdict:** Good reference for a small TypeScript standalone selfbot; explicitly incomplete for streaming and multi-path Quest semantics.

## Architecture

`bot.ts` instantiates `ClientQuest` and orchestrates quests concurrently using `Promise.allSettled`. `src/client.ts` extends discord.js core and uses REST v10 and WebSocket handling. `src/questManager.ts` manages task priority and progress loops, enrollment, reward-claim code (not guaranteed operating) and application-specific activity achievement handling. `src/utils.ts` manages client properties, application-specific OAuth helper and optional webhook integration.

## Findings

| Severity | Finding | Source |
|---|---|---|
| **High** | Selects the first available task from a fixed priority list; no full AND/OR task evaluation. | [questManager.ts L290–307](https://github.com/aiko-chan-ai/Discord-Quest-Auto-Completion-Selfbot/blob/4dd4e6b535cc799708f60cfcbff0ecf33eb504ee/src/questManager.ts#L290-L307) |
| **Definitive unsupported** | `STREAM_ON_DESKTOP` branch says the task does not work in Node and directs the user to the desktop app. | [questManager.ts L340–347](https://github.com/aiko-chan-ai/Discord-Quest-Auto-Completion-Selfbot/blob/4dd4e6b535cc799708f60cfcbff0ecf33eb504ee/src/questManager.ts#L340-L347) |
| **Untested** | Xbox, PlayStation, Play Activity are advertised but labeled untested in the README. They share a generalized task implementation rather than platform-specific verification. | [README](https://github.com/aiko-chan-ai/Discord-Quest-Auto-Completion-Selfbot), [questManager.ts L318–339](https://github.com/aiko-chan-ai/Discord-Quest-Auto-Completion-Selfbot/blob/4dd4e6b535cc799708f60cfcbff0ecf33eb504ee/src/questManager.ts#L318-L339) |
| **Sensitive** | An application-specific OAuth authorization + Discord Says flow is present for `ACHIEVEMENT_IN_ACTIVITY`. This does not generalize to `ACHIEVEMENT_IN_GAME` and creates user-grant risk. | [questManager.ts L496–579](https://github.com/aiko-chan-ai/Discord-Quest-Auto-Completion-Selfbot/blob/4dd4e6b535cc799708f60cfcbff0ecf33eb504ee/src/questManager.ts#L496-L579), [utils.ts](https://github.com/aiko-chan-ai/Discord-Quest-Auto-Completion-Selfbot/blob/4dd4e6b535cc799708f60cfcbff0ecf33eb504ee/src/utils.ts) |
| **Claim limitations** | README explicitly crosses out automatic reward redemption; claiming code includes an unresolved CAPTCHA-related TODO. | [README](https://github.com/aiko-chan-ai/Discord-Quest-Auto-Completion-Selfbot), [questManager.ts L245–260](https://github.com/aiko-chan-ai/Discord-Quest-Auto-Completion-Selfbot/blob/4dd4e6b535cc799708f60cfcbff0ecf33eb504ee/src/questManager.ts#L245-L260) |
| **Positive** | Uses a known REST client library, separate quest lifecycle and utilities, and promise settlement across concurrent tasks. | [client.ts](https://github.com/aiko-chan-ai/Discord-Quest-Auto-Completion-Selfbot/blob/4dd4e6b535cc799708f60cfcbff0ecf33eb504ee/src/client.ts), [bot.ts](https://github.com/aiko-chan-ai/Discord-Quest-Auto-Completion-Selfbot/blob/4dd4e6b535cc799708f60cfcbff0ecf33eb504ee/bot.ts) |

No committed test suite was visible at the inspected root/src level. The README cautions about Discord enforcement and GitHub Actions misuse. These caveats are substantive, not proof of security.

## Conclusion

Useful historical architectural baseline and activity-specific research; **not** a complete solution for streamed, console-linked, game-achievement or multi-requirement Quests. Source completeness and ongoing current support remain unverified.
