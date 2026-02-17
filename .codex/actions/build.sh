#!/usr/bin/env bash
set -euo pipefail
pnpm -C ui build
cargo tauri build --debug
