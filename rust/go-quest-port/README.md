# Go Quest models → native Rust CLI (Linux only)

**Active target: Linux x86_64, command-line only. Verification and packaging run on the connected `main-server` in an isolated Git checkout.** The project is an **offline, read-only inspector for locally supplied Quest JSON**. It is not an operational Discord selfbot, does not authenticate input as server data and cannot perform rewarded engagement, bypass verification, or automatically claim rewards.

Original Go reference: [yeweroooo/Discord-Quest-Auto-Completion-Selfbot @ `352d7f4`](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/tree/352d7f457c9511bc9e7c9b940a71012bfb47572e). The original MIT copyright notice is retained in [LICENSE](LICENSE), with attribution in [NOTICE](NOTICE).

## Build and test — main-server only

The repository has **no active GitHub-hosted test workflow** and no Windows release target. Do not install a self-hosted runner or alter production services.

After a GitHub commit is fast-forward pulled into `/root/quest-rust-ci-20261101`, run:

```bash
cd /root/quest-rust-ci-20261101/rust/go-quest-port
CARGO_BUILD_JOBS=2 nice -n 10 bash scripts/test-linux.sh
CARGO_BUILD_JOBS=2 nice -n 10 bash scripts/package-linux.sh
```

The first script checks the progress manifest, 17 tracker unit tests, local documentation links, formatting, strict Clippy, locked Cargo tests and a Linux release build with synthetic JSON smoke tests. The second packages the **tested Linux x86_64 executable**, README, license/notice and file checksums into `dist/linux-x86_64/`. Scripts prevent automatic rustup toolchain installation. A local server archive is **not** a GitHub release or public downloadable artifact.

See [Linux-only contract](migration/LINUX-ONLY.md), [main-server procedure](migration/MAIN-SERVER-WORKFLOW.md), and [release gates](migration/RELEASE-GATES.md).

**Verified Linux release candidate:** [main-server test, archive SHA256 and measured CPU/RAM](migration/MAIN-SERVER-LINUX-RELEASE-2026-11-01.md). For extraction, system libraries and CLI errors, see [Linux troubleshooting](docs/TROUBLESHOOTING.md). The archive is server-resident; no permanent public GitHub release has been claimed.

## Run the native CLI on Linux

```bash
./target/release/go-quest-port --version
./target/release/go-quest-port demo --json --at 2026-11-01T12:00:00Z
./target/release/go-quest-port inspect examples/sample-quests.json --json
cat examples/sample-quests.json | ./target/release/go-quest-port inspect - --json
```

```text
go-quest-port demo [--json] [--workers N] [--at RFC3339]
go-quest-port inspect <local-quests.json|-> [--json] [--workers N] [--at RFC3339]
go-quest-port --version
```

**Limits and behavior:** UTF-8 JSON only, optional initial UTF-8 BOM; 8 MiB input cap, maximum 10,000 Quest objects, 16 MiB buffered JSON output cap. `--workers` accepts 1–32 bounded analysis threads. `--at` makes report times deterministic. JSON includes `schema_version` and `input_trust: "unverified_local_json"`. The parser separates excluded partial Quests, handles task IDs/reward metadata, flags contradictory status or malformed timestamps and escapes untrusted terminal-control characters. It **never** connects to Discord.

The [golden JSON/text snapshots](tests/golden/) and [test suites](tests/) are synthetic: they do not prove any Quest was genuinely completed.

## State of development and tracking

The authoritative status is [progress.json](migration/progress.json), rendered as [PROGRESS.md](migration/PROGRESS.md). Start work using [START-HERE.md](migration/START-HERE.md), [GO-FILE-MAP.md](migration/GO-FILE-MAP.md), [GAP-AUDIT.md](migration/GAP-AUDIT.md) and [SPEC.md](migration/SPEC.md).

**Historical verification:** [GitHub Actions run #41277276280](https://github.com/7ejal/Discord-Quest/actions/runs/41277276280) passed 65 Rust tests on Linux and Windows before the Linux-only decision. Those results are historical; new validation runs on `main-server` only. Do not claim current code passed based on an earlier commit.

**Remaining engineering work:** Complete model/schema coverage, robustness and stress testing, honest provenance and status reporting, credential-free offline boundary tests, Linux release verification and optional distribution. The tracker distinguishes `done`, `partial`, `todo` and `excluded`; the absence of Windows or GUI support is deliberate, not missing functionality.

## Source and licensing

- [Go-to-Rust file mapping](migration/GO-FILE-MAP.md)
- [Capability limitations](migration/CAPABILITY-CONTRACT.md)
- [Linux-only platform/test policy](migration/LINUX-ONLY.md)
- [MIT LICENSE](LICENSE) · [NOTICE](NOTICE)

No credentials, publisher achievements or gameplay are requested or collected.
