# CLI pipelines, bounded JSON and cross-platform golden testing

**Validation date:** 2026-11-01. **Source commit:** `d0edd8301cc49b9aa5da5153e50e443384ceaf35`.

**GitHub Actions:** [run 41276966591](https://github.com/7ejal/Discord-Quest/actions/runs/41276966591) — **Linux PASS; Windows PASS**, including full Windows Cargo test suite, locked release build, Windows CLI smoke and EXE artifact upload. [Windows binary artifact](https://github.com/7ejal/Discord-Quest/actions/runs/41276966591/artifacts/13071611929).

**Main-server test checkout:** `/root/quest-rust-ci-20261101/rust/go-quest-port` (isolated from production). Server was used for compilation, synthetic CLI tests and report generation only; no production services or Discord accounts were touched.

## Actual implemented changes

- `inspect -` reads UTF-8 JSON from stdin, limited by the same **8 MiB** cap as files, including clean rejection of malformed and non-UTF-8 streams.
- `--json` serializes to a **16 MiB bounded in-memory writer before emitting stdout**; oversized reports fail rather than writing partial invalid JSON.
- The parser accepts one leading UTF-8 BOM; documents nested beyond the JSON recursion limit return a clean error.
- CLI tests verify Unicode filenames containing spaces, BOM handling and invalid bytes. The Windows Actions job now runs `cargo test --all-targets --locked`, not only a binary smoke test.
- Committed human-readable and JSON golden fixtures use a fixed `--at` timestamp; integration tests compare both after normalizing newline styles and check worker counts 1, 2, 4 and 32 give identical output. Golden source files are pinned to LF via `.gitattributes`.

## Evidence

The isolated main-server run at `dee4d87a0b347aeb7a391cf7cee0776e6111bd89` passed:
- `python3 migration/check_progress.py --check`
- `python3 migration/test_check_progress.py` — **17 tracker tests**
- `python3 migration/check_links.py`
- `cargo fmt --all -- --check`
- `cargo clippy --all-targets --locked -- -D warnings`
- `cargo test --all-targets --locked` — **65 passing Rust tests**: 36 library, 3 golden, 8 CLI safety, 4 stdin, 4 excluded, 10 model parity
- `cargo run --release --locked -- demo --json --at 2026-11-01T12:00:00Z` — output matched committed JSON golden byte-for-byte.

Windows initially exposed a **genuine cross-platform failure**: Git checkout converted golden fixtures to CRLF while the CLI emitted LF, so three tests failed. The comparison now normalizes both expected and actual content and the repository pins golden files to LF. The corrected Windows test job passed at `d0edd8301` alongside the Linux job.

## Explicit limits

The **16 MiB cap currently applies to JSON output**, not every human-readable terminal print. The parser has an 8 MiB total input cap and 10,000-record limits, with the JSON library's nested depth rejection. Comprehensive end-to-end memory/CPU output stress testing, configurable limits and authoritative upstream schema verification remain open. No locally supplied Quest JSON is authenticated; fake video, stream, gameplay, CAPTCHA and reward-claim automation remain excluded.

The tracker should not mark full `PARSE-005` done until text-output handling, configurable limits and resource-stress acceptance are satisfied.
