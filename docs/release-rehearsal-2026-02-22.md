# Release Rehearsal Evidence (2026-02-22)

## Operator
- operator: codex session
- date_utc: 2026-02-22

## Commands Executed
1. `bash /Users/d/Projects/ApplyKit/.codex/scripts/run_verify_commands.sh`
2. `bash /Users/d/Projects/ApplyKit/.codex/scripts/run_perf_foundation.sh`
3. `bash /Users/d/Projects/ApplyKit/.codex/scripts/run_perf_enforced.sh`
4. `node /Users/d/Projects/ApplyKit/scripts/ci/require-tests-and-docs.mjs`

## Outcome Summary
- Root delegated verification: pass
- Product canonical verify suite: pass
- Coverage thresholds: pass (Rust >= 70, UI >= 30)
- Diff coverage: pass (>= 80)
- Perf foundation: pass
- Perf enforced compare: pass
- Policy checks: pass

## Packaging Artifacts
From successful verify run:
- `/Users/d/Library/Caches/Codex/build/rust/applykit/debug/bundle/macos/ApplyKit.app`
- `/Users/d/Library/Caches/Codex/build/rust/applykit/debug/bundle/dmg/ApplyKit_0.1.0_aarch64.dmg`

## Perf Baselines Captured
- `/Users/d/Projects/ApplyKit/.perf-baselines/bundle.json`
- `/Users/d/Projects/ApplyKit/.perf-baselines/build-time.json`

## Notes
- Build time baseline is sampled and stored via `scripts/perf/measure-build-time.mjs`.
- Week 3 focus remains advisory debt burn-down and RC ceremony hardening.

## Week 3 Delta (Prepared Early)

- Added strict advisory baseline artifact:
  - `/tmp/applykit_week3_baseline_audit.json`
- Added strict canonical audit evidence:
  - `/tmp/applykit_week3_cargo_audit.log`
- Added CI parity evidence:
  - `/tmp/applykit_week3_ci_parity.log`
- Added full delegated verify evidence:
  - `/tmp/applykit_week3_root_verify.log`
- Outcome:
  - no stale ignore IDs found
  - all remaining ignored advisories documented as blocked/transitive with owner and due date

## Phase 4 Delta (Prepared Early)

- Week 5 regression notes:
  - `/Users/d/Projects/ApplyKit/docs/week5-regression-notes-2026-03-24.md`
- Week 5 hardening closeout:
  - `/Users/d/Projects/ApplyKit/docs/week5-hardening-closeout-2026-03-29.md`
- Week 6 launch log:
  - `/Users/d/Projects/ApplyKit/docs/week6-launch-log-2026-04-01.md`
- Week 6 post-launch verification:
  - `/Users/d/Projects/ApplyKit/docs/week6-post-launch-verification-2026-04-02.md`
- Week 6 launch report:
  - `/Users/d/Projects/ApplyKit/docs/week6-launch-report-2026-04-03.md`
