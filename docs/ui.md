
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
- Export: open folder, export deterministic PDF
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

## Release polish guardrails
- Theme parity: light + dark token sets are required.
- Accessibility target: WCAG 2.2 AA for critical workflows.
- Reduced motion support via `prefers-reduced-motion`.
- Responsive target: desktop-first with laptop/tablet fallback.

## UI verification commands
- `pnpm -C /Users/d/Projects/ApplyKit/ui lint`
- `pnpm -C /Users/d/Projects/ApplyKit/ui test`
- `pnpm -C /Users/d/Projects/ApplyKit/ui test:a11y`
- `pnpm -C /Users/d/Projects/ApplyKit/ui test:e2e:a11y`
- `pnpm -C /Users/d/Projects/ApplyKit/ui test:responsive`
- `pnpm -C /Users/d/Projects/ApplyKit/ui test:coverage`
- `pnpm -C /Users/d/Projects/ApplyKit/ui build`
