# Source audit: unconsents/auto-quest-completer (Python)

**Upstream:** https://github.com/unconsents/auto-quest-completer  
**Pinned revision:** [6d3b9a085dc78ce42d85dd9748f1636eff111a5a](https://github.com/unconsents/auto-quest-completer/tree/6d3b9a085dc78ce42d85dd9748f1636eff111a5a)  
**Reviewed:** 2026-11-01, static source inspection.

## Architecture

A compact `main.py` (~282 lines), with `aiohttp` for requests, basic config, a list of Discord user tokens from `tokens.txt`, concurrent account work, video progress and generalized heartbeat task loops.

## Source findings

- **High: personal credentials in a plaintext file.** `load_tokens()` directly reads `tokens.txt`; when login fails the program rewrites the file to remove that token. Treat the file as a highly sensitive secret, not regular configuration. [main.py L77–93](https://github.com/unconsents/auto-quest-completer/blob/6d3b9a085dc78ce42d85dd9748f1636eff111a5a/main.py#L77-L93), [L255–277](https://github.com/unconsents/auto-quest-completer/blob/6d3b9a085dc78ce42d85dd9748f1636eff111a5a/main.py#L255-L277).
- **High: fixed priority rather than AND/OR.** `get_task` chooses the first recognized task; there is no composite-Quest dependency handling. [main.py L137–148](https://github.com/unconsents/auto-quest-completer/blob/6d3b9a085dc78ce42d85dd9748f1636eff111a5a/main.py#L137-L148).
- **High: heartbeat loop can continue indefinitely.** `run_heartbeat` runs `while True` with no hard elapsed-time or failed-progress limit and catches/logs exceptions before sleeping. [main.py L195–211](https://github.com/unconsents/auto-quest-completer/blob/6d3b9a085dc78ce42d85dd9748f1636eff111a5a/main.py#L195-L211).
- **Medium: video success can be overstated.** `run_video` logs completion after progress attempts even if server confirmation was not established. [main.py L169–194](https://github.com/unconsents/auto-quest-completer/blob/6d3b9a085dc78ce42d85dd9748f1636eff111a5a/main.py#L169-L194).
- **Medium: generic heartbeat groups.** Fixed stream-key construction for multiple task families does not prove stream or genuine desktop activity. [main.py L195–211](https://github.com/unconsents/auto-quest-completer/blob/6d3b9a085dc78ce42d85dd9748f1636eff111a5a/main.py#L195-L211).
- **Unsupported:** Explicitly skips both achievement types and console tasks. [main.py L99–102](https://github.com/unconsents/auto-quest-completer/blob/6d3b9a085dc78ce42d85dd9748f1636eff111a5a/main.py#L99-L102).
- **Positive:** Simple readable control flow and task-specific redaction-friendly progress counters, and split of enrolled versus unenrolled Quest work.

**Verdict:** Research value for observing compact async Python organization, but its credential risk, no watchdog, broad catch-and-retry and optimistic success logs make it a weak reliability reference. No code was executed.
