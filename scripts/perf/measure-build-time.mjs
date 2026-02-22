import { spawnSync } from "node:child_process";
import { mkdirSync, writeFileSync } from "node:fs";

const buildCommand = process.argv.slice(2).join(" ").trim() || process.env.PERF_BUILD_CMD || "pnpm -C ui build";
const sampleCountRaw = Number(process.env.PERF_BUILD_SAMPLES ?? 3);
const sampleCount = Number.isFinite(sampleCountRaw) && sampleCountRaw > 0 ? Math.floor(sampleCountRaw) : 3;

const samples = [];
let failedStatus = 0;

for (let i = 0; i < sampleCount; i += 1) {
  const start = Date.now();
  const result = spawnSync(buildCommand, {
    shell: true,
    stdio: "inherit",
  });
  const end = Date.now();
  const elapsed = end - start;
  samples.push(elapsed);

  if (result.status !== 0) {
    failedStatus = result.status ?? 1;
    break;
  }
}

samples.sort((a, b) => a - b);
const middle = Math.floor(samples.length / 2);
const medianMs =
  samples.length % 2 === 0
    ? Math.round((samples[middle - 1] + samples[middle]) / 2)
    : samples[middle];

mkdirSync(".perf-results", { recursive: true });
writeFileSync(
  ".perf-results/build-time.json",
  JSON.stringify(
    {
      buildMs: medianMs,
      buildSamplesMs: samples,
      sampleCount,
      capturedAt: new Date().toISOString(),
      command: buildCommand,
    },
    null,
    2,
  ),
);

if (failedStatus !== 0) {
  process.exit(failedStatus);
}
