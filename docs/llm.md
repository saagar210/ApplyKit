
# Local LLM Integration (bounded + safe)
**Date:** 2026-02-14

## Providers
- Ollama (default): 127.0.0.1:11434
- LM Studio (OpenAI-compatible local server)
- llama.cpp (server mode)

## Allowed tasks
- summarize_jd
- rewrite_message
- rewrite_bullet

Runtime task toggles are persisted in `config/applykit.user.toml` through Settings.

## Disallowed tasks
- generating achievements
- adding tools/metrics/titles
- upgrading claim levels

## Safety pipeline
Deterministic draft -> optional LLM rewrite -> Truth Gate validate -> fallback on fail.
