# Dependency and execution review policy

Reviewed scope: the Linux-only offline inspector and its main-server test/package
scripts. Direct Rust dependencies are chrono, serde and serde_json; no HTTP,
WebSocket, TLS-client, browser, CAPTCHA or async runtime dependency was added.

The Cargo.lock graph remains pinned. Release packaging runs Cargo metadata with
`--locked --offline --filter-platform x86_64-unknown-linux-gnu`, requires a
declared license for each resolved crate and includes the resulting inventory
and Cargo.lock in the archive. Review SPDX identifiers and upstream notices
before accepting dependency changes. Preserve the upstream MIT license and
NOTICE. Do not silently permit an unlicensed or unknown-license update.

This is a manifest/license and runtime-surface review, not a claim of a fresh
advisory-database scan or a legal opinion. No cargo-audit tool was installed or
unverified vulnerability-free claim made. Review security advisories when
updating dependency versions; absence of new dependencies does not establish
absence of vulnerabilities.

Verification scripts use two build jobs and two Rust test threads, finite
input/output/work limits, and process-local network isolation. Go reference code
uses only the standard library and copied pure helpers. The harness does not
compile or link upstream account/client/engagement code. Exact command at repo
root:

```bash
GO111MODULE=off GOMAXPROCS=2 go run ./tools/go-quest-reference < rust/go-quest-port/examples/sample-quests.json
```

No system packages, permanent runners, firewall rules, production services,
credentials or recurring jobs are changed. Packaging rejects a dirty checkout.
The existing bounded benchmark caps child address space at 512 MiB and CPU time
at 60 seconds, with a 90-second wall timeout; record machine/input sizes and
actual observations, never extrapolate them to Discord engagement performance.
