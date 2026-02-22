# Repo Flattening Dry-Run #1 (Technical, Prepared Early)

Prepared on **2026-02-22** for Week 7 Day 6 (**2026-04-11**).

## Purpose
Prove baseline failure modes when flattening is attempted without staged dual-path compatibility.

## Environment
- Simulation root: `/tmp/applykit_phase5_dryrun1`
- Input: product files copied as flattened candidate (no wrapper workflows present)

## Commands Executed
1. `node scripts/ci/check-ci-parity.mjs`
2. `bash ./.codex-wrapper/scripts/run_verify_commands.sh ./.codex-wrapper/verify.commands`

Evidence:
- `/tmp/applykit_phase5_dryrun1_parity.log`
- `/tmp/applykit_phase5_dryrun1_wrapper_delegation.log`
- `/tmp/applykit_phase5_dryrun_summary.log`

## Result
- Dry-run result: **fail (expected)**
- Failure mode: missing `.github/workflows/quality-gates.yml` in flattened candidate caused parity script to fail immediately.

## Findings
1. Wrapper-era parity assumptions are hard-coded and will break without compatibility shims.
2. Delegated wrapper verify contract cannot be reused unchanged in flattened topology.
3. Path-coupled control-plane files are true blockers and must be migrated as a set.

## Remediation Inputs for Dry-Run #2
1. Introduce temporary `applykit_pack` compatibility alias.
2. Stage wrapper workflow tokens in simulated flattened root.
3. Validate parity passes before removing legacy contracts.

## Exit Gate
- Blocking issues captured with explicit remediation path for Dry-Run #2.

## Live Rerun Addendum (2026-02-22)
Environment:
- Simulation root: `/tmp/applykit_phase5_live_dryrun1`

Evidence:
- `/tmp/applykit_phase5_live_dryrun1_parity.log`
- `/tmp/applykit_phase5_live_dryrun1_wrapper.log`

Live rerun result:
- **fail (expected)** with the same non-shim failure mode:
  - parity script fails because `.github/workflows/quality-gates.yml` is not present in the flattened candidate.

Conclusion:
- Dry-run #1 blocker remains reproducible and unchanged.
