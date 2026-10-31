# Secondary implementation inspections

**Snapshot:** 2026-11-01. **Scope:** Source-file and README inspection at the pinned commits below. This is a **targeted** second-tier review, not a complete function-by-function audit, a malware certification, or a live Quest test.

The [nine main audits](README.md) cover the major standalone and integrated implementations. These six additional repositories cover plugins, browser extensions, desktop executable detection, and large userscripts.

| Project | Pinned revision | Important code observations | Limits / concerns |
|---|---|---|---|
| **QuestMaster (power0matin)** | [b870754](https://github.com/power0matin/discord-quest-auto-completer/tree/b8707549781cbf2efd2d0e675536ea15d27e31fb) | Large ~8,052-line `quest_completer.js` with request timeouts, watchdogs, task handling and UI; ~671-line [reliability.test.js](https://github.com/power0matin/discord-quest-auto-completer/blob/b8707549781cbf2efd2d0e675536ea15d27e31fb/tests/reliability.test.js) contains mocked retry/timeout/task/progress checks | Multiple task types share generic handlers; no independent real-stream or achievement confirmation. README includes a network-fetched PowerShell one-liner; **never run remote scripts without independently reviewing downloaded bytes**. |
| **Seramicx QuestAutocompleter** | [a745abf](https://github.com/Seramicx/discord-quest-autocompleter/tree/a745abf2d1485b9af38e6100d1fea7efa1ac60d4) | ~738-line Vencord userplugin recognizes video, desktop, streaming, activity, activity-achievement, plus account-switch/reload/queue logic | External CAPTCHA solver options and privileged native helper increase trust surface; streamed-task support remains unverified. [index.tsx](https://github.com/Seramicx/discord-quest-autocompleter/blob/a745abf2d1485b9af38e6100d1fea7efa1ac60d4/questAutocompleter/index.tsx), [native.ts](https://github.com/Seramicx/discord-quest-autocompleter/blob/a745abf2d1485b9af38e6100d1fea7efa1ac60d4/questAutocompleter/native.ts) |
| **xenoncolt AutoQuestComplete** | [0b252ac](https://github.com/xenoncolt/AutoQuestComplete/tree/0b252ac53cd84677af7ac943a7883eb0fd3594ce) | ~830-line BetterDiscord plugin. Five named built-in task IDs; source includes achievements/console task labels for presentation | README explicitly credits/copies aamiaa original; **not independent proof of another underlying technique**. Console and game achievements are excluded from completion according to source. [plugin.js](https://github.com/xenoncolt/AutoQuestComplete/blob/0b252ac53cd84677af7ac943a7883eb0fd3594ce/AutoQuestComplete.plugin.js) |
| **nvckai Web Extension** | [23aeaef](https://github.com/nvckai/Discord-Web-Auto-Quest-Extension/tree/23aeaefe15d1fe8d51b9b0fa249196175fe6dd5c) | Manifest V3 browser extension; header request interception, Discord-domain host permissions, content scripts, browser Quest hooks, and five task IDs | Requires powerful access to Discord pages; browser user-agent changes **do not create a real desktop game/stream session**. README says streaming still requires another user in voice. [manifest.json](https://github.com/nvckai/Discord-Web-Auto-Quest-Extension/blob/23aeaefe15d1fe8d51b9b0fa249196175fe6dd5c/manifest.json), [quest-code.js](https://github.com/nvckai/Discord-Web-Auto-Quest-Extension/blob/23aeaefe15d1fe8d51b9b0fa249196175fe6dd5c/quest-code.js) |
| **markterence Windows desktop** | [5319113](https://github.com/markterence/discord-quest-completer/tree/53191138e647304dc7dce6a6ace77ae19f8309c3) | Tauri/Vue/Rust app with separate Windows game-runner source and a local detectable-game list | Focuses on game-process recognition, **not** an all-task Quest engine. Windows supported; Linux/Mac not supported per README. No stream/activity/console/game-achievement universality. [README](https://github.com/markterence/discord-quest-completer/blob/53191138e647304dc7dce6a6ace77ae19f8309c3/README.md) |
| **nicola02nb completeDiscordQuest** | [5a4a664](https://github.com/nicola02nb/completeDiscordQuest/tree/5a4a66493048c80df9c0176fe48e7f0cdf998450) | ~439-line Vencord plugin with task-specific branches for video, desktop, stream and activity | Uses a first-matching-task priority list, not an exhaustive AND/OR evaluator; depends on changing Discord client stores. [index.tsx L249–278](https://github.com/nicola02nb/completeDiscordQuest/blob/5a4a66493048c80df9c0176fe48e7f0cdf998450/index.tsx#L249-L278) |

## What this changes in the ranking

**QuestMaster deserves a closer follow-up audit**, because it is more substantial than a trivial gist and includes a sizeable mock regression suite. Its documentation describes generic streaming task behavior, but having a handler and mock tests is still not proof of genuine completion; the conflicting Orion analysis remains unresolved.

The other five projects primarily add another delivery method (plugin, extension, desktop executable detection), not evidence of comprehensive independent task-type coverage.

No project above is independently confirmed to complete *all* publisher-controlled achievements, linked-console gameplay, genuine desktop streaming, Cloud Play or complex multi-path campaigns.

## Open follow-up tasks

- Audit QuestMaster's entire 8k-line script and its `reliability.test.js` assertions. Compare state/error handling to Orion at pinned revisions, not README marketing.
- Review the native/plugin privileged actions and credential exposure for Seramicx.
- Audit nvckai extension permissions, request rules and content script origins.
- Check desktop dummy-process source/build provenance against distributed release binaries (not done).
- Confirm whether the older plugins' Quest schema handling still matches current task configurations.

**Safety:** Discord [prohibits inauthentic Quest completion](https://discord.com/safety/platform-manipulation-policy-explainer). These source observations are not deployment advice.
