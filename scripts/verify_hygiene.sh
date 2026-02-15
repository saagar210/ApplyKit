#!/usr/bin/env bash
set -euo pipefail

if ! command -v rg >/dev/null 2>&1; then
  echo "error: ripgrep (rg) is required for hygiene checks"
  exit 1
fi

tracked_artifacts="$(
  git ls-files | rg \
    '(^target/|^node_modules/|^ui/node_modules/|^ui/dist/|^src-tauri/gen/|\.DS_Store$|\.tsbuildinfo$|\.log$|\.tmp$)' \
    || true
)"

if [[ -n "${tracked_artifacts}" ]]; then
  echo "error: tracked generated/junk artifacts detected:"
  echo "${tracked_artifacts}"
  exit 1
fi

echo "ok: no tracked generated/junk artifacts detected"
