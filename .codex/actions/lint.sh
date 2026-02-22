#!/usr/bin/env bash
set -euo pipefail
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
pnpm -C ui lint
