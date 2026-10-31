# Contributing and correction policy

This repository is a source-backed **research archive**, not a place to submit account tokens or scripts for prohibited engagement.

## Submit an evidence-backed correction

An issue/report should contain:

1. Exact file and statement to correct.
2. Canonical source URL, source revision and the date checked.
3. Exact task identifier and compatible Discord platform/version (if relevant).
4. What the developer **claims** vs what a reliable source **demonstrates**.
5. How the evidence changes the status label or support matrix.
6. Redacted, privacy-safe explanation of the limitation.

## Evidence quality

- Prefer Discord official documentation for the product contract.
- Cite unofficial docs as an observed implementation model only.
- Do not label code “working” without independent, reproducible evidence.
- Maintain separate statuses for handler existence, server progress, objective completion and claiming.
- Include **untested**, **conditional**, **unsupported** and **deprecated** explicitly.
- Add test date when comparing Discord client releases.
- Preserve historical bug reports with source dates.

## Security restrictions

Do not submit credentials, auth headers, raw cookies, reward codes, third-party API keys, real account IDs, unredacted screenshots, or secret datasets.

Do not propose bypassing CAPTCHA, access restrictions or detecting/evading enforcement. Keep fixture tests offline with synthetic examples.

## Scope

Welcome: documentation fixes, comparison corrections, citation updates, static security analysis of public source, offline task-config parsers, schema fixtures, validation and read-only charts.

Outside current scope: functional selfbot completion engines, account farming, credential collection and any claim of ban-safe automation.

## Citation links

Use [SOURCES.md](research/SOURCES.md) and [EVIDENCE-LOG.md](research/EVIDENCE-LOG.md) to keep evidence consistent.
