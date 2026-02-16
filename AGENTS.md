
# AGENTS.md (ApplyKit)
Model target: GPT-5.3-Codex High Reasoning

## Hard rules
- Business logic only in crates/applykit_core
- src-tauri RPC only
- UI is render/orchestration only
- Truth Gate prevents invented claims
- Deterministic outputs required (fixtures + snapshots)

## Gates
- cargo test must stay green
- UI lint/build must stay green

## Suggested workstreams
1) applykit_core + CLI + fixtures
2) UI scaffolding + markdown preview
3) Diff viewer + bullet picker
4) LLM adapters + validation

## UI Hard Gates (when UI scope changes)
1) Reviewer emits `UIFindingV1[]` from `/Users/d/.codex/contracts/UIFindingV1.schema.json`.
2) Fixer applies UI findings in order: `P0 -> P1 -> P2 -> P3`.
3) Required states: loading, empty, error, success, disabled, focus-visible.
4) Required UI gates: static lint/type/style, visual regression, a11y regression, responsive checks, and Lighthouse CI.
5) UI done-state is blocked if any required gate is `fail` or `not-run`.

## Codex Reliability Contract

### Canonical Verification Commands (Source of Truth)
Source: `.codex/verify.commands` (derived from `.github/workflows/ci.yml`)
- lint: `cargo clippy --workspace --all-targets -- -D warnings`; `pnpm -C ui lint`
- format-check: `cargo fmt --all --check`
- typecheck: `N/A (no standalone typecheck command defined in CI/docs)`
- unit-test: `cargo test`; `pnpm -C ui test`
- integration-test: `./scripts/verify_hygiene.sh`
- build: `pnpm -C ui build`

### Definition of Done
- All commands in `.codex/verify.commands` pass via `.codex/scripts/run_verify_commands.sh`.
- No open `critical` or `high` `ReviewFindingV1` findings.
- Diff scope matches approved task scope.
- Security checks (secrets, dependency, and SAST) are clean or explicitly waived with owner + expiry.

### Agent Contract
- Reviewer agent: read-only and emits only `ReviewFindingV1` findings.
- Fixer agent: applies accepted findings in severity order and reports exact file patches + verification.
- Final verifier: re-runs `.codex/scripts/run_verify_commands.sh` and summarizes `GateReportV1`.
