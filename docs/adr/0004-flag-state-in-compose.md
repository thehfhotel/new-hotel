# ADR 0004 — Operational flag state lives in `docker-compose.yml`, not GitHub variables

**Status:** Accepted — 2026-07-27. Ratified by the project owner.
**Scope:** the reconcile / CT-watermark operational booleans
(`RECONCILE_FORCE_CONVERGE_ENABLED`, `RECONCILE_REINGEST_MISSING_PG_ENABLED`,
`WORKER_RECONCILE_ENABLED`, `SYNC_PER_TABLE_WATERMARK`, and their `HFVILLE_`
siblings). It does **not** cover secrets, which stay in
`/home/deploy/secrets/` (see CLAUDE.md), nor genuinely per-environment values.

## Context

Operational feature flags were configured as GitHub repo variables, injected by
`.github/workflows/docker-build.yml` as `${{ vars.X || 'false' }}` into a deploy
payload, materialised into `.env` by `scripts/deploy/run-deploy.sh`, and read by
`docker-compose.yml` as `${X:-false}`.

Two properties of that chain combined badly:

1. **`run-deploy.sh` materialises `.env` wholesale** from the payload —
   `jq -r '.env | to_entries[] | "\(.key)=\(.value | @sh)"' > .env`. Every key in
   the payload becomes a line in `.env`.
2. **The workflow always shipped a value** (`vars.X || 'false'`), never an
   absence.

Therefore `${X:-false}` in compose **never fell back to its default**. The
compose defaults were unreachable dead code, and the file that looks like the
configuration was not the configuration.

The consequences were not hypothetical:

- On 2026-07-27, `gh variable list` showed `RECONCILE_FORCE_CONVERGE_ENABLED=true`
  on **both** sites while every file in the repo read `false`. The repo did not
  merely omit production state — it asserted the opposite.
- Three separate subagents working on the same change had to be explicitly
  warned not to trust the compose defaults. A new canonical-write path was
  nearly gated on that flag, which would have shipped it live on both sites at
  once, with no canary, on deploy.
- `HFVILLE_WORKER_RECONCILE_ENABLED` was present in compose but in **no**
  workflow file. Because `.env` is rewritten wholesale on every deploy, any
  host-set value was discarded. The knob was un-flippable in practice and only
  worked because its compose default happened to be `true`.

The decisive observation: **a GitHub variable change requires a deploy to take
effect anyway**, because a container only reads its environment at start. The
supposed benefit — "flip without a commit" — was illusory. We paid for a deploy
either way and received invisible state in exchange.

## Decision

Operational flag state is **committed to `docker-compose.yml` defaults**. These
keys are deliberately **absent** from `docker-build.yml`, so nothing injects
them into `.env` and the compose default becomes the effective production value.

Flipping a flag is editing its `:-false` to `:-true`. `docker-compose.yml` is in
the workflow's `deploy` paths filter, so the edit triggers its own deploy — the
change and its delivery mechanism are the same artifact.

The `${VAR:-default}` interpolation form is kept (rather than a bare literal) so
local development can still override via a local `.env`.

Per-site divergence is preserved by keeping distinct `HFVILLE_*` variable names
in the interpolation, so HF Ville can canary a flag independently of HF Hotel.
That property is load-bearing and predates this ADR.

## Consequences

**Gained.** The repo tells the truth. A flag flip is a reviewable one-line diff,
dated in git history — `git log -S RECONCILE_REINGEST_MISSING_PG_ENABLED` now
answers "when did this go live?", a question we could not answer for
force-converge except by reading a GitHub variable's mtime.

**Given up.** Out-of-band emergency flips. Every flag change goes through CI,
so reverting a bad flag takes a deploy cycle (~10 min) rather than being
flippable the instant someone notices. Accepted because the pipeline is already
the only sanctioned path to production (CLAUDE.md Deployment Policy).

**Migration.** The live values at the time of this ADR were transcribed into the
compose defaults in the same commit that removed the workflow keys — verified by
diffing `docker inspect` across `backend`, `sync`, and `sync-hfville` before and
after. Transcribing was not optional: without it, force-converge would have
silently turned **off** on both sites. The now-inert GitHub variables were
deleted afterwards so nothing looks authoritative when it isn't.

## The trap this creates, and the tripwire for it

A future reader will see these flags in `docker-compose.yml` with no
corresponding entry in `docker-build.yml`, reasonably conclude the plumbing was
forgotten, and "fix" it.

**That single act silently reverts every committed flag to `false` on the next
deploy** — the workflow would resume injecting `vars.X || 'false'`, and the
compose defaults would go dead again. It is a plausible-looking change with an
invisible, production-altering effect. It would, among other things, take HF
Ville's reconcile backstop dark.

Guards in place:

- A boxed comment above the flag block in `docker-compose.yml` pointing here.
- A `NOTE (ADR 0004)` comment at the exact spot in `docker-build.yml` where the
  keys would be re-added.

Neither alone is sufficient — the ADR carries the reasoning, the comments are
the tripwire at the point of danger.

## Alternatives considered

**Keep GitHub variables, add observability** (a startup log line and a `/health`
field exposing each flag's live value). Rejected: it makes state *discoverable*
after the fact, but the repo still lies to anyone reading it, and a reviewable
diff beats after-the-fact observability.

**Keep GitHub variables as source of truth, add a CI drift check** that fails
when a variable diverges from the compose default. Rejected: it forces the two
back into agreement at the cost of a check needing `gh` API access from CI —
more machinery to achieve what committing the value achieves directly.

## References

- `docs/coexistence/sync-incident-log.md` → 2026-07-27 (the incident that
  surfaced this).
- `scripts/deploy/run-deploy.sh` — the `.env` materialisation.
- ADR 0002 — indefinite coexistence; these flags are permanent operational
  surface, not transition-period scaffolding, which is what makes their
  legibility worth an ADR.
