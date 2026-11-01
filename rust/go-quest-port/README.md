# Offline Quest inspector — Linux x86_64

A native Rust CLI for inspecting synthetic or sanitized local Quest JSON. No
Discord installation, account token or network connection is needed. The binary
does not enroll accounts, perform engagement, send heartbeats, bypass CAPTCHA,
or claim rewards.

## Build and verify on main-server

Use the isolated `/root/quest-rust-ci-20261101` checkout. Source changes are
committed to GitHub first, reviewed, then imported with a fast-forward pull.
Production services and system configuration are outside this workflow.

```bash
cd /root/quest-rust-ci-20261101/rust/go-quest-port
CARGO_BUILD_JOBS=2 nice -n 10 bash scripts/test-linux.sh
CARGO_BUILD_JOBS=2 nice -n 10 bash scripts/package-linux.sh
```

The test script runs tracker/links checks, Cargo formatting, strict Clippy,
locked tests, the pinned pure-Go comparison, optimized compilation, golden
reports, and smoke tests in a process-local network namespace. Rust 1.98.1 is
pinned; automatic toolchain installation is disabled. Go and Python are build
verification tools only; neither is required to run the released executable.

Packaging refuses an uncommitted checkout, verifies the extracted executable
and all archive hashes, and includes LICENSE, NOTICE, Cargo.lock, compiler/source
provenance and dependency/license inventories. Output is in `dist/linux-x86_64`.
An archive on main-server is a local release candidate, not a public GitHub
release. Windows, GUI and GitHub-hosted verification are outside scope.

## Usage

```bash
./target/release/go-quest-port --help
./target/release/go-quest-port --version
./target/release/go-quest-port demo --json --at 2026-11-01T12:00:00Z
./target/release/go-quest-port inspect examples/sample-quests.json --json
cat examples/sample-quests.json | ./target/release/go-quest-port inspect - --json --verbose
```

Options after `demo` or `inspect <path|->`:

| Option | Meaning |
|---|---|
| `--json` | One versioned JSON document on stdout |
| `--at RFC3339` | Fixed evaluation clock, including timezone |
| `--workers N` | 1–32 bounded workers; default 4 |
| `--verbose` | Counts and trust label on stderr; no raw input or filenames |
| `--max-input-bytes N` | Lower the default 8 MiB input limit |
| `--max-output-bytes N` | Lower the default 16 MiB report limit |

Hard limits: UTF-8 input with optional leading BOM, 10,000 full and 10,000
excluded records per list, depth 64, and 100,000 combined task/progress entries
per document. Both text and JSON reports are buffered before stdout. Oversized
reports fail without printing a partial result. Input/output limits may be
lowered, never raised above the safety caps.

Exit codes: **0** success (including an empty valid list), **64** command/option
error, **65** invalid data or JSON output limit, **70** worker failure, **74** I/O
failure (including text output limit). Errors go to stderr with source values
and filenames omitted. Linux filenames need not be UTF-8; file contents must be.

## Input, schema and privacy

Accepts one full Quest, an array, or a wrapper with `quests`,
`excluded_quests` and optional `quest_enrollment_blocked_until`.
Use `examples/sample-quests.json` as a synthetic starting point; change invented
names and IDs locally. Remove user IDs, targeting/traffic metadata, credentials
and personal details before sharing input or output. No scraping or account
login is part of this workflow.

Duplicate identical Quest records are deduplicated; conflicting duplicates are
errors. V2 config takes precedence without merging; legacy config is accepted
only when v2 is null/absent. Missing/null optional status timestamps carry no
milestone; wrong types fail. Invalid dates and numbers never establish completion.
Unknown and deprecated task IDs retain warnings even when the local file reports
Quest completion or a claimed reward.

The pinned Go model contains **no join operator**. Rust supports string
`and`/`or` only as an explicit offline extension; a lone task with no operator
is `single`. Missing multi-task or unknown operators fail closed. Compatibility
with modern authenticated Discord task-group samples remains unverified.

JSON schema version 1 is additive: every assessment/task has
`input_trust: "unverified_local_json"`, and external redemption is independently
`unknown`. `reward_claim_availability` describes reported local fields and
explicitly leaves eligibility unverified. Legacy field names
`server_task_completed` and verdict `quest_confirmed_unclaimed` are retained
for compatibility; they mean valid timestamps **reported in this local file**,
never independently authenticated server evidence.

Application selection reports both the chosen ID and its source (task config,
campaign fallback or missing). User IDs, opaque metadata and arbitrary source
extensions are not included in summaries. Campaign names, Quest/application IDs,
task/event labels and freshness fields remain visible inspection data. Do not
put secrets in those fields. Terminal controls are escaped in human output;
JSON preserves report strings for machine processing.

## Requested live HTTP / Go parity work (not implemented)

The user requested Go-style raw `Authorization` header account access and live Quest inspection. This is tracked under [GO-LIVE-HTTP-REQUEST.md](migration/GO-LIVE-HTTP-REQUEST.md) and separate pending `AUD-009`, `SEC-005`, `NET-005`–`NET-009`, `QA-011`, `AUD-010` tasks. **The current executable still has no HTTP or account login command.** Current offline test results do not verify real Discord account or Quest functionality. Supported authorization and service rules must be checked before any live integration; do not send, commit or print personal tokens.

## Verification and remaining gates

[Continuation release evidence](migration/MAIN-SERVER-COMPLETION-2026-11-01.md)
records exact tested commits, check results, artifact hashes and measured resource
use. [Progress](migration/PROGRESS.md) is generated from
[progress.json](migration/progress.json); status changes require passing evidence.

The [field/behavior ledger](migration/MODEL-COVERAGE.md) covers all 26 pinned Go
structs, including explicit exclusions and opaque metadata preservation.
[Acceptance mapping](migration/ACCEPTANCE-EVIDENCE.md) maps tests and non-code
artifacts to tasks. [Research-case mapping](migration/RESEARCH-CASE-MAP.md)
distinguishes executable checks from network/OAuth cases that do not apply.

Modern task-group schema verification remains open under `AUD-001` and
`STATE-005`. Public release publication remains unperformed under `REL-003`;
the existing general-release gate is retained. Do not describe this candidate
as fully release-ready. Optional network integrations are deferred, not needed
to run the offline CLI.

Original source: yeweroooo/Discord-Quest-Auto-Completion-Selfbot at
`352d7f457c9511bc9e7c9b940a71012bfb47572e`. The MIT license and original
copyright are preserved in [LICENSE](LICENSE) and [NOTICE](NOTICE).
See [Linux troubleshooting](docs/TROUBLESHOOTING.md) for library/architecture
compatibility and [main-server workflow](migration/MAIN-SERVER-WORKFLOW.md).
