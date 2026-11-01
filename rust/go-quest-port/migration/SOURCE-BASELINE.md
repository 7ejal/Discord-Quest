# Source baseline, licenses and reproducibility

**Audited date:** 2026-11-01. Recheck latest GitHub HEAD before any future parity claim.

## Upstream reference

- Source repository: [yeweroooo/Discord-Quest-Auto-Completion-Selfbot](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot).
- Branch: `golang`.
- Pinned commit: `352d7f457c9511bc9e7c9b940a71012bfb47572e` (verified during migration research).
- Original Go runtime: **Go 1.23**.
- Direct Go requirements from `go.mod`: `github.com/gorilla/websocket v1.5.3`; `github.com/joho/godotenv v1.5.1`.
- Original license: **MIT**, notice reads **Copyright (c) 2026 yeweroooo**. [See original LICENSE](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/blob/352d7f457c9511bc9e7c9b940a71012bfb47572e/LICENSE).
- Root source entries: `.env.example`, `.github/`, `.gitignore`, `LICENSE`, `README.md`, `captcha.go`, `client.go`, `constants.go`, `go.mod`, `go.sum`, `helpers.go`, `main.go`, `manager.go`, `quest.go`, `types.go`, `utils.go` (16 entries).

## Current Rust project

- Active project: `rust/go-quest-port`, not `rust/quest-audit-cli` (older incomplete research prototype).
- Cargo edition: **2021**; package version: **0.1.0** in currently examined `Cargo.toml`.
- Current direct crates: `serde`, `serde_json`, `chrono`.
- Current code: `src/{main,lib,manager,model}.rs`; synthetic demo fixture under `examples/`.
- `Cargo.lock` was **not present in the audited repository tree**. `ENV-005` / `REL-006` remain open until committed lockfile and locked CI checks.
- Two earlier verified build jobs: [GitHub Actions #41256646888](https://github.com/7ejal/Discord-Quest/actions/runs/41256646888) for a specific prior source commit. This is not a current-release proof.
- The Rust project is **not** a 1:1 user-account selfbot. Source features excluded from the approved scope are detailed in [CAPABILITY-CONTRACT.md](CAPABILITY-CONTRACT.md).

## Attribution and license action still required

Original MIT grants broad code reuse **subject to retaining its copyright and license notice** with copies/substantial portions. The Rust folder's `Cargo.toml` says `license = "MIT"`, but that field **does not create or supply the required upstream notice automatically**. Task `ENV-009` must verify provenance/copying, add necessary license and attribution files, and check third-party crate license obligations.

## Original-to-Rust compatibility evidence standard

| Item | What can be concluded today |
|---|---|
| Same code filename or handler name | A source mapping, not behavior parity |
| Equivalent Rust data struct | Model support only for matched fields; omitted fields must be counted |
| Recognized 10 task IDs | Classification, not live reward completion |
| 15 earlier passing unit tests | The tested offline status examples passed at that commit |
| Prior Windows executable artifact | That specific Rust binary compiled and ran its demo smoke test |
| Rust process without Discord installed | Native CLI launch, not proof of game/stream participation |
| New successful current-commit CI | New code passed the checks actually executed; still not universal verification |

**Next AI:** Do not modify the pinned Go source reference without creating a separately reviewed source-bump task. When updating dependencies, capture Rust toolchain, lockfile hash and tested target. Do not silently treat future Go README claims as verified.
