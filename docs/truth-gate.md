
# Truth Gate (No-Invention Contract)
**Date:** 2026-02-14

## Supported sources
- Baseline resume templates (anchored)
- Approved bullets in `data/bullet_bank.json`
- Approved skills in `data/skills_bank.json`

## Claim levels
owned | led | partnered | supported

## Skill levels
admin | operator | familiar

## Rules
1) Never introduce a tool not in SkillsBank unless user adds it.
2) Never upgrade claim level without explicit approval.
3) Resume bullet insertion uses approved bullets only.
4) LLM rewrites must preserve semantic claim.
5) If validation fails, fall back to deterministic draft.

## Validation checks
- Unknown tool scan
- Claim-level verb constraints
- Bullet provenance recorded (bullet_id)
- Deterministic ordering tie-breakers by id
