# applykit_pack .codex command map

| Action | Command | Source |
| --- | --- | --- |
| setup deps | `pnpm -C ui install --frozen-lockfile` | `README.md`, `.github/workflows/ci.yml` |
| lint (rust fmt) | `cargo fmt --all --check` | `.github/workflows/ci.yml` |
| lint (rust clippy) | `cargo clippy --workspace --all-targets -- -D warnings` | `.github/workflows/ci.yml` |
| lint (ui) | `pnpm -C ui lint` | `.github/workflows/ci.yml` |
| test (rust) | `cargo test` | `README.md`, `.github/workflows/ci.yml` |
| test (ui) | `pnpm -C ui test` | `README.md`, `.github/workflows/ci.yml` |
| build | `pnpm -C ui build && cargo tauri build --debug` | `README.md`, `.github/workflows/ci.yml` |
| lean dev | `./scripts/dev_lean.sh` | `README.md` |
