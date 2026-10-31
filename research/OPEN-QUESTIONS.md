# Open research questions

**These are hypotheses, not claims of capability.** No real-account selfbot execution or Quest spoofing is part of the current research.

## Highest priority

1. **Streaming contradiction:** Why does Orion report a failed simulated-stream path while Go and REST projects advertise stream success? Versions, server-side voice/viewer conditions, or unsound completion criteria could explain it. Require a dated, privacy-safe server-completion record from legitimate streams for baseline.
2. **What constitutes a *successful* Quest?** Is a task progress number, quest completed_at, reward claimed_at, or an entitlement the correct endpoint for each research question? Document all separately.
3. **Application-controlled achievements:** Which campaigns use a publisher/game backend versus an embedded app? What is the proper authorized evidence path? Do not assume OAuth handling for one activity generalizes.
4. **Console tracking:** What do linked Xbox and PlayStation campaigns do when the user is online but the eligible game is not actually running? Document legitimate reproduction and account/privacy preconditions.
5. **Cloud Play mapping:** How do Cloud Play sessions relate to task identifier, feature flags and claim logic? Officially an experimental publisher-authenticated flow; a generic desktop heartbeat is not proven sufficient.
6. **QR and mobile handoff:** Which UI flows are simply login/session transfer, and which correspond to actual Quest objectives? Obtain official product description rather than guessing.
7. **Task config evolution:** What differences are observed between deprecated version 1 and version 2, and which projects still rely on stale field paths?
8. **Mobile video eligibility:** Do mobile-only campaigns and desktop-visible campaigns share progress behavior and redemption rules? Confirm within sanctioned product usage.
9. **Server credit vs client predictions:** Which programs mark completed before Discord does? Identify deterministic fake-server test fixtures to detect this.
10. **Claiming:** Which reward types trigger a native platform choice, verification, cooldown or rejection? Document state transitions, not protected challenge values.

## Project audits

- Go yew: locate branch-specific task switches, tests, handling of **“Working (untested)”**, error checking, and whether README claims are supported by source.
- aiko-chan-ai: inspect the stream-unsupport code path, activity handling and reward claiming limits.
- Orion: compare its newest release/changelog to reported streaming and activity bugs; distinguish old regressions from current issues.
- saintordevil: inspect logic.test coverage, native claim handling, resource cleanup and separate enrollment/claim lanes.
- xgg-2: confirm how the stream solver differs technically from game heartbeat handling; don't equate code reuse with real-stream evidence.
- unconsents: review auth/token lifecycle and multi-account backpressure, including the mismatched clone URL.
- SUDIN010: verify Python supported-platform matrix and historical owner redirects.
- SubhuJaat07: inspect tests against mocks and what genuine production evidence exists.
- Browser/Vencord/BetterDiscord projects: review permissions and dependency security.
- GitLab candidate: validate project authenticity and code content before comparing it.

## Research tooling that is appropriate

- An **offline task-config parser** with synthetic JSON fixtures.
- A **read-only comparison dashboard** generated from the machine-readable catalog.
- Source-link checking, stale-date highlighting and schema diff summaries.
- Security/static review of public files without running unknown executables.
- Reproducible report templates and precise counterexamples.
- Explicitly *not* a selfbot workflow, production Quest simulation, CAPTCHA bypass or account-credential collector.

## Status labels

Open / Investigating / Source-backed / Contradicted / Verified (with reproducible independent proof) / Obsolete. The default for a new unobserved behavior is **Open**, not **Working**.
