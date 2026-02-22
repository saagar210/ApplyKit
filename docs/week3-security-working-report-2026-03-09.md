# Week 3 Security Working Report (Prepared Early)

## Context

This report captures the Week 3 security-debt baseline and triage work executed early on **2026-02-22** to de-risk the Week 3 window.

## Baseline Commands and Outputs

1. Baseline advisory scan (no local ignore config):
   - Command:
     - `cd /tmp && cargo audit -f /Users/d/Projects/ApplyKit/Cargo.lock -D warnings --json > /tmp/applykit_week3_baseline_audit.json`
   - Exit: `1` (expected due informational advisories with no local ignore config)
   - Output artifact: `/tmp/applykit_week3_baseline_audit.json`

2. Canonical strict audit (with repo config):
   - Command:
     - `cargo audit -D warnings`
   - Exit: `0`
   - Output artifact: `/tmp/applykit_week3_cargo_audit.log`

3. CI parity guard:
   - Command:
     - `node /Users/d/Projects/ApplyKit/scripts/ci/check-ci-parity.mjs`
   - Exit: `0`
   - Output artifact: `/tmp/applykit_week3_ci_parity.log`

## Advisory Set Comparison

- Active advisory IDs from baseline scan: 18
- Ignore IDs in `/Users/d/Projects/ApplyKit/.cargo/audit.toml`: 18
- Stale ignore IDs: none
- Missing ignore IDs for active advisories: none

## Removability / Upgrade Triage

- Commands:
  - `cargo update --workspace --dry-run`
  - `cargo update -p tauri-utils --dry-run`
  - `cargo update -p glib --dry-run`
  - `cargo update -p proc-macro-error --dry-run`
- Result:
  - No compatible dependency upgrades available in the current version constraints that reduce the active advisory set.

## Working Disposition

- `remove now`: none
- `blocked`: all 18 advisory IDs (upstream/transitive chain constraints)
- Tracking location:
  - `/Users/d/Projects/ApplyKit/docs/security-advisory-tracking.md`
