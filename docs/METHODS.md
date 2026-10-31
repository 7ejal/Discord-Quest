# Research on Quest tracking and claimed automation methods

**Scope:** Technical taxonomy of public methods, for evaluation and defensive accuracy—not instructions to automate a user account. Discord [prohibits simulated Quest completion](https://discord.com/safety/platform-manipulation-policy-explainer). The existence of a protocol handler is not proof of legitimate or accepted completion.

## Mechanism families

### 1. Official participation and in-client tracking
- Genuine video watching in the supported app/browser; switching windows can pause playback.
- Genuine desktop game detection, streaming with another viewer, and active embedded sessions.
- A publisher-linked account reporting actual game achievements.
- Linked Xbox/PlayStation platform tracking and a real cloud game when supported.
- Best baseline for all compatibility studies. Official requirements: [Quest FAQ](https://support.discord.com/hc/en-us/articles/22225719947543-Discord-Quests-FAQ), [Cloud Play](https://support.discord.com/hc/en-us/articles/35372187686295-Discord-Cloud-Play-Quests).

### 2. Discord web/desktop console scripts
- Community scripts inspect Discord's in-memory Quest store and attempt to drive progress using client-context functionality.
- Strength: can observe client state without shipping a full application.
- Weakness: minified module references, runtime restructuring, Content Security Policy, browser/desktop differences, and account policy.
- References: [aamiaa original gist](https://gist.github.com/aamiaa/204cd9d42013ded9faf646fae7f89fbb), [Orion](https://github.com/nyxxbit/discord-quest-completer), [QuestMaster](https://github.com/power0matin/discord-quest-auto-completer).

### 3. Modified-client plugins
- Vencord/BetterDiscord projects hook client stores and sometimes invoke native claim functionality.
- Strength: integrated runtime and event subscriptions. Weakness: Discord client updates, permissions and plugin trust, installation complexity, and selfbot/client-modification policy.
- References: [QuestCompleter](https://github.com/saintordevil/questCompleter), [Orion plugin](https://github.com/nyxxbit/discord-quest-completer), [AutoQuestComplete](https://github.com/xenoncolt/AutoQuestComplete), [QuestAutocompleter](https://github.com/Seramicx/discord-quest-autocompleter).

### 4. Standalone REST/Gateway selfbots
- Python, Go, and Node/TypeScript projects operate separately from Discord's official GUI, using a personal user account.
- Claimed subsystems: Quest listing and enrollment, progress/heartbeat, playing presence, retry/backoff, concurrent jobs and optional claims.
- Critical risk: user tokens, untrusted HTTP destinations, credential persistence, proxy/captcha services, and fabricated completion.
- References: [Go selfbot](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot), [aiko-chan-ai](https://github.com/aiko-chan-ai/Discord-Quest-Auto-Completion-Selfbot), [unconsents](https://github.com/unconsents/auto-quest-completer), [SUDIN010](https://github.com/SUDIN010/discord-auto-quest), [xgg-2 framework](https://github.com/xgg-2/discord-quests).

### 5. Local executable/game-detection experiments
- A Windows utility can create a dummy process recognizable to Discord rather than replacing its internal store.
- Strength: fewer dependencies on Discord's minified JavaScript. Weakness: Windows-specific, limited to process-based gameplay, and not equivalent to genuine gameplay.
- Example: [markterence desktop tool](https://github.com/markterence/discord-quest-completer).

### 6. Browser extensions
- Inject into Discord web session; suited to web-visible video state.
- Desktop-only game presence, native streaming, and console/game-provider achievements cannot be assumed from browser capabilities.
- Example: [nvckai extension](https://github.com/nvckai/Discord-Web-Auto-Quest-Extension).

### 7. Hosted REST dashboard or worker
- Some repositories run jobs and persistence server-side. A functioning server job is not necessarily a valid desktop/voice/game session.
- Advantages claimed: scheduling, retries, multiple tasks and state recovery.
- Risks: credential custody, logs, availability, token exposure and optimistic completion criteria.
- Example: [SubhuJaat07](https://github.com/SubhuJaat07/discord-quest-completor).

### 8. Achievement/application-specific paths
- A separate game or embedded application may issue the actual achievement evidence; this is **not** interchangeable with video or desktop heartbeats.
- Some research repos claim OAuth-based application-specific progress submission; those claims are bounded to a specific flow and should not be called universal.
- Documentation: [Unofficial Quest task reference](https://docs.discord.food/resources/quests); contrasting interpretations: [Orion](https://github.com/nyxxbit/discord-quest-completer), [QuestCompleter](https://github.com/saintordevil/questCompleter).

### 9. Console-linked and cloud game flows
- A platform/account-linking step and actual platform play are fundamentally different from a local Discord timer.
- Cloud Play has publisher authentication and a live in-client cloud session; eligible campaigns vary.
- Sources: [QuestCompleter](https://github.com/saintordevil/questCompleter), [Cloud Play FAQ](https://support.discord.com/hc/en-us/articles/35372187686295-Discord-Cloud-Play-Quests).

### 10. Reward claiming and redemption
- Completion, server claim eligibility, native claim UI, captcha challenge, reward code and external redemption are separate states.
- A selfbot reporting 100% without verified completed_at and claimed_at must not be labeled successful.
- Sources: [Quest structures](https://docs.discord.food/resources/quests), [QuestCompleter](https://github.com/saintordevil/questCompleter).

## Research criteria for comparing methods

|Criterion|What must be documented|
|---|---|
|Environment|Discord desktop/web/mobile, OS, app version and platform|
|Quest schema|Config version, AND/OR groups, task, app ID, target|
|Activity proof|Real game/stream/video/activity, linked account or merely simulated state|
|Progress proof|What Discord actually reported, not merely a local counter|
|Completion proof|Server completion timestamp or equivalent official state|
|Claim proof|Reward claim confirmation, not just attempted HTTP request|
|Failure safety|Timeouts, rate limits, reconnect, cleanup, challenges|
|Security|How account authorization is handled; any external transmission|
|Replicability|Test date, environment, redacted artifacts, independent check|


## Do not conflate these
- **Has a handler** vs **sends a request** vs **Discord credits progress** vs **Discord marks completed** vs **reward is claimed**.
- **Can run concurrently** vs **Discord allows concurrent objective crediting**.
- **Supports a console tracking UI** vs **finishes a console quest without a real game**.
- **Supports an achievement-specific backend** vs **can complete any game's achievements**.
- **Loads in browser** vs **can emulate desktop game/voice/native state**.
