# Research changelog

## 2026-11-01 — Initial publication

- Created source-backed project README and historical/current Quest task reference.
- Compared Go, TypeScript, Python, REST/server, client-plugin, browser extension and Windows dummy-process approaches.
- Distinguished 10 historical/current documented task IDs, including **deprecated PLAY_ON_DESKTOP_V2**.
- Added a matrix that distinguishes claimed support, untested, conditional, unsupported, app-specific and deprecated handling.
- Documented official play/video/stream requirements, external publisher achievements, consoles, experimental Cloud Play and QR handoff distinctions.
- Captured Orion-vs-other-projects streaming disagreement, progress false-positive issue and client change regressions.
- Added schema/AND-OR validation, reward lifecycle, error conditions and security/policy risk register.
- Added sources, evidence ledger, open research questions and a machine-readable task catalog.
- Recorded a GitLab candidate discovery as **unverified** rather than claiming none exist.
- No real-account selfbots or live Quest progress/claim requests executed.

**Revisions:** On each material update, preserve historical observations and cite the newly checked code/release.
\n### 2026-11-01 — Static source audit expansion\n\n- Inspected actual source files in nine main implementations with pinned Git revisions, documented per-project findings and severity.\n- Added source-backed decision assessment at [RECOMMENDATION.md](RECOMMENDATION.md).\n- Added six secondary inspections covering QuestMaster and plugin/extension/desktop variants.\n- Identified error-swallowing false-success risk in xgg-2, local-completion and cookie-token risks in SubhuJaat07, missing default reward processing and unbounded retries in Go, and fixed-priority AND/OR limitations in several projects.\n- No actual Quest executions or test suite runs performed.\n\n### 2026-11-01 — Deeper standalone method comparison\n\n- Reverified exact Git HEAD for the five standalone selfbots and the two no-desktop comparators.\n- Published [method deep dive](standalone/METHODS-DEEP-DIVE-2026-11-01.md) and [22 offline verification fixtures](standalone/METHOD-VALIDATION-FIXTURES.md).\n- Distinguished REST-only, REST + Gateway, game presence, video reporting, real streaming, embedded activities and external achievements.\n- **Corrected SUDIN's earlier overly strong completion-verification assessment:** numeric task progress can trigger local success without Quest-level completed_at; presence is not switched when later selected games run.\n- Clarified conditional EventEmitter behavior in xgg-2's swallowed-error path and separated library/hosted services from local CLI programs.\n- No live Quest completion or test-suite execution performed.\n