#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
DATE_UTC="${DATE_UTC:-$(date -u +%Y-%m-%d)}"
EVIDENCE_DIR="${REPO_ROOT}/docs/evidence"
BASELINE_JSON="${EVIDENCE_DIR}/security-revalidation-${DATE_UTC}-baseline.json"
CANONICAL_AUDIT_LOG="${EVIDENCE_DIR}/security-revalidation-${DATE_UTC}-canonical-audit.log"
VERIFY_LOG="${EVIDENCE_DIR}/security-revalidation-${DATE_UTC}-verify.log"
REPORT_MD="${EVIDENCE_DIR}/security-revalidation-${DATE_UTC}.md"

mkdir -p "${EVIDENCE_DIR}"

set +e
(
  cd /tmp
  cargo audit -f "${REPO_ROOT}/Cargo.lock" -D warnings --json > "${BASELINE_JSON}"
)
BASELINE_EXIT=$?
set -e

(
  cd "${REPO_ROOT}"
  cargo audit -D warnings > "${CANONICAL_AUDIT_LOG}" 2>&1
)

(
  cd "${REPO_ROOT}"
  bash ./.codex/scripts/run_verify_commands.sh > "${VERIFY_LOG}" 2>&1
)

NODE_OUTPUT="$(node - "${BASELINE_JSON}" "${REPO_ROOT}/.cargo/audit.toml" <<'NODE'
const fs = require('fs');
const baselinePath = process.argv[2];
const auditTomlPath = process.argv[3];
const baseline = JSON.parse(fs.readFileSync(baselinePath, 'utf8'));
const warnings = baseline.warnings || {};
const active = new Set();
for (const list of Object.values(warnings)) {
  if (!Array.isArray(list)) continue;
  for (const item of list) {
    const id = item?.advisory?.id;
    if (typeof id === 'string') active.add(id);
  }
}
for (const item of baseline?.vulnerabilities?.list || []) {
  const id = item?.advisory?.id;
  if (typeof id === 'string') active.add(id);
}
const toml = fs.readFileSync(auditTomlPath, 'utf8');
const ignore = new Set((toml.match(/RUSTSEC-\d{4}-\d{4}/g) || []));
const activeIds = Array.from(active).sort();
const ignoreIds = Array.from(ignore).sort();
const stale = ignoreIds.filter((id) => !active.has(id));
const missing = activeIds.filter((id) => !ignore.has(id));
const payload = {
  activeCount: activeIds.length,
  ignoreCount: ignoreIds.length,
  activeIds,
  ignoreIds,
  stale,
  missing,
};
process.stdout.write(JSON.stringify(payload));
NODE
)"

ACTIVE_COUNT="$(node -e 'const d=JSON.parse(process.argv[1]);process.stdout.write(String(d.activeCount));' "${NODE_OUTPUT}")"
IGNORE_COUNT="$(node -e 'const d=JSON.parse(process.argv[1]);process.stdout.write(String(d.ignoreCount));' "${NODE_OUTPUT}")"
STALE_IDS="$(node -e 'const d=JSON.parse(process.argv[1]);process.stdout.write(d.stale.join(","));' "${NODE_OUTPUT}")"
MISSING_IDS="$(node -e 'const d=JSON.parse(process.argv[1]);process.stdout.write(d.missing.join(","));' "${NODE_OUTPUT}")"

GIT_SHA="$(cd "${REPO_ROOT}" && git rev-parse HEAD)"

cat > "${REPORT_MD}" <<REPORT
# Security Revalidation Report (${DATE_UTC})

- date_utc: ${DATE_UTC}
- git_ref: ${GIT_SHA}
- baseline_scan_exit: ${BASELINE_EXIT} (expected non-zero when advisory warnings exist)
- baseline_json: ${BASELINE_JSON}
- canonical_audit_log: ${CANONICAL_AUDIT_LOG}
- verify_log: ${VERIFY_LOG}
- active_advisory_count: ${ACTIVE_COUNT}
- ignore_id_count: ${IGNORE_COUNT}
- stale_ignore_ids: ${STALE_IDS:-none}
- missing_ignore_ids: ${MISSING_IDS:-none}

## Result
$(if [[ -z "${STALE_IDS}" && -z "${MISSING_IDS}" ]]; then echo "- pass: advisory ID mapping remains aligned"; else echo "- fail: advisory mapping drift detected"; fi)
REPORT

if [[ -n "${STALE_IDS}" ]]; then
  echo "stale ignore IDs detected: ${STALE_IDS}" >&2
  exit 1
fi

if [[ -n "${MISSING_IDS}" ]]; then
  echo "missing ignore mappings detected: ${MISSING_IDS}" >&2
  exit 1
fi

echo "ok: monthly security revalidation completed"
echo "report: ${REPORT_MD}"
