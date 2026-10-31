# Failure modes and contested claims

**Research date:** 2026-11-01. A method **sending** a progress request is not evidence of **credited** Quest progress.

## Streaming support disagreement

- Official requirement: stream the required game from desktop to at least one other user in voice for the designated duration ([Discord FAQ](https://support.discord.com/hc/en-us/articles/22225719947543-Discord-Quests-FAQ)).
- [Go selfbot](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot), [xgg-2](https://github.com/xgg-2/discord-quests), [unconsents](https://github.com/unconsents/auto-quest-completer), and [SubhuJaat07](https://github.com/SubhuJaat07/discord-quest-completor) claim various stream-handling strategies.
- [Orion](https://github.com/nyxxbit/discord-quest-completer) explicitly documents that its simulated-stream approach does **not** obtain the necessary credited heartbeat.
- [saintordevil QuestCompleter](https://github.com/saintordevil/questCompleter) requires a **real voice stream and viewer**.
- Conclusion: **unresolved** without same-date, same-version, independently observed credited results. Do not mark universal support.

## Desktop game presence vs authentic activity

- Discord desktop can track real running games; a fake presence or dummy process is not proof of gameplay.
- Orion notes that a real-game executable fingerprint may differ from a fabricated running-game representation; this is a detection and correctness risk, **not an invitation to evade it**.
- Windows dummy executable tools avoid some Discord JavaScript internals but have their own constraints and do not solve achievements, streaming or consoles.
- Source: [Orion README](https://github.com/nyxxbit/discord-quest-completer), [markterence](https://github.com/markterence/discord-quest-completer).

## No server progress but client progress increased

- Orion documents a previous activity implementation that advanced the local meter even when the server granted nothing. The fix uses server-confirmed progress and exits after repeated missing credits.
- Test criterion: if authoritative progress is unchanged, the result must be **not complete**, regardless of the console display.
- Source: [Orion changelog](https://github.com/nyxxbit/discord-quest-completer).

## Application identifier changes

- Per-task applications and config versions can make hardcoded config.application.id logic incorrect. Orion's changelog documents a routing/application-id issue and its effects on desktop and achievement support.
- Unknown join operators and task schemas should be quarantined rather than guessed.
- Sources: [Orion](https://github.com/nyxxbit/discord-quest-completer), [QuestCompleter](https://github.com/saintordevil/questCompleter), [Quest object fields](https://docs.discord.food/resources/quests).

## Webpack/client internals breakage

- Client minification, module cache exposure and native bridge availability can differ between Stable, PTB, Canary, browser and plugin contexts.
- Orion documents incompatibilities of a console userscript on some Stable builds, and modified behavior with Vencord.
- Source: [Orion](https://github.com/nyxxbit/discord-quest-completer).

## Startup race: no quests yet

- A Quest store may be empty early on Discord startup; auto-start must not equate an unfetched store with all Quests completed. Orion documents this regression and fix.
- Source: [Orion README changelog](https://github.com/nyxxbit/discord-quest-completer).

## Game / activity achievement boundaries

- ACHIEVEMENT_IN_GAME is publisher/game controlled, so desktop progress cannot substitute.
- ACHIEVEMENT_IN_ACTIVITY is application-controlled too; Orion and the Go selfbot describe an **application-specific** path, whereas QuestCompleter rejects generalized achievement spoofing.
- A one-off application path should never be generalized to all games.
- Source: [Discord FAQ](https://support.discord.com/hc/en-us/articles/22225719947543-Discord-Quests-FAQ), [Quest reference](https://docs.discord.food/resources/quests).

## Linked console tracking mistakes

- Seeing PLAY_ON_XBOX or PLAY_ON_PLAYSTATION in a Quest is not evidence of desktop compatibility.
- Linked accounts may be offline, private, not playing an eligible title, or outside permitted platforms. Community documentation lists console start errors and linked-account hints.
- [Go selfbot](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot) calls console methods **untested**; [QuestCompleter](https://github.com/saintordevil/questCompleter) explicitly requires real gameplay.

## Mobile, QR, Cloud Play and publisher authentication

- Mobile Quest availability remains limited/beta; identical progress structure does not guarantee eligibility.
- QR codes may transfer a genuine session, not represent a progress event.
- Experimental Cloud Play needs publisher authentication and a **real cloud game** on the Discord desktop app.
- Official: [Quest FAQ](https://support.discord.com/hc/en-us/articles/22225719947543-Discord-Quests-FAQ), [Cloud Play FAQ](https://support.discord.com/hc/en-us/articles/35372187686295-Discord-Cloud-Play-Quests).

## Claiming failures

- CAPTCHA/verification required, invalid reward platform, stale eligibility, expired promotion, already claimed, missing sealed metadata, or backend error.
- Do not auto-mark claimed based on a button click or claim request.
- Source: [QuestCompleter](https://github.com/saintordevil/questCompleter).

## Transport and reliability

- 401/403 may indicate authorization, platform eligibility or a challenged account; not every 403 is retriable.
- 404 may indicate a missing campaign or incompatible route, rather than a transient error.
- 429 signals rate limiting; uncontrolled retries make reliability worse.
- 5xx and disconnect can cause ambiguity about whether a prior action succeeded; reconcile server state before retrying.
- Restore changed local state on stop or reconnect and prevent stale workers from writing into new sessions.
- These are general engineering principles and are not a claim about every Discord error code.

## Account and platform enforcement

Discord explicitly prohibits simulated Quest completion, and projects report 2026 Quest restrictions. A missing warning today is not proof that the account is safe. See [Security and policy](SECURITY-AND-POLICY.md).

## Severity categories for research reports

- **Critical:** token exposure, credentials sent to untrusted site, or claim/result falsification.
- **High:** incorrect account action, unsupported code path labeled working, false reward confirmation.
- **Medium:** startup race, flaky progress reporting, unreliable campaign classification.
- **Low:** display-only progress / formatting defect.

Always include reproducible, redacted evidence. No live Quest spoofing experiments are performed by this repository.
