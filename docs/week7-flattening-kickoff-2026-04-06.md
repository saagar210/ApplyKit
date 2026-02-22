# Week 7 Flattening Kickoff (Prepared Early)

Prepared on **2026-02-22** for Week 7 kickoff (**2026-04-06**).

## Objective
Lock scope and ownership for Phase 5 flattening readiness so Week 7/8 complete all migration planning and dry-run evidence without performing live cutover.

## Scope Freeze
- In scope:
  - repo flattening design and path map
  - CI/verification parity migration spec
  - branch-protection and status-check transition plan
  - dry-run evidence and rollback planning
- Out of scope:
  - live flattening cutover
  - product feature work
  - architecture redesign

## Owners
- Primary owner: `applykit-platform`
- Verification owner: `applykit-platform`
- Release/rollback governance owner: `applykit-platform`

## Source-of-Truth Artifacts (Phase 5)
- Existing baseline:
  - `/Users/d/Projects/ApplyKit/docs/repo-flattening-prep.md`
  - `/Users/d/Projects/ApplyKit/docs/roadmap-2026q1.md`
- Week 7/8 deliverables:
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

## Locked Decisions
1. History strategy: keep product only in final flattened structure.
2. Cutover strategy: staged dual-path before final removal of wrapper-era path contracts.
3. Blocking-gate policy remains unchanged (`fail` and `not-run` block completion).

## Exit Gate
- Scope, owners, decisions, and artifact map are explicitly locked for Week 7 execution.
