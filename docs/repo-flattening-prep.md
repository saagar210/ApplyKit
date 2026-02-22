# Repo Flattening Prep (Post-Launch)

## Goal
Prepare a safe migration from wrapper + nested product layout to a single flattened repository after launch stability is confirmed.

## Required Design Inputs
- Current canonical path at prep time: `applykit_pack/`
- Current wrapper responsibilities: root CI + policy orchestration
- Required preserved contracts:
  - root verification entrypoint behavior
  - required gate matrix
  - release and rollback runbooks

## Migration Checklist
1. Map path rewrites for CI workflows, scripts, and docs links.
2. Define branch protection/status check migration plan.
3. Validate local command parity before/after path rewrite.
4. Dry-run migration in a temporary worktree/branch.
5. Capture rollback strategy for migration itself.

## Risk Controls
- Freeze feature work during migration window.
- Keep one-command verify path available at all times.
- Require green gates before and after cutover.
- Prepare fallback branch with pre-flattening state.

## Exit Criteria
- Flattening checklist is implementation-ready.
- Risks have owners and rollback paths.
- Team can execute migration without further discovery.

## Phase 5 Packet (Prepared Early)
- `docs/week7-flattening-kickoff-2026-04-06.md`
- `docs/repo-flattening-path-inventory-2026-04-07.md`
- `docs/repo-flattening-design-2026-04-08.md`
- `docs/repo-flattening-gate-parity-spec-2026-04-09.md`
- `docs/repo-flattening-branch-protection-plan-2026-04-10.md`
- `docs/repo-flattening-dry-run-1-2026-04-11.md`
- `docs/week7-flattening-closeout-2026-04-12.md`
- `docs/repo-flattening-execution-checklist-2026-04-13.md`
- `docs/repo-flattening-rollback-playbook-2026-04-14.md`
- `docs/repo-flattening-dry-run-2-2026-04-15.md`
- `docs/repo-flattening-communication-plan-2026-04-16.md`
- `docs/repo-flattening-decision-pack-2026-04-17.md`
- `docs/week8-flattening-closeout-2026-04-19.md`

## Live-Window Revalidation Packet
- `docs/phase5-live-window-revalidation-2026-02-22.md`

## Execution Status Update (2026-02-22)
- Flattening cutover was executed; canonical runtime path is now repo root.
- Post-cutover evidence:
  - `docs/repo-flattening-post-cutover-stabilization-2026-02-22.md`
