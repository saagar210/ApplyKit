# Week 3 Checklist (Mar 9 - Mar 15, 2026)

Source alignment: `docs/roadmap-2026q1.md` (Phase 3 Week 3) and `docs/security-advisory-tracking.md`.

## Objectives
1. Audit and reduce security ignore debt.
2. Ensure residual ignores are explicitly tracked with due dates.
3. Capture auditable evidence for release readiness.

## Required Tasks
- [x] Run baseline advisory scan without local ignore config and capture all active warning IDs.
  - Command: `cd /tmp && cargo audit -f <repo>/Cargo.lock -D warnings --json`
- [x] Compare active IDs to `/Users/d/Projects/ApplyKit/.cargo/audit.toml` and remove any obsolete ignore entries.
- [x] For each remaining ignore, confirm owner, mitigation issue, and target removal date in `/Users/d/Projects/ApplyKit/docs/security-advisory-tracking.md`.
- [x] Re-run strict canonical audit path from repo:
  - `cargo audit -D warnings`
- [x] Update release evidence record with who/when/output paths in `/Users/d/Projects/ApplyKit/docs/release-runbook.md`.

## Exit Criteria
- [x] No stale ignore entries remain in `.cargo/audit.toml`.
- [x] Residual ignores are justified and tracked in docs.
- [x] Strict audit command passes in canonical verify flow.
- [x] Week 3 evidence is recorded for RC readiness.

## Week 3 Completion Evidence

- Working report:
  - `/Users/d/Projects/ApplyKit/docs/week3-security-working-report-2026-03-09.md`
- Closeout:
  - `/Users/d/Projects/ApplyKit/docs/week3-closeout-2026-03-13.md`
