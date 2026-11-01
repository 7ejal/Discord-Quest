# Linux CLI: network-isolated release verification — 2026-11-01

**Tested Git SHA:** `44dfcf924ec805729d1ac800c3294d344bce588e` (source, not any later documentation-only commit).  
**Server:** user-connected `main-server`, Linux x86_64, Rust 1.98.1.  
**Isolated working copy:** `/root/quest-rust-finish-audit-20261101`. The existing `/root/quest-rust-ci-20261101` directory and production services were not modified.

## Exact checks on main-server

Executed `CARGO_BUILD_JOBS=2 nice -n 10 bash scripts/test-linux.sh`, and then `CARGO_BUILD_JOBS=2 nice -n 10 bash scripts/package-linux.sh` inside `rust/go-quest-port`.

- Progress manifest validation: **passed** (127 tracked tasks before the two task updates).
- Tracker unit tests: **17 passed**.
- Local documentation links: **24 documents checked**.
- `cargo fmt --all -- --check`: **passed**.
- `cargo clippy --all-targets --locked -- -D warnings`: **passed**.
- `cargo test --all-targets --locked`: **66 Rust tests passed** (36 library, 3 golden, 8 CLI safety, 4 stdin, 4 excluded, 10 model parity, **1 task-catalog contract**).
- Linux `cargo build --release --locked`: **passed**.
- Golden JSON and excluded-metadata privacy smoke tests: **passed**.
- **Isolated network namespace:** `unshare --net --` ran the release binary with zero network interfaces/routes available; the golden JSON result matched and excluded Quest JSON remained parseable. Environment canaries `TOKEN` and `YES_CAPTCHA_API_KEY` were not emitted, and synthetic targeting/traffic secrets stayed absent from stdout.
- The test script fails if `unshare --net` is unavailable; it does not silently skip the negative network test. This is an offline execution check, **not** a dynamic proof that no code could ever open a socket if modified later.
- The new synthetic task-catalog contract test reads `data/task-types.json`, matches **10** known task identifiers, normalized Rust task families, the deprecated `PLAY_ON_DESKTOP_V2` flag, and an unknown-type fallback.

## Linux binary and archive

Generated on the tested main-server checkout:

```text
/root/quest-rust-finish-audit-20261101/rust/go-quest-port/dist/linux-x86_64/go-quest-port-0.1.0-linux-x86_64-44dfcf924ec8.tar.gz
```

**SHA256:** `21e53ea165b11f7c70e27f5bcb13020ee617506c134c1a358883d1b1d173d85e`.  
**Archive size:** 356,612 bytes.  
**Runtime:** native Linux x86_64 binary with system libc and libgcc_s dynamic dependencies. No Go, Node, GUI, Discord installation or account credentials required.

A local archive on `main-server` is **not** a permanent public GitHub release. Never cite an ephemeral transfer as durable publication. The original GO source's personal-token selfbot/engagement spoofing/CAPTCHA/reward claims remain excluded, and a user-supplied JSON file is always unverified data.

## Still open

The canonical [migration tracker](progress.json) continues to list schema fidelity, optional model metadata, robust error-code contract, stress cases, evidence provenance and durable Linux release publication. Mark only `AUD-005` and `QA-005` as completed from these new checks; do not turn the whole Linux port into “100% complete” on the strength of passing tests.
