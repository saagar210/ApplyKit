import { existsSync, readFileSync } from "node:fs";

const rustLcovPath = "coverage/rust.lcov";
const uiSummaryPath = "coverage/ui/coverage-summary.json";
const rustMinPct = Number(process.env.RUST_COVERAGE_MIN_PCT ?? 70);
const uiMinPct = Number(process.env.UI_COVERAGE_MIN_PCT ?? 30);

function assertNumber(name, value) {
  if (!Number.isFinite(value)) {
    console.error(`invalid numeric value for ${name}`);
    process.exit(2);
  }
}

function parseRustLineCoveragePct(lcovText) {
  let linesFound = 0;
  let linesHit = 0;

  for (const line of lcovText.split("\n")) {
    if (!line.startsWith("DA:")) {
      continue;
    }
    const [, lineNoRaw, hitRaw] = line.match(/^DA:(\d+),(\d+)/) ?? [];
    if (!lineNoRaw || !hitRaw) {
      continue;
    }
    linesFound += 1;
    if (Number(hitRaw) > 0) {
      linesHit += 1;
    }
  }

  if (linesFound === 0) {
    return 0;
  }
  return (linesHit / linesFound) * 100;
}

if (!existsSync(rustLcovPath)) {
  console.error(`missing Rust coverage report: ${rustLcovPath}`);
  process.exit(1);
}
if (!existsSync(uiSummaryPath)) {
  console.error(`missing UI coverage report: ${uiSummaryPath}`);
  process.exit(1);
}

const rustPct = parseRustLineCoveragePct(readFileSync(rustLcovPath, "utf8"));
const uiSummary = JSON.parse(readFileSync(uiSummaryPath, "utf8"));
const uiPct = Number(uiSummary?.total?.lines?.pct ?? NaN);

assertNumber("rustPct", rustPct);
assertNumber("uiPct", uiPct);
assertNumber("RUST_COVERAGE_MIN_PCT", rustMinPct);
assertNumber("UI_COVERAGE_MIN_PCT", uiMinPct);

console.log(
  JSON.stringify(
    {
      rust: { linePct: Number(rustPct.toFixed(2)), minPct: rustMinPct },
      ui: { linePct: Number(uiPct.toFixed(2)), minPct: uiMinPct },
    },
    null,
    2,
  ),
);

if (rustPct < rustMinPct) {
  console.error(`Rust line coverage below threshold: ${rustPct.toFixed(2)}% < ${rustMinPct}%`);
  process.exit(1);
}
if (uiPct < uiMinPct) {
  console.error(`UI line coverage below threshold: ${uiPct.toFixed(2)}% < ${uiMinPct}%`);
  process.exit(1);
}

console.log("ok: coverage thresholds satisfied");
