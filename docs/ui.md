
# ApplyKit UI Spec (pleasant + keyboard-friendly)
**Date:** 2026-02-14

## Design principles
- Single-column clarity per pane; no clutter.
- Keyboard-first: ⌘K palette, ⌘Enter generate, ⌘⇧C copy.
- Explainability: every score and suggestion includes “why”.
- Truth-first: gaps are labeled; nothing is auto-invented.

## Layout
AppShell:
- Left sidebar: Jobs, Banks, Templates, Settings
- Main pane: job workflow
- Right pane: Preview/Diff (toggle)

## Screens
### Dashboard
- New Job (primary)
- Recent packets list (company, role, date, track, status)
- Search + filters (track, status, date)

### New Job
Fields:
- Company
- Role title
- Source
- Baseline (1pg/2pg)
- JD input + optional import

Primary action: Generate Packet

### Job Review (tabs)
Tabs:
- Overview: Fit score, track, keywords, gaps
- Resume: Tailor plan + bullet swap controls + preview
- Messages: recruiter/manager/cover note with copy buttons
- Export: open folder, export PDF (later)
- Tracker: status + next action + notes

### Banks Editor
- BulletBank: list + tags + claim level + approve toggle
- SkillsBank: skill + level + approve toggle

### Templates Editor
- Message templates + resume templates

## Must-have components
- AppShell + Sidebar
- SplitPane (Main/Preview)
- MarkdownViewer
- DiffViewer (inline + side-by-side)
- BulletPicker (tag filters, preview, reason chips)
- FitScoreCard (breakdown + why)
- GapList (missing reqs + safe framing)
- Toast notifications
- Command palette (⌘K)
