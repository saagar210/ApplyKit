
# Testing & Verification
**Date:** 2026-02-14

## Golden fixtures
fixtures/jd_*.txt + expected snapshots (normalized)

## Test tiers
- Unit: extraction, classification, scoring, truth validation
- Snapshot: full packet outputs (`JD.txt`, `Extracted.json`, `FitScore.md`, `TailorPlan.md`, resumes/messages, `TrackerRow.csv`, `Diff.md`)
- UI smoke: generate + preview + open folder
- Property-based: JD normalization/extraction determinism and parser robustness
- Export determinism: DOCX/PDF deterministic outputs for identical packet input

## Required Gate Additions (Phase 1)
- Docs check: `./.codex/scripts/docs_check.sh`
- Coverage thresholds: `./.codex/scripts/run_coverage.sh`
- Diff coverage: `node ./.codex/scripts/check_diff_coverage.mjs`
- Required gate waiver enforcement: `node ./.codex/scripts/check_gate_waivers.mjs` (must be empty unless explicitly enabled for emergency renewal)

## Red-team
- prompt injection in JD
- “add skills you don't have” (must be Gap)
- fixture-based injections in `fixtures/jd_redteam_*.txt`
