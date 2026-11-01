# Go HTTP Authorization, permission and security audit (2026-11-01)

**Task evidence:** `AUD-009`, `NET-005`, `SEC-005`. Source is the **pinned** upstream Go commit [`352d7f457c9511bc9e7c9b940a71012bfb47572e`](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/tree/352d7f457c9511bc9e7c9b940a71012bfb47572e). This is a **static source and public-permissions review**, not a test of an active Discord account or any private API. No credentials or live requests were used.

## What Go actually sends (source-level evidence)

- [`main.go`](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/main.go) obtains a raw **personal account** `TOKEN` from the environment, connects to a selfbot Gateway session first, then fetches Quests from the READY callback. Go's `godotenv.Load` can also read a local `.env`; that design is **not** adopted here.
- [`constants.go`](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/constants.go) `MakeHeaders` performs `h.Set("Authorization", token)`, i.e. a **bare token**, without `Bearer ` or `Bot ` prefix. It adds locale/timezone, dynamic desktop/Android user-agent, `x-super-properties`, and synthetic client metadata. Those extra headers are not evidence of official API authorization.
- [`client.go`](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/client.go) uses `https://discord.com/api/v9`, `http.DefaultClient`, and optional JSON bodies. It reads the **whole response body** without a cap, prints raw non-2xx response bodies in errors and retries 429 **recursively without a limit**. Its Gateway Identify includes the user token, and READY is not a separate HTTP authentication proof.

| Pinned Go operation | HTTP method and API path | Expected source model / evidence | Permission / shipping disposition |
|---|---|---|---|
| `FetchQuests` | `GET /quests/@me` | Go `AllQuestsResponse`: `quests`, `excluded_quests`, `quest_enrollment_blocked_until` | **Undocumented for third-party OAuth2 in reviewed official docs.** No approved scope or schema verification; no live implementation. |
| `addExcludedQuest` | `GET /quests/{quest_id}` | Go decodes `QuestConfig`, not verified against today's wire shape | Private/undocumented in reviewed official docs; disabled. |
| `acceptQuest` | `POST /quests/{quest_id}/enroll` | Go expects `QuestUserStatus`; sends location and targeting fields | Mutates enrollment; excluded. |
| `doingWatchVideoQuest` | `POST /quests/{quest_id}/video-progress` | Go supplies synthetic `timestamp`; only reads optional `completed_at` | Simulates rewarded engagement; excluded. |
| Gameplay/activity/stream handlers | `POST /quests/{quest_id}/heartbeat` | Go sends activity/game identifiers and `terminal`; state returned may not prove authentic engagement | Synthetic Quest progression; excluded. |
| `RedeemQuest` | Quest reward-claim request (see pinned `manager.go`) | A separate claim/challenge flow, **commented out in the default `main.go` path** | Claim automation and challenge solving; excluded. |
| Gateway session | `wss://gateway.discord.gg/?encoding=json&v=9` (not HTTP REST) | User-token Identify; READY / heartbeat / presence | Selfbot/client impersonation; excluded. |
| Legitimate app identity check | Documented **Get Current User** `GET /users/@me` | Official User object; requires OAuth2 `identify` scope | **Potentially supportable via an authorized OAuth2 Bearer access token**, not via the Go raw personal user token. Not implemented. |

**No literal HTTP request/response examples in the source prove current availability, eligibility, or Quest reward credit.** Endpoint/path appearances above are source observations, not instructions to call them.

## Current public authorization and API boundary

- [Official Discord OAuth2](https://docs.discord.com/developers/topics/oauth2): `identify` permits `/users/@me` through application-approved OAuth2 access; scopes are enumerated. The published scope list reviewed **does not expose a general user-Quest-read scope**. This absence is **not proof** that no private internal Quest endpoint exists; it means the project lacks a **documented supported grant** for one.
- [Official User resource](https://docs.discord.com/developers/resources/user): `Get Current User` allows an authorized OAuth2 `identify` app to read the account User object, with optional `email` scope for email. It is **not** a Quest endpoint.
- [Official Discord Platform Manipulation Policy](https://discord.com/safety/platform-manipulation-policy-explainer): prohibits selfbots/client manipulation and use of scripts/spoofed apps to simulate Quest completion. **Possession of a personal token does not grant developer API approval.**
- Accordingly, the raw-header personal-account workflow from Go is **not approved for implementation in this repository**. `NET-006` and `NET-008` remain conditional and blocked; do not silently replace the requested header semantics and claim Go parity. A distinct consented **OAuth2 `Bearer`** app identity check could be considered as a separately scoped integration.

## Safe credential-handling design (`SEC-005`, design only)

1. **Trust boundary:** Only a user-consented, scoped OAuth2 access token from a registered Discord app could be accepted in an explicitly enabled future command; reject raw personal-session tokens, bot tokens in user paths, and malformed/blank input. No credential is accepted by the current binary.
2. **Input and lifetime:** Avoid command-line flags/arguments, persistent files, `.env`, shell history, telemetry, and debug output. For a future opt-in flow, use a protected prompt or an established OS credential facility, retain only as long as the one request needs it, and redact diagnostics. On Unix, core dumps, swap, process memory and elevated debuggers are residual risks that cannot be eliminated by ordinary string handling.
3. **HTTP isolation:** HTTPS to a pinned official origin, explicit small time/body limits, **no redirects**, **no proxies inherited from environment**, no uncontrolled retries, no outbound requests in CI. Enforce an allowlist of methods and paths; never pass a credential to third-party hosts or to webhook helpers.
4. **Response and error handling:** Distinguish **200** from **401** unauthenticated, **403** denied, **429** rate-limited and 5xx/transient errors. Fail closed on redirects, malformed JSON, excessive bodies and timeouts. Never log raw error bodies, user IDs, tokens, URLs with query secrets, or any header value.
5. **Testing:** Use synthetic Bearer tokens and an in-memory/mock transport with cases for 200/401/403/429/3xx/5xx, timeout, cancellation, body cap and no leakage. These are **expected HTTP handling contracts, not observations of Discord responses**. Do not run real personal-account token tests.

## Decision and evidence limits

- `AUD-009`: **Static upstream endpoint/header/protocol audit complete** with direct source-file and official-document references; live authorization never assumed.
- `NET-005`: **Permission/design decision complete:** raw personal user-token semantics are unapproved; the only identified supported alternative is scoped OAuth2 `identify` for **User identity**, not Quests. No active login was tested or implemented.
- `SEC-005`: **Threat model/design complete**; design is not implementation, security certification, or approval to store/send credentials.
- `NET-007`: No supported Quest-reading OAuth2 scope or authorized sanitized modern task-group payload was established by this audit. This is a blocker for live Quest ingestion and `AUD-001` / `STATE-005`, not evidence those tasks passed.

Never infer completed, claimed, or redeemed rewards from a local JSON value or a successful HTTP status code.
