# Main-server CI — Go model fidelity and excluded Quest support

**Date:** 2026-11-01  
**Git commit tested:** `5043f64b141b03917a14976c109c9e60a6562903`  
**Server:** connected Gateway `main-server`, isolated checkout `/root/quest-rust-ci-20261101`. No production services were modified, installed or restarted.  
**Cross-platform verification:** [successful Linux and Windows GitHub Actions run #41275135541](https://github.com/7ejal/Discord-Quest/actions/runs/41275135541).  
**Upstream Go baseline:** [`yeweroooo` commit `352d7f4`](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/tree/352d7f457c9511bc9e7c9b940a71012bfb47572e).

## Implemented in this Rust CLI batch

- **TaskEntry.type**: parse Go task type, compare nonempty declarations to the task-map key, and mark contradictory or unknown declarations inconsistent rather than trusting their claimed completion.
- **Application IDs**: skip blank/whitespace per-task application identifiers, warn about them and fall back to the nonblank campaign ID where needed.
- **QuestReward / QuestRewardsConfig**: add assignment method, expiry, platforms, SKU, image/video assets, name metadata and null-safe optional reward messages; explicitly identify orb-quantity fields as source extensions instead of attributes of pinned Go QuestReward.
- **Task progress**: retain event name and updated_at, optional heartbeat last_beat_at and expires_at, warn on invalid dates and expired reported heartbeat metadata. Contradictory task/report event names are flagged inconsistent.
- **Excluded Quests**: preserve `excluded_quests` separately from eligible full Quest records, retain optional partial metadata internally, deduplicate identical partial records, reject conflicting IDs, and emit only limited ID/preview/config-present/status-present summaries.
- **Privacy**: never include raw `targeted_content` or `traffic_metadata_raw/sealed` in CLI output. Explicit synthetic secret-leak tests exercise text and JSON reporting.
- **No change to platform scope**: offline analysis only; no user-token login, activity fabrication, fake stream/game participation or reward submissions.

## Verified on main-server

Working directory: `/root/quest-rust-ci-20261101/rust/go-quest-port`.

| Check | Observed result |
|---|---|
| `python3 migration/check_progress.py --check` | Passed before this tracker update |
| `python3 migration/test_check_progress.py` | **17 Python tests passed** |
| `python3 migration/check_links.py` | **20 active documentation files checked** |
| `cargo fmt --all -- --check` | Passed |
| `cargo clippy --all-targets --locked -- -D warnings` | Passed, no warnings |
| `cargo test --all-targets --locked` | **52 Rust tests passed** (32 library, 6 CLI, 4 excluded-Quest, 10 model parity) |
| `cargo run --release --locked -- inspect tests/fixtures/excluded_demo.json --json --at 2026-11-01T12:00:00Z` | Passed; JSON parse, 1 full + 1 excluded, and no synthetic targeting secrets leaked |
| GitHub Actions Linux and Windows | Both jobs succeeded on tested commit |

All tests use synthetic JSON. The input is explicitly `unverified_local_json` and does not prove authentic Discord server credit or external Quest completion.

## Remaining scope limitations

- The broader Go `types.go` field-by-field matrix, full future-schema compatibility and AND/OR wire format remain unverified.
- A locally supplied `completed_at` timestamp, even when well formatted, is **not authenticated server evidence**.
- Deep nesting and output-size limits beyond the current 8 MiB file / 10,000 Quest count remain tracked.
- Other planned tests, release pipelines, UI features and security reviews remain open in [progress.json](progress.json).
- No operational user-account selfbot, simulated rewarded engagement, CAPTCHA solving or automatic reward claiming was implemented.

This is a source-scoped test record; later commits need their own CI and, for code changes, main-server validation.
