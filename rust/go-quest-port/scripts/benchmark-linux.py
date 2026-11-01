#!/usr/bin/env python3
"""Reproducible synthetic Linux-only CLI resource check (no network or tokens)."""
from __future__ import annotations

import argparse
import json
import os
import pathlib
import resource
import subprocess
import tempfile
import time

HERE = pathlib.Path(__file__).resolve().parents[1]
BINARY = HERE / "target" / "release" / "go-quest-port"
AT = "2026-11-01T12:00:00Z"


def make_fixture(n: int) -> dict:
    return {
        "quests": [
            {
                "id": f"synthetic-{i}",
                "config": {
                    "starts_at": "2026-01-01T00:00:00Z",
                    "expires_at": "2027-01-01T00:00:00Z",
                    "task_config_v2": {"tasks": {"WATCH_VIDEO": {"target": 60}}},
                },
                "user_status": None,
            }
            for i in range(n)
        ]
    }


def run_case(n: int) -> None:
    if n not in (100, 10000, 100001):
        raise ValueError("Use only the documented bounded synthetic fixture sizes")
    with tempfile.TemporaryDirectory(prefix="quest-linux-benchmark-") as temp_dir:
        fixture = pathlib.Path(temp_dir) / "synthetic.json"
        with fixture.open("w", encoding="utf-8") as out:
            json.dump(make_fixture(n), out, separators=(",", ":"))
        size = fixture.stat().st_size
        started = time.perf_counter()
        before = resource.getrusage(resource.RUSAGE_CHILDREN)
        # The measured process has strict Linux-only safety caps: 60 CPU seconds,
        # and 512 MiB of virtual address space, and is never given a token.
        def limits() -> None:
            resource.setrlimit(resource.RLIMIT_CPU, (60, 60))
            resource.setrlimit(resource.RLIMIT_AS, (512 * 1024 * 1024, 512 * 1024 * 1024))
            os.nice(10)

        p = subprocess.run(
            [str(BINARY), "inspect", str(fixture), "--json", "--workers", "4", "--at", AT],
            stdout=subprocess.DEVNULL,
            stderr=subprocess.PIPE,
            timeout=90,
            check=False,
            preexec_fn=limits,
        )
        after = resource.getrusage(resource.RUSAGE_CHILDREN)
        elapsed_ms = (time.perf_counter() - started) * 1000
        cpu_ms = (after.ru_utime - before.ru_utime + after.ru_stime - before.ru_stime) * 1000
        expected_success = n <= 10000 and size <= 8 * 1024 * 1024
        if expected_success != (p.returncode == 0):
            raise RuntimeError(f"Unexpected exit for {n} quests: {p.returncode}, {p.stderr[:250]!r}")
        if not expected_success:
            message = p.stderr.decode("utf-8", errors="replace")
            if not ("Too many Quests" in message or "input limit" in message):
                raise RuntimeError(f"Unexpected rejection for {n} quests: {message[:250]}")
        print(json.dumps({
            "case": "expected_rejection" if not expected_success else "accepted",
            "quests": n,
            "input_bytes": size,
            "exit": p.returncode,
            "wall_ms": round(elapsed_ms, 2),
            "cpu_ms": round(cpu_ms, 2),
            "max_child_rss_kib": after.ru_maxrss,
            "max_child_virtual_memory_mib": 512,
            "cpu_time_limit_seconds": 60,
            "binary_bytes": BINARY.stat().st_size,
        }, sort_keys=True))


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--quests", type=int, choices=(100, 10000, 100001), default=10000)
    args = parser.parse_args()
    if os.uname().sysname != "Linux":
        raise SystemExit("Linux only")
    if not BINARY.is_file():
        raise SystemExit("Missing Linux release executable. Run scripts/test-linux.sh first.")
    run_case(args.quests)
