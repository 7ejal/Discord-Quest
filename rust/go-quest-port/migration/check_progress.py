#!/usr/bin/env python3
"""Validate and generate PROGRESS.md from the canonical migration/progress.json.

Usage:
  python3 migration/check_progress.py --check
  python3 migration/check_progress.py --write

No network, credentials, application access or external packages are used.
"""
from __future__ import annotations

import argparse
import difflib
import json
import re
import sys
import subprocess
from collections import Counter
from pathlib import Path

HERE = Path(__file__).resolve().parent
SOURCE = HERE / "progress.json"
DASHBOARD = HERE / "PROGRESS.md"
STATUSES = ("done", "partial", "todo", "excluded")
PRIORITIES = ("P0", "P1", "P2", "P3")
ID_RE = re.compile(r"^[A-Z]+-\d{3}$")
TICK = chr(96)
PHASES = (
    "inventory", "audit", "build", "models", "parser", "state",
    "cli", "ux", "concurrency", "network", "tests", "quality",
    "security", "release", "tracking", "docs", "excluded",
)


def validate_tracker(data: object) -> list[str]:
    errors = []
    if not isinstance(data, dict):
        return ["Tracker root must be an object"]
    if data.get("schema_version") != 2:
        errors.append("schema_version must be 2")
    tasks = data.get("tasks")
    if not isinstance(tasks, list):
        return errors + ["tasks must be an array"]

    ids = set()
    for i, task in enumerate(tasks):
        if not isinstance(task, dict):
            errors.append(f"tasks[{i}] is not an object")
            continue
        task_id = task.get("id")
        if not isinstance(task_id, str) or not ID_RE.fullmatch(task_id):
            errors.append(f"tasks[{i}]: invalid ID {task_id!r}")
            continue
        if task_id in ids:
            errors.append(f"{task_id}: duplicate task ID")
        ids.add(task_id)

        if task.get("status") not in STATUSES:
            errors.append(f"{task_id}: invalid status")
        if task.get("priority") not in PRIORITIES:
            errors.append(f"{task_id}: invalid priority, expected P0..P3")
        gate = task.get("required_for_release")
        if not isinstance(gate, bool):
            errors.append(f"{task_id}: required_for_release must be boolean")
        if task.get("status") == "excluded" and gate is not False:
            errors.append(f"{task_id}: excluded task cannot gate release")
        for field in ("phase", "title", "acceptance"):
            if not isinstance(task.get(field), str) or not task[field].strip():
                errors.append(f"{task_id}: missing {field}")
        if task.get("phase") not in PHASES:
            errors.append(f"{task_id}: unrecognized phase {task.get('phase')!r}")
        for field in ("go_source", "rust_paths", "evidence", "depends_on"):
            value = task.get(field)
            if not isinstance(value, list) or any(not isinstance(x, str) for x in value):
                errors.append(f"{task_id}: {field} must be a string array")
        if data.get("traceability_required") and task.get("required_for_release") and task.get("status") == "done" and not task.get("validation"):
            errors.append(f"{task_id}: done release gate requires validation mapping")
        if task.get("status") in ("done", "partial") and not task.get("evidence"):
            errors.append(f"{task_id}: status requires at least one evidence item")

    deps_by_id = {
        t["id"]: t.get("depends_on", [])
        for t in tasks
        if isinstance(t, dict) and isinstance(t.get("id"), str)
        and isinstance(t.get("depends_on"), list)
    }
    for task_id, deps in deps_by_id.items():
        for dep in deps:
            if dep not in ids:
                errors.append(f"{task_id}: unknown dependency {dep}")
            if dep == task_id:
                errors.append(f"{task_id}: self dependency")

    colors = {}
    def visit(node: str, trail: tuple[str, ...]) -> None:
        state = colors.get(node, 0)
        if state == 1:
            errors.append("Dependency cycle: " + " -> ".join(trail + (node,)))
            return
        if state == 2:
            return
        colors[node] = 1
        for child in deps_by_id.get(node, ()):
            if child in deps_by_id:
                visit(child, trail + (node,))
        colors[node] = 2
    for task_id in deps_by_id:
        if colors.get(task_id, 0) == 0:
            visit(task_id, ())
    return errors


def verification_warnings(data: dict) -> list[str]:
    verification = data.get("verification", {})
    sha = verification.get("head_sha", "")
    if verification.get("result") != "success" or not re.fullmatch(r"[0-9a-f]{40}", sha):
        return ["Current source has no successful immutable main-server verification"]
    try:
        result = subprocess.run(
            ["git", "diff", "--quiet", sha, "HEAD", "--", "src", "tests", "scripts",
             "Cargo.toml", "Cargo.lock", "rust-toolchain.toml", "../../tools",
             "migration/check_progress.py", "migration/check_links.py", "migration/test_check_progress.py"],
            cwd=HERE.parent, check=False, capture_output=True,
        )
    except OSError:
        return ["Cannot compare verification commit to current source"]
    return ["Verification source is outdated or unavailable; rerun main-server checks"] if result.returncode else []


def safe_cell(value: object) -> str:
    return str(value).replace("|", "/").replace("\r", " ").replace("\n", " ").strip()


def evidence_cell(task: dict) -> str:
    evidence = task.get("evidence", [])
    if not evidence:
        return "—"
    first = str(evidence[0])
    if first.startswith("https://github.com/"):
        return "[source](" + first + ")"
    return TICK + safe_cell(first).replace(TICK, "'") + TICK


def dependency_queues(tasks: list[dict]) -> tuple[list[dict], list[tuple[dict, list[dict]]]]:
    """Return actionable tasks and blocked tasks, using actual dependency status."""
    by_id = {task["id"]: task for task in tasks}
    open_gates = [task for task in tasks if task["required_for_release"] and task["status"] != "done"]
    priority_key = lambda task: (PRIORITIES.index(task["priority"]), task["id"])
    ready = []
    blocked = []
    for task in open_gates:
        unfinished = [by_id[dep] for dep in task["depends_on"] if by_id[dep]["status"] != "done"]
        if unfinished:
            blocked.append((task, unfinished))
        else:
            ready.append(task)
    return sorted(ready, key=priority_key), sorted(blocked, key=lambda entry: priority_key(entry[0]))


def render(data: dict) -> str:
    tasks = data["tasks"]
    counts = Counter(t["status"] for t in tasks)
    gates = [t for t in tasks if t["required_for_release"]]
    open_gates = [t for t in gates if t["status"] != "done"]
    lines = [
        "# Go to Rust — migration progress (generated)",
        "",
        "**This file is generated from [progress.json](progress.json). Do not edit it directly.**",
        "Run " + TICK + "python3 migration/check_progress.py --write" + TICK
        + " after changing JSON, and " + TICK
        + "python3 migration/check_progress.py --check" + TICK + " before committing.",
        "",
        "Project: [rust/go-quest-port](../) · Source: [pinned upstream Go]"
        + "(https://github.com/yeweroooo/Discord-Quest-Auto-Completion-Selfbot/tree/"
        + "352d7f457c9511bc9e7c9b940a71012bfb47572e)",
        "",
        "**Scope:** native, read-only/offline Quest data analysis. Not an operational user-account"
        + " selfbot or an all-games reward-completion system. Excluded tasks are intentional.",
        "",
        "## Task counts",
        "",
        "| Status | Count | Interpretation |",
        "|---|---:|---|",
        f"| DONE | {counts['done']} | Acceptance evidence recorded; check verification commit |",
        f"| PARTIAL | {counts['partial']} | Some implementation exists but acceptance incomplete |",
        f"| TODO | {counts['todo']} | Work not yet demonstrated |",
        f"| EXCLUDED | {counts['excluded']} | Deliberately outside approved scope |",
        f"| **TOTAL** | **{len(tasks)}** | Inventory snapshot, not a completion percentage |",
        "",
        "## Release-gating status (approved offline scope only)",
        "",
        f"**{len(gates)} required tasks · {len(open_gates)} open release gates ·"
        f" {len(gates) - len(open_gates)} done release gates.**",
        "Optional and excluded tasks do not block release. See [release criteria]"
        + "(RELEASE-GATES.md). A green historical CI run does not verify future commits.",
        "",
        "## Ready to start (all dependencies DONE)",
        "",
        "| ID | Priority | Work | Dependencies |",
        "|---|---|---|---|",
    ]
    ready, blocked = dependency_queues(tasks)
    for task in ready[:18]:
        deps = ", ".join(TICK + dep + TICK for dep in task["depends_on"]) or "—"
        lines.append(
            "| " + TICK + task["id"] + TICK + " | " + task["priority"]
            + " | " + safe_cell(task["title"]) + " | " + deps + " |"
        )
    if not ready:
        lines.append("| — | — | No unblocked release tasks | — |")
    lines.extend([
        "",
        "## Blocked release gates (prerequisites not DONE)",
        "",
        "| ID | Priority | Work | Unfinished dependencies (status/priority) |",
        "|---|---|---|---|",
    ])
    for task, unfinished in blocked[:18]:
        blockers = ", ".join(
            TICK + item["id"] + TICK + " " + item["status"].upper() + "/" + item["priority"]
            for item in unfinished
        )
        lines.append(
            "| " + TICK + task["id"] + TICK + " | " + task["priority"]
            + " | " + safe_cell(task["title"]) + " | " + blockers + " |"
        )
    if not blocked:
        lines.append("| — | — | No blocked release tasks | — |")
    lines.extend([
        "",
        "## Detailed task inventory",
        "",
        "All tasks have exact acceptance, evidence and dependencies in the JSON source."
        + " Do not change status without verifying acceptance against the **current code**.",
        "",
    ])
    seen_phases = set()
    for phase in PHASES:
        items = [t for t in tasks if t["phase"] == phase]
        if not items:
            continue
        seen_phases.add(phase)
        lines.extend([
            "### " + phase.replace("_", " ").title(),
            "",
            "| ID | Status | Priority | Gate | Work | Acceptance | Depends on | Evidence (first) |",
            "|---|---|---|---|---|---|---|---|",
        ])
        for task in sorted(items, key=lambda x: (PRIORITIES.index(x["priority"]), x["id"])):
            deps = ", ".join(TICK + dep + TICK for dep in task["depends_on"]) or "—"
            line = (
                "| " + TICK + task["id"] + TICK
                + " | " + task["status"].upper()
                + " | " + task["priority"]
                + " | " + ("Required" if task["required_for_release"] else "Optional/excluded")
                + " | " + safe_cell(task["title"])
                + " | " + safe_cell(task["acceptance"])
                + " | " + deps
                + " | " + evidence_cell(task)
                + " |"
            )
            lines.append(line)
        lines.append("")
    lines.extend([
        "## Evidence caveat and AI update contract",
        "",
        "- The historical [successful Linux/Windows build]"
        + "(https://github.com/7ejal/Discord-Quest/actions/runs/41256646888)"
        + " verified 15 Rust tests and a Windows smoke test **only for its head commit**.",
        "- Task status is not a claim of current CI status, performance, or universal gameplay.",
        "- For a change, select IDs; implement + test; record current-commit evidence in JSON;"
        + " regenerate this file; rerun CI; report remaining gaps.",
        "- Review [START-HERE.md](START-HERE.md), [GAP-AUDIT.md](GAP-AUDIT.md),"
        + " [GO-FILE-MAP.md](GO-FILE-MAP.md), [DECISIONS.md](DECISIONS.md),"
        + " [SPEC.md](SPEC.md), and [CAPABILITY-CONTRACT.md](CAPABILITY-CONTRACT.md).",
        "",
    ])
    return "\n".join(lines)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--check", action="store_true", help="verify the generated dashboard")
    mode.add_argument("--write", action="store_true", help="regenerate dashboard from JSON")
    args = parser.parse_args()
    try:
        data = json.loads(SOURCE.read_text(encoding="utf-8"))
    except (OSError, ValueError) as exc:
        print(f"Failed reading progress.json: {exc}", file=sys.stderr)
        return 2
    errors = validate_tracker(data)
    if errors:
        for error in errors:
            print("ERROR:", error, file=sys.stderr)
        return 1
    for warning in verification_warnings(data):
        print("WARNING:", warning, file=sys.stderr)
    expected = render(data)
    if args.write:
        DASHBOARD.write_text(expected, encoding="utf-8")
        print(f"Generated {DASHBOARD.name} from {len(data['tasks'])} canonical tasks.")
        return 0
    try:
        actual = DASHBOARD.read_text(encoding="utf-8")
    except OSError as exc:
        print(f"Missing dashboard: {exc}", file=sys.stderr)
        return 1
    if actual != expected:
        print("PROGRESS.md out of sync; run --write and commit both files.", file=sys.stderr)
        differences = difflib.unified_diff(
            actual.splitlines(), expected.splitlines(),
            fromfile="PROGRESS.md", tofile="generated expected", lineterm=""
        )
        for index, line in enumerate(differences):
            if index >= 65:
                print("...diff truncated...", file=sys.stderr)
                break
            print(line, file=sys.stderr)
        return 1
    counts = Counter(t["status"] for t in data["tasks"])
    print(
        f"Tracker valid: {len(data['tasks'])} tasks,"
        f" {counts['done']} done, {counts['partial']} partial,"
        f" {counts['todo']} todo, {counts['excluded']} excluded."
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
