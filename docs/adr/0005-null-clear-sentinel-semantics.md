# ADR 0005 — NULL-clear sentinel semantics for guarded reconcile-gate terms

**Status:** Proposed (design only — NOT implemented). Written 2026-07-31 in response to
issue #269 / plan-item T4, which is explicitly flagged "do NOT attempt as a side task."
This document makes it safe to attempt later; it does not attempt it.
**Scope:** `hotel-backend/src/sync/gate_guard.rs` and the `guarded: true` gate terms in
`sync/mappers/booking.rs`, `sync/mappers/checkin.rs`, plus the related (but structurally
different) residuals in `sync/mappers/room.rs` and `sync/mappers/customer.rs`.
**Does not change:** no hash bytes, no gate logic, no `.sqlx/` cache, no migration. This
ADR is the plan; implementation is separately-scoped future work (§6).

## Context

`gate_guard.rs` enforces one mechanical invariant across every CT mapper: **every
reconcile-hash input must be named by a gate term that actually compares it** (the
`every_hash_input_names_a_gate_compared_field` test, `gate_guard.rs:353-408`). Violating
it is the "d09e756 mechanism" — a legacy edit that touches only an ungated-but-hashed
field is idempotency-skipped, the CT watermark advances past it, the delta ages out of
the 2-day retention window, and the reconcile sweep flags a row `force_converge_reconcile_row`
can never close because it just re-drives the same blind gate.

A `GateField::guarded: true` term (`gate_guard.rs:104-124`) is a deliberate, narrower
carve-out inside that invariant: a Some-only comparator that mirrors a
`COALESCE($n, existing)` write. It exists because the *naive* fix for the d09e756 class —
"just compare every hashed field, always" — creates a worse failure for a denormalised
FK-like pointer: if the write itself uses `COALESCE` (kept deliberately, to avoid a
different incident — "Bug A", `booking.rs:1022-1034` — where a non-COALESCE write let a
transient/incomplete projection blank a populated pointer), then an ungrudging gate would
flag a mismatch on every tick a `None` is ever projected, the mapper would re-apply, the
COALESCE write would still refuse to actually change anything, and the loop repeats
forever. `guarded: true` breaks that loop by treating `None` as "nothing to check" — at
the cost, stated explicitly in the doc comment, of never being able to detect a genuine
Some→None legacy transition (`gate_guard.rs:110-114`).

Issue #269 is that stated cost coming due: `docs/legacy-app/` + live evidence below show
it is a real, if narrow, gap — and the obvious fix (widen representational granularity by
introducing a tri-state) touches the one piece of machinery in this codebase pinned
byte-for-byte across two production databases. This ADR designs the representation *and*
the migration path, and gives an honest recommendation on whether to build it.

## 1. What `gate_guard.rs` pins, precisely

- **`GateField<E, P>`** (`gate_guard.rs:100-124`) — a named `matches: fn(&E, &P) -> bool`.
  The production gate is `FIELDS.iter().all(|f| (f.matches)(ex, p))`; there is no name
  list independent of behaviour to edit around.
- **`HashInput<S>`** (`gate_guard.rs:160-228`) — a named `segment: fn(&S) -> String` plus
  `gated_by: &[&str]` naming the gate term(s) that make this input's movement visible.
  `segment` must be byte-identical to what the *production* hash function
  (`scheduler::sync::{customer,booking,checkin}_canonical_hash`) receives for the same
  field — enforced by each entity's `*_hash_bytes_unchanged_for_golden_inputs` test
  (`booking.rs:1954`, `checkin.rs:3229`, `customer.rs:1656`), which asserts the function's
  output against a **literal SHA256 of a hand-written pipe-joined string** — e.g.
  `sha256("R014810|2026-04-25|2026-04-26|C21610")` at `booking.rs:1963`. That literal is
  the actual pin; the descriptor table is proved to reproduce it.
- **`join_hash_segments`** (`gate_guard.rs:84-86`, pinned by
  `join_hash_segments_is_pipe_separated`, `gate_guard.rs:613-622`) — the single `|`
  separator every hash body is built from.
- **Byte-compat note** (`gate_guard.rs:62-69`): `ht_reconcile_log.mssql_hash` and every
  `ht_*_legacy.sync_hash` are **stored** SHA256s. A byte change in what gets hashed
  invalidates all of them at once, on both `hotelnew` and `hotelville`.

Nothing above stops a **gate-only** change (a `matches` fn that isn't cited by any
`HashInput.segment`) — the invariant is one-directional (hash inputs ⊆ gate terms, not the
reverse). That asymmetry turns out to matter a great deal in §6.

## 2. Enumeration of every guarded term, with what each actually risks

| Site | Gate term | Hash input? | Write | Verdict |
|---|---|---|---|---|
| `booking.rs:806-813` | `legacy_cust_no` (bookings) | **Yes** — `booking.rs:911-920`, feeds `booking_canonical_hash` seg. 4 (`scheduler/sync.rs:2646,2652`) | `COALESCE($9, legacy_cust_no)` — `booking.rs:1045` | **In scope** — real gate⊇hash violation for the Some→None case |
| `checkin.rs:1481-1486` | `legacy_cust_no` (checkins) | **Yes** — `checkin.rs:1636-1641`, feeds `checkin_canonical_hash` seg. 5 (`scheduler/sync.rs:2705,2721`) | `COALESCE($13, legacy_cust_no)` — `checkin.rs:1737` | **In scope** — same class, no force-converge arm at all (§3) |
| `booking.rs:816-820` | `book_notes` | **No** — confirmed by `booking.rs:2005-2025` (`guarded_gate_terms_are_recorded_with_their_residual_weakness`: `hash_inputs_on_guarded_terms == vec!["legacy_cust_no"]`, i.e. `book_notes` is deliberately excluded) | `COALESCE($7, book_notes)` — `booking.rs:1043` | **In scope, but gate-only fix** — see §6, no hash touch needed |
| `room.rs:492-503` auto-create INSERT arm | `COALESCE($2, true)` / `COALESCE($3, false)` for `room_clean`/`room_maintenance` | N/A — `room::apply_room_upsert` is `always_writes: true` (`gate_guard.rs:315-321`), **no idempotency gate exists** for rooms at all | INSERT-only, one-time | **Related but out of scope** — see below |
| `customer.rs:583-587` | `legacy_id` | Yes, but as the row's internal legacy PK, write-once `COALESCE($37, legacy_id)` (`customer.rs:926`) | — | **Reviewed, excluded** — see below |

**Rooms residual (#268 leftover).** `room::apply_room_upsert`'s UPDATE arm was fixed
2026-07-28 to write-through (no COALESCE — `room.rs:402-403`, pinned by
`room.rs:842-848`+`:854-864`). The **auto-create INSERT arm** (`room.rs:492-503`,
2026-06-12 "AUTO-CREATE" fallback for an unknown `Room_no`) still defaults a `None`
projection to `true`/`false` instead of writing `NULL`. This is the *same pattern*
(COALESCE masks a legacy NULL) but a different *class*: there is no gate here — rooms has
none — so it cannot violate the gate⊇hash invariant this ADR is about, and the write only
ever executes once per room, on the very first CT observation of a `Room_no` unknown to
canonical. Recommend tracking it as an independent one-line follow-up (bind `NULL`/`false`-
neutral default, or just accept the INSERT-time default choice), not folding it into the
sentinel/hash-version machinery below — the blast radius and the fix shape are both
unrelated.

**Customer `legacy_id` (reviewed, excluded).** This guards a *different* transition:
`customer.rs:801` — "a `None` projection... must not force a re-apply, but a `Some`
projection against a still-NULL stored value MUST mismatch once so the UPSERT backfills."
That is monotonic **None→Some backfill** of `HT_Customers`'s own internal SERIAL PK
(migration 055), not a legacy Some→None clear. Hard-deletes resolve customers by this same
`legacy_id` (`customer.rs:1049-1077`) rather than by nulling it, so there is no legitimate
path back to `None` once set. No fix needed; noted here only because issue #269's evidence
cites `customer.rs:585` and a reader should not have to re-derive that it's a false lead.

## 3. What a Some→None edit does today, concretely — and it is NOT symmetric

Both `legacy_cust_no` sites and `book_notes` are "guarded," but they fail in genuinely
different, differently-dangerous ways. This distinction did not appear in the issue body
and is the single most important finding of this investigation.

### 3a. `legacy_cust_no` (bookings + checkins) — noisy, unfixable, but rare in practice

Both `Book_Cust_ID` (`booking.rs:81`, `BOOK_H_SELECT_COLS = "t.Book_ID, t.Book_Cust_ID, ..."`)
and `Cin_cust_no` (`checkin.rs:95-96`, `CHECKIN_H_SELECT_COLS`) are selected directly off
the CT-tracked base table (`HT_Book_H` / `HT_CheckIn_H`) — not a joined view — so a `None`
observed on an UPDATE-path re-apply (canonical row already exists) is the table's own
current committed column value at query time, not a partial-projection artifact.

On a genuine legacy clear:
1. CT event arrives → mapper re-SELECTs → `p.legacy_cust_no = None`.
2. Gate: `p.legacy_cust_no.is_none() || ...` → `true` → term "matches" → if every other
   term also matches, `existing_matches` → `true` → mapper returns `Ok(None)` (skip): no
   write, no domain event, watermark still advances.
3. **Independently**, the periodic diff-only reconcile sweep re-hashes both sides fresh
   next tick. `booking_canonical_hash`/`checkin_canonical_hash` render
   `legacy_cust_no.unwrap_or("")` (`scheduler/sync.rs:2652`, `:2721`) — legacy now renders
   `""`, canonical still renders its stale `"C21610"` — hashes **differ**.
   `record_divergence` (`scheduler/sync.rs:2950-2999`) inserts an unresolved
   `ht_reconcile_log` row, kind `Value`, and (since `Value` is silenceable,
   `is_silenceable` at `scheduler/sync.rs:2881-2883`) immediately writes the ack cache so
   it isn't re-logged every tick.
4. `auto_resolve_reconcile_log`'s periodic re-test (age-gated, `scheduler/sync.rs:4667-4685`)
   never closes it: `should_auto_resolve` requires the two fresh hashes to actually match,
   and they never will while canonical stays stale. If `RECONCILE_FORCE_CONVERGE_ENABLED`
   is on (it is, both sites, per repo memory), the periodic repair attempt
   (`force_converge_reconcile_row`, `scheduler/sync.rs:4310-4372`) exists for `bookings`
   but **re-drives the identical guarded gate** → `MapperNoop` → still doesn't converge.
   For `checkins` there is **no arm at all** (`scheduler/sync.rs:4368-4370`, "checkins are
   multi-row aggregates whose self-heal is still out of scope") — nothing ever attempts
   even a no-op repair.
5. The row is therefore **permanently unresolved** and ages past the `level_drift_alert`
   threshold, refiring on the alert cooldown indefinitely (the same "refires every 24h"
   cadence documented for the sibling `missing_pg` re-ingest gap,
   `scheduler/sync.rs:4657`) until an operator manually closes it or hand-edits the DB.
   This is worse than "invisible" — it is a **permanent, unfixable page**, and every
   documented reconcile arm assumes a large backlog of permanently-unresolvable rows is
   the specific failure mode to avoid (`scheduler/sync.rs:4631-4638`, the no-fairness /
   starvation warning).

**But**: cross-checking `docs/legacy-app/COMPAT_CHEATSHEET.md` (decompile-derived,
authoritative for iHOTEL's actual write paths) turns up **no documented code path that
ever writes SQL NULL** to `Book_Cust_ID` or `Cin_cust_no`. Every clear-like mutation of
either column is the customer-delete cascade, and it always writes the reserved sentinel
`'C0000'` (cheatsheet lines 420, 438, 611-640, 674, 1482-1486) — which is **Some→Some**,
already correctly caught by the guarded term (`booking.rs:1997-2000`: "Some→Some,
mirroring iHOTEL's customer-delete cascade... The gate term is guarded, so a Some→None
mutation would NOT defeat it — and must not"). New bookings/check-ins always populate a
real customer at creation. So the theoretical trigger for this gap — iHOTEL genuinely
writing NULL — has **no known live path** on either side. It is a correct, provable gap in
the type system, with (as far as current evidence shows) zero observed or reachable
production occurrence. §6 returns to why that matters for prioritisation.

### 3b. `book_notes` (bookings only) — silent, but common and reception-facing

`book_notes` is **not a reconcile-hash input at all** (§2 table; confirmed by the dedicated
regression test at `booking.rs:2005-2025`). So step 3 above never happens for it — there
is no independent hash sweep to catch what the gate misses. The consequence is pure
silence: canonical `book_notes` freezes at whatever it last was, forever, with **zero**
`ht_reconcile_log` signal, ever.

And unlike `legacy_cust_no`, the trigger here is not theoretical. The projection itself
manufactures it:

```
// booking.rs:566-570
let notes = header
    .try_get_str("Book_room_note")?
    .map(str::to_string)
    .filter(|s| !s.is_empty());
```

An ordinary "select all, delete, save" edit in iHOTEL's notes textbox writes an **empty
string**, not SQL NULL — and this line collapses that empty string straight to `None`
*before* it ever reaches the gate. `p.notes.is_none()` is then indistinguishable from "the
column was never populated." This is the single most plausible, everyday trigger anywhere
in this enumeration, and `book_notes` is served straight to reception
(`routes/new_bookings.rs:141,164`, `notes: row.book_notes`) — so a receptionist can be
looking at stale, previously-deleted notes text with no operational signal that anything
is wrong.

## 4. Sentinel design

**Options considered:**

1. **Reserved sentinel string inside `Option<String>`** (e.g. a magic literal standing in
   for "explicitly cleared"). Rejected outright: it doesn't even solve the representational
   problem on its own, because the *segment renderers* already collapse `None` and
   `Some("")` to the same rendered byte (`legacy_cust_no.unwrap_or("")`,
   `scheduler/sync.rs:2652` et al. — and `booking.rs:568-570` does the identical collapse
   one layer up for notes). Making a sentinel-in-a-string work still requires touching
   every `segment`/render call site to special-case it — i.e. the exact hash-byte change
   this ADR has to plan around, with none of the type-safety benefit.
2. **A proper tri-state, `LegacyField<T> { Unset, Cleared, Value(T) }`, at the CT-projection
   boundary.** Exhaustive-match-enforced, no magic strings, and it's the shape issue #269
   itself suggested. **Chosen.**

**Where it lives.** Only at the boundary between "the raw MSSQL row" and "the gate
comparator / hash segment renderer" — **not** a canonical schema change. `ht_bookings.legacy_cust_no`
and `ht_checkins.legacy_cust_no` stay plain nullable columns; canonical `NULL` still just
means `NULL`. `LegacyField<T>` only changes how the *incoming* legacy value is classified
before it's handed to `matches` and `segment`.

**Unset vs. Cleared cannot be read off the raw column alone** — SQL Server has one NULL,
not two — so the classification has to come from context the mapper already holds:

- `fetch_existing` found **no** canonical row yet (first-ever observation, INSERT path):
  always `Unset`. There is no prior value to protect and no COALESCE involved on this path
  today either — behaviour is unchanged.
- `fetch_existing` found an **existing** canonical row (UPDATE-path re-apply) **and** the
  freshly re-SELECTed base-table column is `None`: classify as `Cleared`. This is licensed
  by the same fact established in §3a — both `legacy_cust_no` columns are direct base-table
  selects, not view joins, so a CT-triggered re-apply reading `None` here is the table's
  real current value, not an artifact of a partial/joined projection. (This is exactly the
  property that does **not** hold for `book_notes`'s raw source in general — see below —
  so the two fixes must not share one code path uncritically.)

**`book_notes` needs one more step first.** Because `booking.rs:568-570` already discards
the NULL-vs-empty-string distinction from the raw legacy value before any tri-state logic
could see it, building a real tri-state for notes requires reading the *unfiltered*
`try_get_str` result: raw SQL NULL ⇒ `Unset` (assuming, pending the schema check noted in
§6, that `Book_room_note` is genuinely nullable and iHOTEL doesn't always write at least
`''`), raw `""` ⇒ `Cleared`, raw non-empty ⇒ `Value(x)`. This is a smaller, self-contained
change to one extraction line plus the gate `matches` fn plus the write bind — and,
critically, **does not touch any hash function**, because `book_notes` was never a hash
input (§2, §3b). It needs no `hash_version`, no migration, no flag. It is fixable today, in
isolation, at ordinary PR risk.

**The write must change in lock-step with the gate**, or a `Cleared` mismatch just spins:
`COALESCE($n, existing)` still preserves the old value if `$n` binds SQL NULL. The fix
binds the parameter *by tri-state*, not by `Option`: `Unset ⇒ bind NULL` (COALESCE
preserves, today's behaviour), `Cleared ⇒ bind Some("")` / `Some(NULL-for-legacy_cust_no)`
(COALESCE now applies the empty/absent value), `Value(x) ⇒ bind Some(x)` (unchanged). The
SQL text itself (`COALESCE($n, existing)`) does not need to change at all — only what gets
bound to `$n` does. That keeps this inside "dynamic `sqlx::query()`, no `.sqlx/` cache
implications" for both entities, consistent with the standing ban on new `query!()` macros.

## 5. The migration (the hard part) — `legacy_cust_no` only

`book_notes` needs none of this (§4). Everything below is specific to widening
`legacy_cust_no`'s hash segment on bookings and checkins, because that IS a hash input and
therefore trips the byte-compat invariant in §1.

### The key insight that shrinks the blast radius

The dangerous-sounding part — "any hash byte change invalidates every stored hash on both
sites" — is true of the **stored artifacts** (`ht_reconcile_log.mssql_hash`,
`ht_*_legacy.sync_hash`), but **not** of the live comparison itself, as long as both sides
of every fresh comparison move to the new function together. The periodic diff sweep never
compares an old-function byte string against a new-function one *within a single tick's own
pg-vs-legacy comparison* — `compute_current_pg_hash` and `compute_current_legacy_hash` (or
the customer-loop's inline equivalent, `scheduler/sync.rs:5409-5468`) always hash **both**
sides fresh, live, under whatever function is currently linked. So for every row that is
genuinely converged today (the overwhelming majority), a hash-function change produces
*different* bytes than before, but **identical** bytes on both sides of the comparison —
`record_divergence` is only called when the two differ, so nothing new gets logged for
already-converged data purely because the function changed.

Push this further: design the v2 segment renderer to be **byte-identical to v1 for every
state that exists in production today**, and only diverge for the one state that (per §3a)
has zero known live occurrence:

```
v1 segment:  legacy_cust_no.unwrap_or("")           // Unset and Cleared both render ""
v2 segment:  match tri_state {
                 Unset          => "",                     // identical to v1
                 Value(x)       => x,                       // identical to v1
                 Cleared        => "\u{0}CLEARED\u{0}",      // NEW — v1 had no way to reach this
             }
```

A `Cust_no` is always `C\d{4,5}` or the reserved `'C0000'` (cheatsheet); a NUL-delimited
literal can never collide with one. Given a pre-flight audit (§ below) confirms zero rows
are currently in the `Cleared` state, **v2 produces byte-identical output to v1 for every
row that exists in production right now** — the "re-diff storm" is provably a no-op on
cutover day for the data that exists, and only becomes live the day a genuine clear first
occurs (at which point catching it is the entire point of the fix).

That does not make the machinery around it free, so the plan still needs:

1. **`hash_version` column.** Migration `084_reconcile_hash_version.sql` (next free number
   per `migrations/pg/` — confirmed 083 is the latest today): `ALTER TABLE ht_reconcile_log
   ADD COLUMN hash_version SMALLINT NOT NULL DEFAULT 1`, plus a `resolution_reason TEXT`
   column if one doesn't already exist, so a cutover-driven bulk-resolve is distinguishable
   from a genuine auto-resolve in the audit trail. Update `init-db/init-hotelnew.sql`,
   `migrations/README.md`, and `CARDINALITY_MAP` per the standing repo gate for any PG
   schema change. The ack-cache tables (`ht_bookings_legacy`, `ht_checkins_legacy`)
   deliberately do **not** need a version column — see next point.

2. **No ack-cache schema change; accept one self-healing full-rehash pass.** The ack-cache
   short-circuit (`scheduler/sync.rs:5436-5448`, "identical `mssql_hash` as last
   acknowledged means drift, if any, is already logged") compares a *fresh* v2 hash against
   a *stored* v1 ack value. On the first tick after cutover this comparison fails for every
   row that carries `legacy_cust_no` in its hash — which, given the byte-identical design
   above, is fine: it just means the sweep falls through to actually fetching canonical and
   recomputing both hashes fresh (extra DB load, not a false divergence), confirms they
   still agree, and re-acks with the v2 bytes. One-time cost, self-limiting to one
   full-table pass, no structural change needed. This needs to be **throttled/batched**
   rather than let rip in one 15-minute tick against the shared legacy MSSQL server —
   reuse the existing `LIMIT`-and-age-order pattern already used elsewhere in this file
   (`scheduler/sync.rs:4670-4685`) rather than inventing a new one.

3. **Pre-flight bulk-resolve of currently-open rows for the two affected tables.** Any
   `ht_reconcile_log` row for `bookings`/`checkins` with `divergence_kind = 'value'` and
   `resolved_at IS NULL`, recorded under v1 bytes, will not be recognised by the v1-keyed
   dedupe once fresh hashes are v2 — a straight cutover would **duplicate** those rows
   rather than convert them. Mark them `resolved_at = NOW(), resolution_reason =
   'hash_version_cutover'` as a deploy-time step (SQL, not app code); the very next tick
   re-detects, fresh, under v2, and — because the *gate and write* also ship in the same
   change (§4's write-must-change-in-lock-step point applies here too) — a row that was
   genuinely stuck on a real Some→None now actually converges instead of reopening
   unfixably. A row that reopens anyway is real, current drift, not a version artifact.

4. **Per-site flip via a runtime flag, not a code-only cutover.** Both sites run the same
   binary from the same deploy, so "which site flips first" isn't a code question — it's an
   *effective* question, same shape as every other dark feature in this codebase. Add
   `RECONCILE_HASH_VERSION` (compose default `1`) with an `HFVILLE_RECONCILE_HASH_VERSION`
   sibling, **committed to `docker-compose.yml` per ADR 0004** (not a GitHub variable — the
   exact trap that ADR exists to prevent recurring). Ship the code for both v1 and v2 paths
   in one deploy, dormant; flip Ville first (smaller data volume, already the established
   canary site for sync changes per repo convention), observe one full sweep cycle, then
   flip Hotel. Rollback is a flag flip back to `1` — a ~10 minute deploy, not a code
   revert — because v1 and v2 are two internally-consistent hash universes; toggling which
   one is "live" doesn't corrupt anything already written, it only changes what the *next*
   tick computes fresh.

5. **Distinguishing a genuine new divergence from a version-transition artifact.** Every
   `ht_reconcile_log` row carries `hash_version` (point 1) going forward, and the pre-flight
   bulk-resolve (point 3) is separately tagged via `resolution_reason`. A row opened with
   `hash_version = 2` in the hour after a site's flip is transition-adjacent by definition
   and should get one extra look before paging; a row opened with `hash_version = 2` a week
   later is unambiguously new, real drift.

6. **Alerting during the window.** Preferred design needs **no special-cased suppression
   code at all**: because point 3 pre-resolves everything before the flag flips, and the
   full-rehash pass (point 2) is a same-tick, non-divergent re-ack for the byte-identical
   common case, nothing should cross the `level_drift_alert` age threshold (hours, not
   minutes) as a *direct* result of the cutover. The residual risk is pure **load**, not
   false alerts — a heads-up to on-call ("expect a `[sync]` log-volume bump for ~15-30 min
   post-flip on `bookings`/`checkins`, self-clears, do not action") is the practical
   mitigation, not a code change, consistent with the alerting guardrail (page only on
   confirmed/unrecoverable failures; this window produces neither).

### Pre-flight audit (cheap, do this regardless of whether the rest ever ships)

Two read-only `SELECT`s against legacy MSSQL (permitted — investigation only) that turn
§3a's "no known live path" from an inference into a verified fact:

```sql
-- HF Hotel and HF Ville, both:
SELECT COUNT(*) FROM HT_Book_H    WHERE Book_Cust_ID IS NULL;
SELECT COUNT(*) FROM HT_CheckIn_H WHERE Cin_cust_no  IS NULL;
```

A nonzero count changes the recommendation in §6 immediately and materially.

## 6. Test strategy

- **Old golden vectors stay, unmodified, still passing** —
  `bookings_hash_bytes_unchanged_for_golden_inputs` (`booking.rs:1954`),
  `checkins_hash_bytes_unchanged_for_golden_inputs` (`checkin.rs:3229`). These pin v1
  bytes; a v2 implementation must not touch them, proving v1 is still reachable/correct
  during the flag window.
- **New v2 golden vectors**, same fixture, asserting **byte-identity with v1** for the
  `Unset` and `Value(x)` states — this is the test that proves the "no-op on cutover day"
  claim in §5 rather than just asserting it in prose.
- **New `..._v2_distinguishes_cleared_from_unset`** — a fixture with an existing canonical
  row (`Some("C21610")`) and a fresh projection carrying `Cleared`; assert the v2 segment
  renders the reserved sentinel and differs from both the v1 rendering and any real
  `Cust_no`.
- **Red-test the actual fix** — `legacy_cust_no_gate_v2_defeats_some_to_none_edit`: same
  shape as the existing `guarded_gate_terms_are_recorded_with_their_residual_weakness`
  test but inverted — construct `ex.legacy_cust_no = Some("C21610")`,
  `p = Cleared`, assert `matches()` is now `false`. This is the test that would have failed
  today and must pass post-fix; write it FIRST (this is exactly a `tdd`-skill-shaped task
  when implementation is scheduled).
- **Write-converges test** — assert that, given a `Cleared` projection, the UPDATE actually
  persists `NULL` (or the empty value) into canonical, not just that the gate flags a
  mismatch — otherwise the fix "detects but doesn't heal," reproducing the original bug
  one layer down.
- **No-infinite-loop test** — apply the same `Cleared` projection twice in a row; assert
  the second application's gate now reports `matches() == true` (canonical caught up),
  proving the fix doesn't trade "never converges" for "converges, then re-emits forever."
- **Version-mixing regression** — `should_auto_resolve`/the stale-ghost arm
  (`scheduler/sync.rs:9759-9782`) must not compare a `hash_version = 1`-recorded
  `mssql_hash` against a freshly-computed v2 hash and call it either resolved or diverged;
  pin that the recorded/fresh comparison stays within one version.
- **book_notes' tests are simpler and ship independently** — a gate red-test (empty-string
  clear now defeats the gate) plus a write-converges test; no hash file needs touching
  because `book_notes` was never in the hash body (§2, §3b).

## 7. Sizing and recommendation

| Item | Size | Needs hash_version machinery? |
|---|---|---|
| `book_notes` gate + write fix | **Small** — ~0.5-1 day incl. tests. Gate `matches` fn, one extraction line (`booking.rs:568-570`), one bind-selection change. Ordinary PR risk. | **No** |
| Pre-flight MSSQL audit (§5) | **Trivial** — two `SELECT`s, both sites | N/A |
| `legacy_cust_no` tri-state + hash_version migration + flag + staged rollout | **Large** — realistically 3-5 focused engineering days plus a coordinated live-verification window (same shape as the mark_dirty / round-bill rollouts already in this repo's history), because it touches: the tri-state type, two mappers' gate+write, a new PG migration (+ init-db + README + CARDINALITY_MAP per repo gate), a new compose-committed flag pair (ADR 0004 pattern), the pre-flight bulk-resolve, and a two-site staged flip with its own verification runsheet. | **Yes** |
| Rooms auto-create INSERT residual | **Tiny**, but low value (one-time-only window per room) | No — no gate exists |

**Recommendation: do the cheap parts now, design-park the expensive part.**

*Argument for building `legacy_cust_no` anyway.* Byte-parity-to-legacy is this project's
core value proposition (CLAUDE.md invariant 3); a reconcile row that can never close
teaches operators to distrust/ignore alerts on `bookings`/`checkins` generally, degrading
the tripwire for *other*, real incidents on the same tables. `checkins.legacy_cust_no`
specifically feeds `routes/guest_documents.rs` (resolving which legacy customer a
check-in's TM.30/document mirror belongs to) — a write-adjacent, coexistence-safety-relevant
consumer, not merely a display field, so a stale pointer is a plausible (if currently
unobserved) source of guest-document misattribution across the legacy/canonical boundary —
exactly the class of bug this whole system exists to prevent.

*Argument for accepting the gap, for now.* §3a's cross-check against
`docs/legacy-app/COMPAT_CHEATSHEET.md` found **no documented iHOTEL code path that ever
writes literal NULL** to `Book_Cust_ID` or `Cin_cust_no` — the only clear-like mutation is
the `'C0000'` cascade, which is Some→Some and already handled correctly today. This is, by
current evidence, a real gap in the type system with zero confirmed live occurrences. The
fix requires touching the single most dangerous piece of machinery in this codebase (pinned
hashes, two-site blast radius) for a defect nobody has observed. Per this project's own
alerting principle (page on confirmed/unrecoverable failures, not theoretical ones), and
given the live, currently-open issue list has several *confirmed* active incidents (#276
housekeeping phantom-cleaning, #277 bb8 pool-checkout timeouts, #278 payment-folio
backfill), spending 3-5 days hardening an unconfirmed failure mode ahead of those is
arguably a misallocation.

**Net call:** ship the `book_notes` fix on its own merits soon (cheap, and §3b shows it is
the actually-plausible, currently-silent one). Run the pre-flight audit now — it's nearly
free and converts §3a from an inference to a fact either way. Treat the `legacy_cust_no`
hash-version migration as **fully designed and ready to build** (this document), but
**deliberately not scheduled** — pick it up only if the audit finds a nonzero count, or a
concrete incident (a guest-document misattribution, an operator complaint about a stuck
reconcile row) makes the trigger real. That keeps the backlog honest about a known gap
without spending the riskiest kind of engineering effort on a hypothetical.

## References

- Issue #269 (`gh issue view 269`) — the originating report; #268 (closed) — the sibling
  `room_clean`/`room_maintenance` COALESCE fix this pattern generalises from.
- `hotel-backend/src/sync/gate_guard.rs` — the pinned invariant this design must not break
  outside a dedicated implementation session.
- `docs/legacy-app/COMPAT_CHEATSHEET.md` lines 420, 438, 611-640, 674, 1482-1486 — the
  `'C0000'` sentinel convention that narrows §3a's real-world risk.
- ADR 0004 — the compose-committed-flag pattern this design's rollout mechanism reuses.
- `docs/coexistence/sync-incident-log.md` — prior incidents in the same gate/hash class
  (d09e756, the 2026-07-28 booking/customer gate gaps) that motivated `gate_guard.rs`
  existing at all.
