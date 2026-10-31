# Discord Quest Research

**Status:** Evidence-based technical research; **not** a verified universal Quest completer.  
**Research snapshot:** 2026-11-01 (UTC+04:00)  
**Repository:** [7ejal/Discord-Quest](https://github.com/7ejal/Discord-Quest)

This repository documents Discord Quest task types, observable progress models, methods claimed by open-source projects, implementation differences, failures, account risks, and outstanding research questions. **The existence of code is not evidence that Discord credited a Quest.**

> **Policy notice:** [Discord explicitly prohibits scripts and spoofed applications used to simulate Quest completion](https://discord.com/safety/platform-manipulation-policy-explainer). It also prohibits selfbots and unauthorized client modification. Do not put a real Discord user token into untrusted software. The documents here do not represent approval or independent live validation of any automation method.

## Source-code audit and recommendation

**[Read the source-backed recommendation and project ranking](research/RECOMMENDATION.md)**. Nine projects received detailed audits and six more received targeted code inspections at pinned Git commit revisions: see **[audit index](audits/README.md)** and **[secondary inspections](audits/SECONDARY-IMPLEMENTATIONS.md)** and individual findings for Go, TypeScript, Python, Vencord, userscripts, and the hosted Next.js implementation. These are static source reviews, not live Quest completion tests or malware certifications. Main findings: breadth claims are frequently untested; several projects incorrectly report success without final server confirmation; some custody models expose personal user tokens.\n\n## Start here

| Document | Purpose |
|---|---|
| [Code audit and ranking](research/RECOMMENDATION.md) | Source-backed ranking, confirmed risks and decision criteria |\n| [Nine detailed source audits](audits/README.md) | Version-pinned source files and line references for each implementation |\n| [Quest types](docs/QUEST-TYPES.md) | All 10 documented historical/current task identifiers, including external achievements |
| [Compatibility matrix](docs/COMPATIBILITY-MATRIX.md) | Project-by-task claims, with *untested* and *conditional* separated |
| [Methods](docs/METHODS.md) | How the documented strategies differ, at a conceptual level |
| [Project comparison](docs/PROJECT-COMPARISON.md) | Selfbots, frameworks, userscripts, plugins, extensions, and desktop utilities |
| [Quest schema](docs/SCHEMA-AND-VALIDATION.md) | Config versions, AND/OR task groups, fields, states, and evidence standards |
| [Reward lifecycle](docs/REWARDS-AND-LIFECYCLE.md) | Enrollment, progress, completion, claim, and redemption are distinct |
| [Failure modes](docs/FAILURE-MODES.md) | Streaming, desktop, activity, achievement, limits, and update breakage |
| [Security and rules](docs/SECURITY-AND-POLICY.md) | Token threats, third parties, CAPTCHA, account enforcement |
| [Research methodology](docs/RESEARCH-METHODOLOGY.md) | Reproducibility, verification labels, and test plan |
| [Sources](research/SOURCES.md) | Official and community references with provenance |
| [Evidence log](research/EVIDENCE-LOG.md) | Claims vs observations vs independent verification |
| [Open questions](research/OPEN-QUESTIONS.md) | What is unknown and how to resolve it without exposing accounts |
| [Machine-readable task catalog](data/task-types.json) | Task names, semantics, and research classifications |

## High-level findings

1. **There is no independently verified tool that completes every Quest type.** The [Go selfbot](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot) advertises nine task identifiers, but marks several **untested**, and its `PLAY_ON_DESKTOP_V2` support targets a deprecated identifier.
2. **Official Quest categories and internal task identifiers differ.** Discord's public FAQ describes gameplay, streaming, videos, and publisher-reported actions; the [unofficial Quest resource reference](https://docs.discord.food/resources/quests) lists 10 identifiers, one struck through as deprecated.
3. **Streaming is especially inconsistent.** [Orion](https://github.com/nyxxbit/discord-quest-completer) documents failure without a real stream and viewer even though other projects claim heartbeat-only support.
4. **Game achievements are publisher-controlled.** `ACHIEVEMENT_IN_GAME` is not equivalent to desktop presence or a video-progress event. Some `ACHIEVEMENT_IN_ACTIVITY` implementations claim an application-specific path; that is not a general Quest completion contract.
5. **Linked console and Cloud Play quests may require real activity.** [QuestCompleter](https://github.com/saintordevil/questCompleter) describes Xbox/PlayStation tracking, **not** gameplay replacement. [Cloud Play](https://support.discord.com/hc/en-us/articles/35372187686295-Discord-Cloud-Play-Quests) is a separate experimental desktop-only completion experience.
6. **A claimed client completion is not the same as a server-confirmed completion or claim.** A progress indicator, local timer, or HTTP success response can be insufficient to establish reward eligibility.

## Evidence terminology

- **Official:** explicitly stated in Discord's public support/policy documentation.
- **Unofficial reference:** described by a community-maintained protocol reference.
- **Claimed / implemented:** a repository README or source code describes a path; *not* independently tested.
- **Maintainer-tested:** an author reports a successful test, with limitations and date where available.
- **Independent verification:** reproducible third-party evidence; **none established by this research snapshot**.
- **Untested:** a developer labels it untested, or verification evidence is absent.
- **Conditional:** requires an authentic game, stream, linked account, third-party backend, or eligibility.
- **Unsupported / deprecated:** an explicitly unsupported path or legacy identifier.

## Scope and limitations

Research reviewed official documentation, community API documentation, and public repository READMEs/source references. No personal user-token login, live Quest simulation, selfbot execution, CAPTCHA solving, or reward claiming was performed. Every comparison is a **dated snapshot**, not a guarantee of current compatibility.

**Primary sources:** [Discord Quest FAQ](https://support.discord.com/hc/en-us/articles/22225719947543-Discord-Quests-FAQ) · [Discord policy](https://discord.com/safety/platform-manipulation-policy-explainer) · [Cloud Play FAQ](https://support.discord.com/hc/en-us/articles/35372187686295-Discord-Cloud-Play-Quests) · [Unofficial Quest structures](https://docs.discord.food/resources/quests).

See [CONTRIBUTING.md](CONTRIBUTING.md) for how to report new findings with evidence.
