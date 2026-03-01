# Security Hardening Cycle Report (2026-03-01)

## Scope

This cycle executed remediation against previously identified risks:

1. Unrestricted LLM egress endpoint configuration.
2. Missing LLM network timeout controls.
3. High-severity UI dependency advisories.
4. Ownership gaps for security-critical files.

## Verification Evidence

| Command | Source | Result |
| --- | --- | --- |
| `bash ./.codex/scripts/run_verify_commands.sh` | `/Users/d/Projects/ApplyKit/.codex/verify.commands` | pass |
| `cargo audit -D warnings` | `/Users/d/Projects/ApplyKit/.codex/verify.commands` | pass |
| `cd /tmp && cargo audit -f /Users/d/Projects/ApplyKit/Cargo.lock -D warnings --json > /tmp/applykit_security_baseline_2026-03-01.json` | `/Users/d/Projects/ApplyKit/docs/week3-checklist.md` | expected fail (`exit 1`, informational warnings enabled) |
| `pnpm -C ui audit --audit-level high` | `/Users/d/Projects/ApplyKit/.codex/verify.commands` | pass |
| `pnpm -C ui audit --audit-level moderate` | ad-hoc hardening check | pass |
| `node ./scripts/ci/check-ci-parity.mjs` | `/Users/d/Projects/ApplyKit/.codex/verify.commands` | pass |

## Remediation Applied

### Completed in this cycle

1. Enforced local-only LLM endpoint policy.
   - Files:
     - `/Users/d/Projects/ApplyKit/crates/applykit_core/src/config.rs`
     - `/Users/d/Projects/ApplyKit/crates/applykit_core/src/pipeline.rs`
     - `/Users/d/Projects/ApplyKit/src-tauri/src/lib.rs`
   - Outcome:
     - LLM base URL now rejects non-loopback hosts and userinfo.
     - Settings save path validates URL before persisting.
     - Pipeline fails closed and uses deterministic fallback path when blocked.

2. Added deterministic LLM HTTP timeout controls.
   - File:
     - `/Users/d/Projects/ApplyKit/crates/applykit_llm/src/lib.rs`
   - Outcome:
     - 5s connect timeout + 20s request timeout applied to adapters.

3. Reduced UI dependency risk posture.
   - File:
     - `/Users/d/Projects/ApplyKit/pnpm-workspace.yaml`
   - Outcome:
     - Added workspace overrides for `rollup`, `minimatch`, and `ajv` vulnerable ranges.
     - Current audit status: no high/moderate known vulnerabilities.

4. Enforced ownership map for security-critical surfaces.
   - File:
     - `/Users/d/Projects/ApplyKit/.github/CODEOWNERS`
   - Outcome:
     - Security-sensitive paths now have explicit owner assignment.

5. Added UI dependency-audit gate to canonical verify.
   - Files:
     - `/Users/d/Projects/ApplyKit/.codex/verify.commands`
     - `/Users/d/Projects/ApplyKit/scripts/ci/check-ci-parity.mjs`
   - Outcome:
     - High-severity UI dependency advisories now block verification.

## Prioritized Remediation Backlog (Open)

| Priority | Risk | Current State | Owner | Target Date | Next Action |
| --- | --- | --- | --- | --- | --- |
| P0 | Rust transitive advisory residuals (`RUSTSEC-2024-0411` .. `0420`, `0429`, `0370`, `2025-0057`, `0075`, `0080`, `0081`, `0098`, `0100`) | still present in strict no-ignore baseline; no stale ignore IDs | applykit-platform | 2026-03-31 | Re-open AK-301/302/303 only when compatible `tauri/wry/tauri-utils` upgrade path becomes available and run full verify before removing any ignore entry |
| P1 | UI supply-chain drift regression | high/moderate currently clear | applykit-platform | 2026-03-08 | Keep `pnpm -C ui audit --audit-level high` in blocking verify; promote `moderate` to weekly report-only monitor |
| P1 | LLM egress policy bypass risk via future code paths | main pipeline and settings path protected | applykit-platform | 2026-03-15 | Add policy test coverage for all command entry points that consume runtime settings |
| P2 | Security ownership single-point concentration | all ownership currently maps to one owner | applykit-platform | 2026-03-22 | Define backup reviewer for `.github/workflows`, `.cargo/audit.toml`, and `src-tauri` controls |

## Ownership Map (Current)

- Platform security owner: `applykit-platform`
- Control plane:
  - `/Users/d/Projects/ApplyKit/.github/workflows/`
  - `/Users/d/Projects/ApplyKit/.codex/`
  - `/Users/d/Projects/ApplyKit/scripts/ci/`
- Application boundaries:
  - `/Users/d/Projects/ApplyKit/src-tauri/`
  - `/Users/d/Projects/ApplyKit/crates/applykit_core/`
  - `/Users/d/Projects/ApplyKit/crates/applykit_llm/`
  - `/Users/d/Projects/ApplyKit/ui/`
- Security governance docs:
  - `/Users/d/Projects/ApplyKit/docs/security-*.md`
  - `/Users/d/Projects/ApplyKit/docs/release-*.md`

## Closure Decision

- Hardening cycle status: **complete** for previously identified actionable security gaps.
- Residual risk status: **accepted and time-bound** for transitive Rust advisories pending compatible upstream dependency movement.
- Next checkpoint: monthly security revalidation against baseline advisory JSON and canonical verify gates (first Monday window via `.github/workflows/security-monthly-revalidation.yml`).
