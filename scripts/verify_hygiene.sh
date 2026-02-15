#!/usr/bin/env bash
set -euo pipefail

artifact_pattern='(^target/|^node_modules/|^ui/node_modules/|^ui/dist/|^src-tauri/gen/|\.DS_Store$|\.tsbuildinfo$|\.log$|\.tmp$)'

if command -v rg >/dev/null 2>&1; then
  tracked_artifacts="$(git ls-files | rg "$artifact_pattern" || true)"
else
  tracked_artifacts="$(git ls-files | grep -E "$artifact_pattern" || true)"
fi

if [[ -n "${tracked_artifacts}" ]]; then
  echo "error: tracked generated/junk artifacts detected:"
  echo "${tracked_artifacts}"
  exit 1
fi

echo "ok: no tracked generated/junk artifacts detected"
