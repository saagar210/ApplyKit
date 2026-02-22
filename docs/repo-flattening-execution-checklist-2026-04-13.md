# Repo Flattening Execution Checklist (Prepared Early)

Prepared on **2026-02-22** for Week 8 Day 1 (**2026-04-13**).

## Preflight (Stop if Any Fail)
1. Confirm migration freeze window is active.
2. Confirm rollback owner and communication channel assigned.
3. Confirm latest green baseline logs exist for:
   - root delegated verify
   - parity check
   - perf enforced
4. Snapshot branch protection and ruleset state.
5. Create rollback branch from pre-cutover commit.

## Cutover Steps (Execution Phase)
1. Promote product tree (`applykit_pack`) to repo root.
2. Port required wrapper governance scripts/workflows to root equivalents.
3. Rewrite path-coupled tokens from `applykit_pack/...` to root paths.
4. Enable dual-path compatibility alias for one stabilization window.
5. Run full gate matrix (local + CI).

## Verification Gates (Blocking)
1. `bash ./.codex/scripts/run_verify_commands.sh`
2. `node scripts/ci/check-ci-parity.mjs`
3. `bash ./.codex/scripts/run_perf_enforced.sh`
4. `cargo audit -D warnings`
5. Branch protection required contexts appear and pass.

## Stop/Go Gates
- Stop conditions:
  - Any required gate `fail` or `not-run`
  - Parity drift between workflow and local commands
  - Missing rollback branch or rollback validation evidence
- Go conditions:
  - All required gates pass
  - Dual-path compatibility works
  - Rollback rehearsal pass confirmed

## Post-Cutover Stabilization
1. Monitor first CI runs on default branch.
2. Confirm no hidden path references remain.
3. Remove dual-path compatibility artifacts only after defined trigger criteria are met.

## Exit Gate
- Checklist is stepwise executable without interpretation.

## Execution Addendum (Completed on 2026-02-22)
Status: complete.

Evidence:
- `/Users/d/Projects/ApplyKit/docs/repo-flattening-post-cutover-stabilization-2026-02-22.md`
- `/tmp/applykit_phase5_cutover/precutover_head_sha.txt`
- `/tmp/applykit_phase5_cutover/rollback_branch_status.log`
- `/tmp/applykit_phase5_cutover/precutover_branch_protection.json`
- `/tmp/applykit_phase5_cutover/precutover_rulesets.json`

Completion notes:
1. Product tree was promoted to root and root control-plane files were rewritten to root paths.
2. Full post-cutover gate set was rerun successfully.
3. Dual-path compatibility alias was not retained after stabilization verification.
