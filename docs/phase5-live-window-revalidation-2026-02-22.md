# Phase 5 Live-Window Revalidation (2026-02-22)

## Archival Note
This document captures **pre-cutover** evidence for the wrapper+nested topology.  
Use `/Users/d/Projects/ApplyKit/docs/repo-flattening-post-cutover-stabilization-2026-02-22.md` for live runtime contracts after flattening.

## Purpose
Revalidate Phase 5 readiness artifacts against current repository state before flattening execution handoff.

## Scope
1. Refresh path dependency evidence.
2. Re-run both dry-run scenarios.
3. Re-run canonical verification commands.
4. Close governance visibility gap for branch protection and rulesets.

## Command Evidence (Live Window)
| Command | Source of Truth | Result | Evidence |
| --- | --- | --- | --- |
| `bash /Users/d/Projects/ApplyKit/.codex/scripts/run_verify_commands.sh` | `/Users/d/Projects/ApplyKit/.codex/verify.commands` | pass | `/tmp/applykit_phase5_live_root_verify.log` |
| `node /Users/d/Projects/ApplyKit/scripts/ci/check-ci-parity.mjs` | `/Users/d/Projects/ApplyKit/.codex/verify.commands`, `/Users/d/Projects/ApplyKit/.github/workflows/quality-gates.yml` | pass | `/tmp/applykit_phase5_live_ci_parity.log` |
| `bash /Users/d/Projects/ApplyKit/applykit_pack/.codex/scripts/run_verify_commands.sh` (legacy pre-cutover path) | `/Users/d/Projects/ApplyKit/applykit_pack/.codex/verify.commands` (legacy pre-cutover contract) | pass | `/tmp/applykit_phase5_live_product_verify.log` |
| `bash /Users/d/Projects/ApplyKit/applykit_pack/.codex/scripts/run_perf_enforced.sh` (legacy pre-cutover path) | `/Users/d/Projects/ApplyKit/applykit_pack/.codex/commands.md` (legacy pre-cutover contract) | pass | `/tmp/applykit_phase5_live_perf_enforced.log` |
| `cargo audit -D warnings` (from legacy pre-cutover `applykit_pack` root) | `/Users/d/Projects/ApplyKit/applykit_pack/.codex/verify.commands` (legacy pre-cutover contract) | pass | `/tmp/applykit_phase5_live_cargo_audit.log` |

## Path Inventory Revalidation
Command:
- `rg -n --hidden "applykit_pack/|applykit_pack\\b" /Users/d/Projects/ApplyKit -g '!**/target/**' -g '!**/node_modules/**' -g '!**/.git/**' -g '!**/coverage/**' -g '!**/.perf-results/**' -g '!**/dist/**'`

Evidence:
- `/tmp/applykit_phase5_live_path_inventory_control.log`

Result:
1. 118 references across 29 files were identified.
2. High-risk control-plane references remain concentrated in root verify/workflow/parity/policy surfaces.
3. Existing migration rewrite/shim/retire model remains valid.

## Dry-Run Revalidation
Dry-run #1 (no compatibility shim):
- Result: fail (expected)
- Evidence:
  - `/tmp/applykit_phase5_live_dryrun1_parity.log`
  - `/tmp/applykit_phase5_live_dryrun1_wrapper.log`

Dry-run #2 (staged dual-path):
- Result: pass (expected)
- Evidence:
  - `/tmp/applykit_phase5_live_dryrun2_parity.log`
  - `/tmp/applykit_phase5_live_dryrun2_dual_path.log`

Summary:
- `/tmp/applykit_phase5_live_dryrun_summary.log`

## Governance Revalidation
Commands:
- `gh api repos/saagar210/ApplyKit/branches/main/protection`
- `gh api repos/saagar210/ApplyKit/rulesets`

Evidence:
- `/tmp/applykit_phase5_live_branch_protection_main.json`
- `/tmp/applykit_phase5_live_rulesets.json`

Result:
1. Required check contexts remain `quality` and `quality-gates`.
2. Rulesets endpoint returned `[]` during this validation window.
3. Hidden ruleset visibility risk is closed for the current decision packet.

## Risk Status (End of Live Revalidation)
| Risk | Status | Notes |
| --- | --- | --- |
| CI path drift during rewrite | controlled | Token-level parity spec and dry-run evidence remain valid. |
| Rollback ambiguity | controlled | Rollback playbook remains explicit and aligned with gate outcomes. |
| Branch-protection context mismatch | controlled | Required context names confirmed and mapped. |
| Hidden rulesets not mapped | closed (current window) | Rulesets endpoint captured and empty. |
| Dual-path complexity creep | open, time-boxed | Must enforce explicit removal trigger in execution phase. |

## Decision
Phase 5 prep is execution-ready. Flattening execution can proceed with the existing checklist, rollback playbook, and a final pre-cutover gate rerun.
