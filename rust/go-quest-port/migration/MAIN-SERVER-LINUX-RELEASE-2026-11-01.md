# Linux x86_64 main-server release-candidate verification — 2026-11-01

**Active scope:** CLI-only, offline/read-only, Linux x86_64 only. **Tests and release packaging were executed exclusively on the connected `main-server`**. No Windows tests, hosted GitHub Actions, personal Discord account access, production-service changes or GUI work took place.

**Isolated test checkout:** `/root/quest-rust-ci-20261101`  
**Crate directory:** `/root/quest-rust-ci-20261101/rust/go-quest-port`  
**Verified source commit:** `dcc748abe0ccd43b107c2de4350403005f181a88`  
**Compiler:** `rustc 1.98.1` · Linux kernel 6.12.100+deb13-amd64 · x86_64  
**Build execution:** `CARGO_BUILD_JOBS=2 nice -n 10 bash scripts/package-linux.sh`

## Verification outcome

- **65 Rust tests:** passed (36 library, 3 golden snapshots, 8 CLI safety, 4 stdin, 4 excluded Quest and 10 Go model parity).
- **17 Python tracker tests:** passed.
- **Local link check, manifest consistency, `cargo fmt --check`, locked Cargo/Clippy `-D warnings`:** passed.
- **Linux release CLI:** built with `cargo build --release --locked` and passed JSON/golden/report privacy smoke tests.
- **Archive extraction:** packed tarball extracted in a fresh temp directory; all included SHA256SUMS entries verified; extracted binary ran successfully.
- **Dependency report:** includes pinned `Cargo.lock` name/version inventory; complete third-party license review is still separately tracked.
- **Installed services:** no changes; build/checks used the dedicated test checkout.

## Generated Linux-only archive (server-resident release candidate)

```text
/root/quest-rust-ci-20261101/rust/go-quest-port/dist/linux-x86_64/
  go-quest-port-0.1.0-linux-x86_64-dcc748abe0cc.tar.gz
  go-quest-port-0.1.0-linux-x86_64-dcc748abe0cc.tar.gz.sha256
```

**Tarball SHA256:** `e5833dac0ec29f0782b039802a9cdd5e3349381b128fcb256ba629e1104aed13`.

**Contents:** `go-quest-port` native Linux executable, `README.md`, upstream MIT `LICENSE`, `NOTICE`, `BUILD-INFO.txt` with exact Git SHA and `Cargo.lock` hash, `LOCKED-DEPENDENCIES.txt`, `SHA256SUMS`.

**Dynamic runtime dependencies reported by `ldd`:** `libgcc_s.so.1`, `libc.so.6`, and the ordinary Linux loader. The executable is native Rust and does **not** require Go, Python or Discord to run; it is **not** fully statically linked.

**Publication caveat:** This archive is stored on main-server, **not** a permanent public GitHub release. Any Gateway download link is temporary; task REL-003 remains open until an actual durable distribution link and release record exist.

## Measured synthetic Linux performance

Script: [`scripts/benchmark-linux.py`](../scripts/benchmark-linux.py), using fixed `--at 2026-11-01T12:00:00Z`, four evaluation workers, subprocess `nice +10`, 60 CPU-second limit and 512 MiB virtual-memory cap. No network calls or credentials.

| Input size | Input JSON bytes | Wall time | CPU time | Max child RSS | Result |
|---|---:|---:|---:|---:|---|
| 100 fictional Quests | 18,002 | 2.67 ms | 2.72 ms | 9,984 KiB | Accepted |
| 10,000 fictional Quests | 1,818,902 | 93.88 ms | 105.10 ms | 104,444 KiB | Accepted |
| 100,001 fictional Quests | 18,289,086 | 8.79 ms | 8.35 ms | 10,208 KiB | Correct rejection |

**Linux executable size measured by the benchmark:** 745,192 bytes. Resource values are from **one sample per scenario** on this specific server; cache state, scheduling, input structure, and compiler settings can affect them. These are not a service SLA, performance guarantee or proof of gameplay/Quest completion.

## Remaining work

[Canonical task tracker](progress.json) describes unfinished **supported Linux CLI** items: source-model fidelity, provenance and input-hardening improvements, negative/offline tests, error UX, third-party license review and durable release distribution. Excluded personal-account selfbot/engagement-spoofing tasks are not attempted.

**Historical Windows/GitHub Actions runs are not current validation** after the explicit [Linux-only decision](LINUX-ONLY.md).
