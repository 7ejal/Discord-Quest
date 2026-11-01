# Mandatory main-server validation after Rust code edits

**Purpose:** The user's requested engineering workflow is **GitHub source first → import into isolated main-server checkout → run all tests on main-server → package Linux x86_64 only**. No automatic deployment is implied.

## Approved execution target

- Gateway server selector: **`main-server`** (the connected, user-authorized Linux server).
- Separate working checkout: `/root/quest-rust-ci-20261101`.
- Crate working directory: `/root/quest-rust-ci-20261101/rust/go-quest-port`.
- Server already has Git and Cargo/Rust available. **Do not install new system packages or restart any production services.**
- Compile with `CARGO_BUILD_JOBS=2` and `nice -n 10` to reduce interference with production.
- Keep output/logs separate from production. Never print account tokens, process environments, or private configuration.
- **Do not run untrusted pull-request code or arbitrary user-supplied scripts on a production machine.** Test reviewed/committed `main` revisions only.
- Use the Gateway **git_clone/git_pull** typed operations to import source, **execute_command** for build/test commands. Confirm `git status` and refuse destructive updates when unrelated changes are present.

**Current override:** [LINUX-ONLY.md](LINUX-ONLY.md) is authoritative over historical Windows and GitHub Actions references elsewhere. No GitHub hosted or Windows test workflow is active.

## Required steps following a Rust code commit

1. Commit source and unit/regression tests to `7ejal/Discord-Quest` branch `main` via authorized GitHub tooling.
2. Ensure the checkout at `/root/quest-rust-ci-20261101` is a dedicated test clone. Create it via Gateway git_clone only if missing. Do not overwrite another user's local changes.
3. Use Gateway git_pull in fast-forward-only mode on the test checkout, then record `git rev-parse HEAD`.
4. Run the following in `rust/go-quest-port` on **main-server**:

```sh
set -euo pipefail
export CARGO_BUILD_JOBS=2
CARGO_BUILD_JOBS=2 nice -n 10 bash scripts/test-linux.sh
```

5. Run `CARGO_BUILD_JOBS=2 nice -n 10 bash scripts/package-linux.sh` to build, checksum and smoke-test the Linux x86_64 release candidate locally. Hosted GitHub Actions is disabled; main-server is the only validation host.
6. Record tests performed, exact commit, run URL, failed checks and unverified scope in a durable [main-server validation log](MAIN-SERVER-CI-2026-11-01.md) or new dated log. If a check fails, fix code on GitHub, pull the new revision, and rerun.
7. Update [progress.json](progress.json) evidence/status only after acceptance passes, regenerate [PROGRESS.md](PROGRESS.md), then rerun the tracker/CI checks on the updated commit.

## Important implementation and security boundaries

This testing is for the **offline native Rust CLI**. It never needs a personal Discord user token, a Discord client, fabricated Quest activity or a CAPTCHA solver. The main-server checkout is only a **test workspace**, not a persistent daemon, a live Discord integration, or an auto-deployed service.

If `main-server` is unreachable, lacks safe resources, or has uncommitted unrelated changes, **do not silently skip the step or fabricate success**. Report the exact reason and report the block and leave main-server verification incomplete; do not claim historical GitHub Actions results apply.

This procedure must not be misinterpreted as authority to run commands indefinitely or schedule background jobs without an explicit automation request.

## Verification records

[Continuation record](MAIN-SERVER-COMPLETION-2026-11-01.md) identifies current tested source and artifact details. Earlier records are historical and do not verify later code. Tests include the stdlib-only pinned pure-Go harness; the executable itself has no Go/Python runtime requirement.
