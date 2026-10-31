# Research source index

**Collected:** 2026-11-01. Sources are ranked by provenance; project README statements do **not** imply verified success.

| ID | Source | Provenance | Relevance |
|---|---|---|---|
| OFFICIAL-01 | [Discord Quests FAQ](https://support.discord.com/hc/en-us/articles/22225719947543-Discord-Quests-FAQ) | Official | Public Quest requirements, regional/mobile limits, claiming, game publisher delay |
| OFFICIAL-02 | [Discord Cloud Play FAQ](https://support.discord.com/hc/en-us/articles/35372187686295-Discord-Cloud-Play-Quests) | Official | Experimental cloud session, publisher login, desktop-only completion |
| OFFICIAL-03 | [Discord platform manipulation policy](https://discord.com/safety/platform-manipulation-policy-explainer) | Official | Selfbots, client modification and simulated Quest completion prohibited |
| OFFICIAL-04 | [Discord security bug bounty](https://discord.com/security) | Official | Quest manipulation not ordinarily in security bounty scope |
| REFERENCE-01 | [Discord Userdoccers Quest resource](https://docs.discord.food/resources/quests) | Unofficial | Task identifiers, config versions, status/reward types, interface examples |
| PROJECT-01 | [yeweroooo Go selfbot](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot) | Developer | 9 advertised task types, 3 marked untested, legacy desktop v2 |
| PROJECT-02 | [aiko-chan-ai TypeScript selfbot](https://github.com/aiko-chan-ai/Discord-Quest-Auto-Completion-Selfbot) | Developer | Video, desktop, activity achievement, explicit stream unsupported |
| PROJECT-03 | [Orion](https://github.com/nyxxbit/discord-quest-completer) | Developer | Detailed failure analysis, versions, app-specific achievement flow |
| PROJECT-04 | [saintordevil QuestCompleter](https://github.com/saintordevil/questCompleter) | Developer | Vencord task parsing, real-stream/console conditionality, native claiming |
| PROJECT-05 | [xgg-2 discord-quests](https://github.com/xgg-2/discord-quests) | Developer | 5 built-in claimed task handlers, extensibility and rate-limit design |
| PROJECT-06 | [unconsents auto-quest-completer](https://github.com/unconsents/auto-quest-completer) | Developer | Python multi-account claims; live stream viewer requirement |
| PROJECT-07 | [SUDIN010 discord-auto-quest](https://github.com/SUDIN010/discord-auto-quest) | Developer | Python video/desktop scope, no stream/activity |
| PROJECT-08 | [SubhuJaat07 server app](https://github.com/SubhuJaat07/discord-quest-completor) | Developer | Server jobs, queue, mock tests, credential and claim limitations |
| PROJECT-09 | [lfathh TypeScript CLI](https://github.com/lfathh/Auto-Quest-Discord) | Developer | Standalone selfbot implementation |
| PROJECT-10 | [markterence desktop app](https://github.com/markterence/discord-quest-completer) | Developer | Windows game-detection/dummy process approach |
| PROJECT-11 | [Seramicx QuestAutocompleter](https://github.com/Seramicx/discord-quest-autocompleter) | Developer | Vencord plugin |
| PROJECT-12 | [xenoncolt AutoQuestComplete](https://github.com/xenoncolt/AutoQuestComplete) | Developer | BetterDiscord plugin |
| PROJECT-13 | [nicola02nb completeDiscordQuest](https://github.com/nicola02nb/completeDiscordQuest) | Developer | Earlier Vencord lineage, reported compatibility failures |
| PROJECT-14 | [nvckai browser extension](https://github.com/nvckai/Discord-Web-Auto-Quest-Extension) | Developer | Web extension focused on browser-suitable tasks |
| PROJECT-15 | [power0matin console script](https://github.com/power0matin/discord-quest-auto-completer) | Developer | JavaScript dashboard console helper |
| PROJECT-16 | [aamiaa gist](https://gist.github.com/aamiaa/204cd9d42013ded9faf646fae7f89fbb) | Developer | Original widely referenced script lineage |
| PROJECT-17 | [Ghraven gist v2](https://gist.github.com/Ghraven/5f8dc950b8835e091e26a9d155598cc9) | Developer | Alternative console script |
| PROJECT-18 | [aozoracyrus derived selfbot](https://github.com/aozoracyrus/Auto-Discord-Quest) | Developer | Readme closely follows aiko-chan-ai; not independent evidence |
| GITLAB-01 | [NexusSecurity-group DiscordBotAPI](https://gitlab.com/NexusSecurity-group/DiscordBotAPI) | Unverified candidate | GitLab topic search advertises combined Discord bot and Quest auto-completer; implementation not inspected |

## Retrieval methodology

- Official URLs were opened/read as the highest-confidence sources for **user requirements, policies and rewards**.
- The unofficial reference was used for technical identifiers and schema; it is not a Discord-supported public API contract.
- Selected public GitHub repository READMEs were read with dated results. The Go source README is on branch **golang**, aiko-chan-ai on **main**, Orion on **main**, and saintordevil on **master** at the time examined. No full line-by-line security audit or end-to-end real-account test was done.
- Projects whose source was not examined deeply appear for **discovery and follow-up**, not as confirmed working implementations.
- A GitLab [discord-quest topic search](https://gitlab.com/explore/projects/topics/discord-quest) surfaced one candidate (DiscordBotAPI), **not validated as a standalone Quest solver**. This supersedes an earlier insufficiently precise “no GitLab project” statement.

## Snapshot integrity

Record a URL and retrieval date; a branch file can change at any moment. For high-value claims, capture a commit hash and precise code line in a future review. The README blob SHA is *not* a Git commit SHA and should not be represented as one.

## Important source contradictions

- Go README labels Xbox, PlayStation, and embedded Activity **“Working (untested)”**. Here we treat them as **untested** until credited server state is evidenced.
- Orion explicitly describes unsuccessful simulated-stream completion; several other repositories claim stream support. Both assertions remain recorded.
- Orion claims application-specific activity achievement handling, while QuestCompleter deliberately does not generate such evidence. Neither solves publisher-controlled in-game achievements.
- Mock unit/E2E tests validate project logic but not acceptance by Discord production.
- “Most stars” or “most supported identifiers” is not an independent correctness metric.

## Citation style

Use inline Markdown links to the specific upstream project or official document beside factual claims. Avoid unlabeled link dumps in future evidence notes. For inaccessible sources, write **unverified** rather than inventing content.
