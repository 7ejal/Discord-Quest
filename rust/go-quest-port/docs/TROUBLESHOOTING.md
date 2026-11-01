# Linux x86_64 CLI troubleshooting

These instructions apply only to the **offline** native Rust Quest JSON inspector. Test/build only on the connected `main-server`; do not restart production services or change global network settings.

## Running a prebuilt Linux archive

1. Verify the archive SHA256 against its matching `.tar.gz.sha256` on a trusted Linux machine.
2. Extract: `tar -xzf go-quest-port-<version>-linux-x86_64-<sha>.tar.gz`.
3. In the extracted directory, run `sha256sum -c SHA256SUMS`.
4. Run `./go-quest-port --version` and `./go-quest-port demo --json`.

The native binary dynamically uses standard Linux libraries such as `libc.so.6` and `libgcc_s.so.1`. A Linux distribution with incompatible libc or CPU architecture may not run it. Use `uname -m`, `file ./go-quest-port`, and `ldd ./go-quest-port` to inspect architecture/dependencies. Do not assume this archive is statically linked or works on ARM.

## Building on the designated main server

```bash
cd /root/quest-rust-ci-20261101/rust/go-quest-port
CARGO_BUILD_JOBS=2 nice -n 10 bash scripts/test-linux.sh
CARGO_BUILD_JOBS=2 nice -n 10 bash scripts/package-linux.sh
```

If the compiler is missing, **do not** automatically install Rust or packages on a production host. The scripts disable rustup auto-install. Report the missing toolchain and request approval before changing the host.

## CLI errors

| Message/behavior | Likely cause and safe response |
|---|---|
| `Missing local JSON file path` | Use `inspect file.json`, or `inspect -` to read stdin |
| `Invalid JSON` | Supply valid UTF-8 JSON; an initial UTF-8 BOM is supported |
| `JSON exceeds ... input limit` | The input exceeds the 8 MiB cap; reduce the file rather than disabling safeguards |
| `Too many Quests` | Exceeds 10,000 Quest records; split the input |
| `Expected a Quest object...` | Root must be one Quest object, an array, or a wrapper with `quests` |
| `Quest at index ...` | Invalid/incomplete full Quest entry; check its ID and config |
| `JSON report exceeds output size limit` | JSON serialization exceeded 16 MiB; reduce input or selection rather than emitting a partial JSON document |
| `Unknown task group operator` | Treat as unsupported; do not infer completion |
| `Invalid ... timestamp` | A supplied date is malformed, not proof of a valid server event |
| `Permission denied` | Check extracted executable permissions and filesystem mount; do not unnecessarily run a downloaded binary as root |

`--json` emits one JSON document on success; stderr is used for errors. `input_trust = "unverified_local_json"` explicitly indicates the inspector **cannot authenticate** who generated the input. Do not paste personal account tokens into fixtures, logs or issues.

## Checking resource use

Run `python3 scripts/benchmark-linux.py --quests 10000` **only in the dedicated main-server checkout** after building the executable. It generates synthetic local JSON, caps the measured child process to 512 MiB virtual memory and 60 CPU-seconds, then prints observed CPU time, wall time, RSS and binary size. Performance varies between runs and machines.

## Where the artifact is kept

Packages are created in `dist/linux-x86_64/` inside the isolated checkout, and ignored by Git. A local file on the main server is **not a permanent public GitHub download**. [Latest source-pinned main-server verification](../migration/MAIN-SERVER-LINUX-RELEASE-2026-11-01.md) records the tested SHA and archive checksum.

## Project boundaries

There is no GUI, Discord login, live Quest enrollment, synthetic gameplay/video/streaming, CAPTCHA solver, or automatic reward claim. These are intentionally excluded, not configuration problems. For remaining engineering tasks see [the canonical tracker](../migration/progress.json).
