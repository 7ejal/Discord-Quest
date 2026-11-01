# Requested Go 1:1 parity — evidence and disposition (2026-11-01)

**Task:** `AUD-010`. **Source:** [pinned Go at `352d7f457c9511bc9e7c9b940a71012bfb47572e`](https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/tree/352d7f457c9511bc9e7c9b940a71012bfb47572e). This decision table complements the complete **16-entry** [Go file map](GO-FILE-MAP.md) and [model coverage ledger](MODEL-COVERAGE.md). It does **not** certify any inauthentic reward completion or authorize personal-account token use.

| Upstream feature/function cluster | Rust state as inspected | Evidence/interpretation | Decision |
|---|---|---|---|
| `main.go` configuration, startup and shutdown | Offline `demo` / `inspect` | [`src/main.rs`](../src/main.rs) | **Intentional difference:** no login or Gateway |
| `main.go` READY callback; concurrent quest jobs | Bounded offline parallel analyzer | [`src/manager.rs`](../src/manager.rs) | Analyze local records; not an activity scheduler |
| `client.go` `Connect` / `readLoop` / READY / Identify / heartbeat | No live connection | [HTTP and permission audit](GO-HTTP-AUTH-AUDIT-2026-11-01.md) | **Excluded:** personal-account selfbot/Gateway impersonation |
| `client.go` REST `Request`, `Get`, `Post`, `PostWithHeaders` | No HTTP client; no credential | [HTTP and permission audit](GO-HTTP-AUTH-AUDIT-2026-11-01.md) | Future local mock possible; private personal-token mode unapproved |
| `client.go` `FetchQuests` and `manager.go` `addExcludedQuest` | Local parsed Quest and excluded records | [`src/lib.rs`](../src/lib.rs), [`src/model.rs`](../src/model.rs) | Live Quest read **blocked** by missing supported scope/current schema |
| `client.go` presence, gateway send, webhook callback | No network emissions | [`src/main.rs`](../src/main.rs) | Fake presence excluded; optional sanitized notifications deferred |
| `quest.go` enrollment/completion/expiry/claim field methods | Parsed local record checks | [`src/model.rs`](../src/model.rs) | Implemented *offline*; not authenticated server proof |
| `quest.go` first-supported-task and Android-type selection | Comparison heuristic and local task type classification | [`src/manager.rs`](../src/manager.rs) | Intentionally improve on Go's first-task shortcut |
| `manager.go` `NewQuestManager`, `FilterValid`, `FilterRedeemable` | Offline classification/filtering | [`src/manager.rs`](../src/manager.rs) | Implemented; duplicate conflicts handled more strictly |
| `manager.go` `DoingQuest`, `acceptQuest` | No enrollment or live dispatch | [Capability contract](CAPABILITY-CONTRACT.md) | Enrollment mutation excluded |
| `manager.go` video/game/console/Activity/stream handlers | Task names parsed, prerequisites and progress reported only | [Capability contract](CAPABILITY-CONTRACT.md) | **Excluded** fake rewarded video/heartbeats/stream |
| `manager.go` application-specific achievements, token cleanup | No external grants | [Capability contract](CAPABILITY-CONTRACT.md) | Unauthorized achievement simulation excluded |
| `manager.go` `RedeemQuest` and challenges | Local claimed-at report only | [`src/model.rs`](../src/model.rs) | Reward claiming and challenge bypass excluded; upstream default claim loop disabled |
| `constants.go` user token/custom client headers, desktop/mobile properties | No client metadata imitation | [HTTP and permission audit](GO-HTTP-AUTH-AUDIT-2026-11-01.md) | **Excluded** as client impersonation |
| `utils.go` version scraping and webhooks | No dynamic client build spoofing/webhook sender | [Go source map](GO-FILE-MAP.md) | Excluded spoofing; benign notifications optional |
| `utils.go` third-party proxy/app authorization/progress | No third-party authorization | [Go source map](GO-FILE-MAP.md) | Achievement fabrication excluded |
| `helpers.go` UUID/format utilities | Not needed by supported CLI | [Go source map](GO-FILE-MAP.md) | Optional; no needless dependency |
| `captcha.go` third-party challenge solver | Not present | [Go source map](GO-FILE-MAP.md) | **Excluded** |
| `types.go` Quest/task/reward/models | Go structs mapped or explicitly excluded | [Model ledger](MODEL-COVERAGE.md) | Structured local model parity, not verified live protocol |
| `go.mod`, `go.sum`, `.env.example` | Cargo/Cargo.lock, no personal token input | [`Cargo.toml`](../Cargo.toml) | Native dependency equivalent; secret-bearing config intentionally omitted |
| `.gitignore`, `README.md`, `LICENSE`, `.github/` | Ignore rules, offline docs, upstream MIT notice, Linux-only test scripts | [`../README.md`](../README.md), [Linux-only](LINUX-ONLY.md) | Present; public release not yet published |

## Access matrix

- **Supported present:** local JSON inspection, explicit source-trust label, bounded workers, ten task identifiers as offline classification, formatted machine output, synthetic validation, Linux package build on main-server.
- **Supported by public Discord docs, but not implemented here:** app consent/OAuth2 `identify` and read-only `GET /users/@me` with an **OAuth2 Bearer access token**. This is *not* Go's bare personal user-token header and not a Quest read scope.
- **Unverified/blocked:** an approved Discord OAuth2 scope for personal Quest retrieval, current authenticated TaskConfig task-group format, sanctioned native HTTP Quest data ingestion.
- **Excluded:** personal-account selfbot/Gateway Identify, spoofed activities/progress, unauthorized account enrollment, third-party grant simulation, CAPTCHA solving, automated reward claims, and any 100%-games assertion.

## Test and completion boundary

The existing 99 Rust tests and source-model comparisons cited in the repository are historical commit-scoped evidence and apply **only to offline behavior**. This matrix is a static code/research acceptance artifact; no live account, API permissions or rewards were tested. `AUD-010` documents the full parity delta, **not** a completed Go clone. See [pending live HTTP request](GO-LIVE-HTTP-REQUEST.md) and the [tracker](progress.json) for remaining conditional network milestones.
