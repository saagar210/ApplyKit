#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

echo "clean_bloat.sh is kept for compatibility; forwarding to clean_local_caches.sh"
exec ./scripts/clean_local_caches.sh "$@"
