# Dependabot security alerts — open as of 2026-09-11

Read-only triage of the 12 open Dependabot alerts on `thehfhotel/new-hotel`
(`gh api repos/thehfhotel/new-hotel/dependabot/alerts?state=open --paginate`):
4 critical, 6 high, 2 moderate. All 12 are `npm` ecosystem (no open Rust/Cargo
alerts at this time).

Method: alert data from the API above; direct-vs-transitive and
production-vs-dev determined by grepping `package.json` (`dependencies` /
`devDependencies` / `pnpm.overrides`) and confirmed with `pnpm why <pkg>`
against the installed tree; "fixing PR" determined by diffing every open
Dependabot PR (`gh pr diff <n>`) for a version bump that lands at or above
the patched version.

## Table

| Alert | Package | Ecosystem | Severity | Vulnerable range | Patched | Direct/Transitive | Runtime/Dev | Brought in by | Fixing PR | Manual fix |
|---|---|---|---|---|---|---|---|---|---|---|
| [#182](https://github.com/thehfhotel/new-hotel/security/dependabot/182) | `next` | npm | critical | `>=16.0.0 <16.3.3` | 16.3.3 | **Direct** (`package.json` dependencies) | runtime | — | #297 (**broken**, see below) | Bump `next` `16.2.12` → `16.3.3` in `package.json` |
| [#181](https://github.com/thehfhotel/new-hotel/security/dependabot/181) | `next` | npm | critical | `>=16.0.0 <16.3.3` | 16.3.3 | **Direct** | runtime | — | #297 (**broken**) | Same as above |
| [#179](https://github.com/thehfhotel/new-hotel/security/dependabot/179) | `next` | npm | critical | `>=16.0.0 <16.3.3` | 16.3.3 | **Direct** | runtime | — | #297 (**broken**) | Same as above |
| [#178](https://github.com/thehfhotel/new-hotel/security/dependabot/178) | `next` | npm | critical | `>=16.0.0 <16.3.3` | 16.3.3 | **Direct** | runtime | — | #297 (**broken**) | Same as above |
| [#184](https://github.com/thehfhotel/new-hotel/security/dependabot/184) | `sharp` | npm | high | `< 0.35.4` | 0.35.4 | Transitive, pinned via `pnpm.overrides` (`sharp: >=0.35.0`) — pulled by `next` (image optimization) | runtime | `next@16.2.12 → sharp@0.35.3` | none | Tighten override `sharp: ">=0.35.0"` → `">=0.35.4"` |
| [#183](https://github.com/thehfhotel/new-hotel/security/dependabot/183) | `js-yaml` | npm | high | `>=4.0.0 <4.3.2` | 4.3.2 | Transitive, pinned via `pnpm.overrides` (`js-yaml: ^4.3.0`) — pulled by `eslint` → `@eslint/eslintrc` | development | `eslint@9.39.2 / eslint-config-next@16.2.6 → js-yaml@4.3.0` | none | Bump override `js-yaml: "^4.3.0"` → `"^4.3.2"` |
| [#176](https://github.com/thehfhotel/new-hotel/security/dependabot/176) | `browserslist` | npm | high | `<= 4.28.6` | 4.28.7 | Transitive, **no existing override** — pulled by `@babel/core` → `@babel/helper-compilation-targets` (via `eslint-config-next` and `jest` dev chains) | runtime (per GH scope) | `eslint-config-next@16.2.6` / `jest@30.4.2 → browserslist@4.28.1` | none | Add override `browserslist: "^4.28.7"` |
| [#175](https://github.com/thehfhotel/new-hotel/security/dependabot/175) | `browserslist` | npm | high | `<= 4.28.6` | 4.28.7 | Same as #176 | runtime (per GH scope) | Same as #176 | none | Same as #176 |
| [#174](https://github.com/thehfhotel/new-hotel/security/dependabot/174) | `nanoid` | npm | high | `< 3.3.18` | 3.3.18 | Transitive, **no existing override** — pulled by `next` → `postcss@8.5.23` (also by `@tailwindcss/postcss` dev) | runtime | `next@16.2.12 / postcss@8.5.23 → nanoid@3.3.16` | none | Add override `nanoid: "^3.3.18"` |
| [#172](https://github.com/thehfhotel/new-hotel/security/dependabot/172) | `js-yaml` | npm | high | `>=4.0.0 <4.3.1` | 4.3.1 | Same package/path as #183 (lower patched-version floor; superseded by the #183 fix) | development | Same as #183 | none | Same fix as #183 (`^4.3.2` satisfies both floors) |
| [#180](https://github.com/thehfhotel/new-hotel/security/dependabot/180) | `baseline-browser-mapping` | npm | moderate | `>=2.0.0 <2.11.0` | 2.11.0 | Transitive, **no existing override** — pulled directly by `next` (declares it as its own dependency) and via `browserslist` | runtime | `next@16.2.12 → baseline-browser-mapping@2.10.42`; `browserslist@4.28.1 → baseline-browser-mapping@2.10.37` | none | Add override `baseline-browser-mapping: "^2.11.0"` (not required by this batch's scope — moderate; left for a human, see report). **Note**: the `chore/next-16.3-and-alerts` PR's `next`+`browserslist` bumps happen to pull a second, already-patched `baseline-browser-mapping@2.11.22` in alongside the still-vulnerable `2.10.42` (the copy `next@16.3.3` itself pins directly) — pnpm resolves both side by side, so this alert is **not fully closed** by that PR without also adding the override |
| [#177](https://github.com/thehfhotel/new-hotel/security/dependabot/177) | `@humanfs/node` | npm | moderate | `< 0.16.8` | 0.16.8 | Transitive, **no existing override** — pulled by `eslint` directly | development | `eslint@9.39.2 → @humanfs/node@0.16.7` | none | Add override `"@humanfs/node": "^0.16.8"` (not required by this batch's scope — moderate; left for a human) |

## Alert → PR cross-reference

Of the 17 open Dependabot/PRs (`#240`–`#255` minus closed `#247`, plus `#295`,
`#297`), **only #297** ("Bump the npm_and_yarn group across 1 directory with
2 updates": `next` 16.2.12→16.3.3 plus a transitive `js-yaml` line) touches
any package on this alert list, and it is currently broken:

- Dependabot's lockfile regeneration for #297 **deleted the entire top-level
  `overrides:` block from `pnpm-lock.yaml`** while leaving `package.json`'s
  `pnpm.overrides` (the CVE-remediation pins from `f0a5fc1`) untouched. That
  mismatch fails `pnpm install --frozen-lockfile` with
  `ERR_PNPM_LOCKFILE_CONFIG_MISMATCH` in CI (`test-frontend`).
- The same regeneration also **re-introduces a dangling `js-yaml@3.15.2`**
  resolution (alongside the still-present `js-yaml@4.3.0`) because the
  `js-yaml: ^4.3.0` override that was forcing everything onto the 4.x line
  is gone from the lockfile. This does not fix alerts #183/#172 — `4.3.0` is
  still `< 4.3.2` — and could regress further if merged as-is.
- No open PR bumps `sharp`, `browserslist`, `nanoid`,
  `baseline-browser-mapping`, or `@humanfs/node` — all five need a manual
  `pnpm.overrides` entry (added or tightened) since none of them are direct
  dependencies Dependabot tracks for a version-bump PR.

## Minimal manual fixes (summary)

| Fix | Change |
|---|---|
| `next` (closes #178, #179, #181, #182 — 4 critical) | `package.json` `dependencies.next`: `16.2.12` → `16.3.3` |
| `js-yaml` (closes #172, #183 — 2 high) | `pnpm.overrides.js-yaml`: `^4.3.0` → `^4.3.2` |
| `sharp` (closes #184 — 1 high) | `pnpm.overrides.sharp`: `>=0.35.0` → `>=0.35.4` |
| `browserslist` (closes #175, #176 — 2 high) | add `pnpm.overrides.browserslist`: `^4.28.7` |
| `nanoid` (closes #174 — 1 high) | add `pnpm.overrides.nanoid`: `^3.3.18` |
| `baseline-browser-mapping` (closes #180 — 1 moderate) | add `pnpm.overrides["baseline-browser-mapping"]`: `^2.11.0` — optional, not applied in this batch |
| `@humanfs/node` (closes #177 — 1 moderate) | add `pnpm.overrides["@humanfs/node"]`: `^0.16.8` — optional, not applied in this batch |

All seven fixes are patch/minor bumps of transitive or pinned deps except
`next`, which is a minor bump on its own major line (`16.2` → `16.3`) — none
require a major upgrade. See the companion PR
(`chore/next-16.3-and-alerts`) for the 5 critical/high fixes actually
applied (the 2 moderate ones are left open for a human decision, since they
were out of this batch's required scope).
