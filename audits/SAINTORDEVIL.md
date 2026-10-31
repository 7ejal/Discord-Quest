# Source audit: saintordevil QuestCompleter

- **Upstream:** https://github.com/saintordevil/questCompleter
- **Pinned branch:** [master @ 3e032d4e4110cc7619ace4858451c374e3570e85](https://github.com/saintordevil/questCompleter/tree/3e032d4e4110cc7619ace4858451c374e3570e85)
- **Review:** 2026-11-01; static code inspection, no live completion tests.
- **Verdict:** **Best conservative Quest compatibility and reward-state design** among reviewed integrations. Plugin, not standalone selfbot. Deliberately cannot fabricate all task types.

## Structure

`index.tsx` (~1,563 lines) handles Flux stores, scheduling, enrollment, task state, claim flows and local statistics. `logic.ts` (~255 lines) extracts pure task selection, joining, retry and claim validation. `logic.test.ts` (~155 lines) tests those core behaviors.

## Findings

| Area | Finding | Evidence |
|---|---|---|
| AND/OR | `resolveTask` explicitly evaluates whether candidates satisfy a group and reports unsupported pending tasks. Unknown operators do not silently pass. | [logic.ts L130–174](https://github.com/saintordevil/questCompleter/blob/3e032d4e4110cc7619ace4858451c374e3570e85/logic.ts#L130-L174) |
| Platform preference | `prioritizeTaskNames` distinguishes desktop from console-selected paths. | [logic.ts L150–158](https://github.com/saintordevil/questCompleter/blob/3e032d4e4110cc7619ace4858451c374e3570e85/logic.ts#L150-L158) |
| Retrying | Caps backoff; separates enrollment, task and claim retry maps; classified errors, canceled runtime generation and cleanup paths. | [logic.ts L145–149](https://github.com/saintordevil/questCompleter/blob/3e032d4e4110cc7619ace4858451c374e3570e85/logic.ts#L145-L149), [index.tsx](https://github.com/saintordevil/questCompleter/blob/3e032d4e4110cc7619ace4858451c374e3570e85/index.tsx) |
| Rewards | Uses native claim action; `assessClaimResponse` and `classifyClaimFailure` attempt to distinguish success, transient error and verification. Built-in challenge remains Discord-controlled. | [logic.ts L199–255](https://github.com/saintordevil/questCompleter/blob/3e032d4e4110cc7619ace4858451c374e3570e85/logic.ts#L199-L255), [index.tsx L760–835](https://github.com/saintordevil/questCompleter/blob/3e032d4e4110cc7619ace4858451c374e3570e85/index.tsx#L760-L835) |
| Tests | Tests joining, unsupported/unknown tasks, capped retries, reward selection, response validation and CAPTCHA error classification. **Found, not run.** | [logic.test.ts](https://github.com/saintordevil/questCompleter/blob/3e032d4e4110cc7619ace4858451c374e3570e85/logic.test.ts) |
| Video | README claims multiple campaigns were live-validated by the maintainer; no independent test in this review. | [README](https://github.com/saintordevil/questCompleter) |
| Streaming | Requires **real stream and another viewer**, rather than claiming generic heartbeat-only support. | [README](https://github.com/saintordevil/questCompleter) |
| Consoles | Linked Xbox/PlayStation tracking needs online status, privacy settings and **real platform gameplay**. | [README](https://github.com/saintordevil/questCompleter) |
| Achievements | Recognizes external game/activity achievements but does not falsify either; correctly separates game-controlled evidence. | [logic.ts L7–21](https://github.com/saintordevil/questCompleter/blob/3e032d4e4110cc7619ace4858451c374e3570e85/logic.ts#L7-L21) |

## Gaps and considerations

The main runtime remains tightly coupled to undocumented client stores/Native UI and is concentrated in a large index.tsx, which could be a maintainability hotspot. Logic tests do not prove live server acceptance, especially for the conditional stream/console and claiming flows. This is a Vencord source plugin; packaging requirements differ from a standalone Node or Go selfbot.

**Conclusion:** Strongest reference for **correct task selection and account-conscious claim states**, but does not claim to complete every Quest type without participation.
