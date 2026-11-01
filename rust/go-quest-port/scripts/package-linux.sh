#!/usr/bin/env bash
set -euo pipefail

project_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$project_dir"

# A package must describe exactly committed source, never local edits.
if [[ -n "$(git status --porcelain --untracked-files=normal)" ]]; then
  echo "Refusing to package a dirty checkout" >&2
  exit 1
fi

# The archive is distributed only after a complete test/build on this host.
bash scripts/test-linux.sh

version="$(sed -n 's/^version = "\([^"]*\)"/\1/p' Cargo.toml | head -n 1)"
commit="$(git rev-parse --short=12 HEAD)"
if [[ -z "$version" || -z "$commit" ]]; then
  echo "Unable to determine version or commit" >&2
  exit 1
fi
name="go-quest-port-${version}-linux-x86_64-${commit}"
target_dir="$project_dir/dist/linux-x86_64"
mkdir -p "$target_dir"
tmp="$(mktemp -d)"
trap 'rm -rf -- "$tmp"' EXIT
mkdir -p "$tmp/$name"
install -m 0755 target/release/go-quest-port "$tmp/$name/go-quest-port"
cp README.md LICENSE NOTICE "$tmp/$name/"
cp Cargo.lock "$tmp/$name/"
python3 - "$project_dir/Cargo.lock" > "$tmp/$name/LOCKED-DEPENDENCIES.txt" <<'PY'
import pathlib, sys, tomllib
lock = tomllib.loads(pathlib.Path(sys.argv[1]).read_text(encoding="utf-8"))
packages = sorted(lock["package"], key=lambda p: (p["name"], p["version"]))
print("Locked Cargo dependency inventory (names and versions only)")
print("Individual third-party license review is a separate release task.")
for package in packages:
    print(package["name"], package["version"])
PY

cargo metadata --locked --offline --format-version 1 --filter-platform x86_64-unknown-linux-gnu > "$tmp/metadata.json"
python3 - "$tmp/metadata.json" > "$tmp/$name/DEPENDENCY-LICENSES.txt" <<'PY'
import json, pathlib, sys
metadata = json.loads(pathlib.Path(sys.argv[1]).read_text())
for package in sorted(metadata["packages"], key=lambda p: (p["name"], p["version"])):
    license_id = package.get("license") or ""
    if not license_id:
        raise SystemExit("Missing dependency license declaration")
    print(package["name"], package["version"], license_id)
PY

cat > "$tmp/$name/BUILD-INFO.txt" <<EOF
OS: Linux
Architecture: x86_64
Package version: $version
Git commit: $(git rev-parse HEAD)
Compiler: $(rustc --version)
Cargo: $(cargo --version)
Cargo.lock SHA256: $(sha256sum Cargo.lock | cut -d' ' -f1)
Verification: scripts/test-linux.sh passed on this machine
Source trust: unverified local JSON; not a Discord selfbot
EOF

(cd "$tmp/$name" && sha256sum go-quest-port README.md LICENSE NOTICE BUILD-INFO.txt LOCKED-DEPENDENCIES.txt DEPENDENCY-LICENSES.txt Cargo.lock > SHA256SUMS)
archive="$target_dir/$name.tar.gz"
tar -C "$tmp" -czf "$archive" "$name"
sha256sum "$archive" > "$archive.sha256"
tar -tzf "$archive" >/dev/null
# Validate the unpacked bytes, not just the staging directory.
verify_dir="$(mktemp -d)"
trap 'rm -rf -- "$tmp" "$verify_dir"' EXIT
tar -xzf "$archive" -C "$verify_dir"
(cd "$verify_dir/$name" && sha256sum --check --status SHA256SUMS)
"$verify_dir/$name/go-quest-port" demo --json --at 2026-11-01T12:00:00Z > "$tmp/release-demo.json"
cmp -s "$tmp/release-demo.json" tests/golden/demo.json

"$verify_dir/$name/go-quest-port" --version
echo "Linux archive: $archive"
echo "SHA256:"
cat "$archive.sha256"
echo "Binary dependencies:"
if command -v ldd >/dev/null 2>&1; then
  ldd "$tmp/$name/go-quest-port"
else
  echo "(ldd unavailable)"
fi

