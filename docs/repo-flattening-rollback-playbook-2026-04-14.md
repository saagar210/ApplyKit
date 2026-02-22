# Repo Flattening Rollback Playbook (Prepared Early)

Prepared on **2026-02-22** for Week 8 Day 2 (**2026-04-14**).

## Rollback Triggers
1. Required gate failure after cutover with no same-window fix.
2. CI status-check context mismatch blocking merges.
3. Parity script cannot reconcile workflow and local commands.
4. Critical automation/script path breakage in default branch.

## Rollback Preconditions
- Pre-cutover rollback branch exists and is green.
- Branch protection snapshot is available.
- Rollback owner and communication channel are active.

## Rollback Steps
1. Freeze further migration commits.
2. Reset default branch to rollback branch (or merge rollback commit set).
3. Restore branch protection and required status-check contexts from snapshot.
4. Re-run canonical gates:
   - root delegated verify
   - parity check
   - perf enforced
   - security audit
5. Publish rollback incident summary and follow-up actions.

## Rollback Evidence Template
- trigger: 
- start_utc:
- end_utc:
- rolled_back_to_sha:
- restored_required_checks:
- verify_log_paths:
- parity_log_path:
- perf_log_path:
- audit_log_path:
- residual_risks:
- owner:

## Exit Gate
- Rollback path is deterministic, evidence-backed, and tied to gate outcomes.

## Execution Addendum (2026-02-22)
- Pre-cutover rollback branch created:
  - `codex/chore/flattening-rollback-precutover-20260222`
- Pre-cutover branch/ruleset snapshots captured:
  - `/tmp/applykit_phase5_cutover/precutover_branch_protection.json`
  - `/tmp/applykit_phase5_cutover/precutover_rulesets.json`
- Nested pre-cutover repo backup path:
  - `/tmp/applykit_pack_legacy_20260222_1771760910`
