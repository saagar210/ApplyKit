# Week 5 Regression Notes (Prepared Early)

Prepared on **2026-02-22** for the Week 5 window (**2026-03-23 to 2026-03-29**).

## Scope
- Critical workflow regression pass for:
  - packet generation
  - tracker update persistence tests
  - Markdown/DOCX/PDF export paths
  - critical export error paths

## Commands Executed
1. `cargo run -p applykit_cli -- generate --company "Acme" --role "Senior Support Engineer" --source "LinkedIn" --baseline 1pg --jd fixtures/jd_support_ops_01.txt --outdir /tmp/applykit_phase4_packets --date 2026-03-24`
2. `cargo test -p applykit_export`
3. `cargo test -p applykit_tauri`

Log: `/tmp/applykit_phase4_week5_day2_regression.log`

## Workflow Matrix
| Workflow | Result | Evidence |
| --- | --- | --- |
| Packet generation (CLI fixture) | pass | packet created at `/tmp/applykit_phase4_packets/Acme_Senior_Support_Engineer_2026-03-24` |
| Tracker row generation | pass | `TrackerRow.csv` present in generated packet |
| Markdown export coverage | pass | `applykit_export` tests (`markdown_bundle_exports_canonical_files`) |
| DOCX export coverage | pass | `applykit_export` tests (`docx_export_contains_expected_entries`) |
| PDF deterministic export | pass | `applykit_export` tests (`pdf_export_is_deterministic`) |
| PDF invalid packet error path | pass | `applykit_export` tests (`pdf_export_rejects_missing_packet_dir`) |
| PDF invalid output path error path | pass | `applykit_export` tests (`pdf_export_rejects_non_directory_output_parent`) |
| Tauri PDF command success path | pass | `applykit_tauri` tests (`export_pdf_cmd_writes_pdf_for_valid_packet`) |
| Tauri PDF command non-packet path | pass | `applykit_tauri` tests (`export_pdf_cmd_rejects_non_packet_dir`) |

## Severity Outcome
- Sev-1 issues: 0
- Sev-2 issues: 0
- Launch-blocking regressions: none detected

## Notes
- This is an early execution artifact; repeat spot checks are still expected during the live Week 5 window if dependency graph changes.
