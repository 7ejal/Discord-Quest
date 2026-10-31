# Quest enrollment, progress, completion, claiming, and redemption

**Primary sources:** [Discord Quest FAQ](https://support.discord.com/hc/en-us/articles/22225719947543-Discord-Quests-FAQ) and [unofficial Quest data model](https://docs.discord.food/resources/quests).

## The lifecycle is NOT a single operation

| Stage | Meaning | What does not prove this stage |
|---|---|---|
| Shown/targeted | Discord displays an eligible campaign or ad placement | A link to an expired Quest |
| Accepted/enrolled | Account accepted the specific Quest | Clicking an inert button or a local UI flag |
| Started | Legitimate task underway | A process merely named after the game |
| Progress recorded | Server reports activity progress | Client timer counting up without credited values |
| Completed | Objective satisfied in server status | HTTP 200 on a heartbeat alone |
| Claimable | Reward and campaign conditions allow claim | Completed task if claim window expired |
| Claimed | Discord confirms claimed reward | A claim request that was sent but rejected |
| Redeemed | Code used or entitlement delivered | An unredeemed code screenshot |

## Timing and eligibility

Discord states Quests have individual acceptance and completion periods. Certain promotions may be unavailable by region or account eligibility. On mobile, visibility and tracking remain beta for selected campaigns. Discord currently limits users aged 13–17 to **three Quests per day**, per the FAQ (checked 2026-11-01).

Publisher-reported game actions can take **up to 36 hours** to be reflected. An activity-specific provider might have separate availability and confirmation timing.

## Reward types

The unofficial data model currently enumerates:

1. **REWARD_CODE:** redemption code for a partner.
2. **IN_GAME:** a reward associated with the promoted game.
3. **COLLECTIBLE:** Discord avatar decoration or similar collectible.
4. **VIRTUAL_CURRENCY:** Discord Orbs.
5. **FRACTIONAL_PREMIUM:** limited premium/Nitro entitlement.

Reward distribution can be **ALL** or **TIERED**, and selected platforms can matter. Reward expiry may differ from campaign expiry. **Do not claim reward codes in public research evidence.**

## Official claiming experience

Discord says a completed Quest can be claimed through the Quest bar or Quest Home; eligible mobile views offer claim actions too. For partnered games, Discord supplies a redemption code plus partner website instructions; for Discord-specific decorations and Orbs, the claimed reward is added to the account.

A project reporting *auto-claim* can fail on CAPTCHA, reward-platform requirements, sealed campaign metadata, expired/removed campaigns, account eligibility, or server-side errors. [QuestCompleter](https://github.com/saintordevil/questCompleter) emphasizes native claiming with challenge UI and treats server rejection as authoritative. [SubhuJaat07](https://github.com/SubhuJaat07/discord-quest-completor) documents a manual-claim limitation.

## How to evaluate a claim implementation

- Does the project distinguish *completed* from *claimed*?
- Does it validate server timestamps and returned errors?
- Does it handle multiple reward platforms and tiered rewards?
- Does it fail closed on unexpected response schema?
- Does it keep CAPTCHA under Discord's real verification UI without logging challenge data?
- Does it avoid reading or publishing reward codes or private entitlements?
- Does it account for account restrictions and regional eligibility?
- Does it suppress duplicate claims and allow recovery after reconnect?

## Clarifications about API fields

The community reference lists user_status.enrolled_at, completed_at, claimed_at, progress entries and a separate reward-code object. These explain state, but **are not a supported public Discord bot API**. This repository documents the model, not an endorsed request recipe.

## Useful references

- [Official FAQ, claiming section](https://support.discord.com/hc/en-us/articles/22225719947543-Discord-Quests-FAQ)
- [Unpublished/internal Quest resource structure, community-maintained](https://docs.discord.food/resources/quests)
- [QuestCompleter reward and retry discussion](https://github.com/saintordevil/questCompleter)
- [Orion claim and CAPTCHA limitations](https://github.com/nyxxbit/discord-quest-completer)
