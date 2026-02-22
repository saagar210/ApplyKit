# Launch Checklist

## Go / No-Go Inputs
- [ ] Release runbook completed with evidence.
- [ ] All required gates green (no waivers).
- [ ] Packaging artifacts validated (`.app`, `.dmg`).
- [ ] Security advisory posture reviewed and accepted.
- [ ] Rollback checklist reviewed and executable.

## Launch Steps
1. Confirm target release commit SHA and tag candidate.
2. Build release artifacts from clean environment.
3. Run post-build smoke checks:
   - app starts
   - packet generation works
   - Markdown/DOCX/PDF export succeeds
4. Execute production release cutover.
5. Run post-launch verification within 24 hours.

## Required Outputs
- Launch timestamp
- Released commit SHA
- Artifact checksums/paths
- Known issues at launch
- Owner on-call for first 24 hours

## Phase 4 Execution Snapshot (Prepared Early)
- [x] Release runbook completed with Week 5 and Week 6 evidence.
- [x] All required gates green (no waivers).
- [x] Packaging artifacts validated (`.app`, `.dmg`).
- [x] Security advisory posture reviewed and accepted as residual/transitive risk.
- [x] Rollback checklist reviewed and executable.

### Captured Outputs
- Launch timestamp (prepared early): `2026-02-22T11:12:32Z`
- Released commit SHA: `98b67c0c3cca4efcd3d77e92b6196d9b6ae1cc3a`
- Artifact checksums/paths: `docs/week6-launch-log-2026-04-01.md`
- Known issues at launch: `docs/week6-launch-report-2026-04-03.md`
- On-call owner (first 24 hours): `applykit-platform`
