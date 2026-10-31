# Research methodology and verification checklist

## Collection method

1. Start with official Discord Quest FAQ, Cloud Play FAQ, policy and security documentation.
2. Consult [docs.discord.food](https://docs.discord.food/resources/quests) as **unofficial** protocol research, not a stable supported API.
3. Review relevant public project READMEs and selected source/architecture references.
4. Record a source URL, date, canonical project path, repository default branch and evidence classification.
5. Compare conflicting reports rather than treating the broadest feature table as correct.
6. Track updates to schemas and features without assuming future stability.
7. Use only synthetic fixtures and mock responses for local validation.

## Evidence strength

| Label | Necessary evidence |
|---|---|
| OFFICIAL | Direct Discord support/policy statement |
| REFERENCE | Community resource documenting an observed structure |
| IMPLEMENTED | Specific code path or repository documentation |
| MAINTAINER_TESTED | Author's dated run or reproducible report, with scope |
| INDEPENDENTLY_VERIFIED | Dated reproducible third-party confirmation of *server-confirmed completion* |
| UNTESTED | Explicit warning or absence of meaningful validation |
| UNSUPPORTED | Explicit lack of handling or authoritative limitation |
| DEPRECATED | Unofficial schema or documentation marks removed/legacy |
| CONDITIONAL | Only works with an authentic platform/game/stream/linked account or campaign eligibility |
| CONTRADICTED | Relevant sources report incompatible outcomes |

**Implemented is not the same as works.** Tests against a fake server demonstrate code correctness, not current real Discord acceptance.

## What a complete evidence record needs

- Observation date, source reference, project/revision.
- Specific task identifier and Quest configuration *without* personally identifying data.
- Test platform (desktop/mobile/web) and Discord application version where available.
- Preconditions, especially actual gameplay/stream/viewer/platform account.
- Redacted server-confirmed progress and Quest completion state.
- Whether reward eligibility or claim was confirmed separately.
- Alternative explanations, limitations, and possible expiry.
- Whether source is author-provided or independently reproduced.
- Re-test date, owner and corrected status.

## Recommended deterministic mock tests

- AND/OR task groups, legacy config and unknown tasks.
- Server progress unchanged while UI counter rises.
- Empty store during startup, later populated.
- Retryable/transient vs permanent errors.
- Reward expiry and tiered or platform-limited claims.
- Disconnect/restart while a task or claim is pending.
- Double processing of the same Quest ID.
- Stream with no viewer; linked console offline.
- Malformed response and unexpected field type.
- Canceled task cleans up local data and worker generation.
- Missing entitlements or failed claim despite task completion.
- Data model version change without silent fallback.

## Claims that need special skepticism

- “Supports all quests” without enumerated tasks and dated evidence.
- “Working” and “untested” in the same cell.
- “Complete instantly” despite a genuine timed participation requirement.
- “Captcha free” without a documented explanation of normal verification.
- Generic heartbeat described as proof of console or external achievement completion.
- Local console output as the only success evidence.
- Screenshots that do not show the campaign's server-confirmed result.

## Current limitations

Research was not a complete audit of every commit or dependency in every linked project; no live Quest automation, personal-token login, reward claiming, or malware sandbox execution was performed. Project links can change and authors may ship fixes after this snapshot.

## How to publish corrections

Create an issue including the disputed line, primary source URL, date and the smallest reproducible **redacted** evidence. Cite the originator of each observation and preserve older historical claims with dates rather than deleting the trail.
