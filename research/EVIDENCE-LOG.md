# Evidence ledger

**Evidence collected/reviewed 2026-11-01.** None of these rows are independently reproduced live Quest completions by this repository.

| ID | Basis | Observation / claim | Class | State | Limitation |
|---|---|---|---|---|---|
| E-001 | Official FAQ | Video watching on desktop/browser and the screen-switch pause rule | OFFICIAL | Documented | No automated acceptance test |
| E-002 | Official FAQ | Desktop gameplay requires Discord desktop login | OFFICIAL | Documented | Task-specific campaign rules vary |
| E-003 | Official FAQ | Stream requirement includes another user in voice | OFFICIAL | Documented | Claimed spoof methods may conflict |
| E-004 | Official FAQ | Publisher-linked game actions can have a delay of up to 36h | OFFICIAL | Documented | No generic client-only game achievement |
| E-005 | Unofficial Quest reference | 10 named task identifiers with PLAY_ON_DESKTOP_V2 deprecated | REFERENCE | Documented | Observed undocumented schema may change |
| E-006 | Unofficial Quest reference | Task group join operator is AND or OR | REFERENCE | Documented | Implementation must handle both |
| E-007 | yeweroooo Go README | Nine named handlers including untested console/activity and legacy v2 | IMPLEMENTED CLAIM | Untested entries present | No live independent validation |
| E-008 | aiko-chan-ai README | STREAM_ON_DESKTOP explicitly unsupported | UNSUPPORTED | Author statement | Other task support not independently tested |
| E-009 | Orion README | Simulated STREAM_ON_DESKTOP failed to credit heartbeat | MAINTAINER REPORT | Negative reported test | Differs from claims in other repos |
| E-010 | saintordevil README | Stream needs real viewer; linked console tracking needs real game | CONDITIONAL | Author statement | Tracking != auto-completion |
| E-011 | Orion README | Client progress could falsely increase without server credit | MAINTAINER REPORT | Bug and fix documented | Historical, release dependent |
| E-012 | Orion README | Application-specific achievement route claimed | MAINTAINER REPORT | Claimed live validation | Not generalized to in-game achievement |
| E-013 | QuestCompleter README | Mobile video campaigns reportedly live validated | MAINTAINER REPORT | Author statement | Not independently reproduced |
| E-014 | SubhuJaat07 README | 34 mock end-to-end assertions reported | IMPLEMENTED CLAIM | Mock coverage claimed | Mocks are not production evidence |
| E-015 | Discord Cloud Play FAQ | Experimental cloud play, desktop completion and publisher login | OFFICIAL | Documented | Not equivalent to ordinary game presence |
| E-016 | Discord platform manipulation policy | Simulated Quest completion prohibited | OFFICIAL | Documented | No safe selfbot guarantee |
| E-017 | GitLab topic directory | DiscordBotAPI Quest auto-completer candidate listed | DISCOVERY | Not inspected | Must audit content before classifying |
| E-018 | Discord security bug bounty | Individual Quest manipulation not an ordinary bounty target | OFFICIAL | Documented | Does not authorize misuse |

## Evidence grading

- **Official:** user-facing rule or platform policy.
- **Reference:** community-maintained resource description; not contractual.
- **Maintainer report:** author's described observations, which can be mistaken or stale.
- **Implemented claim:** developer says handler exists; not server-confirmed.
- **Discovery:** candidate project was found, but functionality not verified.

**Explicit non-findings:** No comprehensive independent demonstration of every current Quest task; no production verification of all Go handlers; no general solution to ACHIEVEMENT_IN_GAME; no guarantee that any user token will remain safe when used in prohibited automation.

## Next audit work

For any project moving from “claimed” to “verified”, record a tested commit SHA, test date, exact task type, test environment, objective source and server outcome. Evidence must be collected only by authorized, policy-compliant means. Never publish a token, cookie, redemption code or private account data.

Related files: [source index](SOURCES.md), [matrix](../docs/COMPATIBILITY-MATRIX.md), [failure modes](../docs/FAILURE-MODES.md).
