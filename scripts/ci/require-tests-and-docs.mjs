import { execSync } from 'node:child_process';

const defaultBaseRef = (() => {
  try {
    return execSync('git symbolic-ref refs/remotes/origin/HEAD', { encoding: 'utf8' }).trim().replace('refs/remotes/', '');
  } catch {
    return 'origin/main';
  }
})();

const baseRef = process.env.GITHUB_BASE_REF ? `origin/${process.env.GITHUB_BASE_REF}` : defaultBaseRef;
const changed = execSync(`git diff --name-only ${baseRef}...HEAD`, { encoding: 'utf8' })
  .split('\n')
  .map((line) => line.trim())
  .filter(Boolean);

const normalize = (file) => file.replaceAll('\\', '/');

const isProdCode = (file) => {
  const f = normalize(file);
  const wrapperCode = /^(src|app|server|api|lib)\//.test(f);
  const productCode =
    /^crates\/[^/]+\/src\/.+\.rs$/.test(f) ||
    /^src-tauri\/src\/.+\.rs$/.test(f) ||
    /^ui\/src\/.+\.(ts|tsx|css)$/.test(f);
  const isTestFile =
    /^tests\//.test(f) ||
    /^ui\/src\/.+\.(test|spec)\.[cm]?[jt]sx?$/.test(f) ||
    /^crates\/[^/]+\/src\/tests\.rs$/.test(f) ||
    /\.(test|spec)\.[cm]?[jt]sx?$/.test(f);
  return (wrapperCode || productCode) && !isTestFile;
};

const isTest = (file) => {
  const f = normalize(file);
  return (
    /^tests\//.test(f) ||
    /^ui\/src\/.+\.(test|spec)\.[cm]?[jt]sx?$/.test(f) ||
    /^crates\/[^/]+\/src\/tests\.rs$/.test(f) ||
    /\.(test|spec)\.[cm]?[jt]sx?$/.test(f)
  );
};

const isDoc = (file) => {
  const f = normalize(file);
  return (
    /^docs\//.test(f) ||
    /^openapi\//.test(f) ||
    /^README\.md$/.test(f)
  );
};

const prodChanged = changed.some(isProdCode);
const testsChanged = changed.some(isTest);
const docsChanged = changed.some(isDoc);
const workflowChanged = changed.some(
  (file) =>
    normalize(file).startsWith('.github/workflows/'),
);

if (prodChanged && !testsChanged) {
  console.error('Policy failure: production code changed without test updates.');
  process.exit(1);
}

if (prodChanged && !docsChanged) {
  console.error('Policy failure: production code changed without docs/OpenAPI updates.');
  process.exit(1);
}

if (workflowChanged) {
  try {
    execSync('node scripts/ci/check-ci-parity.mjs', { stdio: 'inherit' });
  } catch {
    console.error('Policy failure: CI workflow updates must pass parity guard.');
    process.exit(1);
  }
}

console.log('Policy checks passed.');
