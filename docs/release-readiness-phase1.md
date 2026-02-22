# Release Readiness Checklist (Phase 1)

**Purpose:** authoritative checklist for the Week 2 release-candidate rehearsal.

## Scope
- Canonical implementation repo: `/Users/d/Projects/ApplyKit`
- Verification and CI orchestration run directly from root.
- This checklist is blocking for Phase 1 completion.

## Gate Status Matrix

| Gate | Command | Expected |
| --- | --- | --- |
| Rust format | `cargo fmt --all --check` | pass |
| Rust lint | `cargo clippy --workspace --all-targets -- -D warnings` | pass |
| Rust tests | `cargo test` | pass |
| Security audit | `cargo audit -D warnings` | pass |
| UI lint | `pnpm -C ui lint` | pass |
| UI tests | `pnpm -C ui test` | pass |
| Coverage thresholds | `./.codex/scripts/run_coverage.sh` | pass |
| Diff coverage | `node ./.codex/scripts/check_diff_coverage.mjs` | pass |
| Docs check | `./.codex/scripts/docs_check.sh` | pass |
| Waiver policy | `node ./.codex/scripts/check_gate_waivers.mjs` | pass (no active required waivers) |
| UI build | `pnpm -C ui build` | pass |
| Desktop package | `cargo tauri build --debug` | pass |
| Hygiene | `./scripts/verify_hygiene.sh` | pass |

## Export Validation

### Markdown bundle
- Trigger export from Job Review.
- Confirm output directory contains canonical packet files.

### DOCX
- Trigger export from Job Review.
- Confirm `.docx` opens and includes expected sections.

### PDF
- Trigger export from Job Review.
- Confirm output `.pdf` exists and opens.
- Re-run export for identical packet input; compare hashes for deterministic repeatability.

## Packaging Validation
- Run `cargo tauri build --debug`.
- Verify both artifacts exist:
  - `debug/bundle/macos/ApplyKit.app`
  - `debug/bundle/dmg/ApplyKit_0.1.0_aarch64.dmg`

## Security / Privacy Validation
- Confirm local-first behavior remains true:
  - no telemetry additions
  - no scraping or auto-apply logic
- Re-run `cargo audit -D warnings` and record advisory status.
- Review `.cargo/audit.toml` ignore list metadata (owner + target removal milestone).
- Confirm `.codex/required-gates-waivers.json` stays empty unless an emergency renewal is explicitly approved.

## Rollback Notes
- Code rollback: revert to previous green commit/tag in `/Users/d/Projects/ApplyKit`.
- Runtime rollback: clear generated build artifacts and relaunch from prior tag.
- Data safety:
  - packet outputs are file-based and append-only by job folder
  - tracker data in `applykit.db` should be backed up before downgrade migrations

## Golden Path Rehearsal
1. `pnpm -C ui install --frozen-lockfile`
2. `bash ./.codex/scripts/run_verify_commands.sh`
3. `cargo run -p applykit_cli -- generate ...` with fixture JD
4. Open desktop app, export Markdown/DOCX/PDF from generated packet
5. `cargo tauri build --debug`
6. Record artifact paths, verification logs, and date
