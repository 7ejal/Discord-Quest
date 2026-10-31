# Offline verification plan for Quest-method research

**Purpose:** Test the *truthfulness and robustness* of Quest data handling, without using a personal Discord token, live Quest, simulated rewarded engagement, CAPTCHA-solving, or hidden client internals. These are research fixtures; not a tool for obtaining rewards.

## Test philosophy

A robust implementation must distinguish:

- A requested or locally estimated progress value.
- A task progress value actually returned by the server.
- A task-level `completed_at`.
- A Quest-level `user_status.completed_at`, with correct AND/OR task groups.
- A reward claim value `user_status.claimed_at`.
- External redemption, which may occur outside Discord and cannot be inferred from the other fields.

**Use dependency injection and fake response objects.** No real credentials, real Quest IDs, HTTP requests or external endpoints are needed.

## Suggested source-independent state model

```text
NOT_STARTED
  -> DISCOVERED
  -> ELIGIBLE
  -> ENROLLED
  -> PROGRESSING
  -> TASK_SATISFIED (not automatically Quest completed)
  -> QUEST_CONFIRMED (server status)
  -> CLAIMABLE
  -> CLAIMED
  -> REDEEMED_EXTERNALLY

Any transient state can additionally be:
  UNKNOWN | UNSUPPORTED | BLOCKED | RATE_LIMITED | STALLED | FAILED | EXPIRED
```

### Pseudocode for a strictly read-only validator

```text
evaluate(quest):
    parse configuration version, task group and progress records
    if expired or not enrolled: report state without guessing completion
    if join_operator unknown: return UNSUPPORTED
    for every task: classify progress, target and completed timestamp
    group_satisfied = all or any completed tasks (based on join operator)
    if not group_satisfied: return INCOMPLETE
    if quest.user_status.completed_at missing:
        return TASKS_APPEAR_SATISFIED_BUT_QUEST_NOT_CONFIRMED
    if quest.user_status.claimed_at missing:
        return QUEST_CONFIRMED_REWARD_NOT_CLAIMED
    return CLAIMED (external redemption is still unknown)
```

No network action is present. This is for comparing the correctness of outputs from other software.

## Synthetic fixture matrix

| Case | Input / simulation | Expected correct result | Which source weakness it targets |
|---|---|---|---|
| F01 | `AND` group: first of two tasks complete, second incomplete, top-level completion null | **INCOMPLETE** | Go/aiko/unconsents/lfathh first-task selection |
| F02 | `OR` group: alternate task complete, top-level completion still null | **TASKS_APPEAR_SATISFIED**, *not* server confirmed | All five, general status distinction |
| F03 | Task numeric value = target; task `completed_at` null, Quest `completed_at` null | **NOT SERVER-CONFIRMED** | SUDIN desktop, xgg percentage completion, local timer flows |
| F04 | Video local progress starts at target, top-level completion null | **NOT SERVER-CONFIRMED** | SUDIN video initialization |
| F05 | Mock HTTP 200 with empty body after progress attempt | **UNKNOWN, NOT COMPLETED** | Go/aiko/unconsents/hosted optimistic success |
| F06 | Five consecutive fake 429 responses, with fabricated retry intervals | **FINITE RESTART/PAUSE, never unbounded recursion** | Go recursive retry |
| F07 | Heartbeat returns same nonzero progress forever in test clock | **STALLED or TIMEOUT** | Go/aiko/SUDIN/unconsents loops |
| F08 | Several independent eligible Quests requiring different app identities | **Do not merge display/game context across Quests** | SUDIN single `matches[0]` presence |
| F09 | Missing per-task app ID; top-level fallback present | **Explicit provenance of chosen ID; flag uncertainty** | Go/aiko top-level-only assumptions |
| F10 | Config `task_config_v2` exists, legacy `task_config` absent | **Supported parse or UNSUPPORTED**, not crash | lfathh legacy config |
| F11 | Unknown `join_operator` and two unknown tasks | **UNSUPPORTED** | All first-task selection heuristics |
| F12 | Task handler throws, consumer has an EventEmitter `error` listener | **FAILED**, not `COMPLETED` | xgg-2 swallowed error |
| F13 | Task handler throws, consumer has **no** error listener | **FAILED**, with error surfaced | xgg-2 EventEmitter behavior |
| F14 | Final server request throws; local timer says 100% | **FAILED or UNKNOWN**, not completed | SubhuJaat07 local timestamp |
| F15 | Quest is completed but reward `claimed_at` null | **COMPLETED_NOT_CLAIMED** | Go/aiko commented-out claim routines |
| F16 | Quest is claimed but external redemption unknown | **CLAIMED_REDEMPTION_UNKNOWN** | Reward lifecycle |
| F17 | OAuth-like grant succeeds, subsequent mock action fails | **CLEANUP ATTEMPTED; no privilege left** | Go/aiko early-return cleanup |
| F18 | Token-like secret accidentally placed in a diagnostic exception | **SECRET REDACTED** | unconsents token prefix, hosted cookie |
| F19 | End user cancels status check while simulated network call stalls | **CANCELLED, resources released** | Missing whole-task cancellation |
| F20 | Console task with only generic desktop progress method | **CONDITIONAL/UNVERIFIED**, not working | Go/aiko console claims |
| F21 | Stream task with no actual viewer evidence | **CONDITIONAL/UNVERIFIED**, not working | Go/unconsents/xgg-2 stream claims |
| F22 | Two tasks share name but different per-task application IDs | **Preserve correct per-task ID** | SUDIN advantage vs top-level-only code |

## Failing implementation anti-patterns

```text
BAD:  if local_percent >= 100: print("Quest completed")
BAD:  if request_status == 200: print("Quest completed")
BAD:  if first_task_complete: mark_entire_AND_group_complete
BAD:  emit("error") without propagating failure into top-level result
BAD:  if no live progress: retry forever
BAD:  if reward claim request was attempted: report "claimed"
```

**Correct diagnostic principle:** A program must not transform **unknown** into **success**. It should clearly distinguish source evidence and label unsupported tasks.

## Mock-only test environment

- Use invented IDs and synthetic Quest JSON; **never record or reuse personal account tokens**.
- Use a fake HTTP transport that returns predefined responses, with configurable simulated time.
- Use a fake Gateway event source for READY, disconnect, and heartbeat timing.
- Use deterministic clocks so the tests finish immediately, not 10–25 minutes.
- Run tests of parser and status assessment **without** making Discord requests.
- Publish assertions and exact expected outputs, not screenshots of a real account.
- Record runtime and memory of the **mock test harness** only when measured, not extrapolated to production.
- Include cleanup tests for cancellation, reconnection and permission handling.

## Prioritized conclusions

1. **Source correctness first:** A false-completed outcome invalidates reliability comparisons, regardless of how many task labels exist.
2. **Parser semantics second:** Handle AND/OR and task-specific application identification.
3. **Bounded requests and tasks third:** HTTP timeout alone is insufficient; task-level cancellation/stall detection also matters.
4. **Security always:** Offline fixtures should never require real account credentials.
5. **No universal coverage claim:** A test with fake data proves program logic, not that Discord will accept a simulated Quest.

For the detailed source context, see [method deep dive](METHODS-DEEP-DIVE-2026-11-01.md), [project code walkthrough index](README.md), and [official Discord policy](https://discord.com/safety/platform-manipulation-policy-explainer).
