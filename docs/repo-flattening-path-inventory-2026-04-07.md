# Repo Flattening Path Inventory (Prepared Early)

Prepared on **2026-02-22** for Week 7 Day 2 (**2026-04-07**).

## Inventory Method
Command:
- `rg -n "applykit_pack/|applykit_pack\\b" /Users/d/Projects/ApplyKit -g '!**/target/**' -g '!**/node_modules/**' -g '!**/.git/**'`

Evidence log:
- `/tmp/applykit_phase5_path_inventory_raw.log`

## Inventory Summary
- Total references found: 57
- High-risk control-plane references: 24
- Documentation references: 33

## Classified Path-Coupled Files
| File | Classification | Risk | Notes |
| --- | --- | --- | --- |
| `/Users/d/Projects/ApplyKit/.codex/verify.commands` | must rewrite | critical | current root verify delegates into `applykit_pack` |
| `/Users/d/Projects/ApplyKit/.github/workflows/quality-gates.yml` | must rewrite | critical | root workflow installs deps in `applykit_pack/ui` |
| `/Users/d/Projects/ApplyKit/.github/workflows/perf-foundation.yml` | must rewrite | high | delegates to `applykit_pack/.codex/scripts/run_perf_foundation.sh` |
| `/Users/d/Projects/ApplyKit/.github/workflows/perf-enforced.yml` | must rewrite | high | delegates to `applykit_pack/.codex/scripts/run_perf_enforced.sh` |
| `/Users/d/Projects/ApplyKit/scripts/ci/check-ci-parity.mjs` | must rewrite | critical | explicitly asserts `applykit_pack/...` tokens |
| `/Users/d/Projects/ApplyKit/scripts/ci/require-tests-and-docs.mjs` | must rewrite | high | regex/path policies currently key on nested paths |
| `/Users/d/Projects/ApplyKit/README.md` | must rewrite | medium | currently states wrapper-only contract |
| `/Users/d/Projects/ApplyKit/docs/release-runbook.md` | temporary shim then rewrite | medium | absolute paths reference nested location |
| `/Users/d/Projects/ApplyKit/docs/release-rehearsal-2026-02-22.md` | temporary shim then rewrite | medium | evidence paths include `applykit_pack` |
| `/Users/d/Projects/ApplyKit/docs/security-advisory-tracking.md` | temporary shim then rewrite | medium | absolute lockfile/config paths |
| `/Users/d/Projects/ApplyKit/docs/week3-*.md` | retire/archive | low | historical, keep as immutable records |
| `/Users/d/Projects/ApplyKit/docs/week5-*.md` and `week6-*.md` | retire/archive | low | historical evidence, update only index links if needed |

## Risk Concentration
1. Verification bootstrap (`.codex/verify.commands`) and CI parity script are coupled and must change together.
2. Workflow path rewrites must remain synchronized with policy scripts to avoid false parity failures.
3. Historical docs contain absolute paths; these should be retained as historical context and not rewritten in place unless required.

## Exit Gate
- 100% of discovered path-coupled references are classified as rewrite/shim/retire with risk tags.

## Live Revalidation Addendum (2026-02-22)
The original inventory command did not include hidden paths, so it missed key control-plane files under `.codex/` and `.github/`. Live-window revalidation used a hidden-path-aware command and excluded generated outputs.

Command:
- `rg -n --hidden "applykit_pack/|applykit_pack\\b" /Users/d/Projects/ApplyKit -g '!**/target/**' -g '!**/node_modules/**' -g '!**/.git/**' -g '!**/coverage/**' -g '!**/.perf-results/**' -g '!**/dist/**'`

Evidence log:
- `/tmp/applykit_phase5_live_path_inventory_control.log`

Live summary:
- Total references found: 118
- Unique files with references: 29
- High-risk control-plane references: 37
  - control-plane files: `/Users/d/Projects/ApplyKit/.codex/verify.commands`, `/Users/d/Projects/ApplyKit/.github/workflows/quality-gates.yml`, `/Users/d/Projects/ApplyKit/.github/workflows/perf-foundation.yml`, `/Users/d/Projects/ApplyKit/.github/workflows/perf-enforced.yml`, `/Users/d/Projects/ApplyKit/scripts/ci/check-ci-parity.mjs`, `/Users/d/Projects/ApplyKit/scripts/ci/require-tests-and-docs.mjs`, `/Users/d/Projects/ApplyKit/README.md`

Revalidation result:
1. Original high-risk file classification remains correct.
2. Hidden-path discovery is now explicitly covered in the migration packet.
3. Path inventory can be treated as execution-ready for cutover planning.
