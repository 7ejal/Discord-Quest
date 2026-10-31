# Source audit: SUDIN010/discord-auto-quest (Python)

**Upstream:** https://github.com/SUDIN010/discord-auto-quest  
**Pinned revision:** [c14afa1dbb3c8c59d7f68c90caa305eff8e15b0e](https://github.com/SUDIN010/discord-auto-quest/tree/c14afa1dbb3c8c59d7f68c90caa305eff8e15b0e)  
**Reviewed:** 2026-11-01, static code.

## Structure and strengths

Single-file `main.py` (~546 lines). A `QuestClient` wraps Python Requests and handles 429, 5xx, transport exceptions and timeouts using a **bounded MAX_RETRIES** loop. It separates task discovery, enrollment, video tracking and desktop activity. It attempts to check server-returned progress more carefully than an unconditional timer, **but numeric progress at target can still be treated as Quest completed without top-level `completed_at`.**

## Findings with code references

| Area | Observation | Evidence |
|---|---|---|
| Network resilience | Bounded retry loop handles 429/5xx; requests have a timeout; hard errors are surfaced. | [main.py L76–116](https://github.com/SUDIN010/discord-auto-quest/blob/c14afa1dbb3c8c59d7f68c90caa305eff8e15b0e/main.py#L76-L116) |
| Task scope | Explicit handler routing for `PLAY_ON_DESKTOP` and the two video task identifiers only. Streaming, activity, console and achievements are not implemented. | [main.py L401–408](https://github.com/SUDIN010/discord-auto-quest/blob/c14afa1dbb3c8c59d7f68c90caa305eff8e15b0e/main.py#L401-L408) |
| Task correctness | Desktop loop checks response status/progress, stops when progress data is absent, and distinguishes a completion log from an incomplete one, **but task progress reaching target also sets success without Quest-level confirmation**. | [main.py L326–359](https://github.com/SUDIN010/discord-auto-quest/blob/c14afa1dbb3c8c59d7f68c90caa305eff8e15b0e/main.py#L326-L359) |
| Video confidence | Usually reports incomplete when no completed_at is returned, **except when cached progress initially meets the target; `finished` can then be true without fresh completed_at**. | [main.py L361–398](https://github.com/SUDIN010/discord-auto-quest/blob/c14afa1dbb3c8c59d7f68c90caa305eff8e15b0e/main.py#L361-L398) |
| Known gap | Gameplay loop is not subject to a fixed overall deadline if Discord keeps returning unchanged nonempty progress. | [main.py L338–352](https://github.com/SUDIN010/discord-auto-quest/blob/c14afa1dbb3c8c59d7f68c90caa305eff8e15b0e/main.py#L338-L352) |
| Presence selection | `run()` builds one Gateway presence from `matches[0]`, then processes later Quests without updating that application presence. | [main.py L473–505](https://github.com/SUDIN010/discord-auto-quest/blob/c14afa1dbb3c8c59d7f68c90caa305eff8e15b0e/main.py#L473-L505) |\n| Task selection | Main dispatcher does not implement general AND/OR task-group satisfaction. | [main.py L401–408](https://github.com/SUDIN010/discord-auto-quest/blob/c14afa1dbb3c8c59d7f68c90caa305eff8e15b0e/main.py#L401-L408) |
| Claims | Manual reward claiming, as explicitly documented by the author. | [README](https://github.com/SUDIN010/discord-auto-quest) |

## Security and applicability

Requires a personal user token, which is sensitive. A simpler and narrower scope is easier to reason about than broad unsupported promises, but it is not inherently safe or policy-compliant. No test suite or live Quest verification was established.

**Verdict:** Among the small standalone Python implementations reviewed, it has clear **bounded HTTP error handling** and task-specific app ID parsing, but limited task types, locally over-optimistic completion edges, stale multi-Quest presence and no general completion guarantee. More detail: [method deep dive](../research/standalone/METHODS-DEEP-DIVE-2026-11-01.md).
