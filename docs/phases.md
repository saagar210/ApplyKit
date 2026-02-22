
# Implementation Plan (Phases)
**Date:** 2026-02-14

## Phase 0: Spec lock + fixtures
- docs + fixtures + expected snapshots
- snapshot tests deterministic

## Phase 1: Core engine (CLI-first)
- normalize/extract/classify/score/tailor/packet
- tests + fixtures

## Phase 2: UI scaffolding
- Dashboard/New Job/Job Review/Settings
- progress events + friendly errors

## Phase 3: Templates + diff + bullet swap
- anchor templates, diff viewer, bullet picker with reasons

## Phase 4: Messages + bounded LLM
- templates + copy buttons
- adapters + post-LLM truth validation

## Phase 5: Tracking + insights
- SQLite statuses + CSV export
- weekly insights dashboard

## Phase 6: DOCX/PDF export
- template-based exports + regression checks

## Phase 7: Hardening
- migrations, atomic writes, fuzz parsing, red-team JDs
