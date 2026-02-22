#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"

bash ./.codex/scripts/run_perf_foundation.sh
mkdir -p .perf-baselines
cp .perf-results/bundle.json .perf-baselines/bundle.json
cp .perf-results/build-time.json .perf-baselines/build-time.json

echo "ok: refreshed .perf-baselines from current perf foundation run"
