# Source audit: lfathh/Auto-Quest-Discord

**Upstream:** https://github.com/lfathh/Auto-Quest-Discord  
**Pinned revision:** [5fe8bd958571fea5e953d23a211d3472cea3ebce](https://github.com/lfathh/Auto-Quest-Discord/tree/5fe8bd958571fea5e953d23a211d3472cea3ebce)  
**Reviewed:** 2026-11-01, targeted static inspection.

## Repository shape

Includes a TypeScript client and manager under `src/`, a `bot.ts` entry point, and a second legacy single-file `code.js` console-style implementation. The bot contains formatted progress dashboards and concurrent work reporting.

## Source-backed findings

- `src/questManager.ts` selects one of five task identifiers by **fixed order**, not generic AND/OR group semantics: video, desktop, stream, Activity, mobile video. [manager.ts L120–139](https://github.com/lfathh/Auto-Quest-Discord/blob/5fe8bd958571fea5e953d23a211d3472cea3ebce/src/questManager.ts#L120-L139).
- The TypeScript manager has video and desktop handlers, while stream and Activity branches **only print explanatory messages**, rather than providing verified standalone execution. [manager.ts L189–240](https://github.com/lfathh/Auto-Quest-Discord/blob/5fe8bd958571fea5e953d23a211d3472cea3ebce/src/questManager.ts#L189-L240).
- Legacy `code.js` has separate browser/console branches, including streaming and activity paths; that is a **different runtime**, and its features should **not** be credited to the TypeScript standalone selfbot. [code.js](https://github.com/lfathh/Auto-Quest-Discord/blob/5fe8bd958571fea5e953d23a211d3472cea3ebce/code.js).
- `bot.ts` uses `Promise.allSettled` for jobs and outputs a summary, which is useful for presentation but does not itself verify server completion. [bot.ts L194–270](https://github.com/lfathh/Auto-Quest-Discord/blob/5fe8bd958571fea5e953d23a211d3472cea3ebce/bot.ts#L194-L270).
- No evidence of generic console or publisher-game achievement completion in the reviewed TypeScript code.

## Verdict

A useful secondary comparison for dashboards and the distinction between legacy console scripts and standalone clients. Do **not** count its console code's streaming/activity functionality as proof of the standalone client supporting these types.
