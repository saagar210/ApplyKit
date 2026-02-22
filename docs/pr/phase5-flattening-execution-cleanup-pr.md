## What
- Executed repo flattening from wrapper+nested structure to single-root product layout.
- Rewrote root control-plane contracts to remove active dependency on nested `applykit_pack` runtime paths.
- Added and updated execution evidence docs for pre-cutover, cutover, and post-cutover stabilization.
- Completed post-cutover cleanup pass and added a historical archive index for phase artifacts.

## Why
- Remove repository boundary confusion and eliminate path-coupled operational drift risk.
- Make verification, CI parity, and release operations run from a single canonical root.
- Preserve auditability while clearly separating runtime contracts from historical evidence.

## How
- Promoted product files to root and retired nested runtime boundary.
- Updated parity/policy/workflow contracts to root-native path assumptions.
- Captured rollback safeguards, governance snapshots, and post-cutover gate evidence.
- Added `docs/history/` manifest to consolidate historical phase records without breaking docs gates.

## Testing
- `pnpm -C ui install --frozen-lockfile` (pass)
- `node scripts/ci/check-ci-parity.mjs` (pass)
- `bash ./.codex/scripts/run_verify_commands.sh` (pass)
- `bash ./.codex/scripts/run_perf_enforced.sh` (pass)
- `cargo audit -D warnings` (pass)
- `./.codex/scripts/docs_check.sh` (pass)

## Risk / Notes
- Historical docs still contain explicit legacy tokens where needed for archival fidelity; active runtime control-plane files are root-native.
- Rollback branch and branch/ruleset snapshots were captured before cutover.
