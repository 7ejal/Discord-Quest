# Main-server verification — deterministic CLI timestamp mode

**Date:** November 1, 2026. **Source SHA:** `ec156d5ac87ce4c678c479da440b12f374db1c1e`. **Environment:** user-connected `main-server`, Linux x86_64, Rust/Cargo 1.98.1. The source was fast-forward imported from `main` into isolated `/root/quest-rust-ci-20261101` without touching production services.

## Verified on the main server

- `cargo fmt --all -- --check` — **passed**.
- `CARGO_BUILD_JOBS=2 cargo clippy --all-targets --locked -- -D warnings` — **passed**.
- `CARGO_BUILD_JOBS=2 cargo test --all-targets --locked` — **38 tests passed**: 32 unit + 6 native CLI integration, zero failures.
- `cargo run --release --locked -- demo --at 2026-11-01T12:00:00Z --json` — **passed** and parsed as JSON; `evaluated_at` matched requested time.

## New features verified

- Native `--version` outputs package name/version and exits successfully.
- `--at <RFC3339 timestamp>` accepts an offset-aware evaluation clock and evaluates Quest expiration deterministically.
- Exactly at the expiry timestamp, verdict becomes `expired`. An instant before remains not expired. `2030-01-01T04:00:00+04:00` equals midnight UTC.
- Invalid `--at` values return failure with a fixed error message and do not echo potentially sensitive input.
- JSON output includes `evaluated_at`; input remains explicitly labeled `unverified_local_json`.

All tests use synthetic offline fixtures; no real Discord session, user token, game activity, or reward-claim operation was involved. The validation covers **only the code SHA above**. Confirm matching Windows CI separately before marking a packaged release ready.

## Reproducing

From `rust/go-quest-port` in the isolated GitHub checkout:

```sh
cargo fmt --all -- --check
CARGO_BUILD_JOBS=2 cargo clippy --all-targets --locked -- -D warnings
CARGO_BUILD_JOBS=2 cargo test --all-targets --locked
cargo run --release --locked -- demo --at 2026-11-01T12:00:00Z --json
```
