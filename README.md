# ApplyKit (Local-First Apply Engine)
Generated: 2026-02-14

ApplyKit generates a deterministic, truth-gated application packet from a job description.

## Repository Status (Flattened Root)
- Canonical product root is now `/Users/d/Projects/ApplyKit`.
- Legacy nested-repo boundary contracts are retired from active control-plane checks.
- Historical phase evidence docs that include old nested absolute paths are archival records and not runtime command sources.

## Principles
- Local-first only: no scraping, no auto-apply, no telemetry.
- Truth Gate: generated claims come only from approved local templates/banks.
- Deterministic: same input + same config => same output.
- Architecture: business logic in `crates/applykit_core`; `src-tauri` is RPC only; UI is orchestration/rendering.

## Repository Layout
- `crates/applykit_core`: normalization, extraction, track classification, scoring, tailoring, truth validation, packet writing, storage.
- `crates/applykit_llm`: bounded local provider adapters (Ollama, LM Studio, llama.cpp-compatible).
- `crates/applykit_export`: deterministic markdown bundle + deterministic DOCX export (`PDF` intentionally bounded stub).
- `crates/applykit_cli`: `applykit generate` command.
- `src-tauri`: desktop RPC bridge.
- `ui`: React + TypeScript desktop UI.

## Setup
1. Install Rust toolchain and Node 22+.
2. Install UI dependencies:
   - `pnpm -C ui install`

## Run CLI
- Generate packet:
  - `cargo run -p applykit_cli -- generate --company "Acme" --role "Senior Support Engineer" --source "LinkedIn" --baseline 1pg --jd fixtures/jd_support_ops_01.txt --outdir /tmp/applykit_packets --date 2026-02-14`

Output folder pattern:
- `<outdir>/<Company>_<Role>_<YYYY-MM-DD>/`
- Files include `JD.txt`, `Extracted.json`, `FitScore.md`, `TailorPlan.md`, tailored resume(s), messages, `TrackerRow.csv`, and `Diff.md`.

Default output base:
- `config/applykit.toml` -> `output.base_dir` (used when `--outdir` is omitted).

## Run Desktop UI
- Normal dev mode:
  - `cargo tauri dev`
    - `src-tauri/tauri.conf.json` runs `pnpm -C ../ui dev` via `beforeDevCommand`.
- Lean dev mode (low disk):
  - `./scripts/dev_lean.sh`
    - Uses temporary cache/build locations (for example Rust `target`) and cleans heavy artifacts when the app exits.
- Production build:
  - `pnpm -C ui build`
  - `cargo tauri build --debug`

## Disk Usage + Cleanup
- Heavy build artifacts only (fast cleanup, keep dependencies for speed):
  - `./scripts/clean_heavy_artifacts.sh`
- Full local reproducible cache cleanup (maximum disk reclaim; slower next startup):
  - `./scripts/clean_local_caches.sh`
- Dry-run either cleanup script:
  - `./scripts/clean_heavy_artifacts.sh --dry-run`
  - `./scripts/clean_local_caches.sh --dry-run`
- Backward-compatible alias:
  - `./scripts/clean_bloat.sh` (forwards to `./scripts/clean_local_caches.sh`)

Tradeoff summary:
- Normal dev: fastest restarts, larger local disk usage (notably `target` and dependency caches).
- Lean dev: lower steady-state disk usage via ephemeral caches and auto-clean, but first compile after each start is slower.

## Verification
- `cargo test -p applykit_core`
- `cargo test`
- `cargo audit -D warnings`
- `pnpm -C ui lint`
- `pnpm -C ui test`
- `./.codex/scripts/run_coverage.sh`
- `node ./.codex/scripts/check_diff_coverage.mjs`
- `pnpm -C ui build`
- `cargo tauri build --debug`
- `./scripts/verify_hygiene.sh`
- `bash ./.codex/scripts/run_perf_foundation.sh`
- `bash ./.codex/scripts/run_perf_enforced.sh`
- `./scripts/clean_heavy_artifacts.sh --dry-run`
- `./scripts/clean_heavy_artifacts.sh`
- `./scripts/clean_local_caches.sh --dry-run`
- `./scripts/clean_local_caches.sh`

## Export Notes
- Markdown bundle export is production-ready and deterministic.
- DOCX export is deterministic and generated from a fixed section-order mapping.
- PDF export is deterministic and uses a fixed page layout, section order, and stable metadata behavior.

## Runtime Settings
- User-level runtime overrides are stored at:
  - `config/applykit.user.toml`
- Settings currently support:
  - `allow_unapproved`
  - local LLM provider/base URL/model
  - allowed LLM task toggles (`rewrite_message`, `rewrite_bullet`, `summarize_jd`)

## UI Highlights
- Dashboard filters: date window + track + status + search.
- Right pane controls: `Preview` / `Diff` toggle and hide/show split pane.
- New Job supports JD paste and optional `.txt`/`.md` import.
- Banks and Templates are editable in-app with validated local saves.

## Safe Data Updates (Truth Gate)
- Bullets: create/edit in-app (Banks screen) or manually in `data/bullet_bank.json` under `bullets[]`.
  - Use unique lowercase snake-style `id` (`^[a-z0-9_]+$`).
  - Keep `approved=true` only for verified claims.
  - Keep `claim_level` within `owned|led|partnered|supported`.
- Skills: create/edit in-app (Banks screen) or manually in `data/skills_bank.json` under `skills` map.
  - Keep `approved=true` only for real experience.
  - Accepted levels normalize to `admin|operator|familiar` (`strong` aliases to `admin`).
- Templates: edit in-app (Templates screen) or in local files.
  - Resume anchors must keep `<!--SECTION:...-->` markers.
  - Message placeholders must remain in `{{var}}` format.

## Operator Runbook
- Updating bullets safely:
  - Add/modify bullet entries in `data/bullet_bank.json`.
  - Keep `approved=false` for any candidate claim until verified.
  - Re-run: `cargo test -p applykit_core`.
- Updating skills safely:
  - Add/modify entries in `data/skills_bank.json`.
  - Keep `approved=true` only for real, defensible experience.
  - Re-run: `cargo test -p applykit_core`.
- Truth-gate troubleshooting:
  - If generation fails with `unknown_tools_detected`, confirm tool tokens exist in approved `skills_bank` or approved bullet `tools`.
  - If generation fails with `provenance_validation_failed`, confirm selected bullet IDs exist and are approved.
  - If generation fails with `claim_level_constraint_failed`, remove escalation language or disallowed title claims from templates/rewrites.
- Tracker troubleshooting:
  - Status updates are validated (`new|applied|reply|interview|closed`) and return an error for unknown job IDs.

## Docs
- `docs/spec.md`
- `docs/ui.md`
- `docs/truth-gate.md`
- `docs/llm.md`
- `docs/phases.md`
- `docs/testing.md`
- `docs/release-readiness-phase1.md`
- `docs/release-runbook.md`
- `docs/launch-checklist.md`
- `docs/rollback-checklist.md`
- `docs/post-launch-verification.md`
- `docs/security-advisory-tracking.md`
- `docs/repo-flattening-prep.md`
- `docs/roadmap-2026q1.md`
- `docs/week3-checklist.md`
- `docs/week3-security-working-report-2026-03-09.md`
- `docs/week3-closeout-2026-03-13.md`
- `docs/release-rehearsal-2026-02-22.md`
- `docs/week5-regression-notes-2026-03-24.md`
- `docs/week5-hardening-closeout-2026-03-29.md`
- `docs/week6-launch-log-2026-04-01.md`
- `docs/week6-post-launch-verification-2026-04-02.md`
- `docs/week6-launch-report-2026-04-03.md`
- `docs/week7-flattening-kickoff-2026-04-06.md`
- `docs/repo-flattening-path-inventory-2026-04-07.md`
- `docs/repo-flattening-design-2026-04-08.md`
- `docs/repo-flattening-gate-parity-spec-2026-04-09.md`
- `docs/repo-flattening-branch-protection-plan-2026-04-10.md`
- `docs/repo-flattening-dry-run-1-2026-04-11.md`
- `docs/week7-flattening-closeout-2026-04-12.md`
- `docs/repo-flattening-execution-checklist-2026-04-13.md`
- `docs/repo-flattening-rollback-playbook-2026-04-14.md`
- `docs/repo-flattening-dry-run-2-2026-04-15.md`
- `docs/repo-flattening-communication-plan-2026-04-16.md`
- `docs/repo-flattening-decision-pack-2026-04-17.md`
- `docs/week8-flattening-closeout-2026-04-19.md`
