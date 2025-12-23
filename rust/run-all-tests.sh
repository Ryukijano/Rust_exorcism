#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
jobs="${JOBS:-$(command -v nproc >/dev/null 2>&1 && nproc || echo 4)}"

manifests=("$root"/*/Cargo.toml)

if [[ ${#manifests[@]} -eq 0 ]]; then
  echo "No exercise crates found under: $root"
  exit 0
fi

printf '%s\n' "${manifests[@]}" | xargs -n1 -P "$jobs" -I{} cargo test --manifest-path "{}"

