import { existsSync, readFileSync } from "node:fs";
import { execSync } from "node:child_process";
import path from "node:path";

const repoRoot = process.cwd();
const rustLcovPath = "coverage/rust.lcov";
const uiLcovPath = "coverage/ui/lcov.info";
const minPct = Number(process.env.DIFF_COVERAGE_MIN_PCT ?? 80);
const sourceRoots = ["crates", "src-tauri", "ui/src"];
const codeExtensions = new Set([".rs", ".ts", ".tsx", ".js", ".jsx"]);

function run(command) {
  return execSync(command, {
    cwd: repoRoot,
    encoding: "utf8",
    stdio: ["ignore", "pipe", "pipe"],
  });
}

function detectBaseRef() {
  const explicit = process.env.DIFF_COVERAGE_BASE_REF;
  if (explicit && explicit.trim() !== "") {
    return explicit.trim();
  }

  if (process.env.CI !== "true") {
    return "HEAD";
  }

  const ciBase = process.env.GITHUB_BASE_REF?.trim();
  const candidates = [
    ciBase ? `origin/${ciBase}` : "",
    "origin/main",
    "origin/master",
    "main",
    "master",
  ].filter(Boolean);
  for (const candidate of candidates) {
    try {
      run(`git rev-parse --verify ${candidate}`);
      return candidate;
    } catch {}
  }

  try {
    run("git rev-parse --verify HEAD~1");
    return "HEAD~1";
  } catch {
    return "HEAD";
  }
}

function normalizeFilePath(rawPath) {
  if (!rawPath) {
    return "";
  }

  const normalized = rawPath.replaceAll("\\", "/");
  if (normalized.startsWith("ui/src/") || normalized.startsWith("crates/") || normalized.startsWith("src-tauri/")) {
    return normalized;
  }
  if (normalized.startsWith("src/")) {
    return `ui/${normalized}`;
  }
  if (normalized.startsWith(repoRoot.replaceAll("\\", "/"))) {
    return normalized.slice(repoRoot.length + 1);
  }
  return normalized.replace(/^(\.\/)+/, "");
}

function parseDiffRanges(diffText) {
  const fileMap = new Map();
  let currentFile = null;

  for (const line of diffText.split("\n")) {
    if (line.startsWith("+++ ")) {
      const rawPath = line.slice(4).trim();
      if (rawPath === "/dev/null") {
        currentFile = null;
        continue;
      }
      const cleaned = rawPath.startsWith("b/") ? rawPath.slice(2) : rawPath;
      currentFile = normalizeFilePath(cleaned);
      if (!fileMap.has(currentFile)) {
        fileMap.set(currentFile, new Set());
      }
      continue;
    }

    if (!currentFile || !line.startsWith("@@")) {
      continue;
    }

    const hunkMatch = line.match(/^\@\@ -\d+(?:,\d+)? \+(\d+)(?:,(\d+))? \@\@/);
    if (!hunkMatch) {
      continue;
    }
    const start = Number(hunkMatch[1]);
    const count = Number(hunkMatch[2] ?? 1);
    const range = fileMap.get(currentFile);
    if (!range) {
      continue;
    }
    for (let i = 0; i < count; i += 1) {
      range.add(start + i);
    }
  }

  return fileMap;
}

function parseLcov(pathToLcov) {
  const result = new Map();
  if (!existsSync(pathToLcov)) {
    return result;
  }

  const text = readFileSync(pathToLcov, "utf8");
  let currentFile = "";

  for (const line of text.split("\n")) {
    if (line.startsWith("SF:")) {
      currentFile = normalizeFilePath(line.slice(3).trim());
      if (!result.has(currentFile)) {
        result.set(currentFile, new Map());
      }
      continue;
    }

    if (!line.startsWith("DA:") || !currentFile) {
      continue;
    }

    const match = line.match(/^DA:(\d+),(\d+)/);
    if (!match) {
      continue;
    }

    const lineNo = Number(match[1]);
    const hits = Number(match[2]);
    const lineMap = result.get(currentFile);
    if (!lineMap) {
      continue;
    }
    lineMap.set(lineNo, hits);
  }

  return result;
}

function isTrackedCodeFile(relPath) {
  const ext = path.extname(relPath);
  if (!codeExtensions.has(ext)) {
    return false;
  }
  if (!sourceRoots.some((root) => relPath.startsWith(`${root}/`) || relPath === root)) {
    return false;
  }

  if (relPath.startsWith("ui/src/")) {
    if (
      relPath.includes("/test/") ||
      relPath.includes("/__tests__/") ||
      relPath.endsWith(".test.ts") ||
      relPath.endsWith(".test.tsx") ||
      relPath.endsWith("/main.tsx")
    ) {
      return false;
    }
  }

  if (relPath.startsWith("crates/") || relPath.startsWith("src-tauri/src/")) {
    if (
      relPath.endsWith("/tests.rs") ||
      relPath.endsWith("_test.rs") ||
      relPath.includes("/tests/")
    ) {
      return false;
    }
  }

  return true;
}

if (!existsSync(rustLcovPath)) {
  console.error(`missing Rust lcov report: ${rustLcovPath}`);
  process.exit(1);
}
if (!existsSync(uiLcovPath)) {
  console.error(`missing UI lcov report: ${uiLcovPath}`);
  process.exit(1);
}
if (!Number.isFinite(minPct)) {
  console.error("DIFF_COVERAGE_MIN_PCT must be numeric");
  process.exit(2);
}

const baseRef = detectBaseRef();
const diffCmd = `git diff --unified=0 ${baseRef} -- crates src-tauri ui/src`;
const diffText = run(diffCmd);
const changedRanges = parseDiffRanges(diffText);

const rustCoverage = parseLcov(rustLcovPath);
const uiCoverage = parseLcov(uiLcovPath);
const mergedCoverage = new Map([...rustCoverage, ...uiCoverage]);

let measured = 0;
let covered = 0;
const uncoveredLines = [];
const skippedFiles = [];

for (const [file, lines] of changedRanges.entries()) {
  if (!isTrackedCodeFile(file)) {
    continue;
  }
  if (lines.size === 0) {
    continue;
  }

  const coverageForFile = mergedCoverage.get(file);
  if (!coverageForFile) {
    skippedFiles.push(file);
    continue;
  }

  for (const lineNo of lines) {
    if (!coverageForFile.has(lineNo)) {
      continue;
    }
    measured += 1;
    if ((coverageForFile.get(lineNo) ?? 0) > 0) {
      covered += 1;
    } else {
      uncoveredLines.push(`${file}:${lineNo}`);
    }
  }
}

if (skippedFiles.length > 0) {
  console.log("note: skipped changed files without direct lcov mappings:");
  for (const file of skippedFiles) {
    console.log(`- ${file}`);
  }
}

if (measured === 0) {
  console.log(`ok: no measured changed lines for diff coverage (base=${baseRef})`);
  process.exit(0);
}

const pct = (covered / measured) * 100;
console.log(
  JSON.stringify(
    {
      baseRef,
      measuredChangedLines: measured,
      coveredChangedLines: covered,
      diffCoveragePct: Number(pct.toFixed(2)),
      minPct,
    },
    null,
    2,
  ),
);

if (pct < minPct) {
  console.error(`diff coverage below threshold: ${pct.toFixed(2)}% < ${minPct}%`);
  if (uncoveredLines.length > 0) {
    console.error("uncovered changed lines:");
    for (const item of uncoveredLines.slice(0, 50)) {
      console.error(`- ${item}`);
    }
  }
  process.exit(1);
}

console.log("ok: diff coverage threshold satisfied");
