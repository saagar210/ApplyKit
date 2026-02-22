# Repo Flattening Post-Cutover Stabilization (2026-02-22)

## Summary
Flattening cutover was executed in `/Users/d/Projects/ApplyKit` and the repo now operates as a single-root product repository.

## Cutover Actions Executed
1. Created rollback branch at pre-cutover state:
   - branch: `codex/chore/flattening-rollback-precutover-20260222`
   - source SHA: `03b03017ce9d7966ee628d22570f60cab9a12c59`
   - evidence: `/tmp/applykit_phase5_cutover/precutover_head_sha.txt`, `/tmp/applykit_phase5_cutover/rollback_branch_status.log`
2. Captured governance snapshots pre-cutover:
   - `/tmp/applykit_phase5_cutover/precutover_branch_protection.json`
   - `/tmp/applykit_phase5_cutover/precutover_rulesets.json`
3. Promoted product tree to repo root via `rsync` from `applykit_pack/`.
4. Rewrote control-plane contracts to root paths:
   - `.github/workflows/quality-gates.yml`
   - `scripts/ci/check-ci-parity.mjs`
   - `scripts/ci/require-tests-and-docs.mjs`
   - `.codex/verify.commands`
   - `.cargo/config.toml`
5. Created nested pre-cutover repo backup, then removed it after stabilization:
   - removed path: `/tmp/applykit_pack_legacy_20260222_1771760910`

## Post-Cutover Verification Evidence
| Command | Source | Result | Evidence |
| --- | --- | --- | --- |
| `pnpm -C ui install --frozen-lockfile` | `.github/workflows/ci.yml` | pass | `/tmp/applykit_phase5_cutover/post_cutover_ui_install.log` |
| `node scripts/ci/check-ci-parity.mjs` | `.codex/verify.commands`, `.github/workflows/quality-gates.yml` | pass | `/tmp/applykit_phase5_cutover/post_cutover_ci_parity.log` |
| `bash ./.codex/scripts/run_verify_commands.sh` | `.codex/verify.commands` | pass | `/tmp/applykit_phase5_cutover/post_cutover_root_verify.log` |
| `bash ./.codex/scripts/run_perf_enforced.sh` | `.codex/commands.md` | pass | `/tmp/applykit_phase5_cutover/post_cutover_perf_enforced.log` |
| `cargo audit -D warnings` | `.codex/verify.commands` | pass | `/tmp/applykit_phase5_cutover/post_cutover_cargo_audit.log` |

## Governance and Contract Drift Checks
1. Post-cutover branch protection snapshot:
   - `/tmp/applykit_phase5_cutover/post_cutover_branch_protection.json`
2. Post-cutover rulesets snapshot:
   - `/tmp/applykit_phase5_cutover/post_cutover_rulesets.json`
3. Active control-plane token scan:
   - `/tmp/applykit_phase5_cutover/post_cutover_active_contract_scan.log`
   - result: no active control-plane files depend on `applykit_pack` paths.

## Risk Closure Status
| Risk | Status | Evidence |
| --- | --- | --- |
| CI path drift during rewrite | closed for cutover scope | parity check pass + full root verify pass |
| Hidden rulesets not mapped | closed | rulesets snapshot captured pre-cutover (`[]`) |
| Rollback ambiguity | closed | rollback branch and pre-cutover governance snapshots captured |
| Branch check context mismatch | controlled | required checks remain `quality`, `quality-gates`; context names unchanged in workflows |
| Dual-path window complexity | closed | no active compatibility alias in runtime control-plane |

## Remaining Follow-Up
1. No blocking follow-up actions remain for the flattening cutover scope.
2. Continue treating week-based historical docs as archival records only.
