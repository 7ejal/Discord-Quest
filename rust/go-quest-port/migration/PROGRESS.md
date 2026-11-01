# Go to Rust — migration progress (generated)

**This file is generated from [progress.json](progress.json). Do not edit it directly.**
Run `python3 migration/check_progress.py --write` after changing JSON, and `python3 migration/check_progress.py --check` before committing.

Project: [rust/go-quest-port](../) · Source: [pinned upstream Go](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/tree/352d7f457c9511bc9e7c9b940a71012bfb47572e)

**Scope:** native, read-only/offline Quest data analysis. Not an operational user-account selfbot or an all-games reward-completion system. Excluded tasks are intentional.

## Task counts

| Status | Count | Interpretation |
|---|---:|---|
| DONE | 38 | Acceptance evidence recorded; check verification commit |
| PARTIAL | 13 | Some implementation exists but acceptance incomplete |
| TODO | 72 | Work not yet demonstrated |
| EXCLUDED | 4 | Deliberately outside approved scope |
| **TOTAL** | **127** | Inventory snapshot, not a completion percentage |

## Release-gating status (approved offline scope only)

**108 required tasks · 72 open release gates · 36 done release gates.**
Optional and excluded tasks do not block release. See [release criteria](RELEASE-GATES.md). A green historical CI run does not verify future commits.

## Next uncompleted high-priority release gates

| ID | Priority | Blocker | Dependencies |
|---|---|---|---|
| `AUD-001` | P0 | Verify real task-group schema variants against sanitized fixtures | `MODEL-004` |
| `AUD-002` | P0 | Diff Go behavior and Rust behavior with a version-pinned contract table | `INV-002` |
| `DOC-005` | P0 | Document provenance limits and safe local JSON input workflows | `STATUS-013` |
| `MODEL-010` | P0 | Check TaskEntry.type against task-map key and supported task family | `PARSE-010` |
| `OPS-003` | P0 | Evidence must identify tested commit or immutable file and not just a claim | `OPS-002` |
| `OPS-009` | P0 | Generate a dependency-aware, actually-startable next-task queue | `OPS-002`, `OPS-004` |
| `PARSE-001` | P0 | Reject empty or invalid Quest objects instead of silently accepting {} | `MODEL-001` |
| `PARSE-002` | P0 | Define duplicate quest IDs and reproduce intentional Go map-dedup semantics | `AUD-002` |
| `PARSE-005` | P0 | Bound input file bytes, nesting depth, quest count, and output size | `CLI-003` |
| `PARSE-006` | P0 | Validate target/progress numeric domain and overflow cases | `MODEL-008` |
| `QA-001` | P0 | Maintain test-to-requirement traceability for all release-gating tasks | `TEST-002` |
| `QA-003` | P0 | Golden CLI snapshots with deterministic fixture time and text/JSON outputs | `STATUS-016`, `UX-002` |
| `QA-005` | P0 | Negative end-to-end test ensures no Discord network, user-token, or reward submission | `SEC-001` |
| `REL-002` | P0 | Build and smoke test Linux and Windows artifacts for tagged commit | `ENV-007` |
| `REL-007` | P0 | Document artifact retention and last-verified commit instead of stale EXE links | `ENV-004` |
| `SEC-003` | P0 | Sanitize terminal control sequences and Unicode formatting attacks | `CLI-003` |
| `STATE-019` | P0 | Validate Quest ID consistency across wrapper, config and user status | `PARSE-001`, `STATUS-010` |
| `STATUS-010` | P0 | Validate completion/enrollment/claim timestamps instead of nonempty-string truthiness | `STATE-001` |

## Detailed task inventory

All tasks have exact acceptance, evidence and dependencies in the JSON source. Do not change status without verifying acceptance against the **current code**.

### Inventory

| ID | Status | Priority | Gate | Work | Acceptance | Depends on | Evidence (first) |
|---|---|---|---|---|---|---|---|
| `INV-001` | DONE | P1 | Required | Pin original Go source revision | All migration references pin upstream commit 352d7f457c9511bc9e7c9b940a71012bfb47572e. | — | [source](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/tree/352d7f457c9511bc9e7c9b940a71012bfb47572e) |
| `INV-002` | DONE | P1 | Required | Map every original Go root file | Each of 16 root entries has an explicit destination, partial flag, or excluded decision. | — | [source](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/tree/352d7f457c9511bc9e7c9b940a71012bfb47572e) |
| `INV-003` | DONE | P1 | Required | Document original Go handler assumptions and known failures | Source-pinned report describes first-match task selection, false positives, unbounded waits, and claim default. | — | [source](https://github.com/7ejal/Discord-Quest/blob/main/research/standalone/GO.md) |

### Audit

| ID | Status | Priority | Gate | Work | Acceptance | Depends on | Evidence (first) |
|---|---|---|---|---|---|---|---|
| `AUD-001` | TODO | P0 | Required | Verify real task-group schema variants against sanitized fixtures | Determine authoritative or sanitized current task group representations; document whether join_operator exists, its data type and default; add Rust parser/AND/OR fixtures and mark unsupported unknown cases. | `MODEL-004` | — |
| `AUD-002` | PARTIAL | P0 | Required | Diff Go behavior and Rust behavior with a version-pinned contract table | For every Go state/filter/dispatcher method, identify exact Rust behavior, intentional differences, and expected offline fixture outputs. | `INV-002` | `migration/GAP-AUDIT.md` |
| `AUD-003` | DONE | P0 | Required | Resolve duplicate Rust project ownership | Document and enforce rust/go-quest-port as the only active code path; archive or explicitly label rust/quest-audit-cli as nonshippable research to prevent agents editing it by mistake. | — | `rust/quest-audit-cli/README.md` |
| `AUD-006` | DONE | P0 | Required | Inventory original feature claims versus actually evidenced operation | Matrix must distinguish source handler, precondition, server confirmation, and real-world verification; no handler or README claim may imply supported engagement. | `INV-003` | `migration/CAPABILITY-CONTRACT.md` |
| `AUD-004` | PARTIAL | P1 | Required | Audit source links, file map, and documentation consistency | Automated check verifies tracked relative markdown links, source-file mapping, and no unresolved placeholder file paths presented as implemented. | `DOC-002` | `migration/check_links.py` |
| `AUD-005` | TODO | P1 | Required | Prevent drift between Rust task families and data/task-types.json | Add a deterministic test or generator comparing recognized identifiers and deprecated tags against catalog with a pinned interpretation. | `MODEL-005` | — |
| `AUD-008` | PARTIAL | P1 | Required | Document upstream code/license provenance and version baselines | Record upstream Git SHA, original Go dependencies, Rust toolchain minimum, scoped license obligations, and evidence for each third-party claim. | `ENV-009` | `migration/SOURCE-BASELINE.md` |
| `AUD-007` | DONE | P2 | Optional/excluded | Clean literal escaped newline sequences and stale links in root documentation | Root README headings, relative links and status statements render correctly on GitHub; targeted formatting check verifies no literal '\n\n' artifacts in prose. | `DOC-001` | `README.md` |

### Build

| ID | Status | Priority | Gate | Work | Acceptance | Depends on | Evidence (first) |
|---|---|---|---|---|---|---|---|
| `ENV-001` | DONE | P1 | Required | Rust Cargo package exists | Cargo metadata defines a native Rust binary using serde/serde_json/chrono. | — | `Cargo.toml` |
| `ENV-002` | DONE | P1 | Required | Native CLI entry point exists | A main entry point accepts demo/inspect and executes the offline analysis path. | — | `src/main.rs` |
| `ENV-003` | DONE | P1 | Required | Linux CI tests and smoke test passed | Linux CI run 41256646888 completed successfully: cargo test and demo JSON. | — | [source](https://github.com/7ejal/Discord-Quest/actions/runs/41256646888) |
| `ENV-004` | DONE | P1 | Required | Windows EXE build, smoke test and artifact passed | Windows CI run 41256646888 compiled and smoke tested EXE and uploaded artifact 13063362600. | — | [source](https://github.com/7ejal/Discord-Quest/actions/runs/41256646888) |
| `ENV-005` | TODO | P2 | Required | Commit Cargo.lock for reproducible builds | Cargo.lock committed and CI uses --locked; updates deliberately reviewed. | `ENV-001` | — |
| `ENV-006` | TODO | P2 | Required | Add fmt, clippy and dependency audit checks | CI fails on cargo fmt --check and clippy -D warnings; audit policy documented. | `ENV-003` | — |
| `ENV-007` | TODO | P2 | Required | Publish Linux executable artifact | Linux release output uploaded and verified by CI. | `ENV-003` | — |
| `ENV-008` | TODO | P2 | Optional/excluded | Automated tagged releases and checksums | Tagged release uploads signed or checksum-verified Windows/Linux artifacts with release notes. | `ENV-004`, `ENV-007` | — |
| `ENV-009` | TODO | P2 | Required | Confirm upstream license compatibility and attribution | Verify actual Go upstream license terms, preserve notices and document any copied code; no ungrounded relicensing. | `INV-001` | — |
| `ENV-010` | TODO | P2 | Optional/excluded | Create optional Windows .bat launcher | Launcher invokes verified local exe with safely quoted args, without adding capabilities. | `ENV-004` | — |

### Models

| ID | Status | Priority | Gate | Work | Acceptance | Depends on | Evidence (first) |
|---|---|---|---|---|---|---|---|
| `MODEL-010` | TODO | P0 | Required | Check TaskEntry.type against task-map key and supported task family | Parse Go TaskEntry.type and compare it to the map key where both exist; mismatches, unrecognized types, empty application IDs and task-specific app mappings remain explicit unknown/inconsistent cases. | `PARSE-010` | — |
| `MODEL-001` | DONE | P1 | Required | Parse Quest object, array, and wrapped collection | All three JSON input shapes parse; invalid root types are rejected. | — | `src/lib.rs` |
| `MODEL-004` | DONE | P1 | Required | Read v2 and legacy task_config | Both schema variants are accepted in tests without panic. | — | `src/model.rs` |
| `MODEL-005` | DONE | P1 | Required | Recognize ten documented current/historical task IDs | All named categories are classified; deprecated type flagged and unknown untrusted. | — | `src/manager.rs` |
| `MODEL-006` | DONE | P1 | Required | Parse task-specific application IDs | Uses per-task applications first, top-level only as fallback; unit test passes. | — | `src/manager.rs` |
| `MODEL-011` | TODO | P1 | Required | Correct reward wire models without inventing source fields | Cover every Go QuestRewardsConfig/QuestReward field with serde aliases, null handling and golden fixture; any Rust orb quantities documented as non-Go extensions, not claimed Go parity. | `MODEL-009`, `PARSE-010` | — |
| `MODEL-002` | PARTIAL | P2 | Required | Port core Quest, status, task, reward data types | All original data types and field mappings reviewed; supported fields deserialized with tests; absent upstream types explicitly documented. | — | `src/model.rs` |
| `MODEL-003` | TODO | P2 | Required | Preserve optional nested metadata, assets and unknown fields | Assets, gradients, metadata and optional fields round-trip or preserve unknown data; regression tests pass. | `MODEL-002` | — |
| `MODEL-007` | TODO | P2 | Required | Typed task enum, unknown variants and versioned schemas | Typed parsing supports unknown future variants without accidentally asserting support; tests cover drift. | `MODEL-002` | — |
| `MODEL-008` | PARTIAL | P2 | Required | Validate timestamps, invalid values, and untrusted local input | Invalid/ambiguous dates, oversized JSON, unexpected numeric values and missing IDs produce explicit unknown/invalid outcomes, not implicit success. | — | `src/model.rs` |
| `MODEL-009` | PARTIAL | P2 | Required | Full read-only reward metadata support | Map Go QuestRewardsConfig and QuestReward field-by-field (assignment_method, rewards_expire_at, platforms, sku_id, asset, asset_video, messages); handle extra Rust fields as separately sourced extensions or remove them. | — | `src/model.rs` |

### Parser

| ID | Status | Priority | Gate | Work | Acceptance | Depends on | Evidence (first) |
|---|---|---|---|---|---|---|---|
| `PARSE-001` | TODO | P0 | Required | Reject empty or invalid Quest objects instead of silently accepting {} | Synthetic {} and missing/blank quest ID or missing config produce a typed invalid/unknown result; never classify as normal not-enrolled without warning. | `MODEL-001` | — |
| `PARSE-002` | TODO | P0 | Required | Define duplicate quest IDs and reproduce intentional Go map-dedup semantics | Multiple entries with same ID have documented deterministic conflict handling, not accidental double counting; tests cover same/different statuses. | `AUD-002` | — |
| `PARSE-005` | TODO | P0 | Required | Bound input file bytes, nesting depth, quest count, and output size | Oversized/deep/wide synthetic documents fail cleanly with typed errors and no unbounded allocations; limits are configurable/documented. | `CLI-003` | — |
| `PARSE-006` | TODO | P0 | Required | Validate target/progress numeric domain and overflow cases | Negative, zero, nonfinite/out-of-range targets and values cannot become confirmed completion; warnings/tests cover arithmetic safety. | `MODEL-008` | — |
| `PARSE-003` | TODO | P1 | Required | Retain excluded_quests and preview/traffic metadata in reports | Go wrapper's excluded list is not discarded, and incomplete excluded configurations are labeled partial/not evaluated; tests prove no silent loss. | `MODEL-002` | — |
| `PARSE-004` | TODO | P1 | Optional/excluded | Preserve unknown future JSON fields when required for round-trip | Schema retains unknown fields or outputs explicit lossiness notices, with fixture round-trip tests for nested extension fields. | `MODEL-003` | — |
| `PARSE-007` | TODO | P1 | Required | Differentiate null, missing, wrong type, and invalid nested status | For timestamps, user_status and progress, synthetic wrong-type fields lead to explicit invalid/unknown handling rather than confusing defaults. | `MODEL-008` | — |
| `PARSE-008` | TODO | P1 | Required | Handle UTF-8, BOM, Windows paths, and malformed JSON safely | CLI tests cover valid Unicode names and paths, BOM policy, malformed UTF-8, and stable diagnostic output under Windows/Linux. | `CLI-003` | — |
| `PARSE-009` | TODO | P1 | Required | Document strict versus tolerant parsing for historical configuration | Tests show version fallback rules, absent task map, unexpected join-operator formats and migration precedence; no guessed fallback silently succeeds. | `AUD-001` | — |
| `PARSE-010` | TODO | P1 | Required | Measure field coverage against original types.go not just top-level names | Generate or review a field-by-field coverage matrix for 26 Go struct declarations and all JSON-tagged fields, mapping each to Rust, intentional exclusion or unimplemented release task; tests for every required family. | `MODEL-002` | — |

### State

| ID | Status | Priority | Gate | Work | Acceptance | Depends on | Evidence (first) |
|---|---|---|---|---|---|---|---|
| `STATE-019` | TODO | P0 | Required | Validate Quest ID consistency across wrapper, config and user status | When quest.id, config.id and user_status.quest_id coexist, report conflicting IDs as inconsistent/unknown and do not display a confirmed reward for mismatched records; synthetic tests. | `PARSE-001`, `STATUS-010` | — |
| `STATUS-010` | TODO | P0 | Required | Validate completion/enrollment/claim timestamps instead of nonempty-string truthiness | Validate RFC3339 completion/enrollment/claim timestamps AND per-task progress.completed_at (including timezone offsets); whitespace or malformed strings cannot yield confirmed status. | `STATE-001` | — |
| `STATUS-011` | TODO | P0 | Required | Detect impossible ordering and contradictory status timestamps | Synthetic claim-before-completion, completion-before-enrollment, starts-after-expiry, and invalid clocks yield explicit inconsistent/unknown states. | `STATUS-010` | — |
| `STATUS-012` | TODO | P0 | Required | Avoid silent unexpired treatment of malformed/missing expiration dates | Distinguish missing/invalid expires_at from valid future expiry and display uncertainty in filters/status; tests cover malformed dates. | `STATE-008` | — |
| `STATUS-013` | TODO | P0 | Required | Label provenance and trust for each reported milestone | Clearly separate locally supplied, untrusted JSON fields from authenticated server observations. Never call a completed_at field 'server-verified' unless its provenance is authenticated; report source and trust level on each verdict. | `STATE-007` | — |
| `STATUS-015` | TODO | P0 | Required | Verify AND/OR groups with mixed progress, missing task statuses, and other operators | Property/table tests cover single/AND/OR, unknown operator, unknown/missing task statuses and conditional eligibility without false success. | `STATE-005`, `AUD-001` | — |
| `STATE-001` | DONE | P1 | Required | Port IsExpired/IsEnrolled/IsCompleted/HasClaimedRewards | Methods derive status from server-shaped timestamps and expiry; tests pass. | — | `src/model.rs` |
| `STATE-002` | DONE | P1 | Required | Preserve original GetTaskName as compatibility-only heuristic | Original first-match precedence is represented but never determines entire Quest completion. | — | `src/model.rs` |
| `STATE-003` | DONE | P1 | Required | Handle Android video-only classification | Recognizes mobile-only video task when desktop video is absent. | — | `src/model.rs` |
| `STATE-004` | DONE | P1 | Required | Port FilterValid/FilterRedeemable semantics | Filters noncompleted/nonexpired and completed/unclaimed, respectively; tests pass. | — | `src/manager.rs` |
| `STATE-005` | PARTIAL | P1 | Required | Correctly evaluate AND/OR task groups | Verify AND/OR behavior against authenticated/sanitized real-world Quest schema samples or an authoritative reference; maintain synthetic regression tests and fail closed on missing/unknown operators. | — | `src/manager.rs` |
| `STATE-006` | DONE | P1 | Required | Do not turn numeric progress into completion | Progress >= target without completed_at is flagged, not reported as Quest completed; tests pass. | — | `src/manager.rs` |
| `STATE-009` | DONE | P1 | Required | Account enrollment-block marker remains diagnostic | Blocked-until input retained and displayed; no false claim of enrollment. | — | `src/main.rs` |
| `STATE-020` | TODO | P1 | Required | Retain and interpret task progress freshness metadata | Parse QuestTaskProgress.updated_at and optional heartbeat last_beat_at/expires_at, and report stale/unknown observation time separately from success; do not fake heartbeats. | `PARSE-010`, `STATUS-010` | — |
| `STATE-021` | TODO | P1 | Required | Validate task and progress event name consistency | Parse TaskEntry.type/event_name and QuestTaskProgress.event_name; classify mismatched task reports as unverified/inconsistent, with synthetic tests. | `MODEL-010`, `STATE-020` | — |
| `STATUS-014` | TODO | P1 | Required | Keep unsupported/unknown tasks distinguishable even on completed Quest records | Mixed known/unknown and deprecated tasks retain warnings; Quest completed_at may be reported but never construed as implemented task handler support. | `MODEL-005` | — |
| `STATUS-016` | TODO | P1 | Required | Inject an evaluation clock and add deterministic --at option | Tests/CLI can evaluate a timestamp without Utc::now nondeterminism; DST/UTC offsets and boundary-time cases are covered. | `STATE-008` | — |
| `STATUS-017` | TODO | P1 | Required | Compare Go and Rust filtering behavior for preview, blocking, starts and duplicates | Document identical and intentionally different filter results with named fixtures; no ambiguous parity claim. | `AUD-002`, `PARSE-002` | — |
| `STATUS-018` | TODO | P1 | Required | Model claimable, claimed, and externally redeemed as separate evidence states | Result format and tests differentiate Quest completion, reward claim availability/confirmation, and externally unknown redemption. | `STATE-007`, `MODEL-009` | — |
| `STATE-007` | PARTIAL | P2 | Required | Separate task, Quest, claim, and external redemption | Quest and claim statuses separate; add explicit externally-unverified redemption representation and tests. | — | `src/manager.rs` |
| `STATE-008` | PARTIAL | P2 | Required | Handle starts_at and expires_at edge cases | Future starts and expired are differentiated; malformed/missing expiry must be flagged as unknown with test coverage. | — | `src/manager.rs` |

### Cli

| ID | Status | Priority | Gate | Work | Acceptance | Depends on | Evidence (first) |
|---|---|---|---|---|---|---|---|
| `CLI-001` | DONE | P1 | Required | demo/inspect, JSON and --workers options | Demo and file inspection work with machine-readable output; options present and smoke-tested. | — | `src/main.rs` |
| `CLI-004` | DONE | P1 | Required | Bundle synthetic example and input/output documentation | Synthetic examples and command usage documented; demo smoke-tested. | — | `examples/sample-quests.json` |
| `CLI-002` | TODO | P2 | Required | Add --version and --verbose and structured errors | New flags documented; error codes predictable; no personal tokens required. | `CLI-001` | — |
| `CLI-003` | PARTIAL | P2 | Required | Robust CLI UX for invalid inputs and safe help output | Tests cover missing flag values, unknown flags, non-UTF8 paths, large input and invalid JSON with stable nonzero exit codes. | — | `src/main.rs` |

### Ux

| ID | Status | Priority | Gate | Work | Acceptance | Depends on | Evidence (first) |
|---|---|---|---|---|---|---|---|
| `UX-002` | TODO | P0 | Required | Version JSON output schema and keep stdout machine-readable | --json emits exactly one documented valid JSON result on stdout with schema_version, errors on stderr and deterministic field naming; golden tests pass. | `CLI-001` | — |
| `UX-006` | TODO | P0 | Required | Keep debug logs sanitized and do not echo raw input or secrets | Tests show token-like strings inside malformed JSON/errors cannot be emitted; diagnostic truncation and escaping documented. | `SEC-002` | — |
| `UX-001` | TODO | P1 | Required | Support --version, --verbose, robust --help and CLI test coverage | Stable version/help output and verbose diagnostic mode that does not disclose sensitive data; tests cover invalid flags. | `CLI-002` | — |
| `UX-004` | TODO | P1 | Required | Document stable exit codes for invalid input and execution errors | Typed failures have predictable nonzero exit codes; success/empty report semantics are documented and tested on both platforms. | `CLI-003` | — |
| `UX-005` | TODO | P1 | Required | Test Windows path quoting, Unicode and terminal encoding behavior | Windows smoke tests cover spaces and non-ASCII filenames and output, including bat wrapper if added. | `PARSE-008` | — |
| `UX-003` | TODO | P2 | Optional/excluded | Add optional stdin/pipeline support with bounded input | A documented '-' stdin input works with identical validation to inspect file and rejects oversized streams; no network or token collection. | `PARSE-005` | — |

### Concurrency

| ID | Status | Priority | Gate | Work | Acceptance | Depends on | Evidence (first) |
|---|---|---|---|---|---|---|---|
| `PAR-001` | DONE | P1 | Required | Bounded parallel assessment in Rust | Analysis worker count bounded 1..32 and output preserves source order; unit test passes. | — | `src/manager.rs` |
| `PAR-002` | TODO | P2 | Required | Add cancellation, clean shutdown and race tests | Cancellation paths release tasks/handles, no unbounded waits and tests cover shutdown. | `PAR-001` | — |
| `PAR-003` | TODO | P2 | Optional/excluded | Async runtime only if justified by supported I/O | If authorized network I/O is added, compare Tokio vs existing threads and document lifecycle; do not add for decoration. | `PAR-001` | — |

### Network

| ID | Status | Priority | Gate | Work | Acceptance | Depends on | Evidence (first) |
|---|---|---|---|---|---|---|---|
| `NET-001` | TODO | P2 | Optional/excluded | Provide credential-free HTTP fixture/mock adapter | Fake in-memory responses drive analysis under tests, with no live Discord requests. | `STATE-005` | — |
| `NET-002` | TODO | P2 | Optional/excluded | Bounded retries, timeouts, rate-limit and cancellation under mocks | Synthetic transport tests prove finite retries, deadlines, controlled 429/5xx and explicit error propagation. | `NET-001` | — |
| `NET-003` | TODO | P2 | Optional/excluded | Document real Gateway protocol and local trace parser | Offline parsing of saved/synthetic Gateway frames and heartbeat lifecycle; no user-token Identify. | `MODEL-002` | — |
| `NET-004` | TODO | P2 | Optional/excluded | Officially authorized application integrations only | If any live service integration is proposed, document supported authorization scopes and review against platform terms before building. | `NET-001` | — |

### Tests

| ID | Status | Priority | Gate | Work | Acceptance | Depends on | Evidence (first) |
|---|---|---|---|---|---|---|---|
| `TEST-001` | DONE | P1 | Required | Fifteen synthetic Rust unit tests passing | 15 unit tests passed in GitHub Actions run 41256646888. | — | [source](https://github.com/7ejal/Discord-Quest/actions/runs/41256646888) |
| `TEST-002` | TODO | P1 | Required | Separate integration tests and golden JSON snapshots | Integration and golden tests verify CLI JSON output, operator logic, timestamp problems and fixtures. | `TEST-001` | — |
| `TEST-003` | PARTIAL | P1 | Required | 22 offline method validation cases | Implement 22 specified test cases as executable offline tests, mapping each case to a test name and result. | — | `research/standalone/METHOD-VALIDATION-FIXTURES.md` |
| `TEST-004` | DONE | P1 | Required | Cross-platform Linux/Windows build and smoke tests verified on historical commit | A recorded earlier CI run built and smoke-tested the Rust project on both Linux and Windows. Reproducible locked builds and tagged-current-commit release verification remain REL-006 and REL-002. | — | [source](https://github.com/7ejal/Discord-Quest/actions/runs/41256646888) |
| `TEST-005` | TODO | P1 | Optional/excluded | Coverage, property tests, fuzzing and benchmark numbers | Published measurements reproduce with inputs, machine specs and tools; no fabricated RAM/CPU claims. | `TEST-001` | — |

### Quality

| ID | Status | Priority | Gate | Work | Acceptance | Depends on | Evidence (first) |
|---|---|---|---|---|---|---|---|
| `QA-001` | PARTIAL | P0 | Required | Maintain test-to-requirement traceability for all release-gating tasks | Every release-gate task has test file/case ID or a verified non-code acceptance artifact; CI checks missing test mapping. | `TEST-002` | `migration/TEST-MATRIX.md` |
| `QA-003` | TODO | P0 | Required | Golden CLI snapshots with deterministic fixture time and text/JSON outputs | Committed expected outputs compare identically on Windows/Linux with normalized paths/timezones; fixture changes reviewed. | `STATUS-016`, `UX-002` | — |
| `QA-005` | TODO | P0 | Required | Negative end-to-end test ensures no Discord network, user-token, or reward submission | Offline CLI works with network blocked and denies user-token operation flags; evidence proves no credential requirement. | `SEC-001` | — |
| `QA-002` | TODO | P1 | Required | Property tests and fuzz harness for schema and untrusted JSON | Fuzz/property tests demonstrate no panics and stable unknown/invalid classification with bounded synthetic inputs; CI smoke-level fuzz schedule specified. | `PARSE-005` | — |
| `QA-004` | TODO | P1 | Required | Stress test duplicate IDs, 100k mock tasks, large values and empty lists | Test resource limits, correct ordering, duplicate handling and no panic on synthetic large/empty documents. | `PARSE-005`, `PARSE-002` | — |
| `QA-007` | TODO | P1 | Required | Regression corpus for source-level failures in related Go/Python/TS projects | Each known false-success, infinite retry, stale status and conditional EventEmitter case has a synthetic input and expected safe diagnostic. | `TEST-003` | — |
| `QA-008` | PARTIAL | P1 | Required | Test tracker checker itself against malformed manifests | Unit tests confirm checker rejects duplicate IDs, broken deps/cycles, stale Markdown, invalid statuses and nonexistent path evidence appropriately. | `OPS-002` | `migration/test_check_progress.py` |
| `QA-009` | TODO | P1 | Required | Differential offline Go-versus-Rust fixtures for original pure methods | Use only non-network Go model/Quest/filter helpers and synthetic JSON; compare deterministic outputs with Rust for known parity and document intentional divergences; never execute selfbot login. | `AUD-002`, `STATE-019` | — |
| `QA-010` | TODO | P1 | Required | Propagate worker errors and panic failures without false-success reports | Tests inject synthetic worker error/panic conditions and show no task/Quest can be marked confirmed from partial results; cancellation and thread cleanup behavior explicit. | `PAR-002` | — |
| `QA-006` | TODO | P2 | Optional/excluded | Reproducible CPU/RAM performance and binary-size measurements | Benchmark setup, platform/hardware, time/memory and reproducible input sets published; no speculative improvement claims. | `TEST-005` | — |

### Security

| ID | Status | Priority | Gate | Work | Acceptance | Depends on | Evidence (first) |
|---|---|---|---|---|---|---|---|
| `SEC-003` | TODO | P0 | Required | Sanitize terminal control sequences and Unicode formatting attacks | Escape or visibly render ANSI/OSC controls, newlines, carriage returns and bidi override characters in untrusted Quest names/IDs/errors; tests prove report cannot manipulate terminal output. | `CLI-003` | — |
| `SEC-001` | DONE | P1 | Required | No personal Discord user-token use in current offline app | Current Rust code has no personal-token login or Discord network API calls. | — | `src/main.rs` |
| `SEC-004` | TODO | P1 | Required | Minimize user identifiers and unexpected sensitive content in output | Review output/diagnostics for identifiers or secret-like data from local JSON, add default-redaction and explicit opt-in rules as appropriate, and test JSON/text privacy behavior. | `SEC-003`, `STATUS-013` | — |
| `SEC-002` | TODO | P2 | Required | Add secret-redacting diagnostics and dependency review | Logging never exposes token fragments or confidential headers; dependency licenses reviewed. | `ENV-009` | — |

### Release

| ID | Status | Priority | Gate | Work | Acceptance | Depends on | Evidence (first) |
|---|---|---|---|---|---|---|---|
| `REL-002` | TODO | P0 | Required | Build and smoke test Linux and Windows artifacts for tagged commit | Both platform binaries built on tag, rerun smoke tests against artifacts, record target and checksums. | `ENV-007` | — |
| `REL-007` | TODO | P0 | Required | Document artifact retention and last-verified commit instead of stale EXE links | README and tracker state which Git commit each binary represents, link stable release or note expiry; no latest-success ambiguity. | `ENV-004` | — |
| `REL-008` | DONE | P0 | Required | Create final release readiness checklist with stop conditions | Must-pass checklist covers tests, release source, schema conformance, policy scope, licenses, docs, reproducibility and rollback. | `ENV-008` | `migration/RELEASE-GATES.md` |
| `REL-001` | TODO | P1 | Required | Pin rust-toolchain, MSRV and target architecture policy | rust-toolchain.toml or documented MSRV consistent with CI; linux/windows target triples and version bump policy explicit. | `ENV-005` | — |
| `REL-003` | TODO | P1 | Required | Publish versioned release notes, SHA256 and release assets | A durable GitHub release (not expiring Actions artifacts only) with checksum manifest, notes, limitations and reproducible tag. | `ENV-008` | — |
| `REL-004` | TODO | P1 | Required | Harden CI permissions, action pinning and dependency updates | Set least-privilege permissions, avoid broad tokens, pin actions or document version policy, scope PR workflows and dependabot policy. | `ENV-006` | — |
| `REL-006` | TODO | P1 | Required | Check reproducible locked builds and supply-chain provenance | Build with --locked from clean checkout, checksum input lockfile, document signing/provenance limitation. | `ENV-005`, `REL-001` | — |
| `REL-009` | TODO | P1 | Required | Verify native Windows EXE dependencies on a clean target machine | CI and documentation verify binary launches on a fresh supported Windows runner without installed Go/Node/Python/Discord; identify required DLL/runtime dependencies and target architecture rather than assuming fully static. | `REL-002` | — |
| `REL-005` | TODO | P2 | Optional/excluded | Generate SBOM or dependency/license inventory for release | Catalog all transitive crates and licenses, verify upstream notice obligations, attach inventory to release. | `ENV-009`, `ENV-005` | — |

### Tracking

| ID | Status | Priority | Gate | Work | Acceptance | Depends on | Evidence (first) |
|---|---|---|---|---|---|---|---|
| `OPS-001` | DONE | P0 | Required | Regenerate Markdown task rows automatically from canonical JSON | When a task is added, changed, or removed in progress.json, --write deterministically regenerates all task rows, headings, summaries, and release gates; --check fails mismatches; curated notes belong in separate non-generated documents. | `DOC-004` | `migration/check_progress.py` |
| `OPS-002` | DONE | P0 | Required | Validate expanded tracker schema, dependencies, cycles, and priorities | Checker enforces required fields, unique IDs, permitted statuses/P0-P3, release_gate flags, DAG dependencies and consistent summaries. | `DOC-004` | `migration/check_progress.py` |
| `OPS-003` | TODO | P0 | Required | Evidence must identify tested commit or immutable file and not just a claim | Tracker records verification date, commit/run/source and CI status; checker warns on unverified or outdated 'done' reports. | `OPS-002` | — |
| `OPS-004` | DONE | P0 | Required | Summarize release-gating versus optional tasks separately | Dashboard computes open release blockers from task metadata; excluded and optional tasks do not distort readiness percentage. | `OPS-002` | `migration/check_progress.py` |
| `OPS-006` | DONE | P0 | Required | Provide one copy-paste next-session AI handoff with task order | Session entry names files, branch, actual status, top five dependencies, exact verification commands, and completion reporting format. | `DOC-003` | `migration/START-HERE.md` |
| `OPS-009` | TODO | P0 | Required | Generate a dependency-aware, actually-startable next-task queue | Dashboard distinguishes P0 tasks whose dependencies are DONE from tasks blocked by PARTIAL/TODO prerequisites, lists blocker IDs/status/priority, and tests update the ready queue automatically after manifest changes. | `OPS-002`, `OPS-004` | — |
| `OPS-005` | DONE | P1 | Required | Maintain architecture decision log and unresolved assumptions | ADR file records exact decision/rationale/evidence for source compatibility, offline-only scope, crate choices and stage changes. | `AUD-002` | `migration/DECISIONS.md` |
| `OPS-007` | DONE | P1 | Required | Check markdown links and declared implemented paths in CI | CI catches broken repo-relative links and evidence paths marked as implemented; designed to avoid false failures for proposed paths. | `AUD-004` | `migration/check_links.py` |
| `OPS-008` | DONE | P2 | Optional/excluded | PR/issue template requires task ID, tests, tracker update and residual risk | GitHub templates prompt authors for task IDs, evidence on current commit, docs status, policy scope and pending blockers. | `DOC-003` | `.github/pull_request_template.md` |

### Docs

| ID | Status | Priority | Gate | Work | Acceptance | Depends on | Evidence (first) |
|---|---|---|---|---|---|---|---|
| `DOC-005` | TODO | P0 | Required | Document provenance limits and safe local JSON input workflows | README explains supported offline input forms and how to create/test sanitized synthetic fixtures without scraping or exposing personal user tokens; distinguishes reported data from independently verified server status. | `STATUS-013` | — |
| `DOC-001` | DONE | P1 | Required | Project README documents offline scope and Windows EXE | README explains commands, no live completion, passing prior CI and artifact link. | — | `README.md` |
| `DOC-002` | DONE | P1 | Required | Publish master 1:1 migration spec, file map, tracker | Docs contain exact scoped goals and all Go source files; machine tracker has statuses, acceptance and evidence. | — | `migration/SPEC.md` |
| `DOC-003` | DONE | P1 | Required | AI agent updating protocol | Agent guidance requires rechecking CI, modifying tracker with code changes, and no false completion claims. | — | `AGENTS.md` |
| `DOC-004` | DONE | P1 | Required | Keep progress manifest and Markdown view synchronized | A local checker or generator validates IDs/statuses/dependencies and summary counts; CI runs it. | `DOC-002` | `migration/check_progress.py` |
| `DOC-006` | TODO | P2 | Optional/excluded | Document installation, common CLI errors and Windows troubleshooting | Published guide covers native EXE/OS architectures, trusted release verification, missing file, malformed input, invalid flags, Unicode paths and artifact expiry without promising live Discord functionality. | `REL-007`, `UX-004` | — |

### Excluded

| ID | Status | Priority | Gate | Work | Acceptance | Depends on | Evidence (first) |
|---|---|---|---|---|---|---|---|
| `SCOPE-001` | EXCLUDED | P3 | Optional/excluded | Personal-account token selfbot and client impersonation | Do not implement; current project is an authorized/offline analytics client. Preserve conceptual source inventory only. | — | — |
| `SCOPE-002` | EXCLUDED | P3 | Optional/excluded | Fabricated video/game/stream/activity progress | Do not implement engagement spoofing; categorize prerequisites and interpret server-shaped reports only. | — | — |
| `SCOPE-003` | EXCLUDED | P3 | Optional/excluded | Achievement OAuth reward simulation and challenge solving | No bypass of account verification, publisher achievements or reward claiming; keep documentation and data types. | — | — |
| `SCOPE-004` | EXCLUDED | P3 | Optional/excluded | Live Quest reward-completion and all-games guarantees | No universal 1:1 Quest selfbot acceptance claim; authentic external actions remain outside this project. | — | — |

## Evidence caveat and AI update contract

- The historical [successful Linux/Windows build](https://github.com/7ejal/Discord-Quest/actions/runs/41256646888) verified 15 Rust tests and a Windows smoke test **only for its head commit**.
- Task status is not a claim of current CI status, performance, or universal gameplay.
- For a change, select IDs; implement + test; record current-commit evidence in JSON; regenerate this file; rerun CI; report remaining gaps.
- Review [START-HERE.md](START-HERE.md), [GAP-AUDIT.md](GAP-AUDIT.md), [GO-FILE-MAP.md](GO-FILE-MAP.md), [DECISIONS.md](DECISIONS.md), [SPEC.md](SPEC.md), and [CAPABILITY-CONTRACT.md](CAPABILITY-CONTRACT.md).
