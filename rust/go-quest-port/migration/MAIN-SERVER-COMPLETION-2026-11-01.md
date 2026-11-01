# Linux continuation verification — 2026-11-01

Target: main-server, isolated `/root/quest-rust-ci-20261101`.
Scope: offline Linux x86_64 CLI; production services untouched.

## Regression evidence

- `f98730e41054a4c4cbb1dd7922a67504ca38af37`: regression-only commit, 11 tests,
  6 passed and 5 failed. The failures exposed unknown single-task operators,
  future completion timestamps, malformed claims, invalid numeric goals/progress
  and unrecognized status objects.
- `ac1f2fa9240b3f775e5bf2852239071332145a20`: all 98 Rust tests passed on
  main-server using `cargo test --all-targets --locked --no-fail-fast -- --test-threads=2`.
- `497e634cdf07f9117f1d3e65e2d1f8a0e4226634`: 23 tracker tests and strict
  Clippy passed; subsequent application-provenance source was tested separately.
- `863aab7dfb3ab91cedc92b51464b03d9b5149671`: final reviewed package source. All 99 Rust tests,
  23 tracker tests, formatting, strict Clippy, locked optimized build, golden
  CLI reports and process-local network-isolation checks passed. The extracted
  executable and every archived file hash passed validation.
- Gateway packaging job: `job_01M6BDN67A00GM33D7NPA1Q9SE`. No hosted or Windows runner.
- Release binary source is unchanged from the measured `0fc6a4c4aad6` binary;
  license-only packaging updates add notices without changing the executable.

## Limits on conclusions

These are synthetic/offline checks, not live Discord validation. Modern
authenticated task-group samples are unavailable (`AUD-001`, `STATE-005`).
Public GitHub release publication has not occurred (`REL-003`).
No Windows/GUI, personal-account login, fake engagement, CAPTCHA bypass,
network transport, OAuth grant or reward submission was implemented.

## Verified Linux artifact

- Source commit: `863aab7dfb3ab91cedc92b51464b03d9b5149671`.
- Archive: `/root/quest-rust-ci-20261101/rust/go-quest-port/dist/linux-x86_64/go-quest-port-0.1.0-linux-x86_64-863aab7dfb3a.tar.gz`.
- Archive SHA256: `bc222238e564949c103ef6e7c0f79dd7ae6d3f57735c1d6b6781d8c6f666d7d8`.
- Executable: `/root/quest-rust-ci-20261101/rust/go-quest-port/target/release/go-quest-port`.
- Binary SHA256: `a4b91229b616bda3edcbb335a240da236310658cb94f200047d48420ddb71df6`.
- Binary size: 713,128 bytes. ELF64 x86_64 PIE, stripped, optimized with thin LTO
  and one codegen unit.
- Cargo.lock SHA256: `89e5a3f96855f953715798b4e4160dde1e7eedf78a4e78e9d0976fb3a72f106f`.
- Rust: `rustc 1.98.1 (48a229cea 2026-09-01)`, target
  `x86_64-unknown-linux-gnu`; `RUSTUP_AUTO_INSTALL=0`.
- Shared runtime dependencies: Linux ELF loader, libc and libgcc_s. Host glibc is
  2.41; the executable references GLIBC symbols up to 2.34. No claim of testing
  a clean older distribution or static linking.
- Package includes LICENSE/NOTICE, Cargo.lock, BUILD-INFO, locked dependency
  inventory, SPDX license inventory, individual dependency license/notice texts
  (including Unicode notices), and hashes checked after extraction.
- Private server-side candidate only. No public release tag/download claimed.

## Reproduction

```bash
cd /root/quest-rust-ci-20261101/rust/go-quest-port
CARGO_BUILD_JOBS=2 nice -n 10 bash scripts/test-linux.sh
CARGO_BUILD_JOBS=2 nice -n 10 bash scripts/package-linux.sh
nice -n 10 python3 scripts/benchmark-linux.py --quests 100
nice -n 10 python3 scripts/benchmark-linux.py --quests 10000
nice -n 10 python3 scripts/benchmark-linux.py --quests 100001
```

The benchmark is the existing repository helper, not a runtime requirement.

## Measured resource evidence

Gateway job: `job_01M6BDG4PQ00KPNHK6DNCN9G9C`, same executable hash as the final package.
Shared host: AMD Ryzen 9 5900X, 24 logical CPUs, 131,829,412 KiB RAM. These are
single synthetic runs on a busy shared server, not isolated statistical latency
benchmarks. Each child is capped at 512 MiB address space, 60 CPU seconds and a
90-second wall timeout; synthetic cases execute sequentially.

| Quest records | Input bytes | Result | Wall ms | CPU ms | Peak child RSS KiB |
|---:|---:|---|---:|---:|---:|
| 100 | 18,002 | Accepted | 2.86 | 2.96 | 9,836 |
| 10,000 | 1,818,902 | Accepted | 141.02 | 156.07 | 122,748 |
| 100,001 | 18,289,086 | Safely rejected by byte cap, exit 65 | 8.40 | 7.94 | 10,232 |

The independent Rust resource test constructs a 100,001-entry task map within
the byte cap and verifies rejection by the task-entry limit. Depth, Quest count,
duplicate/conflicting records, empty lists, output caps and fixed-seed mutations
are separate executable checks.

## Tracker disposition

43 task status transitions in this continuation. Final inventory:
**111 done, 2 partial, 7 todo, 7 excluded**. Required release gates:
**100 done of 103**. Remaining required gates are `AUD-001`, `STATE-005`
(modern authenticated task-group schema evidence) and `REL-003` (public
publication). Six other TODO items are optional/deferred: `ENV-008`,
`PAR-003`, `NET-001`, `NET-002`, `NET-003`, `NET-004`.

Completed code covers offline inspection. The two schema gates cannot be closed
by synthetic fixtures or by assuming a join operator exists in the pinned Go
model. No current schema or public distribution success is fabricated.

