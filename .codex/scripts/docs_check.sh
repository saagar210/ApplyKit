#!/usr/bin/env bash
set -euo pipefail

required_files=(
  "README.md"
  "docs/spec.md"
  "docs/ui.md"
  "docs/truth-gate.md"
  "docs/llm.md"
  "docs/phases.md"
  "docs/testing.md"
  "docs/release-readiness-phase1.md"
  "docs/release-runbook.md"
  "docs/launch-checklist.md"
  "docs/rollback-checklist.md"
  "docs/post-launch-verification.md"
  "docs/security-advisory-tracking.md"
  "docs/repo-flattening-prep.md"
  "docs/roadmap-2026q1.md"
  "docs/week3-checklist.md"
  "docs/week3-security-working-report-2026-03-09.md"
  "docs/week3-closeout-2026-03-13.md"
  "docs/release-rehearsal-2026-02-22.md"
  "docs/week5-regression-notes-2026-03-24.md"
  "docs/week5-hardening-closeout-2026-03-29.md"
  "docs/week6-launch-log-2026-04-01.md"
  "docs/week6-post-launch-verification-2026-04-02.md"
  "docs/week6-launch-report-2026-04-03.md"
  "docs/week7-flattening-kickoff-2026-04-06.md"
  "docs/repo-flattening-path-inventory-2026-04-07.md"
  "docs/repo-flattening-design-2026-04-08.md"
  "docs/repo-flattening-gate-parity-spec-2026-04-09.md"
  "docs/repo-flattening-branch-protection-plan-2026-04-10.md"
  "docs/repo-flattening-dry-run-1-2026-04-11.md"
  "docs/week7-flattening-closeout-2026-04-12.md"
  "docs/repo-flattening-execution-checklist-2026-04-13.md"
  "docs/repo-flattening-rollback-playbook-2026-04-14.md"
  "docs/repo-flattening-dry-run-2-2026-04-15.md"
  "docs/repo-flattening-communication-plan-2026-04-16.md"
  "docs/repo-flattening-decision-pack-2026-04-17.md"
  "docs/week8-flattening-closeout-2026-04-19.md"
)

missing=0
for file in "${required_files[@]}"; do
  if [[ ! -f "$file" ]]; then
    echo "missing required docs file: $file" >&2
    missing=1
  fi
done

if [[ "$missing" -ne 0 ]]; then
  exit 1
fi

echo "ok: required docs artifacts are present"
