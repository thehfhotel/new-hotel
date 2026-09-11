# Dependabot / release-please triage — 2026-09-11

Read-only triage of the 17 open Dependabot PRs (#240-#255 minus #247, which is
closed; plus #295, #297) and release-please #256 (release 2.76.0). No PRs were
merged, closed, pushed to, or commented on while producing this report.

Method: `gh pr view --json statusCheckRollup,baseRefOid,files,mergeStateStatus`
per PR, `hotel-backend/Cargo.toml`/`package.json` read against `origin/master`
to classify `[dependencies]` vs `[dev-dependencies]` / `dependencies` vs
`devDependencies`, `gh pr diff` for every changed line, and
`git rev-list --count <base>..origin/master` for staleness (origin/master HEAD
at triage time: `cb7bd76`). Breaking-change claims for the two Rust majors are
grep-verified against every `jsonwebtoken::`/`base64::` call site in
`hotel-backend/src/`, cross-checked against each crate's own CHANGELOG, and
corroborated by this repo's own CI (`test-backend`/`build-backend`, which
compile and run the real code against the bumped crate).

**Production-deploy reminder**: this repo's `docker-build.yml` `changes` job
treats `package.json`/`pnpm-lock.yaml` as "frontend" and `hotel-backend/**`
(which includes `Cargo.lock`/`Cargo.toml`/`Dockerfile`) as "backend". A
successful `build-frontend` or `build-backend` unconditionally satisfies the
`deploy` job's trigger condition. **Every PR in this batch except the five
pure GitHub-Actions-workflow bumps (#245, #246, #249, #250, #255) redeploys
the live PMS to production the moment it's merged to master.** That is normal
and by design for this repo (CLAUDE.md: "To deploy: simply `git push` to
master") — flagged per-row below only where the *content* of the bump also
touches something prod-sensitive (the backend build toolchain, the async
runtime, or auth-token verification).

## Table

| PR | Package | From → To | Scope | Semver | CI / staleness | Verdict |
|----|---------|-----------|-------|--------|-----------------|---------|
| #245 | `dorny/paths-filter` (action) | 4.0.1 → 4.0.2 | CI workflow-only (`docker-build.yml`) | patch | CLEAN, all green (incl. self-test job). Base 147 commits behind master (old, but a single-line SHA-pin diff can't conflict) | **merge-ready** |
| #246 | `softprops/action-gh-release` (action) | 3.0.1 → 3.0.2 | CI workflow-only (`middleware-build.yml`, `workflow_dispatch`-only — never exercised by PR CI) | patch | CLEAN, lint jobs green; the action itself is never invoked on `pull_request`. Base 154 commits behind | **merge-ready** — untested by CI but a 1-line SHA bump to a `workflow_dispatch` release step; blast radius is the manual middleware release flow only |
| #255 | `actions/checkout` (action) | 7.0.0 → 7.0.1 | CI workflow-only (used in every job across `codeql.yml`, `docker-build.yml`, `middleware-build.yml`) | patch | CLEAN, all green. Base 147 commits behind | **merge-ready**. Cosmetic pre-existing issue (not introduced by this PR): several trailing comments still read `# v5.0.0` even though the pinned SHA is actually the v7.x line — comment drift, not a functional problem |
| #250 | `docker/login-action` (action) | 4.4.0 → 4.5.1 | CI workflow-only (`docker-build.yml`, image push) | minor | CLEAN, all green. Base 154 commits behind | **merge-ready** |
| #249 | `actions/setup-node` (action) | 6.4.0 → 7.0.0 | CI workflow-only (`docker-build.yml`, `middleware-build.yml`) | **major** | CLEAN, lint jobs green; `build-frontend`/middleware jobs **skipped** in this PR's own run (workflow-only diff doesn't trip the `frontend`/`backend` path filter), so the actual `Setup Node.js` step was not exercised by this PR's CI. Base 154 commits behind | **merge-ready** — see migration notes |
| #242 | `tailwindcss` (devDep) | 4.3.1 → 4.3.3 | frontend dev-only | patch | CLEAN, `build-frontend`/`test-frontend` green. Base 143 commits behind | **merge-ready** |
| #244 | `eslint-config-next` (devDep) | 16.2.6 → 16.2.12 | frontend dev-only (lint) | patch-ish (pinned exact, no caret) | CLEAN, green. Base 143 commits behind | **merge-ready** |
| #241 | `@types/node` (devDep) | 25.9.3 → 26.1.2 | frontend dev-only (types, no runtime code) | **major** | CLEAN, `build-frontend`/`test-frontend` (tsc) green. Base 143 commits behind | **merge-ready** — see migration notes |
| #240 | `lucide-react` (dep) | 1.18.0 → 1.27.0 | frontend runtime (icon components) | minor (weekly 1.x releases, additive icon sets) | CLEAN, green. Base 143 commits behind | **merge-ready** |
| #243 | `recharts` (dep) | 3.9.1 → 3.9.2 | frontend runtime (dashboard/report charts) | patch | CLEAN, green. Base 154 commits behind | **merge-ready** |
| #248 | `uuid` (dep, lockfile-only) | 1.23.4 → 1.24.0 | backend runtime (`[dependencies]`, `Cargo.lock`-only — `Cargo.toml` still pins `"1"`) | minor | CLEAN, `test-backend`/`build-backend` green. Base 154 commits behind | **merge-ready** |
| #254 | `async-trait` (dep, lockfile-only) | 0.1.89 → 0.1.91 | backend runtime (`[dependencies]`, `Cargo.lock`-only) | patch | CLEAN, `test-backend`/`build-backend` green. Base 154 commits behind | **merge-ready** |
| #251 | `tokio` (dep, lockfile-only) | 1.52.3 → 1.53.1 | backend runtime (`[dependencies]`, `Cargo.lock`-only — `Cargo.toml` pins `"1"`) | minor | CLEAN, `test-backend`/`build-backend` green (macOS/Windows middleware matrix also green). Base 58 commits behind (freshest of the Rust batch) | **merge-ready** — deploys prod; tokio is the async runtime under every Axum route, `writeback`, `sync` and scheduler binary. Minor bump, no `unsafe`/API-breaking history in 1.53.x, CI green |
| #252 | `base64` (dep) | 0.22.1 → 0.23.0 | backend runtime (`[dependencies]`, `Cargo.toml` + `Cargo.lock`) | "minor" per Cargo (pre-1.0, so treat as potentially breaking) | CLEAN, `test-backend`/`build-backend` green. Base 154 commits behind | **merge-ready** — see migration notes |
| #253 | `jsonwebtoken` (dep) | 10.4.0 → 11.0.0 | backend runtime (`[dependencies]`, `Cargo.toml` + `Cargo.lock`) | **major** | CLEAN, `test-backend`/`build-backend` green. Base 154 commits behind | **merge-ready** — see migration notes. Deploys prod; this crate verifies every card-login, Cloudflare Access, and `/hk` badge JWT, so treat the green CI as necessary but read the notes below before merging |
| #295 | `rust` (Docker base image, `hotel-backend/Dockerfile`) | 1.96-bookworm → 1.98-bookworm | backend build toolchain | minor (toolchain) | CLEAN, `test-backend`/`build-backend` green, digest-pinned. Base **8** commits behind — freshest PR in the batch | **merge-ready** — see migration notes. Rebuilds and relinks every production binary (`hotel-backend`, `writeback`, `sync`, all `backfill_*`, `migrate_legacy`, `create_user`) |
| #297 | npm group: `next` 16.2.12→16.3.3, `js-yaml` (indirect, override interaction) | `next` minor; group also touches `js-yaml` | frontend runtime (`next` is the framework) | minor (`next`); the `js-yaml` line is transitive/override noise | **UNSTABLE / CI FAILING.** `test-frontend` and `build-frontend` both fail with `ERR_PNPM_LOCKFILE_CONFIG_MISMATCH`. Base 10 commits behind | **hold — needs code change** (see below) |
| #256 | release-please: version bump to **2.76.0** | n/a | `CHANGELOG.md`, `package.json` version field, `.release-please-manifest.json` | n/a (meta) | **mergeStateStatus UNSTABLE; `statusCheckRollup` empty.** Every triggered workflow run on this PR (CodeQL, docker-build) since 2026-09-10 has `conclusion: action_required` at 0s — the run never actually started, it's parked waiting on a manual Actions approval. **No CI evidence exists for this PR.** Base 1 commit behind master (freshest possible) | **hold — needs human**: approve/trigger the gated workflow run before there is any CI signal to merge on |

## Why #297 is broken (root cause, not just symptom)

`gh run view` on the failing `test-frontend` job shows the exact error:

```
ERR_PNPM_LOCKFILE_CONFIG_MISMATCH  Cannot proceed with the frozen installation.
The current "overrides" configuration doesn't match the value found in the lockfile
Update your lockfile using "pnpm install --no-frozen-lockfile"
```

The diff explains why: Dependabot's lockfile regeneration for this PR **deleted
the entire top-level `overrides:` block from `pnpm-lock.yaml`**
(`@babel/core`, `ajv`, three `brace-expansion` ranges, `flatted`, `glob`,
`handlebars`, `js-yaml`, `lodash`, three `minimatch` ranges, two `picomatch`
ranges, `postcss`, `sharp`, `ws`) while leaving `package.json`'s
`pnpm.overrides` section (still pinning `js-yaml: ^4.3.0`, `postcss:
^8.5.18`, `sharp: >=0.35.0`, etc.) completely untouched. Those overrides are
the CVE-remediation pins from `f0a5fc1 fix(deps): patch 4 high + 5 medium
Next.js CVEs, plus postcss/sharp/js-yaml/brace-expansion` (already on
master, landed in v2.76.0 per the #256 changelog). The lockfile now disagrees
with `package.json` about what "overrides" should be, so `pnpm install
--frozen-lockfile` (what CI runs) refuses to proceed — it's a config
integrity check, not a real dependency conflict. The dangling
`js-yaml@3.15.2` entry the diff also introduces (alongside the still-present
`js-yaml@4.3.0`) is a symptom of the same regeneration, not a second issue.

**What merging this as-is would risk**: none directly, because CI fails
closed — a broken lockfile can't reach `pnpm install --frozen-lockfile` in
prod build either, so this PR cannot silently ship. The risk is only if
someone "fixes" it by deleting the overrides from `package.json` to match the
lockfile instead of regenerating the lockfile from `package.json` — that
would silently drop the Next.js CVE mitigations.

**Fix**: someone with write access needs to `git checkout` the branch,
run `pnpm install --no-frozen-lockfile` against the current `package.json`
(which still has the correct overrides), commit the regenerated
`pnpm-lock.yaml`, and push. That will also pull in whatever `next`
16.3.3 needs from its transitive tree cleanly. Re-run CI after. Not
something to do inside this read-only triage.

## Major-version migration notes

### #253 — jsonwebtoken 10 → 11

v11.0.0 breaking changes (from the crate's own CHANGELOG):
- `Validation::insecure_disable_signature_validation` removed → `dangerous::insecure_decode`
- `DecodingKey::as_bytes` / `try_get_hmac_secret` removed → `try_get_as_bytes`
- `EncodingKey::inner` renamed to `as_bytes`; `try_get_hmac_secret` removed
- `Algorithm`, `KeyAlgorithm`, `EllipticCurve`, `ThumbprintHash` are now `#[non_exhaustive]`
- `Header.extras` is now a struct instead of a raw map
- `Jwk::thumbprint()` now returns `Result<_>`

Every `jsonwebtoken::` call site in `hotel-backend/src/` was grepped
(`middleware/hfid_assertion.rs`, `middleware/cf_access.rs`,
`middleware/hk_access.rs` — the CF Access JWT verifier, the HF-ID badge
assertion verifier, and the `/hk` access verifier). All three use the same
narrow pattern: `Algorithm::RS256` only in equality checks
(`header.alg != Algorithm::RS256`) and constructors (`Validation::new(...)`,
`Header::new(...)`) — never an exhaustive `match` on `Algorithm`, so the
`non_exhaustive` change is inert here. None of the three call
`insecure_disable_signature_validation`, `DecodingKey::as_bytes`,
`try_get_hmac_secret`, `EncodingKey::inner`, `Header.extras`, or
`Jwk::thumbprint`. The `.as_bytes()` hits from the grep are all
`str::as_bytes()` on a test PEM constant, unrelated to the crate. Combined
with `test-backend`/`build-backend` already green on the PR branch, this is
merge-ready — but because this crate gates every card-login and Cloudflare
Access request in production, don't fire-and-forget it: watch `/health` and
auth error rates for the first few minutes after deploy.

### #252 — base64 0.22 → 0.23

Dependabot labels this "semver-minor" (Cargo's rule for pre-1.0 crates
treats the second component as "minor"), but under normal semver conventions
any `0.x` bump can be breaking, so it was checked as if major. Per the
crate's release notes, 0.23.0 adds SIMD-accelerated engines behind an
on-by-default `simd-unsafe` feature and bumps MSRV to 1.71.0; no changes are
documented to the `Engine` trait or `general_purpose::STANDARD`. Every
`base64::` call site (`render/thai_id_card.rs`, `routes/guest_documents.rs`)
uses only `base64::Engine` + `base64::engine::general_purpose::STANDARD.encode(...)`
— the same stable surface untouched by this release. `Cargo.lock` now carries
both `base64 0.22.1` (pulled transitively, likely via `jsonwebtoken`'s
`ed25519-dalek`) and `base64 0.23.0` (our direct dep) side by side, which
Cargo handles natively — not a conflict. `test-backend`/`build-backend` are
green.

### #295 — rust Docker base image 1.96-bookworm → 1.98-bookworm

`hotel-backend/Dockerfile`'s `chef`/`planner`/`builder` stages use
`rust:1.98-bookworm@<digest>`; the separate **runtime** stage stays on
`debian:bookworm-slim@<digest>` (unchanged by this PR — dynamically linked,
not distroless, but still glibc-based Debian **bookworm**, matching the
builder's own Debian release). Because both stages stay on the same Debian
codename, the glibc ABI the binaries link against in the builder is the one
present in the runtime image; only the Rust *toolchain* version moved, not
the OS base. No `rust-version`/MSRV pin exists in `hotel-backend/Cargo.toml`
(`edition = "2021"`, no `rust-version` key), so there's no MSRV floor this
bump could violate. `test-backend`/`build-backend` are green, and this PR's
base is the freshest of the whole batch (8 commits behind master). This one
rebuilds and relinks every production binary end-to-end (`hotel-backend`,
`writeback`, `sync`, every `backfill_*`, `migrate_legacy`, `create_user`) —
it's the highest-leverage single-line change in this batch even though the
diff itself is trivial.

### #249 — actions/setup-node 6 → 7

Release notes list "migrate to ESM and upgrade dependencies," add two new
outputs (`cache-primary-key`, `cache-matched-key`), and two bug fixes
(dummy `NODE_AUTH_TOKEN` export removed; `mirrorToken` only used when
provided) — no documented change to `node-version`/`cache` input handling.
Both call sites in this repo (`docker-build.yml`, `middleware-build.yml`) use
only `node-version: '20'` and `cache: 'pnpm'`/`'npm'`, none of the
changed/removed surface. Caveat: because this PR only touches workflow YAML,
the `changes` job's own path filter doesn't mark `frontend`/`backend` as
touched, so `build-frontend` (the job that actually runs the `Setup Node.js`
step) was **skipped**, not exercised, in this PR's own CI run — the action
bump is unverified by this specific PR, though it will be exercised for real
on the next PR that also touches frontend source, and workflow-only PRs
don't themselves redeploy prod (see the reminder above), so there's no prod
exposure window from merging this ahead of that verification.

### #241 — @types/node 25 → 26 (types-only)

Devtools-only; the Docker frontend image pins `node:20-alpine` in
`Dockerfile` (root, not `hotel-backend/Dockerfile`) and `package.json` has no
`engines` field, so nothing about the actual Node *runtime* version changed
— only the TypeScript ambient type definitions Dependabot bumped to Node
26's. `tsc` (via `build-frontend`/`test-frontend`) is already green on the
PR branch, so nothing in the app currently trips a type incompatibility. Not
a blocker, but worth a follow-up ticket at some point: the type definitions
now target a Node major three versions ahead of what actually ships
(`node:20-alpine`), which could eventually let a Node-26-only API get used
in app code and type-check locally while failing at runtime on the pinned
Node 20 image. Not something to fix in this triage.

## Ordered merge-ready list (safest first)

CI-only / no-prod-deploy-trigger first, then frontend dev-only, then frontend
runtime, then backend runtime (lockfile-only patches/minors before the ones
that touch `Cargo.toml` or the Dockerfile), release-please last:

1. **#245** `dorny/paths-filter` 4.0.1→4.0.2 (CI-only, patch)
2. **#246** `softprops/action-gh-release` 3.0.1→3.0.2 (CI-only, patch, manual-workflow-only)
3. **#255** `actions/checkout` 7.0.0→7.0.1 (CI-only, patch)
4. **#250** `docker/login-action` 4.4.0→4.5.1 (CI-only, minor)
5. **#249** `actions/setup-node` 6.4.0→7.0.0 (CI-only, major but no breaking-surface hit)
6. **#242** `tailwindcss` 4.3.1→4.3.3 (frontend dev-only, patch)
7. **#244** `eslint-config-next` 16.2.6→16.2.12 (frontend dev-only, patch)
8. **#241** `@types/node` 25.9.3→26.1.2 (frontend dev-only, major but types-only)
9. **#240** `lucide-react` 1.18.0→1.27.0 (frontend runtime, minor, icon set)
10. **#243** `recharts` 3.9.1→3.9.2 (frontend runtime, patch)
11. **#248** `uuid` 1.23.4→1.24.0 (backend runtime, minor, lockfile-only)
12. **#254** `async-trait` 0.1.89→0.1.91 (backend runtime, patch, lockfile-only)
13. **#251** `tokio` 1.52.3→1.53.1 (backend runtime, minor — async runtime under every binary; **redeploys prod**)
14. **#252** `base64` 0.22.1→0.23.0 (backend runtime, pre-1.0 bump, call sites verified clean; **redeploys prod**)
15. **#253** `jsonwebtoken` 10→11 (backend runtime, major, breaking surface verified clean of our 3 call sites; **redeploys prod, gates auth**)
16. **#295** `rust` 1.96→1.98-bookworm (Docker build toolchain; **rebuilds every production binary + redeploys prod**)
17. **#256** release-please → 2.76.0 (last, always — packages up everything already on master; **redeploys prod**)

Not on the merge-ready list — need a human decision:

- **#297** (npm group: `next` 16.2.12→16.3.3 + `js-yaml`): CI is actively
  failing (`ERR_PNPM_LOCKFILE_CONFIG_MISMATCH`) because Dependabot's lockfile
  regeneration dropped the pnpm CVE-override block. Needs someone to
  regenerate `pnpm-lock.yaml` with `pnpm install --no-frozen-lockfile`
  against the current (correct) `package.json` and re-push, or close in favor
  of a fresh Dependabot run.
- **#256** (release-please 2.76.0): every workflow run on this PR has been
  sitting at `conclusion: action_required` since 2026-09-10 — the run never
  started, so there is **no CI evidence** for this PR at all, despite its
  base being only 1 commit behind master. Someone with Actions-approval
  rights needs to approve/re-run the pending workflow before this has a
  green check to merge on. Also note: release-please PRs are the versioning
  mechanism for this whole repo (CLAUDE.md §Versioning) and always land last
  and always redeploy prod on merge — normal, but worth doing deliberately
  after the rest of the batch, not reflexively.
