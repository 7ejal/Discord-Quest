# Discord Quest task type reference

**Snapshot:** 2026-11-01. **Evidence:** [Discord Quest FAQ](https://support.discord.com/hc/en-us/articles/22225719947543-Discord-Quests-FAQ) (official), [Quest structures](https://docs.discord.food/resources/quests) (unofficial).

Discord describes *Play* and *Video* Quests publicly. Internal task identifiers are more granular. Here are **ten identifiers documented in the unofficial reference**, including one deprecated identifier; this is not a claim that ten are concurrently offered.

## Task inventory

|Identifier|User requirement|Tracking family|Main limitation|Status|
|---|---|---|---|---|
|WATCH_VIDEO|Watch an eligible video in Discord|Video progress|Video timing and campaign eligibility|Active|
|WATCH_VIDEO_ON_MOBILE|Watch video on eligible mobile surface|Video progress|Mobile-specific eligibility|Active|
|PLAY_ON_DESKTOP|Play the specified game with Discord desktop tracking|Desktop presence/activity heartbeat|Actual game detection and server acknowledgement|Active|
|PLAY_ON_DESKTOP_V2|Legacy variation of desktop play|Legacy/varied|Not a reliable current task to target|Deprecated|
|STREAM_ON_DESKTOP|Stream the specified game to another user|Stream session + heartbeat|Real active voice stream and viewer may be required|Active|
|PLAY_ACTIVITY|Run an eligible Discord embedded activity|Activity session/heartbeat|Activity and voice state may be required|Active|
|ACHIEVEMENT_IN_ACTIVITY|Complete an in-activity achievement|Application reports achievement|Application-specific backend evidence|Active|
|ACHIEVEMENT_IN_GAME|Complete an in-game objective|Publisher reports game action|External linked game/publisher trust boundary|Active|
|PLAY_ON_XBOX|Play eligible title on linked Xbox|Linked platform presence|Account link, privacy, eligible game, real session|Active|
|PLAY_ON_PLAYSTATION|Play eligible title on linked PlayStation|Linked platform presence|Account link, privacy, eligible game, real session|Active|


## Task-by-task interpretation

### Video: WATCH_VIDEO / WATCH_VIDEO_ON_MOBILE
- Officially a video must be watched. On desktop/browser, Discord says the video pauses when switching windows. Mobile acceptance/tracking is beta and limited to some campaigns.
- Some repositories report video progress values without playing video. A submitted value and a genuine watched session are different things. Campaign eligibility and server confirmation are separate checks.
- The two identifiers may share a protocol family, but **same endpoint does not imply identical campaign eligibility**.
- Sources: [FAQ](https://support.discord.com/hc/en-us/articles/22225719947543-Discord-Quests-FAQ), [Quest API reference](https://docs.discord.food/resources/quests), [QuestCompleter](https://github.com/saintordevil/questCompleter).

### Desktop gameplay: PLAY_ON_DESKTOP
- Discord's FAQ requires playing a game on a desktop client, signed in to Discord, to track progress.
- Approaches found in third-party projects: game presence simulation, spoofed running-game store, dummy executables for local process recognition, or server-directed progress reporting.
- These approaches have different observable footprints and validity; no method should be called universally reliable merely because a local progress counter advances.
- Historical PLAY_ON_DESKTOP_V2 is crossed out in current unofficial documentation. Supporting that label is not proof of supporting an additional current Quest.
- Sources: [FAQ](https://support.discord.com/hc/en-us/articles/22225719947543-Discord-Quests-FAQ), [Orion](https://github.com/nyxxbit/discord-quest-completer), [Windows app](https://github.com/markterence/discord-quest-completer).

### Streaming: STREAM_ON_DESKTOP
- The official requirement includes using Discord desktop to stream the game to **at least one friend** in voice for the specified duration.
- Projects disagree: some advertise a generic heartbeat path; [Orion](https://github.com/nyxxbit/discord-quest-completer) says its simulated-stream approach did not obtain credited heartbeats; [QuestCompleter](https://github.com/saintordevil/questCompleter) requires an actual voice stream and viewer.
- Therefore distinguish **handler exists** from **genuinely credited streaming**. Report stream state, viewer presence, heartbeat results and server completion independently.
- Source: [FAQ](https://support.discord.com/hc/en-us/articles/22225719947543-Discord-Quests-FAQ).

### Embedded activity: PLAY_ACTIVITY
- The activity session is not equivalent to a generic user presence. Some community clients describe activity heartbeats; eligibility and validation vary by Quest.
- A missing or rejected server progress value must not be replaced by an extrapolated local value. Orion documents an earlier false-success bug from local progress extrapolation.
- Sources: [Quest resource](https://docs.discord.food/resources/quests), [Orion](https://github.com/nyxxbit/discord-quest-completer).

### Achievement in activity: ACHIEVEMENT_IN_ACTIVITY
- The unofficial schema says the **application** tracks achievement completion.
- [Orion](https://github.com/nyxxbit/discord-quest-completer) and the [Go selfbot](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot) claim an application-specific authorization / Discord Says route. This is not a documented generic Quest-progress feature and is particularly sensitive.
- Another project, [QuestCompleter](https://github.com/saintordevil/questCompleter), deliberately treats these as server-managed rather than fabricating them.
- Never generalize one application-specific report to all achievement campaigns.

### Achievement in game: ACHIEVEMENT_IN_GAME
- Official FAQ: the game developer supplies in-game completion data after a linked game-account action; processing may take **up to 36 hours**.
- It is **not** a desktop timer task. No general client-only completion path was independently verified in this research.
- Sources: [FAQ](https://support.discord.com/hc/en-us/articles/22225719947543-Discord-Quests-FAQ), [Unofficial tasks](https://docs.discord.food/resources/quests).

### Linked consoles: PLAY_ON_XBOX / PLAY_ON_PLAYSTATION
- External platform play state is involved; matching task names or a selfbot's heartbeat handler alone does not establish genuine console progress.
- [Go selfbot](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot) marks these **Working (untested)**, a contradiction in its own terminology.
- [QuestCompleter](https://github.com/saintordevil/questCompleter) claims linking/tracking but explicitly requires the real console game, online status, and appropriate privacy settings.
- Do not mark these as automatically completable without gameplay.

## Not standalone task identifiers

- **Cloud Play Quests:** an experimental Discord desktop gaming flow, with publisher authentication and a real cloud game session. Official [Cloud Play FAQ](https://support.discord.com/hc/en-us/articles/35372187686295-Discord-Cloud-Play-Quests).
- **QR handoff surfaces:** may transfer to mobile, login, or console. A QR code is not by itself a verified progress task.
- **Missions, Orbs multipliers, rewards, campaign flags:** concepts and features, not necessarily new progress identifiers.
- **Quest enrollment and reward claim:** lifecycle actions, not evidence that an objective has been met.

## Rules for recording a new identifier

Record: exact spelling; capture date; task configuration version; target; task join operator; platform; evidence source; verified behavior; whether the identifier is deprecated; and why it is not a UI-only label. Never publish account tokens, raw cookies, sealed metadata, or personal campaign information.
