# Test matrix and evidence contract

**This is a test plan, not a list of passing tests.** Use [progress.json](progress.json) for actual completion statuses and linked test evidence; use [GAP-AUDIT.md](GAP-AUDIT.md) for concrete current-source failures.

## What has actually passed

The [historical Rust CI run](https://github.com/7ejal/Discord-Quest/actions/runs/41256646888) passed **15 synthetic Rust unit tests**, Linux CLI smoke test, Windows release build and Windows CLI smoke test for the commit it checked. This does **not** validate new parser, security, release or source-parity requirements.

The Go original's real-world reward completion and all-games coverage remain unverified; none of these tests engages Discord.

## Core high-priority cases (all require current-commit checks)

| ID | Requirement task | Test input | Correct expected result |
|---|---|---|---|
| T01 | PARSE-001 | `{}`, missing/blank ID, empty config | Invalid/unknown data, not a normal supported Quest |
| T02 | PARSE-002 | Duplicate ID with identical records | Deterministic deduplication per published policy |
| T03 | PARSE-002 | Duplicate ID with conflicting claimed/completed states | Explicit conflict or specified deterministic policy, not double-count |
| T04 | PARSE-003 | `excluded_quests` metadata alongside full Quests | Preserve excluded metadata without pretending config complete |
| T05 | PARSE-005 | Input exceeding chosen byte size | Controlled error without reading entire payload |
| T06 | PARSE-005 | Deeply nested and huge task maps | Resource limits, no crash/excessive memory |
| T07 | PARSE-006 | `target=0`, negative, overflow, null, unrealistic huge | Explicit invalid/unknown; no completed inference |
| T08 | PARSE-007 | Missing vs null vs wrong-type `user_status` | Distinct diagnostic; not silently successful |
| T09 | PARSE-008 | UTF-8 game name, Windows path with spaces/non-ASCII | Correct parse and clean output |
| T10 | PARSE-009 | Both legacy and v2 config, mismatch | Documented precedence; warnings for unsupported structures |
| T11 | STATUS-010 | `completed_at="garbage"` | **Not confirmed**; invalid timestamp diagnostic |
| T12 | STATUS-010 | RFC3339 timestamp with timezone offset | Recognized valid UTC-equivalent instant |
| T13 | STATUS-011 | `claimed_at` earlier than `completed_at` | Inconsistent/suspicious record |
| T14 | STATUS-011 | `completed_at` earlier than `enrolled_at` | Inconsistent/suspicious record |
| T15 | STATUS-012 | Invalid `expires_at` string | Expiry unknown, not guaranteed unexpired |
| T16 | STATUS-012 | Absent `expires_at` | Explicit unknown/unspecified expiry |
| T17 | STATUS-015 | Two-task AND, one completed | Task group incomplete |
| T18 | STATUS-015 | Two-task AND, both completed, Quest `completed_at` null | Tasks satisfied but **Quest unconfirmed** |
| T19 | STATUS-015 | Two-task OR, one completed, Quest `completed_at` null | Group satisfied but **Quest unconfirmed** |
| T20 | STATUS-015 | Unknown operator or incompatible schema | Unsupported, not guessed complete |
| T21 | STATUS-014 | Mixed unknown/known task, Quest `completed_at` present | Server-shaped Quest state visible; no task-handler support claim |
| T22 | STATUS-016 | Expiry boundary with fixed `--at` | Deterministic exact result |
| T23 | STATUS-017 | Go map dedup and block marker vs Rust reader | Intentional difference recorded in parity table |
| T24 | STATUS-018 | `completed_at` without `claimed_at` | Quest completed, reward not claimed |
| T25 | STATUS-018 | `claimed_at` without `completed_at` | Inconsistent |
| T26 | UX-002 | `--json` with warnings or no Quests | One valid versioned JSON document on stdout |
| T27 | UX-004 | Bad flag, invalid input, nonexistent file | Nonzero documented exit code and sanitized stderr |
| T28 | UX-005 | Native Windows Unicode path and output | Works without codepage corruption |
| T29 | UX-006 | Token-like data in malformed fixture | No sensitive substring in logs/errors |
| T30 | QA-005 | CLI executed with network unavailable | All offline commands still function |
| T31 | QA-004 | 100k synthetic tasks under budget | Deterministic correct count and no uncontrolled allocation |
| T32 | QA-003 | Golden text + JSON CLI output | Exact match, no current-time drift |
| T33 | QA-002 | Arbitrary sanitized JSON fuzz inputs | No panic or false success within defined resource bounds |
| T34 | OPS-002 | Cyclic or missing tracker dependencies | Checker fails before CI tests |
| T35 | OPS-001 | Add new task ID to tracker JSON only | `--check` fails; `--write` adds visible row |
| T36 | REL-006 | Clean checkout with committed lockfile | `cargo build --locked --release` passes |
| T37 | REL-002 | Tagged commit on Windows + Linux | Artifact smoke test passes, checksum matches |
| T38 | AUD-005 | Task catalog vs Rust enum/classifier | Full recognized-task agreement or reviewed exception |

## Additional historical 22-case research suite

[Existing research fixture plan](../../../research/standalone/METHOD-VALIDATION-FIXTURES.md) covers F01–F22, including repeated 429, stalled progress, conditional streaming/console prerequisites, OAuth cleanup and error-event propagation. Some examples are **source-architecture comparisons only**, not required network implementation tests. They must be mapped explicitly to executable mock tests or justified as N/A to the approved offline app before marking `TEST-003` done.

## Test completion definition

For each test case record:
- Source file and test function name.
- Task ID(s) validated.
- Deterministic fixture location and expected output.
- Exact command and observed exit result.
- Relevant code commit hash and GitHub Actions run URL.
- Any caveat: mocked/offline, not a real Discord Quest.
- One-to-one mapping in `progress.json` evidence before declaring associated work `done`.

No new test may rely on real personal-account user tokens, simulated rewarded activity, provider exploits or CAPTCHA bypass.
