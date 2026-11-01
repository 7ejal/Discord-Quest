#!/usr/bin/env bash
set -euo pipefail

project_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$project_dir"

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
cat > "$tmp/$name/BUILD-INFO.txt" <<EOF
OS: Linux
Architecture: x86_64
Package version: $version
Git commit: $(git rev-parse HEAD)
Compiler: $(rustc --version)
Cargo: $(cargo --version)
Verification: scripts/test-linux.sh passed on this machine
Source trust: unverified local JSON; not a Discord selfbot
EOF

(cd "$tmp/$name" && sha256sum go-quest-port README.md LICENSE NOTICE BUILD-INFO.txt > SHA256SUMS)
archive="$target_dir/$name.tar.gz"
tar -C "$tmp" -czf "$archive" "$name"
sha256sum "$archive" > "$archive.sha256"
tar -tzf "$archive" >/dev/null

"$tmp/$name/go-quest-port" --version
echo "Linux archive: $archive"
echo "SHA256:"
cat "$archive.sha256"
echo "Binary dependencies:"
if command -v ldd >/dev/null 2>&1; then
  ldd "$tmp/$name/go-quest-port"
else
  echo "(ldd unavailable)"
fi
