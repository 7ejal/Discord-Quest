# Quest schema, logic, and validation model

**Sources:** [Unofficial Quest reference](https://docs.discord.food/resources/quests) (community-maintained; may change without notice), [Discord FAQ](https://support.discord.com/hc/en-us/articles/22225719947543-Discord-Quests-FAQ) (official user behavior).

## Major entities (research-level)

- **Quest:** id, config, optional user_status, preview and traffic metadata.
- **Config:** config_version, starts_at, expires_at, task_config_v2, application, messages, rewards_config and feature flags.
- **Task group:** join_operator of **and** or **or** and a map of task identifiers to their definitions.
- **Task definition:** type, target, optional applications, external_ids, assets, event_name and account-link instructions.
- **User status:** enrolled_at, completed_at, claimed_at, progress map and optional other fields.
- **Task progress:** event_name, value, updated_at, completed_at, sometimes heartbeat with last_beat_at and expires_at.
- **Reward configuration:** assignment method (all/tiered), reward list, expiration, platforms.
- **Reward outcome:** may include claimed_at, entitlements and error information.

## Historical versions

The current unofficial reference lists **config version 2 active** and **version 1 discontinued**. Older implementations may still parse legacy fields. Do not presume an old configuration maps directly to current task_config_v2.

## AND / OR evaluation

For an **and** group, every required task must be completed. For an **or** group, at least one task must be completed. The task's **target** is a duration in seconds only for duration-oriented tasks; it should not be interpreted universally as a countdown.

Example (synthetic, never from a live account):

~~~json
{
  "config_version": 2,
  "task_config_v2": {
    "join_operator": "or",
    "tasks": {
      "PLAY_ON_DESKTOP": {
        "type": "PLAY_ON_DESKTOP",
        "target": 900,
        "applications": [{"id": "123456789012345678"}]
      },
      "PLAY_ON_XBOX": {
        "type": "PLAY_ON_XBOX",
        "target": 900,
        "external_ids": ["sample-game-id"]
      }
    }
  }
}
~~~

For the synthetic OR example, completing either permitted path should satisfy the objective. A client selecting a convenient path is not proof that the account or platform is eligible.

## A reliable validation state model

1. **Discovered:** campaign appears for an account; does not mean eligible or enrolled.
2. **Eligible:** current date/region/account/platform permit participation.
3. **Enrolled:** user_status.enrolled_at set by server.
4. **In progress:** server reports nonzero progress or a server-recognized ongoing task.
5. **Task complete:** progress entry has completed_at where applicable.
6. **Quest complete:** user_status.completed_at set, satisfying AND/OR.
7. **Claimable:** reward is available to the account within the claim window.
8. **Claimed:** user_status.claimed_at or server claim result confirms it.
9. **Redeemed:** external code actually redeemed, or in-app entitlement delivered (separate result).

Do not infer step 6 from step 4 or step 8 from step 6. In particular, a client-side percent display is not authoritative.

## Validation requirements for a fixture parser

- Reject unknown join operators rather than silently picking a task.
- Treat a missing task group as **unknown schema**, not automatic success.
- Accept task identifiers as strings; preserve unknown task types for research.
- Use per-task application identifiers where present; legacy config.application alone can be wrong for multi-path campaigns.
- Keep game achievements as externally verified outcomes; do not replace missing evidence with locally invented progress.
- Handle expiry, start times, platform eligibility and account restrictions independently.
- Preserve integer/decimal semantics and clamp display percentages only, not server evidence.
- Do not log secrets, sealed metadata, user tokens, actual game-account IDs or reward codes.
- Recognize a deprecated task without claiming current runtime support.
- Differentiate server-time measurements from locally estimated countdowns.

## Useful edge-case fixtures

- OR {desktop, Xbox}, where Xbox is unavailable.
- AND {video, activity}, only video is complete.
- Missing progress entry but nonzero client timer.
- Unknown join operator and unknown task type.
- Expired Quest with a previously claimed reward.
- Eligible user who has not enrolled; enrollment cooldown.
- Same campaign with a changed config_version or task application.
- Claim challenge while unrelated tasks remain pending.
- Game achievement delivered hours after play.
- Empty Quest store on cold boot (Orion documents this).
- Conflicting platform selection for an in-game reward.

## Evidence to log

Only privacy-safe entries: source URL, source snapshot date, client/version, task type, config version, expected state, observed state, error category, evidence classification. For fixture tests use invented IDs. Never collect any real user's credentials.

Reference: [Orion architecture and changelog](https://github.com/nyxxbit/discord-quest-completer), [QuestCompleter logic tests](https://github.com/saintordevil/questCompleter).
