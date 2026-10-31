# Standalone Quest selfbots: complete source walkthrough

**Reviewed:** 2026-11-01. **Goal:** Explain what the **actual source code** of Windows-compatible CLI/executable Discord Quest projects does, why it works *as software*, where it fails, and what is or isn't actually confirmed. **This is a static source-code study, not an execution manual or a guarantee of Quest success.** Discord explicitly prohibits scripts that simulate Quest engagement to obtain rewards ([official policy](https://discord.com/safety/platform-manipulation-policy-explainer)).

## New: method-level deep dive and reproducible evaluation (2026-11-01)\n\n- **[How every method works and which engineering approach is best](METHODS-DEEP-DIVE-2026-11-01.md)** — source-pinned REST/Gateway, task-specific methods, streaming/activity/achievement limitations, retries, validation and updated findings.\n- **[Offline method verification fixtures](METHOD-VALIDATION-FIXTURES.md)** — 22 synthetic scenarios for trustworthy results without Discord credentials or a real Quest.\n\n**Updated correction:** SUDIN010's game and video handlers can infer success from numeric progress even without a fresh top-level Quest completion marker; its Gateway presence also remains tied to the first selected campaign. These are analyzed with exact code links.\n\n## Projects in scope

| Project | Runtime | Entry point | Actual external software dependencies | Detailed walkthrough |
|---|---|---|---|---|
| [yeweroooo](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot) | Go static executable | `main.go` | Go 1.23 for compiling; resulting binary requires no Node/Python/Discord desktop | [Go](GO.md) |
| [aiko-chan-ai](https://github.com/aiko-chan-ai/Discord-Quest-Auto-Completion-Selfbot) | TypeScript/Node CLI | `bot.ts` | Node **24+**, npm/tsx, Discord.js core/REST/WS | [aiko TypeScript](AIKO-TYPESCRIPT.md) |
| [SUDIN010](https://github.com/SUDIN010/discord-auto-quest) | Python CLI | `main.py` | Python, requests, websockets, python-dotenv | [SUDIN Python](SUDIN-PYTHON.md) |
| [unconsents](https://github.com/unconsents/auto-quest-completer) | Python multi-account CLI | `main.py` | Python, discord.py-self, aiohttp, colorama | [unconsents Python](UNCONSENTS-PYTHON.md) |
| [lfathh](https://github.com/lfathh/Auto-Quest-Discord) | TypeScript/Node CLI | `bot.ts` | Node **18+** per manifest, npm/tsx, Discord.js core/REST/WS | [lfathh TypeScript](LFATHH-TYPESCRIPT.md) |

**Additional without Discord installed:** [xgg-2 Node SDK](https://github.com/xgg-2/discord-quests) and [SubhuJaat07 Next.js service](https://github.com/SubhuJaat07/discord-quest-completor). These are separate categories (a library and a hosted service) covered by [additional architecture notes](EXTRA-NODE-AND-HOSTED.md). They are **not** standalone interactive local CLI applications in the same sense as the above five.

### What does “without Discord installed” mean?

All five run an independent programming-language process and communicate with Discord's **network service**. There is no requirement to keep the graphical Discord app open for the *program itself to start*. However, some Quests require a real desktop game detected by the official Discord app, an actual voice stream with a viewer, a linked console game, or an external achievement. Running code without the graphical client is therefore not equivalent to legitimately satisfying every Quest.

A `.bat` file is only a Windows **launcher**. It starts another executable/program and cannot add missing support for a Quest type. The Go project can compile to a single native executable; Node and Python projects ordinarily depend on their installed runtimes.

## Architecture in plain English

```text
Program starts
  |
  +--> loads config and personal-account authentication
  |
  +--> network client: Discord REST and optionally Gateway WebSocket
  |      |
  |      +--> REST: list Quests, read enrollment and server progress
  |      +--> Gateway: connection/ready event, optional user presence
  |
  +--> Quest manager: decides which campaigns/tasks to consider
  |      |
  |      +--> task type / goal / current progress
  |      +--> eligibility, expiry, already completed, AND/OR groups
  |
  +--> task handler (methods differ, server acceptance not guaranteed)
  |
  +--> result: task progress -> Quest completed -> reward claimed
         THESE ARE THREE DIFFERENT THINGS
```

### Core terms explained

**REST API:** Requests to Discord's HTTP service. A program can ask for Quest JSON and receive progress records. The reviewed code accesses **undocumented user endpoints**, not a supported public Discord bot API.

**Gateway / WebSocket:** A persistent two-way connection. The initial READY event identifies an active session; periodic Gateway heartbeats maintain it. **Gateway heartbeat is NOT the Quest-task heartbeat**; they serve different purposes.

**Quest heartbeat:** A progress-reporting action for some task families. Being able to send a request does not mean it will be credited as valid gameplay/streaming.

**Task config:** Per-campaign configuration describing task IDs, target values, applications and `join_operator`. `AND` requires every required task; `OR` needs a valid alternative. Several reviewed programs pick one task from a list and do not implement the combination logic.

**Progress vs completion:** A local timer, accepted HTTP request, task progress value, Quest-level `completed_at`, reward `claimed_at`, and actual redemption are different evidence levels. This distinction explains most false-success bugs.

**OAuth grant:** Authorization given to an application. Specific activity-achievement implementations manipulate an external application's grant and may fail to revoke it after an early error; such behavior adds substantial account security risk.

**CAPTCHA:** Discord account verification; third-party solving providers increase privacy and policy risk and do not establish Quest eligibility.

## Source-pinned review scope

| Project | Exact commit analyzed |
|---|---|
| Go yeweroooo | [352d7f457c9511bc9e7c9b940a71012bfb47572e](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/tree/352d7f457c9511bc9e7c9b940a71012bfb47572e) |
| TypeScript aiko | [4dd4e6b535cc799708f60cfcbff0ecf33eb504ee](https://github.com/aiko-chan-ai/Discord-Quest-Auto-Completion-Selfbot/tree/4dd4e6b535cc799708f60cfcbff0ecf33eb504ee) |
| Python SUDIN | [c14afa1dbb3c8c59d7f68c90caa305eff8e15b0e](https://github.com/SUDIN010/discord-auto-quest/tree/c14afa1dbb3c8c59d7f68c90caa305eff8e15b0e) |
| Python unconsents | [6d3b9a085dc78ce42d85dd9748f1636eff111a5a](https://github.com/unconsents/auto-quest-completer/tree/6d3b9a085dc78ce42d85dd9748f1636eff111a5a) |
| TypeScript lfathh | [5fe8bd958571fea5e953d23a211d3472cea3ebce](https://github.com/lfathh/Auto-Quest-Discord/tree/5fe8bd958571fea5e953d23a211d3472cea3ebce) |

## What each codebase truly implements

Legend: **CODE** = source path exists only; **NO** = explicitly not implemented; **UNTESTED** = code or README explicitly lacks validation; **DEPRECATED** = old task; **EXTERNAL** = requires activity/game-provider proof. No cells mean independently verified success.

| Task | Go yew | TS aiko | Python SUDIN | Python unconsents | TS lfathh |
|---|---|---|---|---|---|
| WATCH_VIDEO | CODE | CODE | CODE | CODE | CODE |
| WATCH_VIDEO_ON_MOBILE | CODE | CODE | CODE | CODE | CODE |
| PLAY_ON_DESKTOP | CODE | CODE | CODE | CODE | CODE |
| PLAY_ON_DESKTOP_V2 | DEPRECATED handler | NO | NO | DEPRECATED handler | NO |
| STREAM_ON_DESKTOP | CODE, unverified | NO | NO | CODE, unverified | NO |
| PLAY_ACTIVITY | UNTESTED, fixed session key | UNTESTED, fixed session key | NO | CODE, unverified | NO |
| ACHIEVEMENT_IN_ACTIVITY | App-specific CODE | App-specific CODE | NO | NO | NO |
| ACHIEVEMENT_IN_GAME | EXTERNAL/NO | EXTERNAL/NO | EXTERNAL/NO | EXTERNAL/NO | EXTERNAL/NO |
| PLAY_ON_XBOX | UNTESTED | UNTESTED | NO | NO | NO |
| PLAY_ON_PLAYSTATION | UNTESTED | UNTESTED | NO | NO | NO |

**Cloud Play, QR and Missions are additional experiences; do not automatically equate them to a normal named Quest task.** Reference: [official FAQ](https://support.discord.com/hc/en-us/articles/22225719947543-Discord-Quests-FAQ) and [unofficial task schema](https://docs.discord.food/resources/quests).

## Highest-confidence conclusions

- **Best standalone for narrowly scoped and understandable control flow:** SUDIN010 Python. Its HTTP retries are bounded and error cases distinguished; it supports fewer categories and still has a potential endless gameplay-progress loop.
- **Broadest *advertised* standalone task list:** Go yeweroooo. Its READMEs mark some entries “working (untested)”; implementation shortcuts and false-success reports make that label unreliable.
- **Most reusable TypeScript service/network framework:** aiko-chan-ai; it uses discord.js libraries but deliberately does not implement streaming and has the same task-selection issues as the Go rewrite.
- **Least convincing advertised breadth:** lfathh's TypeScript standalone handler has separate no-op explanation branches for streaming/activity, and still uses the older task config object.
- **Multi-account rather than reliability focus:** unconsents; plaintext credential files, simplistic HTTP handling, false-success displays and indefinite heartbeat loops add risk.

**Do not rank by lines of code, feature labels or raw completion speed.** A program that prints “Done” despite failure is not more capable.

## Security and validation findings

- All five require a **personal Discord user token**, not an official bot token; credential compromise can affect the account.
- Go and aiko have an optional third-party CAPTCHA solver and a risky application-specific authorization flow. Their standard startup paths **do not automatically claim rewards** even though claim functions exist.
- Unconsents stores one or multiple tokens as plaintext in `tokens.txt`.
- Go uses `http.DefaultClient` (no configured overall timeout) and recursive uncapped 429 retries. Aiko uses discord.js REST queuing instead; SUDIN has a bounded retry loop.
- Most task handlers do **not** perform final Quest-level refetch and confirmation.
- None was independently measured for RAM/CPU, malware presence, or real production completion.
- No sample bot or production Quest was run as part of this study; no tests were executed.

## Recommended reading order

1. [Understand endpoints and status](NETWORK-AND-STATUS.md) before reading code.
2. [SUDIN-PYTHON.md](SUDIN-PYTHON.md) for the simplest split of REST client, UI and Gateway.
3. [AIKO-TYPESCRIPT.md](AIKO-TYPESCRIPT.md) for explicit class layers and task handlers.
4. [GO.md](GO.md) to compare the rewrite line-by-line with aiko.
5. [UNCONSENTS-PYTHON.md](UNCONSENTS-PYTHON.md) for concurrency/credential tradeoffs.
6. [LFATHH-TYPESCRIPT.md](LFATHH-TYPESCRIPT.md) for older client/schema assumptions.
7. [CROSS-PROJECT-FAILURES.md](CROSS-PROJECT-FAILURES.md) for a prioritized research-quality checklist.

**Do not use this repository as a how-to for simulating rewarded engagement or evading account controls.** Its purpose is source understanding and evidence-based reliability analysis.
