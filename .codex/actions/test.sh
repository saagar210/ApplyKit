#!/usr/bin/env bash
set -euo pipefail
cargo test
pnpm -C ui test
