
# ApplyKit Spec (Max Conversion)
**Date:** 2026-02-14  
**Target build tool:** Codex Desktop App with **GPT-5.3-Codex (High Reasoning)**

ApplyKit is a local-first desktop assistant that turns a job description into a **high-quality application packet** (resume + messages + fit score + tailor plan) without scraping or auto-applying. It optimizes for **Max Conversion**: fewer, better applications with consistent, defensible tailoring.

## Packet outputs
For each job, generate a folder:

`Company_Role_YYYY-MM-DD/`
- `JD.txt`
- `Extracted.json`
- `FitScore.md`
- `TailorPlan.md`
- `Resume_1pg_Tailored.md`
- `Resume_2pg_Tailored.md` (optional)
- `RecruiterMessage.md`
- `HiringManagerMessage.md`
- `CoverNote_Short.md`
- `TrackerRow.csv`
- `Diff.md` (optional)

## Non-negotiables
- Local-first. No scraping. No auto-submit. No telemetry.
- Truth Gate: never invent experience. Only use approved bullets/skills.
- Deterministic: same input + same config => same outputs.
- UI is pleasant: clear flows, minimal steps, keyboard-friendly.

## Tracks
- Support/Ops Core
- Identity & Endpoint
- Security & Compliance Ops
- Automation / AIOps-adjacent
- Manager-ish

Rule-based scoring chooses track; user can override.

## Fit score (0–100)
- Role match (30)
- Stack match (30)
- Scale match (20)
- Rigor match (10)
- Signal boost (10)

Outputs must be explainable: “why you match” + “gaps”.

## Local LLM (bounded)
We support Ollama / LM Studio / llama.cpp for:
- Summarize JD into structured items
- Rewrite messages for tone
- Rewrite approved bullets for concision

LLM output must pass Truth Gate validation; otherwise fall back.
