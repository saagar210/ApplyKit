#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

usage() {
  echo "Usage: ./scripts/clean_bloat.sh [--dry-run]"
}

dry_run=0
if [[ "${1:-}" == "--dry-run" ]]; then
  dry_run=1
elif [[ -n "${1:-}" ]]; then
  usage
  exit 1
fi

cleanup_dirs=(
  "target"
  "node_modules"
  "ui/node_modules"
  "ui/dist"
  "src-tauri/target"
  "src-tauri/gen"
  ".codex_audit"
  "coverage"
  ".cache"
  ".turbo"
)

cleanup_patterns=(
  ".DS_Store"
  "Thumbs.db"
  "._*"
  "*.tsbuildinfo"
  ".eslintcache"
  "*.log"
  "*.tmp"
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

clean_pattern() {
  local pattern="$1"
  if [[ "$dry_run" -eq 1 ]]; then
    find . -path "./.git" -prune -o -type f -name "$pattern" -print | sed 's/^/would remove file: /'
    return 0
  fi

  find . -path "./.git" -prune -o -type f -name "$pattern" -print -delete | sed 's/^/removed file: /'
}

echo "Cleaning generated artifacts and junk files in: $repo_root"
if [[ "$dry_run" -eq 1 ]]; then
  echo "Dry run mode enabled; no files will be deleted."
fi

for dir in "${cleanup_dirs[@]}"; do
  remove_dir "$dir"
done

for pattern in "${cleanup_patterns[@]}"; do
  clean_pattern "$pattern"
done

echo "Cleanup complete."
