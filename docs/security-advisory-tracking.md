# Security Advisory Tracking

Source of truth for active `cargo audit` ignore entries in `/Users/d/Projects/ApplyKit/.cargo/audit.toml`.

## Week 3 Status Matrix

| Advisory Group | IDs | Status | Last Validated On | Removal Blocker | Owner | Mitigation Issue | Target Removal Date | Next Review Date |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| GTK3 transitive chain | `RUSTSEC-2024-0411` .. `RUSTSEC-2024-0420`, `RUSTSEC-2024-0429` | blocked | 2026-02-22 | Transitively required by `tauri`/`wry` Linux GTK3 stack; no compatible in-range update in dry-run | applykit-platform | AK-301 | 2026-03-31 | 2026-04-05 |
| Macro dependency in GTK3 chain | `RUSTSEC-2024-0370` | blocked | 2026-02-22 | Pulled through `glib-macros` in the same GTK3 chain; no direct replacement without upstream stack move | applykit-platform | AK-301 | 2026-03-31 | 2026-04-05 |
| Hash crate transitive warning | `RUSTSEC-2025-0057` | blocked | 2026-02-22 | Pulled through `selectors -> kuchikiki -> tauri-utils`; no compatible dry-run update removed chain | applykit-platform | AK-302 | 2026-03-31 | 2026-04-05 |
| Unicode/urlpattern transitive warnings | `RUSTSEC-2025-0075`, `RUSTSEC-2025-0080`, `RUSTSEC-2025-0081`, `RUSTSEC-2025-0098`, `RUSTSEC-2025-0100` | blocked | 2026-02-22 | Pulled through `urlpattern -> tauri-utils`; no compatible dry-run update removed chain | applykit-platform | AK-303 | 2026-03-31 | 2026-04-05 |

## Week 3 Baseline Evidence

- Strict baseline command (without local ignore config):
  - `cd /tmp && cargo audit -f /Users/d/Projects/ApplyKit/Cargo.lock -D warnings --json > /tmp/applykit_week3_baseline_audit.json`
- Active advisory IDs discovered: 18
- Ignore entries in `.cargo/audit.toml`: 18
- Diff result:
  - stale ignore entries: none
  - missing ignore entries for active advisories: none

## Dependency Path Snapshots (Week 3)

- `fxhash` advisory chain (`RUSTSEC-2025-0057`):
  - `fxhash -> selectors -> kuchikiki -> tauri-utils -> tauri`
- Unicode advisory chain (`RUSTSEC-2025-0075`, `0080`, `0081`, `0098`, `0100`):
  - `unic-ucd-ident -> urlpattern -> tauri-utils -> tauri`
- GTK3 advisory chain:
  - `gtk/glib stack -> wry/tauri-runtime-wry -> tauri`
- `proc-macro-error` advisory (`RUSTSEC-2024-0370`):
  - `proc-macro-error -> glib-macros/gtk3-macros -> gtk/glib stack`

## Feasibility Checks Performed

- `cargo update --workspace --dry-run`
- `cargo update -p tauri-utils --dry-run`
- `cargo update -p glib --dry-run`
- `cargo update -p proc-macro-error --dry-run`

All dry-runs reported no compatible lockfile upgrades that remove the active advisory set.

## Week 3 Conclusion

- No stale ignore IDs were found, so no ignore removals were applied in this cycle.
- Residual advisory risk remains explicitly tracked and time-bound by owner and mitigation issue.

## Phase 4 Security Revalidation (Prepared Early)

- Canonical strict audit (with tracked ignore list): pass
  - `/tmp/applykit_phase4_week5_day3_cargo_audit.log`
  - `/tmp/applykit_phase4_week5_day5_cargo_audit.log`
- Baseline no-ignore advisory scan: expected fail (informational warnings enabled)
  - `/tmp/applykit_phase4_week5_day5_baseline_audit.log`
  - `/tmp/applykit_phase4_week5_day5_baseline_audit_summary.log`
- Revalidation outcome:
  - active advisory IDs: 18
  - ignored advisory IDs in `.cargo/audit.toml`: 18
  - stale ignore IDs: none
