#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

usage() {
  cat <<'EOF'
Usage: ./scripts/dev_lean.sh

Starts Tauri dev with ephemeral cache/build directories and cleans heavy
artifacts on exit.
EOF
}

if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then
  usage
  exit 0
fi

if [[ -n "${1:-}" ]]; then
  usage
  exit 1
fi

lean_tmp_root="$(mktemp -d "${TMPDIR:-/tmp}/applykit-lean.XXXXXX")"
tauri_pid=""

cleanup() {
  local exit_code=$?
  trap - EXIT INT TERM

  if [[ -n "$tauri_pid" ]] && kill -0 "$tauri_pid" >/dev/null 2>&1; then
    kill -INT "$tauri_pid" >/dev/null 2>&1 || true
    sleep 0.5
    pkill -TERM -P "$tauri_pid" >/dev/null 2>&1 || true
    wait "$tauri_pid" >/dev/null 2>&1 || true
  fi

  ./scripts/clean_heavy_artifacts.sh >/dev/null 2>&1 || true

  for _ in $(seq 1 5); do
    rm -rf "$lean_tmp_root" 2>/dev/null || true
    if [[ ! -e "$lean_tmp_root" ]]; then
      break
    fi
    sleep 0.2
  done

  if [[ -e "$lean_tmp_root" ]]; then
    echo "warning: temporary lean cache root could not be fully removed: $lean_tmp_root"
  fi

  exit "$exit_code"
}
trap cleanup EXIT INT TERM

export CARGO_TARGET_DIR="$lean_tmp_root/cargo-target"
export XDG_CACHE_HOME="$lean_tmp_root/xdg-cache"
export TMPDIR="$lean_tmp_root/tmp"
mkdir -p "$XDG_CACHE_HOME" "$TMPDIR"

echo "Lean dev temp root: $lean_tmp_root"
echo "CARGO_TARGET_DIR=$CARGO_TARGET_DIR"
echo "XDG_CACHE_HOME=$XDG_CACHE_HOME"

echo "Launching cargo tauri dev (which runs beforeDevCommand from src-tauri/tauri.conf.json)."
echo "Press Ctrl+C to stop. Heavy artifacts and temporary caches will be cleaned automatically."
cargo tauri dev &
tauri_pid="$!"
wait "$tauri_pid"
