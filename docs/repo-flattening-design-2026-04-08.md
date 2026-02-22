# Repo Flattening Design (Prepared Early)

Prepared on **2026-02-22** for Week 7 Day 3 (**2026-04-08**).

## Design Intent
Move from wrapper + nested product layout to single-root product layout while keeping current governance controls.

## Current Layout
- Wrapper root: `/Users/d/Projects/ApplyKit`
- Product root: `/Users/d/Projects/ApplyKit`

## Target Flattened Layout
- New single root: `/Users/d/Projects/ApplyKit` (product content at top level)
- Primary top-level directories after cutover:
  - `.codex/`
  - `.github/`
  - `crates/`
  - `src-tauri/`
  - `ui/`
  - `scripts/`
  - `docs/`
  - `fixtures/`, `config/`, `data/`, `tests/`, `openapi/`

## Preservation Rules
1. Product implementation files become canonical root files.
2. Governance capabilities from current wrapper are retained only where they provide unique control-plane value.
3. Historical evidence docs remain intact and archived under `docs/history/phase1-phase4/` in execution phase (not in prep phase).

## Product-Only History Decision
- Final flattened structure prioritizes product continuity; wrapper history is not preserved as first-class lineage in the target tree.

## Retained Wrapper Assets (By Function)
- CI policy checks and parity guard logic (ported to root `scripts/ci/`).
- Root quality/perf workflows (ported to root `.github/workflows/` with flattened paths).
- Root verify entrypoint semantics (single command from repo root).

## Deprecated/Removed Assets (Post-Cutover)
- Nested `applykit_pack/` directory as runtime implementation boundary.
- Wrapper-only README framing that claims orchestration-only role.
- Path-coupled checks requiring `applykit_pack/...` references.

## Exit Gate
- Target layout and preserve/remove decisions are explicit and implementation-ready.
