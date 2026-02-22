#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"

rm -rf .perf-results
mkdir -p .perf-results

pnpm -C ui build
node scripts/perf/bundle-report.mjs
node scripts/perf/measure-build-time.mjs pnpm -C ui build
node --expose-gc scripts/perf/memory-smoke.mjs
bash scripts/perf/check-assets.sh
node scripts/perf/summarize.mjs

echo "ok: perf foundation metrics captured in .perf-results/"
