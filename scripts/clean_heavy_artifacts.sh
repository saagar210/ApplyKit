#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

usage() {
  echo "Usage: ./scripts/clean_heavy_artifacts.sh [--dry-run]"
}

dry_run=0
if [[ "${1:-}" == "--dry-run" ]]; then
  dry_run=1
elif [[ -n "${1:-}" ]]; then
  usage
  exit 1
fi

# Keep dependency caches intact for faster iteration.
cleanup_dirs=(
  "target"
  "src-tauri/target"
  "src-tauri/gen"
  "ui/dist"
  ".next"
  "build"
)

remove_dir() {
  local dir="$1"
  if [[ ! -d "$dir" ]]; then
    return 0
  fi

  if [[ "$dry_run" -eq 1 ]]; then
    echo "would remove directory: $dir"
    return 0
  fi

  find "$dir" -depth -mindepth 1 -delete
  rmdir "$dir"
  echo "removed directory: $dir"
}

echo "Cleaning heavy build artifacts in: $repo_root"
if [[ "$dry_run" -eq 1 ]]; then
  echo "Dry run mode enabled; no files will be deleted."
fi

for dir in "${cleanup_dirs[@]}"; do
  remove_dir "$dir"
done

echo "Heavy artifact cleanup complete."
