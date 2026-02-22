import { readFileSync } from "node:fs";

const file = ".codex/required-gates-waivers.json";
const raw = readFileSync(file, "utf8");
const parsed = JSON.parse(raw);

const requiredWaivedGates = ["coverage", "diff_coverage"];
const now = new Date();
const sevenDaysMs = 7 * 24 * 60 * 60 * 1000;
const allowRequiredWaivers = process.env.ALLOW_REQUIRED_GATE_WAIVER === "1";

const waivers = parsed.waivers ?? [];
const byGate = new Map(waivers.map((w) => [w.gate, w]));

for (const waiver of waivers) {
  const missing = ["gate", "owner", "mitigation", "reason", "expiresOn"].filter(
    (key) => !waiver[key] || String(waiver[key]).trim() === "",
  );
  if (missing.length > 0) {
    console.error(`waiver ${waiver.gate ?? "<unknown>"} missing fields: ${missing.join(", ")}`);
    process.exit(1);
  }

  const expiry = new Date(`${waiver.expiresOn}T00:00:00Z`);
  if (Number.isNaN(expiry.getTime())) {
    console.error(`waiver ${waiver.gate} has invalid expiresOn: ${waiver.expiresOn}`);
    process.exit(1);
  }

  if (expiry.getTime() < now.getTime()) {
    console.error(`waiver ${waiver.gate} is expired (${waiver.expiresOn})`);
    process.exit(1);
  }

  if (expiry.getTime() - now.getTime() > sevenDaysMs) {
    console.error(
      `waiver ${waiver.gate} exceeds max 7-day window (${waiver.expiresOn})`,
    );
    process.exit(1);
  }
}

if (!allowRequiredWaivers) {
  const activeRequiredWaivers = requiredWaivedGates.filter((gate) => byGate.has(gate));
  if (activeRequiredWaivers.length > 0) {
    console.error(
      `required gate waivers are not allowed by default: ${activeRequiredWaivers.join(", ")}`,
    );
    console.error(
      "Set ALLOW_REQUIRED_GATE_WAIVER=1 only for explicit, time-boxed emergency renewals.",
    );
    process.exit(1);
  }
  console.log("ok: no active required gate waivers");
  process.exit(0);
}

for (const gate of requiredWaivedGates) {
  if (!byGate.has(gate)) {
    console.error(`missing waiver for required gate: ${gate}`);
    process.exit(1);
  }
}

console.log("ok: temporary required gate waivers are valid and explicitly enabled");
for (const waiver of waivers) {
  if (requiredWaivedGates.includes(waiver.gate)) {
    console.log(
      `waived ${waiver.gate} until ${waiver.expiresOn} owner=${waiver.owner} mitigation=${waiver.mitigation}`,
    );
  }
}
