# Instructions for AI agents working in rust/go-quest-port

**Start with [migration/START-HERE.md](migration/START-HERE.md).** The code-aware [GAP-AUDIT.md](migration/GAP-AUDIT.md), [TEST-MATRIX.md](migration/TEST-MATRIX.md), [DECISIONS.md](migration/DECISIONS.md), and [RELEASE-GATES.md](migration/RELEASE-GATES.md) are mandatory. This is the **only active Rust CLI**, not the sibling `rust/quest-audit-cli`.

**Before editing code or claiming completion, read in this order:**

1. [migration/PROGRESS.md](migration/PROGRESS.md) for human-readable current status.
2. [migration/progress.json](migration/progress.json) for canonical task IDs/status/evidence.
3. [migration/SPEC.md](migration/SPEC.md) for approved scope and Definition of Done.
4. [migration/GO-FILE-MAP.md](migration/GO-FILE-MAP.md) for each upstream Go file/function.
5. [migration/AI-WORKFLOW.md](migration/AI-WORKFLOW.md) for the update/validation protocol.
6. [README.md](README.md) for current CLI commands, limitations and historical CI run.

## Mandatory practices

- **Choose named IDs** from progress.json; do not start “port everything” without a scoped change.
- Improve existing source rather than generating separate duplicate Rust projects.
- Keep the app **native**, locally runnable and self-contained for offline Quest inspection.
- Mark tasks `done` only with acceptance evidence; otherwise `partial` or `todo`.
- Reconcile progress.json and PROGRESS.md in the same commit. Run `python3 migration/check_progress.py --check`.
- Verify tests and CI on the current revision; previous run 41256646888 proves only the prior code state.
- Never claim operational 1:1 selfbot feature parity or all-game completion.
- Do not build personal-account selfbot/impersonation, engagement spoofing, CAPTCHA bypass, publisher achievement fabrication, or automated reward claims. These are intentionally excluded.
- Use no genuine personal account tokens or sensitive identifiers in tests/docs.
- Record exact changed files, tests, status and remaining work.

**Task state is determined by the canonical tracker, not by README marketing or source comments.**
