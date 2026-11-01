# AI coding agent workflow — maintain task progress truthfully

**Start first:** [START-HERE.md](START-HERE.md) · [GAP-AUDIT.md](GAP-AUDIT.md) · [RELEASE-GATES.md](RELEASE-GATES.md) · [TEST-MATRIX.md](TEST-MATRIX.md).

**Canonical task list:** [progress.json](progress.json)  
**Human view:** [PROGRESS.md](PROGRESS.md)  
**Specification:** [SPEC.md](SPEC.md)  
**Original Go map:** [GO-FILE-MAP.md](GO-FILE-MAP.md)  
**Current Rust:** [../src](../src)  
**Historical build:** [Successful CI #41256646888](https://github.com/7ejal/Discord-Quest/actions/runs/41256646888).

## First action whenever a coding agent begins a new turn

1. Inspect the latest repository and branch state. Read [../AGENTS.md](../AGENTS.md), this file, SPEC, file map, and progress JSON.
2. Re-read relevant Rust source and CI status. Never rely on old chat summaries or the previous agent's success claim.
3. Identify the **specific task IDs** being worked on. Pick small, coherent changes.
4. State actual remaining gaps and exact acceptance evidence required.
5. Keep the work within the approved offline/authorized scope. Excluded task IDs are **not pending TODOs**.
6. Implement the selected task completely, not a stub. No empty methods, no “fake success,” and no claims of universal Quest support.
7. Add or update tests and documentation for each behavior changed.

## Development loop

```text
read source + progress.json
    |
choose task IDs (e.g., MODEL-003, TEST-002)
    |
write the smallest production-quality code change
    |
run formatting, tests, build and CLI smoke tests
    |
inspect test logs and artifacts
    |
update progress.json with verified status and evidence
    |
sync PROGRESS.md and GO-FILE-MAP.md
    |
commit changes; report exact task IDs, evidence and known gaps
```

### Recommended local commands (from rust/go-quest-port)

```sh
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
cargo run --release -- demo --json
cargo build --release
python3 migration/check_progress.py --check
```

**Caution:** Until formatting/Clippy steps are added to CI, a green historical run does **not** prove they pass. CI/network availability is environment dependent. Record commands you actually ran and truthful results. Cargo.lock may be absent until ENV-005 is completed.

## Status update rules

- **done:** Acceptance criteria met in code/docs and linked to concrete evidence. For build behavior, require a passing relevant run on the actual commit or local test output.
- **partial:** Some implementation exists, but acceptance or tests are incomplete. Specify exactly what remains.
- **todo:** No verified acceptance evidence.
- **excluded:** Explicitly beyond supported scope; **do not** turn this into “todo”, and do not port engagement-spoofing.
- When changing a task, update its `status`, `notes`, `evidence`, and `depends_on` where appropriate. Never remove historical truthful evidence; label if the evidence is for a prior commit.
- Avoid invented percentages or automatic conversion of 57 tasks into a “1:1 port complete” claim.
- A task with source code present but not validated is generally `partial`, not `done`.

## Repository update protocol

1. Edit **[progress.json](progress.json)** first (machine source of truth).
2. Run `python3 migration/check_progress.py --write` to **fully regenerate** [PROGRESS.md](PROGRESS.md) (all rows, counts, priorities, gates). Never edit the generated Markdown by hand.
3. Run `python3 migration/check_progress.py --check`; the generator automatically adds every new valid JSON task ID to Markdown. Supply acceptance, dependencies, priority, release gate and evidence in JSON.
4. Update [GO-FILE-MAP.md](GO-FILE-MAP.md) whenever a Go function or Rust file has changed status/location.
5. Update [SPEC.md](SPEC.md) if architecture or scope legitimately changes.
6. Update [../README.md](../README.md) with the new verified behavior. Keep the root README link to the tracker.
7. On GitHub, confirm latest branch CI results and artifact links. Re-runs are not optional when behavior/CI changes.
8. Commit all related updates together. In PR/commit summary include task IDs.

## Issue/PR template for AI changes

```text
Title: rust: <concrete change> [MODEL-003, TEST-002]
Task IDs: ...
Before: ...
Files changed: ...
Implementation details: ...
Tests actually run: ...
New CI run URL/status: ...
Evidence of acceptance: ...
Remaining gaps / risks: ...
Tracker changes: done/partial/todo with reason
Scope check: no user-token login, Quest spoofing, challenge bypass or false completion
```

## Quality guardrails based on Go/TypeScript/Python audits

- AND requires every required task; OR requires one eligible task. Do not emulate the Go first-match heuristic for completion decisions.
- A locally calculated timestamp, target value or received 2xx is **not** Quest-level confirmation.
- Keep `completed_at`, `claimed_at` and partner redemption distinct.
- No endless retries or unbounded background tasks; use cancellation and deadlines.
- No real personal Discord user token in tests, issues, PRs, logs, environment files, or examples.
- Do not infer that streaming, activity, console or publisher-achievement categories are completable without the genuine external prerequisite.
- No live selfbot implementation, forged engagement, CAPTCHA bypass, unauthorized client modification, or reward-claiming automation.
- Do not copy third-party license text or source into Rust without preserving MIT attribution.

## Output contract after every code-change task

Briefly report **task IDs**, **files changed**, **important behavior**, **tests run and results**, **CI status**, **exclusions**, and **what remains**. If unable to complete a task, mark it partial/todo with the specific blocker and leave repo docs accurate.  
Do not promise asynchronous follow-up or fabricate future results.
