#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TARGET="$ROOT/applykit_pack"

if [ ! -d "$TARGET" ]; then
  echo "applykit_pack directory not found at: $TARGET"
  exit 1
fi

cd "$TARGET"

echo "ApplyKit setup (delegated to applykit_pack, non-destructive)."
command -v node >/dev/null 2>&1 && node -v || echo "node: missing"
command -v pnpm >/dev/null 2>&1 && pnpm -v || echo "pnpm: missing"
command -v cargo >/dev/null 2>&1 && cargo --version || echo "cargo: missing"

echo
echo "Install deps (README.md + .github/workflows/ci.yml):"
echo "  pnpm -C ui install --frozen-lockfile"
echo "Lean dev mode (README.md):"
echo "  ./scripts/dev_lean.sh"
