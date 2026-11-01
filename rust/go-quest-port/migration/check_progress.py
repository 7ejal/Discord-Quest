#!/usr/bin/env python3
"""Validate and synchronize the Rust migration task tracker.

Usage:
    python3 migration/check_progress.py --check
    python3 migration/check_progress.py --write

Only touches migration/PROGRESS.md in --write mode.
Does not access Discord, secrets, the internet, or any account.
"""
from __future__ import annotations

import argparse
import json
import re
import sys
from collections import Counter
from pathlib import Path

HERE = Path(__file__).resolve().parent
TRACKER = HERE / "progress.json"
MARKDOWN = HERE / "PROGRESS.md"
STATUSES = {
    "done": ("DONE", "✅ Done"),
    "partial": ("PARTIAL", "🟡 Partial"),
    "todo": ("TODO", "⬜ Todo"),
    "excluded": ("EXCLUDED", "⛔ Excluded"),
}
TASK_ID = re.compile(r"^[A-Z]+-\d{3}$")
ROW = re.compile(
    r"(?m)^(\| \x60(?P<id>[A-Z]+-\d{3})\x60 \| )(?P<status>DONE|PARTIAL|TODO|EXCLUDED)(?P<tail> \|)"
)


def validate_tracker(data: dict) -> tuple[list[dict], Counter, list[str]]:
    problems: list[str] = []
    tasks = data.get("tasks")
    if not isinstance(tasks, list):
        return [], Counter(), ["progress.json: tasks must be an array"]

    ids = []
    for index, task in enumerate(tasks):
        if not isinstance(task, dict):
            problems.append(f"tasks[{index}] must be an object")
            continue
        task_id = task.get("id")
        if not isinstance(task_id, str) or not TASK_ID.fullmatch(task_id):
            problems.append(f"tasks[{index}]: malformed ID {task_id!r}")
            continue
        ids.append(task_id)
        if task.get("status") not in STATUSES:
            problems.append(f"{task_id}: invalid status {task.get('status')!r}")
        for field in ("phase", "title", "acceptance"):
            if not isinstance(task.get(field), str) or not task[field].strip():
                problems.append(f"{task_id}: missing {field}")
        for field in ("evidence", "depends_on", "go_source", "rust_paths"):
            if not isinstance(task.get(field), list):
                problems.append(f"{task_id}: {field} must be an array")
        if task.get("status") == "done" and not task.get("evidence"):
            problems.append(f"{task_id}: DONE requires explicit evidence")

    seen = set()
    for task_id in ids:
        if task_id in seen:
            problems.append(f"Duplicate task ID: {task_id}")
        seen.add(task_id)

    for task in tasks:
        if not isinstance(task, dict):
            continue
        task_id = task.get("id", "?")
        for dep in task.get("depends_on", []):
            if dep not in seen:
                problems.append(f"{task_id}: unknown dependency {dep}")
            if dep == task_id:
                problems.append(f"{task_id}: may not depend on itself")

    counts = Counter(t["status"] for t in tasks if isinstance(t, dict) and t.get("status") in STATUSES)
    return tasks, counts, problems


def update_markdown(text: str, tasks: list[dict], counts: Counter) -> tuple[str, list[str]]:
    problems: list[str] = []
    by_id = {task["id"]: task for task in tasks if isinstance(task, dict) and isinstance(task.get("id"), str)}
    occurrences: Counter = Counter()

    def replace_row(match: re.Match[str]) -> str:
        task_id = match.group("id")
        occurrences[task_id] += 1
        task = by_id.get(task_id)
        if task is None:
            problems.append(f"PROGRESS.md contains unknown ID: {task_id}")
            return match.group(0)
        status = STATUSES[task["status"]][0]
        return f"{match.group(1)}{status}{match.group('tail')}"

    text = ROW.sub(replace_row, text)
    for task_id in by_id:
        if occurrences[task_id] == 0:
            problems.append(f"PROGRESS.md missing task status row for {task_id}")
        elif occurrences[task_id] != 1:
            problems.append(f"PROGRESS.md repeats task status row for {task_id}")

    for state, (_, title) in STATUSES.items():
        pattern = re.compile(rf"(?m)^(\| {re.escape(title)} \| )\d+( \|)")
        if len(pattern.findall(text)) != 1:
            problems.append(f"PROGRESS.md missing/duplicate summary for {state}")
        text = pattern.sub(lambda m: f"{m.group(1)}{counts[state]}{m.group(2)}", text)

    total = re.compile(r"(?m)^(\| \*\*Total\*\* \| \*\*)\d+(\*\* \|)")
    if len(total.findall(text)) != 1:
        problems.append("PROGRESS.md missing/duplicate Total summary")
    text = total.sub(lambda m: f"{m.group(1)}{len(tasks)}{m.group(2)}", text)
    return text, problems


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--check", action="store_true", help="fail if tracker/Markdown disagree")
    mode.add_argument("--write", action="store_true", help="update counts and existing task status rows")
    args = parser.parse_args()

    try:
        data = json.loads(TRACKER.read_text(encoding="utf-8"))
        current = MARKDOWN.read_text(encoding="utf-8")
    except (OSError, ValueError) as exc:
        print(f"Failed reading tracker: {exc}", file=sys.stderr)
        return 2

    tasks, counts, problems = validate_tracker(data)
    if problems:
        for message in problems:
            print(f"ERROR: {message}", file=sys.stderr)
        return 1

    expected, rendered_problems = update_markdown(current, tasks, counts)
    if rendered_problems:
        for message in rendered_problems:
            print(f"ERROR: {message}", file=sys.stderr)
        print("New task IDs must be added as rows in PROGRESS.md.", file=sys.stderr)
        return 1

    if args.write:
        if expected != current:
            MARKDOWN.write_text(expected, encoding="utf-8")
            print("Updated PROGRESS.md from progress.json.")
        else:
            print("PROGRESS.md already synchronized.")
        return 0

    if expected != current:
        print("PROGRESS.md differs from progress.json. Run --write and commit both.", file=sys.stderr)
        return 1

    print(
        f"Tracker consistent: {len(tasks)} tasks, "
        f"{counts['done']} done, {counts['partial']} partial, "
        f"{counts['todo']} todo, {counts['excluded']} excluded."
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
