# Branch Protection and Status Check Plan (Prepared Early)

Prepared on **2026-02-22** for Week 7 Day 5 (**2026-04-10**).

## Current Branch Protection State (Evidence)
Command:
- `gh api repos/saagar210/ApplyKit/branches/main/protection`

Evidence:
- `/tmp/applykit_phase5_branch_protection_main.json`
- `/tmp/applykit_phase5_branch_protection_master.json`

## Observed Settings (main)
- Required status checks (strict):
  - `quality`
  - `quality-gates`
- Required review count: 1
- Dismiss stale reviews: true
- Enforce admins: true
- Required linear history: true
- Required conversation resolution: true

## Migration Mapping Plan
| Existing Required Check | Migration Handling | Post-Flattening Target |
| --- | --- | --- |
| `quality` | keep context name unchanged | `quality` |
| `quality-gates` | keep during dual-path stage | `quality-gates` then retire after stability window |

## Update Order
1. Land flattened workflows while preserving `quality` and `quality-gates` context names.
2. Confirm contexts appear and pass on migration PR.
3. Update branch protection only if context names must change (default: avoid rename).
4. After stability window, optionally remove `quality-gates` requirement if redundant.

## Rollback for Branch Protection
1. Restore previous required contexts from saved JSON snapshot.
2. Re-enable strict checks exactly as before migration.
3. Re-run verify on rollback branch.

## Unknowns (Original Week 7 Capture)
- Unknown: whether GitHub rulesets (outside classic protection API) apply additional hidden constraints.
- Mitigation: capture rulesets via `gh api repos/saagar210/ApplyKit/rulesets` during execution phase.
- Live status: resolved in the 2026-02-22 live revalidation addendum.

## Exit Gate
- Status-check migration order is deterministic with fallback and API evidence.

## Live Revalidation Addendum (2026-02-22)
Commands:
- `gh api repos/saagar210/ApplyKit/branches/main/protection`
- `gh api repos/saagar210/ApplyKit/rulesets`

Evidence:
- `/tmp/applykit_phase5_live_branch_protection_main.json`
- `/tmp/applykit_phase5_live_rulesets.json`

Live results:
1. Required status checks remain `quality` and `quality-gates` in strict mode.
2. Rulesets endpoint returned `[]` (no additional rulesets detected at validation time).
3. Prior unknown on hidden rulesets is closed for the current live window.

Execution guidance update:
- Keep branch protection context names stable through first cutover pass.
- Re-check both endpoints immediately before live flattening cutover.
