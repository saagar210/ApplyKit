
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
