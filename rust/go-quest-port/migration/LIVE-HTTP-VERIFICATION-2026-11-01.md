# Genuine read-only Discord HTTPS verification — November 1, 2026

**Scope:** Public Quest configuration by a known numeric ID, and a separate OAuth2
`identify` identity-check command. Neither sends Quest enrollment, task
heartbeats, simulated video/game/activity/stream progress, OAuth2 account grants
on behalf of another person, rewards, or claims.

## Real observed HTTPS test (NOT mocked)

On the user's isolated Linux `main-server` checkout, the Rust binary
`go-quest-port quest-config 1552869482471358576 --json` sent an HTTPS GET to
`https://discord.com/api/v10/quests/1552869482471358576`, without an account token.
The response was successfully parsed and reported:

- public Quest ID: `1552869482471358576`
- title: `World of Warcraft: Forever Video`
- starts at: `2026-10-28T17:00:00+00:00`
- expires at: `2026-11-04T01:00:00+00:00`
- task identifiers: `WATCH_VIDEO`, `WATCH_VIDEO_ON_MOBILE`
- labeled provenance: `unverified_remote_public_config`

A separate HTTPS call for `1193992107035983872` also returned a successfully
parsed Quest configuration. This is evidence of actual network I/O and a
real response **on this date**, not an eligibility guarantee, actual enrollment,
user task progress, or server-confirmed reward claim. These fields are
untrusted service data and not independently verified Discord account state.
The public config endpoint appears in an [unofficial Quest API
reference](https://docs.discord.food/resources/quests), not the official
developer OAuth2 API contract. No ongoing uptime or schema guarantee exists.

## Live transport, security boundaries, and validation

- `src/live.rs`: HTTPS client fixed to `discord.com` and API `v10`; public
  config GET path accepts numeric ID (1-20 digits) only, without credentials.
  The OAuth2 check makes exactly one `GET /users/@me` with an official
  user-consented **Bearer** token from piped stdin, never from argv or a file.
- Disabled automatic proxy selection and redirects; 4-second connect and
  10-second total timeouts; 1 MiB response cap; 429/5xx fail without retry.
  Error output is typed and never includes the remote response body or token.
- No real OAuth2 access token was provided or exercised. `oauth-check` is
  **implemented but not independently authenticated against a live account**.
  A raw Discord personal-session token is not supported.
- Tests are local/synthetic for parser validation, bad IDs, missing token,
  invalid responses and keeping remote user status out of public summary.
  Main-server test script also runs network-isolated checks for offline mode.

## Why real Quest completion still is not provided

Discord's [official Quest FAQ](https://support.discord.com/hc/en-us/articles/22225719947543-Discord-Quests-FAQ)
requires real viewing, gaming or streaming depending on task and states that
Linux cannot currently earn Quest rewards. The
[official developer OAuth2 reference](https://docs.discord.com/developers/topics/oauth2)
does not list a third-party OAuth2 grant to read a personal Quest list.
The [platform manipulation
policy](https://discord.com/safety/platform-manipulation-policy-explainer)
prohibits using scripts/spoofed applications to simulate Quest completion for
rewards. Therefore no claim of real per-user progress or all-game reward
completion can be made from this code.

The existing `NET-006`, `NET-007`, `NET-008`, `NET-009`, `QA-011`
feature requests remain separate from the completed **public-config** flow.
Their exact tracker statuses are in `progress.json`.
