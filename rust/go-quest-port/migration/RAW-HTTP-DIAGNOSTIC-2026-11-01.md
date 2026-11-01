# Raw Authorization diagnostic — 2026-11-01

This user-requested diagnostic supersedes older repository statements excluding raw headers only for this narrow command. Discord prohibits personal-account selfbots. Raw personal Authorization is not an approved official developer authentication method and may risk account termination. User consent does not establish platform approval.

Implemented: exactly one GET to https://discord.com/api/v10/users/@me, or /quests/@me with --quests. No retries, redirects, inherited proxies, spoofed client metadata, enrollment, progress, heartbeats, claims or challenge bypass. Connect timeout 4 seconds, total timeout 10 seconds, body limit 1 MiB. Account output contains validity only; Quest output contains counts only. A successful shape check does not verify eligibility or completion. Private Quest schema remains unverified.

## Credential entry

Only after explicit informed approval for the selected endpoint, run in your own interactive Bash terminal on main-server. Never enter the credential in chat, command arguments, files or logs. Disable terminal/session recording first. This subshell disables tracing and core dumps and clears its temporary variable. Shell/process memory and privileged debuggers remain residual risks; zeroize cannot clear HTTP-library copies.

```bash
cd /root/quest-rust-ci-20261101/rust/go-quest-port
(
  set +x
  ulimit -c 0
  trap 'unset quest_authorization' EXIT
  IFS= read -r -s -p 'Authorization value (hidden): ' quest_authorization
  printf '\n' >&2
  printf '%s' "$quest_authorization" | ./target/release/go-quest-port raw-http-check --token-stdin --json
)
```

Paste the bare value or exactly Authorization: followed by one space and the value at the hidden prompt. Bot/Bearer schemes and embedded whitespace/control characters are refused. For a separately approved Quest-count request, add --quests to the command; this selects the Quest endpoint instead of the account endpoint. Each invocation makes at most one request. No arbitrary endpoint override exists.

## Evidence boundaries

Pre-commit validation: Gateway job job_01M6C7VJ9F00A4VBSYZ28VYK87 passed scripts/test-linux.sh: 120 Rust tests and 23 Python tracker tests, format, strict Clippy, release build, golden output, local links and network-isolated privacy checks. Final immutable revision and archive checksums are reported after push/package; this pre-commit run alone does not certify a later commit.

All new automated tests are synthetic: built-request assertions, CLI rejection tests, response reduction, and loopback HTTP responses. They do not authenticate to Discord. Loopback exercises 200, 401, 403, 429, 500, 503, redirects, malformed JSON, oversized bodies and timeout. Existing public-config/OAuth commands share transport tests; offline demo/inspect retain golden and network-isolation checks.

Live raw account check: **UNTESTED**. Live personal Quest check: **UNTESTED**. No consent or real credential was supplied in this session. NET-006 and NET-009 remain partial against their broader acceptance criteria. NET-007 schema verification and NET-008 ingestion remain unresolved. Existing historical verification entries apply only to their recorded commits.
