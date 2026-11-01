# Mandatory main-server validation after Rust code edits

**Purpose:** The user's requested engineering workflow is **GitHub source first → import into isolated main-server checkout → run all tests on main-server → verify Windows GitHub Actions**. No automatic deployment is implied.

## Approved execution target

- Gateway server selector: **`main-server`** (the connected, user-authorized Linux server).
- Separate working checkout: `/root/quest-rust-ci-20261101`.
- Crate working directory: `/root/quest-rust-ci-20261101/rust/go-quest-port`.
- Server already has Git and Cargo/Rust available. **Do not install new system packages or restart any production services.**
- Compile with `CARGO_BUILD_JOBS=2` and `nice -n 10` to reduce interference with production.
- Keep output/logs separate from production. Never print account tokens, process environments, or private configuration.
- **Do not run untrusted pull-request code or arbitrary user-supplied scripts on a production machine.** Test reviewed/committed `main` revisions only.
- Use the Gateway **git_clone/git_pull** typed operations to import source, **execute_command** for build/test commands. Confirm `git status` and refuse destructive updates when unrelated changes are present.

## Required steps following a Rust code commit

1. Commit source and unit/regression tests to `7ejal/Discord-Quest` branch `main` via authorized GitHub tooling.
2. Ensure the checkout at `/root/quest-rust-ci-20261101` is a dedicated test clone. Create it via Gateway git_clone only if missing. Do not overwrite another user's local changes.
3. Use Gateway git_pull in fast-forward-only mode on the test checkout, then record `git rev-parse HEAD`.
4. Run the following in `rust/go-quest-port` on **main-server**:

```sh
set -euo pipefail
export CARGO_BUILD_JOBS=2
python3 migration/check_progress.py --check
python3 migration/test_check_progress.py
python3 migration/check_links.py
cargo fmt --all -- --check
nice -n 10 cargo clippy --all-targets --locked -- -D warnings
nice -n 10 cargo test --all-targets --locked
nice -n 10 cargo run --release --locked -- demo --json > /tmp/quest-rust-ci-demo.json
python3 -m json.tool /tmp/quest-rust-ci-demo.json >/dev/null
```

5. Check that GitHub Actions' **Linux and Windows** build/test/smoke run passes at the relevant commit. Linux validation on main-server does not replace Windows-native validation.
6. Record tests performed, exact commit, run URL, failed checks and unverified scope in a durable [main-server validation log](MAIN-SERVER-CI-2026-11-01.md) or new dated log. If a check fails, fix code on GitHub, pull the new revision, and rerun.
7. Update [progress.json](progress.json) evidence/status only after acceptance passes, regenerate [PROGRESS.md](PROGRESS.md), then rerun the tracker/CI checks on the updated commit.

## Important implementation and security boundaries

This testing is for the **offline native Rust CLI**. It never needs a personal Discord user token, a Discord client, fabricated Quest activity or a CAPTCHA solver. The main-server checkout is only a **test workspace**, not a persistent daemon, a live Discord integration, or an auto-deployed service.

If `main-server` is unreachable, lacks safe resources, or has uncommitted unrelated changes, **do not silently skip the step or fabricate success**. Report the exact reason and use GitHub Actions as available, marking main-server verification as not performed.

This procedure must not be misinterpreted as authority to run commands indefinitely or schedule background jobs without an explicit automation request.

## Last verified run

- Main-server verified Rust code commit `36e386b7f0d4ddef6f3743f997ec3bc5830e525c` with 23 Rust tests and 14 Python tracker tests.
- A subsequent main-server run on documentation/tracker commit `2a73153d8ed6da46aafde51a7c87ef1b9f0fe134` also passed all 23 Rust tests, formatting, Clippy, tracker checks and the release JSON smoke test.
- [Matching successful GitHub Actions run](https://github.com/7ejal/Discord-Quest/actions/runs/41270547366).

**This document defines a reproducible test procedure, not an ongoing automation or completed universal Go-port.**
