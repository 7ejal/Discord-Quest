# Go → Rust Migration Progress

**Snapshot:** 2026-11-01 · **Source of truth:** [progress.json](progress.json) · **Scope:** offline/native Quest inspection and status verification, **not** a 1:1 operational selfbot.

## Status summary

| Status | Count | Meaning |
|---|---:|---|
| ✅ Done | 28 | Existing source/doc evidence meets stated criterion |
| 🟡 Partial | 7 | Some implementation exists; acceptance incomplete |
| ⬜ Todo | 75 | No verified implementation meeting criterion |
| ⛔ Excluded | 4 | Intentionally outside supported scope; not backlog |
| **Total** | **114** | Source and plan snapshot, not percent-complete |

**Verified build evidence:** [Linux/Windows GitHub Actions successful run](https://github.com/7ejal/Discord-Quest/actions/runs/41256646888) (15 Linux Rust tests; Windows executable build, smoke test and artifact upload). This check applies to the historical source commit it tested; **do not assume later changes passed without rerunning CI**.

**What exists now:** `Cargo.toml`, `src/{main,lib,model,manager}.rs`, `examples/sample-quests.json`, offline `demo`/`inspect`/`--json`/`--workers`, bounded parallel evaluation, Quest-status interpretation, and Windows EXE output. It does **not** connect to Discord or carry out rewarded tasks.

**Why not percentage complete:** The source project contains incompatible or untested behaviors. We measure completion of individually accepted supported tasks, not a fictional percentage of a guaranteed all-games selfbot.

## Source audit expansion (8 explicit audit tasks)

A new code-to-source pass found missing validation, duplicate Quest IDs, lost excluded-Quest records, insufficient timestamp validation, source/schema drift, unpinned packaging, and ambiguity between the two Rust projects. The **57 additional tasks below** cover those gaps; they are real unfinished work, **not** implemented features. A complete list today is not a promise that no future requirements will arise.

**Approved-scope release gates:** 96 required tasks, 68 open. The 4 excluded selfbot/spoofing tasks are not release blockers. Gate status is not a raw percentage of an operational selfbot.

**P0 immediate engineering blockers:** PARSE-001 (reject empty Quest), PARSE-002 (duplicate IDs), PARSE-005 (input bounds), STATUS-010 (RFC3339 timestamps), STATUS-012 (invalid expiry), AUD-003 (single active Rust project), and OPS-001/OPS-002 (auto-generated tracker and schema checks). Then move into versioned JSON output, quality tests, and dependable releases.

## Immediate next tasks

| Priority | ID | What to do | Definition of done |
|---|---|---|---|
| 1 | `ENV-005` | Commit Cargo.lock for reproducible builds | Cargo.lock committed and CI uses --locked; updates deliberately reviewed. |
| 2 | `ENV-006` | Add fmt, clippy and dependency audit checks | CI fails on cargo fmt --check and clippy -D warnings; audit policy documented. |
| 3 | `DOC-004` | Keep progress manifest and Markdown view synchronized | A local checker or generator validates IDs/statuses/dependencies and summary counts; CI runs it. |
| 4 | `MODEL-002` | Port core Quest, status, task, reward data types | All original data types and field mappings reviewed; supported fields deserialized with tests; absent upstream types explicitly documented. |
| 5 | `MODEL-003` | Preserve optional nested metadata, assets and unknown fields | Assets, gradients, metadata and optional fields round-trip or preserve unknown data; regression tests pass. |
| 6 | `STATE-008` | Handle starts_at and expires_at edge cases | Future starts and expired are differentiated; malformed/missing expiry must be flagged as unknown with test coverage. |
| 7 | `TEST-002` | Separate integration tests and golden JSON snapshots | Integration and golden tests verify CLI JSON output, operator logic, timestamp problems and fixtures. |
| 8 | `TEST-003` | 22 offline method validation cases | Implement 22 specified test cases as executable offline tests, mapping each case to a test name and result. |
| 9 | `ENV-009` | Confirm upstream license compatibility and attribution | Verify actual Go upstream license terms, preserve notices and document any copied code; no ungrounded relicensing. |
| 10 | `CLI-002` | Add --version and --verbose and structured errors | New flags documented; error codes predictable; no personal tokens required. |

## All tasks (with acceptance and evidence)

### Inventory

| ID | Status | Work | Acceptance / remaining gap | Evidence |
|---|---|---|---|---|
| `INV-001` | DONE | Pin original Go source revision | All migration references pin upstream commit 352d7f457c9511bc9e7c9b940a71012bfb47572e. | [link](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/tree/352d7f457c9511bc9e7c9b940a71012bfb47572e) |
| `INV-002` | DONE | Map every original Go root file | Each of 16 root entries has an explicit destination, partial flag, or excluded decision. | [link](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/tree/352d7f457c9511bc9e7c9b940a71012bfb47572e) |
| `INV-003` | DONE | Document original Go handler assumptions and known failures | Source-pinned report describes first-match task selection, false positives, unbounded waits, and claim default. | [link](https://github.com/7ejal/Discord-Quest/blob/main/research/standalone/GO.md) |

### Build

| ID | Status | Work | Acceptance / remaining gap | Evidence |
|---|---|---|---|---|
| `ENV-001` | DONE | Rust Cargo package exists | Cargo metadata defines a native Rust binary using serde/serde_json/chrono. | `Cargo.toml`, [link](https://github.com/7ejal/Discord-Quest/actions/runs/41256646888) |
| `ENV-002` | DONE | Native CLI entry point exists | A main entry point accepts demo/inspect and executes the offline analysis path. | `src/main.rs`, [link](https://github.com/7ejal/Discord-Quest/actions/runs/41256646888) |
| `ENV-003` | DONE | Linux CI tests and smoke test passed | Linux CI run 41256646888 completed successfully: cargo test and demo JSON. | [link](https://github.com/7ejal/Discord-Quest/actions/runs/41256646888) |
| `ENV-004` | DONE | Windows EXE build, smoke test and artifact passed | Windows CI run 41256646888 compiled and smoke tested EXE and uploaded artifact 13063362600. | [link](https://github.com/7ejal/Discord-Quest/actions/runs/41256646888), [link](https://github.com/7ejal/Discord-Quest/actions/runs/41256646888/artifacts/13063362600) |
| `ENV-005` | TODO | Commit Cargo.lock for reproducible builds | Cargo.lock committed and CI uses --locked; updates deliberately reviewed. | — |
| `ENV-006` | TODO | Add fmt, clippy and dependency audit checks | CI fails on cargo fmt --check and clippy -D warnings; audit policy documented. | — |
| `ENV-007` | TODO | Publish Linux executable artifact | Linux release output uploaded and verified by CI. | — |
| `ENV-008` | TODO | Automated tagged releases and checksums | Tagged release uploads signed or checksum-verified Windows/Linux artifacts with release notes. | — |
| `ENV-009` | TODO | Confirm upstream license compatibility and attribution | Verify actual Go upstream license terms, preserve notices and document any copied code; no ungrounded relicensing. | — |
| `ENV-010` | TODO | Create optional Windows .bat launcher | Launcher invokes verified local exe with safely quoted args, without adding capabilities. | — |

### Models

| ID | Status | Work | Acceptance / remaining gap | Evidence |
|---|---|---|---|---|
| `MODEL-001` | DONE | Parse Quest object, array, and wrapped collection | All three JSON input shapes parse; invalid root types are rejected. | `src/lib.rs`, [link](https://github.com/7ejal/Discord-Quest/actions/runs/41256646888) |
| `MODEL-002` | PARTIAL | Port core Quest, status, task, reward data types | All original data types and field mappings reviewed; supported fields deserialized with tests; absent upstream types explicitly documented. | `src/model.rs` |
| `MODEL-003` | TODO | Preserve optional nested metadata, assets and unknown fields | Assets, gradients, metadata and optional fields round-trip or preserve unknown data; regression tests pass. | — |
| `MODEL-004` | DONE | Read v2 and legacy task_config | Both schema variants are accepted in tests without panic. | `src/model.rs`, [link](https://github.com/7ejal/Discord-Quest/actions/runs/41256646888) |
| `MODEL-005` | DONE | Recognize ten documented current/historical task IDs | All named categories are classified; deprecated type flagged and unknown untrusted. | `src/manager.rs`, [link](https://github.com/7ejal/Discord-Quest/actions/runs/41256646888) |
| `MODEL-006` | DONE | Parse task-specific application IDs | Uses per-task applications first, top-level only as fallback; unit test passes. | `src/manager.rs`, [link](https://github.com/7ejal/Discord-Quest/actions/runs/41256646888) |
| `MODEL-007` | TODO | Typed task enum, unknown variants and versioned schemas | Typed parsing supports unknown future variants without accidentally asserting support; tests cover drift. | — |
| `MODEL-008` | PARTIAL | Validate timestamps, invalid values, and untrusted local input | Invalid/ambiguous dates, oversized JSON, unexpected numeric values and missing IDs produce explicit unknown/invalid outcomes, not implicit success. | `src/model.rs`, `src/manager.rs` |
| `MODEL-009` | PARTIAL | Full read-only reward metadata support | Reward types, amounts, expiry and all expected fields parsed without equating claimed to redeemed. | `src/model.rs` |

### State

| ID | Status | Work | Acceptance / remaining gap | Evidence |
|---|---|---|---|---|
| `STATE-001` | DONE | Port IsExpired/IsEnrolled/IsCompleted/HasClaimedRewards | Methods derive status from server-shaped timestamps and expiry; tests pass. | `src/model.rs`, [link](https://github.com/7ejal/Discord-Quest/actions/runs/41256646888) |
| `STATE-002` | DONE | Preserve original GetTaskName as compatibility-only heuristic | Original first-match precedence is represented but never determines entire Quest completion. | `src/model.rs`, [link](https://github.com/7ejal/Discord-Quest/actions/runs/41256646888) |
| `STATE-003` | DONE | Handle Android video-only classification | Recognizes mobile-only video task when desktop video is absent. | `src/model.rs` |
| `STATE-004` | DONE | Port FilterValid/FilterRedeemable semantics | Filters noncompleted/nonexpired and completed/unclaimed, respectively; tests pass. | `src/manager.rs`, [link](https://github.com/7ejal/Discord-Quest/actions/runs/41256646888) |
| `STATE-005` | DONE | Correctly evaluate AND/OR task groups | AND requires all task timestamps; OR requires one; unknown compound operators are unsupported; tests pass. | `src/manager.rs`, [link](https://github.com/7ejal/Discord-Quest/actions/runs/41256646888) |
| `STATE-006` | DONE | Do not turn numeric progress into completion | Progress >= target without completed_at is flagged, not reported as Quest completed; tests pass. | `src/manager.rs`, [link](https://github.com/7ejal/Discord-Quest/actions/runs/41256646888) |
| `STATE-007` | PARTIAL | Separate task, Quest, claim, and external redemption | Quest and claim statuses separate; add explicit externally-unverified redemption representation and tests. | `src/manager.rs` |
| `STATE-008` | PARTIAL | Handle starts_at and expires_at edge cases | Future starts and expired are differentiated; malformed/missing expiry must be flagged as unknown with test coverage. | `src/manager.rs` |
| `STATE-009` | DONE | Account enrollment-block marker remains diagnostic | Blocked-until input retained and displayed; no false claim of enrollment. | `src/main.rs` |

### Cli

| ID | Status | Work | Acceptance / remaining gap | Evidence |
|---|---|---|---|---|
| `CLI-001` | DONE | demo/inspect, JSON and --workers options | Demo and file inspection work with machine-readable output; options present and smoke-tested. | `src/main.rs`, [link](https://github.com/7ejal/Discord-Quest/actions/runs/41256646888) |
| `CLI-002` | TODO | Add --version and --verbose and structured errors | New flags documented; error codes predictable; no personal tokens required. | — |
| `CLI-003` | PARTIAL | Robust CLI UX for invalid inputs and safe help output | Tests cover missing flag values, unknown flags, non-UTF8 paths, large input and invalid JSON with stable nonzero exit codes. | `src/main.rs` |
| `CLI-004` | DONE | Bundle synthetic example and input/output documentation | Synthetic examples and command usage documented; demo smoke-tested. | `examples/sample-quests.json`, `README.md` |

### Concurrency

| ID | Status | Work | Acceptance / remaining gap | Evidence |
|---|---|---|---|---|
| `PAR-001` | DONE | Bounded parallel assessment in Rust | Analysis worker count bounded 1..32 and output preserves source order; unit test passes. | `src/manager.rs`, [link](https://github.com/7ejal/Discord-Quest/actions/runs/41256646888) |
| `PAR-002` | TODO | Add cancellation, clean shutdown and race tests | Cancellation paths release tasks/handles, no unbounded waits and tests cover shutdown. | — |
| `PAR-003` | TODO | Async runtime only if justified by supported I/O | If authorized network I/O is added, compare Tokio vs existing threads and document lifecycle; do not add for decoration. | — |

### Network

| ID | Status | Work | Acceptance / remaining gap | Evidence |
|---|---|---|---|---|
| `NET-001` | TODO | Provide credential-free HTTP fixture/mock adapter | Fake in-memory responses drive analysis under tests, with no live Discord requests. | — |
| `NET-002` | TODO | Bounded retries, timeouts, rate-limit and cancellation under mocks | Synthetic transport tests prove finite retries, deadlines, controlled 429/5xx and explicit error propagation. | — |
| `NET-003` | TODO | Document real Gateway protocol and local trace parser | Offline parsing of saved/synthetic Gateway frames and heartbeat lifecycle; no user-token Identify. | — |
| `NET-004` | TODO | Officially authorized application integrations only | If any live service integration is proposed, document supported authorization scopes and review against platform terms before building. | — |

### Tests

| ID | Status | Work | Acceptance / remaining gap | Evidence |
|---|---|---|---|---|
| `TEST-001` | DONE | Fifteen synthetic Rust unit tests passing | 15 unit tests passed in GitHub Actions run 41256646888. | [link](https://github.com/7ejal/Discord-Quest/actions/runs/41256646888) |
| `TEST-002` | TODO | Separate integration tests and golden JSON snapshots | Integration and golden tests verify CLI JSON output, operator logic, timestamp problems and fixtures. | — |
| `TEST-003` | PARTIAL | 22 offline method validation cases | Implement 22 specified test cases as executable offline tests, mapping each case to a test name and result. | `research/standalone/METHOD-VALIDATION-FIXTURES.md` |
| `TEST-004` | DONE | Reproducible compile and artifact checks on both platforms | Earlier CI run 41256646888 passed Linux and Windows jobs; note later commits require their own CI. | [link](https://github.com/7ejal/Discord-Quest/actions/runs/41256646888) |
| `TEST-005` | TODO | Coverage, property tests, fuzzing and benchmark numbers | Published measurements reproduce with inputs, machine specs and tools; no fabricated RAM/CPU claims. | — |

### Security

| ID | Status | Work | Acceptance / remaining gap | Evidence |
|---|---|---|---|---|
| `SEC-001` | DONE | No personal Discord user-token use in current offline app | Current Rust code has no personal-token login or Discord network API calls. | `src/main.rs`, `src/lib.rs` |
| `SEC-002` | TODO | Add secret-redacting diagnostics and dependency review | Logging never exposes token fragments or confidential headers; dependency licenses reviewed. | — |

### Docs

| ID | Status | Work | Acceptance / remaining gap | Evidence |
|---|---|---|---|---|
| `DOC-001` | DONE | Project README documents offline scope and Windows EXE | README explains commands, no live completion, passing prior CI and artifact link. | `README.md`, [link](https://github.com/7ejal/Discord-Quest/actions/runs/41256646888) |
| `DOC-002` | DONE | Publish master 1:1 migration spec, file map, tracker | Docs contain exact scoped goals and all Go source files; machine tracker has statuses, acceptance and evidence. | `migration/SPEC.md`, `migration/progress.json` |
| `DOC-003` | DONE | AI agent updating protocol | Agent guidance requires rechecking CI, modifying tracker with code changes, and no false completion claims. | `AGENTS.md`, `migration/AI-WORKFLOW.md` |
| `DOC-004` | DONE | Keep progress manifest and Markdown view synchronized | A local checker or generator validates IDs/statuses/dependencies and summary counts; CI runs it. | — |

### Excluded

| ID | Status | Work | Acceptance / remaining gap | Evidence |
|---|---|---|---|---|
| `SCOPE-001` | EXCLUDED | Personal-account token selfbot and client impersonation | Do not implement; current project is an authorized/offline analytics client. Preserve conceptual source inventory only. | — |
| `SCOPE-002` | EXCLUDED | Fabricated video/game/stream/activity progress | Do not implement engagement spoofing; categorize prerequisites and interpret server-shaped reports only. | — |
| `SCOPE-003` | EXCLUDED | Achievement OAuth reward simulation and challenge solving | No bypass of account verification, publisher achievements or reward claiming; keep documentation and data types. | — |
| `SCOPE-004` | EXCLUDED | Live Quest reward-completion and all-games guarantees | No universal 1:1 Quest selfbot acceptance claim; authentic external actions remain outside this project. | — |

## Update protocol for every coding agent

1. Read `AGENTS.md`, [SPEC.md](SPEC.md), [GO-FILE-MAP.md](GO-FILE-MAP.md), and **this** file before changing code.
2. Choose task IDs in [progress.json](progress.json). Do not start with vague “finish port” work.
3. Commit/test implementation and log exact files, commit, tests and limitations.
4. Only mark `done` after acceptance criteria pass and evidence is linked. Otherwise keep `partial` or `todo`.
5. Never turn `excluded` into `todo` without an explicitly approved, policy-compliant scope review.
6. Rebuild this Markdown summary from the JSON tracker using `python migration/check_progress.py --write` once the validator exists; until then update the table and JSON in the same commit.
7. Rerun supported CI. A historical passing run is not a guarantee for newer changes.
8. Keep no personal Discord credentials in examples, logs, issues or CI secrets.

**Source evidence:** [Pinned Go source](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/tree/352d7f457c9511bc9e7c9b940a71012bfb47572e) · [Static Go audit](../../../research/standalone/GO.md) · [Method limitations](../../../research/standalone/METHODS-DEEP-DIVE-2026-11-01.md).

## Additional audit-backed tasks (new; canonical source: progress.json)

These tasks have no implementation merely because they appear here. **Required** indicates an approved-scope release blocker; **optional** is separately tracked and does not inflate core release readiness.

### AUDIT

| ID | Status | Work | Acceptance / remaining gap | Evidence | Priority | Gate |
|---|---|---|---|---|---|---|
| `AUD-001` | TODO | Verify real task-group schema variants against sanitized fixtures | Interpret join_operator/string/numeric and config v1/v2 using vetted sanitized fixtures; unsupported representations remain explicitly unknown. | — | P0 | Required |
| `AUD-002` | TODO | Diff Go behavior and Rust behavior with a version-pinned contract table | For every Go state/filter/dispatcher method, identify exact Rust behavior, intentional differences, and expected offline fixture outputs. | — | P0 | Required |
| `AUD-003` | TODO | Resolve duplicate Rust project ownership | Document and enforce rust/go-quest-port as the only active code path; archive or explicitly label rust/quest-audit-cli as nonshippable research to prevent agents editing it by mistake. | — | P0 | Required |
| `AUD-004` | TODO | Audit source links, file map, and documentation consistency | Automated check verifies tracked relative markdown links, source-file mapping, and no unresolved placeholder file paths presented as implemented. | — | P1 | Required |
| `AUD-005` | TODO | Prevent drift between Rust task families and data/task-types.json | Add a deterministic test or generator comparing recognized identifiers and deprecated tags against catalog with a pinned interpretation. | — | P1 | Required |
| `AUD-006` | TODO | Inventory original feature claims versus actually evidenced operation | Matrix must distinguish source handler, precondition, server confirmation, and real-world verification; no handler or README claim may imply supported engagement. | — | P0 | Required |
| `AUD-007` | TODO | Clean literal escaped newline sequences and stale links in root documentation | Root README headings, relative links and status statements render correctly on GitHub; targeted formatting check verifies no literal '\n\n' artifacts in prose. | — | P2 | Optional |
| `AUD-008` | TODO | Document upstream code/license provenance and version baselines | Record upstream Git SHA, original Go dependencies, Rust toolchain minimum, scoped license obligations, and evidence for each third-party claim. | — | P1 | Required |

### PARSER

| ID | Status | Work | Acceptance / remaining gap | Evidence | Priority | Gate |
|---|---|---|---|---|---|---|
| `PARSE-001` | TODO | Reject empty or invalid Quest objects instead of silently accepting {} | Synthetic {} and missing/blank quest ID or missing config produce a typed invalid/unknown result; never classify as normal not-enrolled without warning. | — | P0 | Required |
| `PARSE-002` | TODO | Define duplicate quest IDs and reproduce intentional Go map-dedup semantics | Multiple entries with same ID have documented deterministic conflict handling, not accidental double counting; tests cover same/different statuses. | — | P0 | Required |
| `PARSE-003` | TODO | Retain excluded_quests and preview/traffic metadata in reports | Go wrapper's excluded list is not discarded, and incomplete excluded configurations are labeled partial/not evaluated; tests prove no silent loss. | — | P1 | Required |
| `PARSE-004` | TODO | Preserve unknown future JSON fields when required for round-trip | Schema retains unknown fields or outputs explicit lossiness notices, with fixture round-trip tests for nested extension fields. | — | P1 | Optional |
| `PARSE-005` | TODO | Bound input file bytes, nesting depth, quest count, and output size | Oversized/deep/wide synthetic documents fail cleanly with typed errors and no unbounded allocations; limits are configurable/documented. | — | P0 | Required |
| `PARSE-006` | TODO | Validate target/progress numeric domain and overflow cases | Negative, zero, nonfinite/out-of-range targets and values cannot become confirmed completion; warnings/tests cover arithmetic safety. | — | P0 | Required |
| `PARSE-007` | TODO | Differentiate null, missing, wrong type, and invalid nested status | For timestamps, user_status and progress, synthetic wrong-type fields lead to explicit invalid/unknown handling rather than confusing defaults. | — | P1 | Required |
| `PARSE-008` | TODO | Handle UTF-8, BOM, Windows paths, and malformed JSON safely | CLI tests cover valid Unicode names and paths, BOM policy, malformed UTF-8, and stable diagnostic output under Windows/Linux. | — | P1 | Required |
| `PARSE-009` | TODO | Document strict versus tolerant parsing for historical configuration | Tests show version fallback rules, absent task map, unexpected join-operator formats and migration precedence; no guessed fallback silently succeeds. | — | P1 | Required |
| `PARSE-010` | TODO | Measure field coverage against original types.go not just top-level names | Every Go struct field has a Rust counterpart, intentional ignore, or explicit annotation in a generated/manual coverage table; compile tests for supported families. | — | P1 | Required |

### STATE

| ID | Status | Work | Acceptance / remaining gap | Evidence | Priority | Gate |
|---|---|---|---|---|---|---|
| `STATUS-010` | TODO | Validate completion/enrollment/claim timestamps instead of nonempty-string truthiness | Whitespace and non-RFC3339 timestamps cannot establish a server-confirmed state; valid timezone offsets are supported. | — | P0 | Required |
| `STATUS-011` | TODO | Detect impossible ordering and contradictory status timestamps | Synthetic claim-before-completion, completion-before-enrollment, starts-after-expiry, and invalid clocks yield explicit inconsistent/unknown states. | — | P0 | Required |
| `STATUS-012` | TODO | Avoid silent unexpired treatment of malformed/missing expiration dates | Distinguish missing/invalid expires_at from valid future expiry and display uncertainty in filters/status; tests cover malformed dates. | — | P0 | Required |
| `STATUS-013` | TODO | Label provenance and trust for each reported milestone | Output explicitly identifies source field, local inference, synthetic fixture versus server-shaped record, and never presents locally inferred status as authoritative. | — | P0 | Required |
| `STATUS-014` | TODO | Keep unsupported/unknown tasks distinguishable even on completed Quest records | Mixed known/unknown and deprecated tasks retain warnings; Quest completed_at may be reported but never construed as implemented task handler support. | — | P1 | Required |
| `STATUS-015` | TODO | Verify AND/OR groups with mixed progress, missing task statuses, and other operators | Property/table tests cover single/AND/OR, unknown operator, unknown/missing task statuses and conditional eligibility without false success. | — | P0 | Required |
| `STATUS-016` | TODO | Inject an evaluation clock and add deterministic --at option | Tests/CLI can evaluate a timestamp without Utc::now nondeterminism; DST/UTC offsets and boundary-time cases are covered. | — | P1 | Required |
| `STATUS-017` | TODO | Compare Go and Rust filtering behavior for preview, blocking, starts and duplicates | Document identical and intentionally different filter results with named fixtures; no ambiguous parity claim. | — | P1 | Required |
| `STATUS-018` | TODO | Model claimable, claimed, and externally redeemed as separate evidence states | Result format and tests differentiate Quest completion, reward claim availability/confirmation, and externally unknown redemption. | — | P1 | Required |

### UX

| ID | Status | Work | Acceptance / remaining gap | Evidence | Priority | Gate |
|---|---|---|---|---|---|---|
| `UX-001` | TODO | Support --version, --verbose, robust --help and CLI test coverage | Stable version/help output and verbose diagnostic mode that does not disclose sensitive data; tests cover invalid flags. | — | P1 | Required |
| `UX-002` | TODO | Version JSON output schema and keep stdout machine-readable | --json emits exactly one documented valid JSON result on stdout with schema_version, errors on stderr and deterministic field naming; golden tests pass. | — | P0 | Required |
| `UX-003` | TODO | Add optional stdin/pipeline support with bounded input | A documented '-' stdin input works with identical validation to inspect file and rejects oversized streams; no network or token collection. | — | P2 | Optional |
| `UX-004` | TODO | Document stable exit codes for invalid input and execution errors | Typed failures have predictable nonzero exit codes; success/empty report semantics are documented and tested on both platforms. | — | P1 | Required |
| `UX-005` | TODO | Test Windows path quoting, Unicode and terminal encoding behavior | Windows smoke tests cover spaces and non-ASCII filenames and output, including bat wrapper if added. | — | P1 | Required |
| `UX-006` | TODO | Keep debug logs sanitized and do not echo raw input or secrets | Tests show token-like strings inside malformed JSON/errors cannot be emitted; diagnostic truncation and escaping documented. | — | P0 | Required |

### QUALITY

| ID | Status | Work | Acceptance / remaining gap | Evidence | Priority | Gate |
|---|---|---|---|---|---|---|
| `QA-001` | TODO | Maintain test-to-requirement traceability for all release-gating tasks | Every release-gate task has test file/case ID or a verified non-code acceptance artifact; CI checks missing test mapping. | — | P0 | Required |
| `QA-002` | TODO | Property tests and fuzz harness for schema and untrusted JSON | Fuzz/property tests demonstrate no panics and stable unknown/invalid classification with bounded synthetic inputs; CI smoke-level fuzz schedule specified. | — | P1 | Required |
| `QA-003` | TODO | Golden CLI snapshots with deterministic fixture time and text/JSON outputs | Committed expected outputs compare identically on Windows/Linux with normalized paths/timezones; fixture changes reviewed. | — | P0 | Required |
| `QA-004` | TODO | Stress test duplicate IDs, 100k mock tasks, large values and empty lists | Test resource limits, correct ordering, duplicate handling and no panic on synthetic large/empty documents. | — | P1 | Required |
| `QA-005` | TODO | Negative end-to-end test ensures no Discord network, user-token, or reward submission | Offline CLI works with network blocked and denies user-token operation flags; evidence proves no credential requirement. | — | P0 | Required |
| `QA-006` | TODO | Reproducible CPU/RAM performance and binary-size measurements | Benchmark setup, platform/hardware, time/memory and reproducible input sets published; no speculative improvement claims. | — | P2 | Optional |
| `QA-007` | TODO | Regression corpus for source-level failures in related Go/Python/TS projects | Each known false-success, infinite retry, stale status and conditional EventEmitter case has a synthetic input and expected safe diagnostic. | — | P1 | Required |
| `QA-008` | TODO | Test tracker checker itself against malformed manifests | Unit tests confirm checker rejects duplicate IDs, broken deps/cycles, stale Markdown, invalid statuses and nonexistent path evidence appropriately. | — | P1 | Required |

### RELEASE

| ID | Status | Work | Acceptance / remaining gap | Evidence | Priority | Gate |
|---|---|---|---|---|---|---|
| `REL-001` | TODO | Pin rust-toolchain, MSRV and target architecture policy | rust-toolchain.toml or documented MSRV consistent with CI; linux/windows target triples and version bump policy explicit. | — | P1 | Required |
| `REL-002` | TODO | Build and smoke test Linux and Windows artifacts for tagged commit | Both platform binaries built on tag, rerun smoke tests against artifacts, record target and checksums. | — | P0 | Required |
| `REL-003` | TODO | Publish versioned release notes, SHA256 and release assets | A durable GitHub release (not expiring Actions artifacts only) with checksum manifest, notes, limitations and reproducible tag. | — | P1 | Required |
| `REL-004` | TODO | Harden CI permissions, action pinning and dependency updates | Set least-privilege permissions, avoid broad tokens, pin actions or document version policy, scope PR workflows and dependabot policy. | — | P1 | Required |
| `REL-005` | TODO | Generate SBOM or dependency/license inventory for release | Catalog all transitive crates and licenses, verify upstream notice obligations, attach inventory to release. | — | P2 | Optional |
| `REL-006` | TODO | Check reproducible locked builds and supply-chain provenance | Build with --locked from clean checkout, checksum input lockfile, document signing/provenance limitation. | — | P1 | Required |
| `REL-007` | TODO | Document artifact retention and last-verified commit instead of stale EXE links | README and tracker state which Git commit each binary represents, link stable release or note expiry; no latest-success ambiguity. | — | P0 | Required |
| `REL-008` | TODO | Create final release readiness checklist with stop conditions | Must-pass checklist covers tests, release source, schema conformance, policy scope, licenses, docs, reproducibility and rollback. | — | P0 | Required |

### TRACKING

| ID | Status | Work | Acceptance / remaining gap | Evidence | Priority | Gate |
|---|---|---|---|---|---|---|
| `OPS-001` | TODO | Regenerate Markdown task rows automatically from canonical JSON | Adding/removing a task ID through JSON and --write updates whole table, phase headings, counts without manual copying, preserving existing curated prose. | — | P0 | Required |
| `OPS-002` | TODO | Validate expanded tracker schema, dependencies, cycles, and priorities | Checker enforces required fields, unique IDs, permitted statuses/P0-P3, release_gate flags, DAG dependencies and consistent summaries. | — | P0 | Required |
| `OPS-003` | TODO | Evidence must identify tested commit or immutable file and not just a claim | Tracker records verification date, commit/run/source and CI status; checker warns on unverified or outdated 'done' reports. | — | P0 | Required |
| `OPS-004` | TODO | Summarize release-gating versus optional tasks separately | Dashboard computes open release blockers from task metadata; excluded and optional tasks do not distort readiness percentage. | — | P0 | Required |
| `OPS-005` | TODO | Maintain architecture decision log and unresolved assumptions | ADR file records exact decision/rationale/evidence for source compatibility, offline-only scope, crate choices and stage changes. | — | P1 | Required |
| `OPS-006` | TODO | Provide one copy-paste next-session AI handoff with task order | Session entry names files, branch, actual status, top five dependencies, exact verification commands, and completion reporting format. | — | P0 | Required |
| `OPS-007` | TODO | Check markdown links and declared implemented paths in CI | CI catches broken repo-relative links and evidence paths marked as implemented; designed to avoid false failures for proposed paths. | — | P1 | Required |
| `OPS-008` | TODO | PR/issue template requires task ID, tests, tracker update and residual risk | GitHub templates prompt authors for task IDs, evidence on current commit, docs status, policy scope and pending blockers. | — | P2 | Optional |

