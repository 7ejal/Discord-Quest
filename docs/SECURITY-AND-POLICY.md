# Account security, privacy and platform rules

## Discord's explicit rule

Discord's official [Platform Manipulation Policy](https://discord.com/safety/platform-manipulation-policy-explainer) disallows scripts, spoofed applications or other tools that **simulate Quest completion** without performing the required activity, and disallows selfbots and unauthorized client modification. Authors' “educational use only” disclaimers do not grant an exception.

Consequences can include Quest access restrictions, removal of rewards, account standing violations, or suspension; exact enforcement is determined by Discord. [Orion](https://github.com/nyxxbit/discord-quest-completer) documents reported restrictions and uncertainty about their timing; these reports are not controlled studies.

## Personal user tokens are sensitive credentials

Several standalone projects ask for a user token in environment files, text files, cloud CI secrets, web dashboards, or command-line arguments. Anyone possessing a functioning user token may obtain broad account access. This is not comparable to an ordinary Discord bot token with explicit app permissions.

**Never** publish tokens, log their fragments, send them to third-party websites, or paste them into unknown scripts. Treat a token leak as an account compromise and follow Discord's official account-security recovery procedures.

## Risk checklist for external repositories

| Area | Questions |
|---|---|
| Network destinations | Does code call anything other than known vendor endpoints? Is there hidden telemetry, webhooks, analytics or remote loader code? |
| Token custody | Token in local file, server memory, logs, cloud runner, backend database, browser extension storage or hosted service? |
| Dependencies | Pinned dependencies, lockfile, native modules, post-install scripts, transitive execution? |
| Code distribution | Is a downloaded executable reproducibly built from the shown source? |
| Updates | Can upstream replace logic silently through automatic updates or remote downloads? |
| Browser privileges | Extension permissions, clipboard access, website injection scope, cross-origin requests? |
| Client mods | Are credentials or Discord internals readable by another installed plugin? |
| CAPTCHA and OAuth | Are authorization permissions explained and revoked correctly? Is any challenge solved by an untrusted external service? |
| Logging | Can error traces accidentally include authorization headers, cookies, query tokens or reward codes? |
| Shutdown | Are session state and temporary local modifications cleaned up? |

**A public repository is not automatically safe.** Do not run unknown source merely because it is open source.

## High-risk implementation categories

### Multi-account selfbots
Higher impact if token files contain multiple accounts. Concurrent account automation may add policy and rate-limit exposure.

### Server-hosted dashboards and cloud CI
A token entrusted to a remote server is under its operator's control. Even “self-hosted” setups can expose secrets via logs, misconfigured public ports, hosting provider backups or accidental commits.

### Application-specific achievement/OAuth flows
Review requested scopes and third-party endpoints. An authorization grant and an external application's claim of achievement are more sensitive than passive status viewing.

### Browser console scripts / extensions
A pasted script executes with the user's session privileges. A malicious version could access credentials or message data even if it presents itself as a Quest helper.

### CAPTCHA solving
Challenge services may receive sensitive session information. Verification is controlled by Discord and should not be logged, redistributed or silently bypassed.

## Safe handling guidelines for this documentation repo

- **No real access tokens, session cookies, OAuth secrets, full raw headers, reward codes, user IDs or identifying screenshots.**
- Only synthetic example IDs and redacted records.
- Do not publish code for bypassing anti-abuse, CAPTCHA, access restrictions or account protections.
- Prefer immutable source references or at least URLs plus verification dates.
- Compare claims without running prohibited activities on real accounts.
- Preserve authorship, license conditions, and upstream links.
- Use ordinary GitHub documentation; no scheduled selfbot workflows.

## Responsible research and bug bounty

Discord's [Security Bug Bounty page](https://discord.com/security) states that merely manipulating an individual account's Quest completion is **not** a bounty target. It focuses on issues enabling reward abuse at scale. This distinction is not permission to exploit the system.

## References

[Official Platform Manipulation Policy](https://discord.com/safety/platform-manipulation-policy-explainer) · [Official Terms](https://discord.com/terms) · [Security Bug Bounty](https://discord.com/security) · [Orion warnings](https://github.com/nyxxbit/discord-quest-completer) · [Go selfbot security model claims](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot) · [QuestCompleter safeguards](https://github.com/saintordevil/questCompleter).
