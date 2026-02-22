# ApplyKit .codex command map

| Action | Command | Source |
| --- | --- | --- |
| setup deps | `pnpm -C ui install --frozen-lockfile` | `README.md`, `.github/workflows/ci.yml` |
| lint (rust fmt) | `cargo fmt --all --check` | `.github/workflows/ci.yml` |
| lint (rust clippy) | `cargo clippy --workspace --all-targets -- -D warnings` | `.github/workflows/ci.yml` |
| lint (ui) | `pnpm -C ui lint` | `.github/workflows/ci.yml` |
| test (rust) | `cargo test` | `README.md`, `.github/workflows/ci.yml` |
| test (ui) | `pnpm -C ui test` | `README.md`, `.github/workflows/ci.yml` |
| coverage | `./.codex/scripts/run_coverage.sh` | `.github/workflows/ci.yml` |
| diff coverage | `node ./.codex/scripts/check_diff_coverage.mjs` | `.github/workflows/ci.yml` |
| docs check | `./.codex/scripts/docs_check.sh` | `AGENTS.md`, `README.md` |
| required-gate waivers | `node ./.codex/scripts/check_gate_waivers.mjs` | `AGENTS.md`, `.codex/required-gates-waivers.json` |
| build | `pnpm -C ui build && cargo tauri build --debug --bundles app` | `README.md`, `.github/workflows/ci.yml` |
| perf foundation | `bash ./.codex/scripts/run_perf_foundation.sh` | `.github/workflows/perf-foundation.yml` |
| perf enforced | `bash ./.codex/scripts/run_perf_enforced.sh` | `.github/workflows/perf-enforced.yml` |
| lean dev | `./scripts/dev_lean.sh` | `README.md` |
