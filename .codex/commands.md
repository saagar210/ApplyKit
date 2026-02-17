# ApplyKit .codex command map

This wrapper repo delegates command execution to `/Users/d/Projects/ApplyKit/applykit_pack`.

| Action | Command | Source |
| --- | --- | --- |
| setup deps | `pnpm -C ui install --frozen-lockfile` | `applykit_pack/README.md`, `applykit_pack/.github/workflows/ci.yml` |
| lint (rust fmt) | `cargo fmt --all --check` | `applykit_pack/.github/workflows/ci.yml` |
| lint (rust clippy) | `cargo clippy --workspace --all-targets -- -D warnings` | `applykit_pack/.github/workflows/ci.yml` |
| lint (ui) | `pnpm -C ui lint` | `applykit_pack/.github/workflows/ci.yml` |
| test (rust) | `cargo test` | `applykit_pack/README.md`, `applykit_pack/.github/workflows/ci.yml` |
| test (ui) | `pnpm -C ui test` | `applykit_pack/README.md`, `applykit_pack/.github/workflows/ci.yml` |
| build | `pnpm -C ui build && cargo tauri build --debug` | `applykit_pack/README.md`, `applykit_pack/.github/workflows/ci.yml` |
| lean dev | `./scripts/dev_lean.sh` | `applykit_pack/README.md` |
