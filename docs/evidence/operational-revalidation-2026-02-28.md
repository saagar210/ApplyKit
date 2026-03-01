# Operational Revalidation Evidence (2026-02-28)

## Run Metadata
- `operator`: codex-session
- `date_utc`: 2026-02-28T22:20:00Z
- `git_ref`: 4b7e653567ab9a64b845dfa024f0a8da4948785a
- `repository`: `/Users/d/Projects/ApplyKit`

## Command Evidence Summary

| Command | Source of truth | Result |
| --- | --- | --- |
| `bash ./.codex/scripts/run_verify_commands.sh` | `.codex/verify.commands` | pass |
| `bash ./.codex/scripts/run_perf_foundation.sh` | `.codex/commands.md` + `.github/workflows/perf-foundation.yml` | pass |
| `bash ./.codex/scripts/run_perf_enforced.sh` | `.codex/commands.md` + `.github/workflows/perf-enforced.yml` | pass |
| `cargo audit -D warnings` | `.codex/verify.commands` | pass |
| `cd /tmp && cargo audit -f <path-to-repo>/Cargo.lock -D warnings --json` | `docs/week3-checklist.md` | expected non-zero (`EXIT:1`) for informational advisories |
| `node scripts/ci/check-ci-parity.mjs` | `.codex/verify.commands` + `.github/workflows/quality-gates.yml` | pass |

## Advisory Drift Evidence
- Active advisory IDs discovered by baseline scan: 18
- Ignore IDs in `.cargo/audit.toml`: 18
- Stale ignore IDs: 0
- Missing ignore IDs: 0
- Residual advisory issues tracked:
  - AK-301 ([#7](https://github.com/saagar210/ApplyKit/issues/7))
  - AK-302 ([#8](https://github.com/saagar210/ApplyKit/issues/8))
  - AK-303 ([#9](https://github.com/saagar210/ApplyKit/issues/9))

## Governance Snapshot
- Branch protection:
  - required approvals: 1
  - required conversation resolution: true
  - required checks: quality, verify, perf-enforced, perf-foundation, dependency_and_misconfig, sast, secrets, enforce
- Active ruleset:
  - `main-operational-readiness` (`id=13367455`)

## Decision
- Operational readiness posture remains `go` for current scope.
- Residual security risk remains time-bound and tracked through AK-301/302/303.

## Later Outcome Note (2026-03-01)
- This file is a point-in-time snapshot from 2026-02-28.
- AK-301/302/303 were later closed on 2026-03-01 under AK-304 residual-risk acceptance, with explicit reopen triggers tied to future compatible dependency updates.
