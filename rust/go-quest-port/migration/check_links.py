#!/usr/bin/env python3
"""Verify local Markdown links in the active Rust project's AI documentation.

Only checks repository-relative paths. HTTP URLs are intentionally not fetched.
"""
from __future__ import annotations

import re
import sys
from pathlib import Path
from urllib.parse import unquote, urlsplit

ROOT = Path(__file__).resolve().parents[3]
PROJECT = ROOT / "rust" / "go-quest-port"
FILES = [ROOT / "README.md", ROOT / "AGENTS.md"]
FILES += list(PROJECT.glob("*.md"))
FILES += list((PROJECT / "migration").glob("*.md"))
LINK = re.compile(r"\]\(([^)\s]+)(?:\s+['\"][^)]*['\"])?\)")


def evidence_errors(tracker: dict, project: Path, root: Path) -> list[str]:
    failures = []
    for task in tracker.get("tasks", []):
        if task.get("status") != "done":
            continue
        for reference in task.get("evidence", []):
            if "://" in reference or reference.startswith("#"):
                continue
            if not any((base / reference).exists() for base in (project, root)):
                failures.append(f"{task['id']}: evidence path missing: {reference}")
        if tracker.get("traceability_required") and task.get("required_for_release"):
            validation = task.get("validation", "")
            path = project / validation
            if not path.is_file() or task["id"] not in path.read_text(encoding="utf-8"):
                failures.append(f"{task['id']}: missing executable/non-code acceptance mapping")
    return failures


def broken_links() -> list[str]:
    failures: list[str] = []
    for path in sorted(set(FILES)):
        if not path.exists():
            failures.append(f"{path.relative_to(ROOT)}: required Markdown source missing")
            continue
        text = path.read_text(encoding="utf-8")
        for match in LINK.finditer(text):
            uri = match.group(1)
            if uri.startswith(("#", "mailto:", "data:")) or "://" in uri:
                continue
            rel = unquote(urlsplit(uri).path)
            if not rel:
                continue
            target = (path.parent / rel).resolve()
            if not target.is_relative_to(ROOT.resolve()) or not target.exists():
                failures.append(f"{path.relative_to(ROOT)}: broken relative link {uri}")
    # Prevent previously observed literal escaped newline corruption in README.
    root_readme = (ROOT / "README.md").read_text(encoding="utf-8")
    if "\\n\\n" in root_readme:
        failures.append("README.md: literal escaped double-newline sequence found")

    # Every local evidence reference for a DONE task must correspond to a real
    # file/directory in the active project or repository root. Historical URLs
    # remain references, not current-commit validation.
    import json
    tracker = json.loads((PROJECT / "migration" / "progress.json").read_text(encoding="utf-8"))
    failures.extend(evidence_errors(tracker, PROJECT, ROOT))
    return failures


if __name__ == "__main__":
    errors = broken_links()
    for error in errors:
        print("ERROR:", error, file=sys.stderr)
    if errors:
        raise SystemExit(1)
    print(f"Verified local Markdown links in {len(set(FILES))} active Rust/agent docs.")
