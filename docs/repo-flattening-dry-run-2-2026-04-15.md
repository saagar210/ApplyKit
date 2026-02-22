# Repo Flattening Dry-Run #2 (Operational, Prepared Early)

Prepared on **2026-02-22** for Week 8 Day 3 (**2026-04-15**).

## Purpose
Validate staged dual-path compatibility behavior in a simulated flattened environment.

## Environment
- Simulation root: `/tmp/applykit_phase5_dryrun2`
- Compatibility model:
  - temporary `applykit_pack -> .` symlink
  - wrapper quality/perf workflows staged in simulated root

## Commands Executed
1. `node scripts/ci/check-ci-parity.mjs`
2. contract smoke: `test -d applykit_pack && test -f applykit_pack/.codex/verify.commands`

Evidence:
- `/tmp/applykit_phase5_dryrun2_parity.log`
- `/tmp/applykit_phase5_dryrun2_delegation_smoke.log`
- `/tmp/applykit_phase5_dryrun_summary.log`

## Result
- Dry-run result: **pass**
- Parity check passed with staged compatibility layer.
- Delegated path contract smoke passed with symlink-based shim.

## Findings
1. Dual-path strategy is feasible and resolves immediate parity blocker from Dry-Run #1.
2. Compatibility window can be short-lived if parity and verify remain green.
3. Migration should preserve workflow context names during first cutover pass.

## Residual Risks
- Full branch ruleset posture still needs explicit capture during live execution.
- Compatibility alias must be removed only after trigger criteria are met.

## Exit Gate
- No unresolved Sev-1/Sev-2 risks in the simulated compatibility path.

## Live Rerun Addendum (2026-02-22)
Environment:
- Simulation root: `/tmp/applykit_phase5_live_dryrun2`

Evidence:
- `/tmp/applykit_phase5_live_dryrun2_parity.log`
- `/tmp/applykit_phase5_live_dryrun2_dual_path.log`

Live rerun result:
- **pass** with staged dual-path compatibility:
  - parity script passed.
  - dual-path smoke checks passed.

Conclusion:
- Dry-run #2 assumptions remain valid for execution handoff.
