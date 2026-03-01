# UI Contrast Audit (2026-03-01)

## Scope
Manual contrast review of core theme tokens and interactive states in `/Users/d/Projects/ApplyKit/ui/src/styles/global.css` for WCAG 2.2 AA conformance.

## Method
- Evaluated representative foreground/background token pairs used by body text, muted text, buttons, danger states, and focus indicators.
- Contrast ratios calculated with WCAG relative luminance formula.
- Thresholds:
  - Normal text: >= 4.5:1
  - Large text/UI components/focus indicator contrast: >= 3:1

## Results
| Pair | Ratio | Threshold | Status |
|---|---:|---:|---|
| Light text on surface (`#0f1f33` on `#ffffff`) | 16.61 | 4.5 | Pass |
| Light muted on surface (`#44546a` on `#ffffff`) | 7.71 | 4.5 | Pass |
| Light soft on surface (`#657791` on `#ffffff`) | 4.56 | 4.5 | Pass |
| Light primary button text (`#ffffff` on `#0b8367`) | 4.71 | 4.5 | Pass |
| Light primary button text strong stop (`#ffffff` on `#096e55`) | 6.23 | 4.5 | Pass |
| Light danger text (`#b3272a` on `#ffffff`) | 6.48 | 4.5 | Pass |
| Light focus ring (`#2668d2` on `#ffffff`) | 5.26 | 3.0 | Pass |
| Dark text on surface (`#ecf4ff` on `#0f1f33`) | 14.99 | 4.5 | Pass |
| Dark muted on surface (`#b5c6db` on `#0f1f33`) | 9.54 | 4.5 | Pass |
| Dark primary button text (`#04241e` on `#2dd2a6`) | 8.53 | 4.5 | Pass |
| Dark primary button text strong stop (`#04241e` on `#1eb28a`) | 6.10 | 4.5 | Pass |
| Dark danger text (`#ff6d72` on `#0f1f33`) | 6.08 | 4.5 | Pass |
| Dark focus ring (`#8ab6ff` on `#0f1f33`) | 8.08 | 3.0 | Pass |

## Remediation Applied
- Updated primary accent token and button foreground token to resolve prior light-theme primary button contrast risk.
  - `--accent` changed to `#0b8367`.
  - `--accent-strong` retained at `#096e55`.
  - Added `--accent-foreground` token with theme-specific values.

## Verdict
- Core token-driven text, state, and focus combinations reviewed in this audit meet WCAG 2.2 AA thresholds.
- No open contrast blockers remain from this review pass.
