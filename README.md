# Discord Quest Research

**Status:** Evidence-based technical research; **not** a verified universal Quest completer.  
**Research snapshot:** 2026-11-01 (UTC+04:00)  
**Repository:** [7ejal/Discord-Quest](https://github.com/7ejal/Discord-Quest)

This repository documents Discord Quest task types, observable progress models, methods claimed by open-source projects, implementation differences, failures, account risks, and outstanding research questions. **The existence of code is not evidence that Discord credited a Quest.**

> **Policy notice:** [Discord explicitly prohibits scripts and spoofed applications used to simulate Quest completion](https://discord.com/safety/platform-manipulation-policy-explainer). It also prohibits selfbots and unauthorized client modification. Do not put a real Discord user token into untrusted software. The documents here do not represent approval or independent live validation of any automation method.

## Source-code audit and recommendation

**[Read the source-backed recommendation and project ranking](research/RECOMMENDATION.md)**. Nine projects received detailed audits and six more received targeted code inspections at pinned Git commit revisions: see **[audit index](audits/README.md)** and **[secondary inspections](audits/SECONDARY-IMPLEMENTATIONS.md)** and individual findings for Go, TypeScript, Python, Vencord, userscripts, and the hosted Next.js implementation. These are static source reviews, not live Quest completion tests or malware certifications. Main findings: breadth claims are frequently untested; several projects incorrectly report success without final server confirmation; some custody models expose personal user tokens.

## Standalone .exe / CLI / .bat source research (November 2026)

**[Read the full standalone code walkthrough index](research/standalone/README.md).** This detailed series covers **Go yeweroooo**, **TypeScript aiko-chan-ai**, **Python SUDIN010**, **Python unconsents**, and **TypeScript lfathh**, with supplementary [Node SDK and hosted server](research/standalone/EXTRA-NODE-AND-HOSTED.md) analysis. It explains startup, exact file responsibilities, HTTP vs Gateway connections, task selection, progress state, rewards, concurrency, dependencies, and code-level failure examples with links to pinned Git commits.

- [What REST/Gateway/task heartbeats mean](research/standalone/NETWORK-AND-STATUS.md)
- [Detailed project-by-project source guides](research/standalone/README.md)
- [Common implementation defects and ranking](research/standalone/CROSS-PROJECT-FAILURES.md)

**Important:** Standalone processes can start without the Discord desktop app, but that does not prove they satisfy real-game, streaming, publisher-achievement or console Quest requirements. No live tasks were run, and inauthentic Quest completion violates Discord's policy.

**[New: deeper method-by-method research (November 2026)](research/standalone/METHODS-DEEP-DIVE-2026-11-01.md)** compares HTTP vs Gateway, video/game/activity/stream/achievement tracking, exact source-level errors, and which *engineering method* is strongest for a standalone CLI. Includes [22 offline test fixtures](research/standalone/METHOD-VALIDATION-FIXTURES.md) and corrects an earlier overstatement of SUDIN completion verification.

## Start for the next AI agent

**[START-HERE — exact next tasks, repository workflow and evidence rules](rust/go-quest-port/migration/START-HERE.md)**. Follow the canonical [machine task tracker](rust/go-quest-port/migration/progress.json), [auto-generated progress board](rust/go-quest-port/migration/PROGRESS.md), [new source-level gap audit](rust/go-quest-port/migration/GAP-AUDIT.md), [38-case test matrix](rust/go-quest-port/migration/TEST-MATRIX.md), and [release gates](rust/go-quest-port/migration/RELEASE-GATES.md). The scope is a verified **offline** native Rust analyzer, not an operational Discord selfbot.

## AI-tracked Go-to-Rust migration

The Rust migration has a **[live progress checklist](rust/go-quest-port/migration/PROGRESS.md)** and **[machine-readable task tracker](rust/go-quest-port/migration/progress.json)** so coding agents can see exactly what is **done / partial / todo / intentionally excluded**. Task IDs include acceptance criteria, dependencies and evidence; these are not percentages or a claim of full operational selfbot parity.

- [Complete migration specification](rust/go-quest-port/migration/SPEC.md)
- [Go-to-Rust map for all 16 original entries and function clusters](rust/go-quest-port/migration/GO-FILE-MAP.md)
- [AI instructions and update rules](AGENTS.md)
- [Agent development workflow](rust/go-quest-port/migration/AI-WORKFLOW.md)

GitHub CI now checks that the canonical tracker and Markdown progress table stay synchronized. The project remains an **offline native status analyzer**, not a reward-spoofing selfbot.

## Go-to-Rust port (native Windows CLI)

The [Rust Go-project port](rust/go-quest-port/README.md) implements local Quest JSON parsing, Go-style Quest filtering, typed data models, AND/OR task-group checks, bounded concurrent analysis and **15 passing unit tests**. **[Windows EXE download](https://github.com/7ejal/Discord-Quest/actions/runs/41256646888/artifacts/13063362600)** — verified successful [Linux and Windows CI run](https://github.com/7ejal/Discord-Quest/actions/runs/41256646888).

**Scope:** This is an offline analysis CLI, not a functional Discord selfbot or simulated reward-completion tool. It requires no Discord installation or personal Discord token.

## Rust research prototype

A [read-only Rust Quest status analyzer library](rust/quest-audit-cli/README.md) has been started. It contains typed Quest JSON parsing, task-group (AND/OR) evaluation, separate Quest/reward states, and 14 synthetic unit tests. **It is not yet a runnable executable; Rust compilation and tests are not verified.** It does not submit progress or simulate rewarded activity.

## Start here

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
