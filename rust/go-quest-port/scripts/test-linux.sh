#!/usr/bin/env bash
set -euo pipefail

project_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$project_dir"

if [[ "$(uname -s)" != "Linux" || "$(uname -m)" != "x86_64" ]]; then
  echo "This project is supported/tested on Linux x86_64 only." >&2
  exit 2
fi

export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-2}"
echo "Commit: $(git rev-parse HEAD)"
echo "Toolchain: $(rustc --version)"
python3 migration/check_progress.py --check
python3 migration/test_check_progress.py
python3 migration/check_links.py
cargo fmt --all -- --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --all-targets --locked
cargo build --release --locked

temp_dir="$(mktemp -d)"
trap 'rm -rf -- "$temp_dir"' EXIT

./target/release/go-quest-port demo --json --at 2026-11-01T12:00:00Z > "$temp_dir/demo.json"
cmp -s "$temp_dir/demo.json" tests/golden/demo.json || {
  echo "Golden Linux JSON report mismatch" >&2
  exit 1
}

./target/release/go-quest-port inspect tests/fixtures/excluded_demo.json --json --at 2026-11-01T12:00:00Z > "$temp_dir/excluded.json"

python3 - "$temp_dir/excluded.json" <<'PY'
import json, pathlib, sys
text = pathlib.Path(sys.argv[1]).read_text(encoding="utf-8")
result = json.loads(text)
assert result["schema_version"] == 1
assert result["input_trust"] == "unverified_local_json"
assert result["analyzed"] == 1 and result["excluded_count"] == 1
for secret in ("HIDDEN_TRAFFIC_SECRET", "HIDDEN_TARGETING_SECRET", "HIDDEN_SEALED_SECRET"):
    assert secret not in text
print("Synthetic privacy and JSON verification: OK")
PY

echo "PASS: Linux x86_64, tracker, fmt, clippy, tests, release build, JSON smoke"
