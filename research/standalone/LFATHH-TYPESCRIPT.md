# Detailed source walkthrough: lfathh/Auto-Quest-Discord (TypeScript CLI)

**Pinned upstream:** [main @ 5fe8bd958571fea5e953d23a211d3472cea3ebce](https://github.com/lfathh/Auto-Quest-Discord/tree/5fe8bd958571fea5e953d23a211d3472cea3ebce)  
**Reviewed:** 2026-11-01. Static source inspection only, no tests or live Quest execution.

## What the project contains

This repository has **two substantially different approaches**:

1. A **TypeScript standalone Node.js selfbot** running `bot.ts`, `src/client.ts`, `src/questManager.ts`, `src/quest.ts`, and related data/constants.
2. A separate **legacy browser/console-style `code.js` script** with its own task branches. Those are **not automatically available in the standalone Node CLI**.

That distinction is essential: an old console script's streaming path does not prove its separate CLI supports streaming.

## Runtime and dependencies

The [package.json](https://github.com/lfathh/Auto-Quest-Discord/blob/5fe8bd958571fea5e953d23a211d3472cea3ebce/package.json) requires **Node >=18** and includes `@discordjs/core`, `@discordjs/rest`, `@discordjs/ws`, `discord-api-types`, `undici`, and the terminal UI packages `chalk`, `figlet`, `cli-table3`. The entry command uses `tsx` and an environment file. No installed graphical Discord client is required for the CLI to launch.

## Program entry and user interface

`bot.ts` creates a `ClientQuest`, subscribes to READY, retrieves valid Quests and uses `Promise.allSettled` to perform asynchronous work, while periodically updating a terminal dashboard. It has a hardcoded `QUEST_DB` mapping some IDs to names and expected durations and a fallback that parses task information. This is useful for readable output but **not** evidence that the Quest has been completed.

The helper `getQuestData()` reads `quest.config.task_config.tasks` and takes the **first key** for duration. This can misdescribe newer v2 task-group campaigns and makes the displayed countdown a poor substitute for server state. [bot.ts L95–130](https://github.com/lfathh/Auto-Quest-Discord/blob/5fe8bd958571fea5e953d23a211d3472cea3ebce/bot.ts#L95-L130), [L194–270](https://github.com/lfathh/Auto-Quest-Discord/blob/5fe8bd958571fea5e953d23a211d3472cea3ebce/bot.ts#L194-L270).

## Quest management

`src/questManager.ts` handles campaign selection and work. The main path:

```text
READY -> get valid Quests -> maybe enroll
 -> read config.task_config
 -> choose first of: VIDEO, DESKTOP, STREAM, ACTIVITY, MOBILE_VIDEO
 -> call appropriate branch -> print result / render dashboard
```

The program reads the **older `task_config`** property rather than reliably using current `task_config_v2`. According to the [unofficial Quest reference](https://docs.discord.food/resources/quests), config version 1 is discontinued and version 2 is current. This may cause an absent-property/runtime error on modern Quests rather than graceful unsupported-task handling.

Task selection again takes the first recognized key, not AND/OR logic. [questManager.ts L119–137](https://github.com/lfathh/Auto-Quest-Discord/blob/5fe8bd958571fea5e953d23a211d3472cea3ebce/src/questManager.ts#L119-L137).

## What code actually executes

### Video and mobile video

A timestamp-oriented handler uses cached progress and elapsed time since enrollment; after a final attempt it logs `Quest completed!` **even if it hasn't verified the final server completion status**. This is an unreliable success signal. [questManager.ts L139–188](https://github.com/lfathh/Auto-Quest-Discord/blob/5fe8bd958571fea5e953d23a211d3472cea3ebce/src/questManager.ts#L139-L188).

### Desktop gameplay

A heartbeat loop checks a cached `Quest` object updated from responses, and emits a success message after the local completion flag becomes true. If that state never changes, the code has no overall timeout, creating potential endless processing. [questManager.ts L189–222](https://github.com/lfathh/Auto-Quest-Discord/blob/5fe8bd958571fea5e953d23a211d3472cea3ebce/src/questManager.ts#L189-L222).

### Streaming and embedded activity

In the **TypeScript standalone manager**, both are effectively unsupported: streaming logs a message explaining it cannot be performed in Node and Activity logs an unsupported message. These are **not real CLI implementations**. [questManager.ts L223–241](https://github.com/lfathh/Auto-Quest-Discord/blob/5fe8bd958571fea5e953d23a211d3472cea3ebce/src/questManager.ts#L223-L241).

### Achievements / consoles

No general handlers for achievement-in-game, achievement-in-activity, Xbox or PlayStation were present in the standalone manager's task selection. Do not credit it support based on labels found elsewhere.

### Legacy `code.js`

`code.js` (~265 lines) has distinct browser-client branches for video, game, stream and Activity. It belongs to an in-client console-script context, which may depend on Discord's running client stores. This **cannot be counted** as proof of runtime capabilities of `bot.ts`. [code.js](https://github.com/lfathh/Auto-Quest-Discord/blob/5fe8bd958571fea5e953d23a211d3472cea3ebce/code.js).

## Network architecture

`src/client.ts` builds a Discord.js REST client and Gateway `WebSocketManager`, using an overridden `WebSocketShard.prototype.send` method for Identify. Its architecture resembles the aiko TypeScript project, but with older declared library versions. [client.ts](https://github.com/lfathh/Auto-Quest-Discord/blob/5fe8bd958571fea5e953d23a211d3472cea3ebce/src/client.ts).

## Issues and teaching points

| Severity | Issue | File |
|---|---|---|
| **High** | Legacy `task_config` assumption instead of current v2 task map | [questManager.ts L125–136](https://github.com/lfathh/Auto-Quest-Discord/blob/5fe8bd958571fea5e953d23a211d3472cea3ebce/src/questManager.ts#L125-L136) |
| **High** | Streaming and Activity are explicitly nonworking in standalone manager | [questManager.ts L223–241](https://github.com/lfathh/Auto-Quest-Discord/blob/5fe8bd958571fea5e953d23a211d3472cea3ebce/src/questManager.ts#L223-L241) |
| **High** | First-matching task does not evaluate AND/OR | [questManager.ts L127–136](https://github.com/lfathh/Auto-Quest-Discord/blob/5fe8bd958571fea5e953d23a211d3472cea3ebce/src/questManager.ts#L127-L136) |
| **High** | Video completion log does not always require confirmed server completion | [questManager.ts L139–188](https://github.com/lfathh/Auto-Quest-Discord/blob/5fe8bd958571fea5e953d23a211d3472cea3ebce/src/questManager.ts#L139-L188) |
| **Medium** | Terminal dashboard contains estimated metadata from a hardcoded Quest database and first task | [bot.ts L8–66, L95–130](https://github.com/lfathh/Auto-Quest-Discord/blob/5fe8bd958571fea5e953d23a211d3472cea3ebce/bot.ts#L95-L130) |
| **Medium** | Legacy script and standalone code can cause misleading cross-runtime feature claims | [code.js](https://github.com/lfathh/Auto-Quest-Discord/blob/5fe8bd958571fea5e953d23a211d3472cea3ebce/code.js), [bot.ts](https://github.com/lfathh/Auto-Quest-Discord/blob/5fe8bd958571fea5e953d23a211d3472cea3ebce/bot.ts) |

**Verdict:** A useful teaching example of a terminal dashboard and older Discord.js architecture. It is **not** a competitive modern standalone project for multi-task Quest configs or real stream/activity/achievement coverage. Its CLI may fail when confronted with v2-only task configuration.
