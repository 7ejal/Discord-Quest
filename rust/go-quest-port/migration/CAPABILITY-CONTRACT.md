# Rust capability contract and source parity boundaries

This document prevents mistaken claims that recognizing a Quest task ID means the CLI earns its reward. **Source-backed on 2026-11-01.**

| Quest type | Current Rust behavior | Original Go approach (source claim) | Evidence required for real completion | Current Rust live support |
|---|---|---|---|---|
| WATCH_VIDEO | Offline parse + evaluate server-shaped progress | Submits simulated video progress | Actual video participation and Quest-level confirmation | **None** |
| WATCH_VIDEO_ON_MOBILE | Offline parse + classify mobile-video task | Same simulated progress route | Real mobile eligibility/viewing where required | **None** |
| PLAY_ON_DESKTOP | Offline parse + per-task application ID | Generic game heartbeat | Real supported game play and progress credited by service | **None** |
| PLAY_ON_DESKTOP_V2 | Offline deprecated warning | Generic game heartbeat | Historical/deprecated, don't infer current availability | **None** |
| PLAY_ON_XBOX | Offline console prerequisite warning | Desktop-style handler, untested | Real linked console playtime / externally confirmed Quest | **None** |
| PLAY_ON_PLAYSTATION | Offline console prerequisite warning | Desktop-style handler, untested | Real linked console playtime / externally confirmed Quest | **None** |
| STREAM_ON_DESKTOP | Offline stream/viewer prerequisite warning | Presence + periodic progress claim | Real stream, viewer and server confirmation | **None** |
| PLAY_ACTIVITY | Offline Activity classification | Placeholder session heartbeat | Real Activity session and credited participation | **None** |
| ACHIEVEMENT_IN_ACTIVITY | Offline application-achievement category | Application-specific provider/OAuth flow | Actual Activity-provider achievement | **None** |
| ACHIEVEMENT_IN_GAME | Offline publisher-achievement category | No general handler | Actual publisher/game reporting | **None** |

**Source authenticity warning:** A local JSON file can contain fabricated `completed_at`/task progress fields. The offline Rust CLI cannot verify the file was produced by Discord; status labels mean “present in supplied input,” **not** an independently authenticated server result. See `STATUS-013` / [pre-code audit](PRE-CODE-REVIEW-2026-11-01.md).

**Current supported behavior:** Local `demo`, local `inspect`, JSON reporting, task recognition, optional application ID parsing, AND/OR groups, time-based status assessment and bounded offline parallel processing. `15` tests passed in a previous Linux CI run, plus Windows EXE smoke test.

**Never say:** “supports 10 Quest types” without specifying whether **recognition**, **local evaluation**, **real action**, or **confirmed server credit** is intended.

**Exclusions:** Personal user-token selfbot sessions, forged client/device metadata, synthetic rewarded video/game/stream/activity, unauthorized publisher achievements, CAPTCHA bypass, automated reward claims. Track as `excluded`, not failing implementation tasks.

**Coverage questions for AI:** Does each supported status claim cite a source field? Is the task-group join operator evaluated? Is the Quest's top-level `completed_at` valid? Is claim distinct from completion? Was this behavior tested on current source? Are exclusions labeled?

See [GO-FILE-MAP.md](GO-FILE-MAP.md), [GAP-AUDIT.md](GAP-AUDIT.md), and [SPEC.md](SPEC.md).
