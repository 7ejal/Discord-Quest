# Requested Go-style HTTP parity — backlog, not implemented

**Requested:** 2026-11-01. **Platform:** Linux x86_64 only, CLI only, tested exclusively in the isolated main-server checkout. This document records the user request; it is not evidence that live Discord API access or Go behavioral parity works.

## Requested behavior

The user wants the Rust CLI to accept an account credential locally and use the same HTTP `Authorization` header approach as the original Go program, not to substitute an official OAuth2 flow. They want to test an account and inspect actual Quest state, rather than relying solely on local JSON.

**Current implementation update (2026-11-01):** `demo` and `inspect` remain offline. A new opt-in read-only HTTPS `quest-config` command fetched public Quest config directly from Discord and `oauth-check` implements an official OAuth2 `identify` request (not authenticated live with any user token). There is still no raw personal-user token login, live personal Quest list, Gateway session, or reward-completion automation. See [live verification](LIVE-HTTP-VERIFICATION-2026-11-01.md). The earlier incomplete account module was reverted in Git commit `3170899f96631a0357684656d838653af1c12248`.

## Separate milestones to verify

| Milestone | Tasks | Acceptance |
|---|---|---|
| Protocol and permission research | `NET-005`, `AUD-009` | Compare pinned Go HTTP calls with supported service documentation and terms; decide what may actually ship before adding personal-account access |
| Credential safety | `SEC-005` | No token argument, repo file, logs, shell history or crash diagnostics; explicit one-shot operation; no credential persistence |
| Isolated read-only account HTTP check | `NET-006` | One fixed HTTPS GET with the specifically requested header behavior, only if approved after protocol/terms review; strict timeout, redirect and proxy policies; minimized response |
| Read-only Quest ingestion | `NET-007`, `NET-008` | Endpoint/permission evidence, bounded/sanitized responses, trustworthy provenance, no false server confirmation |
| Mock-based validation and Linux performance | `QA-011`, `NET-009` | Tests use local mock responses, no live personal credentials or outbound requests in CI; regression, timeout and rate-limit tests; CPU/RAM measurements |
| Full original-Go feature parity | `AUD-010` | Record every upstream behavior in a contract matrix, explicitly mark unsupported or excluded functionality rather than claiming a complete port |

## Platform and trust constraints

- Personal-account automation and selfbots can violate Discord's rules and expose accounts to enforcement. A raw `Authorization` header is **not** the same as an officially authorized OAuth2 grant.
- Do not collect credentials in chat or GitHub, hardcode them in Rust, commit `.env` files, or expose them in logs. Any local proof must use placeholders or a localhost mock.
- Keep rewards, game/video/stream progress fabrication, challenge bypass and auto-claim functions outside the supported read-only CLI. The existing `SCOPE-001` through `SCOPE-004` exclusions are unchanged.
- The current offline Linux release gates remain separate. A request for live HTTP parity does **not** retroactively turn offline successes into authenticated-network validation, nor silently mark requested capabilities as working.
- If the desired live Quest endpoint cannot be verified as legitimately accessible, document the blocker and continue offline fixture testing rather than guessing private API structures.

## Execution workflow

Inspect the exact Go reference at commit `352d7f457c9511bc9e7c9b940a71012bfb47572e`. Start by validating design choices and writing mock-based tests, then implement approved code. Commit to GitHub, fast-forward pull to `/root/quest-rust-ci-20261101`, run `scripts/test-linux.sh` with low resource limits, and update task evidence on the tested immutable commit.

**Status update (2026-11-01):** Protocol/security/feature audits `AUD-009`, `SEC-005`, `NET-005`, `AUD-010` are documented in [HTTP audit](GO-HTTP-AUTH-AUDIT-2026-11-01.md) and [feature parity matrix](GO-LIVE-PARITY-DECISION-2026-11-01.md). Current private Quest access remains unsupported and `NET-007` is partial/blocked. **Live public Quest config retrieval is implemented and verified, while real OAuth2 identity account verification and current-user Quest status are unverified.**
