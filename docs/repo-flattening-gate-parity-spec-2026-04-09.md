# Gate and CI Parity Rewrite Spec (Prepared Early)

Prepared on **2026-02-22** for Week 7 Day 4 (**2026-04-09**).

## Goal
Define deterministic, token-level rewrites so verification, CI, and policy checks remain aligned across flattening cutover.

## Current-to-Target Contract Map
| Surface | Current Token/Path | Target Token/Path |
| --- | --- | --- |
| Root verify command | `cd applykit_pack && bash ./.codex/scripts/run_verify_commands.sh` | `bash ./.codex/scripts/run_verify_commands.sh` |
| Root quality workflow UI install | `pnpm -C applykit_pack/ui install --frozen-lockfile` | `pnpm -C ui install --frozen-lockfile` |
| Root perf foundation run | `bash applykit_pack/.codex/scripts/run_perf_foundation.sh` | `bash ./.codex/scripts/run_perf_foundation.sh` |
| Root perf enforced run | `bash applykit_pack/.codex/scripts/run_perf_enforced.sh` | `bash ./.codex/scripts/run_perf_enforced.sh` |
| Parity script check file | `applykit_pack/.github/workflows/ci.yml` | `.github/workflows/ci.yml` |
| Parity script token | `applykit_pack/.codex/verify.commands` | `.codex/verify.commands` |
| Tests/docs policy regex | `^applykit_pack/...` | flattened root regex (`^crates/`, `^src-tauri/`, `^ui/src/`, `^docs/`) |

## Staged Dual-Path Window
Duration: one migration PR + one post-merge validation window.

### Stage A: Compatibility On
- Keep legacy `applykit_pack` path alias/symlink in migration branch only.
- Keep parity script accepting both old and new tokens.
- Keep workflows emitting existing required check names.

### Stage B: Compatibility Off
- Remove legacy alias and old-token support.
- Enforce root-only flattened paths in parity and policy scripts.
- Confirm required checks still map to branch protection.

## Removal Trigger
All must be true before removing dual-path support:
1. Flattened root verify command passes in CI and local.
2. Parity script passes with only new tokens enabled.
3. Branch protection required checks match active workflow contexts.
4. Rollback branch remains green.

## Exit Gate
- Every affected gate has explicit old/new token mapping and staged removal conditions.
