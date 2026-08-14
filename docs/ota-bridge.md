# OTA booking bridge (`/api/ota/*`)

**Status: implemented, SHIPPED DARK.** `OTA_BRIDGE_ENABLED` defaults `false`, so
every `/api/ota/*` request answers **503** until an operator flips it. Nothing on
this page changes the behaviour of `/api/*`, of the desk UI, or of the legacy
writeback path.

Counterpart repo: **`ota-desk`** (`~/HF/ota-desk`). The interface contract below
is joint — do not change a path, a variable name, or a field name on one side
only.

## Why this exists

`ota-desk` creates PMS bookings today by calling five ordinary `/api/*` routes
over the `hotel-network` docker bridge carrying **no credential of any kind**
(`ota-desk/lib/pms.ts` sends only `content-type`). The backend publishes no host
port and sits behind no proxy, so the blast radius is "any container on that
bridge", but that is still an unauthenticated booking-create surface.

The naive fix does not work: the PMS **browser UI** reaches those same routes
through the Next.js rewrite (`next.config.js`, `/api/:path* →
${BACKEND_URL}/api/:path*`). Requiring a bearer on `/api/*` would break the
entire front desk.

So this ships a **parallel machine surface** instead: the same handler functions,
re-mounted under a second prefix that IS bearer-gated. The desk keeps `/api/*`;
machines move to `/api/ota/*`.

It also closes a second coupling. `ota-desk`'s reconciler currently opens its own
`pg` pool against the PMS PostgreSQL as `mcp_ro` and runs a hand-written query
over `ht_bookings` / `ht_customers` — a second repo holding a copy of our schema.
`GET /api/ota/reconcile/bookings` replaces that read with an API call.

## Surface

| Method + path | Handler | Change |
|---|---|---|
| `GET /api/ota/customers/search?branch=&search=` | `routes::new_customers::list_customers` | re-mount, unchanged |
| `POST /api/ota/customers?branch=` | `routes::new_customers::create_customer` | re-mount, unchanged |
| `GET /api/ota/rooms?branch=&limit=` | `routes::new_rooms::list_rooms` | re-mount, unchanged |
| `POST /api/ota/bookings/validate` | `routes::new_bookings::validate_booking` | re-mount, unchanged |
| `POST /api/ota/bookings?branch=` | `routes::new_bookings::create_booking` | re-mount, unchanged |
| `GET /api/ota/reconcile/bookings` | `routes::ota::reconcile_bookings` | **NEW** |

The first five re-mount **existing handler functions**. No handler, DTO, query
struct or repository method was altered: each takes only `State` + `Query`/`Json`
and never reads its own path, so **a request that differs only in prefix produces
a byte-identical response**. That equality is the contract — any drift silently
breaks `ota-desk/lib/pms.ts`.

Deliberately **not** mounted: cancel / modify (`PUT /api/bookings/{id}[/cancel]`).
`ota-desk` has no client code for them; they belong to writeback Phase 3 and get
added when that ships. Minimal surface = the actual need.

## Layering

Outermost first: **`require_ota_token` → `ville_write_guard` → routes.** Built by
`routes::ota::router_with_config`, which the integration tests mount directly so
the tested wiring cannot drift from `main.rs`.

- **`require_ota_token` outermost** — an unauthenticated probe is rejected before
  it can touch state or any other gate.
- **`ville_write_guard` is mandatory, not optional.** It is present on the desk
  mounting and keys on `?branch=` + method, exactly what `ota-desk` sends.
  Omitting it would silently punch a HF Ville write hole around ADR 0002's
  admission gate. `is_ville_exempt_path` matches only the maid cleaning route, so
  `/api/ota/*` gets no exemption. Pinned by
  `tests/test_ota_bridge.rs::ville_booking_write_is_refused_by_the_layered_guard`,
  which asserts **403** (guard refused) and specifically not 503 (gate answered
  first) or 2xx/4xx from the handler (guard bypassed).
- **`require_auth` is deliberately absent** — the caller is a service with no
  cookie session, identical to `/api/channel/*`. Note the consequence: the OTA
  token becomes load-bearing the moment `AUTH_ENABLED` is flipped true.
- `AUTHORIZATION` is deliberately **not** in the CORS `allow_headers` list, so a
  browser cannot present a bearer here at all. Free structural guarantee that
  this stays machine-only.

## The gate: three-state, permissive-then-enforce

`hotel-backend/src/middleware/ota_token.rs`. Structurally a clone of
`channel_token.rs` (`extract_bearer` / `constant_time_eq` are **copied**, not
refactored out of it — a change here must never move the loyalty channel's auth)
plus one extra axis, because unlike the loyalty channel this replaces an
*already-live* unauthenticated caller.

| `enabled` | `enforce` | accepted tokens | `Authorization` | outcome | HTTP |
|---|---|---|---|---|---|
| false | * | * | * | `Disabled` | **503** |
| true | true | empty | * | `Disabled` | **503** (misconfigured → fail closed) |
| true | true | non-empty | absent / non-Bearer | `Unauthorized` | **401** |
| true | * | any | Bearer, no match | `Unauthorized` | **401** |
| true | false | any | absent / non-Bearer | `AllowedUnauthenticated` | **200** + WARN |
| true | * | non-empty | matches primary | `Allowed` | **200** |
| true | * | non-empty | matches previous | `AllowedRotating` | **200** + WARN |

**The load-bearing invariant: a *presented but wrong* credential is 401 in every
mode.** Only a completely absent `Authorization` header rides the permissive
lane. That is what makes the enforce flip provably safe — once the
"accepted WITHOUT a bearer token" WARN count is zero, enforcing cannot break a
caller — and it means a botched rotation can never be masked as success.

All seven rows are unit-tested at the pure decision function
(`check_ota_access`); the router-level behaviour is in `tests/test_ota_bridge.rs`.

Log lines (never a token, a header, or a body):

```
WARN ota_bridge: ota bridge request accepted WITHOUT a bearer token (OTA_BRIDGE_ENFORCE=false) path=/api/ota/rooms
WARN ota_bridge: ota bridge authenticated with the PREVIOUS token — finish the rotation
```

Startup line (INFO normally; **WARN** with the suffix `— PERMISSIVE,
unauthenticated calls are accepted` while `enabled && !enforce`):

```
OTA bridge: enabled=true enforce=false token=ab12cd previous=unset
```

`token=` / `previous=` are the first 6 hex of `sha256(value)`, or `unset`. That
fingerprint is how an operator confirms both repos hold the same secret **without
either side printing it**.

## Config

| Env var | Purpose | Default | Lives in |
|---|---|---|---|
| `OTA_BRIDGE_ENABLED` | Master switch for the whole surface | **off** — everything 503 | `docker-compose.yml` |
| `OTA_BRIDGE_ENFORCE` | Require the bearer (vs. permissive) | **off** — un-credentialed calls served + WARN | `docker-compose.yml` |
| `OTA_BRIDGE_TOKEN` | Shared bearer | unset | `/run/secrets/ota_bridge_token` |
| `OTA_BRIDGE_TOKEN_PREVIOUS` | Rotation slot; accepted with a WARN | unset | `/run/secrets/ota_bridge_token_previous` |

> **INVARIANT — `OTA_BRIDGE_TOKEN` (new-hotel) and `PMS_BRIDGE_TOKEN` (ota-desk)
> MUST hold the identical string.** Two names for one value, following the
> `PORTAL_NOTIFY_TOKEN` ≡ portal `NOTIFY_INGRESS_TOKEN` idiom. Generate with
> `openssl rand -hex 32`. Verify by comparing the `sha256(token)[0..6]`
> fingerprint printed by this repo's startup line and by ota-desk's
> `pnpm pms:ping`.

**Flags are compose-owned (ADR 0004).** Both flags are deliberately **absent**
from `.github/workflows/docker-build.yml`; the ADR-0004 NOTE there names them.
`run-deploy.sh` materialises `.env` wholesale from the workflow payload, so a
`vars.X || 'false'` entry would make the compose default unreachable dead code
and silently revert the authentication posture on the next deploy. Flipping
either flag is a one-line reviewable diff to `docker-compose.yml`, which is in
the workflow's `deploy` paths filter and therefore triggers its own deploy —
exactly the property wanted for an auth-enforcement change.

*(Other ship-dark flags — `LAYOUT_WRITEBACK_ENABLED`, `BOOKING_VALIDATION_ENABLED`,
… — still ride `vars.X || 'false'` in the workflow. ADR 0004 is the later ratified
decision and those were simply never migrated. Do not "harmonise" these two into
the workflow.)*

**Tokens are compose secrets.** GH secret → workflow `env:` → `jq --arg` →
payload `.secrets{}` → `run-deploy.sh` writes `/home/deploy/secrets/<key>` mode
0444 **before** `docker compose up` → compose top-level `secrets:` bind-mounts
`/run/secrets/<key>` → `secrets.rs` hydrator populates the env var (env-var-first;
**empty file == missing**).

That last detail is why the rollout is split the way it is: `run-deploy.sh` writes
a file for **every** key present in `.secrets`, *including empty values*. With the
GH secret unset the file exists but is empty, the hydrator skips it, and the gate
fails closed. A compose `secrets:` entry pointing at a **missing** file, by
contrast, aborts the **entire stack start**. So the payload key must land and
deploy green one commit **before** the compose `secrets:` declaration.

## `GET /api/ota/reconcile/bookings`

```
?branch=hfhotel|hfville      required; `all` → 400
&checkin_from=YYYY-MM-DD     required
&checkin_to=YYYY-MM-DD       required, >= checkin_from else 400
&limit=1..1000               optional, default 500, out-of-range → 400
&cursor=<bookNo>             optional keyset cursor (exclusive lower bound)
```

```json
{ "success": true,
  "data": [ { "bookNo":   "B260814001",
              "checkIn":  "2026-08-14",
              "checkOut": "2026-08-16",
              "guestName":"Somchai Jaidee",
              "notes":    "จ่ายแล้ว Agoda " } ],
  "nextCursor": "B260814001" }
```

Status codes: `200` ok · `400` bad params · `401` bad/missing bearer (per the
matrix) · `503` surface disabled or misconfigured · `500` when `branch=hfville`
and the Ville pool is absent (`state.ville_pool()?`).

**All five row fields are non-nullable strings.** Three properties are
load-bearing and must not be "cleaned up":

1. **`checkIn` / `checkOut` are exactly `YYYY-MM-DD`**, produced by `::text` on a
   `DATE` column — never a `NaiveDate`/`NaiveDateTime` serde field. The client
   compares them to its own date strings; a datetime would render
   `"2026-08-14T00:00:00"` and take the matcher to zero matches **silently**
   (rows just stay `unmatched`, no error anywhere). This is the single
   highest-value assertion in the suite.
2. **`coalesce(book_status,'') <> 'cancelled'`** — a NULL status is *included*.
   Do not rewrite as `IS DISTINCT FROM` + NOT NULL, nor as a status whitelist.
3. **`guestName` and `notes` are composed in SQL**, one expression in one place,
   so the two repos cannot drift. A nameless guest (or a LEFT JOIN miss) yields
   `""` via `trim(' ')`, never null. `notes` keeps its interior single space and
   its possible **trailing** space — the client substring-searches this value.

`nextCursor` is the last row's `bookNo` when the page is FULL
(`data.length === limit`), otherwise `null`.

Every clause is copied verbatim from the `mcp_ro` query it replaces
(`ota-desk/ingest/reconcile.ts`). Pool selection mirrors `list_bookings`
(`Hfhotel → new_pool`, `Hfville → ville_pool()`); `all` is refused because
reconciliation is per-property and a union would break the client's per-property
`consumed` bookkeeping.

Pagination is **keyset** on `book_no` (`VARCHAR(20) NOT NULL UNIQUE` — a total
order), so pages can neither duplicate nor skip within a snapshot; the date
predicate rides `ix_ht_bookings_checkin`. There is deliberately **no cap on the
date span**: silently clamping would produce false `unmatched` rows, and rejecting
a legitimately wide window would stall reconciliation. Volume is bounded by
pagination.

The existing `GET /api/bookings` cannot serve this — its date filter is
stay-*overlap* not check-in-*between*, its DTO has no `book_special_requests`, its
status filter is exact-match, and it serialises `check_in` as a datetime.

All SQL here is **runtime `sqlx::query`** (same policy as `routes::hk`), so this
module needs no `.sqlx/` regeneration and adds no cache-drift CI gate.

## Rollout — flip points

Every step is independently deployable and independently revertible. `NX` =
new-hotel deploy, `OX` = ota-desk deploy. **Never run an `NX` and an `OX` first
deploy simultaneously.**

| # | What | Result |
|---|---|---|
| **N1** | `docker-build.yml` payload keys `.secrets.ota_bridge_token{,_previous}` + backend code + compose `environment:` flags at `:-false` + tests + this doc | `/api/ota/*` is not yet mounted against any secret file. Nothing reads the token. |
| **N2** | compose `secrets:` declaration + backend `secrets:` mount | `/api/ota/*` answers **503** to everything. `/api/bookings` and the `mcp_ro` read are untouched. |
| **N3** | one-line compose diff `OTA_BRIDGE_ENABLED:-true` | **Flip point 1.** Surface answers 200, with a permissive WARN per request. |
| **O1** | ota-desk `lib/pms.ts` + `lib/pms-bookings.ts` + probe, shipped with `PMS_BRIDGE_TOKEN` **unset** and `PMS_RECONCILE_SOURCE=db` | Provable no-op: prefix stays `/api`, no header, DB reconcile. |
| **O2** | operator sets GH secret `PMS_BRIDGE_TOKEN` = the same value as `OTA_BRIDGE_TOKEN`; redeploy | **Flip point 2.** ota-desk moves to `/api/ota` + bearer. |
| **O3** | repo variable `PMS_RECONCILE_SOURCE=dual`; soak **≥48 h** | **Flip point 3.** Exit criterion: zero `DIFF` lines at both properties. |
| **O4** | `PMS_RECONCILE_SOURCE=api` | **Flip point 4.** The direct DB read is now unused. |
| **N4** | one-line compose diff `OTA_BRIDGE_ENFORCE:-true` | **Flip point 5.** Unauthenticated `/api/ota/*` is now 401. |

Ordering constraints that are **not** negotiable:

1. **N1 before N2.** A compose `secrets:` entry pointing at a file the deploy has
   never written aborts the whole stack start.
2. **N2 + N3 live before O2**, or ota-desk points at a 503 surface.
3. **O2 before O3.** `fetchPmsBookings` asserts this and throws a named error
   otherwise — the reconcile endpoint exists only under the token-gated prefix.
4. **N4 last**, and only after ≥24 h of zero "accepted WITHOUT a bearer token"
   WARNs following flip point 4.

**Setting the GH secret `OTA_BRIDGE_TOKEN` is a separate operator step from
shipping N1/N2** and is not required for either to deploy green: an unset secret
yields an empty file, which the config treats as absent, and the surface stays
503.

### Verifying without host ports

The backend publishes no host port. The probe runs from the only other container
on `hotel-network`:

```
docker exec ota-desk-ingest node -e \
  "fetch('http://backend:3003/api/ota/rooms?branch=hfhotel&limit=1').then(r=>console.log(r.status))"
```

- after **N2** → `503`
- after **N3** → `200`, plus one permissive WARN in the backend log
- after **N4** → `401` (and `pnpm pms:ping`, which sends the bearer, still `200`)

After N1 only, confirm the secret file exists from the deploy log's `wc -c` line
(`run-deploy.sh`) — the value is never printed. `0` bytes = the GH secret is not
set yet, which is the expected state until the operator sets it.

## Rotation

1. Set `OTA_BRIDGE_TOKEN_PREVIOUS` to the current value, deploy. Both are now
   accepted; the old one logs "finish the rotation".
2. Set `OTA_BRIDGE_TOKEN` to the new value on new-hotel and `PMS_BRIDGE_TOKEN` to
   the same new value on ota-desk; deploy both.
3. Confirm the "finish the rotation" WARNs have stopped and the two `sha256[0..6]`
   fingerprints match, then clear `OTA_BRIDGE_TOKEN_PREVIOUS` and deploy.

A mismatch during rotation surfaces as a **401**, never as a silent success —
see the mode matrix.

## Out of scope

`PMS_WRITEBACK_ENABLED` (ota-desk) stays `false` throughout. Nothing here flips
it, and nothing here depends on it being flipped — which is why E's cutover is
verified by an explicit probe rather than by observing traffic.

Retiring the direct `mcp_ro` read (dropping `PMS_HF_DB_URL` /
`PMS_HFVILLE_DB_URL`, revoking the grants) is a **separate task**, and the grant
revoke must wait on confirming the role is not shared with the HF Hotel Data MCP
server.
