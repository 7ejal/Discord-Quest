# Architecture decisions (ADRs) and unresolved assumptions

**Record changes with task IDs and source/CI evidence.** Decisions are current project choices, not implied Discord product contracts.

## ADR-001 — The target is a native offline Quest-state inspector

**State:** Accepted, 2026-11-01. A literal operational user-token selfbot is outside scope. No simulated rewarded activity. Source mapping documents these exclusions as `SCOPE-001` through `SCOPE-004`.

**Rationale:** Distinguishing task status from authentic engagement avoids unsupported completion promises and account-security risks. **Revisit only with a new approved, policy-compliant goal.**

## ADR-002 — Freeze upstream Go source at a commit

**State:** Accepted. Go reference `352d7f457c9511bc9e7c9b940a71012bfb47572e`; don't silently merge changed upstream HEAD into source parity. Re-audit any later upstream revision as separate work.

## ADR-003 — Retain the existing Rust crate as sole active implementation

**State:** Accepted in planning. The active application is `rust/go-quest-port`; `rust/quest-audit-cli` is an incomplete earlier prototype with a library but no CLI binary. **Work item AUD-003 remains TODO until the other crate is explicitly labeled/archived and CI guards correct workspace.**

## ADR-004 — Use scoped threads for offline analysis, not Tokio by default

**State:** Accepted for current code. `std::thread::scope` bounded to 1–32 already passed tests. Tokio/HTTP/WebSockets are optional future *authorized* integrations; they should only be added with a measurable justification.

## ADR-005 — Never collapse local progress and server-confirmed completion

**State:** Accepted. Numeric goal, task completed, Quest completed, reward claimed and partner redemption are distinct. Fix unvalidated timestamp nonempty-string truthiness before adding stronger confirmation claims (STATUS-010).

## ADR-006 — Correct intentional semantic differences from Go

**State:** Accepted. Keep `GetTaskName` as a comparison heuristic only. Evaluate AND/OR correctly. Enrollment-block metadata is shown rather than aborting all offline analysis. Duplicate-ID behavior needs an explicit policy (PARSE-002) and fixture parity.

## ADR-007 — Source data schema remains partly unverified

**State:** Open. Some `task_config_v2` representations may differ from the Go/TypeScript implementations; actual sanitized fixtures are needed (AUD-001). Until then, unknown variants should be unsupported, not inferred to satisfy a Quest.

## ADR-008 — Evidence and CI are commit-scoped

**State:** Accepted. A passing historical run is valid for its `head_sha`, not all future commits. Release-ready requires passing checks on the relevant tagged commit (REL-002/REL-007/OPS-003).

## ADR-009 — Tracker JSON is authoritative; Markdown is generated

**State:** Planned; work item OPS-001. Editing `PROGRESS.md` alone cannot change a task status. The task ID and acceptance criterion remain stable; create new IDs for new requirements.

## ADR-010 — Native release goals

**State:** Accepted Linux-only override, 2026-11-01. Linux x86_64 native CLI only, tested and packaged on the isolated main-server checkout; Windows and GitHub-hosted workflows retired. No Go/Node/Python/Discord installation needed to *run* the compiled binary. Reproducible locked Cargo builds, license, checksum and durable release notes are required before a general release.

## ADR-011 — Linux-only/main-server-only testing

**State:** Accepted 2026-11-01. [LINUX-ONLY.md](LINUX-ONLY.md) supersedes earlier cross-platform scope. No GitHub-hosted actions and no persistent GitHub runner on production. Linux release candidates must include LICENSE/NOTICE, locked dependency inventory, hashes and exact source commit. Published GitHub release assets are a separate decision; a file on main-server is not public.

## ADR-012 — User-requested Go-style live HTTP parity is a separate unimplemented milestone

**State:** Requested, not yet accepted as a supported network mode, 2026-11-01. The user explicitly asked for raw personal-token HTTP `Authorization` behavior like the Go original, account connectivity testing, live Quest retrieval and fuller Go parity, on Linux-only/main-server-only. The [backlog](GO-LIVE-HTTP-REQUEST.md) and new task IDs `AUD-009`, `SEC-005`, `NET-005` through `NET-009`, `QA-011` and `AUD-010` record the request. They are TODO, do not imply approval or working code, and are **not** required for the existing offline release.

**Decision boundary:** Before live network code, verify protocol, present-day access permissions, platform terms and credential handling. The raw header method in the original Go is not equivalent to supported OAuth2 permissions. The original personal-account selfbot, fabricated rewarded engagement, challenge bypass and reward-claiming exclusions remain unchanged unless independently evaluated and explicitly approved. Do not claim full Go parity from a read-only account check.

## Unresolved decisions requiring validation

- How to represent incomplete `excluded_quests` without inventing a complete config (PARSE-003).
- Duplicate-ID last-write-wins parity vs reject inconsistent duplicates (PARSE-002).
- Any alternate `join_operator` representation and backward-compatible scheme (AUD-001).
- Whether to preserve unknown nested fields verbatim or explicitly label information loss (PARSE-004).
- Limits for local JSON size, depth, and Quest count (PARSE-005).
- Whether authorized network integrations are needed at all for an offline release (NET-004 is **optional**, not an implicit plan to add selfbot login).
- Releasing compiled binaries with stable checksums and provenance (REL-002/REL-003).

**AI change policy:** For every changed ADR, record why, what is implemented, relevant test IDs, code commit and tracker update. Proposed is not implemented.

## ADR-013 — Raw personal-user Authorization rejected for this Rust product (2026-11-01)

**State:** Reviewed. [Pinned Go HTTP audit](GO-HTTP-AUTH-AUDIT-2026-11-01.md) confirms bare personal-token header and private Quest routes; official OAuth2 `identify` supports only a distinct user-identity request. Discord's policy prohibits selfbot/client impersonation and fake Quest completion. The scope excludes Go-style raw personal credentials. Live Quest schema access remains unverified under `NET-007`; optional authorized OAuth2 account identity is a separate decision and not implemented. [Complete parity matrix](GO-LIVE-PARITY-DECISION-2026-11-01.md).
