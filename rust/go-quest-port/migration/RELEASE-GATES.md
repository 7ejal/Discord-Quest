# Native Rust offline inspector — release criteria and stop conditions

**This is not a release checklist for a reward-completion selfbot.** Release eligibility is evaluated against the approved **offline inspection** scope. All `excluded` tasks remain out of scope and never turn into release blockers.

**Platform override:** Linux x86_64 only; [main-server-only verification](LINUX-ONLY.md). Windows packaging, Windows compatibility testing and GitHub-hosted runners are explicitly out of scope.

## Gate 0 — Data safety and truthful behavior (must)

- Reject blank/malformed Quest objects, duplicate ambiguities, invalid task numeric ranges and excessive input (`PARSE-001`, `PARSE-002`, `PARSE-005`, `PARSE-006`).
- Validate RFC3339 timestamps and contradictory states; unknown expiry is not silently treated as future (`STATUS-010` through `STATUS-013`).
- AND/OR task groups must not fabricate completion; unsupported task types and unknown operator must be labeled (`STATUS-015`, `MODEL-005`).
- No real user-account token authentication, disguised engagement, CAPTCHA bypass, or reward claiming.
- Tests cover all 10 task identifiers as **classification**, not real completion.

## Gate 1 — Complete supported parser and status model (must)

- Every Go source wire model covered or deliberately documented as excluded (`MODEL-002`, `MODEL-003`, `PARSE-010`).
- Preserve and accurately classify excluded Quest metadata; do not silently lose it (`PARSE-003`).
- Explicit evidence for task config and legacy schema variants (`AUD-001`, `PARSE-009`).
- Support claim/reward distinctions; output stable schema_version and source provenance (`STATE-007`, `STATUS-018`, `UX-002`).
- Deterministic regression fixtures and tests (`TEST-002`, `TEST-003`, `QA-003`).

## Gate 2 — Quality and security (must)

- `cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test --all-targets`.
- Dependency/lockfile policies, minimal CI permissions and security review (`ENV-005`, `ENV-006`, `REL-004`).
- No unbounded time, memory or retries; mock transport tests only if such a layer exists.
- Sensitive input/redaction tests (`SEC-002`, `UX-006`, `QA-005`).
- Tracker JSON/Markdown and Go source mapping remain consistent (`OPS-001`, `OPS-002`, `AUD-002`).

## Gate 3 — Native packaging (must)

- Build and smoke-test the **Linux x86_64 executable only** on `main-server`, from the exact source commit recorded in the archive.
- Locked Cargo build from clean checkout; recorded Rust toolchain and target triples.
- Current-commit test run, not historical stale-commit success.
- Releasable artifact download, SHA256 manifest, version, changelog and known limitations (`REL-001`, `REL-002`, `REL-003`, `REL-006`, `REL-007`).
- Confirm original MIT license obligations, attribution and third-party notices (`ENV-009`).

## Gate 4 — Documentation and AI handoff (must)

- Active Rust crate clearly designated and older incomplete crate explicitly separated (`AUD-003`).
- README commands and screenshots/outputs match released binary.
- [PROGRESS.md](PROGRESS.md) is generated from [progress.json](progress.json); newly added tasks appear without manual copy.
- [GO-FILE-MAP.md](GO-FILE-MAP.md) lists implemented/deferred/excluded source functions accurately.
- [START-HERE.md](START-HERE.md) can be handed to a fresh agent without chat context.
- Final release note **must not** claim all-game Quest reward completion or unverified live capability.

## Optional improvements (not part of hard release gate)

- Extra local UI features, `stdin` convenience, bat launcher, CPU/RAM benchmarks, SBOM, test fuzzing beyond required safeguards, and mock network integration **unless a supported current feature actually depends on it**.

## Hard stop conditions

**Do NOT say “finished”** if any of the following holds: failing required CI/test; missing release-gating task with no accepted deferral; unsupported source mapped as working; “100% games” claims; unpinned/expired release with no provenance; missing upstream copyright notices for copied material; active unsafe account-token collection; docs and tracker disagree.

## Verification record template

```text
Release tag:
Git commit:
GitHub Actions run URL:
Linux binary architecture / checksum:
Main-server validation record:
Cargo.lock SHA256:
Passed commands + results:
Tracker open release gates (expected zero):
Documented exclusions:
Known non-gating limitations:
Who verified / date:
```

**Current status:** Not release-ready until all applicable Linux release gates pass. Historical Windows/hosted CI evidence is not part of the active verification process. Follow [LINUX-ONLY.md](LINUX-ONLY.md) and do not claim a public release when the archive exists only on main-server.
