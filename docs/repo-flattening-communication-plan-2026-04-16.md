# Repo Flattening Communication Plan (Prepared Early)

Prepared on **2026-02-22** for Week 8 Day 4 (**2026-04-16**).

## Audiences
1. Engineering contributors
2. Release/ops stakeholders
3. PM/leadership stakeholders

## Messages
### Contributor Notice
- What changes: repository paths flatten to single root.
- What to update: local commands, docs links, script paths.
- What remains: quality/security/perf gate standards and blocking behavior.

### Operations Notice
- Migration window start/end UTC.
- Rollback owner and escalation path.
- Required status-check continuity expectations.

### PM/Leadership Notice
- Cutover scope and risk posture.
- Go/no-go criteria.
- Expected impact window and fallback plan.

## Communication Timeline
1. T-3 days: announce migration window and freeze scope.
2. T-1 day: confirm owners, rollback readiness, and final dry-run status.
3. T-0: announce start, checkpoints, and completion.
4. T+1 day: publish stability report and remaining follow-ups.

## Required Artifacts to Link in Messages
- `docs/repo-flattening-execution-checklist-2026-04-13.md`
- `docs/repo-flattening-rollback-playbook-2026-04-14.md`
- `docs/repo-flattening-decision-pack-2026-04-17.md`

## Exit Gate
- Communication packet is ready to send with dates, owners, and escalation paths.
