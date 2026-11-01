# Linux-only development and verification contract

**Decision (2026-11-01): Linux x86_64 CLI only; test and package only on the user's connected `main-server`.** This supersedes earlier Windows/macOS and GitHub Actions requirements. The hosted `.github/workflows/rust-go-quest-port.yml` workflow was removed: no GitHub-hosted or Windows runners should execute this project automatically.

## Authorized environment and isolation

- Linux target: `x86_64-unknown-linux-gnu`; Rust toolchain tested: `1.98.1`.
- Gateway server: **`main-server`**. Working copy: `/root/quest-rust-ci-20261101`, created for this project only.
- Never compile or modify production services, existing Discord bots, Docker containers, firewall rules, system packages, system network namespaces, global services, or persistent cron jobs.
- Use `CARGO_BUILD_JOBS=2`, `nice -n 10` when appropriate. The build scripts set `RUSTUP_AUTO_INSTALL=0` to avoid unexpected toolchain downloads; stop and report if the pinned toolchain is absent.
- The repository still lives on GitHub, but **GitHub is source control, not a separate CI executor**. No self-hosted GitHub Actions runner has been installed on the main server.

## Mandatory workflow after each source commit

1. Commit the changed Rust source and tests to GitHub `main`.
2. Confirm dedicated test checkout has no unrelated changes; only handle project-generated cache/artifact files inside it.
3. Fast-forward pull the committed GitHub revision into the isolated checkout using Gateway `git_pull` (no merge/rebase or production deploy).
4. Run:
   ```bash
   cd /root/quest-rust-ci-20261101/rust/go-quest-port
   CARGO_BUILD_JOBS=2 nice -n 10 bash scripts/test-linux.sh
   ```
5. For a distributable Linux artifact, run:
   ```bash
   CARGO_BUILD_JOBS=2 nice -n 10 bash scripts/package-linux.sh
   ```
6. Record `git rev-parse HEAD`, test count, compiler version, archive path and SHA256, and any failures or limitations. Updates to `progress.json` must be followed by regenerating `PROGRESS.md` and rerunning the server checks.
7. **Never** claim a GitHub release or public download exists when the artifact is only stored on the main server. A tarball in `dist/` is a server-side release candidate, not a published release.

## Exact release output

After successful packaging:

```text
rust/go-quest-port/dist/linux-x86_64/
    go-quest-port-<version>-linux-x86_64-<commit>.tar.gz
    go-quest-port-<version>-linux-x86_64-<commit>.tar.gz.sha256
```

Archive includes the Linux executable, README, upstream MIT LICENSE, NOTICE, build-info file and SHA256SUMS for each included file. Verify the archive and run `ldd` on the binary; a native binary may still need ordinary Linux system libraries.

## Scope and remaining work

CLI-only, primarily offline and read-only, with explicit public-Quest HTTPS and optional official OAuth2 identity checks. No GUI, raw personal-account selfbot authentication, fabricated rewarded participation, Discord presence spoofing, CAPTCHA solving or automatic reward claiming. Input JSON is unverified local data. Release milestones and remaining work are tracked in [progress.json](progress.json) and [RELEASE-GATES.md](RELEASE-GATES.md).

The GitHub Actions Windows results retained elsewhere in the repository are **historical evidence only**, not current platform support. New task IDs or changed acceptance criteria should use this Linux-only contract.

This contract does not authorize background execution or periodic tests: commands execute only when requested.
