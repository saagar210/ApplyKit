# ApplyKit (Local-First Apply Engine)
Generated: 2026-02-14

ApplyKit generates a deterministic, truth-gated application packet from a job description.

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
- `pnpm -C ui build`
- `cargo tauri build --debug`
- `./scripts/verify_hygiene.sh`
- `./scripts/clean_heavy_artifacts.sh --dry-run`
- `./scripts/clean_heavy_artifacts.sh`
- `./scripts/clean_local_caches.sh --dry-run`
- `./scripts/clean_local_caches.sh`

## Export Notes
- Markdown bundle export is production-ready and deterministic.
- DOCX export is deterministic and generated from a fixed section-order mapping.
- PDF export is currently a bounded stub by design (`export_pdf` returns a clear message).

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
