#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"

bash ./.codex/scripts/run_perf_foundation.sh

node scripts/perf/compare-metric.mjs .perf-baselines/bundle.json .perf-results/bundle.json totalBytes "${PERF_BUNDLE_MAX_RATIO:-0.20}"
node scripts/perf/compare-metric.mjs .perf-baselines/build-time.json .perf-results/build-time.json buildMs "${PERF_BUILD_MAX_RATIO:-0.25}"

echo "ok: perf enforced checks passed"
