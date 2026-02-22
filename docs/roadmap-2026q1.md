# ApplyKit Production Roadmap Tracker (2026-02-23 to 2026-04-19)

## Phase 2 (Weeks 1-2): Gate Completion + CI Parity
- Week 1 target: enforce coverage + diff coverage and remove waivers.
- Week 2 target: re-enable perf workflows with delegated real commands and artifact capture.

### Phase 2 Completion Snapshot (2026-02-22)
- Week 1 status: complete
  - Evidence: coverage and diff coverage gates are now in `/.codex/verify.commands`.
  - Evidence: required gate waivers file is empty by default.
- Week 2 status: complete
  - Evidence: root and product perf workflows are active and delegated to real scripts.
  - Evidence: perf baselines captured and enforced compare path passes.
  - Evidence: release rehearsal evidence captured in `docs/release-rehearsal-2026-02-22.md`.

## Phase 3 (Weeks 3-4): Security + Release Engineering
- Week 3 target: reduce audit ignore debt and track remaining items with due dates.
- Week 4 target: complete full release-candidate rehearsal with evidence.

### Phase 3 Week 3 Snapshot (Prepared Early on 2026-02-22)
- Week 3 status: complete
  - Evidence: baseline advisory scan captured and compared against `.cargo/audit.toml` with no stale IDs.
  - Evidence: all residual advisories documented with status/blocker/owner/dates in `docs/security-advisory-tracking.md`.
  - Evidence: release-runbook fields populated with Week 3 logs/artifacts.
  - Evidence: delegated root verification and strict canonical audit both passed.

## Phase 4 (Weeks 5-6): Production Launch
- Week 5 target: run final hardening sweep and repeat full gate passes.
- Week 6 target: execute launch and complete post-launch verification.

### Phase 4 Snapshot (Prepared Early on 2026-02-22)
- Week 5 status: complete
  - Evidence: full delegated gate run #1 and #2 passed.
  - Evidence: perf enforced checks passed.
  - Evidence: strict canonical audit passed and baseline no-ignore advisory map remained aligned.
  - Evidence: hardening closeout published in `docs/week5-hardening-closeout-2026-03-29.md`.
- Week 6 status: complete
  - Evidence: launch log published in `docs/week6-launch-log-2026-04-01.md`.
  - Evidence: post-launch verification published in `docs/week6-post-launch-verification-2026-04-02.md`.
  - Evidence: launch report published in `docs/week6-launch-report-2026-04-03.md`.

## Phase 5 Prep (Weeks 7-8): Flattening Readiness
- Week 7 target: finalize flattening migration design and dry-run.
- Week 8 target: finalize migration execution checklist and communication plan.

### Phase 5 Prep Snapshot (Prepared Early on 2026-02-22)
- Week 7 status: complete
  - Evidence: kickoff, path inventory, design, gate parity spec, branch protection mapping, and technical dry-run #1 docs published.
  - Evidence: dry-run #1 captured expected compatibility failure without dual-path shim.
- Week 8 status: complete
  - Evidence: execution checklist, rollback playbook, operational dry-run #2, communications plan, and final decision pack published.
  - Evidence: dry-run #2 validated staged dual-path compatibility model.
  - Evidence: closeout artifact published in `docs/week8-flattening-closeout-2026-04-19.md`.

### Phase 5 Live-Window Revalidation Snapshot (2026-02-22)
- Status: complete
  - Evidence: live path inventory rerun captured in `docs/phase5-live-window-revalidation-2026-02-22.md`.
  - Evidence: both dry-run scenarios rerun with fresh logs (expected fail for no-shim, expected pass for staged dual-path).
  - Evidence: root delegated verify and product canonical verify passed in live rerun.
  - Evidence: GitHub branch protection and rulesets endpoints captured; rulesets returned empty array at validation time.
  - Evidence: decision pack and week8 closeout were refreshed with live-window outcomes.

### Phase 5 Execution Snapshot (2026-02-22)
- Status: complete
  - Evidence: product tree promoted to root and control-plane rewrites landed.
  - Evidence: rollback branch and governance snapshots captured pre-cutover.
  - Evidence: post-cutover stabilization report published in `docs/repo-flattening-post-cutover-stabilization-2026-02-22.md`.

## Status Tracking Fields
For each weekly checkpoint capture:
- `status`: not started | in progress | blocked | complete
- `evidence`: command logs/artifact paths
- `risks`: unresolved blockers
- `owner`: accountable owner

## Historical Index
- Archive index: `docs/history/README.md`
- Detailed manifest: `docs/history/phase-archive-manifest-2026q1.md`
