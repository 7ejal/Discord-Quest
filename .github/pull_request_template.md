## Change summary

Describe the concrete change. Do not describe code-only Quest classification as actual rewarded engagement.

**Canonical task IDs:** (required: e.g., `PARSE-001`)

**Approved-scope / exclusions check:** Native offline Quest analysis only; no personal user-token selfbot, simulated rewarded engagement, challenge solving or fake reward claims.

## What changed

- Go source method/field and pinned commit:
- Rust files changed:
- Behavior before:
- Behavior after:
- Intentional Go parity deviation:
- New/updated fixtures and test IDs:

## Verification

- [ ] `python3 migration/check_progress.py --check` from `rust/go-quest-port`
- [ ] `python3 migration/test_check_progress.py`
- [ ] `python3 migration/check_links.py`
- [ ] `cargo test --all-targets`
- [ ] `cargo run --release -- demo --json`
- [ ] `cargo build --release`
- [ ] Relevant Windows smoke test
- [ ] Code format and Clippy (when integrated into CI)
- [ ] CI checked **this change's commit**, not only a historical run

**Actual test commands and observed results:**

**Latest CI commit SHA / run URL:**

## Tracker and documentation

- [ ] Added or updated task IDs with `status`, `acceptance`, `priority`, `required_for_release`, dependencies and evidence in `migration/progress.json`.
- [ ] Regenerated `migration/PROGRESS.md` via `--write` (no manual row edits).
- [ ] Updated source function map `migration/GO-FILE-MAP.md` if function coverage changed.
- [ ] Updated `migration/DECISIONS.md` when behavior/scope decisions changed.
- [ ] Updated user README and limitations if relevant.
- [ ] Maintained original license notices if code was reused.

## Remaining work and risks

Open release gates and linked IDs:

Known edge cases or unresolved verification:
