#!/usr/bin/env bash
set -euo pipefail

project_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$project_dir"

if [[ "$(uname -s)" != "Linux" || "$(uname -m)" != "x86_64" ]]; then
  echo "This project is supported/tested on Linux x86_64 only." >&2
  exit 2
fi

export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-2}"
# Avoid rustup silently downloading a missing compiler onto the production host.
export RUSTUP_AUTO_INSTALL=0
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

# Network-isolated smoke test: no Quest API, gateway, token or web access.
# The connected main-server is capable of unshare --net; this test fails closed
# instead of silently assuming that successful ordinary execution proves offline use.
if ! command -v unshare >/dev/null 2>&1 || ! unshare --net -- true; then
  echo "Cannot verify offline mode: Linux network namespace unavailable" >&2
  exit 1
fi
unshare --net -- ./target/release/go-quest-port demo --json --at 2026-11-01T12:00:00Z > "$temp_dir/offline.json"
cmp -s "$temp_dir/offline.json" tests/golden/demo.json || {
  echo "Isolated-network golden JSON report mismatch" >&2
  exit 1
}
TOKEN="TEST_FAKE_TOKEN_CANARY" YES_CAPTCHA_API_KEY="TEST_FAKE_CANARY" \
  unshare --net -- ./target/release/go-quest-port inspect tests/fixtures/excluded_demo.json --json --at 2026-11-01T12:00:00Z > "$temp_dir/offline-excluded.json"
python3 - "$temp_dir/offline-excluded.json" <<'PY'
import json, pathlib, sys
raw = pathlib.Path(sys.argv[1]).read_text(encoding="utf-8")
data = json.loads(raw)
assert data["analyzed"] == 1 and data["excluded_count"] == 1
for secret in ("TEST_FAKE_TOKEN_CANARY", "TEST_FAKE_CANARY", "HIDDEN_TARGETING_SECRET", "HIDDEN_TRAFFIC_SECRET"):
    assert secret not in raw, "untrusted input or environment was echoed"
print("Network-isolated CLI and privacy test: OK")
PY

echo "PASS: Linux x86_64, tracker, fmt, clippy, tests, release build, JSON smoke"
