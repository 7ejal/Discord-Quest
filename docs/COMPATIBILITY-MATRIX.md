# Quest compatibility matrix (claim-level, not live-tested)

This file is deliberately conservative. **No cell represents our own successful live Quest test.** Sources include repository READMEs reviewed on 2026-11-01 and the [unofficial Quest task catalog](https://docs.discord.food/resources/quests).

## Legend

- **C** = claimed handler/implementation (not independently confirmed).
- **U** = explicitly untested.
- **K** = conditional genuine activity required (e.g., viewer or real console game).
- **N** = explicitly unsupported.
- **?** = no substantiated claim/insufficient evidence.
- **D** = deprecated historical task ID; any claimed support does not add current coverage.
- **A** = application-specific achievement method claimed, not general across all campaigns.

|Task|Go yew|TS aiko|Orion|QuestCompleter|xgg-2|unconsents|SUDIN010|
|---|---|---|---|---|---|---|---|
|WATCH_VIDEO|C|C|C|C|C|C|C|
|WATCH_VIDEO_ON_MOBILE|C|C|C|C|C|C|C|
|PLAY_ON_DESKTOP|C|C|C|C|C|C|C|
|PLAY_ON_DESKTOP_V2|D|?|D|D|?|?|?|
|STREAM_ON_DESKTOP|C|N|N|K|C|K|N|
|PLAY_ACTIVITY|U|U|C|C|C|C|N|
|ACHIEVEMENT_IN_ACTIVITY|A|A|A|N|?|?|?|
|ACHIEVEMENT_IN_GAME|N|N|N|N|?|?|?|
|PLAY_ON_XBOX|U|U|?|K|?|?|?|
|PLAY_ON_PLAYSTATION|U|U|?|K|?|?|?|


Notes: Source labels can differ by version, particularly Orion/Go achievement support. Orion recognizes stream tasks but documents **failure to obtain credited progress** from its simulated-stream approach. QuestCompleter console tracking does not replace gameplay. SUDIN010 describes mobile video support in its README, though its narrower feature table emphasizes desktop/video.

## Coverage scores (what scores mean)

**Do not use a single support count to decide which project completes every Quest.** Counting task names mixes current/deprecated identifiers, untested code, real-game requirements and server-managed achievements.

- **Go yew:** nine named handlers in its README, including a deprecated type and three marked untested; no generalized game achievement.
- **Orion:** strong coverage of common client-driven categories and application-specific achievement claims, but explicitly negative stream test evidence.
- **saintordevil:** supports native tracking/claim flows for some tasks, with authentic stream/console conditions; refuses to fabricate achievements.
- **xgg-2:** five common named handlers but no independent streaming success; extensibility is not automatic coverage of unseen types.
- **unconsents:** claims the five common categories; real viewer required for streaming; credential-handling risk.
- **aiko-chan-ai:** stream explicitly unsupported; several remaining types labeled untested.
- **SUDIN010:** a deliberately narrower video/desktop scope.

## Quest-level coverage is not task-level coverage

An OR task group may be satisfiable through one valid task, while an AND group requires every required task. A project may understand an identifier yet fail because of platform restrictions, expiry, remote provider confirmation, or claiming.

**Confidence:** Official definitions are comparatively high; unofficial schema is informative but not contractual; project coverage evidence is low-to-medium and version-sensitive.

## Primary repository links

[Go](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot) · [aiko](https://github.com/aiko-chan-ai/Discord-Quest-Auto-Completion-Selfbot) · [Orion](https://github.com/nyxxbit/discord-quest-completer) · [QuestCompleter](https://github.com/saintordevil/questCompleter) · [xgg-2](https://github.com/xgg-2/discord-quests) · [unconsents](https://github.com/unconsents/auto-quest-completer) · [SUDIN010](https://github.com/SUDIN010/discord-auto-quest).
