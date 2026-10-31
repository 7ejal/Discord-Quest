# Source-code audits — November 1, 2026

**Status:** Targeted static audits of actual public source files. None of the audited projects was run against a live Discord account, and no entire dependency chain or binary provenance was independently verified. Conclusions are repository-revision specific.

| Project | Runtime | Detailed audit |
|---|---|---|
| yeweroooo | Go standalone | [GO-YEWEROOOO.md](GO-YEWEROOOO.md) |
| aiko-chan-ai | TypeScript standalone | [AIKO-CHAN-AI.md](AIKO-CHAN-AI.md) |
| nyxxbit Orion | Userscript/Vencord plugin | [ORION.md](ORION.md) |
| saintordevil | Vencord plugin | [SAINTORDEVIL.md](SAINTORDEVIL.md) |
| xgg-2 | Node/TypeScript library | [XGG-2.md](XGG-2.md) |
| unconsents | Python multi-account | [UNCONSENTS.md](UNCONSENTS.md) |
| SUDIN010 | Python standalone | [SUDIN010.md](SUDIN010.md) |
| SubhuJaat07 | Hosted Next.js service | [SUBHUJAAT07.md](SUBHUJAAT07.md) |
| lfathh | TypeScript + legacy script | [LFATHH.md](LFATHH.md) |

### Additional project inspections

Six further repositories were inspected at pinned revisions: [QuestMaster, Seramicx, xenoncolt, nvckai, markterence, and nicola02nb](SECONDARY-IMPLEMENTATIONS.md). QuestMaster's substantial regression suite is a promising follow-up but was not executed or fully audited.\n\n## Boundaries

This is not an exhaustive audit of **all** public Quest code or a malware safety certification. The repositories above were prioritized for implementation differences, source accessibility and technical claims. Shorter source-discovery notes for additional projects are in [PROJECT-COMPARISON.md](../docs/PROJECT-COMPARISON.md) and [SOURCES.md](../research/SOURCES.md).

For a direct comparison of what is best for various **research purposes**, use [RECOMMENDATION.md](../research/RECOMMENDATION.md).

## Review discipline

1. Read source, not only README.
2. Link to the pinned revision and smallest relevant file range.
3. Classify **source implemented**, **maintainer-tested**, **untested**, **unsupported**, **conditional**, **deprecated**, and **independently verified** separately.
4. Distinguish task ID recognized from task solved and **server-confirmed Quest completed** from a local “done” log.
5. Distinguish reward claim attempt, actual claim, and subsequent redemption.
6. State whether tests were run. None were run here.
7. Treat sensitive account access as a separate risk.
8. Do not claim a repository is malicious or safe without stronger evidence.

## Cross-project findings

- Fake streaming metadata is **not sufficient** to establish a valid active voice stream with a viewer.
- Simply recognizing Xbox/PlayStation task names does not establish genuine console gameplay validation.
- Application-controlled achievements and publisher-controlled in-game achievements have different trust boundaries.
- Many standalone projects use first-matching-task heuristics, not AND/OR task group semantics.
- The strongest source checks and tests do **not** guarantee live Discord acceptance.
- Multiple projects can incorrectly mark a local run “completed” without top-level server confirmation.
