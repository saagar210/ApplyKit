# Release Runbook

## Purpose
Authoritative release ceremony for ApplyKit release candidates and production launches.

## Preconditions
- Canonical repo: `/Users/d/Projects/ApplyKit`
- Required gates pass from repo root:
  - `bash ./.codex/scripts/run_verify_commands.sh`
- Required-gate waivers are empty by default:
  - `node ./.codex/scripts/check_gate_waivers.mjs`

## Evidence Capture (Required)
Record for each run:
- `operator`: name/initials
- `date_utc`: ISO timestamp
- `git_ref`: commit SHA
- `verify_log_path`: local log path
- `perf_artifact_path`: `.perf-results/summary.json`
- `package_artifacts`: paths for `.app` and `.dmg`
- `security_audit_result`: pass/fail + notes
- `decision`: go/no-go

## Week 3 Evidence Snapshot (Executed Early)

- `operator`: codex-session
- `date_utc`: 2026-02-22T10:45:27Z
- `git_ref`: 98b67c0c3cca4efcd3d77e92b6196d9b6ae1cc3a
- `verify_log_path`: /tmp/applykit_week3_root_verify.log
- `perf_artifact_path`:
  - /Users/d/Projects/ApplyKit/.perf-baselines/bundle.json
  - /Users/d/Projects/ApplyKit/.perf-baselines/build-time.json
- `package_artifacts`:
  - /Users/d/Library/Caches/Codex/build/rust/applykit/debug/bundle/macos/ApplyKit.app
  - /Users/d/Library/Caches/Codex/build/rust/applykit/debug/bundle/dmg/ApplyKit_0.1.0_aarch64.dmg
- `security_audit_result`:
  - canonical strict audit: pass (`/tmp/applykit_week3_cargo_audit.log`)
  - baseline no-ignore scan: expected fail with informational advisories (`/tmp/applykit_week3_baseline_audit.json`)
  - stale ignore IDs: none
- `parity_check_result`:
  - pass (`/tmp/applykit_week3_ci_parity.log`)
- `decision`:
  - go (Week 4 RC rehearsal unblocked)

## Week 4 RC Rehearsal Status

- Evidence source: `/Users/d/Projects/ApplyKit/docs/release-rehearsal-2026-02-22.md`
- Status: complete (prepared early)
- Outcome:
  - root delegated verification: pass
  - perf foundation and enforced checks: pass
  - packaging artifacts: present

## Phase 4 Week 5 Evidence Snapshot (Prepared Early)

- `operator`: codex-session
- `date_utc`: 2026-02-22T11:12:32Z
- `git_ref`: 98b67c0c3cca4efcd3d77e92b6196d9b6ae1cc3a
- `verify_log_path`:
  - /tmp/applykit_phase4_week5_gate1_root_verify.log
  - /tmp/applykit_phase4_week5_gate2_root_verify.log
- `reliability_regression_log`:
  - /tmp/applykit_phase4_week5_day2_regression.log
- `parity_check_result`:
  - pass (/tmp/applykit_phase4_week5_day3_ci_parity.log)
- `perf_artifact_path`:
  - /tmp/applykit_phase4_week5_day4_perf_enforced.log
  - /Users/d/Projects/ApplyKit/.perf-results/summary.json
- `package_artifacts`:
  - /Users/d/Library/Caches/Codex/build/rust/applykit/debug/bundle/macos/ApplyKit.app
  - /Users/d/Library/Caches/Codex/build/rust/applykit/debug/bundle/dmg/ApplyKit_0.1.0_aarch64.dmg
- `security_audit_result`:
  - canonical strict audit: pass (/tmp/applykit_phase4_week5_day3_cargo_audit.log, /tmp/applykit_phase4_week5_day5_cargo_audit.log)
  - baseline no-ignore scan: expected fail (/tmp/applykit_phase4_week5_day5_baseline_audit.log)
  - baseline advisory ID summary: 18 unique IDs, no stale drift (/tmp/applykit_phase4_week5_day5_baseline_audit_summary.log)
- `decision`:
  - go (Week 6 launch window ready)

## Phase 4 Week 6 Launch Snapshot (Prepared Early)

- `operator`: codex-session
- `date_utc`: 2026-02-22T11:12:32Z
- `git_ref`: 98b67c0c3cca4efcd3d77e92b6196d9b6ae1cc3a
- `verify_log_path`: /tmp/applykit_phase4_week6_day1_root_verify.log
- `post-docs-final-verify_log_path`: /tmp/applykit_phase4_final_root_verify.log
- `parity_check_result`: pass (/tmp/applykit_phase4_week6_day1_ci_parity.log)
- `waiver_policy_result`: pass (/tmp/applykit_phase4_week6_day1_gate_waivers.log)
- `package_artifacts`:
  - /Users/d/Library/Caches/Codex/build/rust/applykit/debug/bundle/macos/ApplyKit.app
  - /Users/d/Library/Caches/Codex/build/rust/applykit/debug/bundle/dmg/ApplyKit_0.1.0_aarch64.dmg
- `artifact_checksums`:
  - app binary (`Contents/MacOS/applykit_tauri`): `0c208a49edff7ff05130f8e8cc3e75eacd7be55615b41cdb847377d054749d76`
  - dmg: `7809747cdf81ec9169f013f8d90d9d0824e53dd92766c51a90fc3381063c3d36`
- `post_launch_verification_artifact`:
  - /Users/d/Projects/ApplyKit/docs/week6-post-launch-verification-2026-04-02.md
- `decision`:
  - go (Phase 4 complete; proceed to Phase 5 prep)

## Operational Follow-up Snapshot (2026-02-28)

- `operator`: codex-session
- `date_utc`: 2026-02-28T22:20:00Z
- `git_ref`: 4b7e653567ab9a64b845dfa024f0a8da4948785a
- `evidence_artifact`:
  - `docs/evidence/operational-revalidation-2026-02-28.md`
- `verify_result`:
  - pass
- `perf_result`:
  - foundation: pass
  - enforced: pass
- `security_audit_result`:
  - canonical strict audit: pass
  - baseline no-ignore scan: expected fail (informational advisories)
  - advisory drift check: pass (0 stale, 0 missing IDs)
- `parity_check_result`:
  - pass
- `governance_snapshot`:
  - branch protection: approvals=1, conversation resolution required=true
  - required checks: quality, verify, perf-enforced, perf-foundation, dependency_and_misconfig, sast, secrets, enforce
  - active ruleset: `main-operational-readiness` (`id=13367455`)
  - advisory disposition: AK-301 (`#7`), AK-302 (`#8`), AK-303 (`#9`) closed on 2026-03-01 via AK-304 (`#12`) residual-risk decision
- `decision`:
  - go (operational readiness posture unchanged; residual advisory risk accepted and tracked via closed AK-301/302/303 with reopen triggers)

## Security Hardening Snapshot (2026-03-01)

- `operator`: codex-session
- `date_utc`: 2026-03-01T13:24:00Z
- `git_ref`: working tree hardening cycle (pre-commit)
- `verify_log_path`:
  - `/tmp/applykit_security_hardening_verify_2026-03-01.log`
- `security_audit_result`:
  - canonical strict audit: pass
  - baseline no-ignore scan: expected non-zero (`/tmp/applykit_security_baseline_2026-03-01.json`)
  - UI dependency audit high: pass (`pnpm -C ui audit --audit-level high`)
  - UI dependency audit moderate: pass (`pnpm -C ui audit --audit-level moderate`)
- `decision`:
  - go (hardening cycle complete; residual transitive Rust advisories remain tracked and time-bound)

## Final Closure Snapshot (2026-03-01)

- `operator`: codex-session
- `date_utc`: 2026-03-01T14:48:00Z
- `git_ref`: 523b0706cbdb141110feed42f7680a143233b195
- `verify_log_path`:
  - `docs/evidence/security-revalidation-2026-03-01-verify.log`
- `security_audit_result`:
  - canonical strict audit: pass (`docs/evidence/security-revalidation-2026-03-01-canonical-audit.log`)
  - baseline no-ignore scan: expected non-zero with informational advisories (`docs/evidence/security-revalidation-2026-03-01-baseline.json`)
  - advisory mapping drift: none (`docs/evidence/security-revalidation-2026-03-01.md`)
- `ui_accessibility_result`:
  - unit a11y + responsive suites: pass
  - Playwright a11y smoke: pass
  - manual contrast audit: pass (`docs/ui-contrast-audit-2026-03-01.md`)
- `decision`:
  - go (final closure gates green; no release-blocking follow-ups remain)

## Golden Path
1. `pnpm -C ui install --frozen-lockfile`
2. `bash ./.codex/scripts/run_verify_commands.sh`
3. `bash ./.codex/scripts/run_perf_enforced.sh`
4. `cargo tauri build --debug`
5. Validate export outputs for Markdown, DOCX, and PDF from a generated packet.
6. Complete launch and rollback checklists before any production cutover.

## Blocking Failure Rules
- Any required gate in `fail` or `not-run` state blocks release.
- Any missing artifact evidence blocks release.
- Any Sev-1/Sev-2 unresolved issue blocks release.

## Flattening Note (2026-02-22)
- Legacy nested-path tokens in historical evidence sections above are archival records from pre-cutover runs.
- Current operational commands are root-scoped and match the Golden Path section.
