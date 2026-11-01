#!/usr/bin/env python3
"""Unit tests for the stdlib-only migration tracker generator and validator."""
from __future__ import annotations

import copy
import unittest

from check_progress import render, validate_tracker


def task(task_id: str, *, status: str = "todo", deps: list[str] | None = None) -> dict:
    return {
        "id": task_id,
        "phase": "models",
        "title": "Synthetic tracker task",
        "status": status,
        "priority": "P0",
        "required_for_release": status != "excluded",
        "go_source": ["types.go"],
        "rust_paths": ["src/model.rs"],
        "acceptance": "Mock criterion was checked.",
        "evidence": ["src/model.rs"] if status in ("done", "partial") else [],
        "depends_on": deps or [],
        "notes": "This fixture does not affect real project tasks.",
    }


class TrackerTests(unittest.TestCase):
    def setUp(self) -> None:
        self.data = {
            "schema_version": 2,
            "tasks": [task("MODEL-101", status="done"), task("MODEL-102", deps=["MODEL-101"])],
        }

    def test_valid_fixture(self) -> None:
        self.assertEqual(validate_tracker(self.data), [])

    def test_render_counts_release_gates_and_rows(self) -> None:
        text = render(self.data)
        self.assertIn("| DONE | 1 |", text)
        self.assertIn("| TODO | 1 |", text)
        self.assertIn("2 required tasks · 1 open release gates", text)
        self.assertIn("MODEL-101", text)
        self.assertIn("MODEL-102", text)

    def test_new_task_automatically_appears(self) -> None:
        self.data["tasks"].append(task("MODEL-103"))
        self.assertIn("MODEL-103", render(self.data))

    def test_duplicate_id_rejected(self) -> None:
        self.data["tasks"].append(copy.deepcopy(self.data["tasks"][0]))
        self.assertTrue(any("duplicate task ID" in e for e in validate_tracker(self.data)))

    def test_unknown_dependency_rejected(self) -> None:
        self.data["tasks"][1]["depends_on"] = ["OTHER-999"]
        self.assertTrue(any("unknown dependency" in e for e in validate_tracker(self.data)))

    def test_cycle_rejected(self) -> None:
        self.data["tasks"][0]["depends_on"] = ["MODEL-102"]
        self.assertTrue(any("Dependency cycle" in e for e in validate_tracker(self.data)))

    def test_self_reference_rejected(self) -> None:
        self.data["tasks"][0]["depends_on"] = ["MODEL-101"]
        self.assertTrue(any("self dependency" in e for e in validate_tracker(self.data)))

    def test_priority_type_checked(self) -> None:
        self.data["tasks"][1]["priority"] = "urgent"
        self.assertTrue(any("invalid priority" in e for e in validate_tracker(self.data)))

    def test_excluded_cannot_gate_release(self) -> None:
        self.data["tasks"].append(task("SCOPE-123", status="excluded"))
        self.data["tasks"][-1]["required_for_release"] = True
        self.data["tasks"][-1]["phase"] = "excluded"
        self.assertTrue(any("cannot gate release" in e for e in validate_tracker(self.data)))

    def test_missing_evidence_fails_done_status(self) -> None:
        self.data["tasks"][0]["evidence"] = []
        self.assertTrue(any("requires at least one evidence" in e for e in validate_tracker(self.data)))

    def test_missing_required_fields_fail(self) -> None:
        self.data["tasks"][0]["acceptance"] = ""
        self.assertTrue(any("missing acceptance" in e for e in validate_tracker(self.data)))

    def test_unknown_phase_fails(self) -> None:
        self.data["tasks"][1]["phase"] = "random"
        self.assertTrue(any("unrecognized phase" in e for e in validate_tracker(self.data)))

    def test_wrong_schema_version_fails(self) -> None:
        self.data["schema_version"] = 1
        self.assertTrue(any("schema_version must be 2" in e for e in validate_tracker(self.data)))

    def test_render_is_deterministic(self) -> None:
        self.assertEqual(render(copy.deepcopy(self.data)), render(self.data))


if __name__ == "__main__":
    unittest.main()
