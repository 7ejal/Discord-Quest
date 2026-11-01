# Acceptance evidence index

This maps task IDs to executable tests or reviewed non-code artifacts.
Current main-server results and exact source commits are recorded in
[the completion record](MAIN-SERVER-COMPLETION-2026-11-01.md).
A mapping is not proof that an open task has passed. Modern task-group samples
and public publication remain pending; optional networking is deferred.

| Task | Acceptance check / artifact |
|---|---|
| `INV-001` | MODEL-COVERAGE.md; reviewed documentation/source artifact and check_links.py |
| `INV-002` | MODEL-COVERAGE.md; reviewed documentation/source artifact and check_links.py |
| `INV-003` | MODEL-COVERAGE.md; reviewed documentation/source artifact and check_links.py |
| `ENV-001` | scripts/test-linux.sh and package-linux.sh; extracted binary smoke/hash checks; completion record |
| `ENV-002` | scripts/test-linux.sh and package-linux.sh; extracted binary smoke/hash checks; completion record |
| `ENV-003` | scripts/test-linux.sh and package-linux.sh; extracted binary smoke/hash checks; completion record |
| `ENV-004` | scripts/test-linux.sh and package-linux.sh; extracted binary smoke/hash checks; completion record |
| `ENV-005` | scripts/test-linux.sh and package-linux.sh; extracted binary smoke/hash checks; completion record |
| `ENV-006` | DEPENDENCY-REVIEW.md; package DEPENDENCY-LICENSES.txt, Cargo.lock, LICENSE/NOTICE; strict Clippy |
| `ENV-007` | scripts/test-linux.sh and package-linux.sh; extracted binary smoke/hash checks; completion record |
| `ENV-008` | Public publication PENDING; no release URL claimed |
| `ENV-009` | scripts/test-linux.sh and package-linux.sh; extracted binary smoke/hash checks; completion record |
| `ENV-010` | LINUX-ONLY.md / CAPABILITY-CONTRACT.md; deliberately excluded |
| `MODEL-001` | tests/offline_regressions.rs, offline_contract.rs, model_parity.rs, excluded_quests.rs; src/lib.rs tests; MODEL-COVERAGE.md |
| `MODEL-002` | tests/offline_regressions.rs, offline_contract.rs, model_parity.rs, excluded_quests.rs; src/lib.rs tests; MODEL-COVERAGE.md |
| `MODEL-003` | tests/offline_regressions.rs, offline_contract.rs, model_parity.rs, excluded_quests.rs; src/lib.rs tests; MODEL-COVERAGE.md |
| `MODEL-004` | tests/offline_regressions.rs, offline_contract.rs, model_parity.rs, excluded_quests.rs; src/lib.rs tests; MODEL-COVERAGE.md |
| `MODEL-005` | tests/offline_regressions.rs, offline_contract.rs, model_parity.rs, excluded_quests.rs; src/lib.rs tests; MODEL-COVERAGE.md |
| `MODEL-006` | tests/offline_regressions.rs, offline_contract.rs, model_parity.rs, excluded_quests.rs; src/lib.rs tests; MODEL-COVERAGE.md |
| `MODEL-007` | tests/offline_regressions.rs, offline_contract.rs, model_parity.rs, excluded_quests.rs; src/lib.rs tests; MODEL-COVERAGE.md |
| `MODEL-008` | tests/offline_regressions.rs, offline_contract.rs, model_parity.rs, excluded_quests.rs; src/lib.rs tests; MODEL-COVERAGE.md |
| `MODEL-009` | tests/offline_regressions.rs, offline_contract.rs, model_parity.rs, excluded_quests.rs; src/lib.rs tests; MODEL-COVERAGE.md |
| `STATE-001` | tests/offline_regressions.rs state/operator table; model_parity.rs; cli_safety.rs fixed clock; src/lib.rs tests |
| `STATE-002` | tests/offline_regressions.rs state/operator table; model_parity.rs; cli_safety.rs fixed clock; src/lib.rs tests |
| `STATE-003` | tests/offline_regressions.rs state/operator table; model_parity.rs; cli_safety.rs fixed clock; src/lib.rs tests |
| `STATE-004` | tests/offline_regressions.rs state/operator table; model_parity.rs; cli_safety.rs fixed clock; src/lib.rs tests |
| `STATE-005` | MODEL-COVERAGE.md schema rules; synthetic tests pass, modern authenticated schema evidence PENDING |
| `STATE-006` | tests/offline_regressions.rs state/operator table; model_parity.rs; cli_safety.rs fixed clock; src/lib.rs tests |
| `STATE-007` | tests/offline_regressions.rs state/operator table; model_parity.rs; cli_safety.rs fixed clock; src/lib.rs tests |
| `STATE-008` | tests/offline_regressions.rs state/operator table; model_parity.rs; cli_safety.rs fixed clock; src/lib.rs tests |
| `STATE-009` | tests/offline_regressions.rs state/operator table; model_parity.rs; cli_safety.rs fixed clock; src/lib.rs tests |
| `CLI-001` | tests/offline_contract.rs privacy/flags/exit/limit tests; cli_safety.rs; cli_stdin.rs; cli_golden.rs |
| `CLI-002` | tests/offline_contract.rs privacy/flags/exit/limit tests; cli_safety.rs; cli_stdin.rs; cli_golden.rs |
| `CLI-003` | tests/offline_contract.rs privacy/flags/exit/limit tests; cli_safety.rs; cli_stdin.rs; cli_golden.rs |
| `CLI-004` | tests/offline_contract.rs privacy/flags/exit/limit tests; cli_safety.rs; cli_stdin.rs; cli_golden.rs |
| `PAR-001` | src/lib.rs::concurrency_preserves_result_order; tests/offline_contract.rs duplicate_empty_and_large_lists_preserve_order_and_fail_conflicts |
| `PAR-002` | src/manager.rs::worker_tests (cancellation, worker error, panic and joins) |
| `PAR-003` | Optional future integration DEFERRED; no current transport dependency |
| `NET-001` | Optional future integration DEFERRED; no current transport dependency |
| `NET-002` | Optional future integration DEFERRED; no current transport dependency |
| `NET-003` | Optional future integration DEFERRED; no current transport dependency |
| `NET-004` | Optional future integration DEFERRED; no current transport dependency |
| `TEST-001` | Cargo locked all-target tests, golden fixtures and scripts/test-linux.sh; completion record |
| `TEST-002` | Cargo locked all-target tests, golden fixtures and scripts/test-linux.sh; completion record |
| `TEST-003` | RESEARCH-CASE-MAP.md; executable cases and explicit network/OAuth N/A |
| `TEST-004` | Cargo locked all-target tests, golden fixtures and scripts/test-linux.sh; completion record |
| `TEST-005` | tests/offline_contract.rs resource/duplicate/order tests; scripts/benchmark-linux.py measured under child limits |
| `SEC-001` | tests/offline_contract.rs privacy/flags/exit/limit tests; cli_safety.rs; cli_stdin.rs; cli_golden.rs |
| `SEC-002` | DEPENDENCY-REVIEW.md; package DEPENDENCY-LICENSES.txt, Cargo.lock, LICENSE/NOTICE; strict Clippy |
| `DOC-001` | README.md; reviewed documentation/source artifact and check_links.py |
| `DOC-002` | migration/SPEC.md, migration/progress.json; reviewed documentation/source artifact and check_links.py |
| `DOC-003` | AGENTS.md, migration/AI-WORKFLOW.md; reviewed documentation/source artifact and check_links.py |
| `DOC-004` | migration/check_progress.py, migration/PROGRESS.md, scripts/test-linux.sh; reviewed documentation/source artifact and check_links.py |
| `SCOPE-001` | LINUX-ONLY.md / CAPABILITY-CONTRACT.md; deliberately excluded |
| `SCOPE-002` | LINUX-ONLY.md / CAPABILITY-CONTRACT.md; deliberately excluded |
| `SCOPE-003` | LINUX-ONLY.md / CAPABILITY-CONTRACT.md; deliberately excluded |
| `SCOPE-004` | LINUX-ONLY.md / CAPABILITY-CONTRACT.md; deliberately excluded |
| `AUD-001` | MODEL-COVERAGE.md schema rules; synthetic tests pass, modern authenticated schema evidence PENDING |
| `AUD-002` | tests/go_parity.rs; MODEL-COVERAGE.md pure-method/dispatcher ledger |
| `AUD-003` | rust/quest-audit-cli/README.md, AGENTS.md, migration/START-HERE.md; reviewed documentation/source artifact and check_links.py |
| `AUD-004` | migration/check_links.py, migration/GO-FILE-MAP.md; reviewed documentation/source artifact and check_links.py |
| `AUD-005` | tests/task_catalog.rs, data/task-types.json, migration/MAIN-SERVER-NETWORK-ISOLATION-2026-11-01.md; reviewed documentation/source artifact and check_links.py |
| `AUD-006` | migration/CAPABILITY-CONTRACT.md, migration/GO-FILE-MAP.md; reviewed documentation/source artifact and check_links.py |
| `AUD-007` | README.md, migration/check_links.py; reviewed documentation/source artifact and check_links.py |
| `AUD-008` | DEPENDENCY-REVIEW.md; package DEPENDENCY-LICENSES.txt, Cargo.lock, LICENSE/NOTICE; strict Clippy |
| `PARSE-001` | tests/offline_regressions.rs, offline_contract.rs, model_parity.rs, excluded_quests.rs; src/lib.rs tests; MODEL-COVERAGE.md |
| `PARSE-002` | tests/offline_regressions.rs, offline_contract.rs, model_parity.rs, excluded_quests.rs; src/lib.rs tests; MODEL-COVERAGE.md |
| `PARSE-003` | tests/offline_regressions.rs, offline_contract.rs, model_parity.rs, excluded_quests.rs; src/lib.rs tests; MODEL-COVERAGE.md |
| `PARSE-004` | tests/offline_regressions.rs, offline_contract.rs, model_parity.rs, excluded_quests.rs; src/lib.rs tests; MODEL-COVERAGE.md |
| `PARSE-005` | tests/offline_regressions.rs, offline_contract.rs, model_parity.rs, excluded_quests.rs; src/lib.rs tests; MODEL-COVERAGE.md |
| `PARSE-006` | tests/offline_regressions.rs, offline_contract.rs, model_parity.rs, excluded_quests.rs; src/lib.rs tests; MODEL-COVERAGE.md |
| `PARSE-007` | tests/offline_regressions.rs, offline_contract.rs, model_parity.rs, excluded_quests.rs; src/lib.rs tests; MODEL-COVERAGE.md |
| `PARSE-008` | tests/offline_regressions.rs, offline_contract.rs, model_parity.rs, excluded_quests.rs; src/lib.rs tests; MODEL-COVERAGE.md |
| `PARSE-009` | tests/offline_regressions.rs, offline_contract.rs, model_parity.rs, excluded_quests.rs; src/lib.rs tests; MODEL-COVERAGE.md |
| `PARSE-010` | tests/offline_regressions.rs, offline_contract.rs, model_parity.rs, excluded_quests.rs; src/lib.rs tests; MODEL-COVERAGE.md |
| `STATUS-010` | tests/offline_regressions.rs state/operator table; model_parity.rs; cli_safety.rs fixed clock; src/lib.rs tests |
| `STATUS-011` | tests/offline_regressions.rs state/operator table; model_parity.rs; cli_safety.rs fixed clock; src/lib.rs tests |
| `STATUS-012` | tests/offline_regressions.rs state/operator table; model_parity.rs; cli_safety.rs fixed clock; src/lib.rs tests |
| `STATUS-013` | tests/offline_regressions.rs state/operator table; model_parity.rs; cli_safety.rs fixed clock; src/lib.rs tests |
| `STATUS-014` | tests/offline_regressions.rs state/operator table; model_parity.rs; cli_safety.rs fixed clock; src/lib.rs tests |
| `STATUS-015` | tests/offline_regressions.rs state/operator table; model_parity.rs; cli_safety.rs fixed clock; src/lib.rs tests |
| `STATUS-016` | tests/offline_regressions.rs state/operator table; model_parity.rs; cli_safety.rs fixed clock; src/lib.rs tests |
| `STATUS-017` | tests/go_parity.rs; MODEL-COVERAGE.md pure-method/dispatcher ledger |
| `STATUS-018` | tests/offline_regressions.rs state/operator table; model_parity.rs; cli_safety.rs fixed clock; src/lib.rs tests |
| `UX-001` | tests/offline_contract.rs privacy/flags/exit/limit tests; cli_safety.rs; cli_stdin.rs; cli_golden.rs |
| `UX-002` | tests/offline_contract.rs privacy/flags/exit/limit tests; cli_safety.rs; cli_stdin.rs; cli_golden.rs |
| `UX-003` | tests/offline_contract.rs privacy/flags/exit/limit tests; cli_safety.rs; cli_stdin.rs; cli_golden.rs |
| `UX-004` | tests/offline_contract.rs privacy/flags/exit/limit tests; cli_safety.rs; cli_stdin.rs; cli_golden.rs |
| `UX-005` | LINUX-ONLY.md / CAPABILITY-CONTRACT.md; deliberately excluded |
| `UX-006` | tests/offline_contract.rs privacy/flags/exit/limit tests; cli_safety.rs; cli_stdin.rs; cli_golden.rs |
| `QA-001` | migration/check_progress.py, test_check_progress.py and check_links.py (23 tests); this per-ID index |
| `QA-002` | tests/offline_contract.rs::deterministic_mutation_corpus_never_panics_or_confirms_completion (1024 fixed-seed mutations per run) |
| `QA-003` | Cargo locked all-target tests, golden fixtures and scripts/test-linux.sh; completion record |
| `QA-004` | tests/offline_contract.rs resource/duplicate/order tests; scripts/benchmark-linux.py measured under child limits |
| `QA-005` | Cargo locked all-target tests, golden fixtures and scripts/test-linux.sh; completion record |
| `QA-006` | tests/offline_contract.rs resource/duplicate/order tests; scripts/benchmark-linux.py measured under child limits |
| `QA-007` | RESEARCH-CASE-MAP.md; executable cases and explicit network/OAuth N/A |
| `QA-008` | migration/check_progress.py, test_check_progress.py and check_links.py (23 tests); this per-ID index |
| `REL-001` | scripts/test-linux.sh and package-linux.sh; extracted binary smoke/hash checks; completion record |
| `REL-002` | scripts/test-linux.sh and package-linux.sh; extracted binary smoke/hash checks; completion record |
| `REL-003` | Public publication PENDING; no release URL claimed |
| `REL-004` | DEPENDENCY-REVIEW.md; package DEPENDENCY-LICENSES.txt, Cargo.lock, LICENSE/NOTICE; strict Clippy |
| `REL-005` | DEPENDENCY-REVIEW.md; package DEPENDENCY-LICENSES.txt, Cargo.lock, LICENSE/NOTICE; strict Clippy |
| `REL-006` | scripts/test-linux.sh and package-linux.sh; extracted binary smoke/hash checks; completion record |
| `REL-007` | scripts/test-linux.sh and package-linux.sh; extracted binary smoke/hash checks; completion record |
| `REL-008` | scripts/test-linux.sh and package-linux.sh; extracted binary smoke/hash checks; completion record |
| `OPS-001` | migration/check_progress.py, test_check_progress.py and check_links.py (23 tests); this per-ID index |
| `OPS-002` | migration/check_progress.py, test_check_progress.py and check_links.py (23 tests); this per-ID index |
| `OPS-003` | migration/check_progress.py, test_check_progress.py and check_links.py (23 tests); this per-ID index |
| `OPS-004` | migration/check_progress.py, test_check_progress.py and check_links.py (23 tests); this per-ID index |
| `OPS-005` | migration/check_progress.py, test_check_progress.py and check_links.py (23 tests); this per-ID index |
| `OPS-006` | migration/check_progress.py, test_check_progress.py and check_links.py (23 tests); this per-ID index |
| `OPS-007` | migration/check_progress.py, test_check_progress.py and check_links.py (23 tests); this per-ID index |
| `OPS-008` | migration/check_progress.py, test_check_progress.py and check_links.py (23 tests); this per-ID index |
| `MODEL-010` | tests/offline_regressions.rs, offline_contract.rs, model_parity.rs, excluded_quests.rs; src/lib.rs tests; MODEL-COVERAGE.md |
| `MODEL-011` | tests/offline_regressions.rs, offline_contract.rs, model_parity.rs, excluded_quests.rs; src/lib.rs tests; MODEL-COVERAGE.md |
| `STATE-019` | tests/offline_regressions.rs state/operator table; model_parity.rs; cli_safety.rs fixed clock; src/lib.rs tests |
| `STATE-020` | tests/offline_regressions.rs state/operator table; model_parity.rs; cli_safety.rs fixed clock; src/lib.rs tests |
| `STATE-021` | tests/offline_regressions.rs state/operator table; model_parity.rs; cli_safety.rs fixed clock; src/lib.rs tests |
| `SEC-003` | tests/offline_contract.rs privacy/flags/exit/limit tests; cli_safety.rs; cli_stdin.rs; cli_golden.rs |
| `SEC-004` | tests/offline_contract.rs privacy/flags/exit/limit tests; cli_safety.rs; cli_stdin.rs; cli_golden.rs |
| `QA-009` | tests/go_parity.rs; MODEL-COVERAGE.md pure-method/dispatcher ledger |
| `QA-010` | src/manager.rs::worker_tests (cancellation, worker error, panic and joins) |
| `REL-009` | LINUX-ONLY.md / CAPABILITY-CONTRACT.md; deliberately excluded |
| `DOC-005` | migration/MAIN-SERVER-CI-2026-11-01.md; reviewed documentation/source artifact and check_links.py |
| `DOC-006` | README.md, docs/TROUBLESHOOTING.md, migration/MAIN-SERVER-LINUX-RELEASE-2026-11-01.md; reviewed documentation/source artifact and check_links.py |
| `OPS-009` | migration/check_progress.py, test_check_progress.py and check_links.py (23 tests); this per-ID index |
