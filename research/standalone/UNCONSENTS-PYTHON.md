# Detailed source walkthrough: unconsents (Python multi-account CLI)

**Pinned source:** [main @ 6d3b9a085dc78ce42d85dd9748f1636eff111a5a](https://github.com/unconsents/auto-quest-completer/tree/6d3b9a085dc78ce42d85dd9748f1636eff111a5a)  
**Reviewed:** 2026-11-01; static code only, no token use, live tests, dependency trust audit or runtime measurements.

## What makes it different?

This is a compact (~282-line) Python CLI optimized for **running multiple user accounts simultaneously**, rather than sophisticated Quest task correctness. It uses [main.py](https://github.com/unconsents/auto-quest-completer/blob/6d3b9a085dc78ce42d85dd9748f1636eff111a5a/main.py), [requirements.txt](https://github.com/unconsents/auto-quest-completer/blob/6d3b9a085dc78ce42d85dd9748f1636eff111a5a/requirements.txt) (`discord.py-self`, `aiohttp`, `colorama`), and [config.json](https://github.com/unconsents/auto-quest-completer/blob/6d3b9a085dc78ce42d85dd9748f1636eff111a5a/config.json). The `tokens.txt` filename is a design red flag: it stores **plaintext personal-account credentials**. A GitHub checkout is not a safe location for real tokens.

It does **not** require Discord desktop to start. It does require running Python and libraries and access to Discord's network service.

## Structure by function group

| File/function | What it does |
|---|---|
| `main.py` L1–75 | Imports, ANSI/color helpers, progress bar and text display |
| `load_config()`, `load_tokens()` L80–92 | Loads settings and user credentials from disk |
| `req_headers()` L111–117 | Builds common HTTP request headers |
| `http_get`, `http_post` L118–129 | `aiohttp` wrappers; reject non-2xx responses |
| `get_task()` L137–148 | First-matching task identifier from hardcoded priority list |
| `split_quests()` L157–168 | Divides enrolled vs unenrolled nonexpired, noncompleted Quests |
| `run_video()` L169–194 | Progress-reporting loop for video types |
| `run_heartbeat()` L195–211 | Generic heartbeat loop shared across desktop, legacy desktop, streaming and Activity tasks |
| `run_account()` L213–253 | Per-account Quest listing, enrollment, concurrent task handling |
| `main()`, `login()` L255–282 | Starts a selfbot Gateway client for each credential and runs account work |

## Call graph, explained

```text
main
  -> load tokens.txt and config
  -> login(token) for each account concurrently
       -> discord.py-self gateway client
       -> after login/ready: run_account(token)
            -> aiohttp.ClientSession
            -> list Quests
            -> split already enrolled vs not enrolled
            -> try enroll one by one
            -> asyncio.gather(task jobs for that account)
                 -> run_video OR run_heartbeat
       -> show per-account output
```

It combines **two distinct network layers**:

1. **discord.py-self** manages a personal-account Gateway login; this library is not the standard official Discord bot SDK usage.
2. **aiohttp** opens its own HTTP session and sends Quest-related requests asynchronously.

`asyncio.gather` is used both to handle multiple account logins and to process tasks belonging to one account. The script does not impose a global task semaphore/concurrency ceiling.

## Understanding its data decisions

**Task priority:** `task_priority` contains video, mobile video, desktop gameplay, historical desktop V2, streaming and Activity. `get_task` returns the first present key, not the task group `join_operator` decision. There is no general handling of an AND group. A fallback can select unknown non-skipped keys, though the dispatcher does not have a handler for them.

**Skipped task types:** Both achievement families and Xbox/PlayStation are explicitly in `skip_tasks`; **these are unsupported**, even if a README markets broad Quest coverage. The historical `PLAY_ON_DESKTOP_V2` identifier should not count as an additional contemporary category.

**Enrollment state:** After attempting enrollment, if the returned object is empty the script locally **invents an `enrolled_at` timestamp** and appends the Quest to the enrolled list. This creates a risk of claiming enrollment succeeded when the server did not confirm it. [main.py L213–235](https://github.com/unconsents/auto-quest-completer/blob/6d3b9a085dc78ce42d85dd9748f1636eff111a5a/main.py#L213-L235).

## Core handlers

### Video

The loop advances a time-based progress counter and reads responses, but after a final attempt it logs `video complete` even if the server didn't confirm a completed Quest. Local progress is not an authoritative substitute for server completion. [main.py L169–194](https://github.com/unconsents/auto-quest-completer/blob/6d3b9a085dc78ce42d85dd9748f1636eff111a5a/main.py#L169-L194).

### Generic heartbeat

`run_heartbeat` shares a single style of heartbeat processing among `PLAY_ON_DESKTOP`, deprecated `PLAY_ON_DESKTOP_V2`, `STREAM_ON_DESKTOP` and `PLAY_ACTIVITY`. This means the code **does not establish the specific true requirements of streaming, real game presence or embedded activity sessions**. It loops `while True`, catches/logs errors, and sleeps before retrying. If Discord never confirms enough progress, there is no exit deadline or stuck-credit threshold. [main.py L195–211](https://github.com/unconsents/auto-quest-completer/blob/6d3b9a085dc78ce42d85dd9748f1636eff111a5a/main.py#L195-L211).

### Completion and rewards

It can print `All quests finished!` after `asyncio.gather`, even when individual tasks logged failure or were skipped. It does not independently verify **claimability / claimed_at / redemption** for each completed task; there is no full auto-claim component in this core script. [main.py L238–253](https://github.com/unconsents/auto-quest-completer/blob/6d3b9a085dc78ce42d85dd9748f1636eff111a5a/main.py#L238-L253).

## HTTP/reliability observations

- The `aiohttp` wrappers raise exceptions for non-200/non-204 responses and lack a **specialized bounded 429 retry/backoff policy**.
- The script catches exceptions and in some loops continues indefinitely. Because the event loop has asynchronous sleeps, it can keep many tasks active without a per-task deadline.
- A token login error logs the **first 16 characters** of the personal token, an unnecessary secret disclosure into logs. It then removes the failing token from the plaintext file. [main.py L257–276](https://github.com/unconsents/auto-quest-completer/blob/6d3b9a085dc78ce42d85dd9748f1636eff111a5a/main.py#L257-L276).
- Plaintext token storage is a material threat if backups, logs, malicious dependencies, or repository commits leak.
- `activity_channel_id` exists in configuration, but the generic heartbeat code must not be assumed to establish a genuine active voice session solely because the setting is present.
- A full malicious-code or transitive dependency audit was **not** performed, and these risks do not prove the author intended harm.

## Key source-backed deficits

| Severity | Deficit | Source |
|---|---|---|
| High | Multiple personal tokens stored in plaintext | [main.py L77–93](https://github.com/unconsents/auto-quest-completer/blob/6d3b9a085dc78ce42d85dd9748f1636eff111a5a/main.py#L77-L93) |
| High | First task wins; no AND/OR | [main.py L137–148](https://github.com/unconsents/auto-quest-completer/blob/6d3b9a085dc78ce42d85dd9748f1636eff111a5a/main.py#L137-L148) |
| High | Unbounded heartbeat loop | [main.py L195–211](https://github.com/unconsents/auto-quest-completer/blob/6d3b9a085dc78ce42d85dd9748f1636eff111a5a/main.py#L195-L211) |
| High | False-success possible on video/enrollment | [main.py L169–194, L213–235](https://github.com/unconsents/auto-quest-completer/blob/6d3b9a085dc78ce42d85dd9748f1636eff111a5a/main.py#L169-L194) |
| Medium | Raw token fragment written to output | [main.py L257–276](https://github.com/unconsents/auto-quest-completer/blob/6d3b9a085dc78ce42d85dd9748f1636eff111a5a/main.py#L257-L276) |

## Verdict

**Useful to study:** Python `asyncio`, concurrent tasks, per-account sessions, and `aiohttp` response handling.  
**Weak for reliability/security:** Plaintext credential collection, broad task claims backed by generic methods, indefinite loops, missing final validation and optimistic logs. **Not a good standard for safe account authentication, accurate success reporting or Quest validation.**
