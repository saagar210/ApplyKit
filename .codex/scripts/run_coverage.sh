#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"

rm -rf coverage
mkdir -p coverage/ui

if ! cargo llvm-cov --version >/dev/null 2>&1; then
  echo "cargo-llvm-cov is required; install with: cargo install cargo-llvm-cov --locked" >&2
  exit 1
fi

echo "Running Rust coverage (lcov)..."
cargo llvm-cov \
  --workspace \
  --all-targets \
  --lcov \
  --output-path coverage/rust.lcov

echo "Running UI coverage (vitest v8)..."
pnpm -C ui test:coverage

node ./.codex/scripts/check_coverage_thresholds.mjs
