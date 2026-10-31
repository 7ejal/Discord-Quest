# Open-source project inventory

**As of 2026-11-01.** These are **project claims and source-documentation observations**, not independent successful live tests. Project content can change; re-open a repository before treating details as current.

## Standalone/selfbot/framework implementations

|Project|Language / format|Claimed strengths|Verified limitations / warnings|
|---|---|---|---|
|[yeweroooo](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot)|Go standalone selfbot|Nine task identifiers; concurrent jobs; runtime client properties; optional claim integration|Its own README labels Xbox, PlayStation and Activity untested; PLAY_ON_DESKTOP_V2 is deprecated; no ACHIEVEMENT_IN_GAME|
|[aiko-chan-ai](https://github.com/aiko-chan-ai/Discord-Quest-Auto-Completion-Selfbot)|TypeScript selfbot|Auto-enroll; video, desktop, mobile, activity-achievement claims; scheduling example|Explicitly no STREAM_ON_DESKTOP; Xbox, PS and Activity marked untested; rewards not auto-redeemed|
|[unconsents](https://github.com/unconsents/auto-quest-completer)|Python multi-account|Concurrent video, desktop, stream, activity claimed; auto-enrollment|Requires multiple user tokens; stream requires other voice participant; captcha challenges can block enrollment|
|[SUDIN010](https://github.com/SUDIN010/discord-auto-quest)|Python CLI|Video/mobile-video and desktop game; rate-limit retry; presence|Explicitly no stream or activity; no automatic reward claim|
|[lfathh](https://github.com/lfathh/Auto-Quest-Discord)|TypeScript CLI|CLI selfbot, progress and reward information|Task-level claims must be verified in source; no general achievement/console evidence|
|[xgg-2](https://github.com/xgg-2/discord-quests)|Node.js/TS library|Five common handlers, extensibility, proxy support, isolated rate limits|A registered custom handler proves extensibility, not that a new task is completable; streaming claim conflicts with other research|
|[SubhuJaat07](https://github.com/SubhuJaat07/discord-quest-completor)|JS/TS hosted server|Self-hosted queues, monitoring, mock E2E tests, REST strategy|Mock tests are not live Quest confirmation; user token custody, streaming and claim limitations|


## Client-integrated and desktop implementations

|Project|Delivery|Research value|Important limitation|
|---|---|---|---|
|[Orion](https://github.com/nyxxbit/discord-quest-completer)|Userscript + Vencord plugin|Extensive changelog and failure analysis; app-specific achievement method; server-confirmed progress safeguards|Explicitly documents stream failure; userscript depends on Discord/Vencord internals; sensitive achievement flow|
|[saintordevil QuestCompleter](https://github.com/saintordevil/questCompleter)|Vencord plugin|AND/OR handling, reward claiming via native UI, safe cleanup, mobile-video maintainer tests, console tracking|Stream needs real viewer; consoles need real gameplay; achievement tasks intentionally not fabricated|
|[markterence](https://github.com/markterence/discord-quest-completer)|Windows desktop tool|Dummy executable/process approach without needing full game download|Limited primarily to Windows desktop gameplay; not a general Quest solver|
|[nicola02nb](https://github.com/nicola02nb/completeDiscordQuest)|Vencord plugin|Older implementation derived from console scripts|Orion reports breakage after application-id configuration changes; independently recheck|
|[Seramicx](https://github.com/Seramicx/discord-quest-autocompleter)|Vencord plugin|Auto-accept, queue, optional claims|Feature claims require source and dated live validation|
|[xenoncolt](https://github.com/xenoncolt/AutoQuestComplete)|BetterDiscord plugin|Alternate client plugin approach|No independent complete task coverage established|
|[nvckai](https://github.com/nvckai/Discord-Web-Auto-Quest-Extension)|Browser extension|Accessible video-focused browser automation|Browser-only constraints on desktop game and stream states|
|[power0matin](https://github.com/power0matin/discord-quest-auto-completer)|JavaScript console script|Quest overview/progress/dashboard-style controls|Client runtime breakage risk; coverage not independently tested|
|[aamiaa original](https://gist.github.com/aamiaa/204cd9d42013ded9faf646fae7f89fbb)|GitHub gist|Historical baseline used by multiple forks|History of scripts is not proof of current compatibility|
|[Ghraven v2](https://gist.github.com/Ghraven/5f8dc950b8835e091e26a9d155598cc9)|GitHub gist|Alternative multi-quest script|No independent Quest-type matrix established|


## Reliability ranking (research quality, NOT a recommendation to run)

1. **Most detailed engineering/failure documentation:** [Orion](https://github.com/nyxxbit/discord-quest-completer), which openly lists failures, UI regressions, runtime changes and risk.
2. **Most thorough task/claim state modeling:** [saintordevil/questCompleter](https://github.com/saintordevil/questCompleter), with explicit conditional console/stream requirements and pure-logic tests.
3. **Broadest claimed standalone task count:** [yeweroooo Go](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot). Count is inflated by including deprecated or untested entries.
4. **Most flexible claimed framework:** [xgg-2](https://github.com/xgg-2/discord-quests), but custom solver registration is not proof of server acceptance.
5. **Simplest narrowly scoped standalone:** [SUDIN010](https://github.com/SUDIN010/discord-auto-quest), which honestly documents missing categories.

## Source-specific concerns and contradictions

- The Go project's table literally says **Working (untested)** for multiple entries. In this repository those are classified **UNTESTED** rather than Working.
- Orion says streamed-game spoofing did not receive credited progress; other projects advertise a stream heartbeat. Different codepaths or versions may explain disagreement, but no general success is established.
- Orion reports a mechanism for ACHIEVEMENT_IN_ACTIVITY via a specific application backend; QuestCompleter deliberately does not use it. Neither claim means ACHIEVEMENT_IN_GAME is solved.
- SUDIN010 README installation references an older/different owner path (xdluru), so users should always verify the canonical repository and dependency chain.
- The Go README contains a placeholder example clone URL (YOUR_USERNAME/dqst); documentation mistakes count against setup reliability.
- The unconsents README references a saint-tools clone path rather than its own owner; identify the canonical upstream and verify dependencies.
- GitHub project stars/forks are weak indicators of correctness and **not** evidence of security.
- GitLab: no comparable public GitLab selfbot was independently validated in this review; **absence of a verified result is not proof no projects exist**.

See [Evidence log](../research/EVIDENCE-LOG.md) for dated support classifications.
