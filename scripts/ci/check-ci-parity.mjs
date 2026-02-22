import { readFileSync } from "node:fs";

const checks = [
  {
    file: ".github/workflows/quality-gates.yml",
    contains: [
      "pnpm -C ui install --frozen-lockfile",
      "bash ./.codex/scripts/run_verify_commands.sh",
    ],
  },
  {
    file: ".github/workflows/perf-foundation.yml",
    contains: ["bash ./.codex/scripts/run_perf_foundation.sh"],
  },
  {
    file: ".github/workflows/perf-enforced.yml",
    contains: ["bash ./.codex/scripts/run_perf_enforced.sh"],
  },
  {
    file: ".github/workflows/ci.yml",
    contains: ["bash ./.codex/scripts/run_verify_commands.sh"],
  },
  {
    file: ".github/workflows/codex-quality-security.yml",
    contains: ["bash ./.codex/scripts/run_verify_commands.sh"],
  },
  {
    file: ".codex/verify.commands",
    contains: [
      "node scripts/ci/check-ci-parity.mjs",
      "./.codex/scripts/run_coverage.sh",
      "node ./.codex/scripts/check_diff_coverage.mjs",
      "node ./.codex/scripts/check_gate_waivers.mjs",
    ],
  },
];

let failed = false;

for (const check of checks) {
  const content = readFileSync(check.file, "utf8");
  for (const needle of check.contains) {
    if (!content.includes(needle)) {
      console.error(`missing required parity token in ${check.file}: ${needle}`);
      failed = true;
    }
  }
}

if (failed) {
  process.exit(1);
}

console.log("ok: root/product CI parity tokens verified");
