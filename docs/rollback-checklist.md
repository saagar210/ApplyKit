# Rollback Checklist

## Rollback Triggers
- Sev-1 incident after launch.
- Reproducible data integrity or export corruption issue.
- Core workflow unavailable (packet generate/export failure).

## Preconditions
- [ ] Previous known-good release commit/tag identified.
- [ ] Rollback owner assigned.
- [ ] Communication channel prepared.

## Rollback Steps
1. Halt new rollout actions and freeze release branch.
2. Rebuild or re-publish previous known-good release artifacts.
3. Confirm application starts and critical flows pass:
   - packet generation
   - tracker update
   - Markdown/DOCX/PDF exports
4. Announce rollback completion and known residual issues.
5. Open incident follow-up with root cause and remediation plan.

## Evidence Required
- Trigger reason and severity
- Rolled back to commit/tag
- Validation command logs
- Time to recover (start/end timestamps)

## Phase 4 Readiness Snapshot (Prepared Early)
- [x] Previous known-good release ref identified: `98b67c0c3cca4efcd3d77e92b6196d9b6ae1cc3a` (current green baseline).
- [x] Rollback owner assigned: `applykit-platform`.
- [x] Communication channel prepared: release runbook + launch report artifacts in `docs/`.
- [x] Validation command logs captured:
  - `/tmp/applykit_phase4_week6_day1_root_verify.log`
  - `/tmp/applykit_phase4_week6_day2_tauri_build.log`

## Notes
- No rollback was triggered during Phase 4 preparation.
- This checklist remains the authoritative execution path if Sev-1/Sev-2 conditions appear during live launch window.
