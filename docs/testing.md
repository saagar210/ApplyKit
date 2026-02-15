
# Testing & Verification
**Date:** 2026-02-14

## Golden fixtures
fixtures/jd_*.txt + expected snapshots (normalized)

## Test tiers
- Unit: extraction, classification, scoring, truth validation
- Snapshot: full packet outputs (`JD.txt`, `Extracted.json`, `FitScore.md`, `TailorPlan.md`, resumes/messages, `TrackerRow.csv`, `Diff.md`)
- UI smoke: generate + preview + open folder
- Property-based: JD normalization/extraction determinism and parser robustness

## Red-team
- prompt injection in JD
- “add skills you don't have” (must be Gap)
- fixture-based injections in `fixtures/jd_redteam_*.txt`
