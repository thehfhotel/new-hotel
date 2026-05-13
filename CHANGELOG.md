# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [vNext]

### Added

- **Track F4 — `ht_rate_tiers` canonical pricing matrix
  (`audit-2026-05-13.md` T1 CRIT-4).** Closes the structural mismatch
  between the canonical PG rate table (which modeled
  `weekday/weekend/special`) and legacy iHOTEL's `HT_Rooms_Price`
  (`Room_Type × Cust_Type` composite). New canonical surface keyed on
  `(rate_tier_room_type, rate_tier_cust_type)` with `Room_Price` /
  `Room_Price_H` / `Room_Price_M` columns mirrored exactly.
  - **Migration 042** (`042_create_ht_rate_tiers.sql`): create
    `ht_rate_tiers` with UNIQUE constraint on the composite key + index
    on `rate_tier_room_type` for the room-lookup query path.
  - **`sync::mappers::rate_tiers::reload_rate_tiers`**: periodic-poll
    mapper wired into the existing `scheduler::mirror::reload_mirror_dimensions`
    (15-min reconcile cadence — `HT_Rooms_Price` is not CT-enabled and
    changes on the order of weeks). UPSERT keyed on the composite, NOT
    on the legacy `id`, so iHOTEL DELETE+REINSERT of a row keeps the
    canonical row pinned.
  - **`routes::new_rates`**: GET endpoints migrated to `ht_rate_tiers`.
    `GET /api/new/rates` returns the legacy single-row-per-room-type
    shape filtered by the default `ราคาปกติ` tier (frontend stays
    compatible). New `GET /api/new/rate-tiers?roomType=X&custType=Y`
    surfaces the full per-tier matrix. `lookup_by_room_and_cust_type`
    helper exported for future booking/check-in price resolution paths.
  - **`ht_rates` deprecation**: table NOT dropped. POST/PUT/DELETE
    continue to write to it so the existing /rates form does not error;
    canonical reads no longer touch it. Removal in a follow-on once the
    frontend retires.
  - **CARDINALITY_MAP.md**: `ht_rates` row updated to DEPRECATED;
    new `ht_rate_tiers` row added with `1:1` (composite key) to
    `HT_Rooms_Price`.
  - **Out of F4 scope** (deferred to Track G follow-ons):
    `HT_ContinueTime` (hourly extension master), `HT_Order_Up` /
    `HT_Order_Down` (monthly tier multipliers). Pricing writeback into
    iHOTEL stays one-way today.
  - Tests: `tests/test_rate_tiers.rs` (5 integration tests covering
    UPSERT idempotency, blank-key skip semantics, active-flag filter)
    plus `sync::mappers::rate_tiers` unit tests on the
    `is_acceptable_row` validator and `routes::new_rates::tests` on
    the date-parsing helper and default-tier constant.

## [2.63.16] - 2026-05-13

### Added

- **Track H — Process + CI gates (`audit-2026-05-13.md` Track H).** Closes
  three of the audit's Track H items.
  - `scripts/check-cardinality-map.sh` — CI guard that scans
    `migrations/pg/*.sql` for `CREATE TABLE ht_*` (bare-namespace,
    excludes `legacy_mirror.*`, `ville.*`, and `*_legacy` mirrors) and
    fails if any table lacks a backtick-quoted row in
    `docs/coexistence/CARDINALITY_MAP.md`. Self-tested via
    `--self-test` (PASS-case + FAIL-case) so a refactor of the script
    itself can't ship with a broken matcher. Today: 3 `ht_*` tables
    introduced by migrations (`ht_reconcile_log`, `ht_users`,
    `ht_sessions`) — all mapped.
  - `.github/workflows/docker-build.yml` — new `lint-cardinality-map`
    job runs on every PR + master push; gates `test-backend`,
    `build-backend`, and the `deploy` job via `needs:` and the deploy
    `if:` predicate. PROCESS.md P1 was reviewer discipline only; now a
    PR introducing a new `ht_*` table without a `CARDINALITY_MAP.md`
    entry is a red CI status.
  - `hotel-backend/tests/test_walkin3_multiroom_fixture.rs` — PROCESS.md
    P3 spike-capture-to-fixture promotion for
    `walkin3-20260424-100000` and `booking-checkin-20260424-101838`.
    Contains an active single-room sanity assertion + an `#[ignore]`'d
    `two_room_walkin_emits_two_checkin_ds_rows_and_one_header` test
    that codifies the expected multi-room shape per
    `COMPAT_CHEATSHEET.md:427-430`. Un-ignore when Track B (T1 CRIT-1)
    lands the `ht_checkin_rooms` junction + multi-room walk-in
    payload.

### Changed

- **`docs/coexistence/PROCESS.md` — P2.1 sub-rule added.** Rust sync
  mappers, writeback recipes, or service-layer code changes that
  depend on a legacy-mssql migration MUST apply the migration in the
  same change-window as the Rust deploy. Lesson from the 2026-05-12
  GuestRegistryMapper / migration 022 14-hour gap (1-per-second
  `Change tracking is not enabled` log spam, TM.30 under-count
  during the window). Documents the
  `docs/coexistence/RUNBOOK-mssql-022-apply.md` structure as the
  standard template for future legacy-mssql migration runbooks.

### Deferred

- **T7 HIGH-4 — MSSQL service container in CI for coexistence smoke
  test.** Larger infra work; tracked as Track H2.
- **T7 MED-3 — Backup off-site.** Tracked as Track H2.
- **T7 MED-4 — Deploy rollback workflow.** Tracked as Track H2.

## [vNext] - 2026-05-13 (Track F1)

### Added

- **Track F1 — `ht_room_calendar` canonical for `HT_Room_Status`
  (`audit-2026-05-13.md` T1 HIGH-4).** New per-(room, date) canonical
  ledger projected from the legacy booking-calendar table. Closes the
  T1 finding that no PG canonical existed for the central booking-grid
  source — every dashboard query previously reconstructed availability
  from `ht_bookings` + `ht_checkins`, silently missing direct iHOTEL
  edits to `HT_Room_Status` (mark-clean, walk-in, extend-stay, mid-stay
  room moves, …).
  - `migrations/pg/039_create_ht_room_calendar.sql` — `ht_room_calendar`
    with `BIGSERIAL` PK, `UNIQUE (rcal_room_id, rcal_date)` business
    key, FKs to `ht_rooms_new` / `ht_bookings` / `ht_checkins`, plus
    `rcal_legacy_id` (legacy allocator id), `rcal_status` (Thai/English
    literal preserved verbatim — `จอง` / `เข้าพัก` / `Check Out` / …),
    `rcal_customer_label` (legacy `room_Details` grid-tile text).
    Indices: `ix_ht_room_calendar_date_status` for future grid queries,
    `ux_ht_room_calendar_legacy_id` partial-unique for reconcile lookup.
  - `init-db/init-hotelnew.sql` — mirrored CREATE TABLE + migration
    seed (039) for fresh deploys.
  - `hotel-backend/src/sync/mappers/room_calendar.rs` — new
    `RoomCalendarMapper` (per-row CT dispatch). UPSERTs keyed on the
    `(rcal_room_id, rcal_date)` business pair; D-events delete by
    `rcal_legacy_id`. Required fields (`room_no`, `room_date`,
    `room_status`) fail loud; optional FKs (`room_Book_No`,
    `room_CheckIn_No`) collapse to NULL when missing rather than
    deferring the whole tile.
  - `hotel-backend/src/bin/sync.rs` — re-wires `HT_Room_Status` from
    the 5.4 retired-stub `RoomStatusMapper` to the new
    `RoomCalendarMapper`. The CT subscription was already enabled in
    Phase 5; this just adds the canonical projection.
  - `docs/coexistence/CARDINALITY_MAP.md` — row added linking
    `ht_room_calendar` ↔ `HT_Room_Status` (cardinality `1:1` per
    `(room, date)`, source `shared`, sync mapper
    `sync/mappers/room_calendar.rs`).
  - Read-path migration is intentionally deferred — existing
    `routes/calendar.rs` + `routes/rooms.rs` keep working off the
    bookings+checkins reconstruction until a follow-up track switches
    them to canonical reads.
  - Tests: 17 new lib tests in `sync::mappers::room_calendar::tests`
    locking the projection contract (Thai literal passthrough,
    NULL-on-required loud errors, date truncation, optional FK
    extraction, SELECT-vs-projection alignment) plus a refreshed
    wiring assertion in `bin/sync.rs` (`build_mappers_wires_room_status_to_room_calendar_mapper`).

## [2.63.15] - 2026-05-13

### Added

- **Wave 5c — QR / web payment routing to `Cin_Pay_web` (`audit-2026-05-13.md`
  Decision #1).** Closes the writeback-audit-2026-05-12 Wave 5c deferral.
  Previously the `POST /api/new/checkins/:id/payments` endpoint short-circuited
  `method = "qr"` through `insert_qr_payment_directly` — a direct
  `ht_payments` insert that bypassed `record_payment` entirely, never
  enqueued a `WritebackIntent::RecordPayment`, and consequently was
  invisible to iHOTEL's legacy reports.
  - `domain::payment::PaymentMethod::Web` — new variant whose
    `legacy_column()` returns `"Cin_Pay_web"` (alongside the existing
    Cash / Credit / Tran columns).
  - `routes::new_payments` — `"qr"` now maps to `PaymentMethod::Web` and
    flows through `record_payment`, so the canonical write, outbox
    enqueue, and event publish all happen atomically.
  - `writeback::recipes::payment` — tender-column match extended to route
    `Web → Cin_Pay_web`; the legacy split-tender invariant
    `Cin_Pay_Ds_Price = Cash + Credit + Free + Tran + Web` still holds
    (asserted via `debug_assert!` in `build_statements`).
  - `service::payment::method_to_legacy_string` — `Web` deliberately
    renders as `"qr"` (not `"web"`) in `ht_payments.pay_method` so the
    canonical wire contract / dashboard SSE feeds stay stable.
  - Test: `payment_recipe_emits_to_cin_pay_web_for_qr_method`.

- **Wave 5c — VAT percent read from `ht_settings.vat_percent` instead of
  hardcoded constant (`audit-2026-05-13.md` Decision #2).** The legacy
  `HT_Receipt_H.Receipt_VatPer` column previously stamped the value of
  `writeback::constants::RECEIPT_VAT_PERCENT = 7` verbatim, forcing a
  code-change-+-deploy cycle to flip between 0% and 7%.
  - `repository::settings::get_vat_percent` — new helper that reads
    `ht_settings.setting_value` keyed on `'vat_percent'`. Falls back to
    `DEFAULT_VAT_PERCENT = 7` on missing row / NULL / unparseable string /
    PG error (best-effort lookup; payment flow must not block on a
    settings hiccup). Accepts integer (`"7"`) or decimal (`"7.0"`) shapes.
  - `service::RecordPaymentCommand.vat_percent: Option<i32>` — threaded
    through to `WritebackIntent::RecordPayment.vat_percent` and into the
    writeback recipe's `PaymentInputs.vat_percent`.
  - `writeback::recipes::payment` — `vat_inclusive_split` now consumes
    `inputs.vat_percent`; the recipe falls back to the legacy constant
    only for queue rows enqueued before this field landed.
  - `migrations/pg/038_seed_vat_percent.sql` — seeds the row with
    `setting_value='7.0'`. To flip:
    `UPDATE ht_settings SET setting_value='0' WHERE setting_key='vat_percent'`.
  - Test: `vat_split_respects_settings_vat_percent` — verifies that at 0%
    Receipt_H emits `(Total, 0, Total, 'True', 0)` and at 7% it emits the
    legacy capture split.

### Changed

- `routes::new_payments` — the `insert_qr_payment_directly` bypass
  function is retired; all four tender methods (`cash` / `credit` /
  `transfer` / `qr`) now go through `state.payments_service.record_payment`.
- `WritebackIntent::RecordPayment` gained a `vat_percent: Option<i32>`
  field. Older queued intents deserialize with `None` (via
  `#[serde(default)]`) and the recipe falls back to the legacy hardcoded
  default — no replay required.
- `writeback::recipes::payment::execute` signature extended with a
  trailing `vat_percent: Option<i32>` parameter (already wrapped in
  `#[allow(clippy::too_many_arguments)]`).

## [2.63.14] - 2026-05-13

### Fixed

- **`bin/writeback.rs` — queue-depth alert silently disabled (Track D
  regression caught in 2026-05-13 production verification).** The
  `make_interval(mins => $1)` SQL in `fetch_queue_depth` was binding
  `QUEUE_STUCK_IN_PROGRESS_AGE_MINS: i64 = 10`. PostgreSQL's
  `make_interval` is overloaded only on `int` (i32) — a `bigint` (i64)
  bind raises `function make_interval(mins => bigint) does not exist`,
  which the janitor logs every 60s but never escalates. Net effect:
  the queue-depth Slack alert (stuck `in_progress` jobs older than 10
  minutes) has been silently dead since the alert was shipped.
  - Narrow the const to `i32`. `i32::MAX` minutes is ~4083 years so
    the narrower type costs nothing semantically.
  - Add unit test
    `queue_stuck_in_progress_age_mins_is_i32_for_make_interval` that
    pins the type via a `let bind_value: i32 = ...` compile-time
    assertion; a future widening fails to build.

- **Scheduler — Slack replay storm on every backend container redeploy
  (production verification 2026-05-13).** The polling jobs
  (`poll_checkins`, `poll_checkouts`, `poll_new_bookings`) tracked their
  watermark in process memory only. On restart the watermark seeded
  with `chrono::Utc::now().naive_utc()`, but the legacy MSSQL columns
  filtered against (`Cin_Room_In`, `Cin_Room_Out`, `Book_Date`) store
  **Thai local time** (GMT+7). The 7-hour offset between the seeded
  watermark and the source data made every event from the previous
  ~7 hours look "new" on the first post-deploy poll — ~45 Slack pages
  of replayed checkouts per redeploy.
  - `migrations/pg/037_scheduler_notification_state.sql` adds a
    singleton-per-pair `scheduler_notification_state(site_id,
    notification_type, last_event_at, updated_at)` table.
  - New `scheduler::notification_state` module: `load_watermark`,
    `save_watermark`, plus `now_thai_local()` for the seed case (used
    only if the PG row doesn't yet exist).
  - `scheduler::jobs` polling functions: on first poll after process
    start, hydrate from PG (or seed in Thai-local time if no row).
    Persist after every advance via UPSERT. Three unit tests pin the
    behavior:
    `now_thai_local_is_approximately_seven_hours_ahead_of_utc`,
    `notification_type_str_is_stable_db_key`,
    `notification_type_str_values_are_all_distinct`.

### Added

- **`docs/coexistence/RUNBOOK-mssql-022-apply.md` — apply runbook for
  the un-applied legacy-mssql migration 022 (Track E1).** The
  `GuestRegistryMapper` shipped in v2.63.12 has been failing ~1/sec on
  both sites because the legacy-side CT enablement
  (`HT_CheckIn_Other_People` PK + CT) never landed. Runbook covers the
  Phase-5-style sqlcmd-via-docker apply pattern for both sites (HF
  Hotel on `FRONT2\SQLEXPRESS` / db; HF Ville on `192.168.11.51,1436` /
  HOTEL via the `hfville` WireGuard interface), pre- and post-apply
  verification queries on both MSSQL and PG, the rollback path via the
  existing `.rollback.sql` companion, and what success looks like. The
  human operator (NOT this PR's deploy pipeline) executes the runbook
  during a quiet receptionist window — coordination required because
  the `ADD CONSTRAINT PRIMARY KEY` operation takes a brief Sch-M lock.

## [2.63.13] - 2026-05-13

### Added

- **Track E2 column expansion (`docs/coexistence/audit-2026-05-13.md`).**
  Widen canonical PG schema to mirror the full legacy `HT_Customers`
  and `HT_Rooms` surface so receptionist-visible state on our app and
  iHOTEL agree column-for-column. Previously the CT mapper silently
  dropped 20+ customer columns and 8 room columns; reconcile hashes
  hid the drop because both sides computed the hash over the same
  narrow projection.

  - `migrations/pg/035_track_e2_customer_columns.sql` adds 27
    columns to `ht_customers`: `cust_price_over` (running debt
    balance — business-critical, mutated on every check-in/out by
    legacy `Module1.UPDATE_MONEY`), the 8-field address tuple
    (`cust_add_no` through `cust_add_code`), the 12-field
    work-address tuple (`cust_work_name` / `_no` / `_moo` / `_soi` /
    `_road` / `_tambon` / `_ampore` / `_province` / `_code` / `_tel`
    / `_fax` / `_tax`), `cust_name2` (English/secondary name, used
    by FrmReportRR4), `cust_sex`, `cust_price_tier` (legacy
    `Cust_Type` rate-tier label, distinct from existing `cust_type`
    which mirrors `Cust_Type_Main`), `cust_last_change`, and
    `cust_contry` (preserved with the legacy spelling). All columns
    are nullable to match legacy NULL-tolerance. The existing
    `cust_address` column continues to receive a copy of
    `cust_add_no` for backwards compatibility with single-line
    address readers. [T1 HIGH-2]
  - `migrations/pg/036_track_e2_room_columns.sql` adds 8 columns to
    `ht_rooms_new`: `room_use_count` (running nights total),
    `room_x` / `room_y` (drag-drop grid coordinates), `room_group`
    (floor/wing), `room_power_open` / `_close` / `_status`
    (electricity relay state), `room_polity` (policy id). Defaults
    mirror legacy NOT NULL / DEFAULT clauses so the canonical row
    stays valid before the first CT tick lands. [T1 HIGH-3]

### Fixed

- **Customer sync mapper drops 9+ columns — Track E2 / T2 HIGH-4.**
  `EAGER_FETCH_COLUMNS` and the CT JOIN's `SELECT` clause in
  `hotel-backend/src/sync/mappers/customer.rs` widened from 8 to 33
  legacy columns. Every column the new schema (migration 035)
  captures is now also written by the UPSERT and INSERT branches in
  the I/U path and the eager-mirror path used by check-in. The
  idempotency `matches()` check correspondingly compares all 33
  mirrored columns so a debt-balance change can no longer
  idempotency-skip on a re-applied row.
- **Room sync mapper drops 8 columns — Track E2 / T1 HIGH-3.**
  `RoomMasterMapper::select_sql()` widened to project
  `Room_Use_Count`, `Room_X`, `Room_Y`, `Room_Group`,
  `Room_Power_OPEN` / `_CLOSE` / `_STATUS`, and `Room_Polity`. The
  `apply_room_upsert` UPDATE writes the new values with `COALESCE`
  semantics so a NULL legacy column preserves PG-side state.

### Notes

- Writeback for `Cust_Price_Over` (our app mutating the running debt
  balance during check-in/out) is intentionally NOT in Track E2 —
  for now we read the running balance from canonical PG (sync'd
  from legacy) but don't modify it. Track G owns the behavioural
  change.
- The `room_price_weekday/weekend/special` axis on `ht_rooms_new`
  still doesn't match the legacy `Room_PriceA/B/C` model (legacy
  indexes prices by customer-type, not day-of-week). That's a
  Track F (canonical rate-table model) item and stays open.

## [2.63.12] - 2026-05-13

### Added

- **`HT_CheckIn_Other_People` CT subscription + canonical mapper —
  Track E1 / T2 HIGH-3 (`docs/coexistence/audit-2026-05-13.md`).**
  The legacy iHOTEL app records companion-guest entries via INSERT on
  save (FrmCheckIn.cs:9490) and DELETE-then-REINSERT on edit
  (FrmCheckIn.cs:9975); until now the table had no primary key, no
  CT subscription, and no mapper — so `ht_guest_registry` was silently
  stale and TM.30 immigration reporting (Thai legal obligation for
  foreign-guest registration) was under-counting companion entries
  every time the receptionist used the iHOTEL "Other People" tab.
  - `migrations/legacy-mssql/022_phase5e_other_people_rooms_cancel.sql`
    enables `PRIMARY KEY` on `HT_CheckIn_Other_People.id` (IDENTITY)
    and enables Change Tracking. Rollback script provided.
  - `migrations/pg/034_ht_guest_registry_legacy_id.sql` adds
    `guest_legacy_id INTEGER UNIQUE` to `ht_guest_registry` so the
    sync mapper can UPSERT cleanly across iHOTEL's DELETE+REINSERT
    edit pattern without accumulating duplicate companion rows. This
    is the only schema change in Track E1; the wider column expansion
    on `ht_customers` / `ht_rooms_new` is deferred to Track E2.
  - `migrations/pg/033_sync_status_seed_track_e1.sql` adds
    `legacy_sync_status` rows for `HT_CheckIn_Other_People` and
    `HT_Rooms_Cancel` so the CT watcher's per-tick observability
    update path hits a row instead of silently no-op-ing.
  - `sync/mappers/guest_registry.rs::GuestRegistryMapper` projects
    `(id, Cin_no, Cin_name, Cin_contry)` (the deliberate iHOTEL typo
    `Cin_contry` is preserved verbatim) and UPSERTs into
    `ht_guest_registry` keyed on `guest_legacy_id`, resolving
    `guest_cin_id` from the parent `ht_checkins` row. Deferred apply
    on parent-not-yet-mirrored matches the existing booking / customer
    defer pattern.
  - `bin/sync.rs` wires `HT_CheckIn_Other_People` into
    `CT_ENABLED_TABLES` + `build_mappers`.

- **`HT_Rooms_Cancel` mirror mapper — Track E1 / T2 HIGH-5
  (`docs/coexistence/audit-2026-05-13.md`).** CT was enabled on
  `HT_Rooms_Cancel` back in Phase 5 (legacy-mssql migration 020) but
  no mapper existed — a dangling subscription that kept SQL Server's
  CT retention growing forever without a consumer. The new
  `RoomsCancelMirrorMapper` in `sync/mappers/mirror.rs` mirrors the
  cancelled-room ledger into `legacy_mirror.ht_rooms_cancel` (mirror
  table existed since migration 020). Mapper is wired into
  `CT_ENABLED_TABLES` + `build_mappers` in `bin/sync.rs`.

### Fixed

- **`HT_CheckIn_Ds` D-event orphan recovery — Track E1 / T2 HIGH-6
  (`bin/sync.rs`).** When a CT tick delivered only `HT_CheckIn_Ds`
  D-events without a sibling header CT row, the existing
  `CheckInRoomsMapper::coalesce_key` returned `None` (because the
  joined `Cin_No` column is nulled on D rows) and the canonical
  aggregate sweep never ran — stranding the canonical row in its
  pre-delete state forever. The dispatcher now (a) routes all
  `HT_CheckIn_Ds` batches into the coalesced aggregate path (even
  pure-D-only batches), and (b) for D rows that produced no key,
  back-queries `ht_checkins.legacy_checkin_ds_id` to recover the
  parent `Cin_no`. Lookup misses (mirror never had the row) emit a
  WARN; recovery hits emit a DEBUG log so operators can audit the
  recovery path.

- **Walk-in vs parse-failure log distinction — Track E1 / T2 MED-4
  (`sync/mappers/checkin.rs`).** The walk-in short-circuit (no
  parent-booking lookup) matched both `Some("")` (legitimate walk-in
  where `Cin_Book_no=''`) AND `None` (legacy column was NULL OR a
  parse failure upstream cleared `legacy_book_id`). The two are
  operationally distinct: walk-in is normal flow; a parse failure is
  a sync-quality signal. A debug log at the branch entry now records
  the exact `legacy_book_id` shape so operators can distinguish the
  two cases in production trace output.

- **`HT_Book_Date` intentional-drop documentation — Track E1 / T2
  MED-1 (`sync/parent_loader.rs` + `sync/mappers/booking.rs`).** The
  per-night `Book_ok` cancellation state was loaded into
  `BookingAggregate.nights` but silently dropped in `project_aggregate`.
  Both call sites now carry a doc comment explaining the deliberate
  drop pending a Track E2 / Track G column on `ht_bookings`.
  Behaviour unchanged.

## [2.63.11] - 2026-05-13

### Fixed

- **Coexistence observability runtime fixes — Track D
  (`docs/coexistence/audit-2026-05-13.md`).** Six findings from T7
  (operational observability). No UX behavior removed; one schema
  migration (032) adds three nullable columns to `ht_reconcile_log`.
  The audit's headline conclusion was "the system has good
  availability observability and almost no correctness observability";
  Track D closes that gap before Tracks B/E/F/G touch production.
  - **T7 CRIT-1 — Cardinality-aware reconcile
    (`scheduler/sync.rs`).** The reconcile job's ack cache silenced
    hash-mismatch divergences after the first detection. For a
    cardinality drift (e.g. 3 multi-room `View_CheckIn_Ds` rows
    collapsed into 1 `ht_checkins` row), hashes can never match
    regardless of CT-mapper correctness — but the ack-by-hash
    silenced every subsequent tick, and a single multi-room folio
    never tripped the 50-rows/hr volume alert. The reconcile now
    fetches `(mssql_hash, mssql_row_count)` and
    `(pg_hash, pg_row_count)` and classifies divergences as one of
    `value` / `cardinality` / `missing_pg` / `missing_mssql`. Only
    `value` and `missing_mssql` are silenceable; `cardinality` and
    `missing_pg` re-fire every tick until operator action repairs
    canonical state.
  - **T7 CRIT-2 — `/api/new/sync/status` reads `legacy_sync_status`
    (`routes/new_sync.rs`).** The endpoint was reading `sync_status`
    (4 entity_types written only by the demoted 15-min reconciler);
    12 CT-tracked entities had no health surface so a half-broken CT
    watcher silently lost mappings while the dashboard showed
    "healthy". Now reads `legacy_sync_status` (16 entities — 10
    canonical + 6 legacy_mirror) and unions a `writebackQueue` block
    with grouped `(intent, status)` counts so operators see queue
    depth without switching to the DB shell.
  - **T7 CRIT-3 — Watermark stall watchdog (`bin/sync.rs`).** Shadow
    mode rolls back every tick including the watermark UPDATE;
    `legacy_sync_status` counters keep advancing (`rows_skipped++`)
    so nothing looks broken until the 2-day MSSQL CT retention cliff
    silently drops changes. New background task polls
    `legacy_ct_state` every 60s and pages on either (a)
    `last_seen_version` not advancing for
    `LEGACY_SYNC_WATERMARK_STALL_ALERT_SECS` (default 30 min) in
    live mode, or (b) shadow mode running past the 36h hardcoded
    ceiling (below the 48h cliff with 12h cushion). Per-condition
    30 min cooldown.
  - **T7 HIGH-1 — Level-triggered drift alert
    (`scheduler/sync.rs`).** The pre-existing 50-rows/hr threshold is
    edge-triggered on volume; a single divergence that persists
    forever never alerts. New level-triggered digest fires when ANY
    table has unresolved rows older than 4h, per-table cooldown 24h.
    Complements (does not replace) the volume alert. The
    `pg_hash IS NULL` (canonical missing) path is also locked as
    never-silenceable.
  - **T7 HIGH-2 — Writeback queue depth alert (`bin/writeback.rs`).**
    Existing per-job exhausted-alert covers single jobs; bulk
    failures looked like single failures. New janitor task polls
    `writeback_jobs` every 60s and pages on `pending > 500`,
    `failed > 100`, or `in_progress > 5 with claimed_at > 10 min`.
    Per-condition 30 min cooldown.
  - **T7 HIGH-3 — Fingerprint expansion (`writeback/fingerprint.rs`).**
    The legacy schema-drift guard covered 15 writeback-touched tables
    only; 4 CT-watched mirror tables (`HT_CheckIn_Product`,
    `HT_Deposit`, `HT_Bill_Debt_H`, `HT_Bill_Debt_Ds`) and 1
    user-impact table (`HT_CheckIn_Other_People` — TM.30 immigration
    registry, Track E HIGH-3 will add the mapper) had no
    fingerprint guard. New `CT_EXTRA_FINGERPRINTED_TABLES` +
    `verify_ct_schema_fingerprint` cover them with an independent
    hash so vendor drift on a CT-only table doesn't force a
    writeback baseline bump and vice versa. `bin/sync.rs` now calls
    the CT-side guard.
  - T7 HIGH-4 (CI smoke test with MSSQL service container) deferred
    to a follow-on PR — the infra work is non-trivial and benefits
    from its own focus.

### Added

- Migration 032 — `ht_reconcile_log.divergence_kind` (TEXT) +
  `legacy_row_count` (INT) + `pg_row_count` (INT). Nullable for
  backward compatibility with rows inserted before Track D. Documented
  in `migrations/pg/032_ht_reconcile_log_cardinality.sql` and the
  migrations README.

## [2.63.10] - 2026-05-13

### Fixed

- **Coexistence payment concurrency hardening — Track C
  (`docs/coexistence/audit-2026-05-13.md`).** Four findings spanning T5
  (concurrency) and T2 (sync mappers). No schema migrations, no UX
  behavior removed. The spike (2026-04-24 §6) validated booking-edit
  race-safety with `TABLOCKX`+`HOLDLOCK`, but payment was never
  retested — this wave closes that gap by applying the same
  held-through-COMMIT pattern to every payment-touching write.
  - **T5 CRIT-1 — Payment `Total_Price_Pay` / `Total_Price_vat`
    re-aggregated from `HT_CheckIn_Pay` rows under
    UPDLOCK+HOLDLOCK held through COMMIT.**
    `writeback/recipes/payment.rs` previously emitted
    `Total_Price_Pay = ISNULL(...,0) + amount` (additive) and
    `Total_Price_vat = Total_Price_vat + amount` (additive). The
    header row's X-lock under default Read Committed isolation
    released at statement end (not COMMIT), so iHOTEL's absolute
    `SET Total_Price_Pay=<precomputed>` could commit after our
    additive UPDATE and silently overwrite our contribution —
    payment vanished from the legacy view. Both UPDATEs now
    re-aggregate live from `HT_CheckIn_Pay` rows held under
    UPDLOCK+HOLDLOCK; the lock survives through COMMIT and blocks
    any concurrent iHOTEL read-modify-write until we finish.
    Cancelled tender rows (`ISNULL(Cin_Status,'1') <> N'ยกเลิก'`)
    excluded per T2 CRIT-2.
  - **T5 HIGH-4 — Checkout `Total_Price_Pay` / `Total_Price_Balance`
    re-aggregated at recipe time, not trusted from intent payload.**
    `writeback/recipes/checkout.rs` previously emitted
    `Total_Price_Pay = <intent payload pay_total>` (absolute). The
    payload's `pay_total` was computed from PG state at intent-emit
    time; a payment writeback committing between emit and the
    checkout's `BEGIN TRAN` would clobber the second payment with
    the stale value. The recipe now emits a SELECT subquery
    aggregating live from `HT_CheckIn_Pay` rows under
    UPDLOCK+HOLDLOCK, same shape as payment.rs. The intent's
    `pay_total` is kept as a sanity-check input — `execute()` reads
    the live aggregate and logs a WARN if drift ≥ 0.005 baht.
  - **T5 HIGH-3 — `HT_Housewife` INSERT guarded by 5-minute dedup
    window.** `writeback/recipes/mark_clean.rs` previously emitted
    an unconditional INSERT. With no UNIQUE constraint we control on
    the legacy schema (read-only), two concurrent mark-clean events
    (housekeeper in iHOTEL at T0 + mobile app at T0+50ms) both
    succeeded → audit log over-counted. INSERT now uses
    `INSERT … SELECT … WHERE NOT EXISTS (SELECT 1 FROM HT_Housewife
    WHERE h_room=… AND h_cin=… AND h_date > DATEADD(minute, -5,
    GETDATE()))`. The 5-minute window matches realistic concurrent
    housekeeping scenarios; beyond it, subsequent marks are treated
    as legitimate re-cleans.
  - **T2 CRIT-2 — `Cin_Status` projected on the payment mapper +
    parent loader.** `sync/mappers/payment.rs::PAYMENT_SELECT_COLS`
    and `sync/parent_loader.rs::load_checkin_aggregate` now SELECT
    `Cin_Status` (the per-row cancel marker — `'1'` active or
    `'ยกเลิก'` cancelled per COMPAT_CHEATSHEET line 106 / 492). The
    aggregate sweep already mirrors the header's
    `Total_Price_Pay` (which now correctly excludes cancelled
    rows via T5 CRIT-1's filter), so this projection closes the
    CT-pipeline-to-canonical link. `Cin_Pay_Free` and `Cin_Pay_web`
    also added to the projection so a future aggregate-by-tender
    canonical column can be derived without another pipeline
    change. `ht_payments.pay_voided` schema column was already
    present in `init-db/init-hotelnew.sql` (since v2.13.0), so no
    new migration is required.

### Security

- **Track C closes a financial-integrity coexistence gap.** Concurrent
  iHOTEL save-payment + our writeback could silently drop our payment
  from the legacy view (T5 CRIT-1). Concurrent payment + checkout could
  silently clobber the second payment (T5 HIGH-4). Concurrent
  iHOTEL+our-app mark-clean could double-write the housekeeping audit
  log (T5 HIGH-3). Cancelled payments (`Cin_Status='ยกเลิก'`) were
  silently kept in `cin_paid_amount` (T2 CRIT-2). All four windows now
  closed with the UPDLOCK+HOLDLOCK pattern validated under live load in
  spike §6.

## [2.63.9] - 2026-05-13

### Fixed

- **Coexistence read-path triage — Track A
  (`docs/coexistence/audit-2026-05-13.md`).** Six findings spanning T3
  (read paths) and T6 (identifiers/formats). No schema migrations, no UX
  behavior removed.
  - **T3 HIGH-4 — `/api/occupancy` reads canonical `ht_checkins`.**
    `routes/occupancy.rs` now joins canonical and counts DISTINCT
    `cin_room_id`, filtered to `cin_status IN ('active','checkedout')`.
    Previously read `ht_checkins_legacy`, which has been demoted to
    drift-detection-only since 2026-04-28 — the occupancy trend chart had
    been silently broken for two weeks. The query is exposed as
    `OCCUPANCY_TREND_SQL` and four inline tests pin the canonical shape.
  - **T3 HIGH-5 — `/api/customers/*` reads canonical
    `ht_customers` + `ht_bookings` + `ht_checkins`.**
    `routes/customers.rs::list_customers_pg`, `get_customer_bookings_pg`,
    and `get_customer_stats_pg` migrated. Customer-name search now
    operates against the derived
    `TRIM(cust_firstname || ' ' || COALESCE(cust_lastname, ''))` alias.
    A new `resolve_customer_id_int` helper accepts either the canonical
    integer `cust_id` or the legacy `cust_no` (e.g. `C21636`) on path
    parameters so saved links and frontend pagination keep working
    through cutover. The legacy `book_status` i32-keyed map is gone; the
    canonical text enum is forwarded verbatim. Eight inline tests pin
    each canonical SQL string.
  - **T3 MED-3 — calendar `mode=new` drops the duplicate legacy fetch.**
    `routes/calendar.rs` previously ran both `fetch_legacy_calendar_data_pg`
    AND `fetch_new_calendar_data` when `SystemMode::New`, producing
    duplicate rows where the frontend dedup-by-key papered over divergent
    field values. The match-on-mode now picks exactly one source per mode.
  - **T3 MED-1 + MED-2 — Bangkok timezone for "today" / 06:00
    morning-flip.** `routes/stats.rs` exposes
    `BANGKOK_TODAY_SQL` and `BANGKOK_HOUR_SQL` constants and routes
    `stats::get_stats_pg`, `rooms::get_checkouts_today_pg`, and
    `new_stats::get_stats` through them. Previously bare `CURRENT_DATE`
    and `EXTRACT(HOUR FROM NOW())` evaluated in UTC; the
    `today_check_outs` tile missed any departure done before 07:00 BKK
    and the 06:00 morning-flip fired at 13:00 BKK. Four inline tests pin
    the constants against regression to bare UTC.
  - **T6 HIGH-1 — `Utc::now()` captured once per recipe in
    `booking_create`, `payment`, `checkout`, `mark_clean`.** Each recipe's
    `execute()` now captures `Utc::now()` at entry and threads it into
    `build_statements` via an `Inputs::created_at` field. Matches the
    Wave 5b pattern in `walkin` / `checkin_to_booking`. Closes the
    BKK-midnight straddle window where a recipe's first `Utc::now()`
    landed on today and a later one on tomorrow. Each recipe gains a
    `build_statements_is_pure_with_fixed_instant` test.
  - **T3 CRIT-1 — inactive-room "58 = 23 + 34" mystery: DEFERRED.**
    Live data shows zero `room_active = false` rows
    (`SELECT room_no FROM ht_rooms_new WHERE NOT room_active` → 0 rows),
    so the audit's hypothesis (one inactive cell rendered as
    "ไม่พบ") is not the proximate cause. The dashboard formula
    `available = total - occupied - checkout - booked` reconciles
    correctly with today's numbers (58 = 34 + 1 + 0 + 23). The audit
    rated this finding "Medium confidence — requires data spot-check"
    and the spot-check failed; reopen if/when an inactive room appears.

## [2.63.8] - 2026-05-12

### Changed

- **Writeback LOW tidying sweep (Wave 6 — audit cleanup
  `docs/legacy-spike/writeback-audit-2026-05-12.md`).** Final wave of the
  writeback audit, closing every LOW-severity item that did not require
  product policy or live-DB schema introspection. Items 1, 2, 4, 7, 10 are
  pure refactors / documentation; items 5, 6, 8, 9 add defensive guards
  with focused unit tests.
  - **Item 1 — shared `end_of_stay_at_almost_noon` + `enumerate_calendar_nights`.**
    Both helpers moved to `writeback/format.rs`. Previously defined
    identically in `booking_create.rs`, `booking_modify.rs`,
    `walkin.rs`, `checkin_to_booking.rs`, and `extend_stay.rs`.
    `enumerate_calendar_nights` now returns `Result` so its empty-range +
    365-cap guards (item 6) can surface failures to callers.
  - **Item 2 — shared `guest_prefix_for_country`.** Moved from `walkin.rs`
    and `checkin_to_booking.rs` (identical definitions) to
    `writeback/recipes/helpers.rs` next to `mark_cupon_printed`.
  - **Item 3 — `booking_cancel.rs:63` double-space documented.** The
    legacy capture's `delete from  HT_Book_Date` has TWO spaces between
    `from` and the table name; the parity-pinning comment was added in
    `writeback/recipes/booking_cancel.rs` so a future formatter autofix
    cannot silently normalize it away.
  - **Item 4 — money formatting consolidated to 2dp.** Every raw
    `{baht}` / `{price}` / `{total}` / `{amount}` interpolation on f64
    money values across `booking_create.rs`, `booking_modify.rs`,
    `walkin.rs`, `checkin_to_booking.rs`, `checkout.rs`, `extend_stay.rs`,
    `checkin_cancel.rs`, and `payment.rs` now renders with 2 decimal
    places. Matches the existing `HT_Receipt_H` / `HT_CheckIn_Pay` shapes
    and the `money_2dp` helper introduced in Wave 2 H4.
  - **Item 5 — `nights` validation hardened.** `walkin.rs`,
    `checkin_to_booking.rs`, and `booking_create.rs` now return a
    `WritebackError::Recipe` when `payload.nights < 1`; the silent
    `.max(1)` clamp that masked caller bugs is gone.
  - **Item 6 — `enumerate_calendar_nights` cap + empty-range guard rails.**
    The shared helper now logs a WARN when the 365-night cap truncates
    the range and returns `Err` on an empty range. The earlier per-recipe
    copies silently injected a phantom single-night row, which papered
    over caller bugs at the cost of one ghost `HT_Book_Date` row in
    MSSQL. Tests in `writeback/format.rs` pin both guards.
  - **Item 7 — i32-wrap observability.**
    `writeback/allocate.rs::select_next_int_with_lock` logs a WARN when
    the next allocated id approaches `i32::MAX / 2`. Affects every i32
    allocator (`HT_Book_Date.id`, `HT_CheckIn_Ds.id`, `HT_Receipt_H.id`,
    `HT_Room_Status.id`, `HT_Rooms_Cancel.id`, `HT_Customers.id`).
  - **Item 8 — Ville cutover collation safety.**
    `writeback/fingerprint.rs::verify_legacy_collation_safety` runs at
    worker startup and refuses to start when `SERVERPROPERTY('Collation')`
    contains `_CS_` (case-sensitive). Recipes pin every string literal
    to the case the .NET app emits, so a fresh Ville with the wrong
    collation would silently fork our writes; this check fails fast at
    startup. Wired into `bin/writeback.rs` between the pool init and the
    schema fingerprint check.
  - **Item 9 — `error::is_retryable` pattern-matches PG SQLSTATEs.**
    `writeback/error.rs` now only retries `sqlx::Error::Database` when
    the SQLSTATE is one of the documented transient codes (`40001`,
    `40P01`, `57P01`-`57P03`, `08000`-`08006`, `53300`). Integrity
    violations (`23xxx`), syntax errors (`42xxx`), and user-raised
    exceptions (`P0001`) now correctly fail permanently instead of
    pinning a worker thread on a deterministic failure (e.g.
    `unique_violation` on `writeback_jobs`). Wire-level
    `sqlx::Error::Io` / `Tls` / `PoolTimedOut` / `PoolClosed` /
    `WorkerCrashed` remain retryable.
  - **Item 10 — `set_context_info` pool-isolation contract re-checked.**
    `writeback/dispatcher.rs` docs now explicitly explain why no runtime
    assertion is feasible (bb8 has no API to enumerate other handles to
    the same `Pool` instance; the backend's separate pool lives in a
    separate process per `docker-compose.yml`). The contract remains
    structural and enforced by code organization.

This closes the writeback audit. Remaining: Wave 5c (QR routing + VAT
consistency) deferred pending product policy.

## [2.63.7] - 2026-05-12

### Fixed

- **Writeback MED cluster part B — multi-room, purity, misc (Wave 5b
  `docs/legacy-spike/writeback-audit-2026-05-12.md`).** Six MED-severity
  fixes that tighten the recipes' multi-room behaviour, pin `build_statements`
  purity, and harden two `booking_modify` shapes against legacy quirks.
  - **Item 1 — `mark_clean` prior-occupant filter.**
    `writeback/recipes/mark_clean.rs::fetch_prior_occupant` now filters by
    per-room `Cin_Room_Status = N'Check-Out'` (matches
    `COMPAT_CHEATSHEET.md:864-866`) instead of the whole-check-in
    `cin_status NOT IN ('ยกเลิก')`. Without the per-room filter a multi-room
    check-in where one room was already out and another still occupied
    surfaced the still-occupying sibling as the "prior occupant" for the
    housekeeping log. The whole-check-in cancellation guard is kept as
    belt-and-suspenders, and `ORDER BY` now pins NULLs-last on
    `Cin_Room_Out` so a NULL checkout time can never out-rank a real prior
    occupant (audit LOW-4 folded in opportunistically).
  - **Item 2 — `extend_stay` step-5 multi-room revert.**
    `writeback/recipes/extend_stay.rs::build_statements` step-5 now uses
    the same `room_no IN (SELECT Cin_Room_No FROM HT_CheckIn_Ds
    WHERE Cin_no=… AND Cin_Room_Status<>'Check-Out')` subquery as step-1.
    Prior single-room shape (`WHERE room_no={inputs.room_no}`) left every
    sibling room of a multi-room check-in stuck `room_use='no'` after the
    step-1 wipe.
  - **Item 3 — `checkin_cancel` HT_POWER_LOG row-target precision.**
    `writeback/recipes/checkin_cancel.rs::build_statements` step-7 now
    restricts the close-lights UPDATE to `id = (SELECT MAX(id) FROM
    HT_POWER_LOG WHERE room_no=… AND ROOM_POWER_END_BY='')`. Prior shape
    would close every open row for the room — overwriting `ROOM_POWER_NOTE2`
    / `ROOM_POWER_END_BY` on a crashed prior session's leftover row and
    corrupting the power-log audit trail.
  - **Item 4 — `Utc::now()` lifted out of `build_statements`
    (`walkin.rs`, `checkin_to_booking.rs`).** Both recipes' input structs
    gain a `created_at: DateTime<Utc>` field, threaded in by `execute()`
    via a single `Utc::now()` capture at the top. `build_statements` is
    now PURE (its doc-comment claim is finally true). Existing byte-parity
    tests upgraded from substring-matching to full-line exact-equality;
    two new `build_statements_is_pure_with_fixed_instant` tests pin the
    determinism contract.
  - **Item 5 — `booking_modify` notes also write `HT_Book_Ds.Book_Room_Note`.**
    `writeback/recipes/booking_modify.rs::build_statements` now pushes
    `[Book_Room_Note]={q}` to `ds_sets` whenever `new_notes` is set, in
    addition to the existing `[Book_room_note]={q}` on `header_sets`.
    iHOTEL's edit-booking form binds the visible note input to
    `HT_Book_Ds.Book_Room_Note` (capital R per `SCHEMA.sql:6` /
    `COMPAT_CHEATSHEET.md:671`), so without this write a note edit was
    invisible in iHOTEL until the receptionist re-saved.
  - **Item 6 — `booking_modify` `Book_date_ds` NOT-IN cast safety.**
    The kept-dates filter now casts BOTH sides to `DATE`:
    `AND CAST(Book_date_ds AS DATE) NOT IN (CAST('4/25/2026' AS DATE), …)`.
    Defense-in-depth — guarantees correct behaviour even if a future row
    is stored with a non-midnight time component (the legacy app stores
    midnight today, but no schema constraint enforces it).

## [2.63.6] - 2026-05-12

### Fixed

- **Writeback MED cluster part A — payload, retry, fingerprint (Wave 5a
  `docs/legacy-spike/writeback-audit-2026-05-12.md`).** Six MED-severity
  fixes that thicken the service→intent→recipe payload, harden the
  worker's back-population step against stolen-claim races, and expand
  the startup schema-drift detector to cover the five tables the recipes
  actually touch:
  - **Item 1 — customer phone preserve.**
    `writeback/recipes/checkin_to_booking.rs::execute` now threads the
    booking-time phone from `CreateCheckInPayload.customer_phone` into
    `Cust_Add_tel` instead of passing `None` unconditionally. Prior code
    wiped `HT_Customers.Cust_Add_tel` on every booking-linked check-in,
    forcing receptionists to re-enter the phone after each guest
    arrival. Plumbed end-to-end:
    `routes/new_checkins::build_check_in_writeback_context` now reads
    `ht_customers.cust_phone`, `service/checkin::CheckInWritebackContext`
    carries it, `service/checkin::enqueue_create_check_in` copies it
    into the payload, and the recipe's
    `customer_phone_preserved_when_supplied` + `_renders_empty_string_when_none`
    tests pin both branches.
  - **Item 2 — `price_per_night` plumbed from service.**
    `WritebackIntent::RecordPayment` gains `price_per_night_baht:
    Option<f64>` and `nights: Option<i32>` fields, populated by
    `routes/new_payments::resolve_checkin_billing` from
    `ht_checkins.cin_rate_per_night` + the derived stay-nights count.
    The recipe's `amount/nights` fallback is preserved as defensive
    only (kept for queue rows enqueued before this field landed).
    `price_per_night_from_payload_used_when_supplied` +
    `_falls_back_to_amount_over_nights_when_none` pin both paths.
  - **Item 3 — `ht_payments` back-population.** Migration 030
    (`030_add_ht_payments_legacy_columns.sql`) adds
    `legacy_pay_no VARCHAR(20)`, `legacy_receipt_no VARCHAR(20)`,
    `aggregate_id UUID` to `ht_payments` (mirrors migration 014's
    convention for `ht_bookings` / `ht_checkins`).
    `WritebackIntent::RecordPayment` carries
    `payment_aggregate_id: Option<Uuid>` so the worker's
    `back_populate_legacy_ids` can target the canonical payment row
    after the recipe allocates `Pay_no` / `Receipt_no`.
    `service/payment::record_payment` stamps `aggregate_id` on the
    fresh row inside the same PG transaction as the INSERT and the
    outbox enqueue, so an operator can trace any
    `ht_payments` row back to its legacy `HT_CheckIn_Pay.Pay_no` /
    `HT_Receipt_H.Receipt_no` from the canonical state alone — no
    JSONB dive into `writeback_jobs.legacy_ids` required.
  - **Item 4 — back-pop guard on `Err` arm.**
    `bin/writeback.rs::mark_done` now `return`s early on `Err(err)`
    from the status-flip UPDATE, matching the `Ok(None)` (stolen-claim)
    behavior. The prior code fell through and ran
    `back_populate_legacy_ids` even when the UPDATE itself errored,
    risking a clobber of a stolen-claim winner's `legacy_*` columns
    with our (possibly stale) values. The resolver's self-heal path
    (`salvage_legacy_ids`) still recovers the IDs from
    `writeback_jobs.legacy_ids` JSONB at the next intent, so no data
    loss — just a slower lookup. Pinned by
    `mark_done_err_arm_returns_before_back_population`, a textual
    source-code structural assertion analogous to
    `dispatcher::dispatch_calls_set_context_info_before_recipes`.
  - **Item 5 — fingerprint expansion.**
    `writeback/fingerprint.rs::FINGERPRINTED_TABLES` now covers 15
    tables (up from 10): added `HT_Cupon` (loyalty mark-printed),
    `HT_CheckIn_Pay` (payment row), `HT_Receipt_Ds` (receipt line),
    `HT_POWER_LOG` (lights audit), and `HT_Changed_Room` (room-move
    audit, pre-emptive). `EXPECTED_SCHEMA_BASELINE` extended with the
    55 new column tuples derived from
    `docs/legacy-spike/schema/01-baseline-schema.txt` lines 200-207,
    253-274, 292-298, 422-429, 445-454. `EXPECTED_FINGERPRINT`
    recomputed → `8e076342babe5394b149c6e5aea5801348329e4a6a227118b31714e5e5d504b0`.
    A vendor rename on any of these five tables previously slipped
    past the startup drift check and would have silently corrupted
    the recipe; now the worker refuses to start. Six new tests
    (`fingerprinted_tables_includes_wave_5a_additions`,
    `tracks_ht_{cupon,checkin_pay,receipt_ds,power_log,changed_room}_table`).
  - **Item 6 — `set_context_info` pool reuse hygiene.** Documented
    the writeback pool isolation contract in
    `writeback/dispatcher.rs::dispatch`'s docstring: `SET CONTEXT_INFO
    0x4E48` persists on the bb8 connection after release, but that's
    safe because the writeback binary creates its own MSSQL pool at
    `main()` startup and no other code path acquires from it (the
    backend's `AppState.legacy_pool` is a separate `bb8::Pool` in a
    different process). If a future refactor shares the writeback pool
    with non-writeback callers, an `on_release` hook clearing
    `CONTEXT_INFO` to 0 will be required to avoid leaking the tag onto
    reader queries.

  Eleven new unit tests across `writeback::recipes::checkin_to_booking::tests`,
  `writeback::recipes::payment::tests`, `writeback::fingerprint::tests`,
  and `bin::writeback::tests` lock in the fixes. Full unit suite (486
  tests across lib + binaries, up from 475 baseline) passes; the 3
  PG-requiring integration tests in `tests/test_bookings.rs` continue to
  fail on a host without a live PostgreSQL instance (pre-existing
  baseline). Clippy output unchanged at 81 warnings — zero new.

### Database

- **Migration 030 — `ht_payments.legacy_pay_no` + `legacy_receipt_no` +
  `aggregate_id`.** See `migrations/pg/030_add_ht_payments_legacy_columns.sql`.
  Adds three nullable columns with partial unique / lookup indexes; existing
  rows get NULL (the writeback worker's self-heal path handles missing
  IDs by falling back to `writeback_jobs.legacy_ids` JSONB).

## [2.63.5] - 2026-05-12

### Fixed

- **Writeback robustness — pool, scope, validation (Wave 4
  `docs/legacy-spike/writeback-audit-2026-05-12.md`).** Five
  HIGH-severity bugs plus a Wave 3 follow-up that left the writeback
  worker exposed to pool poisoning, wire-scope corruption, NaN/Infinity
  injection, photo audit-trail poisoning, prefix-injection sinks, and
  HT_Room_Status double-inserts:
  - **H11** `bin/writeback.rs::process_job` — defensive
    `IF @@TRANCOUNT > 0 ROLLBACK` sentinel runs on every legacy-conn
    checkout. If a previous job's `run_in_transaction` saw its
    `ROLLBACK TRAN` itself fail (network blip mid-rollback, MSSQL hiccup)
    the connection would return to the bb8 pool with an open transaction
    and the next `BEGIN TRAN` would nest instead of starting fresh
    (T-SQL `BEGIN TRAN` bumps `@@TRANCOUNT` rather than opening a new
    outer scope), causing the next TABLOCKX to hang or commit against
    the wrong scope. The sentinel is idempotent — zero wire overhead
    beyond a round-trip when `@@TRANCOUNT` is 0 (the normal case),
    heals the connection otherwise. Scoped to the writeback worker
    only (not added to `db::pool::create_pool`'s shared init) because
    only the writeback worker opens explicit transactions against the
    legacy pool — sync.rs / backfill_rooms.rs / API routes don't.
    Pinned by `reset_trancount_sql_is_guarded_idempotent_form`.
  - **H12** `writeback/recipes/mod.rs` — replaced the dual-statement
    `execute_capturing_identity_at` + `fetch_scope_identity` helpers
    with a single `execute_insert_with_output_id` that runs
    `INSERT … OUTPUT INSERTED.id … VALUES …` and reads the id from the
    INSERT response itself. The old helpers issued the INSERT and
    `SELECT SCOPE_IDENTITY()` as two separate `simple_query` calls,
    and SCOPE_IDENTITY is batch-scoped on the wire — tiberius would
    occasionally return `NULL` when the prior call's batch closed
    before the SELECT executed, surfacing as a confusing `Recipe(...)`
    error. `OUTPUT INSERTED.<col>` survives any scope quirk because
    the id streams back as part of the INSERT response. A
    `debug_assert!` pins the `OUTPUT INSERTED.` substring at the
    helper boundary so any future caller wired up without the clause
    fails immediately in test/dev. The legacy IDENTITY-keyed tables
    we touch today (`HT_CheckIn_Ds`, `HT_Receipt_H`, `HT_Book_Date`,
    `HT_Room_Status`, `HT_Rooms_Cancel`) had IDENTITY stripped by the
    vendor — all current recipes allocate via TABLOCKX MAX+1 — so the
    helper is `#[allow(dead_code)]` until a future table needs true
    IDENTITY capture.
  - **H13** `recipes/walkin.rs`, `recipes/checkin_to_booking.rs`,
    `recipes/checkout.rs` — every `execute()` entry point now calls
    `helpers::validate_finite(&[…])` to reject NaN/Infinity *before*
    any allocation or SQL formatting. `format!("{}", f64::NAN)` emits
    the literal string `"NaN"`, producing invalid SQL like
    `[Cin_Room_Price]=NaN` that fails MSSQL mid-transaction and leaves
    partial state (e.g. checkout's power log already stamped off but
    the totals UPDATE never written). For walkin / checkin_to_booking
    the values currently flow from `Money::as_satang()` (always finite)
    so the guard is defense-in-depth; for checkout the values arrive
    straight from the caller as `f64` with no Money wrapper, so the
    guard is necessary. Six new tests pin the labels so the error
    messages stay operator-grep-able.
  - **H14** `recipes/walkin.rs:148-156`, `recipes/checkin_to_booking.rs:138-146`
    — empty / whitespace-only `tmp_no` now suppresses the
    `Tb_Save_Image` UPDATE entirely. Previously
    `if let Some(tmp_no) = inputs.photo_tmp_no` accepted `Some("")`
    and emitted `WHERE tmp_no=''`, which matched every
    orphan-pending-cleanup row (the legacy app leaves `tmp_no=''`
    after a successful save) and re-stamped them with THIS check-in's
    `cin_no` / `cust_no` — poisoning the photo audit trail for
    unrelated guests. Filter via
    `.filter(|s| !s.trim().is_empty())`. Four new tests
    (`tmp_no_empty_string_skips_…`, `tmp_no_whitespace_only_skips_…`
    on both recipes).
  - **H15** `writeback/allocate.rs::validate_allocator_prefix` —
    defense-in-depth regex (hand-rolled, no `regex` dev-dep) validates
    every allocator prefix interpolated into `LIKE '{prefix}%'`
    before the SUBSTRING/MAX+1 SELECT executes. Accepts 1-4 uppercase
    ASCII letters + up to 4 ASCII digits + optional trailing `-`;
    rejects `'` `;` `%` `_` whitespace and anything outside the
    documented shape. Wired into `allocate_cin_no_with_now`,
    `allocate_pay_no_with_now`, `allocate_receipt_no_with_now`.
    Today every caller derives the prefix from integer year/month
    arithmetic so production values are guaranteed safe — the guard
    closes the templated-injection sink against any future code path
    that takes the prefix from untrusted input (config, request
    payload, etc.). Three new tests
    (`allocate_prefix_validation_accepts_real_prefixes`,
    `…_rejects_special_chars`, `…_caps_digit_run_length`).
  - **Wave 3 follow-up** `recipes/checkin_to_booking.rs:189-220`
    — the night-1..N branch now emits a single-statement
    `IF EXISTS … UPDATE … ELSE INSERT` upsert per night instead of
    a plain INSERT. Wave 3's H7 fix made `booking_create`
    pre-insert `HT_Room_Status` rows for every booked night with
    `status='จอง'`; when a multi-night booking later converts to a
    check-in, those rows already exist for nights 1..N and a plain
    INSERT would create duplicates (the table has no unique
    constraint per `SCHEMA.sql` inspection). The upsert matches the
    legacy app's "upsert per night" semantics
    (`COMPAT_CHEATSHEET.md:347-348`) and keeps the recipe atomic; a
    future check-in that extends past the original booking's last
    night still works (the extra night has no pre-existing row, so
    it INSERTs through the `ELSE` branch). Two new tests
    (`additional_nights_are_upserts_not_plain_inserts`,
    `multi_night_does_not_double_insert_room_status`) replace the
    old `additional_nights_are_inserted` test.

  Fifteen new unit tests across `writeback::allocate::tests`,
  `writeback::recipes::walkin::tests`,
  `writeback::recipes::checkin_to_booking::tests`,
  `writeback::recipes::checkout::tests`, `writeback::recipes::mod::tests`,
  and `bin::writeback::tests` lock in the fixes. Full unit suite
  (475 tests across lib + binaries) passes; the 3 PG-requiring
  integration tests in `tests/test_bookings.rs` continue to fail on a
  host without a live PostgreSQL instance (pre-existing baseline,
  not regression). Clippy output unchanged modulo the removal of the
  now-dead `fetch_scope_identity` / `execute_capturing_identity_at`
  warnings (net -2 warnings, zero new).

## [2.63.4] - 2026-05-12

### Fixed

- **Writeback booking-visibility coordination — Wave 3
  (`docs/legacy-spike/writeback-audit-2026-05-12.md`).** Four HIGH-severity
  bugs that left bookings invisible (or partially visible) in the .NET
  app's calendar grid, all in `hotel-backend/src/writeback/recipes/`:
  - **H7** `booking_create.rs` — now inserts one `HT_Room_Status` row per
    booked night with `status='จอง'`, `room_Book_No=Book_ID` per
    `COMPAT_CHEATSHEET.md:347`. Without these rows the calendar grid (which
    filters `where (room_status='จอง' or room_status='เข้าพัก')`) showed
    the booking as empty AND `checkin_to_booking`'s night-0 UPDATE
    matched 0 rows silently when a check-in was created against the
    booking. Added `room_status_id_base` field + allocator wiring.
  - **H8** `booking_modify.rs:202-234` — caption rewrite (`UPDATE HT_Rooms
    SET room_book_ds=…`) now fires on a date-only edit. Previously only
    fired when `customer_name + room_no + stay` were ALL `Some`; a date
    change cleared the caption at step 0b but never re-wrote it, so the
    calendar grid lost the booking caption for the new date range. Added
    `fetch_existing_customer_name` helper (mirrors `fetch_existing_room_no`)
    so the rewrite uses the existing values when the payload doesn't
    carry them.
  - **H9** `booking_modify.rs:99-102` — `HT_Book_H.Book_Date_in/out` now
    uses the date-only format (`'4/25/2026'`) to match `booking_create`
    and the latest legacy capture. Previously emitted midnight-suffix
    (`'4/25/2026 12:00:00 AM'`), creating two shapes for the same column
    in one writeback session. Dropped `midnight_of` from the imports.
    Rewrote the related test (was pinning the wrong format).
  - **H10** `booking_modify.rs:144-148` — price-only modify now preserves
    `Book_Room_Night` (and the computed `Book_Room_PriceToTal`) by
    fetching the existing value from MSSQL. Previously fell back to
    `new_nights_calendar.len().max(1) = 1` when `new_stay` was None,
    corrupting a 3-night booking's total from `2670.00` to `890.00`.
    Added `fetch_existing_book_room_night` helper; `validate_finite`
    guard now uses the same precedence so it actually defends the
    written total.
  - Added `ROOM_STATUS_RESERVED` constant (`'จอง'`) to
    `writeback/constants.rs`.

## [2.63.3] - 2026-05-12

### Fixed

- **Writeback financial integrity — Wave 2 (`docs/legacy-spike/writeback-
  audit-2026-05-12.md`).** Six HIGH-severity bugs from the audit plus the
  payment-idempotency MED, all in `hotel-backend/src/writeback/`:
  - **H1** `recipes/checkout.rs::execute()` no longer hardcodes
    `nights=1, room_price_total=0, product_total=0, net_total=0,
    pay_total=0, balance=0`. Every checkout was wiping real revenue from
    MSSQL with zeros. `WritebackIntent::CheckOut` now carries the real
    totals (added as `Option<f64>` with `#[serde(default)]` for back-compat
    with pre-Wave-2 queued events — the dispatcher logs a WARN and falls
    back to zeros for in-flight rows so the queue drains without manual
    intervention). `CheckOutCommand` extended with the same fields;
    `routes/new_checkins.rs::checkout` computes them from the canonical PG
    state (rate × nights + `cin_total_amount`).
  - **H2** `recipes/checkout.rs:106` — `Room_Use_Count` bumps by the real
    nights count (`Room_Use_Count + {nights}`) per
    `COMPAT_CHEATSHEET.md:289`, not always `+1`. Multi-night stays were
    under-counting by `nights − 1` (hidden because the spike captures were
    all 1-night).
  - **H3** `recipes/payment.rs` — `Cin_Pay_Ds_Price` and
    `Cin_Pay_Ds_PriceTotal` now equal the tender amount, restoring the
    legacy invariant `Cin_Pay_Ds_Price = Cash + Credit + Free + Tran +
    Web` (`COMPAT_CHEATSHEET.md:534`). Prior code wrote the nightly total
    on partial payments, breaking the shift report. `Cin_Pay_Ds_PriceOne`
    (unit price) and `Cin_Pay_Ds_Num` (nights) stay verbatim so the
    printed receipt line still shows the per-night breakdown. A
    `debug_assert!` validates the sum at build time.
  - **H4** `recipes/payment.rs` `HT_Receipt_Ds` line items now emit
    `S_Unit=1.00, S_Price=<amt>.00, S_Total=<amt>.00, S_PriceDiscount=0.00`
    via `money_2dp(…)?`, matching live capture
    `invoice-20260424-100827/writes.txt:8`. Prior code emitted bare
    integers (`1, 711, 711, 0`) — printed receipts had inconsistent
    decimal styling.
  - **H5** `domain/payment.rs:46` — `PaymentMethod::Transfer.legacy_column()`
    now returns `"Cin_Pay_Tran"` per `COMPAT_CHEATSHEET.md:515`, not
    `"Cin_Pay_Credit"`. The recipe was already correct; only the helper
    was wrong. Latent bug — the helper has no callers today but is
    `pub` and would silently misroute transfers if used by future code.
  - **H6** `format.rs::round_money` switched to banker's rounding
    (`(value * 100.0).round_ties_even() / 100.0`) to match .NET's
    `Math.Round(value, 2)` default (`MidpointRounding.ToEven`). The
    prior `f64::round` diverged at the 0.005 / 0.015 / 0.025 midpoints —
    invisible today (all prices are whole baht) but VAT splits or future
    fractional pricing would silently fork from the legacy app.
  - **Payment idempotency** (MED): `HT_CheckIn_Pay` INSERT wrapped in
    `INSERT … SELECT … WHERE NOT EXISTS (SELECT 1 FROM HT_CheckIn_Pay
    WHERE Pay_no=…)`. The follow-on
    `UPDATE Total_Price_Pay = ISNULL(...) + amount` would double-count
    on a retry after network-drop-on-COMMIT; the guard makes the insert
    a no-op when the `Pay_no` (allocated under TABLOCKX) already exists.

  Twelve new unit tests in `writeback::recipes::checkout::tests`,
  `writeback::recipes::payment::tests`, `writeback::format::tests`,
  `domain::payment::tests`, and `outbox::intent::tests` (including a
  pre-Wave-2 JSON deserialize regression) lock in the fixes. Existing
  byte-parity tests updated for the `INSERT … SELECT … WHERE NOT EXISTS`
  shape (column list and value tuple remain byte-for-byte identical to
  the legacy capture).

## [2.63.2] - 2026-05-12

### Fixed

- **Writeback allocators now emit legacy-compatible IDs and use Bangkok wall-
  clock for period prefixes.** Three CRIT findings from
  `docs/legacy-spike/writeback-audit-2026-05-12.md` (Wave 1):
  - `allocate_pay_no` now emits `R{yyMM}-{4digit}` (e.g. `R2604-0241`) instead
    of `P{yyMM}-{6digit}` (e.g. `P2604-000001`). Pre-fix, the `LIKE 'P2604-%'`
    predicate never matched the iHOTEL-allocated rows visible in
    `findings.md` §2 line 129 / live capture
    `docs/legacy-spike/raw/invoice-20260424-100827/07-events.txt:154`, so our
    MAX+1 sequence ran in a parallel namespace — invisible collisions would
    have appeared the instant writeback turned on.
  - `allocate_receipt_no` now emits `B{yyMM}-{4digit}` instead of
    `RC{yyMM}-{6digit}` (`findings.md` §2 line 130 / capture
    `walkin-20260424-095304/07-events.txt:120`).
  - `allocate_cin_no` / `allocate_pay_no` / `allocate_receipt_no` now derive
    year/month from the Bangkok wall-clock (via `format::bangkok_date`) rather
    than `Utc::now()`. The 7-hour BKK-vs-UTC offset would otherwise emit the
    previous period's prefix around month/year rollover.
  - `SUBSTRING(..., offset, 50)` offsets are now derived from `prefix.len()+1`
    rather than hardcoded, so future format tweaks can't silently misalign
    the MAX scan.

  New unit tests in `hotel-backend/src/writeback/allocate.rs::tests` lock in
  the format invariants and the Bangkok-calendar boundary behaviour without
  requiring a live MSSQL. The existing `payment.rs` byte-parity test (which
  bypasses the allocator with hardcoded `R2604-0250`) continues to pass.

## [2.63.1] - 2026-05-12

### Security

- **Next.js bumped 16.2.4 → 16.2.6** (also `eslint-config-next`). Closes
  Dependabot alerts #136 + #137: "Next.js Vulnerable to Denial of Service
  with Server Components" — vulnerable range `>= 16.0.0, < 16.2.5`. No
  application code changes required; `pnpm build` clean on 16.2.6 with the
  existing route tree (23 static + dynamic pages compiled).

### Fixed

- **Canonicalize `ht_checkins.cin_status` post-checkout terminal value.** Three
  forms had accumulated in production: `'checked_out'` (CT mapper),
  `'completed'` (bootstrap `bin/migrate_legacy.rs`), and `'checkedout'`
  (route layer via `repository/checkin.rs`). The route-layer readers in
  `routes/new_reports.rs` and the calendar route in `routes/rooms.rs`
  already filtered for `'checkedout'`, so reports + calendar were silently
  missing every CT-mapper-written checkout and the entire HF Ville bootstrap
  pool. Standardized on `'checkedout'`:
  - Migration `029_normalize_cin_status_terminal.sql` flips existing
    `'checked_out'` + `'completed'` rows (HF Hotel: 187, HF Ville: 1543).
  - CT mapper `sync/mappers/checkin.rs::derive_room_state` now writes
    `'checkedout'`.
  - Bootstrap binary `bin/migrate_legacy.rs` updated to write `'checkedout'`
    for forward consistency if ever re-run.
  - Writeback contract (`Cin_Room_Status = 'Check-Out'` to legacy MSSQL) is
    unaffected — the new app's canonical literal and the legacy MSSQL
    literal were always distinct.

## [2.63.0] - 2026-05-11

### Added

- **`docs/legacy-app/` — iHOTEL coexistence reference restored to repo.** Five
  analysis docs derived from the legal `de4dot` + `ilspycmd` decompile of the
  production `HOTEL.exe` (~210 KB total): `COMPAT_CHEATSHEET.md` (1900-line
  per-table contract with cascade catalog), `FEATURE_MAP.md` (UI-screen →
  tables map), `REPORTS_INVENTORY.md` (Crystal Report catalog), `SCHEMA.sql`
  (live schema dump), `OBFUSCATOR_STUBS_REMOVED.md` (de4dot cleanup notes).
  Previously committed as `legacy-reference/` and removed in `9cfc8ac`
  alongside the vendor binaries — but the analysis-only docs are ours,
  contain no vendor code, and are critical for resolving discrepancies
  between canonical PG state and what iHOTEL displays to receptionists.
- **`docs/legacy-app/EVERGREEN_ARTIFACTS.md`** — pointer to the off-repo
  vendor binaries + full decompile at `evergreen:/home/nut/new-hotel/legacy/`.
  Documents the public-flip blocker: `9cfc8ac` removed the working-tree
  copy but the vendor binaries (HOTEL.exe, vendor DLLs, Crystal Reports)
  are still reachable through git pack objects; history rewrite is required
  before going public. Also notes the single-host-loss risk and recommended
  off-host backup mitigations (not yet implemented).
- **Dashboard live updates via Server-Sent Events** — the receptionist
  homepage (`app/page.tsx`) now subscribes to `/api/events` and refetches
  stats/rooms/checkins automatically whenever a relevant `DomainEvent`
  arrives (`RoomMarkedClean`/`RoomMarkedDirty`, `CheckInCreated`/
  `CheckOutCompleted`/`CheckInCancelled`, `BookingCreated`/
  `BookingModified`/`BookingCancelled`). No more manual reloads after a
  booking or housekeeping flip. A 500ms debounce collapses event bursts
  (e.g. multi-night booking aggregates) into a single refetch, and a
  `visibilitychange` listener forces a refresh when the tab regains focus
  (covers laptop sleep / dropped SSE). A tiny dot in the page header
  shows live-connection status (green = connected, gray = reconnecting).
- **`/api/events` is now branch-aware** — `routes/events.rs` accepts
  `?branch=hfhotel|hfville|all`. HF Hotel uses `state.new_pool` (default,
  unchanged for existing callers), HF Ville switches to `state.ville_pool`,
  and `all` opens TWO `PgListener`s and multiplexes both `domain_events`
  channels through a single SSE stream via `tokio::select!`. If
  `ville_pool` is unavailable, the endpoint degrades to hfhotel-only with
  a tracing warning rather than 500-ing the connection. Previously Ville
  receptionists never saw live updates because their `domain_events`
  channel lives on a different PG database.

### Fixed

- **Diff-only reconcile job now compares MSSQL against canonical `ht_*`
  instead of the demoted `ht_*_legacy` mirrors** — `scheduler/sync.rs`
  (the 15-min drift tripwire) had been comparing MSSQL hashes against
  `ht_*_legacy.sync_hash`, but after the Phase 5.5 cutover on 2026-04-28
  the mirror tables stopped getting their data columns populated by the
  CT watcher — only `sync_hash`/`synced_at` tick. As a result, drift
  detection had become cosmetic noise: `ht_reconcile_log` accumulated
  ~2300+ unresolved entries pointing at nothing actionable (the
  canonical PG state was current via the CT watcher; only the mirror
  was stale). The DiffOnly hot path now hashes each MSSQL row in
  canonical shape and compares against the same-shape hash of the
  canonical `ht_*` row (joined by `legacy_cust_no` / `legacy_room_no` /
  `legacy_book_id` / `legacy_cin_no`). Drift now means "the CT mapper
  has a real gap" — an actionable signal worth alerting on. The field
  set is narrowed to columns the CT mapper actually projects (e.g.
  drops `Room_Group`/`Room_Book_Time`/`Book_Cust_Name` denormalisations
  that canonical doesn't store), so the signal-to-noise ratio stays
  meaningful. `Upsert` mode is retained as a forensic escape hatch but
  is no longer exercised by any deployed code path. 11 new unit tests
  cover hash alignment (canonical mirror = legacy → equal hashes),
  drift sensitivity (any tracked field change → different hashes), and
  the `bool_to_yesno` / `legacy_yesno_canonical` round-trip contract
  for room.clean/maintenance translation. The migration also adds a
  per-PK cache-only ack via `ht_*_legacy.sync_hash`: each unique
  `mssql_hash` is logged once per PK and subsequent ticks short-circuit
  before re-querying canonical, preserving the Phase 6 anti-spam
  property of the prior implementation.
- **Stay-extension dates now propagate from legacy** — `sync/mappers/
  checkin.rs::derive_stay_range` now sources `cin_expected_checkout`
  from `max(HT_CheckIn_Ds.Cin_Room_Out)` across still-active Ds rows,
  falling back to `HT_CheckIn_H.Cin_Date_Out` only when no Ds row is
  loaded, every Ds row is already `'Check-Out'`, or no active row has
  a populated `Cin_Room_Out` yet. Per `docs/legacy-app/COMPAT_CHEATSHEET
  .md` §`HT_CheckIn_Ds` (line "Update on extend (ClickUSE.cs:1146):
  updates Cin_Room_Out for stay extension."), the legacy iHOTEL writes
  stay extensions to the Ds row's `Cin_Room_Out`, NOT to the header's
  `Cin_Date_Out`. The previous mapper only read the header, so
  extensions never reached canonical PG — 4 production rows
  (HF Hotel CH26-005351, CH26-005385; HF Ville CH26-001041,
  CH26-001057) showed `cin_expected_checkout` 3-5 days in the past
  despite guests still actively staying. Includes 7 unit tests covering
  the single-room extend, multi-room max, empty-Ds fallback,
  fully-checked-out fallback, mixed (one checked-out, one active),
  NULL-Cin_Room_Out fallback, and end-to-end via `project_aggregate`.
  Backfill applied to the 4 stranded canonical rows (CURRENT_DATE on
  HF Hotel and HF Ville) so the dashboard isn't misleading until each
  guest's next CT tick re-projects the aggregate.
- **Calendar route `/api/rooms/status` migrated off demoted legacy
  mirrors** — `routes/rooms.rs::get_room_status_pg` (the per-room-per-date
  calendar) was the last reader still hitting `ht_rooms_legacy`,
  `ht_checkins_legacy`, and `ht_bookings_legacy`, all of which stopped
  receiving row-level updates after the Phase 5.5 cutover on 2026-04-28
  (deferred from the v2.63.0 dashboard fix family because of its
  cross-join + `generate_series` + double-LEFT-JOIN shape). Rewritten
  against canonical `ht_rooms_new` (filtered `room_active = true`) +
  `ht_room_types` (for `type_name`) + `ht_checkins` (joined on the
  `cin_room_id` FK rather than the legacy `cin_room_no` string, with
  occupancy window `[cin_checkin_time, COALESCE(cin_checkout_time,
  cin_expected_checkout))` and `cin_status IN ('active','checkedout')`)
  + `ht_bookings` via `ht_booking_rooms.br_room_id` (window
  `[book_checkin, book_checkout)`, `book_status IN ('confirmed',
  'pending','checkedin')`). Checkin still takes precedence over an
  overlapping booking — the booking LEFT JOIN carries the
  `ci.cin_id IS NULL` guard. Response shape unchanged
  (`room_no`/`room_date`/`room_status`/`room_details`/`room_checkin_no`/
  `room_type`), with the legacy `cin_checkin_no` field now sourced from
  the canonical `cin_no` column.
- **Dashboard occupancy/checkin counts no longer stale** — `/api/stats`
  and `/api/rooms` now read canonical PG tables (`ht_rooms_new`,
  `ht_checkins`, `ht_bookings`, `ht_customers`) instead of the demoted
  `ht_*_legacy` mirror tables, which the Phase 5.5 diff-only reconcile
  job stopped updating on 2026-04-28. The receptionist dashboard had
  been showing ~2-week-stale occupancy/booking/checkout counts since
  the cutover.
  - `routes/stats.rs` — all 8 dashboard counters (`total_rooms`,
    `occupied_rooms`, `checkout_rooms`, `booked_rooms`,
    `today_check_ins`, `today_check_outs`, `active_bookings`,
    `total_customers`) rewritten against canonical tables. Preserves
    the 06:00 "checkout today" flip rule and the response shape
    consumed by `app/page.tsx`.
  - `routes/rooms.rs` — `list_rooms_pg`, `list_rooms_legacy_only`
    (HF Ville), `get_room_pg`, `get_room_legacy_only`, and
    `get_checkouts_today_pg` rewritten against canonical tables.
    API contract (`Room_no`, `Room_Type`, `Room_Use`, `Room_Book`,
    `Room_Clean`, `Room_Manternace`, etc.) preserved verbatim — no
    frontend changes required. Bool→yes/no string mapping and the
    morning grace period for checkouts are unchanged.
  - CT watcher (`bin/sync.rs`) was already keeping canonical tables
    fresh, so this is a pure routing layer fix — no migration or
    backfill needed.
- **Room maintenance flag now propagates from legacy** — extended the
  CT room mapper (`sync/mappers/room.rs`) to project legacy
  `Room_Manternace` (sic) into canonical `ht_rooms_new.room_maintenance`
  on every HT_Rooms CT tick. Previously only `room_clean` was being
  written by the mapper, so `room_maintenance` stayed at the
  backfill-time value forever — making the dashboard's maintenance
  indicator unreliable. Silent UPSERT (no domain event emitted),
  matching the existing pattern for non-clean column edits.
- **`/api/checkins` "Recent Activity" list no longer blank** — same
  Phase 5.5 fallout: the route was reading `ht_checkins_legacy` whose
  data columns are NULL for every row inserted after 2026-04-28, so
  the dashboard's recent-activity panel rendered "ไม่ระบุ / ห้อง /
  1 ม.ค. - 1 ม.ค." for every entry. Rewritten to JOIN canonical
  `ht_checkins` against `ht_rooms_new` (room number) and `ht_customers`
  (guest name). `Cin_Room_Out` falls back to `cin_expected_checkout`
  when the guest hasn't actually checked out yet, so active stays
  display their planned departure instead of epoch.
- Checkin CT mapper now eagerly mirrors the referenced customer from
  MSSQL when canonical `ht_customers` doesn't have it yet, replacing
  the previous defer-then-skip pattern that could permanently strand
  checkin rows during bulk catch-up. Production log lines like
  `ht_checkins apply deferred: customer not yet mirrored
  cin_no="CH26-001061" legacy_cust_no=Some("C1951")` were the
  observable form of the strand: the CT watermark advanced past the
  deferred row, so recovery depended on a later CT update for the same
  checkin re-firing the aggregate load. The mapper now pulls the
  matching `HT_Customers` row in-band via tiberius and INSERTs it into
  `ht_customers` in the same TX (`ON CONFLICT (legacy_cust_no) DO
  NOTHING` for concurrent-insert safety), then retries the FK lookup.
  Falls back to a distinct WARN (`customer not in MSSQL — leaving
  checkin deferred`) only when the row is truly missing in MSSQL — a
  defensive branch since the legacy app's own ordering invariant
  guarantees the customer exists before any checkin can reference it.
  - `sync/mappers/checkin.rs` — new
    `resolve_customer_or_eager_mirror` helper replaces the inline
    `resolve_customer_id` + WARN+`Ok(None)` block at the FK-resolution
    site; new `CustomerSource` enum carries either the live MSSQL pool
    or a test-injected stub supplier so the path is unit-testable.
  - `sync/mappers/customer.rs` — new `upsert_customer_from_row` helper
    exposed at `pub(crate)` scope; reuses the existing `project`
    projection and INSERTs via `ON CONFLICT (legacy_cust_no) DO
    NOTHING` against the partial unique index from migration 018.
  - Tests: 3 new PG-backed integration tests in
    `tests/test_sync_phase54_integration.rs`
    (`eager_mirror_inserts_customer_when_canonical_row_missing`,
    `eager_mirror_path_lets_checkin_apply_succeed_without_pre_seeded_customer`,
    `eager_mirror_defers_when_supplier_returns_no_row`).
- **Checkin CT mapper no longer strands cancellations when legacy
  deleted all `HT_CheckIn_Ds` rows but left the header.** Apply path
  now short-circuits to a `cin_status='cancelled'` UPDATE on the
  existing canonical row instead of waiting indefinitely for a
  resolvable room FK. Two production rows that had been stuck in
  `cin_status='active'` (`legacy_cin_no` IN `('CH26-005252',
  'CH26-005270')`) were backfilled alongside the deploy.
  - `sync/mappers/checkin.rs::apply_checkin_aggregate` — before FK
    resolution, when `projection.cin_status == 'cancelled'` AND a
    canonical row already exists, route into a new
    `apply_cancelled_for_present_header` helper that issues the same
    UPDATE as the header-gone `apply_cancelled` path. Guard keeps the
    legitimate-defer behaviour when the canonical row doesn't exist
    yet (the original INSERT CT row hasn't landed).
  - No domain event emitted on the short-circuit path — this is a
    recovery, not a transition. Subscribers re-observe state by
    re-reading the canonical row.
  - Tests: 1 new unit test in `sync::mappers::checkin::tests`
    (`project_cancelled_with_deleted_ds_rows_carries_no_room`) plus
    2 new PG-backed integration tests in
    `tests/test_sync_phase54_integration.rs`.
- **CT watcher / bootstrap survive transient MSSQL outages at startup.**
  `bin/sync.rs` previously exited with code 1 when the initial
  `create_pool` failed (e.g. WG tunnel down at container boot). Docker's
  `restart: on-failure:5` policy then capped retries at 5 attempts —
  HF Ville observed a 13-minute sync outage on 2026-05-11 because the
  legacy MSSQL came back online ~10 min after boot, well after Docker
  gave up. `create_pool_with_retry` now wraps both pool-init sites
  (CT watcher main + `--bootstrap`) in an exponential-backoff loop
  (5s, 10s, 20s, 40s, 60s, 60s, ... capped at 60s), Slack-pages once
  total elapsed crosses 5 min, and never propagates the error to
  `main()`. PG pool init is intentionally untouched — PG lives in the
  same docker network and exit-1 on its failure is the right behaviour
  (docker-compose dependency ordering catches it). New env-var knobs:
  `LEGACY_SYNC_INIT_RETRY_INITIAL_SECS`,
  `LEGACY_SYNC_INIT_RETRY_MAX_SECS`,
  `LEGACY_SYNC_INIT_RETRY_ALERT_AFTER_SECS`. Tests: 6 new unit tests
  in `src/bin/sync.rs::tests` covering the backoff schedule, saturation
  behaviour, default config, paging cadence, and an async loop-doesn't-
  exit assertion against an unreachable MSSQL.

### Changed

- **Follow-up legacy-mirror readers** still pending migration to
  canonical tables (tracked separately, NOT in this release):
  - `GET /api/rooms/status` calendar route (`get_room_status_pg`) —
    requires careful date-range/booking-window remodelling.
  - `scheduler/sync.rs` reconcile job — still pointed at legacy mirrors.

## [2.62.3] - 2026-05-10

### Security

- **Phase 7 audit medium-severity fixes** (post-public-flip
  hardening pass):
  - **M-2** (`8741678`): per-IP rate limit on `POST
    /api/auth/login` — 10 attempts per IP per 15 minutes,
    sliding-window. Custom `Arc<Mutex<HashMap<IpKey,
    VecDeque<Instant>>>>` limiter (~150 LOC); `tower_governor`
    rejected because cargo-tree showed 261 lines of transitive
    deps including `tonic`/gRPC. IP source: `X-Forwarded-For`
    leftmost → `X-Real-IP` → shared `IpKey::Unknown` bucket
    (header-omission can't bypass). 429 response body
    `{"error":"too_many_attempts"}` + `Retry-After` header in
    seconds. Only the login route is throttled — `/api/auth/me`
    and `/api/auth/logout` remain unmetered. +11 unit tests.
  - **M-3** (`8741678`): tightened CORS in `hotel-backend/src/main.rs`
    — replaced `.allow_methods(Any)` with explicit
    `[GET, POST, PUT, PATCH, DELETE, OPTIONS]` and
    `.allow_headers(Any)` with `[CONTENT_TYPE]`. Origin allowlist
    (`BACKEND_ALLOWED_ORIGINS`) unchanged from v2.59.2.
    `.allow_credentials(true)` retained for cookie sessions.
  - **M-4** (`8741678`): `tracing::trace!` in
    `writeback/recipes/mod.rs` no longer logs the full SQL
    statement. Now logs `stmt_kind` (first whitespace-delimited
    word — `INSERT`/`UPDATE`/etc) + `stmt_len` only. Production
    runs at `info` level so the previous behaviour was dormant
    in practice; this prevents a `RUST_LOG=trace` toggle from
    leaking guest PII into logs.
  - **M-5** (`f935bef`): locked `thai-id-middleware-tauri/src-tauri/src/server.rs`
    CORS — replaced `Any/Any/Any` with env-driven `AllowOrigin::list`
    (env var `CARD_READER_ALLOWED_ORIGINS`, default
    `http://localhost:3003,http://web:3003`), methods limited to
    `[GET, OPTIONS]`, headers to `[CONTENT_TYPE]`, no credentials.
    Closes the cross-origin Thai-ID exfiltration finding flagged
    in Phase 7. New `thai-id-middleware-tauri/README.md`
    documents the env var. +3 unit tests.

  Verified on combined master: `cargo test --lib` = 363 pass
  (was 352, +11 from M-2). Tauri `cargo check` clean.

### Added

- **Phase 9 — CodeQL workflow** (`9b9e9e7` — landed earlier in
  this version cycle, recorded here for completeness).
  `.github/workflows/codeql.yml` runs CodeQL on every push,
  every PR, and weekly cron, with a matrix over
  `javascript-typescript` + `rust`. Uses the `security-and-quality`
  query pack for verbose post-flip soak; can downgrade to
  `security-extended` later if too noisy.

- **Branch protection on `master`** (set via REST API, not in
  source). Blocks `force-push` and branch deletion. No required
  reviewers / status checks (single-maintainer repo).

## [2.62.2] - 2026-05-10

### Fixed

- **Backend healthcheck failed after AUTH_ENABLED=true cutover** —
  the docker-compose + Dockerfile probes hit `/api/mode`, which is
  registered inside the `build_new_routes` subrouter that's wrapped
  by `require_auth`. With auth on, the cookieless healthcheck
  request got back 401 → container marked `unhealthy` → `web`
  service refused to start (depends_on backend healthy). Switched
  both healthcheck commands to hit `/health` instead, which is
  mounted in `health_routes` outside the auth-gated subrouter
  exactly for this case. v2.62.1 deploy was tagged `failure` for
  this reason; v2.62.2 reapplies it cleanly.

## [2.62.1] - 2026-05-10

### Changed

- **Wire `AUTH_ENABLED` + `NEXT_PUBLIC_AUTH_REQUIRED` through the
  deploy pipeline** — Phase 4 cutover infrastructure. Without this
  the auth flags only worked when manually edited on prod and
  reverted on every CI deploy.
  - `docker-compose.yml`: uncommented `AUTH_ENABLED=${AUTH_ENABLED:-false}`
    on the backend service so the value propagates from `.env` to the
    container env.
  - `Dockerfile` (frontend): added `ARG NEXT_PUBLIC_AUTH_REQUIRED` +
    `ENV NEXT_PUBLIC_AUTH_REQUIRED=$NEXT_PUBLIC_AUTH_REQUIRED` to the
    builder stage so Next.js inlines the value at `pnpm build` time.
  - `.github/workflows/docker-build.yml`:
    - `build-frontend` job now passes `NEXT_PUBLIC_AUTH_REQUIRED=${{ vars.AUTH_REQUIRED || 'false' }}`
      as a build-arg to the docker build.
    - `deploy` job now passes `AUTH_ENABLED=${{ vars.AUTH_ENABLED || 'false' }}`
      through the JSON payload to `/srv/run-deploy.sh` so the value
      is written to prod's `.env` on every deploy.
  - GitHub Actions repo **variables** (not secrets — these are
    booleans, not credentials): `AUTH_ENABLED=true` and
    `AUTH_REQUIRED=true` set 2026-05-10. Both default to `false` if
    unset, preserving the previous "auth off" behavior for any
    fork/clone/replay.

### Operations

- Phase 4 cutover executed:
  - Provisioned admin `winut` (role=admin, user_id=1) via
    `docker compose exec backend ./create_user`. Password held
    out-of-band in this conversation.
  - `AUTH_ENABLED=true` set as repo variable; persists through
    redeploys.
  - Backend `/api/auth/login` returns 200 + `session` cookie for
    valid creds; `/api/new/*` returns 401 unauthenticated once
    this v2.62.1 deploy lands.

## [2.62.0] - 2026-05-10

### Security

- **Phase 7 pre-flip audit fixes** (audit run 2026-05-10 against
  master @ `2096d53`; full report archived in conversation):
  - **H-1**: replaced literal `REDACTED-sa-pw` placeholder in
    `hotel-backend/.env.example`, `hotel-backend/README.md`, and
    `scripts/sqlx-prepare.sh` with `CHANGE-ME-LOCAL-DEV-ONLY`.
    The literal is the actual production legacy MSSQL `sa`
    password (per the docker-compose comment fixed in H-2), so
    leaving it in `.env.example` after the public flip would
    leak the prod credential to anyone who reads the example +
    the comment side-by-side.
  - **H-2**: rewrote the `docker-compose.yml:334` comment that
    spelled out `sa/REDACTED-sa-pw` in plain text — replaced with a
    pointer to the rotation runbook (no literal credentials).
  - **H-3**: open-redirect guard on `/login?redirect=...` —
    previously `app/login/page.tsx` fed any `redirect` query
    param straight to `router.replace()`, accepting external
    URLs (`?redirect=https://evil.com`) and protocol-relative
    URLs (`?redirect=//evil.com`). Now requires the value to
    start with `/` AND not start with `//`; otherwise falls
    back to `/`.
  - **M-1**: invalidate all sessions for a user when an admin
    resets their password. Without this, a stolen cookie kept
    its full 24h expiry past the rotation. Added
    `SessionRepository::delete_for_user` /
    `delete_all_for_user` methods + wired into
    `AuthService::update_user` so any `password.is_some()`
    update bulk-revokes that user's `ht_sessions` rows in the
    same flow. Mock repo updated to match.

### Added

- **`create_user` binary in the deployed backend image.** The
  `bin/create_user.rs` source shipped in v2.60.0 (PR1) but
  `hotel-backend/Dockerfile` only COPY'd the 5 production
  binaries from the builder stage — the CLI was missing from
  the runtime image, blocking the operator's Phase 4 cutover
  step (`docker compose exec backend ./create_user ...`). Added
  `--bin create_user` to the cargo build line + the COPY in the
  runtime stage. Image grows by one ~5 MB statically-linked Rust
  binary.

### Operations (no code change, recorded for audit trail)

- `POSTGRES_PASSWORD` rotated on prod (`newdb` container live
  ALTER USER + `.env` update + `gh secret set` + `backend` /
  `sync` / `writeback` recreate). New value is held only in
  the GitHub Actions secret + `/home/deploy/new-hotel-production/.env`
  on evergreen. Old value (`REDACTED-pg-2026`) is now dead.
- `SLACK_WEBHOOK` rotated to a new Slack incoming webhook URL.

## [2.61.0] - 2026-05-10

### Added

- **Phase 4 PRs 2 + 3 + 4 — full backend authentication + frontend
  login + admin user management.** Combined release of 3
  parallel-developed PRs cherry-picked onto master in order
  (PR2 backend → PR3 frontend → PR4 admin). Like PR1, **inert in
  production by default** — operator must flip both
  `AUTH_ENABLED=true` (backend) and `NEXT_PUBLIC_AUTH_REQUIRED=true`
  (frontend) to take effect.

  - `f4bec2e` (PR2) — `feat(auth): add /api/auth/{login,logout,me}
    routes + AUTH_ENABLED middleware`. Three new endpoints under
    `/api/auth/*`. `POST /api/auth/login` validates creds, sets
    `Set-Cookie: session=<64-hex>; HttpOnly; SameSite=Lax; Path=/;
    Max-Age=86400[; Secure]` (Secure when `X-Forwarded-Proto:
    https`). `POST /api/auth/logout` clears it (idempotent).
    `GET /api/auth/me` returns the current `UserDto` (drops
    `password_hash`) or 401. Uniform `invalid_credentials` error
    on missing-user / wrong-password / inactive-user paths to
    prevent enumeration. New `axum-extra = "0.12"` (cookie
    feature) + `cookie = "0.18"` deps. New
    `hotel-backend/src/middleware/auth.rs` provides
    `require_auth` — when `AUTH_ENABLED=false` (default) it's a
    no-op pass-through; when true it validates the cookie,
    injects `crate::domain::user::User` into request extensions,
    or returns 401 JSON. Applied via `route_layer` to the
    `/api/new/*` subrouter only (public routes — health, mode,
    changelog, auth itself — are not gated). +14 unit tests in
    `routes::auth::tests`.

  - `cd0058a` (PR3) — `feat(auth): add /login page, AuthContext,
    and 401-redirect guard`. New `contexts/AuthContext.tsx` exposes
    `user`, `loading`, `error`, `login()`, `logout()`, `refresh()`
    via the `useAuth()` hook. `AuthGuard` (embedded in the
    provider) reads `process.env.NEXT_PUBLIC_AUTH_REQUIRED` —
    when `'true'` and the user is null and we're not already on
    `/login`, redirects with `?redirect=<current-path>`; when
    anything else, renders children regardless (the dev-mode
    escape hatch). `app/login/page.tsx` renders a Thai-localized
    centered card matching the dashboard's flat-panel style
    (ชื่อผู้ใช้ / รหัสผ่าน / เข้าสู่ระบบ); Suspense-wrapped per
    Next.js 16's `useSearchParams` rules. New `lib/api.ts`
    `apiFetch` helper always sends `credentials: 'include'`.
    `components/Sidebar.tsx` got a logout block above the
    collapse toggle (icon-only when collapsed). +1 test suite,
    +5 tests.

  - `ea7dff0` (PR4) — `feat(auth): admin user management
    endpoints + /admin/users page`. Three admin-only endpoints
    under `/api/admin/users` (gated by the same `require_auth`
    middleware + per-handler `actor.role == Admin` check):
    `GET` lists, `POST` creates (409 on username collision),
    `PATCH /{user_id}` updates any subset of `{active, role,
    password}` (422 on bad role string, 404 on missing user).
    Repository got `list_all`, `update_active`, `update_role`,
    `update_password_hash` methods plus `apply_admin_patch`
    façade. Service got `list_users`, `create_user`,
    `update_user` plus new `AuthError::UserNotFound` and
    `AuthError::UsernameTaken` variants. New
    `app/admin/users/page.tsx` renders a table with toggle-active
    and reset-password actions; sidebar's new "ผู้ใช้งาน" entry
    only appears when `useAuth().user?.role === 'admin'`. The
    `CreateUserModal` and `ResetPasswordModal` follow existing
    modal patterns. +8 service tests + 12 route tests + 3
    frontend tests.

  Combined verification on master:
  - `cargo test --lib` = **352 passed** (was 318 pre-Phase-4;
    PR1 +12 + PR2 +14 + PR4 +20 = +46 net)
  - `pnpm test` = **667 passed across 25 suites** (was 621/22
    pre-Phase-4; PR3 +5 + PR4 +3 = +8 net)
  - `cargo build --bin hotel-backend` and
    `cargo build --bin create_user` both link
  - `pnpm build` produces `/login` + `/admin/users` as
    pre-rendered routes

### Operator cutover (deferred — flip when ready)

1. `cargo run --bin create_user -- --username <op> --role admin`
   on prod (or container exec equivalent) — bootstrap the first
   admin while AUTH still off.
2. Set `AUTH_ENABLED=true` in backend env.
3. Set `NEXT_PUBLIC_AUTH_REQUIRED=true` in frontend env (the
   web image must rebuild for this since `NEXT_PUBLIC_*` is
   inlined at build time).
4. Push a no-op commit (or restart containers via the deploy
   runner) to roll the new env.
5. Verify: `curl -i https://<host>/api/auth/me` → 401,
   `/api/new/rooms` → 401, `/login` renders, post-login the
   admin sees the `ผู้ใช้งาน` sidebar entry and can create
   receptionist accounts at `/admin/users`.

## [2.60.0] - 2026-05-10

### Added

- **Phase 4 PR1 — Backend authentication foundation** (commit
  `486a236`, the first of 4 PRs to add auth to a backend that has
  none today). **Inert in production** — no HTTP routes, no
  middleware, no env flags wired. Future PRs add the routes (PR2),
  the frontend login page (PR3), and the admin user UI (PR4).

  - **Schema** (migrations 027 + 028):
    - `ht_users` — `user_id BIGSERIAL PK`, `username` UNIQUE,
      `password_hash TEXT` (argon2id PHC strings), `role` CHECK
      `IN ('admin','receptionist')`, `active`, `created_at`,
      `last_login_at`.
    - `ht_sessions` — `session_id VARCHAR(64) PK` (32 random bytes
      hex-encoded; the cookie itself IS the bearer token, so no
      additional hashing), `user_id` FK with `ON DELETE CASCADE`,
      `created_at`, `expires_at` (indexed for periodic cleanup),
      `ip INET`, `user_agent TEXT`. 24-hour fixed expiry by
      default (PR2 will decide on sliding-expiry).
    - `init-db/init-hotelnew.sql` updated with the same tables
      for fresh deploys; `migrations/README.md` updated with the
      new entries + ownership rows.

  - **Layered code** (per `docs/architecture.md` §6):
    - `domain/{user,session}.rs` — `User`, `Role` (Admin |
      Receptionist), `Session` with `is_expired()`. `User`
      derives `Serialize` for tests only — PR2 must wrap it in a
      `UserDto` that drops `password_hash` before HTTP exposure.
    - `repository/{user,session}.rs` — `UserRepository` +
      `SessionRepository` traits with `PgUserRepository` /
      `PgSessionRepository` impls. Uses `sqlx::query()` (dynamic)
      to avoid the `.sqlx/` cache regeneration step; PR2 may
      switch to `sqlx::query!()` once 027/028 land on a dev DB.
      Two high-level façade methods (`create_and_touch_login`,
      `delete_by_id`) wrap `pool.begin()` so unit tests can mock
      without fabricating a real `sqlx::Transaction`.
    - `service/auth.rs` — `AuthService` with `hash_password`,
      `verify_password` (constant-time, returns false on parse
      error), `login`, `logout`, `validate_session`. `AuthError`
      enum: `InvalidCredentials`, `UserDeactivated` (returned
      AFTER successful password verify so timing matches; PR2 may
      collapse on the wire), `Db`, `Hash`. Wrong-password and
      missing-user both return `InvalidCredentials` to prevent
      user enumeration.

  - **Admin CLI** (`bin/create_user`):
    - `cargo run --bin create_user -- --username NAME --role
      admin|receptionist [--password PASS]`. Without `--password`,
      reads from tty via `rpassword` with confirmation prompt.
      Uses `NewDbConfig::from_env()` to build the pool. Insert +
      hash in one TX.

  - **New crate deps** (`hotel-backend/Cargo.toml`): `argon2 =
    "0.5"` (clean transitive tree: `base64ct`, `blake2`,
    `password-hash`, `rand_core`), `rpassword = "7"` (only
    `libc` + `rtoolbox`).

  - **12 new unit tests** in `service::auth::tests` — all use
    in-memory `Mutex<HashMap>` mocks of the repository traits,
    no DB needed. Cover password round-trip, login success/wrong-
    password/missing-user/deactivated paths, session validation
    for unknown/expired/post-deactivation cases, logout idempotency.
    Plus 4 pure tests in `domain/{user,session}.rs`.

  Verified: `cargo check` clean (only pre-existing warnings),
  `cargo test --lib service::auth` 12/12 pass on combined master,
  `cargo build --bin create_user` links cleanly. PR1 is mergeable
  in isolation and CANNOT affect production behavior.

## [2.59.3] - 2026-05-10

### Security

- **Tauri 2.9.5 → 2.11.1** — patches CVE-2026-42184
  (GHSA-7gmj-67g7-phm9, "Origin Confusion in `is_local_url()`",
  CVSS 4.0 = 6.1 medium). Affects `thai-id-middleware-tauri/`, the
  Tauri desktop helper distributed to receptionist workstations for
  Thai national ID card reading. The bug allowed remote pages whose
  domain's first label matches a registered custom protocol (e.g.
  `http://app.attacker.com/`) to be classified as local origins on
  Windows/Android — letting them invoke IPC commands explicitly
  scoped `local: true`. We don't ship a malicious frontend, but this
  is patched defensively and to keep Dependabot quiet now that the
  repo is moving toward public.

  Lock-only update via `cargo update -p tauri --precise 2.11.1`.
  Chained bumps: `tao 0.34.5→0.35.2`, `tauri-build 2.5.3→2.6.1`,
  `tauri-codegen/macros 2.5.2→2.6.1`, `tauri-runtime{,-wry}
  2.9.x→2.11.1`, `tauri-utils 2.8.1→2.9.1`, `tray-icon 0.21.3→0.23.1`,
  `wry 0.53.5→0.55.1`, plus a few transitive additions
  (`web_atoms`, `string_cache`, `tendril`).

### Added

- **`SECURITY.md`** (Phase 0 of the public-flip plan). Defines the
  vulnerability reporting process (email + 5-business-day ack +
  90-day disclosure), scope (this repo's app + middleware +
  deployment config), supported versions (latest master only), and
  out-of-scope items (social engineering, DoS, third-party services).

## [2.59.2] - 2026-05-10

### Security

- **Phase 3 hardening — CORS allowlist + transport security headers
  + branch type-narrowing** (3-agent parallel batch, cherry-picked
  onto master in order):

  - `81a7530` (was `a55de69`) — `fix(backend): lock CORS to env-driven
    allowlist`. Replaces `CorsLayer::new().allow_origin(Any)` in
    `hotel-backend/src/main.rs` with a `BACKEND_ALLOWED_ORIGINS`
    env-driven list (default `http://localhost:3003,http://web:3003`
    — the only legitimate callers today). Malformed entries panic at
    startup, mirroring the `require_secret` style. Backend port
    isn't externally exposed today, so this is defense-in-depth in
    case it gets exposed later. Documented under `backend` in
    `docker-compose.yml`.

  - `9fb2073` (was `39b9cfc`) — `feat(security): add CSP, HSTS,
    Permissions-Policy response headers`. `next.config.js` now sends:
    - `Strict-Transport-Security: max-age=31536000; includeSubDomains`
      (1y, no `preload` — internal app, off the HSTS preload list).
    - `Permissions-Policy: camera=(), microphone=(), geolocation=(),
      payment=(), usb=(), interest-cohort=()` (denies features the
      app doesn't use, opts out of FLoC).
    - `Content-Security-Policy: default-src 'self'; script-src 'self'
      'unsafe-inline' 'unsafe-eval'; style-src 'self' 'unsafe-inline';
      img-src 'self' data: blob:; font-src 'self' data:; connect-src
      'self'; frame-ancestors 'none'; base-uri 'self'; form-action
      'self'`. The `'unsafe-inline'`/`'unsafe-eval'` in `script-src`
      are required by Next.js client bundles + recharts; tightening
      them needs a nonce-injection pass (deferred). `frame-ancestors
      'none'` matches the existing `X-Frame-Options: DENY`.

### Changed

- `67d0e96` (was `707a7be`) — `refactor(backend): replace
  stringly-typed branch with Branch enum`. Five route query structs
  (`NewCustomersQuery`, `NewBookingsQuery`, `NewRoomsQuery`,
  `InventoryRoomsQuery`, `NewCheckInsQuery`) migrated from
  `branch: Option<String>` to `Option<Branch>`, reusing the existing
  enum from `routes::mode` (already used by `routes::calendar`).
  Serde handles wire-form parsing — invalid branch values now reject
  at request-parse time instead of being silently treated as
  "neither hfhotel nor hfville" deep inside each handler. Five
  `params.branch.as_deref() == Some("hfville")` checks updated to
  `params.branch == Some(Branch::Hfville)`.

  Verified: `cargo check`, `cargo clippy --all-targets -- -D
  warnings` (zero new lints; the 58 pre-existing errors on master
  are out-of-scope), `cargo test --no-run` all 20 binaries compile.

## [2.59.1] - 2026-05-10

### Security

- **Phase 3 hardening — SQL LIKE/equality injection fixed across 7
  files** (audit HIGH-3 from the 2026-05-05 threat model). Previous
  pattern was string concat with `escaped = search.replace('\'',
  "''")` which only neutralised single quotes — leaving `%` and `_`
  LIKE wildcards open for enumeration attacks (`%a%`, `%b%`, etc.) and
  the broader SQL-injection-shaped pattern fragile to any future
  PostgreSQL `standard_conforming_strings` flip.

  Fix landed via 5 parallel agent worktrees, cherry-picked onto master
  in order:
  - `c069d22` `customer.rs` — 5 LIKE patterns on firstname/lastname/phone/email/idcard
  - `1373d70` `checkin.rs` + `room.rs` — status equality filters
  - `7c51d6c` `inventory.rs` — 3 functions (item search, room search, transactions filter)
  - `f72172c` `routes/checkins.rs` + `routes/new_maintenance.rs` —
    WHERE filters AND 5 UPDATE SET sites in the maintenance update
    handler (more involved refactor: tracks `next_param_index` across
    SET clauses, reserves the WHERE `mreq_id` slot last)
  - `40b5f26` `booking.rs` — 5 conditional binds (search + status +
    start_date + end_date + customer_id), the most complex of the batch

  Pattern applied uniformly: `let pattern = format!("%{}%",
  search.replace('\\', "\\\\").replace('%', "\\%").replace('_', "\\_"));`
  then `LIKE $N ESCAPE '\\'`. Equality filters use `= $N`. Date ranges
  use `>= $N::date` / `<= $N::date`. All `sqlx::query(&q).bind(value)`
  with the chained `let q = match &p { Some(v) => q.bind(v), None => q };`
  idiom so bind order matches conditional-add order.

  Integer columns (`customer_id`, `room_id`, `category_id`, etc.) and
  `bool` columns (`active`) are still inlined since their Rust types
  preclude string injection.

  Verified: `cargo check --workspace` clean on each branch + on the
  combined master. Integration tests deferred to CI (no local PG;
  test-backend job exercises the full path).

## [2.59.0] - 2026-05-09

### Security

- **Phase 3 hardening — replace `NEW_DB_PASSWORD` silent fallback with
  `panic!` (via `require_secret`).** `hotel-backend/src/config.rs:109`
  previously did `unwrap_or_else(|_| "REDACTED-sa-pw".to_string())` if the
  env var was missing or empty. The legacy `REDACTED-sa-pw` value matches
  the historical leaked MSSQL `sa` password, so a deploy that dropped
  `NEW_DB_PASSWORD` (CI secret rotation typo, runbook §4a `.env`-rewrite
  pitfall) would silently fall back to that string and may successfully
  connect to a freshly-bootstrapped PG instance using the same default.

  Now uses the existing `require_secret()` helper (same pattern already
  applied to `DB_PASSWORD` and `VILLE_DB_PASSWORD`) which panics at
  config load with a clear message. A misconfigured deploy fails LOUD
  at startup instead of silently routing to a default-credentials DB.

  Also collapsed `hotel-backend/src/bin/migrate_legacy.rs:734-742` —
  was duplicating the env-var read logic with the same `REDACTED-sa-pw`
  fallback. Replaced with `NewDbConfig::from_env()` so both paths share
  the same panic-on-missing-password contract. Same shape as
  `create_legacy_pool` already does for the MSSQL side via `DbConfig`.

  Audit reference: HIGH-2 in the threat model agent's report
  (`a89bc2c9f267b1f2d` session, 2026-05-05).

## [2.58.6] - 2026-05-09

### Removed

- **`scripts/legacy-monitor/` and the corresponding evergreen systemd
  service have been retired.** The directory held an Extended Events
  session that served two purposes during the 2026-04 migration:
  1. Tripwire for the CT/PK enable migrations on legacy MSSQL.
  2. Capture stream for the writeback reverse-engineering spike.

  Both are satisfied. The CT/PK enable went live 2026-04-25 (HF Hotel)
  and 2026-04-30 (HF Ville Phase 5/5.5) with 9+ days of clean post-
  cutover burn-in. The spike findings have been finalised in
  `docs/legacy-spike/findings.md` (700 LOC, the source of truth for
  every writeback recipe). The XE-session log files had grown to
  ~7.4 GB of mostly noise (`error_number = 0` `sql_batch_completed`
  reads) on evergreen and were no longer being consulted.

  Changes:
  - Deleted `scripts/legacy-monitor/` (9 files: 3 SQL setup files,
    `tail-loop.sh`, `start.sh`, `stop.sh`, `check-errors.sh`,
    `check-activity.sh`, `README.md`).
  - Stopped + disabled `legacy-monitor.service` on evergreen, removed
    `~/.config/systemd/user/legacy-monitor.service` and `~/legacy-monitor/`.
  - Dropped the `scripts/legacy-monitor/` line from
    `hotel-backend/.dockerignore`.
  - Updated `docs/architecture.md` §3.7: replaced "Extended Events
    session in `scripts/legacy-monitor/` is the canonical way" with a
    pointer to ad-hoc XE sessions + `docs/legacy-spike/` methodology.
  - Updated `docs/architecture.md` §10 item 8: corrected the
    rollback-script location to `migrations/legacy-mssql/` (which is
    where the apply/rollback files actually live).
  - Dropped the `scripts/legacy-monitor/` companion-doc bullet from
    `CLAUDE.md`.

  Should a future schema change need a similar tripwire, the
  methodology in `docs/legacy-spike/README.md` covers how to spin one
  up ad-hoc — no need to keep the long-running daemon.

## [2.58.5] - 2026-05-09

### Changed

- **`docker-compose.yml`** — `writeback-hfville` service default flipped
  from `WRITEBACK_ENABLED=false` to `true`, mirroring the post-cutover
  default already in place for `sync-hfville` (`LEGACY_SYNC_ENABLED=true`).

  Background: the #76 Ville cutover landed 2026-04-30 but the writeback
  flag was never flipped, so PG canonical state has not been mirrored
  back into Ville's legacy MSSQL since cutover. The Ville legacy app is
  still active at the receptionist desk for nightly billing/reports —
  without writeback those workflows don't see new bookings/checkins
  written through the new system.

  Operator can still set `HFVILLE_WRITEBACK_ENABLED=false` in `.env` to
  pause writeback (e.g. during a Ville-MSSQL incident) and that override
  wins over the new default.

  Mirrors prior pattern: `HFVILLE_LEGACY_SYNC_ENABLED:-true` (set 2026-04-30)
  and `HFVILLE_LEGACY_SYNC_SHADOW_MODE:-false` (set 2026-04-30).

## [2.58.4] - 2026-05-09

### Fixed

- **`hotel-backend/src/bin/sync.rs`** — added a connectivity probe at
  the top of `run_one_tick`. When the legacy WG tunnel flaps, the bb8
  pool's 15s `connection_timeout` was firing once per CT-enabled
  table, turning a 2-minute outage into a ~4-minute (16×15s)
  sequential WARN sweep before the watcher caught up.

  The probe acquires a connection and runs `SELECT 1`. If it fails,
  the whole tick is skipped (one WARN logged); the next tick (1s
  later) re-probes and resumes the moment the tunnel comes back.

  Burn-in evidence (HF Ville sync, 2026-05-05 → 2026-05-08): three
  daily ~2-3 min bursts of the form "CT fetch failed: Timed out in
  bb8" repeated across all 16 tables. Pattern matches WG handshake
  flap on the `10.10.10.4 → MikroTik DNAT → 192.168.11.51,1436` path.
  Watcher self-recovered without restart, but recovery was slow
  (~4 min) and noisy (~16 WARNs per outage). Probe collapses both.

## [2.58.3] - 2026-05-09

### Fixed

- **CT watcher: empty-fetch path now clears stale `last_error` on HF Ville's
  low-traffic / empty CT-tracked tables.** `bin/sync.rs` `poll_table` was
  early-returning on a successful 0-row CT fetch (`if rows.is_empty() {
  return Ok(()) }`) without calling `bump_skipped` to update
  `last_processed_at` and clear `last_error` / `consecutive_failures`.
  Combined with HF Ville's transient WireGuard tunnel flaps that produce
  bb8 connection-pool timeouts on every table in the loop, this left
  five Ville tables permanently STUCK in `legacy_sync_status` with
  `consecutive_failures` in the dozens and a stale "Timed out in bb8"
  `last_error` even though the watcher was in fact polling them
  successfully (just with no CT changes to ingest):

  | Table             | Ville rows | CT rows ever | Why stuck                     |
  |-------------------|-----------:|-------------:|-------------------------------|
  | `HT_Cupon`        |          0 |            0 | Empty table; only failures bump |
  | `HT_Deposit`      |          0 |            0 | Empty table; only failures bump |
  | `HT_Bill_Debt_H`  |          0 |            0 | Empty table; only failures bump |
  | `HT_Bill_Debt_Ds` |          0 |            0 | Empty table; only failures bump |
  | `HT_Receipt_H`    |        101 |            0 | Pre-CT rows; no change events  |

  CT was confirmed enabled with PKs on all 5 tables on Ville's MSSQL
  (`<ville-mssql-host>:1436` / `HOTEL`); migrations 020 + 021 had
  applied correctly during the 2026-04-30 Phase 5/5.5 cutover. Root
  cause was purely an observability bug in the watcher.

  Fix: call `bump_skipped(pg, table, 0, false)` before the early return,
  mirroring what the NoopMapper short-circuit at the top of `poll_table`
  already does. Adds a regression test
  (`empty_fetch_clears_error_via_bump_skipped`) that source-greps for
  the call inside the empty-rows guard region.

  Verification post-deploy: query `legacy_sync_status` on the
  `hotelville` PG and confirm the 5 tables show `consecutive_failures =
  0`, `last_error = NULL`, and a recent `last_processed_at`. No
  Ville-side MSSQL changes required — receptionist coordination NOT
  needed.

## [2.58.2] - 2026-05-09

### Removed

- **Vestigial `ville_sync` references scrubbed from code comments.** The
  `bin/ville_sync.rs` binary was retired on 2026-04-30 (task #77) when HF
  Ville's MSSQL was upgraded to SS2025 Express, allowing the unified
  `bin/sync.rs` CT watcher to serve both HF Hotel and HF Ville via
  per-site env (`SITE_ID`). The binary, its container service, and its
  `Cargo.toml` `[[bin]]` entry were removed at that time. Three stale
  comments survived and are now scrubbed:
  - `hotel-backend/src/main.rs` — dropped the "formerly fed by the
    retired ville_sync FreeTDS poll" parenthetical from the HF Ville
    pool comment block.
  - `hotel-backend/Dockerfile` — replaced the `build-ville-sync`
    sibling-build example with the current real pair
    (`build-frontend vs build-backend`).
  - `hotel-backend/.env.example` — dropped `+ ville-sync` from the
    `MSSQL_POOL_MAX_SIZE` description, leaving `(writeback + sync)`.

  Cosmetic only; no behaviour change, no build/runtime impact.

## [2.58.1] - 2026-05-06

### Added

- **`docs/runbook-history-rewrite.md`** — runbook for the eventual git
  history rewrite step (Phase 6.5 of the public-flip plan). Covers
  pre-flight checklist, `git filter-repo` invocation with replacement
  text format, force-push procedure, maintainer re-clone instructions,
  rollback path, and explicit "this is hygiene, not security — Phase 3
  rotation is what kills the credential" framing.

  Document only — no execution. The rewrite is gated on Phase 3 secret
  rotation being verified working, and shouldn't run unless we're
  committed to flipping public soon after (force-push cost is only
  worth paying when it unblocks the flip).

  Tool of choice documented as `git filter-repo` over BFG: more modern,
  better regex support, GitHub-recommended, no JVM dependency. BFG
  fallback procedure also included for operators who already have it
  installed.

## [2.58.0] - 2026-05-05

### Removed

- **Phase 2 sanitization landed** — pre-public-flip cleanup of the repo
  surface. Three stages, each its own commit:
  - **Stage A** (`44683df^`): deleted `legacy-reference/` entirely
    (35 MB, 466 files). Was decompiled iHOTEL2025 vendor source +
    commercial DLLs (DotNetBar, C1FlexGrid, Office Interop,
    ThaiNationalIDCardByKP). Distributing decompiled source of a paid
    commercial competitor product on a public repo is a copyright/EULA
    exposure that's bigger than any PII finding in the audit.
    Analytical value is preserved in `docs/legacy-spike/` SQL recipes,
    `docs/architecture.md` writeback explanations, and the writeback
    code itself.
  - **Stage B** (`44683df`): bulk sed replacing internal IPs + hostnames
    across 41 docs/scripts/migration files (2400 line replacements).
    Mappings: `192.168.100.222 → <legacy-mssql-host>`,
    `192.168.11.51 → <ville-mssql-host>`, `10.10.10.x → <wg-*>`,
    `DESKTOP-* / FRONT2 → <legacy-host>` etc. Workflow YAML and
    `docker-compose.yml` deliberately untouched (need real values for
    runtime). `hotel-backend/src/config.rs:46` fallback string also
    sanitized; Phase 3 will replace with `panic!` to remove the
    fallback entirely.
  - **Stage C** (`6858501`): prose-level fixes the bulk sed couldn't
    cover. "MikroTik" → "the edge router" (don't reveal firewall
    vendor), VLAN role descriptions genericized, real customer name
    `<REDACTED-real-guest-name>` (was a Thai full name) redacted from
    `docs/legacy-spike/findings.md`, XE capture outputs, and the
    `mark_clean.rs` doc comment.

  Still in repo (deliberate):
  - `evergreen.thehfhotel.org` in workflow YAML (needed for runtime;
    hostname is publicly resolvable anyway)
  - Real IPs in `docker-compose.yml` env defaults (runtime-required)
  - `lib/hotel-info.ts` legal-entity values — Thai tax IDs are
    DBD-public; Phase 3 will decide whether to env-var-ize
  - CHANGELOG historical tax-ID mentions — same as above

  Not yet done (separate from this working-tree pass):
  - Git history rewrite to remove sanitized blobs from prior commits.
    Live secrets there will be rotated in Phase 3 anyway, so the
    rewrite is a flip-day step rather than now.

## [2.57.3] - 2026-05-05

### Fixed

- **Phase 1 ghcr auth gap**: the soak-substitute deploy (v2.57.2) failed
  because evergreen's docker daemon couldn't pull the private GHCR
  images — `denied: denied` on
  `ghcr.io/v2/thehfhotel/new-hotel-backend/manifests/latest`. Old
  workflow logged docker into ghcr ON evergreen via `docker/login-action`
  (because the runner WAS evergreen); the new GH-hosted runner logs
  itself in but evergreen never gets the auth.

  Fix: pass `GHCR_USER` + `GHCR_TOKEN` (= `${{ github.actor }}` +
  `${{ secrets.GITHUB_TOKEN }}`) in the JSON payload's new `ghcr`
  block. `/srv/run-deploy.sh` does `docker login ghcr.io
  --password-stdin` once at start of run, before `compose pull`. Token
  expires when the workflow run ends, so no long-lived ghcr credential
  lives on evergreen.

  This was the only path the Phase 1 first-deploy (v2.57.1) didn't
  exercise — that deploy didn't actually pull a new image (frontend
  hadn't changed in a way that produced a new SHA), so compose's pull
  step silently returned "no work needed" without authenticating.
  v2.57.2 was the first one that needed a fresh pull (new backend image
  from migration 026), which surfaced the auth gap.

## [2.57.2] - 2026-05-05

### Added

- **Phase 1 soak-substitute test (#tests 1+2)** — `migrations/pg/026_phase1_soak_no_op.sql`
  is a pure no-op migration (`SELECT 1 WHERE FALSE`) that triggers the
  full backend deploy path:
  - `changes` filter sees `migrations/pg/**` → backend filter fires
  - `init-db-migrations-drift-check` runs (deploy filter also fires) →
    must pass
  - `test-backend` runs → must pass
  - `build-backend` runs → produces a new image SHA (functionally
    identical, but recreates the container on deploy)
  - `deploy` step runs migrate.sh → applies migration → seeds
    schema_migrations row
  - Backend container is force-recreated (or recreated by image-SHA
    change), healthcheck verifies it stays healthy

  All paths the v2.57.1 deploy DIDN'T exercise (it was workflow + script
  + docs only). Combined with manual flock + rollback drills already
  done, this substitutes for the 2-week natural-traffic soak window
  the runbook originally proposed.

  Migration is safe to leave permanently or remove later — `SELECT 1
  WHERE FALSE` returns no rows and modifies nothing. If removed, the
  schema_migrations row stays (benign; migrate.sh tolerates extra rows).

## [2.57.1] - 2026-05-05

### Fixed

- **Phase 1 first-deploy bug: snap-docker can't see `/srv/`.** The first
  CI run via the new SSH path (`4ccaca3`) failed with
  `compose pull → no configuration file provided: not found` despite the
  file being present at `/srv/new-hotel-production/docker-compose.yml`
  with correct perms. Root cause: the snap-confined `docker` package on
  evergreen has the standard `home` interface, which only grants access
  to `/home/`, `/media/`, `/mnt/` — `/srv/` is outside the confinement
  allowlist. The runbook's "move dir to `/srv/`" step was wrong for this
  host's docker installation.

  Fix: `DEPLOY_DIR` moved to `/home/deploy/new-hotel-production`. The
  symlink at `/home/nut/new-hotel-production` now points there for
  muscle-memory parity. The deploy script (`/srv/run-deploy.sh` itself
  stays root-owned at /srv — bash isn't snap-confined) updated. Runbook
  3b updated with a heads-up about the snap confinement constraint so
  future operators don't hit the same trap.

  All other Phase 1 work (SSH key + forced-command + cloudflared
  transport + JSON payload + script logic) is unchanged. Pure DEPLOY_DIR
  path fix.

## [2.57.0] - 2026-05-05

### Changed

- **Phase 1 CI/CD modernization landed: workflow refactored to GitHub-hosted
  runners + SSH-based deploy.** Replaces the v2.54.31 self-hosted-runner
  setup that ran ALL jobs on evergreen and inlined ~250 lines of deploy
  shell in the workflow YAML.

  All 6 jobs (`changes`, `test-frontend`, `test-backend`,
  `init-db-migrations-drift-check`, `build-frontend`, `build-backend`,
  `deploy`) now `runs-on: ubuntu-latest`. Build/test jobs run in parallel
  again (~4 min wall vs. ~12 min on the serial self-hosted runner). No
  more snap-docker network flap blocking builds.

  Deploy job is rewritten end-to-end. New flow:
  1. Install `cloudflared` on the runner (curl from GitHub releases)
  2. Install ed25519 SSH key + pinned host key from GH Secrets
     (`EVERGREEN_DEPLOY_SSH_KEY`, `EVERGREEN_HOST_KEY`)
  3. tar up deploy artifacts (`docker-compose.yml`, `init-db/`,
     `migrations/pg/`, `scripts/migrate.sh`)
  4. `jq -n` builds JSON payload: `{commit_sha, deploy_payload_b64,
     env: {...all secrets...}}`
  5. Pipe to `ssh deploy@evergreen.thehfhotel.org` over the existing
     `asgard` cloudflared tunnel (no CF Access app gates the hostname,
     so no service token needed — verified via CF Access API)
  6. Forced-command in `deploy@`'s `authorized_keys` runs
     `/srv/run-deploy.sh` (root-owned, mode 755). The SSH key cannot
     execute anything else.

  Compared to the old setup:
  - **No more "the runner IS prod"**: workflow can no longer shell out
    arbitrary commands on evergreen
  - **No password SSH**: hardening fragment in
    `/etc/ssh/sshd_config.d/00-hardening.conf` disables
    `PasswordAuthentication`, `KbdInteractiveAuthentication`,
    `ChallengeResponseAuthentication`
  - **No root deploys**: `deploy` user is in the `docker` group; nothing
    else
  - **Snap-docker reliability isolated**: still flaky, but only the deploy
    step on evergreen sees it (and the `retry_compose` helper in
    `/srv/run-deploy.sh` masks it). Build/test on github-hosted is
    pristine. Snap → apt CE migration becomes optional, not blocking.

  Verified pre-merge:
  - SSH transport tested end-to-end (Phase 6a in runbook): cloudflared
    tunnel → SSH key auth as `deploy` → forced-command runs script →
    JSON validation rejects malformed payload cleanly. Proves auth
    + transport + script all work; full deploy via this commit is the
    integration test.

  Self-hosted runner stays REGISTERED but disabled for the next 2 weeks
  as the rollback path. After 14 days of green deploys via the new flow,
  fully deregister per `docs/runbook-deploy-modernization.md` step 8.

  Setup recipe captured in `docs/runbook-deploy-modernization.md` (added
  in v2.56.5). All steps 2–6 of that runbook are complete; this commit
  is step 7.

## [2.56.5] - 2026-05-05

### Added

- **Phase 1 CI/CD modernization artifacts** — `scripts/deploy/run-deploy.sh`
  + `docs/runbook-deploy-modernization.md`. Documents and implements the
  upgrade from "self-hosted runner IS prod, deploy as `nut` with full sudo"
  to "GH-hosted runner SSHes via Cloudflare Access service token to a
  dedicated `deploy` user, whose authorized_keys forces a single root-owned
  script."

  Files added are passive — `.github/workflows/docker-build.yml` is
  intentionally NOT changed in this version. The workflow swap is a
  separate follow-up (step 7 of the runbook) that only happens after the
  evergreen-side setup (steps 1–6) is verified working manually. So the
  current self-hosted-runner pipeline keeps running unchanged; the new
  files are reference + future use.

  Hardening applied vs. typical SSH-deploy patterns: `restrict` directive
  + forced-command in `authorized_keys` (deploy key can ONLY trigger the
  script), `flock`-based mutex against concurrent deploys, scoped `umask
  077` only around `.env` write, payload size cap at 16 MB to bound OOM
  risk, `KbdInteractiveAuthentication no` alongside `PasswordAuthentication
  no` in a fragment file under `sshd_config.d/`, CF Access service-token
  credentials passed via `TUNNEL_SERVICE_TOKEN_*` env vars (not CLI flags
  — keeps secrets out of `/proc/<pid>/cmdline`).

  Independent code review surfaced 4 must-fix issues (umask placement,
  `/var/log/deploy` ownership, sed regex BRE/ERE confusion, missing
  `KbdInteractiveAuthentication`) — all addressed before this commit.

  Phase 1 is one part of a broader plan (Phases 0–9) toward eventually
  flipping the repo public; this is the first concrete deliverable.

## [2.56.4] - 2026-05-03

### Changed

- **Bumped `tailwindcss` from `3.4.17` to `4.2.4`** (major version 3 to 4
  migration). Ran the official `npx @tailwindcss/upgrade` tool, which:
  - Replaced `tailwind.config.ts` with CSS-based `@theme { ... }` block
    inside `app/globals.css`. The brand palette, SAP Fiori shell tokens,
    13px-base type scale, and squashed border-radius values are all
    preserved as `--color-*`, `--text-*`, and `--radius-*` custom
    properties. The legacy `tailwind.config.ts` is deleted.
  - Switched `app/globals.css` from `@tailwind base/components/utilities`
    directives to the v4 `@import 'tailwindcss';` single-line import.
  - Migrated `postcss.config.js` from the standalone `tailwindcss`
    plugin to `@tailwindcss/postcss` (new in v4). Removed
    `autoprefixer` from `devDependencies` — Tailwind v4's PostCSS
    plugin includes vendor-prefixing built in via Lightning CSS.
  - Renamed deprecated v3 utility classes across 37 component/page
    files: `shadow-sm` → `shadow-xs` (10 sites), `outline-none` →
    `outline-hidden` (68 sites), `backdrop-blur-sm` → `backdrop-blur-xs`
    (1 site), `flex-shrink-0` → `shrink-0` (29 sites). All are 1:1
    behaviour-preserving renames per the Tailwind v4 release notes.
  - Added a v4-compat `@layer base` block to `app/globals.css` that
    pins the default border colour to `var(--color-gray-200)`, since
    v4 changed the default border colour to `currentcolor`. This keeps
    every existing `border` utility looking identical to v3.
- Updated `__tests__/components/StatsCard.test.tsx` and
  `__tests__/components/Charts.test.tsx` to assert the new `shadow-xs`
  class name (was `shadow-sm`). All 621 component tests pass.
- Supersedes Dependabot PR #38 (which proposed the same 3.4.19 → 4.2.4
  bump but without the upgrade-tool migration steps).

**Visual regression risk** — Tailwind v4 may produce subtly different
output beyond the explicit utility renames (shadow, ring colour,
gradient interpolation now in OKLab, border colour). The compat shim
covers border-color; other defaults could shift. Manual `pnpm dev`
walkthrough recommended before considering this fully verified.

## [2.56.3] - 2026-05-03

### Changed

- **Bumped `recharts` from `^2.15.0` (was 2.15.4 installed) to `^3.8.1`** to
  pick up the v3 line. Supersedes the dependabot proposal in PR #41 (which
  also targeted 3.8.1 — this branch tracks the same version but ships the
  required v3 API migrations rather than just bumping the lockfile).

  Two chart-bearing files were touched for the v3 breaking changes:

  - `components/Charts.tsx:340-344` — `<Pie label>` callback signature
    changed in v3. The callback now receives a `PieLabelRenderProps` object
    where the source datum lives under `props.payload`, not flattened onto
    the root. Rewrote the inline `({ roomType, percentage }) => …` arrow
    to read `props.payload` and guard the `undefined` case before
    formatting `${roomType} (${percentage}%)`.

  - `app/reports/page.tsx:401-405` — same `<Pie label>` signature fix,
    typed against the local `RoomTypeRevenue` interface.

  - `app/reports/page.tsx:460` — `<Tooltip formatter>` now types its
    `value` parameter as `ValueType | undefined` instead of `number`.
    Dropped the `(value: number)` annotation, let TS infer, and added a
    `value ?? 0` fallback so the existing `${value} รายการ` string
    rendering is preserved.

  Tooltip/Legend custom-content components were unaffected (we already
  type props inline via destructuring, not through the renamed
  `TooltipProps` → `TooltipContentProps` import). No use of removed v3
  props (`activeIndex`, `Pie blendStroke`, `Legend payload`,
  `Reference alwaysShow`/`isFront`, `Area animateNewValues`). Multiple
  YAxis ordering change does not affect `RevenueChart` (default `yAxisId`
  `0` sorts before custom `right`, matching prior visual order).

  All 621 component tests pass; production build succeeds. Visuals on
  the reports page were not verified in a real browser by this change —
  reviewer should spot-check the pie-slice labels and the line-chart
  tooltip on `/reports` once deployed to dev.

## [2.56.2] - 2026-05-03

### Changed

- **Bumped `lucide-react` from `^0.469.0` to `^1.14.0`.** This is the
  upstream's first stable v1 release (Oct 2026) plus all subsequent
  v1.x patches. Supersedes Dependabot PR #43 (which only reached 1.14.0
  by aiming at the same target).

  Audit results from this repo:
  - 84 unique icon imports across 53 frontend files (`app/`,
    `components/`). Every single one still exists in v1.14.0 — no
    renames required, no code changes to import statements.
  - The "numbered alias" icons we rely on (`BarChart3`, `CheckCircle2`,
    `Edit2`, `Edit3`, `Grid3X3`, `Loader2`) are all still exported.
  - The v1.x release notes' renames (`text-select` →
    `square-dashed-text`, etc.) do not touch any icon we use.

  What does change between 0.469 and 1.x — but does not require code
  changes from us:
  - Brand icons removed (we use none).
  - `aria-hidden="true"` is now set on every icon by default (better
    a11y; visual unchanged).
  - UMD build dropped, ESM/CJS only (we consume via the bundler so this
    is a no-op).
  - New context-provider API for setting default props app-wide
    (available; not yet adopted).

  Verified locally: `pnpm test:components` → 621/621 passing,
  `pnpm build` → green with no TypeScript errors.

## [2.56.1] - 2026-05-03

### Changed

- **Bumped React + React DOM to 19.2.5** (from `^19.1.0`) and matching
  type packages `@types/react` to `^19.2.14` and `@types/react-dom` to
  `^19.2.3`. Latest 19.x stable line; 19.2.5 lands additional cycle
  protections for React Server Components. Compatible with our pinned
  `next@16.2.4` (peer requires `^19.0.0`) and all React-consuming deps
  (`@testing-library/react`, `react-datepicker`, `recharts`,
  `lucide-react`). Supersedes Dependabot PR #40 which only bumped
  `react` + `@types/react` and left `react-dom` / `@types/react-dom`
  out of lockstep. All 621 component tests pass; `pnpm build` clean.
## [2.56.0] - 2026-05-01

### Changed

- **Backend: bumped `axum` 0.7.9 → 0.8.9** (latest stable, supersedes
  Dependabot PR #32 which proposed the same bump but without applying
  the required code migration). Pulled `axum-core` 0.4 → 0.5 and
  `tower-http` 0.5 → 0.6 in lockstep so they share the same `http`
  1.x family. Lockfile re-resolved, `cargo check --workspace --locked`
  clean, all 342 unit + bin tests pass; integration tests defer to
  CI's postgres service container as usual.

  **Breaking change handled — path-parameter syntax migration**
  (`matchit` 0.7 → 0.8, axum #2645): every route declared with the
  old `/:param` / `/*rest` syntax now panics at startup. Migrated
  all 24 affected routes in `hotel-backend/src/main.rs` (`/api/...`)
  to the new `{param}` form, e.g.:

  ```diff
  - .route("/api/new/checkins/:id/guests/:guest_id", delete(...))
  + .route("/api/new/checkins/{id}/guests/{guest_id}", delete(...))
  ```

  No call-site changes were needed inside route handlers — the
  `Path<T>` extractor and `#[derive(Deserialize)]` field names work
  identically; only the `route()` literal had to change. Frontend
  HTTP clients are untouched (they already issue concrete URLs like
  `/api/new/checkins/123`, not pattern strings).

  No `axum-extra`, `axum-macros`, websocket, multipart, or `Host`
  extractor usage in the codebase, so the other 0.8 breaking
  changes (Host moved to `axum-extra`, `Option<Path>` rejection
  semantics, `WebSocket::close` removal, `axum::extract::ws::Message`
  switching to `Bytes`/`Utf8Bytes`, mandatory `Sync` on handlers)
  required no edits. `cargo check`, `cargo test --no-run`, and the
  unit + bin test runs all passed without warnings introduced by
  this bump.

  No `sqlx::query!()` SQL strings changed, so the `.sqlx/` offline
  cache is still valid — no `cargo sqlx prepare` rerun needed; the
  CI image build with `SQLX_OFFLINE=true` will continue to work.

## [2.55.2] - 2026-05-02

### Fixed

- **CI deploy: retry-wrapped `docker login` and `docker compose pull`
  in the `deploy` job** to mask the snap-docker network-shim flap on
  evergreen. After 4 manual `gh run rerun` cycles to land the v2.55.1
  deploy, the daemon-log evidence pointed at snap confinement (not the
  network or ghcr.io itself):

  ```
  docker.dockerd[1385115]: Handler for POST /v1.54/auth returned error:
    Get "https://ghcr.io/v2/": dial tcp 20.205.243.164:443:
    connect: network is unreachable
  ```

  ~30 % of outbound TCP attempts from snap-confined dockerd return
  `ENETUNREACH` immediately while `curl` from the same shell to the
  same IP works fine — a classic snap-mount-namespace artifact.
  Also visible in dmesg: `error="copy shim log" error="read
  /proc/self/fd/138: file already closed"` and `write unix
  /var/run/docker.sock->@: write: broken pipe` during container churn.

  The proper fix is to migrate from `snap install docker` (canonical
  29.3.1) to apt Docker CE (`get.docker.com`). That's a planned
  ~45-minute maintenance window with `pg_dump newdb` insurance — too
  risky to do under CI pressure.

  Workflow-only band-aid: 5 attempts with 5/10/15/20s linear backoff
  on every ghcr.io operation in the `deploy` job. Covers the worst
  flap window observed (~60 s). Build jobs (`build-frontend`,
  `build-backend`) still use the unwrapped `docker/login-action`
  because they use docker buildx, which has a different network path
  and hasn't hit the issue. If they start failing, extend the same
  pattern there.

  - `Log in to Container Registry` step: replaced `docker/login-action`
    call with an inline `for attempt in 1..5` loop wrapping
    `echo $GHCR_TOKEN | docker login ghcr.io --password-stdin`.
  - `Deploy` step: added `retry_compose_pull()` bash function near
    `wait_healthy()`, then swapped all 4 `docker compose pull` calls
    to use it (top-level pull + 3 profile-scoped pulls for writeback,
    sync, and the hfville pair).

  Net effect: deploys self-heal through the snap-docker flap. Each
  failed attempt logs a `::warning::` so we can still see the
  underlying issue is not gone, just papered over until the CE
  migration.

## [2.55.1] - 2026-05-02

### Fixed

- **CI: two leftover bugs from the v2.54.31 self-hosted-runner migration
  surfaced the moment a deploy-trigger commit (#44) needed both
  `test-backend` and `init-db-migrations-drift-check` to pass.** Both
  jobs failed pre-deploy, blocking the ville_sync retirement
  rollout from reaching evergreen. Production was unchanged (gate
  worked as designed); fix below restores the pipeline.

  **Bug 1 — `test-backend` step ordering.** The "Install build + test
  prerequisites" step (which apt-installs `postgresql-client`) was
  written AFTER the "Initialize database" step that shells `psql`. On
  github-hosted ubuntu-latest the order didn't matter because the
  image ships psql; on the self-hosted runner the apt-install is the
  only source. Reordered so prereqs install first.

  **Bug 2 — `init-db-migrations-drift-check` container-name collision.**
  The service-container `--name new-hotel-db` clashes with the prod
  `new-hotel-db` PostgreSQL container that lives on the same evergreen
  host as the runner. Docker daemon refused to create the second
  container with that name, killing the job before it could even start
  Postgres. Renamed the service container to `drift-check-db` (unique)
  and set `DB_CONTAINER=drift-check-db` on the migrate.sh step so the
  script targets the right container — `migrate.sh` already supports
  the env-var override (`DB_CONTAINER="${DB_CONTAINER:-new-hotel-db}"`),
  so prod behaviour is unchanged.

  Both fixes are within the workflow file only; no code, schema, or
  migrate.sh changes were needed.

## [2.55.0] - 2026-05-02

### Removed

- **Phase 7 — retired the legacy `ville_sync` FreeTDS hash-polling
  infrastructure (task #77).** Phase 5 Ville cutover (#76, 2026-04-30)
  repointed the backend's `ville_pool` from the old `hotelnew.ville.*`
  cache to the new `hotelville` PG database (fed by the central
  `sync-hfville` Change-Tracking watcher). Strangler-pattern soak
  defined in ADR 0001 Q5 (~1 week of clean cutover operation) was
  shortened to 48 h after evidence proved acceptable: 21 WARN-level
  bb8 connection timeouts in 48 h (two transient bursts on 2026-05-01
  09:05 UTC and 2026-05-02 05:55 UTC, all 16 tables hit in sequence
  consistent with brief Ville MSSQL connectivity drops), zero ERROR
  events, watcher recovers automatically each retry cycle, watermark
  advanced cleanly from 651 → 1880 (1200+ events processed). The
  `ville-sync` container on the Ville jumpbox would have hit the same
  transients (same MSSQL endpoint over the same WireGuard path), so
  keeping it adds no resilience — only operational surface area.

  Files deleted:
  - `hotel-backend/src/bin/ville_sync.rs` — the FreeTDS-based
    hash-polling worker (1299 lines).
  - `hotel-backend/Dockerfile.ville-sync` — separate image build for
    that bin (82 lines).
  - `deploy/hfville/` — entire jumpbox compose stack
    (`docker-compose.yml` + `init-db/init-hfville.sql`, 172 lines).

  `hotel-backend/Cargo.toml` drops the `[[bin]] ville_sync` entry. No
  dependencies were removable: `ville_sync.rs` only used crates
  (`chrono`, `sha2`, `sqlx`, `tokio time`) that are still used by the
  remaining workers (`bin/sync.rs`, `bin/writeback.rs`,
  `writeback/fingerprint.rs`, etc.).

  `.github/workflows/docker-build.yml` drops the `build-ville-sync` and
  `deploy-hfville` jobs, the `hfville` paths-filter entry, and the
  `cache-from: type=gha,scope=ville-sync` GHA cache scope. The Ville
  jumpbox is no longer addressed by CI; the `room-daily-reporter`
  workload it hosts (different project) is unaffected.

  `docker-compose.yml` drops the `<wg-self>:5441:5439` host-port
  publish on `newdb` — that mapping existed solely to expose PG to the
  Ville jumpbox over WireGuard for the `ville_sync` push.

  `migrations/pg/025_drop_ville_schema.sql` (NEW) drops the now-orphaned
  `ville` schema in `hotelnew` via `DROP SCHEMA IF EXISTS ville
  CASCADE`. Migration 010 stays in the repo for archaeology; 025 is its
  rollback companion. `init-db/init-hotelnew.sql` is updated: fresh
  deploys skip migration 010's DDL outright (no point creating tables
  we'd immediately drop) and seed both `010` + `025` rows in
  `schema_migrations` so the init-db ↔ migrations drift-check stays
  green.

  GitHub Secrets to be cleaned up manually (pipeline no longer
  references them; flagged for operator deletion in the GH Secrets UI):
  - `HFVILLE_SSH_KEY`
  - `HFVILLE_SSH_HOST`
  - `HFVILLE_SSH_USER`
  (`HFVILLE_PG_PASSWORD` and `HFVILLE_MSSQL_PASSWORD` are also no
  longer used by this pipeline, but verify no other workflow references
  them before removal.)

  Doc updates: `CLAUDE.md`, `docs/architecture.md`,
  `hotel-backend/README.md`, `hotel-backend/.dockerignore`,
  `hotel-backend/src/{main.rs,config.rs,db/pool.rs}`, and
  `.github/dependabot.yml` had stale `ville_sync` / `ville-sync`
  references rewritten to reflect the post-cutover topology
  (HF Ville now uses the same `bin/sync.rs` CT watcher with per-site
  env). `docs/runbook-cutover-hfville.md` left untouched (historical
  cutover playbook).

## [2.54.33] - 2026-04-30

### Changed

- **Bumped remaining Node.js 20 actions to node24-runtime majors.** The
  v2.54.32 bump only cleared two actions (the only ones called out in
  the prior run's annotation); the next run surfaced six more still on
  Node 20. Bumping all of them now to clear the deprecation in one
  pass:

  | Action | From | To | Breaking? |
  |--------|------|----|-----|
  | `actions/setup-node` | v4 | v6.4.0 (`48b55a0…`) | New `package-manager-cache` opt-in auto-detects from `package.json`'s `packageManager` field — we don't have that field, so behaviour is unchanged. Existing `cache: 'pnpm'` input still works. |
  | `pnpm/action-setup` | v4 | v5.0.0 (`b307475…`) | Runtime-only. |
  | `docker/build-push-action` | v6 | v7.1.0 (`bcafcac…`) | Removed `DOCKER_BUILD_NO_SUMMARY` and `DOCKER_BUILD_EXPORT_RETENTION_DAYS` env vars + legacy export-build summary tool. We don't set/use any of these. |
  | `docker/login-action` | v3 | v4.1.0 (`4907a6d…`) | Runtime + ESM only. |
  | `docker/metadata-action` | v5 | v6.0.0 (`030e881…`) | List inputs preserve `#` inside values. Our `tags:` / `labels:` blocks have no `#`. |
  | `docker/setup-buildx-action` | v3 | v4.0.0 (`4d04d5d…`) | Removed deprecated inputs/outputs. We don't set any inputs. |

  All bumps require Actions Runner v2.327.1+. The `evergreen` runner
  successfully ran v2.54.32 (which did require v2.327.1+ for the
  checkout v5 bump), so it's already at or past that bar.

  Pinned by SHA per Batch D. The previously-pinned `dtolnay/rust-toolchain`,
  `mozilla-actions/sccache-action`, `Swatinem/rust-cache`, and
  `webfactory/ssh-agent` weren't in the warning list — either already
  on Node 24 or not Node-based.

## [2.54.32] - 2026-04-30

### Changed

- **Bumped `actions/checkout` v4 → v5.0.0 and `dorny/paths-filter` v3 →
  v4.0.1.** Both releases are runtime-only updates that move the action
  from Node.js 20 to Node.js 24, clearing the deprecation warning that
  surfaced on the first self-hosted CI run (run `25182166656`):

  > Node.js 20 actions are deprecated. Actions will be forced to run
  > with Node.js 24 by default starting June 2nd, 2026. Node.js 20 will
  > be removed from the runner on September 16th, 2026.

  No syntax / API changes either side. Both pinned by commit SHA per
  Batch D convention (the `# vX.Y.Z` comment is the human-readable
  reference; SHA is the trust anchor).

  - `actions/checkout` → `93cb6efe18208431cddfb8368fd83d5badbf9bfd`
    (v5.0.0). Requires runner v2.327.1+; self-hosted runners
    auto-update by default and the `evergreen` runner had just
    succeeded the v2.54.31 build, so this is met.
  - `dorny/paths-filter` → `fbd0ab8f3e69293af611ebaee6363fc25e6d187d`
    (v4.0.1).

## [2.54.31] - 2026-04-30

### Changed

- **CI jobs moved off GitHub-hosted runners onto the existing self-hosted
  runner on evergreen.** Driver: GitHub Actions Free-tier minute quota was
  exhausted (push `6fe875c` blocked with "recent account payments have
  failed or your spending limit needs to be increased"). User explicitly
  rejected pay-as-you-go to keep CI cost predictable; self-hosted runner
  minutes don't count against Actions billing, so `spending limit = $0`
  + `runs-on: [self-hosted, linux, deploy]` everywhere = $0 forever, with
  no risk of a surprise invoice.

  Edits in `.github/workflows/docker-build.yml`:
  - `changes`, `test-frontend`, `test-backend`,
    `init-db-migrations-drift-check`, `build-frontend`, `build-backend`,
    `build-ville-sync` now all run on `[self-hosted, linux, deploy]`,
    same label the existing `deploy` + `deploy-hfville` jobs already use.
  - `Install mold linker` step (in `test-backend`) renamed to
    `Install build + test prerequisites` and extended with
    `postgresql-client` since the self-hosted runner doesn't ship `psql`
    by default the way the GitHub-hosted ubuntu-latest image did.
    `apt-get install -y` is idempotent so re-runs on a warm runner skip
    already-installed packages.
  - `init-db-migrations-drift-check` now installs `postgresql-client`
    too, for the same reason — it shells `psql` to seed the throwaway
    Postgres before running `migrate.sh`.

  Trade-offs accepted:
  - **Single point of failure**: evergreen down = no CI. Today CI worked
    even when evergreen was down. Acceptable because evergreen also
    hosts every container CI deploys to; if it's down, deploys can't
    land anyway.
  - **Serial execution**: a single runner now serializes all CI jobs on
    one host. Wall time goes from ~4 min (5 jobs in parallel on
    GitHub-hosted) to ~10–15 min. Acceptable for current change cadence.
  - **Resource competition**: Rust full rebuilds use ~4 cores at 100%
    for ~2 min and can pressure the prod containers (backend, web,
    newdb, writeback, sync, sync-hfville, writeback-hfville) on the
    same host. Watch for backend healthcheck flapping during builds;
    if it becomes a problem, register a second runner on a separate VM
    for builds and keep the existing one for deploy-only.
  - **Self-hosted runner security**: GitHub recommends self-hosted only
    for private repos because PRs from forks could otherwise execute
    untrusted code. This repo is private — fine.
  - **Disk pressure**: cargo `target/`, pnpm store, Docker buildkit
    layers accumulate on the runner. Add a periodic cleanup if disk
    usage on evergreen creeps up (`docker system prune --volumes`,
    `cargo cache --autoclean` if the binary is installed).

## [2.54.30] - 2026-04-30

### Fixed

- **Phase 5.5b Ville (#80): `docker-compose.yml` allowlist default flipped
  to empty so post-021 state survives CI deploys.** Phase 5 originally
  pinned `LEGACY_SYNC_TABLE_ALLOWLIST` for `sync-hfville` to the 11
  canonical-sync tables enabled by migration 020 — anything else (the
  6 mirror tables) would have failed CT polling. Now that 021 has been
  applied at Ville and all 16 tables are CT-enabled, the watcher needs
  to see them all. Operator hot-fix on evergreen cleared the env var,
  but compose's hardcoded 11-table default kept reasserting itself on
  every CI deploy (CI rewrites `.env` from GH secrets, so the operator
  edit didn't survive). Default flipped from the 11-table list to `""`
  so empty = "all CT-enabled tables", matching post-Phase-5.5b state.
  An operator can still narrow via `HFVILLE_LEGACY_SYNC_TABLE_ALLOWLIST=HT_X`
  for debugging.

  Phase 5.5b was applied during the same session: 021 DDL ran cleanly
  at Ville (all 6 mirror tables show `ct_enabled=YES` with proper PKs),
  bootstrap-mirror snapshot loaded HT_Cupon/HT_CheckIn_Product (40)/
  HT_Deposit/HT_Changed_Room (171)/HT_Bill_Debt_H/HT_Bill_Debt_Ds and
  stamped watermark 643. Closes #80, #81. Task #82 (mount
  `LegacyMirrorPanels` in billing detail) was already done in commit
  `fde9dc1` (v2.53.0, 2026-04-29) — no code change needed.

## [2.54.29] - 2026-04-30

### Fixed

- **Real legal-entity values filled into `lib/hotel-info.ts`** for both
  branches. HF Hotel was previously hardcoded to `123 ถนนตัวอย่าง`
  ("123 Sample Road") + `02-123-4567` + `0REDACTED-sa-pw9012` — placeholders
  that pre-dated this repo's billing module and somehow never got
  replaced. HF Ville was the placeholder set the #91 agent shipped on
  2026-04-30 (`[HF Ville — info pending]` etc.).

  Both sites now carry their actual production values:
  - HF Hotel: from legacy MSSQL `TB_SETTINGS` table — address `33 ถนนชนเกษม ต.ตลาด อ.เมืองสุราษฎร์ธานี จ.สุราษฎร์ธานี 84000`, phone `077313808`, taxId `0845557000341`.
  - HF Ville: provided by operator — address `196/6 หมู่ 5 ตำบลมะขามเตี้ย อำเภอเมืองสุราษฎร์ธานี จังหวัดสุราษฎร์ธานี 84000`, phone `077275838`, taxId `0845557000341` (same legal entity as HF Hotel — `บริษัท สายชล เฮอริเทจ จำกัด`).

  Display `name` field uses the brand string (`HF Hotel` / `HF Ville`)
  rather than the registered company name, matching how the staff and
  guests refer to each property. If invoice templates need to print the
  legal entity (e.g. "นิติบุคคล: บริษัท สายชล เฮอริเทจ จำกัด"), that's
  a separate template-level addition, not a `hotel-info` field.

  Source for HF Hotel values is `<legacy-mssql-host> / db / TB_SETTINGS`
  rendered via `sqlcmd`; the legacy app and the receptionist printers
  read the same row.

## [2.54.28] - 2026-04-29

### Fixed

- **Phase 5.5 Ville (#93): `routes/legacy_mirror.rs` always queried
  `state.new_pool` regardless of `?branch=`.** Same shape as the main.rs
  ville_pool wiring bug fixed earlier today (#90/#94 family). The frontend's
  `LegacyMirrorPanels.tsx` correctly appends `?branch=hfville` via
  `useBranchFetch`, but every handler in `legacy_mirror.rs` (coupons,
  in-stay POS / minibar, mid-stay room moves, pricing reference) ignored
  the param and read from HF Hotel's `new_pool`. Result: a Ville
  receptionist viewing legacy-mirror panels would silently see HF Hotel
  data — or empty results once the mirror schemas existed at
  `hotelville` but the dispatch was wrong.
  
  Surgical dispatch fix mirroring the `bookings.rs` pattern: each handler
  resolves `let pool = match branch { Branch::Hfville => state.ville_pool()?,
  _ => &state.new_pool };` and threads `pool` to the existing SQL. Query
  strings, response shapes, and route paths are unchanged — only the
  pool selection. Added a `BranchOnlyQuery` for `get_pricing_reference`
  (no `cin_no`) and extended `CinNoQuery` with an optional `branch` field
  for the three per-checkin endpoints. Also rewrote the stale
  `bin/sync.rs:CT_ENABLED_TABLES` comment that called the mirror tables
  "HF Hotel only" — Ville's CT was enabled by migrations 020 + 021 after
  the 2026-04-29 SS2025 upgrade, so the mirror mappers run for both
  sites.

  The 6 mirror tables at `hotelville` are still empty today (Phase 5.5
  Ville bootstrap #80–82 not done yet); empty results when querying
  `?branch=hfville` are expected and correct post-fix. This unblocks #82
  — once the mirror tables get bootstrapped, the panels will populate
  without further code changes.

## [2.54.27] - 2026-04-29

### Fixed

- **Phase 5 Ville (#94): three more frontend modals also bypassed `branchFetch`,
  follow-up to #90.** While verifying #90's fix (commit `bac0d10`,
  `CheckOutModal` / `PaymentModal` / `GuestRegistryModal` were missed in the
  initial sweep — same routing-bypass bug, different files. Each called bare
  `fetch('/api/new/*')` instead of `branchFetch`, so a Ville receptionist's
  checkout, payment record, or TM.30 guest add/delete would silently land
  in HF Hotel's `new_pool` instead of `ville_pool`.

  Surgical swap of `fetch` → `branchFetch` (with `useBranchFetch()` hooked
  at component top) in:
  - `components/modals/CheckOutModal.tsx` — checkout submit (PUT
    `/api/new/checkins/:id/checkout`)
  - `components/modals/PaymentModal.tsx` — payment submit (POST
    `/api/new/checkins/:id/payments`)
  - `components/modals/GuestRegistryModal.tsx` — guest list fetch (GET),
    guest add (POST), guest delete (DELETE) on
    `/api/new/checkins/:id/guests[/:guestId]`. `branchFetch` added to
    `fetchGuests`'s `useCallback` dep array.

  Same zero-behaviour-change pattern as #90 — `branchFetch` returns the
  same `Response` shape, so all call sites, response handlers, and error
  paths are unchanged. Component test URL assertions updated to expect the
  `?branch=hfhotel` suffix that the hook appends in the default
  `BranchContext` state.

## [2.54.26] - 2026-04-30

### Fixed

- **Phase 5 Ville cutover got reverted by CI deploy** — the operator-set
  `HFVILLE_LEGACY_SYNC_SHADOW_MODE=false` line (added to
  `/home/nut/new-hotel-production/.env` during the 2026-04-30 ~17:00 UTC
  cutover) got wiped by the next CI deploy, which rewrites `.env` from
  GH secrets on every push. With the secret unset, the compose default
  `:-true` kicked in and `sync-hfville` restarted in shadow mode,
  silently reverting the cutover to TX-rollback. Watermark stopped
  advancing. Caught by post-deploy verification when log showed
  `shadow_mode=true allowlist=Some({...11 tables...})`.

  Fix: flip the compose default for `LEGACY_SYNC_SHADOW_MODE` from `true`
  to `false`. Now the post-cutover state survives CI deploys without an
  operator action. To re-enable shadow (e.g. rollback), the operator
  can set `HFVILLE_LEGACY_SYNC_SHADOW_MODE=true` in `.env` and force-recreate
  — that env-var value still wins over the compose default.

  This is the same `.env`-rewrite pitfall we hit with `VILLE_DB_PASSWORD`
  during the 2.54.7 deploy (CHANGELOG entry under that version). The
  pattern is durable: once cutover happens, compose defaults must
  reflect the post-cutover state, not the pre-cutover state.

## [2.54.25] - 2026-04-30

### Fixed

- **Phase 5 Ville (#90): four frontend modals/forms used bare `fetch('/api/new/*')`
  instead of `branchFetch`, silently routing Ville users' writes into the HF
  Hotel pool.** After Phase 5 cutover (2026-04-30) the backend dispatches by
  `?branch=hfhotel|hfville` to two different connection pools (`new_pool` vs
  `ville_pool`). The shared `useBranchFetch()` hook auto-appends the active
  branch from `BranchContext`, but these four call sites called the global
  `fetch` directly — so the URL had no `?branch=` param and the backend
  defaulted to HF Hotel. A Ville receptionist creating a check-in, walking up
  a customer, filing a maintenance ticket, or adjusting inventory stock would
  see the write succeed in the UI but the row would land in `hotelnew` (HF
  Hotel) instead of `hotelville`.

  Surgical swap of `fetch` → `branchFetch` (with `useBranchFetch()` invoked
  at component top) in:
  - `components/modals/QuickCheckInModal.tsx` — customer search (GET) +
    check-in submit (POST)
  - `components/forms/BookingForm.tsx` — new-customer create (POST)
  - `components/modals/MaintenanceRequestModal.tsx` — request create (POST)
    + request update (PUT)
  - `components/modals/StockAdjustmentModal.tsx` — item search (GET) +
    adjustment submit (POST)

  No behavioural changes — `branchFetch` returns the same `Response` shape, so
  every call site, response handler, and error path is unchanged. Component
  tests updated to assert the new `?branch=hfhotel` URL suffix that the hook
  appends in the default `BranchContext` state. All 621 component tests pass.

## [2.54.24] - 2026-04-30

### Fixed

- **Phase 5 Ville (#91): invoice + receipt templates were hardcoded to
  HF Hotel identity.** `app/billing/[id]/page.tsx` declared a module-level
  `hotelInfo` constant with `name: 'The HF Hotel'` plus HF Hotel address /
  phone / tax ID, and that single value was passed unconditionally to
  `InvoiceTemplate` (and would have been to `ReceiptTemplate`). When a
  receptionist switched the branch picker to HF Ville and printed an
  invoice for a Ville guest, the document showed HF Hotel's name,
  address and tax ID — a customer-facing legal-entity defect.

  Fix: extracted the data into `lib/hotel-info.ts`
  (`HOTEL_INFO_BY_BRANCH` keyed on `Exclude<Branch, 'all'>` so TypeScript
  enforces both branches are present) and a thin
  `hotelInfoForBranch(branch)` resolver that defensively falls back to
  HF Hotel for the dashboard-only `'all'` value. The billing page now
  reads `useBranch()` and passes the resolved info into
  `InvoiceTemplate`. Templates remain pure prop-driven renderers
  (no internal branch lookup) so they stay easy to test.

  **OPERATOR ACTION REQUIRED before HF Ville guests check out:** the
  `hfville` entry in `lib/hotel-info.ts` ships with deliberate
  placeholders (`'[HF Ville — info pending]'`, `'[address pending —
  fill in lib/hotel-info.ts]'`, `'[phone pending]'`, `'[tax id
  pending]'`) so the gap is visible on a printed invoice rather than
  masked with fake-but-plausible data. Replace these four strings with
  the real legal-entity name, address, phone and tax ID once the
  operator provides them — a one-file edit, no schema changes.

## [2.54.23] - 2026-04-30

### Fixed

- **Phase 5 Ville cutover (#76): `ville_pool` was bypassing
  `VilleDbConfig`** — `main.rs:95-114` constructed the connection string
  from `config.new_db.*` with a hardcoded `?options=-csearch_path%3Dville`,
  ignoring every `VILLE_DB_*` env var. So the cutover step "set
  VILLE_DB_NAME=hotelville" had zero runtime effect and the route
  handlers continued reading from `hotelnew.ville.*` (the stale
  `ville_sync` push, now 22 h+ stale since `ville_sync` was stopped
  pre-cutover). Caught during smoke test when
  `/api/customers?branch=hfville` returned phone-number-shaped IDs and
  type fields containing Thai address-template strings — clear signal
  the data wasn't from the new `hotelville` DB. Fix: replace the
  hardcoded format!() with `config.ville_db.connection_string()`,
  honoring all five `VILLE_DB_*` env vars per `VilleDbConfig::from_env()`.
  Logs now show `HF Ville pool created (newdb:5439/hotelville)`
  instead of `... (ville schema in newdb)`.

  `docker-compose.yml` also gains explicit `VILLE_DB_{SERVER,PORT,NAME,USER}`
  env passthrough so the deploy step writes them to `.env` consistently.
  Defaults match the post-cutover topology (`newdb:5439/hotelville`,
  user from `POSTGRES_USER`).

## [2.54.22] - 2026-04-30

### Removed

- **Phase 8 transition (HF Hotel) — MSSQL-fallback dead code in user-facing
  read routes.** Production has been running with `LEGACY_READ_SOURCE` UNSET
  (PG default) since the CT mappers + drift-reconcile job stabilized the
  `ht_*_legacy` mirrors, so the `*_sqlserver` branches were unreachable
  dead code on the read path. Deleted from
  `routes/{bookings,customers,rooms,checkins,stats,occupancy,calendar}.rs`:
  the `use_sqlserver()` / `use_pg_source()` env-flag helpers, every
  `*_sqlserver` private function (10 in total), and every `if use_sqlserver()
  { … } else { *_pg } …` branch in the public handlers. Net diff: -1407
  lines / +84 lines across 8 files. `cargo check` clean (5 pre-existing
  warnings); `cargo test --lib` passes (301/301).
- **`LEGACY_READ_SOURCE` env var — formally deprecated.** The flag is no
  longer read anywhere in the codebase. MSSQL is now write-only for the
  legacy .NET app (writeback worker + ville_sync continue to use
  `legacy_pool`); the user-facing read path is PG-only. See
  `docs/architecture.md` Phase 8.

### Changed

- **`routes/calendar.rs` — legacy calendar fetch is PG-only.** Dropped the
  `fetch_legacy_calendar_data` MSSQL helper; HF Hotel now reads from
  `ht_bookings_legacy` + `ht_checkins_legacy` exclusively. The HF Ville
  branch (already PG via `ville_pool`) and the new-mode `fetch_new_calendar_data`
  branch are unchanged.
- **`routes/{bookings,customers,rooms,checkins,stats,occupancy,calendar}.rs`
  — module docstrings updated** to reflect "Reads from PG (`ht_*_legacy`
  cache, fed by drift-reconcile + CT mappers)".

## [2.54.21] - 2026-04-29

### Fixed

- **`migrate_legacy.rs` populates `legacy_cust_no` / `legacy_book_id` /
  `legacy_cin_no`** on the canonical inserts. Caught during Phase 5
  Ville #85 backfill: customers + bookings + check-ins all imported
  cleanly, but the CT mappers continued to emit "customer not yet
  mirrored" warnings because they look up by `legacy_cust_no` while
  the import only set `cust_code`. Same shape applies to bookings
  (`legacy_book_id` vs `book_no`) and check-ins (`legacy_cin_no` vs
  `cin_no`). Each INSERT now binds `$1` to BOTH the user-facing key
  AND the legacy lookup column. For the existing hotelville import
  ran 2026-04-29, a one-shot `UPDATE … SET legacy_X = X WHERE source =
  'legacy' AND legacy_X IS NULL` filled the columns retroactively
  (1812 customers + 1079 bookings + 1417 check-ins). Mapper warnings
  silenced within seconds of the UPDATE.

## [2.54.20] - 2026-04-29

### Fixed

- **`migrate_legacy.rs` checkin import duplicate-key error** —
  `View_CheckIn_Ds` returns multiple rows per `Cin_no` for group
  bookings (one row per booked room — the same multi-row shape
  that bit us in #64). The check-in import loop only deduped
  against the existing-set populated from `ht_checkins` at the
  start, not against rows it had already inserted in the current
  pass. So the second row for any group-booking `Cin_no` blew up
  with `duplicate key value violates unique constraint`. Fix:
  insert each successfully-imported `cin_no` into the dedup set
  so subsequent rows for the same PK skip cleanly. Caught and
  fixed during Phase 5 Ville #85 backfill.
- **`init-db/init-hotelnew.sql` schema_migrations seed for 023 + 024.**
  CI's drift check fired because the init-db baseline had the
  migrated schema for both ALTERs but no matching seed rows in
  schema_migrations — so `migrate.sh` on a fresh init-baselined
  DB saw 023 + 024 as pending and tried to re-apply them, which
  the drift gate rejects. Appended two `INSERT INTO schema_migrations`
  rows matching the pattern for 008-022.

## [2.54.19] - 2026-04-29

### Fixed

- **`migrate_legacy.rs` referenced `Book_Room_No` column** that does not
  exist in `View_Booking_Ds` at EITHER site (legacy reservations record
  only the room TYPE at booking time; the actual room is assigned at
  check-in via `HT_CheckIn_Ds.Cin_Room_No`). Discovered 2026-04-29 when
  trying to backfill canonical state for HF Ville (task #85). The
  binary worked at HF Hotel only because nobody had run it post-cutover
  there — canonical bookings flowed in via CT events instead. Fix:
  remove the column from the SELECT, set `room_id = None` in the
  booking-room loop, rely on the checkin-import / CT-mapper flow to
  materialise the room linkage. The booking-room insert still skips
  when `room_id` is None (as it always did for any booking that didn't
  match a known room).
- **Migration 024** widened canonical `ht_customers` VARCHAR(20)
  columns to match what migration 009 did to the `ht_customers_legacy`
  cache. Specifically `cust_phone` 20 → 200, plus
  `cust_code` / `cust_idcard` / `cust_title` / `cust_taxid` /
  `legacy_cust_no` to 100, and `source` to 50. Required because one
  HF Ville customer's `Cust_Add_tel` is 21 chars — exceeds the old
  cap. Manually applied to both DBs on evergreen ahead of CI; later
  CI deploys see migration 024 in `schema_migrations` and skip.

## [2.54.18] - 2026-04-29

### Added

- **`migrate_legacy` binary now ships in the production backend image**
  (added to `hotel-backend/Dockerfile` builder + runtime stages).
  Previously only `hotel-backend`, `writeback`, `sync`, and
  `backfill_rooms` were copied into the runtime image. Discovered
  during Phase 5 Ville soak that the checkin mapper defers with
  "customer not yet mirrored" warnings — the canonical `ht_customers`
  / `ht_bookings` / `ht_checkins` at `hotelville` are empty because
  bootstrap snapshots only the legacy CACHE tables (`ht_*_legacy`),
  not canonical state. `migrate_legacy.rs` already does the right
  imports (`View_Customers` → `ht_customers`,
  `View_Booking_Ds` → `ht_bookings` + `ht_booking_rooms`,
  `View_CheckIn_Ds` → `ht_checkins`) but wasn't reachable from the
  deploy host. Now it is — operator can run a one-shot
  `docker compose --profile backfill run --rm
  -e DB_SERVER=… -e DB_NAME=HOTEL -e MSSQL_PORT=1436
  -e DATABASE_URL=…/hotelville
  --entrypoint ./migrate_legacy backfill-rooms`
  to seed canonical state before #76 cutover (task #85).

## [2.54.17] - 2026-04-29

### Fixed

- **`legacy_mirror.ht_order_up` / `ht_order_down` PK changed from
  `(id)` to composite `(id, cust_type, cast_type)`** (task #84,
  diagnosed 2026-04-29 during Phase 5 Ville bootstrap). HF Ville's
  legacy `HT_Order_Up` / `HT_Order_Down` tables hold 8 rows each
  where `id` is a tier number (1, 2, 3) — NOT a unique key. Real
  composite key is `(id, cust_type, cast_type)`. The original
  single-column PK from migration 020 blew up the dimension reload
  TX with `duplicate key value violates unique constraint` on every
  reconcile tick at any site that actually populated the tables.
  HF Hotel never hit it because both tables are empty there. Fix:
  new migration 023 plus matching `init-hotelnew.sql` schema. Both
  DBs converge on the corrected baseline. Phase 5.5b mirror feature
  (#80) can now bootstrap these dimension tables successfully.

  Pre-flight verified all 4 affected tables (hotelnew + hotelville,
  ht_order_up + ht_order_down) were empty before applying — no row
  data conversion needed for the `ALTER COLUMN ... NOT NULL` step.
  Migration 023 manually applied to both DBs on evergreen 2026-04-29
  ahead of CI; subsequent CI deploys see migration 023 already in
  schema_migrations and skip cleanly.

## [2.54.16] - 2026-04-29

### Added

- **`scripts/backup-db.sh --site <hfhotel|hfville>`** flag (task #79,
  extending the per-site rollout pattern set by #78's `sync-status.sh`).
  Selects which site's canonical PG database to back up — `hfhotel`
  (default) → `hotelnew`, `hfville` → `hotelville`. Both databases live
  in the same `new-hotel-db` container, so only the `pg_dump` target and
  the output filename change. Output filenames now embed the site
  discriminator: `hotelnew-hfhotel-YYYYMMDD_HHMMSS.sql` and
  `hotelville-hfville-YYYYMMDD_HHMMSS.sql` so HF Hotel and HF Ville
  dumps coexist in the same `BACKUP_DIR` without colliding. The script
  header now also documents the recommended cron entries for both
  sites (operator action — not auto-installed) with a 5-minute stagger
  to avoid I/O contention on the shared disk.
- **`scripts/migrate.sh --site <hfhotel|hfville>`** flag (task #79).
  Same site → DB-name mapping as `backup-db.sh`. Precedence is
  `POSTGRES_DB` env var > `--site` derivation > legacy `hotelnew`
  default, so the upcoming Phase 5 CI matrix can call
  `migrate.sh --site hfville` once per site without duplicating the
  script while one-off operator sessions can still pin
  `POSTGRES_DB=…`. Pre-migration backup filenames + the prune-old-backups
  glob are now scoped per-site (`${DB_NAME}-${SITE}-*.sql`) so a Ville
  migration can't evict HF Hotel's pre-migration history.
- Both scripts reject any `--site` value other than `hfhotel` or
  `hfville` with a stderr message + exit 2, matching the convention
  established in `scripts/sync-status.sh`. Runtime activation (cron
  entries on evergreen, CI matrix wiring) is deferred to post-cutover.

## [2.54.15] - 2026-04-29

### Fixed

- **`web` container healthcheck false-positive `unhealthy`** discovered
  during HF Hotel post-Phase-5-Ville-deploy audit. The probe was
  `wget --spider http://localhost:3003`; the container's `/etc/hosts`
  defines BOTH `127.0.0.1 localhost` and `::1 localhost`, and wget
  prefers IPv6, but Next.js 16 standalone with `HOSTNAME=0.0.0.0`
  binds IPv4 only — so the IPv6 connect to `[::1]:3003` returns
  Connection refused even though the site serves correctly to all
  external traffic. Healthcheck flapped `unhealthy` for 21 consecutive
  attempts despite zero user-facing impact. Fix: change probe to
  `http://127.0.0.1:3003` (force IPv4). Same outcome could be reached
  by setting `HOSTNAME=::` to dual-bind, but explicit IPv4 in the
  healthcheck is the smaller diff and matches the actual production
  bind. Inline comment captures the trap.

## [2.54.14] - 2026-04-29

### Added

- **`scripts/sync-status.sh --site <hfhotel|hfville>`** flag (task #78,
  finishing the per-site observability slice begun in #69). Selects which
  site's canonical PG database to query — `hfhotel` (default) → `hotelnew`,
  `hfville` → `hotelville`. Both databases live in the same `new-hotel-db`
  container, so only the `psql -d` target changes; SSH host, container
  name, and Postgres user are unchanged. Default is `hfhotel` for
  back-compat with every pre-#78 invocation. `SYNC_STATUS_PG_DATABASE`
  override still wins for `hfhotel`; a new `SYNC_STATUS_PG_DATABASE_HFVILLE`
  override is recognized for ad-hoc Ville database renames.

### Changed

- **Backend `/health` endpoint** now also returns the current site's CT
  watermark snapshot — `ct_watermark` (i64) and `last_polled_at` (RFC3339
  string), pulled from the canonical PG `legacy_ct_state` row (task #78).
  The endpoint remains a LIVENESS probe — `ok: true` always, even when
  the watermark fields are `null`. Pre-bootstrap installs (no row),
  legacy-only mode (no PG pool), and transient PG errors all collapse to
  `null` watermark fields with a `tracing::warn!` on the SQL-error path,
  so the load-balancer probe contract is preserved. Existing fields
  (`site`, `ok`, `service`) are unchanged. New unit tests cover both the
  populated and null-watermark response shapes; the SQL fetch path uses
  the runtime `sqlx::query_as` (no `.sqlx/` macro cache regeneration
  needed).

## [2.54.13] - 2026-04-29

### Fixed

- **drift-reconcile None-branch cache leak (#65).** The 4 `sync_*`
  functions in `hotel-backend/src/scheduler/sync.rs` (customers, rooms,
  bookings, checkins) had a slow leak in the `DiffOnly` mode None-branch:
  when the legacy PK was absent from `ht_*_legacy` (cache-miss), they
  called `record_divergence()` and incremented `added`, but never
  inserted a cache row. The next reconcile tick (every 15 min) re-read
  the same cache-miss, recorded the same divergence again, ad infinitum.
  Observed steady-state rate: ~3 entries per 15 min on bookings driven
  by genuinely new PKs landing between ticks (below the 50/hour alert
  threshold so it didn't spam, but the unresolved-rows count grew with
  new-PK velocity). Fix: each None-branch now writes a minimal
  cache-only marker row (`INSERT ... ON CONFLICT DO NOTHING` with PK +
  sync_hash + synced_at; detail columns left NULL) so the next tick
  sees the row and goes through the Some-branch's already-correct
  UPDATE path. Canonical state remains owned by the CT watcher; the
  marker row is purely diff-only bookkeeping. Companion to #62
  (Some-branch cache UPDATE) and #64 (multi-row aggregation). Bookings
  uses the composite UNIQUE `uq_bookings_legacy_key (book_no,
  book_room_type)` with `room_type_key` (always non-NULL — empty
  string for legacy NULL) so subsequent SELECTs using
  `COALESCE(book_room_type,'')` match this row.

## [2.54.12] - 2026-04-29

### Fixed

- **`bin/backfill_rooms.rs` + `bin/migrate_legacy.rs`** — both binaries
  had their own duplicated MSSQL connection logic that bypassed task
  #68's `MSSQL_PORT` env handling, password-fallback removal, and bb8
  circuit-breaker timeouts. Caught when trying to run
  `backfill_rooms` against HF Ville on port 1436: TCP connect timed
  out because the binary was hardcoded to 1433 (the SS2025 Express
  instance at Ville does not listen on the default port). Refactor:
  both binaries now call `DbConfig::from_env()` + `db::create_pool()`
  so they inherit the same env handling and timeouts as the main
  backend. Removed the duplicated `unwrap_or_else(|| "REDACTED-sa-pw")`
  fallbacks and the hardcoded `.port(1433)` calls.

## [2.54.11] - 2026-04-29

### Changed

- **`sync-hfville` defaults flipped to shadow-soak posture.** With Phase 5
  Ville bootstrap complete (task #74, watermark stamped at 2026-04-29
  ~17:50 ICT), the CT watcher now needs to keep polling in shadow mode
  through the 48 h soak (task #75). Operator-side `.env` edits don't
  survive CI deploys (the deploy step rewrites `.env` from GH secrets),
  so the `docker-compose.yml` defaults must encode the shadow posture:
  - `HFVILLE_LEGACY_SYNC_ENABLED` default `false` → **`true`** (was previously default-OFF as a safety guard for the not-yet-bootstrapped state; that guard is no longer needed now that bootstrap is done)
  - `HFVILLE_LEGACY_SYNC_SHADOW_MODE` stays default `true` (operator flips to `false` at cutover #76)
  - `HFVILLE_LEGACY_SYNC_TABLE_ALLOWLIST` default empty → **`HT_Customers,HT_Rooms,HT_Book_H,HT_Book_Ds,HT_Book_Date,HT_CheckIn_H,HT_CheckIn_Ds,HT_CheckIn_Pay,HT_Room_Status,HT_Rooms_Cancel,HT_Receipt_H`** (the 11 tables enabled by 020). Phase 5.5b (021, task #80) adds 6 more mirror tables — at that point clear the env to fall back to "all enabled".

  Profile gate (`profiles: [hfville]`) still ensures the container only
  starts when explicitly opted in via `--profile hfville`. HF Hotel
  containers do not read these env vars and remain unaffected.

## [2.54.10] - 2026-04-29

### Added

- **`sync-hfville` + `writeback-hfville` services in `docker-compose.yml`**
  (both under `profiles: [hfville]`, default-INERT). The Phase 5 (Ville)
  worker pair — same backend image, different env: Ville MSSQL via WG
  (`<ville-mssql-host>,1436`), `hotelville` PG database, `SITE_ID=hfville`.
  Per ADR 0001 Q2 the Ville stack runs CENTRAL on evergreen alongside
  HF Hotel — no separate backend container needed (the existing
  `backend` service's `ville_pool` repoints at `hotelville` at cutover,
  task #76). Ships with all watcher knobs default-disabled
  (`HFVILLE_LEGACY_SYNC_ENABLED=false`, `HFVILLE_WRITEBACK_ENABLED=false`)
  so the binaries exit `Ok(0)` and `restart: on-failure:5` does NOT
  loop. Bootstrap (task #74) flips the env to enable shadow mode;
  cutover (#76) flips to live.
- **CI deploy step** now pulls + force-recreates the hfville profile
  after the HF Hotel pair, mirroring the legacy-profile pattern.
  Container naming uses the `-hfville` suffix
  (`new-hotel-production-sync-hfville-1` etc.) so they cannot collide
  with HF Hotel containers under the same compose project.

### Notes

- Both Ville services reuse `DB_PASSWORD` for now since both sites' MSSQL
  share `sa/REDACTED-sa-pw`. Split into a dedicated `HFVILLE_DB_PASSWORD` GH
  secret when one site rotates ahead of the other (Phase 8 ops hardening).
- No CI matrix refactor was needed — for two sites a single compose with
  per-site profiles is simpler than a deploy-job matrix. Revisit when a
  third site appears.

## [2.54.9] - 2026-04-29

### Fixed

- **Backend container crash-loop after 2.54.7 deploy** — the password
  fallback removal in 2.54.6 (`config.rs::require_secret`) made
  `VILLE_DB_PASSWORD` strictly required when `VILLE_DB_ENABLED=true`,
  but no operator had ever set this env var because the existing
  single-cluster setup (Ville pool reads the `ville` schema in the
  same `newdb` container) shares POSTGRES_PASSWORD by reuse. The
  10:27 UTC deploy hit `panic at config.rs:79` on backend startup,
  declared unhealthy, and the deploy job exited 1. Hot-fix on
  evergreen restored production at 10:30 UTC.

  Permanent fix: in `docker-compose.yml`, default the backend
  service's `VILLE_DB_PASSWORD` env to `${POSTGRES_PASSWORD}` via
  shell expansion. No new GH secret needed; existing `POSTGRES_PASSWORD`
  is already validated. Operator overrides `VILLE_DB_PASSWORD` only
  when Ville migrates to its own cluster (Phase 5 Ville cutover —
  task #76).

## [2.54.8] - 2026-04-29

### Added

- **`init-db/01-create-hotelville-database.sh`** — auto-creates the
  `hotelville` PostgreSQL database alongside `hotelnew` on first
  container init. Per ADR 0001 (Phase 5 Ville multi-site, decision
  Q1 = per-DB topology): HF Hotel and HF Ville share the same PG
  cluster but live in separate logical databases. Each DB gets its
  own `legacy_ct_state` row, its own `legacy_sync_status`, its own
  `ht_reconcile_log`, etc. — the single-row `CHECK (id=1)`
  constraints stay valid because each DB has its own copy.
  Same script also re-applies `init-hotelnew.sql` against the new
  `hotelville` DB for schema baseline parity.
  For existing clusters (evergreen production), the equivalent
  CREATE DATABASE + init was applied as a one-shot on 2026-04-29 —
  hotelnew and hotelville now have matching schema_migrations rows
  (000, 008-020, 022).

## [2.54.7] - 2026-04-29

### Added

- **`SITE_ID` env var (default `"hfhotel"`).** Phase 5 multi-site
  observability prerequisite (task #69). HF Hotel and HF Ville will
  each run a separate instance of the same `hotel-backend` /
  `bin/sync` / `bin/writeback` binaries against their own per-DB
  PostgreSQL, but they share a single Slack webhook + log sink. Without
  a site marker an on-call operator can't tell which deployment fired
  a `:rotating_light:` alert. New `SiteConfig` validates the env var
  is one of `"hfhotel"` | `"hfville"` (panics on any other value to
  catch typos like `hfvilel` at startup, not at 3am during an outage).
  Threaded into:
  * **Slack alerts** — every existing alert (schema fingerprint
    refusal, retention overflow, cold-replay refusal, drift threshold,
    self-heal, listener UNHEALTHY, exhausted/resolved jobs, hourly
    report, check-in/checkout/booking notifications) now prefixes the
    message text with `[site=<id>] ` via the new
    `SlackMessage::with_site_text` helper. The prefix lives in the
    `text` field (Slack notification preview), so even a Block Kit
    payload surfaces the site at-a-glance.
  * **Tracing spans** — `bin/sync.rs` wraps the main poll loop in
    `info_span!("ct_watcher", site = %config.site_id)`; `bin/writeback.rs`
    wraps its main loop in `info_span!("writeback_worker", ...)`;
    the 15-min reconcile cron job in `scheduler/jobs.rs` wraps each
    tick in `info_span!("reconcile_tick", ...)`. Every log line emitted
    inside these scopes carries `site=<id>` automatically.
- **`/health` endpoint.** Returns `{ "site": "<id>", "ok": true,
  "service": "backend" }` so external monitors / smoke tests can
  confirm they hit the right deployment when both HF Hotel and HF
  Ville are reachable on the same hostname infrastructure. HTTP
  status code unchanged from the legacy probe contract — additive
  only.
- **`LEGACY_RECONCILE_DRIFT_ALERT_THRESHOLD_<SITE>` per-site override.**
  The existing `LEGACY_RECONCILE_DRIFT_ALERT_THRESHOLD` env var
  continues to work as a single global. When the per-site var is set
  (e.g. `LEGACY_RECONCILE_DRIFT_ALERT_THRESHOLD_HFVILLE=20`), it
  takes precedence — letting the smaller property run a tighter
  alert threshold without affecting HF Hotel's tuning. Garbage values
  in the per-site var fall through to the global instead of panicking.

### Changed

- **`scheduler::sync::run_sync` signature.** Added a trailing
  `site_id: &str` parameter so the drift-alert message names which
  deployment fired and the per-site threshold env var can be looked
  up. Updated callers: `bin/sync::run_bootstrap` (passes the parsed
  `SiteConfig::id`) and `scheduler::jobs::init_scheduler` (forwards
  from the `SiteConfig` plumbed in from `main.rs`).
- **`scheduler::init_scheduler` signature.** Added a trailing
  `site: SiteConfig` parameter, threaded through to all four cron
  jobs (hourly report, check-in / checkout / booking polling) so
  every Slack message they emit carries the site prefix and every
  log line carries `site=<id>`.

## [2.54.6] - 2026-04-29

### Added

- **`MSSQL_PORT` env var (default `1433`).** Phase 5 enabling work for HF
  Ville: the new SS2025 Express instance there listens on `1436`, not the
  default `1433`. Previously `hotel-backend/src/db/pool.rs` hardcoded
  `tib_config.port(1433)`, so a multi-site deploy was impossible without
  forking. The new `DbConfig::port: u16` field threads through into
  `tiberius::Config::port()`, defaulting to 1433 for back-compat at HF
  Hotel. Plumbed into `docker-compose.yml` for `backend`, `writeback`,
  `sync`, and `backfill-rooms` services as `MSSQL_PORT=${MSSQL_PORT:-1433}`,
  and into `.github/workflows/docker-build.yml` as an optional GH secret
  (defaults to 1433 in the generated `.env` when the secret is absent).
  A typo in the value (`MSSQL_PORT=garbage`) panics at startup with a
  clear message rather than silently falling back to 1433 and connecting
  to the wrong instance. Documented in both `.env.example` files.

### Changed

- **bb8/tiberius circuit-breaker timeouts.** Audit-flagged gap: the bb8
  pool used the library defaults (`connection_timeout: 30s`,
  `max_lifetime: 30 min`, `idle_timeout: 10 min`) implicitly, and
  tiberius's `Config` builder exposes no socket-level connect timeout.
  Result: a hung MSSQL or a dropped WireGuard tunnel produced an
  unbounded bb8 acquire queue with no alert. Now `db/pool.rs`
  explicitly sets `connection_timeout(15s)` (also caps the TCP connect
  since bb8 wraps `ConnectionManager::connect` in
  `tokio::time::timeout`), `max_lifetime(30 min)`, and `idle_timeout(10
  min)` as named `POOL_*` constants with one-line "why" comments. Pool
  builder values asserted in unit tests via `pool.config()` so a future
  typo on the timeout setter can't silently regress to the bb8 default.

### Security

- **Removed the hardcoded `DB_PASSWORD` fallback (`"REDACTED-sa-pw"`).** Audit
  finding: `hotel-backend/src/config.rs:34` previously fell through to a
  default password string when the GH secret was missing/empty — exactly
  the silent-credential-leak failure mode flagged in the
  `.env`-rewrite-by-CI pitfall (see `docs/runbook-sync.md §4a`). The new
  `require_secret` helper panics loudly at startup if `DB_PASSWORD` is
  unset OR is the empty string (a botched `.env` rewrite produces
  `DB_PASSWORD=''`, which `env::var` returns as `Ok("")` — also rejected).
  Same fail-loud policy applied to `VilleDbConfig::password` when
  `VILLE_DB_ENABLED=true`; left as best-effort when the mirror is
  disabled (the HF Hotel default) so operators without the Ville secret
  provisioned can still boot the backend. Six new unit tests cover
  default port, explicit port, garbage port, missing password, empty
  password, and password-propagation paths.

## [2.54.5] - 2026-04-29

### Added

- **`migrations/legacy-mssql/020_phase5_enable_ct.sql`** (+ rollback) —
  backfill of the original Phase 5 cutover DDL. At HF Hotel the
  equivalent statements were applied manually 2026-04-25 before the
  legacy-mssql migration system existed; this file captures them so
  HF Ville (and any future restore / new site) can reach the same
  pre-Phase-5.5 baseline auditably from a single file. Covers DB-level
  CT enable + PKs + per-table CT on the 11 canonical-sync tables:
  `HT_Customers`, `HT_Rooms`, `HT_Book_H`, `HT_Book_Ds`, `HT_Book_Date`,
  `HT_CheckIn_H`, `HT_CheckIn_Ds`, `HT_CheckIn_Pay`, `HT_Room_Status`,
  `HT_Rooms_Cancel`, `HT_Receipt_H`. Pre-flight verified clean against
  Ville (0 NULLs, 0 dupes on every PK candidate column). DO NOT
  re-apply at HF Hotel — `ALTER DATABASE SET CHANGE_TRACKING ON` would
  error if already enabled. README + reference network path for Ville
  (`<ville-mssql-host>,1436` over WG `hfville` interface) updated to match.

## [2.54.4] - 2026-04-29

### Fixed

- **Drift-reconcile spam loop, round 2 (multi-row PK aggregation).** The
  2.54.2 fix (cache-write the new hash after `record_divergence`) was
  necessary but not sufficient: `View_CheckIn_Ds` returns 41-45 rows
  per `Cin_no` (24,904 total / 19,169 distinct PKs) and
  `View_Booking_Ds` returns up to 3 rows per `(Book_No, Book_Room_Type)`
  composite PK. The per-row loop in `sync_checkins` / `sync_bookings`
  hashed each row independently against the single-row-per-PK
  `ht_*_legacy` cache — every iteration produced a different hash, and
  consecutive iterations of the same PK in one tick re-flagged
  divergence and overwrote the cache, so the next reconcile tick
  re-flagged the whole world again. Slack alert fired every 15 min at
  ~8,773 unresolved rows on `checkins`. Fix: aggregate all detail rows
  by PK into a `BTreeMap<PK, Vec<Detail>>` first, deterministic-sort
  the group (checkins by `(room_no, room_in, room_out)`; bookings by
  the six non-key fields), and hash a deterministic concatenation of
  the entire group — one `record_divergence` and one cache UPDATE per
  PK regardless of how many rows the view projected. Extracted pure
  helpers `aggregate_checkin_hash` / `aggregate_booking_hash` (+ small
  `CheckinDetail` / `BookingDetail` private structs) so the
  determinism + sensitivity contract is unit-tested. SQL strings
  unchanged; `ht_*_legacy` schema unchanged. `sync_customers` /
  `sync_rooms` left untouched (1:1 PKs, not affected). Also: in
  `DiffOnly` mode the `mssql_row_json` payload now carries the full
  sorted `details` array so an operator can see what changed across
  all rooms on the booking, not just one row.

## [2.54.3] - 2026-04-29

### Removed

- **`scripts/deploy-dev.sh` + `/home/nut/new-hotel-dev/` build dir on evergreen.**
  The script was a documented break-glass that rsynced source to a
  separate dir on the deploy host, built `:dev-local` natively in
  evergreen's Docker daemon, retagged it to `:latest`, and force-recreated
  the backend + writeback containers — bypassing CI/tests/registry
  entirely and leaving production running an image that doesn't
  correspond to any commit. Direct contradiction of CLAUDE.md §"Deployment
  Policy" ("Pipeline is the only way"). Removed the script, deleted the
  build dir on evergreen, and untagged stale `:dev-local` and orphaned
  `ghcr.io/jwinut/new-hotel:latest` (wrong namespace) and `new-hotel:test`
  images so the version-mixing surface area is gone. If a true break-glass
  is needed in future: push a branch, build with a dated tag (NOT
  `:latest`), and pin one container at it explicitly.
- **Stale source-tree at `/home/nut/new-hotel/` on evergreen** (3.2 G,
  last updated Apr 26). The deploy job copies into `/home/nut/new-hotel-production/`
  exclusively; this checkout was a leftover never read by the pipeline.
  Removed for the same version-mixing-prevention reason as above.

## [2.54.2] - 2026-04-29

### Fixed

- **Drift-reconcile spam loop** — Phase 6 alert was firing every 15 min
  with 22-24k unresolved `ht_reconcile_log` rows on `checkins` (~3,041
  distinct PKs being re-flagged ~1.9× per tick). Root cause: `DiffOnly`
  mode in `scheduler::sync` logs divergence via `record_divergence()`
  but never writes the new hash back to the `ht_*_legacy` cache, so
  every subsequent reconcile tick recomputes the same hash, sees the
  same prior_hash mismatch, and re-logs the same drift forever. Fix:
  after `record_divergence` in each of the 4 `sync_*` DiffOnly
  Some-branches (customers/rooms/bookings/checkins), `UPDATE` the
  matching `ht_*_legacy.sync_hash + synced_at` so the cache reflects
  what we just observed. Cache-only write — does NOT mutate canonical
  state (which the CT watcher owns). Best-effort: a failed cache
  update only re-fires the alert next tick, never a correctness issue.
  Also: marked the existing 1,521,844 stale unresolved log entries as
  resolved on prod (one-shot SQL outside the deploy). None-branch
  (legacy row with no PG cache row yet) deferred — no observed spam
  coming from that path.

## [2.54.1] - 2026-04-27

### Fixed

- **Inventory dashboard stats card silently rendered zeros.** The frontend
  read `statsData.data` but the backend `/api/new/inventory/stats` returns
  `{ success, stats }` (verified in `hotel-backend/src/routes/new_inventory.rs`,
  `StatsResponse` struct). Switched the access to `statsData.stats`. Also
  renamed the local `DashboardStats.categoriesCount` field to
  `totalCategories` so it aligns with the backend's camelCased
  `totalCategories` (from `total_categories: i32`) — and updated the JSX
  reference at the categories card so it actually reads the new field
  instead of `undefined`. Without the JSX fix, the categories tile would
  have continued to show `0` even after the data-flow fix.
- **`web` container healthcheck flapped `unhealthy` despite traffic working.**
  Next.js 16 standalone binds to the docker bridge IP only (e.g.
  `172.19.0.5:3003`), so the in-container `wget http://localhost:3003`
  healthcheck always timed out. Added `HOSTNAME=0.0.0.0` to the `web`
  service env in `docker-compose.yml`, mirroring the `HOST=0.0.0.0` line
  on `backend`. No service depends on `web`'s health, so the alternative
  (disabling the healthcheck) was viable, but matching the existing
  bind-on-all-interfaces pattern is the smallest, most consistent diff
  and keeps an honest signal for ops.

## [2.54.0] - 2026-04-27

### Phase 6 — drift-reconcile finalize + retire polling-sync

The CT watcher (`bin/sync.rs`) is the authoritative real-time path;
the demoted `scheduler::sync::run_sync` is now a slower safety net
with operator alerting. Per `docs/architecture.md` §8 (Phase 6 row).

### Added

- **Drift alert at end of every reconcile tick.** `scheduler::sync::run_sync`
  now counts unresolved `ht_reconcile_log` rows added in the last hour,
  grouped by `table_name`. If any table breaches the threshold (default
  **50**, override via `LEGACY_RECONCILE_DRIFT_ALERT_THRESHOLD`), one
  `:rotating_light:` Slack message is fired listing every offending table.
  Always logs the same data even when Slack is unconfigured. The
  threshold logic is split into a pure function
  (`tables_breaching_threshold`) and unit-tested for boundary behaviour
  (strict-greater-than, empty input, garbage env values, custom thresholds).
- **`SlackClient` plumbed through `scheduler::sync::run_sync`.** Signature
  now takes `Option<&SlackClient>`. `init_scheduler` constructs the
  client when `SlackConfig::is_configured()` and threads it into the
  cron job. Bootstrap (`bin/sync --bootstrap`) passes `None` because
  every legacy row is a "PG miss" by construction during a fresh seed
  and would otherwise trigger an unconditional alert.
- **Runbook §9 — Phase 6 drift-reconcile safety net.** Documents cadence,
  alert mechanic, three-step investigation procedure (classify → resolve),
  threshold tuning guidance, and the contract that
  `record_success`/`record_error` continue to feed dashboards.

### Changed

- **Reconcile cadence locked at 15 min on the quarter-hour** (`0 */15 * * * *`).
  Doc-comment in `scheduler/jobs.rs` now spells out why we deliberately
  do NOT poll faster: the CT watcher's sub-second path covers the
  latency-sensitive case; reconcile faster would only add legacy-MSSQL
  load without changing recovery posture (operator response time to a
  Slack alert dominates).
- `record_success` and `record_error` in `scheduler/sync.rs` carry
  Phase 6 doc-comments stating the dashboard contract — they continue
  to update `sync_status` rows in BOTH `Upsert` and `DiffOnly` modes
  for observability and must not be removed without updating consumers.

### Verified unchanged

- `LEGACY_SYNC_RECONCILE_MODE` already defaults to `DiffOnly` (locked
  in v2.45.0). Re-asserted by existing
  `from_env_defaults_to_diff_only_when_unset` test.
- `scheduler/mirror.rs` reload of the 4 legacy-only dimension tables is
  NOT polling-sync to retire — it's the canonical source for tables
  with no CT mapper. Untouched.
- CT watcher infrastructure (`bin/sync.rs` and `src/sync/`) unchanged
  — Phase 6 is a safety-net feature, not a watcher replacement.

## [2.53.2] - 2026-04-29

### Fixed

- **Audit finding N1 — bootstrap-vs-CT race.** `bin/sync.rs` now hard-
  refuses `--bootstrap` when `LEGACY_SYNC_ENABLED=true` unless the
  operator opts in via `LEGACY_SYNC_ALLOW_LIVE_BOOTSTRAP=true`.
  Previously the operator got a warning and the bootstrap proceeded;
  the snapshot's `DELETE FROM legacy_mirror.<table>` could clobber
  `mirror_source='ct'` rows the watcher just wrote. The refusal
  matches the cold-replay / overflow style (loud Slack alert if
  configured, 60s sleep before exit to throttle Docker restart
  cadence). Added unit tests pinning the refusal-message wording so
  Slack-triage and runbook references stay in sync.
- **Audit finding N2 — bootstrap data-loss window for mirror tables.**
  `run_bootstrap` now reads `CHANGE_TRACKING_CURRENT_VERSION()` BEFORE
  the reconcile + transactional snapshot (was: after) and stamps that
  captured value as the watermark. `CHANGETABLE(CHANGES <table>,
  @version)` returns rows strictly greater than `@version`; reading
  AFTER snapshots silently skipped any CT row produced during the
  snapshot window. For canonical tables this self-heals on the next
  update (idempotent UPSERT-by-hash), but mirror-table INSERTs in that
  window with no follow-up update would never land.
- **Audit finding N5 — `snapshot_*` panic on schema drift.** Replaced
  every `r.get::<T, _>(N)` with `r.try_get::<T, _>(N).ok().flatten()`
  across the 6 transactional snapshots and the 3 dimension reload
  paths in `scheduler/mirror.rs`. The schema-fingerprint check
  upstream catches most drift; this is defense-in-depth so a type
  mismatch becomes `None` for that cell instead of crashing the entire
  bootstrap.
- **Audit finding N7 — orphan NULL `bill_no` rows in
  `HT_Bill_Debt_Ds`.** `snapshot_bill_debt_ds` now skips rows whose
  `bill_no` (FK to `HT_Bill_Debt_H`) is NULL and counts them in a new
  `skipped_null_bill_no` log field. Downstream queries always filter
  by `bill_no`, so a NULL-bill_no row is invisible to consumers
  anyway. Empty table at HF Hotel today; no migration required.

### Changed

- `docs/runbook-sync.md` env-var matrix gained an entry for
  `LEGACY_SYNC_ALLOW_LIVE_BOOTSTRAP`.

## [2.53.1] - 2026-04-29

### Fixed

- **Audit finding N4 — D-row PK NULL overwrite in `materialise_row`**
  silently broke every Delete event on the 6 legacy_mirror tables.
  `bin/sync.rs::materialise_row` previously inserted PK columns
  first, then ran the projection loop second; the projection loop
  unconditionally wrote `MockValue::Null` for any cell where
  `read_cell` returned None. On D rows the LEFT JOIN nulls every
  `t.<col>` projection, so for any mapper whose PK column is also
  projected (every mirror mapper — `t.cupon_no`, `t.id`, `t.Bill_No`)
  the projection loop's NULL clobbered the real CT-side PK and
  `apply()` crashed with "PK NULL — should not happen post Phase
  5.5b" on every legacy delete. Canonical mappers were unaffected
  because their `id` PK is never projected.
  Fix: extracted the inner logic into a pure helper
  `build_materialised_row` and reordered the loops so the projection
  runs first and the PK loop runs last (overwriting the projection's
  NULL on D rows). Insert/Update behaviour unchanged (CT-side and
  table-side PK agree on I/U).
  Latent in production since 2026-04-29 ~16:00 UTC; no observed
  hits prior to the fix because no legacy DELETE on the 6 mirror
  tables had fired yet (receptionists rarely delete coupons /
  minibar / room-moves).

### Added

- **Regression tests for the loop-order fix** (`bin/sync.rs::tests`):
  `d_row_pk_survives_null_projection_overwrite` and
  `iu_row_pk_value_consistent_after_loop_swap` lock the new
  `build_materialised_row` ordering at the unit level (no
  `tiberius::Row` construction needed).
- **Integration tests for `CuponMirrorMapper`** in
  `tests/test_sync_phase55c_mirror_apply.rs` — Insert lands a row
  with `mirror_source='ct'`; Delete on a D-shape row (PK populated,
  every projected non-PK column NULL) removes the mirror row;
  Delete on an already-gone row is idempotent. Other 5 mirror
  mappers follow identical structure — backlog (test-suite-analyzer
  P1 follow-up) but skipped here intentionally since the bug under
  repair lives in shared `materialise_row` code.

## [2.53.0] - 2026-04-29

### Added

- **Phase 5.5e — `LegacyMirrorPanels` UI component** + integration on
  the billing/folio detail page (`app/billing/[id]/page.tsx`). Three
  read-only panels render below the printable invoice (hidden in
  print via `no-print`):
  - **Coupons attached** (food / breakfast vouchers from `HT_Cupon`)
  - **Minibar / in-stay POS charges** (from `HT_CheckIn_Product`)
  - **Mid-stay room moves** (from `HT_Changed_Room`)

  Each panel has its own loading/empty/error states so unused
  features at this site (e.g. minibar table is empty at HF Hotel)
  show meaningfully ("ไม่มีข้อมูล") instead of breaking the layout.
  An amber "จากระบบเดิม / view-only" badge on every panel header
  makes the legacy provenance clear at a glance.

  Each panel calls one `/api/legacy-mirror/*` endpoint with the
  legacy `cinNo` from the loaded invoice. `InvoiceData` type
  extended with optional `cinNo` field; pages that don't populate
  it just don't render the panels.

  Phase 5.5 user-visible payoff: receptionists no longer need to
  switch to the .NET app to check coupon / minibar / room-move
  history for a check-in.

## [2.52.0] - 2026-04-29

### Added

- **Phase 5.5d — `/api/legacy-mirror/*` read-only HTTP endpoints.**
  Surfaces the `legacy_mirror.*` schema to the UI:
  - `GET /api/legacy-mirror/coupons?cin_no=…` — coupons attached to
    one check-in.
  - `GET /api/legacy-mirror/products?cin_no=…` — in-stay POS / minibar
    charges per check-in.
  - `GET /api/legacy-mirror/room-changes?cin_no=…` — mid-stay room-
    move audit per check-in.
  - `GET /api/legacy-mirror/pricing` — consolidated reference data
    (extension prices, room prices, pricing tiers up/down) in one
    response so the settings page makes one fetch.

  All four endpoints are thin SELECTs against `legacy_mirror.*` —
  no service layer, no event emission. Endpoints take legacy
  `cin_no` directly (the UI already has it from the loaded check-in
  object) — keeps them decoupled from our PG UUIDs and easy to drop
  on decommission. Unblocks the 5.5e UI panels.

## [2.51.1] - 2026-04-29

### Added

- **Phase 5.5c-b — bootstrap-only snapshot of the 6 transactional
  legacy_mirror tables.** New
  `scheduler::mirror::snapshot_mirror_transactional_tables()` does a
  one-shot DELETE+INSERT for each of `HT_Cupon` (~17,894 rows at HF
  Hotel), `HT_CheckIn_Product`, `HT_Deposit`, `HT_Changed_Room`
  (~3,872 rows), `HT_Bill_Debt_H`, `HT_Bill_Debt_Ds`. Wired into
  `bin/sync --bootstrap` — runs after the canonical reconcile and
  the dimension reload but before the watermark stamp. Closes the
  Phase 5.5c known limitation: pre-DDL historical rows are now
  brought into the mirror once at bootstrap, then CT mappers
  maintain steady state from there. `mirror_source = 'reconcile'`
  distinguishes snapshot rows from CT-incremental ('ct') rows.
- Periodic reconcile (`run_sync` on the 5-min cron) does NOT touch
  these tables — that would defeat the CT real-time mirror. Re-run
  `--bootstrap` manually if drift recovery is ever needed.

## [2.51.0] - 2026-04-29

### Added

- **Phase 5.5c — CT mappers for the 6 transactional `legacy_mirror.*`
  tables.** New `hotel_backend::sync::mappers::mirror` module ships
  six per-table `MssqlChangeMapper` impls (`CuponMirrorMapper`,
  `CheckinProductMirrorMapper`, `DepositMirrorMapper`,
  `ChangedRoomMirrorMapper`, `BillDebtHMirrorMapper`,
  `BillDebtDsMirrorMapper`). Each translates a CT row from the
  corresponding legacy table into an UPSERT (I/U) or DELETE (D) on
  `legacy_mirror.<table>`, sets `mirror_source = 'ct'`, and returns
  `Ok(None)` from `apply` (no `DomainEvent` emission — mirrors are
  opaque pass-through; nothing in our app subscribes to them via the
  bus). Flat per-row dispatch — no aggregate coalescing needed.
- **Phase 5.5c — `bin/sync.rs` extended to 16 tables.**
  `CT_ENABLED_TABLES` adds the 6 mirror tables; `build_mappers()`
  wires each to its `MirrorMapper`. The watcher picks them up on
  the next tick after deploy. PG watermark (1888) > MIN_VALID_VERSION
  (1887) for all new tables, so the v2.49.4 startup guardrail passes
  cleanly on deploy (no bootstrap step required for cutover).
- **Migration 022 — seed `legacy_sync_status` for the 6 new tables**
  so the watcher's per-tick `UPDATE legacy_sync_status WHERE
  table_name = $1` finds rows to update and the dashboard surfaces
  per-table observability for them.

### Known limitations

- **Historical pre-DDL rows are not mirrored.** CT enablement on
  2026-04-29 means MSSQL only has CT history from version 1887
  onward; the existing 17,894 HT_Cupon rows and 3,872
  HT_Changed_Room rows will NOT appear in the mirror unless they're
  touched by the .NET app post-deploy. A bootstrap-snapshot path
  for the 6 transactional mirror tables (full DELETE+INSERT once,
  same pattern as `scheduler::mirror::reload_mirror_dimensions`) is
  the natural follow-up — deferred from 5.5c since the UX
  hypothesis ("show coupons / room-moves on current/upcoming
  check-ins") is satisfied by deltas.

## [2.50.1] - 2026-04-29

### Changed

- **Phase 5.5b — CT enabled on 6 legacy-only tables (HF Hotel only;
  HF Ville pending audit).** Per-table Change Tracking with
  `TRACK_COLUMNS_UPDATED=ON` enabled on `HT_Cupon` (17,894 rows),
  `HT_CheckIn_Product`, `HT_Deposit`, `HT_Changed_Room` (3,872 rows),
  `HT_Bill_Debt_H`, `HT_Bill_Debt_Ds`. Where the natural-key column
  was nullable (`HT_Cupon.cupon_no`, `HT_Deposit.id`,
  `HT_Bill_Debt_H.Bill_No`) it was tightened to NOT NULL first
  (verified zero NULLs and zero duplicates pre-flight). PKs added on
  every table (`PK_<table>`); CT enabled. .NET app unaffected.
  Maintenance window completed 2026-04-29 ~01:00 UTC. CT
  current_version at apply time: 1887. Apply + rollback scripts
  formalized in new `migrations/legacy-mssql/` directory (the prior
  `/tmp/disable-ct-rollback.sql` pattern is retired).
  HF Ville's legacy DB still needs an independent audit + apply
  before its CT mappers can come online.

## [2.50.0] - 2026-04-28

### Added

- **Phase 5.5a — `legacy_mirror.*` schema for opaque pass-through of
  legacy-only features.** New PG schema with 11 mirror tables for the
  legacy-only features per `docs/architecture.md` §11: `ht_cupon`
  (food/breakfast vouchers), `ht_checkin_product` (in-stay POS /
  minibar), `ht_deposit` (standalone deposit ledger), `ht_continuetime`
  (hourly extension price master), `ht_changed_room` (mid-stay room-
  move audit), `ht_rooms_cancel` (per-room cancel audit),
  `ht_rooms_price` (per-customer-type room price overrides),
  `ht_bill_debt_h` + `ht_bill_debt_ds` (credit-sales ledger),
  `ht_order_up` + `ht_order_down` (per-customer-type pricing tiers).
  Each mirror table uses the legacy natural key as PK plus two
  bookkeeping columns: `mirrored_at TIMESTAMPTZ` (last write) and
  `mirror_source TEXT` (`'reconcile'` or `'ct'`). On decommission the
  schema is dropped wholesale.
- **Phase 5.5a — full-table reload of 4 dimension mirror tables in
  `scheduler::sync`.** `HT_ContinueTime`, `HT_Rooms_Price`,
  `HT_Order_Up`, `HT_Order_Down` are slow-changing reference data
  (max ~32 rows per table at HF Hotel) — reloaded via DELETE+INSERT
  in one PG TX per table on the existing reconcile cadence. No CT
  enablement needed on the legacy DB. Lays the foundation for the
  CT-mapper work in Phase 5.5c which will populate the 6 transactional
  mirror tables incrementally.

## [2.49.4] - 2026-04-28

### Added

- **Phase 5 sync — startup-time CT-retention overflow guardrail.**
  After the 2-day shadow-mode soak we discovered a foreseeable trap:
  shadow mode rolls back the PG transaction every tick, which freezes
  `legacy_ct_state.last_seen_version`. Meanwhile SQL Server's CT
  garbage collector keeps running, and once `MIN_VALID_VERSION`
  marches past our frozen watermark on any tracked table the row
  history we'd need to catch up incrementally is gone. Per-table
  observability counters (`bump_skipped` writes outside the TX) hide
  this — the dashboard stays green right up until everything goes red
  at once. The watcher now runs `check_retention()` against every
  CT-tracked table at startup and refuses to start (parallel to the
  existing cold-replay refusal) if any table has overflowed. Operator
  must run `bin/sync --bootstrap` to recover. Override available via
  `LEGACY_SYNC_ALLOW_OVERFLOW=true` (data-loss escape hatch — never
  in production). Documented in `docs/runbook-sync.md` §4b with the
  full recovery sequence and prevention guidance.

## [2.49.3] - 2026-04-27

### Fixed

- **Phase 5 sync — drop dead `Cin_Pay_Status` column from check-in
  aggregate loader.** `parent_loader::load_checkin_aggregate` projected
  `Cin_Pay_Status` from `HT_CheckIn_Pay`, but that column does not
  exist in the legacy schema (verified against the live INSERT captured
  in `docs/legacy-spike/raw/checkout2-20260424-101023/07-events.txt`).
  Result: every CT-driven check-in aggregate load failed with
  `Invalid column name 'Cin_Pay_Status'` — ~470 errors/min, all
  silently swallowed by the skip-and-continue path. Caught during the
  shadow-mode soak (TX rolled back so no PG damage); would have meant
  zero check-in / payment mirroring once flipped live. The mapper
  itself only consumes `id, Cin_No, Cin_Pay_Cash, Cin_Pay_Credit,
  Cin_Pay_Tran, Pay_No`, so dropping the projection is non-functional.
- **Phase 5 sync — close alerting gap that hid the above for ~16h.**
  `bump_counters` and `bump_skipped` unconditionally cleared
  `last_error`, `last_error_at`, and reset `consecutive_failures = 0`
  at the end of every tick. Per-row error increments from
  `record_table_error` were therefore wiped before the dashboard could
  read them, so `legacy_sync_status.consecutive_failures` stayed at 0
  through a 100%-failing 16h window. Threaded an `errored` flag
  through `poll_table` → `bump_counters` / `bump_skipped`; when set,
  the SQL no longer touches the error fields, so per-error increments
  survive into the next tick and the dashboard's
  `consecutive_failures >= 5` alert criteria actually fires.

## [2.49.2] - 2026-04-26

### Changed

- **Phase 5 sync — bump MSSQL pool to 20 + throttle CT retention check
  30s → 5min.** Operational tuning to silence "Timed out in bb8"
  warnings observed on the hotter mappers (HT_CheckIn_H, HT_CheckIn_Ds,
  HT_CheckIn_Pay, HT_Receipt_H — the ones whose mappers also do
  parent-aggregate re-loads). Two complementary changes:
  - **`MSSQL_POOL_MAX_SIZE` env var (default 20).** Was previously
    `DB_POOL_MAX` (default 10). The bb8-tiberius pool is shared by
    writeback + sync + ville-sync, so doubling the cap gives all three
    workers headroom. Backward compatible: `DB_POOL_MAX` still honored
    if set, but the new name is preferred for clarity.
  - **`LEGACY_SYNC_RETENTION_CHECK_INTERVAL_SECS` env var (default
    300).** Was effectively firing every poll-tick (1s) per table — 10
    tables × 1Hz × 1 MSSQL connection each = 10 conn/s of pure
    safety-net overhead. Throttled to once per 5 min per table inside
    the watcher's main loop via a per-table `Instant` map.
    Trade-off: retention overflow is a >48h outage scenario; recovery
    is operator-driven via Slack alert + manual `--bootstrap`
    reconcile, so 5-min detection vs 30s detection is operationally
    equivalent. The 10× reduction in pool pressure removes the bb8
    timeout noise.
  - Data-path CT poll cadence (`CT_POLL_INTERVAL_MS=1000`) untouched —
    latency budget preserved.

## [2.49.1] - 2026-04-26

### Fixed

- **`scripts/sync-status.sh` — mode-aware readiness checks.** The
  cutover-readiness section now detects whether the sync worker is in
  `shadow` or `live` mode (via `docker inspect` of the container's
  `LEGACY_SYNC_SHADOW_MODE` env) and runs different checks accordingly.
  - **Shadow mode** (the cutover-soak state): only checks container
    alive, bootstrap done, no per-table consecutive failures. Soak
    duration is informational (recommend 24h+, not blocking). Doesn't
    check watermark freshness or rows_skipped because those counters
    update inside the polling TX which rolls back in shadow mode —
    stale `legacy_sync_status` is expected design, not a problem.
    Doesn't check reconcile drift growth either, because the watcher
    rolls back its writes and the 15-min reconcile keeps detecting the
    same divergence — that's expected, not a regression.
  - **Live mode**: keeps the strict watermark-fresh + drift-stable
    checks since these signals are real once the worker commits.
  - JSON output adds `mode` field; `ready_to_flip` is mode-aware.

## [2.49.0] - 2026-04-26

### Changed

- **Phase 5 cutover step 1 — wire `LEGACY_SYNC_*` flags into deploy `.env`.**
  The `deploy` job now reads `LEGACY_SYNC_ENABLED` and `LEGACY_SYNC_SHADOW_MODE`
  from GH secrets and writes them into `~/new-hotel-production/.env` on
  evergreen, so the operator's flip survives subsequent deploys (the known
  pitfall documented in `docs/runbook-sync.md` §4a). Both secrets are
  added to the non-empty validation gate. Initial values: both `true` —
  worker enabled, shadow-mode rolling back every TX (observation only).
  Cutover sequence next: `--bootstrap` once, then `./scripts/sync-status.sh
  --watch` to soak 24–48h before flipping `LEGACY_SYNC_SHADOW_MODE=false`
  for live writes.

## [2.48.1] - 2026-04-26

### Reverted

- **Reverted 07b1dcb (`perf(ci): move backend tests into Docker test
  stage`).** The Docker `test` stage RUN step requires reaching the
  runner's PG service container at `localhost:5439`. Buildkit's
  docker-container driver runs the buildkit daemon in its OWN
  container; even with `network: host` + `--allow-insecure-entitlement
  network.host`, the host-network namespace exposed to the test RUN is
  the BUILDKIT container's host, not the runner's host. Result:
  `Failed to connect to test database: PoolTimedOut` on every test
  that calls `common::create_test_pool()`.
  - 3 failures in `test_bookings.rs::booking_*`, then aborted (sequential
    --test-threads=1 means the suite stops at the first PG-touching
    test). CI run 24958364935 — failure log retained for audit.
  - To re-attempt this optimisation we'd need either:
    `driver: docker` on `setup-buildx-action` (uses runner's docker
    daemon, so RUN host-network = runner host-network), or a sidecar
    PG container inside the buildkit network. Both are non-trivial
    and out of scope for this batch.
  - Test-backend reverts to runner-mode cargo with sccache + Swatinem
    rust-cache. Net effect on per-push CI time: zero (we're back to
    the post-cargo-chef baseline).
  - cargo-chef itself (commit e51eea8) remains in place — the
    revert is scoped to the Docker `test` stage + workflow changes.

## [2.48.0] - 2026-04-26

### Changed

- **Tightened `paths-filter` so doc-only commits skip every build job.**
  Previous filter set used a coarse `code` filter that triggered on most
  frontend-side files; doc-only commits (CHANGELOG, docs/**, README.md,
  AGENTS.md) sometimes still triggered jobs depending on which file
  in the root was touched.
  - Renamed `code` → `frontend`. New filter list explicitly enumerates
    every Next.js / tooling path: `app/**`, `components/**`, `lib/**`,
    `public/**`, `__tests__/**`, package files, `next.config.*`,
    `tsconfig.json`, `tailwind.config.*`, `postcss.config.*`,
    `Dockerfile`, `.dockerignore`.
  - Added `migrations/pg/**` and `init-db/**` to the `backend` filter.
    Schema changes affect `sqlx::query!()` compile-time validation, so
    a migration MUST retest + rebuild the backend before deploy. This
    closes a class of subtle drift bugs where the `.sqlx/` cache could
    silently go stale.
  - Added `scripts/backup-db.sh` to the `deploy` filter (it ships into
    the deploy directory; a change must trigger a re-deploy).
  - Module-level comment in the workflow documents the design
    contract: paths NOT covered by any filter intentionally cause every
    job to skip — that's the correct outcome for doc-only commits.
  - All `if:` conditions migrated from `outputs.code` to
    `outputs.frontend`. No catch-all globs introduced; the test/build
    matrix remains legible for future maintainers.

## [2.47.0] - 2026-04-26

### Changed

- **Restored cargo-chef in `hotel-backend/Dockerfile` + `Dockerfile.ville-sync`.**
  Source-only changes (the common case) now hit a cached `cargo chef cook`
  layer and skip the ~800-crate dependency recompile. Expected backend
  build time on warm cache: ~5min → ~1-2min. Cold-cache build (deps
  changed) is unchanged.
  - `chef → planner → builder → runtime` four-stage layout. The planner
    derives `recipe.json` (a hash of the dependency graph); the builder
    `cooks` deps once before COPYing the source. Docker's layer cache
    keys the cook step on `recipe.json` alone — when only source files
    change, the cook layer is a hit and never re-executes.
  - cargo-chef pinned to `0.1.71` (security + reproducibility — there is
    no Dependabot ecosystem for `cargo install` binaries; bumps are
    Dockerfile edits).
  - All Batch C / D guarantees preserved: pinned base-image digest,
    non-root runtime user, OCI labels, HEALTHCHECK, `--locked` flag,
    `inspect_*` debug bins excluded from runtime.
  - Both Dockerfiles share the `hotel-backend-target` cache id so a
    build of either image warms the other's `target/`.

## [2.46.5] - 2026-04-26

### Fixed

- **`customer_search_by_name` flake — shared-DB race, root-caused.**
  The integration suite shares a single PostgreSQL instance and the
  `ht_*` schema namespace. `customer_crud_lifecycle` called the shared
  `tests/common/mod.rs::cleanup` helper, which deleted every row matching
  `cust_notes LIKE 'TEST_%'`. That glob also matched `'TEST_search_by_name'`
  — the marker on the search test's just-inserted rows — so when the two
  tests ran concurrently (cargo's default), the lifecycle cleanup nuked
  the search test's data between its first assertion (1 row, passed)
  and its second (expected 2, got 0). Surfaced as 3 cascading CI failures
  (runs 24956846194, 24957055216, 24957295146).
  - Tightened `cleanup()` to use exact-match markers (`'TEST_customer_crud'`)
    instead of `LIKE 'TEST_%'`, so the helper only touches rows owned by
    the lifecycle test that calls it. Each query now uses parameter
    binding (`= $1`) for safety.
  - Added `--test-threads=1` to the CI `cargo test` invocation as
    defense-in-depth against shared-DB races we haven't enumerated.
    Cost: ~5-10s on a 13-test suite.
  - Documented the rule in the module-level comment of `tests/common/mod.rs`.

## [2.46.4] - 2026-04-26

### Fixed

- **BFG over-scrubbed `REDACTED-sa-pw` from non-credential contexts.** The
  history-rewrite pass replaced every occurrence of the 4 leaked
  literals with `***REMOVED***`, but `REDACTED-sa-pw` was the most generic of
  them — it appeared as fake Thai ID/phone/tax-ID test data, as fallback
  default values in dev binaries, and as the literal in legacy-reference
  decompiled C#. The replacement broke a regex literal in
  `InvoiceTemplate.test.tsx` (turning `/…REDACTED-sa-pw…/` into
  `/…***REMOVED***…/` — `*` is a regex metacharacter, "Nothing to repeat"
  parse error), which cascaded to 3 failed CI runs.
  - Restored `REDACTED-sa-pw` in test fixtures (Thai IDs/phones/tax IDs),
    .env.example fallbacks, dev-binary `unwrap_or_else` defaults,
    sqlx-prepare.sh dev DSN, and legacy-reference decompiled C#.
  - Kept the scrub in `CHANGELOG.md` and one workflow comment, but
    rewrote those entries with descriptive prose ("the SSH password",
    "<hardcoded>") so future readers understand without the literal
    being re-introduced.
  - The actual sensitive `REDACTED-sa-pw` use (HF Ville `sa` MSSQL password
    in `deploy/hfville/docker-compose.yml`) stays gone — it's now
    `${HFVILLE_MSSQL_PASSWORD:?…}` from a GH secret per Batch A.

## [2.46.3] - 2026-04-26

### Added

- **`scripts/sync-status.sh`** — Phase 5 sync worker observability dashboard.
  SSHes to evergreen and prints a sectioned, color-coded report covering:
  container status (sync/writeback/backend/newdb running + healthy?),
  CT watermark + freshness (last_seen_version vs last_polled_at),
  per-table activity (`legacy_sync_status` rows_ingested/skipped/errors),
  reconcile drift (`ht_reconcile_log` unresolved counts + sample),
  and a 6-check cutover-readiness verdict. Modes: `--watch` (refresh
  every 30s), `--json` (machine-readable), `--readiness` (focused
  exit-code-driven check; 0=green-to-flip, 1=not ready). Designed as
  the operator's primary tool for deciding when to flip
  `LEGACY_SYNC_SHADOW_MODE=false`.

### Fixed

- **Disable image-baked HEALTHCHECK on writeback + sync services.**
  The `hotel-backend` Dockerfile bakes a `HEALTHCHECK CMD curl /api/mode`
  for the backend HTTP service. Writeback and sync share that image but
  run different binaries that never bind port 3003 — Docker was reporting
  both as `unhealthy` indefinitely (surfaced by `sync-status.sh`).
  Compose-level `healthcheck: disable: true` per service.

## [2.46.2] - 2026-04-26

### Security

- **Removed unused `tiberius` features carrying vulnerable transitive deps.**
  Dropped `rustls` and `winauth` features from the legacy MSSQL driver. We
  connect to legacy SQL Server in plaintext over WireGuard (no TLS) using
  SA auth (no Windows NTLM), so neither feature was exercised. Eliminates
  4 Dependabot alerts:
  - `rustls-webpki@0.101.7` (GHSA-xgp8-3hg3-c2mh, GHSA-965h-392x-2mh5) — `< 0.103.12`
  - `rand@0.7.3` (GHSA-cq8v-f236-94qc) — pulled in by `winauth` (Windows NTLM)
  Bumped `bb8-tiberius 0.15 → 0.16` (necessitates `bb8 0.8 → 0.9`).
  HIGH alert (#133) on `rustls-webpki@0.103.13` was already patched (we have
  the patched version); will auto-close on next Dependabot scan.

## [2.46.1] - 2026-04-26

### Changed

- **CI/CD operational quality (Batch E of post-Phase-5.5 audit).**
  Final follow-up to the 5x batch audit (A–D shipped 2026-04-26).
  - **`docs/runbook-sync.md` — operator-flip-revert pitfall (doc-only).**
    New §4a "Known operator pitfall — `.env` is rewritten by every CI
    deploy" documents that every master push regenerates
    `~/new-hotel-production/.env` from GitHub Secrets, so SSH-edited
    flag flips (`LEGACY_SYNC_ENABLED=true` etc.) are silently reverted
    on the next deploy. Two remediations laid out side-by-side:
    (a) recommended GH-secrets approach (`gh secret set
    LEGACY_SYNC_ENABLED ...` + add to the deploy heredoc) and
    (b) alternative PG flag-table (`legacy_sync_control`) for mid-tick
    flips without redeploy. Cutover §4 steps 3-4 + rollback §5 now
    cross-link §4a. **Wiring is intentionally NOT included in this
    commit** — the operator decides when to flip the flag for the
    first time before exposing it via secrets.
  - **`init-db-migrations-drift-check` CI job** in
    `.github/workflows/docker-build.yml`. Spins up a throwaway
    `postgres:17-alpine` (same Batch C digest as the runtime newdb),
    applies `init-db/init-hotelnew.sql`, then runs `scripts/migrate.sh`
    and asserts "No pending migrations". Fails loudly with
    `Drift detected: init-db/init-hotelnew.sql is out of sync with
    migrations/pg/.` if the seed file is missing DDL or
    `schema_migrations` rows that any migration adds. Runs on every
    push (PRs catch drift early) and gates the `deploy` job's `if:`.
    Backfilled the four missing seed rows (009, 014, 015, 016) into
    `init-hotelnew.sql` so the check passes on master today.
  - **`-- @transactional false` pragma in `scripts/migrate.sh`.**
    Migration files can now opt out of the default per-migration
    `BEGIN`/`COMMIT` wrap by including the pragma comment in the first
    20 lines. Required for statements PostgreSQL forbids inside a
    transaction (`CREATE INDEX CONCURRENTLY`, `VACUUM`, etc.). The
    runner streams the body via `\set ON_ERROR_STOP on` and records
    the `schema_migrations` row in a separate atomic statement only
    after the body succeeds. Documented in the `migrate.sh` header
    block + `migrations/README.md`. Pragma-detection coverage added
    to `scripts/test-migrate-parse.sh` (case-insensitive,
    extra-whitespace, header-window-bounded — 9 new assertions, 25
    total).

## [2.46.0] - 2026-04-26

### Added

- **`.github/dependabot.yml`** — covers cargo (`/hotel-backend`), npm
  (`/` and `/thai-id-middleware-tauri`), github-actions, and docker
  (`/` and `/hotel-backend`). Weekly cadence. The previous file was
  the GitHub-template stub with `package-ecosystem: ""`.

### Changed

- **CI hygiene (Batch D of post-Phase-5.5 audit).**
  - **`build-frontend`, `build-backend`, `build-ville-sync` now
    `needs:` their respective test jobs** with explicit success-or-
    skipped guards in the `if:`. The `:latest` tag is no longer
    pushed to GHCR if tests fail, which the existing `deploy.if`
    treated as recoverable.
  - **Per-job `permissions:` blocks** (least-privilege GITHUB_TOKEN):
    - `changes`, `test-frontend`, `test-backend`, middleware `build`:
      `{ contents: read }` only.
    - `build-frontend`, `build-backend`, `build-ville-sync`:
      `{ contents: read, packages: write }` (push to GHCR).
    - `deploy`, `deploy-hfville`: `{ contents: read, packages: read }`
      (pull only — they don't push).
  - **GHA cache `scope:`** added to all 3 `docker/build-push-action`
    invocations: `frontend`, `backend`, `ville-sync`. Stops one image
    build's cache from evicting another's.
  - **`cargo test --locked`** in CI (matches the production Dockerfile
    build).
  - **`npm install` -> `npm ci`** in `middleware-build.yml`: refuses to
    update the lockfile, fails on any drift.

### Security

- **Pin third-party actions to commit SHA**:
    dorny/paths-filter           v3      -> d1c1ffe0…
    mozilla-actions/sccache-action v0.0.10 -> 9e7fa8a1…
    Swatinem/rust-cache          v2      -> e18b4977…
    pnpm/action-setup            v4      -> b906affc…
    webfactory/ssh-agent         v0.9.0  -> dc588b65…
    softprops/action-gh-release  v2      -> 3bb12739…
    dtolnay/rust-toolchain       1.89    -> 193d6aa1…
  Each call site has a `# vN.x.y` comment so future bumps stay obvious.
  The `actions/*` and `docker/*` calls remain on float — they're owned
  by GitHub and Docker respectively, so a SHA pin trades trust for
  Dependabot churn with no real attack-surface reduction.
- **Pin `dtolnay/rust-toolchain` to a `1.89` branch SHA** in BOTH
  `docker-build.yml` (was `@stable`) and `middleware-build.yml` (was
  `@stable`). Same Rust toolchain across both pipelines + matches the
  version that generated `.sqlx/` cache files.

## [2.45.3] - 2026-04-26

### Security

- **Image hardening (Batch C of post-Phase-5.5 audit).**
  - **Pinned every base image to digest** in all 3 Dockerfiles +
    `docker-compose.yml`. Sources (Docker Hub registry API on 2026-04-26):
    `node:20-alpine` -> `sha256:fb4cd12c…372293`,
    `rust:1.89-bookworm` -> `sha256:948f9b08…3c84ff`,
    `debian:bookworm-slim` -> `sha256:f9c6a2fd…c5c252`,
    `postgres:17-alpine` -> `sha256:c7526c0f…338609`.
    Stops a base-image hijack from injecting a malicious layer between
    pull and rebuild. Dependabot (`docker` ecosystem, added in Batch D)
    will surface new digests as PRs.
  - **Web runner stage now drops root.** Mirrors the backend pattern:
    creates `nextjs:nextjs` (UID 1001), `chown -R nextjs /app`,
    `USER nextjs`. UID 1001 picked to avoid colliding with the
    backend's UID 1000 if both ever land on the same host.
  - **Web runner stage is bare `node:20-alpine` (NOT `FROM base`).**
    No pnpm, no corepack, no build tooling in the runtime image —
    smaller surface, nothing for an attacker to leverage if they
    get RCE on the web pod.
  - **`hotel-backend/.dockerignore` widened.** Was 6 lines (`target/`,
    `.git/`, `.gitignore`, `Dockerfile`, `.dockerignore`, `*.md`,
    `.env`), now explicitly excludes `node_modules/`, `.next/`,
    `coverage/`, `.github/`, `.vscode/`, `.claude/`, `*.log`,
    `docs/`, `__tests__/`, `app/`, `components/`, `public/`, `lib/`,
    `legacy-reference/`, etc. Defense-in-depth: even if the build
    context ever changes from `./hotel-backend` to `.`, sensitive
    repo state stays out of the image build.
  - **Dropped `inspect_booking` and `inspect_schema` debug binaries
    from the production runtime image.** They remain available via
    `cargo run --bin inspect_*` locally for ops use, but never ship.
    Removes ~10 MB and a debugger-grade Postgres client from the
    image attack surface.
  - **Added `HEALTHCHECK` directives to all 3 runtime stages.** Web:
    `wget --spider http://localhost:3003`. Backend:
    `curl /api/mode`. ville-sync: declared no-op (`CMD true`) — the
    binary has no HTTP endpoint or liveness sentinel today; a deeper
    probe (touch `/tmp/sync_alive` per poll-loop iteration) is a
    follow-up. Container-level health complements the existing
    compose-level healthchecks for non-compose runners.
  - **Added OCI labels** (`org.opencontainers.image.source`,
    `org.opencontainers.image.licenses=Proprietary`) to all 3 images.
    Makes the GHCR repo back-link visible in `docker inspect` and
    `docker scout`.

### Changed

- **Resource limits added to every service in `docker-compose.yml`**
  (`deploy.resources.limits` / `reservations`). Per-service ceilings:
  newdb 2G/2 cpus, backend 1G/2 cpus, web 512M/1 cpu, writeback
  512M/1 cpu, sync 512M/1 cpu. Caps a runaway query or worker from
  starving the host.

## [2.45.2] - 2026-04-26

### Fixed

- **Deploy ordering, concurrency, healthcheck polling, restart policies
  (Batch B of post-Phase-5.5 audit).**
  - **Concurrency guards** on both deploy jobs — `deploy-prod-evergreen`
    and `deploy-hfville` groups, `cancel-in-progress: false`. Two
    back-to-back master pushes can no longer interleave their
    `docker compose up` (the cancel-mid-flight case left
    `migrate.sh` half-applied + backend on stale schema).
  - **Migrations now run BEFORE the new backend image starts**
    (`pull -> up -d newdb -> migrate -> up -d`). Previously:
    `pull -> up -d (full stack) -> migrate -> restart backend`, which
    started the new backend against the OLD schema for ~10s. Operator
    note: backwards-incompatible migrations still need expand/contract
    or a manual `scale backend=0` first — flagged in the deploy step
    comment.
  - **`wait_healthy` polling helper** replaces the fixed `sleep 3/5/5`
    waits. Polls `State.Health.Status` (or falls back to `State.Status`
    for services without a healthcheck) every 2s up to a per-call
    timeout, and dumps the last 50 log lines on timeout.
  - **Backend post-deploy probe** now uses `State.Health.Status`
    (actual healthcheck) instead of `State.Status` (which can be
    `running` mid-crash-loop). Same probe is invoked twice — once
    after `up -d`, once after the writeback/sync recreation — so a
    crash-loop introduced by the worker recreations is also caught.
  - **`deploy-hfville` now `needs: [deploy]`** with
    `needs.deploy.result == 'success'` in the `if:`. Stops a half-broken
    HF Hotel deploy from cascading into a HF Ville push that would
    write to a stale-schema target.
  - **`web` service healthcheck** added (`wget --spider http://localhost:3003`).
    The deploy job's wait_healthy on the backend container now has a
    parallel for the frontend, and `depends_on: backend service_healthy`
    in compose chains correctly.
  - **`writeback` and `sync` restart policy** changed from
    `unless-stopped` to `on-failure:5`. The clean `Ok(0)` exit when
    `WRITEBACK_ENABLED=false` / `LEGACY_SYNC_ENABLED=false` no longer
    triggers the respawn-loop (Docker treats exit 0 as expected under
    on-failure; the `:5` cap still protects against a real hot-loop).
  - **Migration version-comparison hardening** in `scripts/migrate.sh`.
    The previous `grep -q "^${version_padded}$"` would substring-match
    on a multi-line input under some `grep` versions and failed silently
    on malformed filenames. Replaced with:
    1. Strict regex on filename (`^([0-9]{1,3})_.+\.sql$` — anything
       else aborts the script with a clear error).
    2. Explicit set-membership lookup against an associative array
       (`APPLIED_SET[$version_padded]`). Documented expected naming.
  - **Tests:** `scripts/test-migrate-parse.sh` covers 16 cases against
    the parser logic without needing a live PostgreSQL — runs in <1s.

## [2.45.1] - 2026-04-26

### Security

- **CI/CD pipeline credential cleanup (Batch A of post-Phase-5.5 audit).**
  Replaced every hardcoded credential in `.github/workflows/docker-build.yml`
  and `deploy/hfville/docker-compose.yml` with GitHub Secrets references.
  - **Removed `sshpass -p <hardcoded>`** from the `deploy-hfville` job
    (previously plaintext-shipped the jump-box SSH password literal twice
    per deploy and once per `sudo -S`). The job now loads
    `HFVILLE_SSH_KEY` into ssh-agent via `webfactory/ssh-agent@v0.9.0`,
    pins the host key with a one-shot `ssh-keyscan -H` (drops
    `StrictHostKeyChecking=no`), and runs `sudo docker …` directly —
    requires a one-time NOPASSWD sudoers rule on the jump box (see
    "Operator setup" below).
  - **Replaced the production-PG password literal** in the `test-backend`
    job (CI-only Postgres) with `${{ secrets.TEST_POSTGRES_PASSWORD }}`. The
    secret value is constrained to URL-safe characters so it embeds in
    `DATABASE_URL` without percent-encoding.
  - **Replaced the HF Ville hfville-db PG, legacy-MSSQL `sa`, and
    production-PG password literals** in `deploy/hfville/docker-compose.yml` with
    `${HFVILLE_PG_PASSWORD:?…}`, `${HFVILLE_MSSQL_PASSWORD:?…}`, and
    `${POSTGRES_PASSWORD:?…}` (fail-fast if unset). The deploy-hfville
    step writes `~/hfville/.env` from GH Secrets via `umask 077`+heredoc
    so the file is born 600 (no 644-then-chmod race window).
  - **Added secret-non-empty validation** at the top of both `deploy`
    and `deploy-hfville` jobs — fails loud (`::error::`) before writing
    `.env`, so a misconfigured GH secret never silently produces an
    empty value the runtime would accept and misuse.
  - **Removed the `--- .env file check ---` debug echo** from the
    `deploy` job (info disclosure: enumerated env-var key names in the
    log even though it omitted the values).
  - **Operator setup required (one-time, separately from this commit):**
    1. Install the public half of `HFVILLE_SSH_KEY` into
       `~nut/.ssh/authorized_keys` on the HF Ville jump box (<wg-jumpbox>).
    2. Add `nut ALL=(ALL) NOPASSWD: /usr/bin/docker, /usr/bin/docker compose`
       to `/etc/sudoers.d/nut-docker` on the jump box (replaces the
       `echo <password> | sudo -S …` pattern).
    3. After the operator confirms steps 1–2, rotate the in-place
       passwords on the jump box's PG + MSSQL + production-push target.
    4. Future password rotations: edit the corresponding GH Secret +
       re-run the deploy workflow — no code change needed.

## [2.45.0] - 2026-04-26

### Added

- **Phase 5.5 — production cutover scaffolding for the CT watcher.**
  The 10-table mapper coverage shipped in 5.4 (commit ea3dae0); 5.5
  ships the operational scaffolding so an operator can flip the
  watcher live.
  - **`sync` docker service block** (`docker-compose.yml`). Same image
    as `backend` + `writeback`, different command (`./sync`), under
    `profiles: [legacy]` so plain `docker compose up` skips it.
    Activate via `docker compose --profile legacy up -d`. Ships
    default-DISABLED — `LEGACY_SYNC_ENABLED=false` causes the binary
    to log + exit 0 cleanly. `.github/workflows/docker-build.yml`
    `deploy` step now force-recreates the `sync` container alongside
    the existing writeback recreation, with a state check that
    surfaces `running` (live) vs `exited` (default-disabled — the
    expected post-deploy state) without failing the deploy.
  - **`bin/sync --bootstrap` flag.** Cold-start path the operator
    runs ONCE before flipping `LEGACY_SYNC_ENABLED=true`. Steps:
    (1) verify schema fingerprint, (2) invoke
    `scheduler::sync::run_sync` in temporarily-overridden `upsert`
    mode to bring canonical PG state up to date with MSSQL, (3) read
    `CHANGE_TRACKING_CURRENT_VERSION()` from MSSQL, (4) UPSERT it as
    the watermark via direct UPDATE (overwrites unconditionally — does
    NOT use `watermark::advance`'s `<=` guard, so a partial prior run
    can be force-recovered). Exit 0. Bootstrap runs INDEPENDENTLY of
    `LEGACY_SYNC_ENABLED` — operator bootstraps first, then flips the
    flag (per `docs/runbook-sync.md` cutover sequence).
  - **Cold-replay refusal** (`bin/sync.rs` main loop). When
    `last_seen_version=0` (the migration's seed value) AND
    `LEGACY_SYNC_ALLOW_COLD_REPLAY != true` (default), the watcher
    refuses to start with a clear error message pointing at
    `--bootstrap`, fires a Slack alert, sleeps 60s before exit (to
    throttle Docker `restart: unless-stopped` cadence), and returns
    Err. Mitigates the "process every CT row from time-zero" footgun
    flagged in the original Phase 5 plan.
  - **Migration 019 — `ht_reconcile_log`.** Drift-detection tripwire
    table consumed by the demoted `scheduler::sync::run_sync`
    diff-only mode. Schema: `id`, `detected_at`, `table_name`,
    `legacy_pk`, `pg_hash`, `mssql_hash`, `mssql_row_json`,
    `pg_row_json`, `resolved_at`. Two partial indexes on
    `WHERE resolved_at IS NULL` for cheap dashboard / alerting reads.
    Mirrored in `init-db/init-hotelnew.sql` for fresh deployments.
  - **`docs/runbook-sync.md`** — operator runbook covering: bootstrap
    procedure, env-var matrix (every flag, default, when to flip),
    Slack alert meanings, cutover procedure (deploy → bootstrap →
    shadow soak 24h → enable → live soak 24h), rollback procedure,
    known limitations (payment-cancel cascade race window, receipt
    `pay_method='cash'` default, MSSQL probe in tests), 5-scenario
    receptionist test plan (create / cancel booking, check-in /
    check-out, add / cancel payment) for the receptionist team to
    run during live soak, and observability dashboard SQL pointers
    (`legacy_sync_status`, `legacy_ct_state`, `event_log` filter,
    `ht_reconcile_log` unresolved-set query).
  - **`SYNC_TEST_SKIP_MSSQL_PROBE` env var.** QoL escape hatch in
    `tests/test_sync_phase54_integration.rs::mssql_stub`. When set,
    skips the bb8-tiberius probe and returns `None` immediately —
    saves ~30s per test process when MSSQL is unreachable. Documented
    in the runbook env-var matrix.
  - **Tests:** `tests/test_scheduler_sync_diff_only.rs` (4 cases —
    PG-miss logged, hash divergence logged, canonical state
    untouched, partial-index unresolved-set query) +
    `tests/test_sync_phase55_bootstrap.rs` (6 cases — watermark
    overwrite semantics, cold-replay sentinel contract, post-bootstrap
    non-zero invariant, plus 3 compile-time guards on the env-var /
    flag literal names that must stay in sync with the runbook).

### Changed

- **`scheduler::sync::run_sync` demoted to diff-only safety net**
  (Phase 5.5). The CT watcher (`bin/sync`) is now authoritative for
  canonical PG state. The legacy 5-min full-UPSERT job is downgraded
  to a 15-min drift-detection tripwire that LOGS divergent rows into
  `ht_reconcile_log` instead of UPSERTing canonical state.
  - Env var `LEGACY_SYNC_RECONCILE_MODE` controls behaviour:
    `diff_only` (default) = hash-compare + log; `upsert` = original
    behaviour, kept as an escape hatch for operational rollback.
    Unknown values fall back to the safe (non-mutating) default with
    a warning log.
  - Cron schedule changed from `0 */5 * * * *` (every 5 min) to
    `0 */15 * * * *` (every 15 min — quarters of the hour for easier
    log correlation). Operators relying on the legacy 5-min cadence
    must understand canonical state is now sub-second-fresh via the
    CT watcher; the 15-min reconcile is purely a safety net.
  - Per-entity log lines now include the active mode tag (e.g.
    `[Sync] Customers (DiffOnly): … added, … updated, … unchanged …`)
    so operators can tell at a glance which path ran.

## [2.44.1] - 2026-04-26

### Added

- **Phase 5.4 — checkin + payment CT mappers (completing 10-table CT
  coverage). Every CT-enabled table now has a real mapper or an
  intentional retired stub.**
  - **`sync::parent_loader::load_checkin_aggregate(cin_no)`** — thin
    wrapper around the 5.3-factored `fetch_rows` helper. Owns
    `HT_CheckIn_H` (where_col=`Cin_no`), `HT_CheckIn_Ds` (where_col
    `Cin_No` — capital N, verbatim from legacy schema), and
    `HT_CheckIn_Pay`. Returns `CheckInAggregate { header, rooms,
    payments }`.
  - **`sync::resolve` module** — factored out of `mappers/booking.rs`
    for symmetry with the check-in mapper. Exposes
    `resolve_customer_id`, `resolve_booking_id`, `resolve_room_id`,
    `resolve_checkin_id`. Defer-on-missing semantics
    (`Ok(None)` → caller skips publish, next tick re-resolves).
  - **`sync::mappers::checkin`** ships two `MssqlChangeMapper` impls
    (`CheckInHeaderMapper`, `CheckInRoomsMapper`) that delegate to the
    shared `apply_checkin_aggregate` helper. The helper:
    - Re-loads the aggregate via `load_checkin_aggregate`.
    - Idempotently UPSERTs `ht_checkins` (skip-on-no-change).
    - Emits exactly one `CheckInCreated` / `CheckInCancelled` /
      `CheckOutCompleted` event per aggregate per tick.
    - On full check-out (every `Cin_Room_Status='Check-Out'`) AND a
      non-empty `Cin_Book_no`, also re-projects the parent booking
      aggregate inside the same TX so `Book_Status='ออกแล้ว'` →
      `book_status='completed'` propagates atomically.
  - **`sync::mappers::payment`** ships two impls — `PaymentMapper`
    (`HT_CheckIn_Pay` — coalesces by `Cin_No` so payment changes flow
    through the check-in aggregate sweep, keeping `cin_paid_amount`
    in sync with the legacy `Total_Price_Pay`) and `ReceiptMapper`
    (`HT_Receipt_H` — per-row dispatch, UPSERTs into `ht_payments`,
    emits `PaymentReceived`). Receipt cancellations
    (`status_name='ยกเลิก'`) flip `pay_voided=true`.
  - **`Cin_status` Thai literal mapping** — `'ปกติ'` (and unknown) →
    `'active'`; `'ยกเลิก'` → `'cancelled'`. Per-room
    `Cin_Room_Status='Check-Out'` (English with HYPHEN, verbatim from
    legacy) on every detail row promotes the canonical status to
    `'checked_out'`. Header cancel always wins.
  - **`RoomStatusMapper` retired**. `HT_Room_Status` CT rows are now
    documented log-only stubs; the check-in aggregate is the source
    of truth for "which room is occupied tonight". The mapper stays
    registered so the table appears in `legacy_sync_status`
    observability — operators can still see CT row counts tick.
  - **Wiring**: `bin/sync.rs::build_mappers` now wires real mappers
    for `HT_CheckIn_H`, `HT_CheckIn_Ds`, `HT_CheckIn_Pay`, and
    `HT_Receipt_H` (was `NoopMapper` in 5.3). The coalesced dispatch
    path branches on table name to route booking / checkin / payment
    aggregates through their respective `apply_*_aggregate` helpers.
  - **`apply_checkin_aggregate` mssql parameter is `Option<&DbPool>`**
    so contexts without legacy access (unit tests, walk-in flows
    that never trigger the parent re-projection side-effect) can
    pass `None`. The watcher always passes `Some(&pool)`.
  - 30+ new unit tests covering legacy-status mapping, projection,
    coalescing dedup, idempotency, FK defer paths, checkout
    detection, and event-shape regression. 6 new integration tests
    against testcontainers PG covering walk-in upsert, idempotent
    re-apply, status flip to cancelled, header-delete cancel,
    full-checkout transition, and customer-defer behavior.
  - **Migration**: none needed — `ht_checkins.legacy_*` +
    `aggregate_id` already added in migration 014;
    `ht_payments.pay_reference` (the receipt-no key) is part of the
    baseline schema.

- **Phase 5.3 — booking aggregate CT mapper (3-table HT_Book_H +
  HT_Book_Ds + HT_Book_Date with parent re-load + per-tick coalescing
  emitting one DomainEvent per aggregate).**
  - **`sync::parent_loader`** (`hotel-backend/src/sync/parent_loader.rs`) —
    `load_booking_aggregate(book_no)` pulls header + every line + every
    calendar night for one booking from MSSQL into a `BookingAggregate`
    struct of `MappableRow` projections. Internal `fetch_rows` helper
    is generic over `(table, where_col, projection)` so 5.4's
    `load_checkin_aggregate(cin_no)` is one extra public wrapper, no
    refactor.
  - **`sync::mappers::booking`** ships three `MssqlChangeMapper` impls
    (`BookingHeaderMapper`, `BookingRoomsMapper`, `BookingDatesMapper`)
    that all delegate to a shared `apply_booking_aggregate` helper.
    The helper re-loads the aggregate, idempotently UPSERTs
    `ht_bookings` + `ht_booking_rooms`, and emits exactly one
    `BookingCreated` / `BookingModified` / `BookingCancelled` event
    per aggregate per tick.
  - **Coalescing layer in `bin/sync.rs::poll_table`.** Mappers opt in
    via the new `MssqlChangeMapper::coalesce_key(row)` trait method;
    when present, the watcher groups CT rows by aggregate root key
    (HashSet dedup) and dispatches `apply_booking_aggregate` exactly
    once per unique key per tick. Customer / room mappers return
    `None` (the trait default) and stay on the legacy 5.2 per-row
    dispatch path.
  - **`MappableRow` extended with a decimal arm** (`try_get_decimal`)
    so booking-header monetary columns (`Book_Price_Total`,
    `Book_Price_Pay`) project cleanly. Stored as `f64` to match the
    existing writeback (`writeback/format.rs`) + repository
    (`$N::float8` casts) precedent — see the rustdoc on the trait
    method for the rationale.
  - **`HashMapRow::MockValue::Decimal`** mirrors the new arm;
    `try_get_f64` and `try_get_decimal` cells are interchangeable so
    fixtures pick whichever variant reads more naturally.
  - **`Book_Status` literal mapping** (`'จอง'` → `'confirmed'`,
    `'เข้าพัก'` → `'checked_in'`, `'ยกเลิก'` → `'cancelled'`,
    `'ออกแล้ว'` → `'completed'`, anything else → `'pending'`) —
    legacy literals stay verbatim per the user's standing constraint;
    PG canonical column reuses the existing string convention from
    `routes/new_bookings`.
  - **Customer / room FK resolvers** in the booking mapper defer the
    apply when `legacy_cust_no` (resolved against `ht_customers`) or
    `room_no` (resolved against `ht_rooms_new`) hasn't been mirrored
    yet — the next tick's customer / room CT row brings the booking
    in via the same code path.
  - **Real wiring in `bin/sync.rs::build_mappers`** for `HT_Book_H`,
    `HT_Book_Ds`, `HT_Book_Date` (was `NoopMapper` in 5.2). Mapper
    count and CT-enabled-table list unchanged.
  - 28 new unit tests covering legacy-status mapping, projection,
    coalescing dedup, idempotency, and event-shape regression. 5 new
    integration tests under `tests/test_sync_phase53_integration.rs`
    covering insert / re-apply / modify / cancel / coalescing
    end-to-end against the canonical PG.
  - **No new migration** — `ht_bookings.legacy_book_id` /
    `legacy_cust_no` / `aggregate_id` already added by migration 014.
    `ht_booking_rooms` schema (already shipped) accommodates the
    projection.
  - **`LEGACY_SYNC_ENABLED` stays default-false.** Operator decision
    to flip live; the watcher remains opt-in.

## [2.43.1] - 2026-04-26

### Added

- **Phase 5.2 — customer + room CT mappers (read-only mode).**
  - **Migration 018** (`018_ht_customers_aggregate_keys.sql`) adds
    `legacy_cust_no VARCHAR(20)` and `aggregate_id UUID` to
    `ht_customers`, with partial unique indexes mirroring the
    migration-014 pattern. Required so the new CT mapper can map MSSQL
    `Cust_no` to the canonical PG row and emit `DomainEvent` payloads
    keyed on a stable aggregate UUID.
  - **`MappableRow` trait** (`hotel-backend/src/sync/row.rs`) — testable
    abstraction over a single CT-projection row. Production impl wraps
    `tiberius::Row`; the `HashMapRow` fixture (also reused by the
    watcher binary as the boundary representation) means production and
    tests both flow through the same `MappableRow` code path. Mapper
    `apply` signature changed from `Option<&tiberius::Row>` to
    `Option<&dyn MappableRow>`.
  - **Real JOIN dispatch in `bin/sync.rs`.** `poll_table` now composes
    the per-mapper `CHANGETABLE(CHANGES …) LEFT JOIN <table>` query
    using `mapper.select_sql()` + `primary_key_cols()`, parses each row
    into a `HashMapRow`, dispatches to the mapper, and persists the
    returned `DomainEvent` into `event_log` via `EventBus::publish`
    (live mode) or logs "would publish" + rolls back (shadow mode).
    Per-table TX with per-table watermark advance after commit. `0x4E48`
    `SYS_CHANGE_CONTEXT` filter applied on the SELECT.
  - **`CustomerMapper`** (`hotel-backend/src/sync/mappers/customer.rs`,
    ~370 LOC). Full I/U/D coverage for `HT_Customers`. UPSERT into
    `ht_customers` resolved by `legacy_cust_no`; `aggregate_id` derived
    via `service::ids::aggregate_uuid(AggregateKind::Customer, …)` and
    pinned on first insert. Idempotent — re-applies with identical
    content skip publication. D events soft-delete via
    `cust_deleted_at = now()` and emit no `DomainEvent` in 5.2
    (subscribers will re-add it as needed in later phases). Translates
    legacy Thai `Cust_Type_Main` literal back to the canonical
    `CustomerType` enum.
  - **`RoomMasterMapper`** (`hotel-backend/src/sync/mappers/room.rs`).
    Mirrors `HT_Rooms` (Room_Clean / Room_Use) into `ht_rooms_new` per
    user constraint "stick to current setup we have in HOTEL legacy
    app for now" — no new English status taxonomy, no metadata schema
    additions, no `RoomMasterChanged` event variant. Emits
    `RoomMarkedClean` / `RoomMarkedDirty` only when `Room_Clean`
    actually flips. Resolves rows by `legacy_room_id_int` then
    `room_no`. Refuses to auto-create unknown rooms (operator runs
    `bin/backfill_rooms` first).
  - **`RoomStatusMapper`** (same file). Phase 5.2 logging stub for
    `HT_Room_Status` — the per-night occupancy ledger is owned by the
    upcoming 5.3 / 5.4 booking + checkin mappers (rebuilt from
    `HT_Book_Date` / `HT_CheckIn_Ds`); duplicating it here would
    diverge from the canonical reconstruction. Returns `Ok(None)` for
    every row in 5.2.
  - **Tests.** 33 unit tests across `sync::mappers::customer` (10),
    `sync::mappers::room` (8), `sync::row` (5), plus 4 wiring tests in
    `bin/sync.rs` confirming each table dispatches to the right mapper
    type. Integration suite `tests/test_sync_phase52_integration.rs`
    (6 tests, gated on live PG via `DATABASE_URL`) covers the
    customer-insert / customer-idempotent-reapply / customer-update /
    customer-soft-delete + room-clean-flip / room-clean-no-op end-to-end
    against the real `event_log`.

  Phase 5.2 stays opt-in: `LEGACY_SYNC_ENABLED` defaults to `false`
  (the watcher binary exits 0 when disabled, intentionally). Flipping
  it to `true` in production is an operator decision.

## [2.43.0] - 2026-04-26

### Added

- **Phase 5.1 — Change Tracking watcher skeleton.** First sub-phase of
  the MSSQL→PG sync half of the decommission boundary
  (`docs/architecture.md` §3.6d, §3.7, §4d-tris, §10 #8). Lands the
  scaffolding the 5.2+ per-table mappers will plug into without changing
  any existing flow:
  - **New binary `bin/sync.rs`** — long-running CT watcher modeled on
    `bin/writeback.rs`. Honours `LEGACY_SYNC_ENABLED` (intentional
    disable exits 0, not 1, so Docker `restart: unless-stopped` doesn't
    loop). Parses `LEGACY_SYNC_SHADOW_MODE`, `LEGACY_SYNC_TABLE_ALLOWLIST`,
    `CT_POLL_INTERVAL_MS` (default 1000ms). Verifies the legacy schema
    fingerprint, surfaces drift to Slack, sleeps 60s before exit to
    throttle Docker restart cadence. Per-mapper panic isolation via
    `tokio::spawn`, SIGTERM drains the in-flight tick. Per-poll
    `MIN_VALID_VERSION` retention check refuses to advance past CT
    cleanup and Slack-alerts ops with the recovery path. Ships 10
    `NoopMapper`s in 5.1 — real mappers swap in one-by-one in 5.2+.
  - **New module `src/sync/`** — `MssqlChangeMapper` async-trait + the
    `NoopMapper` placeholder, `ChangeOp` typed wrapper around CT's
    `'I'`/`'U'`/`'D'` `SYS_CHANGE_OPERATION` codes,
    `watermark::{read_last_seen, advance}` against `legacy_ct_state`,
    and a `SyncError` enum covering sqlx/tiberius/bb8 + a typed
    `RetentionOverflow` variant for the CT-cleanup path.
  - **Loop-prevention chokepoint.** `db::mssql_session::set_context_info`
    issues `SET CONTEXT_INFO 0x4E48` ("NH" = New Hotel) on the writeback
    session. SQL Server CT surfaces it as `SYS_CHANGE_CONTEXT`; the
    watcher's `CHANGETABLE` SELECT filters those rows out. Wired as the
    first statement of `writeback::dispatcher::dispatch` — the single
    entry point through which every recipe runs. Belt-and-suspenders:
    5.2+ mappers will be idempotent UPSERTs so a missed tag costs at
    most one extra cycle.
  - **Per-table observability table `legacy_sync_status`** (migration
    017) — pre-seeded for the 10 CT-enabled tables (`HT_Customers`,
    `HT_Rooms`, `HT_Room_Status`, `HT_Book_H`, `HT_Book_Ds`,
    `HT_Book_Date`, `HT_CheckIn_H`, `HT_CheckIn_Ds`, `HT_CheckIn_Pay`,
    `HT_Receipt_H`). Rows ingested / skipped / last error / consecutive
    failure count so operators can spot a wedged mapper without
    log-tailing. Same migration adds `ht_customers.cust_deleted_at` for
    the upcoming HT_Customers `D` (delete) mapper in 5.2.
  - **Tests** — exhaustive unit coverage of the new code: `ChangeOp`
    parsing (I/U/D recognised, unknown rejected, char round-trip),
    allowlist parsing (none/blank/comma/whitespace), mapper-builder
    filtering, `CT_ENABLED_TABLES` ↔ migration-017 seed parity,
    structural assertion that `dispatch()` calls `set_context_info`
    BEFORE the recipe `match`, structural assertion that
    `count_ct_rows` filters `SYS_CHANGE_CONTEXT <> 0x4E48`. Integration
    test `tests/test_sync_watermark.rs` exercises the read/advance
    round-trip against a live PG.

  Phase 5.1 is intentionally behavior-preserving: every entry in the
  watcher's mapper list is a `NoopMapper`, the watermark is left
  un-advanced (commented-out with a 5.2 TODO so we don't silently skip
  the rows the real mappers will need), and no `docker-compose.yml`
  service block is wired (5.5). The chokepoint is the only code change
  that touches an existing runtime path, and its only effect on the
  legacy DB is one extra `SET CONTEXT_INFO 0x4E48` statement per
  writeback session.

## [2.42.0] - 2026-04-26

### Changed

- **`docs/architecture.md`** — incorporated 2026-04 reverse-engineering
  findings from `legacy-reference/`. Recipe-level audits (booking, check-in,
  checkout/payment) found no new code divergences — recipes are aligned with
  decompiled C#. Doc updates only:
  - Fixed misleading "owns its own IDs (UUIDs)" bullet in §1 ASCII diagram —
    PG owns UUIDs internally; writeback emits legacy-shape string IDs
    (`C0001`, `R000001`, `CH26-000001`).
  - New §3.7 "Ground-truth principle" — codifies the 3-tier source-of-truth
    precedence (live captures > decompiled C# > inferred analysis), with the
    `HT_CheckIn_Ds.id IDENTITY` mistake as the case-in-point. Documents
    pricing source (`HT_Rooms_Price` not `HT_Rooms.Room_PriceA/B/C`), VAT 7%
    inclusive split formula, mixed Thai/English status enum landmines, and
    the `varchar Thai_CI_AS` text-encoding rule.
  - §4a — added legacy ID format table + the `MAX(id)+1 + TABLOCKX/HOLDLOCK`
    allocation pattern for the two non-IDENTITY PKs (`HT_CheckIn_Ds.id`,
    `HT_Receipt_H.id`).
  - §8 roadmap — promoted Phase 5 (CT watcher) to **TOP PRIORITY** and
    framed it as the missing half of co-existence; added Phase 5.5
    (read-only mirror tables for legacy-only entities — coupons, deposits,
    products, room moves, etc.).
  - §10 #8 — Change Tracking marked enabled & live-verified (was warning).
  - New §11 "Legacy-only features (opaque pass-through)" — explicit table
    of 10 legacy-only tables we mirror but never write to, plus 10 legacy
    behaviors we don't replicate (coupons, standalone deposits, in-stay
    POS, credit sales, hourly pricing, room-move, photos, SMS, etc.).

## [2.41.1] - 2026-04-26

### Added

- **`legacy-reference/`** — reverse-engineering artifacts derived from the
  original Windows `HOTEL.exe` binary, complementary to (not authoritative
  over) `docs/legacy-spike/`. Contains:
  - `analysis/` — `_FEATURE_MAP.md` (every form + navigation graph),
    `_COMPAT_CHEATSHEET.md` (1,901-line code-derived coexistence contract),
    `_REPORTS_INVENTORY.md` (46 Crystal Reports cataloged + QuestPDF
    replacement plan), `_SCHEMA.sql` (all 61 CREATE TABLEs).
  - `decompiled-source/` — buildable C# reference codebase (~298 .cs files)
    reconstructed from `HOTEL-cleaned.exe` via ilspycmd. Loads in
    Rider/VS/VSCode for F12 navigation; doesn't fully build (~280 known
    decompiler-artifact errors documented in its own README), but that
    doesn't matter for reference use.
  - `binaries/` — original `HOTEL.exe` (obfuscated, Dec 2024),
    `HOTEL-cleaned.exe` (de4dot output with .NET Reactor protections
    stripped and string literals decrypted), and `HOTEL.pdb` (debug
    symbols — what made the decompile so clean).
  - `vendor/` — commercial 3rd-party DLLs the .csproj references
    (DotNetBar, C1FlexGrid, BarcodeLib, etc.). Internal use only;
    do not redistribute.
  - Excluded from Docker builds via `.dockerignore`.

  Where this folder and `docs/legacy-spike/` disagree, **trust
  `docs/legacy-spike/`** — it's based on live Extended Events captures of
  the running app; this folder is inferred from decompiled source.

## [2.41.0] - 2026-04-25

### Fixed

- **Writeback recipes now emit byte-for-byte legacy SQL.** Re-derived
  every `INSERT INTO [HT_*]` statement against the live capture log
  (`/tmp/legacy-events-full.log`, 270 MB of `.Net SqlClient` events
  recorded from the production legacy app), then patched each recipe
  to match column order, casing, value forms, and Thai literals
  exactly. Receivers diffing our writeback output against legacy
  output now see no spurious deltas.

  Recipe-level changes:

  - `payment.rs` (`HT_CheckIn_Pay`):
    - Reordered to the legacy 20-column canonical layout (PriceTotal
      precedes PriceOne).
    - Added 6 missing columns: `Cin_Pay_Note`, `Pay_by`,
      `Cin_Pay_Free`, `Cin_Pay_Tran`, `Branch`, `Cin_Pay_web`.
    - `Cin_Pay_Ds` now carries the room number (was empty); unit
      column emits the literal string `'รายการ'` (was integer 1);
      product-id is `'P001'` (the payment-row code, distinct from
      the receipt-line `'SEV-001'`); branch defaults to
      `'สำนักงานใหญ่'` (Thai "head office"); per-night vs total
      price split honored.
  - `payment.rs` (`HT_Receipt_H`):
    - VAT-inclusive split (`Total / 1.07`) computed via the new
      `vat_inclusive_split` helper. `Receipt_VatIn='True'`,
      `Receipt_VatPer=7`.
    - Added missing columns: `Receipt_ref` (lowercase r),
      `Receipt_note` (Thai stay-period blurb),
      `Receipt_noteUP` (`'Booking'` when sourced from a booking),
      `Receipt_Tax` (caller-supplied tax/customer ID).
    - `status_name='ปกติ'` (was empty).
  - `walkin.rs` + `checkin_to_booking.rs` (`HT_CheckIn_H`):
    - Dropped 3 obsolete columns (`Total_Price_vat`, `Cin_note`,
      `Cin_Work_number`); fixed casing on `Cin_Date_Out` and
      `Cin_Type`; reordered `Cin_by` before `Cin_Date_in`.
    - `Cin_status='ปกติ'` (was `'เข้าพัก'` — that's the Ds-level
      `Cin_Room_Status` value, not the header status); added new
      `CIN_STATUS_NORMAL` constant.
    - `Cin_cust_price='ราคาปกติ'` (was empty);
      `Cin_Room_ALL='{room_no} '` with trailing space;
      `Cin_foreign='False'` (literal string); `Cin_Type=0` integer;
      `Total_Price_Pay=0.00` and `Total_Price_Balance=Net` (settle
      is now correctly pre-payment).
  - `walkin.rs` + `checkin_to_booking.rs` (`HT_CheckIn_Ds`):
    - Renamed `[Cin_Dep_Status]` to lowercase `[Cin_dep_status]`;
      dropped `[Dep_by]`; value is `'ไม่เก็บค่ามัดจำ'` (added
      `CIN_DEP_STATUS_NONE` constant).
  - `walkin.rs` + `checkin_to_booking.rs` (`HT_CheckIn_Other_People`):
    - Replaced hardcoded `'Mr. {name}'` with country-aware prefix
      heuristic (`'TH*'` → `'นาย'`, else `'Mr.'`) and a trailing
      space matching the legacy `name + ' ' + name2` pattern when
      `name2` is empty. Plumbing the actual `Cust_perfix` through
      the payload is a separate task.
  - `walkin.rs` + `checkin_to_booking.rs` (`Tb_Save_Image`):
    - Added optional photo-link UPDATE driven by new
      `CreateCheckInPayload.photo_tmp_no`. The .NET app fires this
      on every save (UPDATE matches 0 rows when no photo was
      uploaded). When the field is `None` we skip the UPDATE.
  - `booking_create.rs` (`HT_Customers`):
    - Dropped 4 obsolete columns (`Cust_sex`, `Cust_IDcard`,
      `Cust_Contry`, `Cust_Work_Tax`); added `Cust_Last_Change`
      with today's Bangkok date.
    - `Cust_Type_Main='ราคาปกติ'` (was `'บุคคลธรรมดา'` — that
      latter form is the UPDATE path's value, not INSERT). Added
      `CUST_TYPE_MAIN_NORMAL` constant; kept
      `CUST_TYPE_MAIN_INDIVIDUAL` for the UPDATE-path value.
  - `booking_create.rs` (`HT_Book_H`):
    - Dropped `[Book_Notify_Note]`; emits `Book_Notify_Day,Book_sale`
      WITHOUT square brackets to match the legacy column-list builder.
    - `Book_Date_in/out` are date-only forms (`'4/25/2026'`) — not
      midnight datetimes; the .NET booking-list view binds to the
      date string directly.
  - `booking_modify.rs` (`UPDATE [HT_Customers]`):
    - Extended the customer re-save from 26 to 31 SET fields,
      adding `Cust_Work_tax`, `Cust_perfix`, `Cust_sex`,
      `Cust_IDcard`, `Cust_Contry`. Renamed `[Cust_Type_Main]` to
      lowercase `[Cust_Type_main]`. `WHERE` becomes lowercase `where`.
      Extended `CustomerResave` payload with the 5 new fields.
  - `booking_cancel.rs`:
    - Documented the per-room DELETE pattern the legacy app emits;
      the bulk DELETE is functionally equivalent and remains in
      place until multi-room tracking lands.

  New helpers:

  - `format::vat_inclusive_split` — splits a VAT-inclusive total
    into `(before_vat, vat)` rounded to 2dp. Three tests assert
    parity with captured legacy receipts.
  - `format::money_2dp` — renders a finite f64 with exactly 2
    decimal places, the form the legacy app uses for VAT split
    and tender columns.
  - `constants::CIN_STATUS_NORMAL` (`'ปกติ'`),
    `CIN_DEP_STATUS_NONE` (`'ไม่เก็บค่ามัดจำ'`),
    `CUST_TYPE_MAIN_NORMAL` (`'ราคาปกติ'`),
    `BRANCH_HEAD_OFFICE` (`'สำนักงานใหญ่'`),
    `PAY_DS_NAME_ROOM` (`'ค่าห้อง'`),
    `PAY_DS_UNIT_ITEM` (`'รายการ'`),
    `PAY_DS_ID_ROOM` (`'P001'`),
    `RECEIPT_VAT_PERCENT=7`,
    `RECEIPT_VAT_INCLUSIVE` (`'True'`),
    `RECEIPT_STATUS_NORMAL` (`'ปกติ'`),
    `RECEIPT_NOTE_UP_BOOKING` (`'Booking'`),
    plus the `receipt_stay_note(check_in, check_out)` helper.

  Each touched recipe gained a `*_matches_legacy_capture_byte_for_byte`
  test that asserts column-list + value-tail substrings against the
  exact statement captured in `/tmp/legacy-events-full.log`. Total
  test count grew from 122 to 147 (25 new tests).

## [2.37.0] - 2026-04-25

### Added

- **Legacy booking number displayed in our app's UI.** `BookingListRow`
  and `BookingDetailRow` now carry `legacy_book_id` (the `R\d{6}`
  identifier the writeback worker back-populates after mirroring to the
  .NET app). The `/api/new/bookings*` JSON response surfaces it as
  `legacyBookId` (omitted when absent — booking hasn't been written
  back yet). The bookings list table shows it as a small mono-font
  reference under the booking number. Lets receptionists cross-
  reference a single booking between our app and the legacy app
  without manual lookups.

  `repository::booking::get` was converted from `sqlx::query!` macro
  to runtime `sqlx::query()` so column additions don't require
  regenerating the offline cache for every change.

## [2.36.0] - 2026-04-25

### Fixed

- **`booking_create` recipe now bumps `HT_Book_Date.Book_ok`** —
  closes the 2026-04-25 incident where bookings written via our app's
  writeback (R014831/2/3/...) appeared in the .NET app's booking-list
  view but NOT in the calendar grid view (room view).

  Per spike capture `booking-checkin/writes.txt:27`, the legacy app's
  pattern after every `HT_Book_Date` INSERT is to immediately fire
  `update HT_Book_Date set Book_ok=Book_ok+1 where id=<id>`.
  `Book_ok` defaults to 0 on INSERT; the calendar grid view filters
  for rows where `Book_ok > 0` (treating `Book_ok = 0` as draft state),
  so without the bump our bookings stayed invisible in the grid.

  Fix is a single additional `update HT_Book_Date set Book_ok=Book_ok+1
  where id={id}` after each night's INSERT. New regression test
  `book_date_inserts_each_followed_by_book_ok_increment` locks in both
  the count (one bump per night) and the ordering (bump immediately
  follows its INSERT).

  Existing bookings R014831/2/3 written before this release will need
  manual patching — either receptionist re-saves them in the .NET app
  (which fires the legacy pattern) or operator runs:
  `UPDATE HT_Book_Date SET Book_ok=Book_ok+1 WHERE Book_no IN
   ('R014831','R014832','R014833') AND Book_ok=0`.

  119 lib tests pass.

## [2.35.0] - 2026-04-25

### Fixed

- **Writeback worker — final 6 audit findings closed in parallel.**
  Three worktree-isolated agents tackled the deferred items
  concurrently; reconciled into 3 commits on top of `ac4ca67`.

  - **HIGH-1: atomic `mark_failed`** (commit `25ccb3c`). Folded the
    backoff computation into the post-claim `attempts` already on
    `ClaimedJob`, eliminating the second SELECT (`get_attempts_for_backoff`
    deleted). Removes the read-modify-write race against the janitor in
    a multi-worker deploy.
  - **MED-2: claim-gated `mark_done`/`mark_failed`** (commit `25ccb3c`).
    `ClaimedJob` carries `claimed_at: DateTime<Utc>`; the UPDATE adds
    `AND status='in_progress' AND claimed_at = $X`. On 0-row response
    the worker logs "re-claimed by another worker; discarding result"
    and skips back-population, preventing two workers from racing to
    write different `legacy_*` values into the same `ht_*` row.
  - **MED-4: throttled self-heal Slack alert** (commit `d3307cb`).
    `salvage_legacy_ids` now records each event into a process-local
    counter; after 5 events within 5 minutes a single Slack message
    fires (with the inspection SQL the operator should run), then
    resets. Pure `should_alert(state, now, threshold, window)` extracted
    for unit testing.
  - **LOW-2: resolution Slack on exhausted→done** (commit `d3307cb`).
    `mark_done`'s UPDATE rewritten as a CTE capturing the prior status
    atomically with the flip; on `prior_status='exhausted'` posts a
    `:white_check_mark:` Slack via new `send_resolved_alert`. Operator
    sees closure, not just alarms.
  - **LOW-3: listener auto-respawn** (commit `d3307cb`). `run_listener`
    wrapped in `run_listener_supervised` that loops with 5s backoff
    after every error; after 10 consecutive failures fires Slack and
    backs off to 60s. Never gives up — exiting would silently degrade
    the worker to 30s polling.
  - **MED-1: `validate_finite` in `booking_modify` + `checkin_cancel`**
    (this commit). Adds NaN/Infinity guards at the top of `execute()`
    in both recipes, matching the pattern in `payment.rs` /
    `extend_stay.rs` / `booking_create.rs`. 4 new unit tests.

  All 118 lib + 15 binary tests pass. Release build clean.

  Operational footprint of the audit work (commits `e6e5e66` →
  `25ccb3c`, ~10 commits):
  - 2 CRIT, 4 HIGH, 5 MED, 4 LOW closed.
  - 3 PG migrations: 014 (legacy_* columns), 015 (writeback_jobs retry
    state).
  - Lib tests: 78 → 118 (+40); binary tests: 0 → 15.
  - Self-heal, panic isolation, atomic claim-gating, exponential
    backoff, exhausted state, Slack alerts on exhaustion + schema drift
    + resolution + listener degradation. End-to-end error catching /
    retry / recovery / reporting now production-ready for the live
    receptionist test.

## [2.34.0] - 2026-04-25

### Fixed

- **Writeback worker — final pre-live-test hardening** (audit findings
  HIGH-2, HIGH-3, MED-3, MED-5, LOW-1).

  - **HIGH-3 — panic isolation.** Every `process_job` call now runs in
    `tokio::spawn`; a panic inside any recipe (or a tiberius driver
    bug) is caught via `JoinHandle::is_panic`, the job is force-
    exhausted with the panic message, and the main loop continues.
    Without this, a single panic would kill the worker; Docker would
    restart it but the in-flight job would sit `in_progress` for 5
    minutes before the janitor reclaimed — invisible to the operator
    until the alert fired elsewhere.
  - **HIGH-2 — non-retryable errors short-circuit to exhausted.**
    `mark_failed_with_retryable` checks `WritebackError::is_retryable()`
    and skips the retry budget for deterministic failures (intent
    mismatch, recipe business-rule errors, payload deserialize, schema
    drift). Slack alert fires immediately instead of waiting 12 minutes
    of wasted retries on the same payload.
  - **MED-5 — schema-drift Slack flood throttle.** Worker sleeps 60s
    after posting the schema-fingerprint Slack alert and before
    returning `Err`, so Docker's `restart: unless-stopped` doesn't loop
    the worker every 5s and page the operator 6×/min.
  - **MED-3 — exhausted-job Slack instruction.** The `:rotating_light:`
    Slack message now includes the full reset SQL (`status='pending',
    attempts=0, next_retry_at=NULL`) instead of just the status — the
    previous instruction would leave `attempts` at the cap and
    re-exhaust on the first retry.
  - **LOW-1 — head+tail error truncation in Slack.** `truncate_head_tail()`
    keeps the first 200 + last 300 chars (multi-byte safe for Thai),
    preserving the row context tiberius/sqlx errors put at the end.

  3 new binary unit tests for `truncate_head_tail` (short pass-through,
  long head+tail preservation, Thai multi-byte safety). 114 lib + 7
  binary tests pass.

## [2.33.0] - 2026-04-25

### Added

- **Operational hardening for the writeback worker** before first live
  test — error catching / retry / recovery / reporting paths now closed.

  **Recovery from stuck `in_progress` jobs.** Worker process crashes
  mid-recipe, or `mark_done`/`mark_failed` PG failures, previously left
  the `writeback_jobs` row in `status='in_progress'` forever and
  required manual SQL intervention. Migration 015 adds `claimed_at`;
  `claim_next_job` now also re-claims `in_progress` rows whose
  `claimed_at < NOW() - 5 min` (`STUCK_IN_PROGRESS_TIMEOUT_SECS`).
  No more stuck jobs.

  **Retry backoff.** A `failed` job was previously re-claimable on the
  next loop tick, burning through `WRITEBACK_MAX_ATTEMPTS` in seconds
  during an MSSQL outage. Migration 015 adds `next_retry_at`;
  `mark_failed` schedules retries with exponential backoff (30s, 2min,
  10min by default) and `claim_next_job` filters by it.

  **`exhausted` terminal status.** Once `attempts >= max_attempts` the
  worker now transitions the row to `status='exhausted'` instead of
  leaving it as `failed` (which was indistinguishable from "currently
  retrying"). Operator triage is one query:
  `SELECT * FROM writeback_jobs WHERE status='exhausted'`.

  **Slack alerts on terminal failures.** New
  `send_exhausted_alert()` posts a `:rotating_light:` message to
  `SLACK_WEBHOOK_URL` (when configured) the moment a job exhausts its
  retry budget, with the intent name, aggregate id, attempt count,
  truncated last error, and the recovery one-liner. Schema-fingerprint
  failures at startup also alert before the worker exits — the operator
  sees the failure even if they're not tailing logs.

  4 new unit tests for the backoff + timeout constants. 114 lib +
  4 binary tests pass.

  Operator notes:
  - Set `SLACK_WEBHOOK_URL` to enable alerts; otherwise failures are
    log-only.
  - To manually retry an `exhausted` job after fixing the underlying
    cause: `UPDATE writeback_jobs SET status='pending',
    attempts=0, next_retry_at=NULL WHERE id=...`.
  - To inspect queue depth: `SELECT status, COUNT(*) FROM
    writeback_jobs GROUP BY status`.

## [2.32.0] - 2026-04-25

### Fixed

- **Writeback resolver self-heals from `writeback_jobs.legacy_ids`** —
  closes audit LOW-1 (back-population race). The `writeback_jobs` audit
  row's `legacy_ids` JSONB is the **source of truth** for what the recipe
  allocated; the `ht_*.legacy_*` columns are a denormalized cache. If
  `mark_done` fails to back-populate the cache (PG hiccup, pool
  starvation, network glitch between MSSQL COMMIT and the PG UPDATE),
  the next intent's resolver now falls back to a `SELECT legacy_ids
  FROM writeback_jobs WHERE aggregate_id = $1 AND status = 'done' ORDER
  BY completed_at DESC LIMIT 1` lookup. Logs `warn!` so an operator can
  see when the cache is stale, but no manual intervention is needed —
  the system recovers automatically.

  Also: `mark_done` now retries back-population with bounded
  exponential backoff (3 attempts, 100/400/1600 ms) before giving up.
  Most transient failures self-recover at write time without ever
  reaching the resolver fallback.

  Net effect: the previously-documented "manually patch the row"
  recovery path is no longer required. Worst case is one extra SELECT
  per future intent on the affected aggregate.

## [2.31.0] - 2026-04-25

### Fixed

- **Writeback resolver — close CRIT-3.** The Phase 4b writeback worker's
  resolver in `bin/writeback.rs` queried `legacy_book_id` /
  `legacy_cin_no` / `legacy_room_no` columns that never existed in the PG
  schema, and joined on `WHERE id = $1` against tables whose primary keys
  are actually `book_id` / `cin_id` / `room_id` (and they're SERIAL `i32`,
  while the intent's `*_id` is a `Uuid`). Result: only `CreateBooking` and
  walk-in `CreateCheckIn` worked end-to-end; every modify / cancel /
  extend / checkout / payment / mark-clean intent silently failed at
  resolution time.

  Closes the gap with three coordinated changes (commit `<this release>`):

  - **Migration 014** adds `legacy_book_id` / `legacy_cust_no` to
    `ht_bookings`; `legacy_cin_no` / `legacy_room_no` / `legacy_cust_no`
    / `legacy_checkin_ds_id` to `ht_checkins`; `legacy_room_no` /
    `legacy_room_id_int` to `ht_rooms_new`; plus an `aggregate_id UUID`
    column on each (UNIQUE partial index, NULL allowed for pre-migration
    rows). `init-db/init-hotelnew.sql` mirrors the same columns for
    fresh deployments.
  - **Service layer** (`service/booking.rs`, `service/checkin.rs`) now
    stamps `aggregate_id = aggregate_uuid(kind, serial_id)` onto each
    freshly inserted row in the same transaction as the outbox enqueue.
    New `BookingRepository::set_aggregate_id` and
    `CheckInRepository::set_aggregate_id` methods. Without this stamp,
    the worker's resolver cannot map UUID→row.
  - **Writeback worker** now resolves via `WHERE aggregate_id = $1` (no
    more spurious `id` column reference). The renamed `mark_done`
    callback also back-populates the freshly allocated `legacy_*`
    identifiers (e.g. `R014812`, `CH26-005230`) onto the canonical PG
    row, so the next intent on the same aggregate (modify, cancel,
    extend, etc.) resolves immediately. Walk-in / check-in-to-booking
    recipes capture `SCOPE_IDENTITY()` after the `HT_CheckIn_Ds` INSERT
    via the new `recipes::execute_capturing_identity_at` helper, so
    `legacy_checkin_ds_id` is available for ExtendStay / CheckOut.
  - **`backfill_rooms`** also writes `legacy_room_no` / `legacy_room_id_int`
    / `aggregate_id` so MarkRoomClean works on the existing 58 rooms
    without waiting for the next room mutation.

  New `LegacyIds.with_room_no()` and `with_checkin_ds_id()` builders;
  walkin / checkin_to_booking populate both. JSON shape of
  `writeback_jobs.legacy_ids` extended with `room_no` and `checkin_ds_id`.

  All 114 lib tests pass — no recipe SQL changed, only orchestration
  around the existing INSERT statements.

## [2.30.0] - 2026-04-25

### Fixed

- **Writeback safety audit — CRIT/HIGH/MED gaps closed before live test.**
  Independent re-audit of the merged writeback worker (commit `d02d20d`)
  surfaced 2 CRIT bugs that would corrupt data on the first receptionist
  action, plus 3 HIGH and 2 MED issues. All addressed in this release.
  One CRIT (PG schema missing `legacy_*` resolver columns) is documented
  as task #24 — substantive design work, not a quick patch.

  **CRIT (2):**
  - **CRIT-1: Recipes now run inside an explicit MSSQL transaction.**
    Previously `bin/writeback.rs` issued statements in autocommit mode,
    which releases `TABLOCKX, HOLDLOCK` at the end of each statement —
    voiding the spike §6 race-safety guarantee. The MAX+1 SELECT and the
    INSERT it feeds were no longer in the same lock scope, so a
    concurrent worker / .NET client could read the same value. Wrapping
    `dispatch()` in `BEGIN TRAN ... COMMIT/ROLLBACK` restores the
    guarantee and gives us atomic rollback on partial-recipe failure.
  - **CRIT-2: Bangkok timezone conversion for every legacy datetime.**
    `format_legacy_datetime` previously called `dt.naive_utc()`, which
    treats a real UTC instant as Thai-local. Recipes pulling from PG
    `TIMESTAMPTZ` columns and `Utc::now()` would write times 7h behind
    the wall clock the receptionist actually entered. New `chrono-tz`
    dependency; `format_legacy_datetime`, `midnight_of`,
    `end_of_stay_at_almost_noon`, and `enumerate_calendar_nights` all
    convert to `Asia/Bangkok` first. New `format::bangkok_date` and
    `format::format_bangkok` helpers used across recipes.

  **HIGH (3):**
  - **HIGH-1: `booking_modify` falls back to existing room_no when the
    user only changes dates.** Previously every new `HT_Book_Date` row
    got `Book_type=''`, making the booking disappear from the .NET app's
    calendar grid. New `ModifyBookingInputs.existing_room_no` field;
    `execute()` queries MSSQL for the current `Book_type` when needed.
  - **HIGH-3: `payment` recipe accumulates instead of overwriting
    totals.** Previously set `Total_Price_Pay={amount}` and
    `Balance=0`, losing prior partial payments and clobbering Room/Net
    totals (which `booking_create` / `extend_stay` own). Now uses
    additive `Total_Price_Pay = ISNULL(...,0) + amount` and recomputes
    `Balance = Net - new_Pay`.
  - **HIGH-4: NaN/Infinity guard before SQL formatting.** New
    `helpers::validate_finite()` rejects non-finite f64 at recipe
    `execute()` entry. `format!("{}", f64::NAN)` would have produced
    the literal string `"NaN"` in SQL. Wired into `payment`,
    `extend_stay`, and `booking_create`.

  **MED (2):**
  - **MED-1: Empty legacy ID strings are now treated as missing.**
    `dispatcher.rs` resolution previously accepted `Some("")` as a
    valid resolved ID, producing silent no-op `WHERE Cin_no=''`
    UPDATEs. New `nonempty()` helper used at every resolution site.
  - **MED-3: TM.30 random IDs constrained to positive i32 range.**
    `rand::random::<i32>()` could produce negatives that the .NET
    WinForms grid may mishandle. New `positive_i32()` helper masks the
    sign bit.

  **Test suite:** 99 → 104 writeback unit tests (5 new regressions).
  Existing tests' wall-clock-as-UTC fixtures updated to Bangkok-local-as-UTC
  (e.g. `Utc.with_ymd_and_hms(_, _, _, 12, 0, 0)` became `(_, _, _, 5, 0, 0)`
  to mean noon Bangkok). All 114 lib tests pass.

## [2.29.0] - 2026-04-25

### Fixed

- **Writeback recipe gaps from spike-vs-recipe audit** — comprehensive
  follow-up to commit `0179f81` (which closed the room-display gap on
  booking create). The audit compared every recipe in
  `hotel-backend/src/writeback/recipes/` against the captured legacy SQL
  in `docs/legacy-spike/raw/*/writes.txt` and `docs/legacy-spike/findings.md`
  §3a–k and surfaced 17 divergences. This release fixes 16 of them in
  one branch (gap #1 was a miscount in the audit and verified
  already-correct). New shared helper `writeback/recipes/helpers.rs`
  hosts cross-recipe SQL (`mark_cupon_printed`).

  **HIGH severity (3):**
  - `booking_modify` now re-saves `HT_Customers` (spike §3c capture
    lines 5/16/28) when the new `BookingChanges.customer_resave` payload
    is set. Phone/address edits will propagate to the customer master
    once the route is enriched.
  - `payment` now updates `HT_CheckIn_Ds.Cin_Room_Pay_Total` per-room
    (spike §3h capture line 3) when `RecordPaymentCommand.checkin_ds_id`
    is `Some`. Multi-room revenue tracking restored.
  - `extend_stay` now emits two leading TM.30 touches
    (`UPDATE HT_CheckIn_H SET Cin_Work_number=<random>`) per spike §3a +
    §3f capture lines 1-2. Random `i32` per touch via the new `rand`
    dependency.

  **MEDIUM severity (6):**
  - `walkin` and `checkin_to_booking` both now emit
    `UPDATE HT_Cupon SET cupon_print=1` (spike §3a `walkin/writes.txt:9`
    / §3d `booking-checkin/writes.txt:39`) via the shared
    `mark_cupon_printed` helper.
  - `booking_modify` clears stale `HT_Rooms` display fields BEFORE the
    date diff (spike §3c capture lines 6/14/17) — fires on every modify
    to keep the calendar grid in sync.
  - `booking_modify` re-writes the `HT_Rooms` display caption with the
    new dates after a stay-range change (spike §3c capture line 26)
    when the new `BookingChanges.new_customer_name` field is supplied.
    Mirrors the `booking_create` caption format from `0179f81`.
  - `checkin_to_booking` `UPDATE HT_Customers` now spans the full ~25
    address + work field set (spike §3d capture line 28) instead of just
    the 5 fields it previously updated.
  - `payment` increments `HT_CheckIn_H.Total_Price_vat` (spike §3h
    capture line 6) — this hotel uses no VAT but the .NET app still
    writes the running total, so we mirror for parity with reports.
  - `payment` clears `HT_CheckIn_Product` defensively (spike §3h capture
    line 2) — almost always a no-op since we don't write that table, but
    emitted for byte-level parity with the legacy capture.

  **LOW severity (6):**
  - `payment` `HT_CheckIn_Pay` INSERT now includes the `[Cin_Pay_Ds]`
    column (spike §3h capture line 4 / schema column 4, varchar(500)).
    Empty-string default mirrors the .NET capture.
  - `payment` `HT_Receipt_H` INSERT — `[Receipt_c_no]` column verified
    against the schema baseline (column 4, varchar(50), nullable). The
    recipe already included it; documented the rationale (storing
    `cin_no` here lets receipts trace back to the originating check-in).
  - `extend_stay` payload extended with `stay_start`, `guest_label`, and
    the four totals (`new_room_price_total`, `new_net_total`,
    `new_pay_total`, `new_balance_total`). The recipe now enumerates
    calendar nights across the full `[stay_start, new_end)` range
    instead of `[today, new_end)`, fixing the bug where extends from a
    past start date would leave gaps in `HT_Room_Status`.
  - `checkin_cancel` payload extended with `room_price` + `pay_to_subtract`
    so the recipe can SUBTRACT the right amounts from `HT_CheckIn_H`
    totals (multi-room safe per spike §3i). Previously zeroed,
    leaving legacy totals stale.
  - `payment` recipe logs a warning when `RecordPaymentReceipt` is
    fully empty (all three of `customer_name` / `address` / `tel`
    blank) — surfaces silent route-enrichment failures in worker logs.

  **Audit gap #1 (CRIT, claimed):** verified the
  `booking_create.rs` `HT_Customers` INSERT has 33 columns and 33
  values aligned correctly (`'บุคคลธรรมดา'` lands in `[Cust_Type_Main]`
  at position 31). The audit's gap claim was a miscount; no fix needed.

  **Payload extensions:**
  - `outbox::intent::WritebackIntent::CancelCheckIn` gained `room_price`,
    `pay_to_subtract`.
  - `outbox::intent::WritebackIntent::ExtendStay` gained `stay_start`,
    `guest_label`, `new_room_price_total`, `new_net_total`,
    `new_pay_total`, `new_balance_total`.
  - `outbox::intent::WritebackIntent::RecordPayment` gained
    `checkin_ds_id: Option<i32>` for per-room apportionment.
  - `outbox::intent::BookingChanges` gained `new_customer_name` and
    `customer_resave: Option<CustomerResave>`. New `CustomerResave`
    struct mirrors the .NET app's full `HT_Customers` UPDATE field set.
  - `service::checkin::CancelCheckInCommand` and `ExtendStayCommand`
    gained matching fields. `service::payment::RecordPaymentCommand`
    gained `checkin_ds_id`.
  - All payload struct extensions use `#[serde(default)]` so jobs
    enqueued under the previous schema continue to deserialize cleanly
    (zero-downtime upgrade).

  **Tests:** all 107 lib tests pass (97 in `writeback::*` — up from 78).
  Added 11 new tests covering the cupon helper, walkin / checkin-to-booking
  cupon emission, the full HT_Customers field set, the `extend_stay`
  TM.30 touches and night-enumeration, the `payment` per-room
  apportionment / cart-clear / VAT accumulator / `Cin_Pay_Ds` column,
  the `booking_modify` room-book clear ordering, customer re-save, and
  caption rewrite, and the `checkin_cancel` price subtraction.

  **New dep:** `rand = "0.8"` (default-features-off, `std` + `std_rng`)
  for the TM.30 batch-number generator.

## [2.28.1] - 2026-04-25

### Added
- **One-shot rooms backfill binary** (`hotel-backend/src/bin/backfill_rooms.rs`).
  Mirrors the 58 rooms in legacy `HT_Rooms` and the 8 room types in
  `HT_SET_RoomType` into `ht_rooms_new` + `ht_room_types`, preserving the
  legacy integer ids as PG primary keys so the writeback worker can resolve
  `room_id → legacy room_no` and so the frontend's room picker has rows.
  Idempotent (ON CONFLICT DO UPDATE). Inverts `Room_Clean` per spike §3i.
  New `backfill-rooms` service in `docker-compose.yml` under
  `profiles: [backfill]`. Run via `docker compose --profile backfill run --rm backfill-rooms`.

### Fixed
- **Writeback payload gaps** — bookings synced from the new app to legacy MSSQL
  appeared in the .NET booking list with empty `Book_Cust_Name`,
  `Book_Cust_Tel`, `Book_Room_Type` (room number column), and
  `Book_Room_Price=0`. Same blanks on `HT_CheckIn_Ds.Cin_Room_No` /
  `Cin_Room_Type` / `Cin_Room_Price` for check-ins, and on the receipt
  header (`HT_Receipt_H.Receipt_Name` / `Address` / `Tel`) for payments. Root
  cause: route handlers (`create_booking`, `create_checkin`, `create_payment`)
  filled the writeback context with empty placeholders — a known gap flagged
  in [2.28.0]. Fix lookups + forwards customer/room metadata into the
  writeback intent payload:
  - `routes::new_bookings::create_booking` — looks up the customer
    (`ht_customers`) and the first assigned room (`ht_rooms_new`) before
    constructing `BookingWritebackContext`. Pulls `customer_name`,
    `customer_phone`, `room_no`, `room_type` (from the joined `ht_room_types`),
    and per-night price (request override → room weekday default →
    booking total).
  - `routes::new_checkins::create_checkin` — same pattern; resolves the
    customer through the booking for booking-linked check-ins, otherwise
    uses the request `customer_id`. Populates `room_no` / `room_type` /
    `price_per_night` (with weekday-default fallback) and
    `guest_name_for_registry` for the walk-in customer INSERT and the
    TM.30 primary-guest row.
  - `routes::new_payments::create_payment` — looks up the check-in's
    customer to populate the receipt header
    (`Receipt_Name` / `Address` / `Tel`).
  - **`outbox::intent::WritebackIntent::RecordPayment`** gained a `receipt:
    RecordPaymentReceipt` field carrying the receipt-header metadata. New
    `RecordPaymentReceipt` struct (customer_name + address + tel) lives in
    `outbox::intent`. Recipe signature
    `writeback::recipes::payment::execute` updated to consume it.
  - **`service::payment::RecordPaymentCommand`** gained a matching `receipt`
    field. The other two contexts (`BookingWritebackContext`,
    `CheckInWritebackContext`) already had the right shape — only the route
    helpers needed enrichment.
  - Lookup failures (deleted customer / deleted room between form submit
    and route) degrade to empty strings rather than failing the canonical
    write — preserves the legacy app's tolerance for orphaned FKs and
    matches the prior behavior for receipts.

## [2.28.0] - 2026-04-25

### Added
- **Phase 4b — writeback worker** (per `docs/architecture.md` §3.6c, §6, §8).
  New `hotel-backend/src/writeback/` module + `bin/writeback.rs` binary that
  drains the `writeback_jobs` outbox into the legacy MSSQL DB.
  - **9 recipes** in `writeback/recipes/`, each a faithful translation of
    `docs/legacy-spike/findings.md` §3a–k:
    - `walkin` (§3a) — 7 INSERTs + 3 UPDATEs across 7 tables for a walk-in
      check-in. Allocates `Cust_no`, `Cin_no`, `HT_Customers.id`, and
      `HT_Room_Status.id` under TABLOCKX.
    - `checkin_to_booking` (§3d) — like walk-in but skips the customer
      INSERT, sets `Cin_Book_no`, marks the booking `'เข้าพัก'`, and
      UPDATEs the first existing `HT_Room_Status` row instead of INSERTing
      it.
    - `booking_create` (§3b + §3k visibility) — 4 INSERTs (5 if customer
      new). Sets `book_room_type=2`, `Book_status=1`, midnight
      `Book_Date_in/out`, `Book_Notify_Day=3`, all-empty optional varchars
      so the booking is visible in the .NET app's main booking-list view.
    - `booking_modify` (§3c) — **targeted UPDATEs only**. Deliberately
      skips the legacy app's destructive DELETE-everything+REINSERT
      pattern; diffs `HT_Book_Date` rows to drop dates that no longer
      apply and INSERTs new ones with `WHERE NOT EXISTS` for idempotency.
    - `booking_cancel` (§3g-bis) — 4 UPDATEs + 1 DELETE. Soft-cancels
      `HT_Book_H` (`'ยกเลิก'`) and `HT_Book_Ds` (`status=3`); hard-deletes
      `HT_Book_Date`. Preserves the duplicate `book_status` UPDATE for
      byte-for-byte parity with the .NET capture.
    - `checkin_cancel` (§3i) — 7 statements. Allocates `HT_Rooms_Cancel.id`
      under TABLOCKX. **Subtracts** the cancelled room's price from
      `HT_CheckIn_H` totals (multi-room safe). Lights off with the
      cancel-specific note `'ปิดไฟ เนื่องจากยกเลิกห้องพัก'`.
    - `extend_stay` (§3f) — 5 fixed UPDATEs + N HT_Room_Status INSERTs.
      Skips the destructive Phase B per spike recommendation.
    - `checkout` (§3e Phase 2 ONLY) — 5 UPDATEs. **Skips Phase 1** (the
      destructive DELETE+REINSERT pattern in the legacy app). Uses
      `'Check-Out'` (hyphen) on `HT_CheckIn_Ds` and `'Check Out'` (space)
      on `HT_Room_Status` per spike §4c verbatim.
    - `payment` (§3h) — 4 statements: payment + totals refresh + receipt
      header + receipt line. Cash → `Cin_Pay_Cash`; credit/transfer →
      `Cin_Pay_Credit`. Receipt label `'ค่าห้องพัก [{room_no}]'`,
      service code `SEV-001`, unit `'คืน'`, no-VAT zeros.
    - `mark_clean` (§3j) — 2 statements + a prior-occupant lookup query.
      Filters by `HT_Rooms.id` (not `room_no`!) per spike §4e. Looks up
      the **most recent non-cancelled** occupant of the room for the
      `HT_Housewife.h_cin/h_cin_name` audit fields.
  - **Race-safe MAX+1 ID allocation** via `WITH (TABLOCKX, HOLDLOCK)`
    (verified live in spike §6 Test 2 — receptionist's UI hitched 10s
    but never erred). Helpers in `writeback/allocate.rs` cover all 10
    legacy ID counters (`Cust_no`, `Book_ID`, `Cin_no`, `Pay_No`,
    `Receipt_no`, `HT_Book_Date.id`, `HT_Room_Status.id`,
    `HT_Rooms_Cancel.id`, `HT_CheckIn_Ds.id`, `HT_Receipt_H.id`).
  - **Schema fingerprint guard** (`writeback/fingerprint.rs`): on startup
    hashes live MSSQL `(table, ord, column, type)` for the 10
    fingerprinted tables and refuses to start if it differs from the
    captured baseline. Fingerprint:
    `5f2c17bc402edfc80e04fecb9dd741e26ed4cf1036f16855626051cd276376d2`.
  - **Worker loop** (`bin/writeback.rs`): `LISTEN writeback_channel` +
    30-sec poll fallback, atomic `UPDATE … RETURNING` claim with
    `FOR UPDATE SKIP LOCKED` so multiple worker replicas can run safely.
    SIGTERM drains in-flight, then exits. Toggle via `WRITEBACK_ENABLED`
    env var (per architecture §7).
  - **Date format helpers** (`writeback/format.rs`): `M/D/YYYY h:mm:ss tt`
    matching .NET's `CultureInfo.InvariantCulture` per spike §4b.
    `format_legacy_datetime`, `format_legacy_date`, `date_to_ole_serial`
    (for `room_date_oa`), `sql_quote` (with `''`-doubling), `midnight_of`
    for §3k visibility.
  - **Constants** (`writeback/constants.rs`): mixed Thai/English literals
    copied verbatim per spike §4c — `'เข้าพัก'`, `'ยกเลิก'`, `'Check-Out'`
    (hyphen), `'Check Out'` (space), power-log note templates, etc.
  - **docker-compose**: new `writeback` service under `profiles: [legacy]`
    so State C deploys (`docker compose up`) skip it. Activate with
    `docker compose --profile legacy up -d`.
  - **90 unit tests** (`cargo test --lib writeback::`) cover SQL output,
    section-3k visibility, mixed Thai/English literal preservation, and
    skipping the destructive Phase 1/B patterns.

### Changed
- `hotel-backend/Cargo.toml` version bumped 2.8.3 → 2.9.0 (new feature).
- Added `signal` feature to tokio dependency for SIGTERM handling in the
  writeback worker.

## [2.27.3] - 2026-04-25

### Fixed
- **Migration 011/012/013 deploy failure** — each file's body contained a
  redundant `INSERT INTO schema_migrations ... ON CONFLICT DO NOTHING`,
  which collided with `scripts/migrate.sh`'s appended INSERT (without
  ON CONFLICT) in the same transaction:
  `duplicate key value violates unique constraint "schema_migrations_version_key"`.
  Removed the internal INSERTs; tracking is owned by `migrate.sh` (it also
  records the file checksum, which the internal INSERTs did not).

## [2.27.2] - 2026-04-25

### Fixed
- **Backend integration tests** (`hotel-backend/tests/test_outbox.rs`):
  - `test_enqueue_inserts_row` asserted on the wrong JSON path: with
    `serde(tag="intent", content="payload")` the variant fields are wrapped
    under `payload`, and the `CreateBooking` variant has a struct field also
    named `payload` — so the inner `CreateBookingPayload` lives at
    `payload.payload`, not `payload`. Assertion adjusted; no source change.
  - `test_publish_inserts_event_log_and_notifies` and
    `test_rollback_emits_no_event_and_no_notify` raced against each other on
    the shared `domain_events` PG channel under cargo's parallel test runner.
    Added a `recv_for_booking` helper that drains `pg_notify` messages until
    one matches the test's own `booking_id` (or times out), so cross-test
    notifications no longer cause spurious failures.

## [2.27.1] - 2026-04-25

### Security
- **Bumped Next.js 16.1.6 → 16.2.4** (and `eslint-config-next` to match) — closes
  6 advisories: DoS via Server Components, request smuggling in rewrites,
  unbounded `next/image` cache, unbounded postponed-resume buffering, null-origin
  Server Actions CSRF bypass, null-origin dev HMR websocket CSRF bypass.
- **Forced patched transitive deps via pnpm overrides**: `lodash >=4.18.0`,
  `handlebars >=4.7.9`, `postcss >=8.5.10`, `flatted >=3.4.2`, `ajv >=6.14.0`,
  scoped `brace-expansion`/`minimatch`/`picomatch` to patched versions per
  affected major. Resolves ~19 transitive advisories (lodash code-injection,
  handlebars JS-injection, postcss XSS, flatted prototype-pollution, ajv ReDoS,
  brace-expansion DoS, minimatch ReDoS, picomatch ReDoS).
- **Backend `cargo update`**: `rustls-webpki 0.103.9 → 0.103.13` (CRL-panic
  DoS + CRL-distribution-point logic), `rand 0.8.5 → 0.8.6` in both
  `hotel-backend` and `thai-id-middleware-tauri`.
- 3 low-severity Rust advisories remain transitively pinned by `tiberius@0.12.3`
  (latest), and will resolve when MSSQL is decommissioned per
  `docs/architecture.md`: `rand@0.7.3` (via `winauth`) and two
  `rustls-webpki@0.101.7` name-constraint issues (via `rustls@0.21`).

### Fixed
- **CI `test-backend` job**: install `mold` + `clang` on the Ubuntu runner so
  `hotel-backend/.cargo/config.toml`'s `-fuse-ld=mold` link flag resolves.
  Previously `cargo test` failed with `invalid linker name in argument
  '-fuse-ld=mold'` because the Dockerfile installs mold but the bare-runner
  backend test job did not.

### Removed
- **Legacy `thai-id-middleware/` (Electron) sub-project** — superseded by
  `thai-id-middleware-tauri/` (the only target of `.github/workflows/middleware-build.yml`,
  per its own release notes "~10MB Tauri vs ~150MB Electron"). Deletion drops
  ~50 Dependabot advisories tied to the bundled Electron + npm transitive tree
  (electron CVEs, xmldom XML injection, lodash, minimatch, picomatch, tar,
  path-to-regexp, brace-expansion, ajv, electron-builder).

## [2.27.0] - 2026-04-25

### Added
- **Backend SSE endpoint** `GET /api/events` (`hotel-backend/src/routes/events.rs`)
  — Phase 4a per `docs/architecture.md` §3.6e. Long-lived Server-Sent Events
  stream of every `DomainEvent` published via `EventBus::publish`. Each request
  opens a dedicated `sqlx::postgres::PgListener`, `LISTEN`s on the
  `domain_events` channel, and forwards each notification to the browser as
  `event: <DomainEvent::type_name()>` / `data: <raw JSON payload>`. 30-second
  `KeepAlive` heartbeat; client disconnect releases the PG connection.
- **Cargo deps** — `async-stream = "0.3"`, `futures-util = "0.3"`.
- **Frontend `useRealtimeEvents` hook** (`lib/use-realtime-events.ts`) — Phase 4a-frontend
  per `docs/architecture.md` §3.6e. Opens a single `EventSource('/api/events')` and
  fans 11 `DomainEvent` variants out to mapped cache buckets via a window
  `CustomEvent('realtime:invalidate')`. Companion `useRealtimeInvalidate(key, refetch)`
  lets list views subscribe in one line.
  - Mapping: `BookingCreated/Modified/Cancelled → ['bookings', 'rooms']`;
    `CheckInCreated/CheckOutCompleted/CheckInCancelled → ['checkins', 'rooms']`;
    `CustomerCreated/Modified → ['customers']`;
    `PaymentReceived → ['payments', 'checkins']`;
    `RoomMarkedClean/Dirty → ['rooms', 'housekeeping']`.
  - `EventSource` auto-reconnects per WHATWG spec; `onerror` only logs.
  - **Window `CustomEvent` fallback** because the app does not currently bundle
    React Query / SWR. `EVENT_TO_QUERY_KEYS` is the migration contract: once a
    cache lib lands, swap the dispatch for `queryClient.invalidateQueries(...)`
    without renaming any listener. TODO marked in the source.
  - Wired into `<AppShell>` — active app-wide.
- **Hook unit tests** (`__tests__/components/useRealtimeEvents.test.tsx`) — 14 tests
  covering EventSource lifecycle, per-variant fan-out, and key filtering.

### Changed
- **Routes thinned (Phase 2.5)** per `docs/architecture.md` §1, §6. Write
  handlers in `routes/new_{customers,bookings,checkins,payments}.rs` now
  delegate to the service layer instead of calling repositories directly.
  Reads (GET/list) keep calling repositories. `EventSource::our_app(Uuid::nil(),
  Uuid::new_v4())` is a temporary placeholder pending auth middleware.
- **Endpoint contracts unchanged.** Frontend `/api/new/*` calls behave
  identically; specific 4xx wording preserved via thin error mappers.

## [2.26.0] - 2026-04-25

### Added
- **Backend service layer** (`hotel-backend/src/service/`) — Phase 2 per
  `docs/architecture.md` §1, §6. One service per aggregate, each opening a
  single PG transaction, performing the canonical write through the
  repository, enqueuing the matching `WritebackIntent` via
  `OutboxRepository::enqueue`, publishing the matching `DomainEvent` via
  `EventBus::publish`, and committing — all four effects atomic.
  - `customer.rs` — `CustomerService { create, update }` + `CreateCustomerCommand` / `UpdateCustomerCommand` / `CustomerOutcome`.
  - `booking.rs` — `BookingService { create, modify, cancel }` + `CreateBookingCommand` / `ModifyBookingCommand` / `CancelBookingCommand` / `BookingOutcome` / `BookingSnapshotInputs` / `BookingWritebackContext` / `BookingRoomCommand`.
  - `checkin.rs` — `CheckInService { walk_in, check_in_to_booking, cancel, extend, check_out }` + `WalkInCommand` / `CheckInToBookingCommand` / `CancelCheckInCommand` / `ExtendStayCommand` / `CheckOutCommand` / `CheckInOutcome` / `CheckInWritebackContext`.
  - `payment.rs` — `PaymentService { record_payment, generate_receipt }` + `RecordPaymentCommand` / `GenerateReceiptCommand`.
  - `housekeeping.rs` — `HousekeepingService { mark_clean, mark_dirty }` + `MarkCleanCommand` / `MarkDirtyCommand`.
  - `error.rs` — `ServiceError` enum (`Validation` / `NotFound` / `Conflict` / `Repository(sqlx::Error)` / `Outbox` / `Internal`) with `From<sqlx::Error>` and a bridge `From<ServiceError> for ApiError`.
  - `ids.rs` — deterministic `i32` ⇄ `Uuid` aggregate-id bridge via `Uuid::new_v5(NAMESPACE_OID + "new-hotel.aggregate.<kind>")`. Lets the `WritebackIntent`/`DomainEvent` `Uuid` contracts coexist with today's SERIAL `i32` PG schema. Forward-compatible: when the schema migrates to native UUID columns the shim disappears without changing event payloads.
- **`AppState` service handles** — `customers_service`, `bookings_service`, `checkins_service`, `payments_service`, `housekeeping_service` (each `Arc<…Service>`). Constructed via `AppState::wire_services` from existing repositories + outbox + event bus + new pool. Routes are NOT yet refactored to delegate (Wave 4 Agent F).

### Changed
- **`hotel-backend/src/lib.rs`** — declares `pub mod service;` so the service layer is reachable from the binary, integration tests, and Wave 4 routes.
## [2.25.0] - 2026-04-25

### Added
- **Backend repository layer** (`hotel-backend/src/repository/`) — Phase 1b per
  `docs/architecture.md` §1, §6. Each aggregate gets a trait + PostgreSQL impl
  (`customer`, `booking`, `checkin`, `room`, `payment`, `inventory`).
- **Backend outbox + event-bus runtime** (`hotel-backend/src/outbox/`) — Phase 3b
  per `docs/architecture.md` §3.6c:
  - `queue.rs` — `OutboxRepository::enqueue()` writes a `writeback_jobs` row
    inside the caller's `&mut Transaction<Postgres>`, atomic with canonical write.
  - `bus.rs` — `EventBus::publish()` performs `INSERT event_log` + `pg_notify('domain_events', ...)` in caller's TX (NOTIFY deferred to COMMIT).
  - `idempotency.rs` — deterministic UUID v5 keys (namespace `d86fe320-5424-58cd-8c00-50ea7d998b36`).
- **`AppState.outbox: Arc<OutboxRepository>`** + **`AppState.events: Arc<EventBus>`** — wired into route state for service-layer callers.
- **`hotel-backend/src/lib.rs`** — exposes modules so integration tests can `use hotel_backend::…`.
- **Integration tests** (`hotel-backend/tests/test_outbox.rs`): 4 sqlx::test cases + 5 pure unit tests in `outbox::idempotency`.

### Changed
- **Routes thinned**: `routes/new_{customers,bookings,checkins,rooms,payments,inventory}.rs` no longer call `sqlx::query!()` directly; SQL text is byte-identical so existing `.sqlx/` cache stays valid.
- **`hotel-backend/Cargo.toml`**: declared `async-trait = "0.1"` explicitly; `uuid` feature `v5` enabled; `sqlx` feature `uuid` enabled.
- **`hotel-backend/src/main.rs`**: switched from inline `mod foo;` to `use hotel_backend::{...}` (single compilation between binary + tests).
- **Endpoint contracts unchanged.** Frontend `/api/new/*` calls behave identically.

## [2.24.0] - 2026-04-25

### Removed
- **Legacy app tree** (`app/(legacy)/*`): legacy dashboard, bookings, calendar, rooms pages and their `BlueNavbar` shell. Superseded by the modern Sidebar-based UI. Per `docs/architecture.md` §8 Phase 0.
- **`components/Navbar.tsx`** (legacy blue navbar with branch picker + Legacy/New mode toggle). Replaced by the renamed `Navbar` (formerly `NewNavbar`).
- **`__tests__/components/LegacyDashboard.test.tsx`**: tested the deleted `app/(legacy)/page.tsx` (10 tests).

### Added
- **Backend domain layer** (`hotel-backend/src/domain/`) — pure types, no I/O, no SQL.
  Per `docs/architecture.md` §1, §2 (Phase 1a). New modules:
  - `customer.rs` — `Customer` struct + `CustomerType` enum + Thai national-ID checksum validation
  - `booking.rs` — `Booking` struct + `BookingState` state machine (Pending / Active / CheckedIn / Completed / Cancelled) with legacy literal mappings
  - `checkin.rs` — `CheckIn` struct + `CheckInState` enum (Active / CheckedOut / Cancelled) with split `Cin_Room_Status` vs `room_status` legacy-literal helpers per spike findings §3e
  - `room.rs` — `Room` struct + `RoomStatus` + `CleanState` enums (with the inverted `Room_Clean='yes'` = "needs cleaning" semantic preserved per spike §3i)
  - `payment.rs` — `Payment` struct + `PaymentMethod` enum (Cash / Credit / Transfer)
  - `shared.rs` — `DateRange`, `Money` (i64 satang), `RoomNumber` primitives
- **Backend outbox enums** (`hotel-backend/src/outbox/`) — type-only contracts (Phase 3a):
  - `event.rs` — `DomainEvent` (11 variants) + `EventSource` + `BookingSnapshot` / `CheckInSnapshot` / `CustomerSnapshot` per `architecture.md` §3.6b
  - `intent.rs` — `WritebackIntent` (one variant per spike-validated recipe §3a–k) with `CreateBookingPayload` / `CreateCheckInPayload` / `BookingChanges`
- **PostgreSQL migrations** (Phase 3a tables, no consumers yet — Wave 2 fills them in):
  - `011_writeback_jobs.sql` — outbox queue (per `architecture.md` §4c)
  - `012_event_log.sql` — durable domain-event bus with 3 indexes (per `architecture.md` §4d-bis)
  - `013_legacy_ct_state.sql` — single-row Change Tracking watermark (per `architecture.md` §4d-tris)
  - Same DDL appended to `init-db/init-hotelnew.sql` for fresh deploys
- **Cargo dependency**: `uuid = "1"` declared explicitly with `["serde", "v4"]` features
  (was previously only available transitively through tiberius).

### Changed
- **App tree collapsed**: `app/new/*` promoted to `app/*` — every former `/new/...` URL is now its canonical `/...` path (e.g. `/new/bookings` → `/bookings`). Internal `<Link>` hrefs and Sidebar entries updated accordingly. **Backend `/api/new/*` routes are unaffected.**
- **`components/NewNavbar.tsx` → `components/Navbar.tsx`**: renamed (history preserved via `git mv`); breadcrumb logic updated to drop the obsolete `/new` prefix; "Legacy" escape link removed.
- **Root layout (`app/layout.tsx`)**: now wraps children in a new `AppShell` client component that renders `<Sidebar>` + `<BranchProvider>` (logic lifted from the deleted `app/new/layout.tsx`). Single root layout for the whole app.
- **Sidebar**: nav entries point at the new flat URLs; ported `card-reader`, `customers`, `changelog` added so all formerly-legacy features remain reachable. Bottom "Legacy" exit link removed (no longer a destination).
- **`hotel-backend/Cargo.toml`** version bumped 2.8.1 → 2.8.2
- **`hotel-backend/src/main.rs`** — registers new `domain` and `outbox` top-level modules

## [2.23.0] - 2026-04-24

### Added
- **Design system**: SAP Fiori Compact UI with oxidized blood-red brand palette (`brand-50` … `brand-800`)
  - 13px base font, 28px row height, dense spacing
  - Sarabun (Thai government typeface) replaces DM Sans — supports Thai + Latin
  - Tailwind tokens for `shell`, `panel`, `headerBar`, `border`, `borderStrong`, `text`, `textMuted`
  - Semantic colours: `success`, `warning`, `error`, `info`
  - All border radii squashed to 2px (rounded-full preserved for circular elements)
- **Inventory backend**: missing mutation endpoints
  - `GET /api/new/inventory/rooms` — room rollup list with inventory count + last-checked
  - `POST /api/new/inventory/rooms/:room_id/check` — record an inventory check
  - `POST /api/new/inventory/rooms/:room_id/replenish` — replenish room stock (deducts from main inventory, logs OUT transactions)
  - `POST /api/new/inventory/adjustments` — generic add/remove/set stock adjustment
- **Backend healthcheck**: `/api/mode` probe + curl in Docker image; web service now waits for backend `service_healthy` before starting

### Fixed
- **Backend NUMERIC casts**: every dynamic-SQL `SELECT` reading DECIMAL columns now `::float8` casts so `try_get::<f64, _>` succeeds instead of silently defaulting (rooms prices, rate values, room-type base price/size, booking totals, inventory cost, report revenue)
- **Backend invoice**: `LEFT JOIN` columns `book_no`/`cust_firstname`/`room_no`/`type_name` are now `COALESCE`'d so walk-in check-ins without a booking/customer no longer fail (`new_invoice.rs`)
- **Backend invoice/inventory**: `.sqlx/` cache regenerated for all modified `query!()` calls
- **Branch filter**: `GET /api/new/{rooms,bookings,customers,checkins,inventory/rooms}` now honour `?branch=hfville` by returning empty results (HotelNew DB only contains HF Hotel data)
- **Room inventory checklist**: backend response shape now matches frontend expectations (`{ success, data: { roomId, roomNumber, roomType, items: [{ assignedQuantity, ... }] } }`)
- **Migrations**: `psql -v ON_ERROR_STOP=1` + `\set ON_ERROR_STOP on` so a failed migration aborts and the `schema_migrations` row is NOT inserted (previously a SQL error was silently ignored and the migration was marked applied)
- **Docker compose**: `web` waits for `backend: service_healthy` (and backend has a healthcheck on `/api/mode`); previously web could start before backend was listening

### Changed
- **Sidebar**: redesigned per Fiori — active item is `bg-brand-50` + 3px left border + `brand-700` text; nav rows reduced to `px-3 py-1.5`; section labels at 10px uppercase; removed `Hotel` logo icon and red "NEW" pill
- **NewNavbar**: thin 40px top bar with breadcrumb + Legacy link
- **DataTable**: `bg-headerBar` 12px header, 32px (h-8) rows with `even:bg-rowAlt` zebra stripes, sort indicators in `text-brand-500`; removed `rounded-lg` wrapper
- **Button / Input / Card / Badge / StatCard**: re-skinned to brand palette and Fiori sizing
  - Button primary: `bg-brand-500` + `border-brand-700`; sizes `h-6/h-7/h-8`
  - Input: 28px tall (`h-7`), `bg-panel`, `border-borderStrong`, `focus:border-brand-500`
  - Card: flat panel, `p-3` default, optional `<CardHeader>` with header-bar styling
  - Badge: flat rectangles with semantic 1px border (no pills)
  - StatCard: 20px value text, 11px uppercase label
- **Dashboard tiles**: removed `bg-{red,yellow,orange,blue}-50` tint backgrounds and `border-b-4` colored borders; now neutral white panels with a 8px coloured status dot in the corner
- **Dashboard modal**: flat panel, no shadow, no rounded corners
- **Page headers**: top pages (`/new`, `/new/bookings`, `/new/rooms`) now use a 40px-tall flat panel header bar with `text-base font-semibold` titles instead of `text-2xl font-bold`
- **app/layout.tsx**: switched to `Sarabun` from `next/font/google`, exposed as `--font-sarabun` CSS variable
- **app/globals.css**: removed body gradient; added brand palette CSS variables; re-skinned react-datepicker to brand palette
- **Backend Dockerfile**: install `curl` for HTTP healthcheck
- **Backend Docker builds**: switched from the dummy-source dependency-cache trick to `cargo-chef` (`planner` + `builder` stages cooking `recipe.json`); fragile-when-Cargo.toml-changes pattern replaced with the standard chef recipe (applied to both `Dockerfile` and `Dockerfile.ville-sync`)
- **Backend Docker builds**: install `mold` + `clang` in the Rust builder stage and added `hotel-backend/.cargo/config.toml` with a target-scoped `linker = "clang"` + `-fuse-ld=mold` rustflag (only applies to `x86_64-unknown-linux-gnu`, so macOS/aarch64 local builds are unaffected); cuts release link time substantially
- **CI test-backend job**: added `mozilla-actions/sccache-action` + `RUSTC_WRAPPER=sccache` (with `SCCACHE_GHA_ENABLED=true`) alongside the existing `Swatinem/rust-cache` step, giving per-rustc-call cache hits on top of the whole-`target/` cache
- **Backend dep graph**: shrunk transitive crate count from 662 → 568 (-94, ~14%) for faster cold Docker builds (`hotel-backend` v2.8.0 → v2.8.1):
  - Replaced `reqwest` with `ureq 2.12` for the Slack webhook client (drops `hyper-tls`, `h2`, `hyper-rustls`, ~90 transitive crates); blocking call dispatched via `tokio::task::spawn_blocking` so the async runtime is never blocked; same 3-attempt retry semantics with exponential backoff
  - Slimmed `tokio` from `["full"]` to explicit minimal feature list `["macros", "rt-multi-thread", "net", "time", "sync"]` based on actual usage audit
  - Dropped the `bigdecimal` feature from `sqlx` — no `BigDecimal` types are read in code; all `DECIMAL`/`NUMERIC` columns are `::float8`-cast to `f64` at the SQL level (per CLAUDE.md guidance)

### Removed
- `Hotel` lucide icon import in Sidebar (now bare wordmark)

## [2.22.1] - 2026-03-04

### Fixed
- **Charts**: Fix missing `yAxisId="right"` YAxis in LineChart causing runtime error; guard empty data domain
- **BookingDetailDrawer**: Fix stale closure in useEffect, add dialog role/aria-modal/Escape handler, replace alert() with inline errors, increase delete button touch target
- **PaymentModal**: Fix useEffect silently resetting user-typed payment amount on re-render
- **XSS**: Escape all server data in inventory transactions print view (document.write)
- **Timezone**: Use UTC methods in formatDateBE, toBuddhistYear, customers page dates per CLAUDE.md convention
- **StayTimeline**: Guard dayData.reduce against empty array crash; add aria-labels to nav buttons
- **StockAdjustmentModal**: Differentiate add/remove/set button colors (green/red/blue)
- **DataTable**: Use unique data ID as React key instead of array index
- **Dashboard**: Accumulate fetch errors instead of overwriting; show error banner
- **Customers page**: Add error banner for fetch failures; fix date-fns timezone issue
- **Bookings page**: Add aria-labels to pagination buttons
- **BookingForm**: Rename shadowing BookingFormData to BookingFormState; fix calculateNights date parsing

### Security
- **CORS**: Restrict thai-id-middleware from `origin: '*'` to localhost app origins
- **Headers**: Add X-Content-Type-Options, X-Frame-Options, Referrer-Policy; disable X-Powered-By
- **Credentials**: Remove hardcoded passwords from docker-compose.yml; require .env file
- **escapeHtml**: Add single-quote escaping for defense-in-depth
- **URL encoding**: Add encodeURIComponent to branch query parameter

### Changed
- BranchContext and ModeContext now wrap children in Provider during initialization (no flash of empty)
- Exclude playwright.config.ts from TypeScript compilation
- Update .env.example with placeholder passwords and POSTGRES container vars
- GuestRegistryModal TM.30 notice uses higher-contrast text colors

## [2.22.0] - 2026-02-21

### Added
- **Push HF Ville data to local cache for faster API reads** — ville_sync now pushes data to production `ville` schema in newdb
  - `ville` schema with 4 cached tables (`ht_rooms_legacy`, `ht_bookings_legacy`, `ht_checkins_legacy`, `ht_customers_legacy`) + `sync_status`
  - ville_sync writes to two targets: local jump box PG (store-and-forward buffer) + production newdb (primary target for API reads)
  - Backend reads from local `ville` schema instead of crossing WireGuard tunnel (<1ms vs ~50ms latency)
  - Push is optional/graceful — if production unreachable, local buffer continues; next cycle reconciles via SHA256 hash comparison
  - Migration `010_ville_cache_schema.sql` creates the ville schema
  - newdb port exposed on WireGuard interface (`<wg-self>:5439`) for ville_sync push access

### Changed
- Backend `ville_pool` now connects to local newdb with `search_path=ville` instead of remote PG via socat
- Removed VILLE_DB_SERVER/PORT/NAME/USER/PASSWORD env vars from backend (uses newdb credentials)
- `hfville-pg-forward` socat service on production can now be stopped (no longer needed)

## [2.21.0] - 2026-02-19

### Added
- **Multi-branch support: HF Ville integration** — second hotel branch (สุราษฎร์ธานี, 34 rooms) integrated into the system
  - Branch selector in Sidebar (new system) and Navbar (legacy system): HF Hotel | HF Ville | ทั้งหมด
  - `BranchContext` + `useBranchFetch` hook — all API calls automatically include `?branch=X`
  - Backend `Branch` enum (`hfhotel`, `hfville`, `all`) with `ville_pool: Option<PgPool>` in AppState
  - Branch parameter added to 7 route handlers: rooms, bookings, checkins, customers, stats, occupancy, calendar
  - HF Ville room layout (2 floors, rooms 101-218) with stacked "All" view showing both hotels
  - `VilleDbConfig` in backend config with `VILLE_DB_ENABLED` env var for graceful degradation
  - SSH tunnel (`hfville-tunnel` systemd service) for remote PG access via cloudflared
  - `ville_sync` binary: syncs HF Ville SQL Server 2005 → PostgreSQL mirror via FreeTDS (SHA256 delta sync, 90s interval)
  - Jump box deployment: `deploy/hfville/docker-compose.yml` with postgres:17-alpine + sync binary
  - HF Ville PG mirror schema: `deploy/hfville/init-db/init-hfville.sql` (rooms, bookings, checkins, customers + sync_status)
  - 8 frontend pages updated with branch-aware fetching (legacy: dashboard, calendar, bookings, rooms, customers; new: dashboard, calendar, bookings)

### Fixed
- HF Ville room queries failing with `relation "ht_rooms_new" does not exist` — added legacy-only query functions for ville pool (which only has `ht_rooms_legacy` tables, not HotelNew tables)
- Room grid not showing "ทำความสะอาด" (cleaning) status — `Room_Clean = "yes"` means room needs cleaning (not "is clean"); fixed in both legacy and new dashboards
- Garbage tsql output artifacts (locale messages, prompt markers) stored as room/customer/checkin rows in HF Ville PG mirror — fixed parser and added cleanup

## [2.20.0] - 2026-02-19

### Changed
- **Calendar revamp** — simplified `StayTimeline` from 3 confusing stacked segments to a clean 2-color model
  - Past dates: single sky-500 bar showing rooms checked-in (occupied) that day
  - Future dates: single amber-400 bar showing rooms booked for that day
  - Today: two bars side-by-side — checked-in (sky) + booked (amber) with red ring highlight
  - Simplified `DayData` interface: `checkedIn`/`booked`/`checkinStays`/`bookingStays` (was 6 fields)
  - Simplified detail panel: `'checkin' | 'booking'` segment types (was 3 types including `'continuing'`)
  - Legend reduced from 3 items to 2: เข้าพัก (sky-500) + การจอง (amber-400)
  - Tooltip now context-aware: shows only relevant data for past/today/future
  - Bars scaled independently against `maxCount` (max of all checkedIn/booked) instead of stacked totals

## [2.19.0] - 2026-02-19

### Changed
- **Light theme for new system** — converted entire `app/new/` from dark zinc theme to light gray theme
  - Foundation: `globals.css` (removed `.new-system-layout` dark overrides, light datepicker/scrollbar styles), `app/new/layout.tsx` (bg-gray-50), `Sidebar.tsx` (white bg, gray borders, red accent)
  - 13 UI primitives: Card, Modal, Drawer, Input, Select, Textarea, Button, Badge, PageHeader, StatCard, Skeleton, EmptyState, PrintButton
  - 10 pages: dashboard, bookings, calendar, room-types, housekeeping, maintenance, inventory, reports, billing, rates
  - 5 additional pages: inventory/items, inventory/rooms, inventory/transactions, billing/[id], admin/sync
  - 22 shared components: forms, modals, pickers, housekeeping, maintenance, inventory, DataTable, BookingDetailDrawer, StayTimeline, RateCalendar
  - Color mapping: zinc-950→gray-50, zinc-900→white, zinc-800→gray-100, text-zinc-100→text-gray-900, etc.
  - Preserved: red-600 accent buttons, status colors (emerald/amber/sky/orange), print templates (light)

## [2.18.0] - 2026-02-18

### Added
- **Unified architecture foundation** — new shared utilities, types, UI primitives, sidebar navigation, and unified layout for the single-system redesign
  - `lib/format.ts` — consolidated formatting utilities (`formatCurrency`, `toBuddhistYear`, `formatBuddhistDate`, `formatDateForApi`, `formatThaiDate`, `calculateNights`) from 8+ duplicate implementations
  - `lib/status.ts` — centralized status color/label maps for bookings, rooms, housekeeping, maintenance, payments with `getStatusColor()`/`getStatusLabel()` helpers
  - `types/common.ts`, `types/booking.ts`, `types/customer.ts`, `types/room.ts`, `types/checkin.ts` — shared TypeScript type definitions extracted from scattered page-level types
  - 12 UI primitives in `components/ui/`: Badge, Button, Card, Modal, Drawer, Input, Select, Textarea, PageHeader, StatCard, Skeleton, EmptyState
  - `components/Sidebar.tsx` — collapsible left sidebar navigation (240px/64px) with localStorage persistence, responsive defaults, and smooth transitions
  - `app/(unified)/layout.tsx` — unified layout using Sidebar with synchronized collapse state

- **PostgreSQL paths for remaining SQL Server endpoints** (Phase A — SQL Server independence)
  - `GET /api/rooms/status` — new `get_room_status_pg()` using `generate_series()` + joins on `ht_rooms_legacy`, `ht_checkins_legacy`, `ht_bookings_legacy` to replicate `View_Room_status` behavior
  - `GET /api/bookings/:id` — new `get_booking_pg()` querying `ht_bookings_legacy` with `book_total` support, dispatched via `use_pg_source()` feature flag
  - `GET /api/calendar` — new `fetch_legacy_calendar_data_pg()` querying PG mirror tables for bookings and check-ins, dispatched via `use_pg_source()` feature flag

## [2.17.1] - 2026-02-18

### Fixed
- **Sync: invalid column names** — removed `Book_Room_No` from booking sync and `Cin_CheckIn_No` from check-in sync (columns don't exist in SQL Server views)
- **Sync: customer truncation** — widened `cust_no`, `cust_type`, `cust_phone`, `cust_idcard` columns in `ht_customers_legacy` to prevent "value too long" errors
- **Sync: datetime panics** — use `try_get()` for all datetime fields in booking/check-in sync to handle empty/invalid values gracefully

## [2.17.0] - 2026-02-18

### Added
- **One-time legacy data migration CLI** (`cargo run --bin migrate_legacy`) — imports all historical data from SQL Server into PostgreSQL in a single transaction
  - Extracts distinct room types from legacy rooms into `ht_room_types`
  - Imports rooms (`HT_Rooms` -> `ht_rooms_new`) with floor parsing and type linking
  - Imports customers (`View_Customers` -> `ht_customers`) with name splitting (first/last)
  - Imports bookings (`View_Booking_Ds` -> `ht_bookings` + `ht_booking_rooms`) grouped by Book_No
  - Imports check-ins (`View_CheckIn_Ds` -> `ht_checkins`) with customer/room linking
  - Bumps PostgreSQL sequences past max imported IDs to avoid conflicts
  - All imported records tagged with `source = 'legacy'`
- **Status code mapping**: Legacy `Book_Status` integers mapped to string statuses (1=confirmed, 2=checkedin, 3=completed, 4=cancelled, 0/other=pending)
- **Safety features**: Full transaction rollback on error, `--dry-run` flag, idempotent (skips existing records)

## [2.16.0] - 2026-02-18

### Added
- **Legacy-to-PostgreSQL background sync** — new scheduler job replicates data from SQL Server every 5 minutes using SHA256 change detection, enabling gradual migration away from the legacy database
  - `ht_rooms_legacy` mirrors `HT_Rooms`
  - `ht_bookings_legacy` mirrors `View_Booking_Ds` (composite key: book_no + room_type)
  - `ht_checkins_legacy` mirrors `View_CheckIn_Ds` (unique key: cin_no)
  - `ht_customers_legacy` mirrors `View_Customers` (unique key: cust_no)
  - `sync_status` table tracks per-entity sync health, timing, and error counts
- **Sync status API** — `GET /api/new/sync/status` returns last sync time, record counts, and health indicator per entity type
- **Sync admin dashboard** — `app/new/admin/sync/page.tsx` displays real-time sync health with auto-refresh every 30 seconds
- **Data source tracking** — `source` column added to `ht_bookings`, `ht_checkins`, `ht_customers` to distinguish between 'new' (app-created) and 'legacy' (synced) records
- **SYNC_ENABLED environment variable** — set to `false` to disable the background sync job without code changes

- **`LEGACY_READ_SOURCE` feature flag** — all legacy read routes now default to PostgreSQL mirror tables; set `LEGACY_READ_SOURCE=sqlserver` to fall back to direct SQL Server queries
  - `GET /api/rooms` — reads from `ht_rooms_legacy` (with `ht_rooms_new` price overrides)
  - `GET /api/rooms/:id` — room detail + current guest from `ht_checkins_legacy`
  - `GET /api/rooms/checkouts-today` — checkout detection from PG
  - `GET /api/bookings` — paginated bookings from `ht_bookings_legacy`
  - `GET /api/checkins` — paginated check-ins from `ht_checkins_legacy`
  - `GET /api/customers` — search/sort/pagination from `ht_customers_legacy`
  - `GET /api/customers/:id/bookings` — booking history from `ht_bookings_legacy`
  - `GET /api/customers/:id/stats` — customer stats from PG mirror tables
  - `GET /api/stats` — dashboard statistics from PG mirror tables
  - `GET /api/occupancy` — occupancy trends from `ht_checkins_legacy`
  - Exceptions: `GET /api/rooms/status` and `GET /api/bookings/:id` still use SQL Server (no PG equivalent)

### Changed
- Scheduler `init_scheduler()` now accepts an optional `PgPool` for the sync job (backwards compatible — Slack notification jobs unchanged)
- All legacy read routes refactored with dual-source pattern: PG implementation + SQL Server fallback per endpoint

## [2.15.2] - 2026-02-07

### Fixed
- Fixed all legacy endpoint datetimes showing 7 hours behind actual time — `NaiveDateTime` from SQL Server now converted to `DateTime<Utc>` with `Z` suffix so frontend `timeZone: 'UTC'` displays stored Thai time correctly
  - Affected: checkins, bookings, rooms (detail + status), customers (list, booking history, stats), calendar (legacy + new sources)
  - 19 datetime fields across 10 structs updated

## [2.15.1] - 2026-02-07

### Changed
- Renamed CI/CD pipeline jobs for consistency: `test` → `test-frontend`, `build-and-push` → `build-frontend`
- Simplified `build-backend` condition — removed unnecessary `always()` and `skipped` check
- Simplified `deploy` dependencies — removed redundant `test-backend` from `needs`

## [2.15.0] - 2026-02-07

### Added
- **Compile-time SQL verification with `sqlx::query!()` macros** — ~76 static SQL queries now verified at compile time against the PostgreSQL schema, catching column name typos, type mismatches, and schema drift before runtime
  - Dynamic queries (~30) that build SQL with string concatenation remain as `sqlx::query()` runtime queries
  - `DECIMAL` columns use `::float8` casts for ergonomic `f64` return types
  - Added `bigdecimal` feature to sqlx for `NUMERIC` parameter binding
- **`.sqlx/` offline compilation cache** — 76 query cache files enable compilation without a live database connection
  - `SQLX_OFFLINE=true` environment variable enables offline mode in Docker builds and CI
  - `scripts/sqlx-prepare.sh` helper script to regenerate the cache after query changes
- **Backend integration tests** — New `hotel-backend/tests/` directory with database integration tests
  - `test_schema.rs` — Validates all 18 expected tables exist and `schema_migrations` has baseline row
  - `test_customers.rs` — Customer CRUD lifecycle and search tests
  - `test_rooms.rs` — Room CRUD, status updates, and room type association tests
  - `test_bookings.rs` — Booking creation with room assignments and cancellation
  - `test_payments.rs` — Payment recording and void (soft delete) tests
  - `test_stats.rs` — Dashboard statistics query validation
  - Shared test infrastructure (`tests/common/mod.rs`) with `TEST_` prefix cleanup
- **CI/CD backend test job** — `test-backend` job runs integration tests against PostgreSQL 17 service before Docker build

### Changed
- Backend sqlx features updated: added `macros` and `bigdecimal` (was runtime queries only)
- Dockerfile updated with `SQLX_OFFLINE=true` and `.sqlx/` directory copy for offline compilation
- Backend version bumped to 2.7.0

### Fixed
- **Latent SQL bugs caught by compile-time verification**:
  - `new_bookings.rs` — `br_total_price` column reference that doesn't exist in `ht_booking_rooms`
  - Various type mismatches between `Option<T>` struct fields and NOT NULL database columns

## [2.14.0] - 2026-02-07

### Added
- **Automated PostgreSQL migration system** — Schema changes are now automatically applied during CI/CD deployment
  - `scripts/migrate.sh` — Migration runner that applies pending `migrations/pg/*.sql` files
  - `scripts/backup-db.sh` — Manual database backup utility
  - `schema_migrations` table tracks applied migrations with version, filename, checksum, and timestamp
  - Pre-migration `pg_dump` backups created automatically (keeps last 10)
  - Each migration runs in a transaction — rolls back on failure
  - Backup pruning to prevent disk bloat
  - `migrations/pg/000_baseline.sql` — Baseline marker for initial schema
- **CI/CD pipeline integration** — Deploy job now copies migration files, runs `migrate.sh` after DB health check, and restarts backend
- **Path filter expansion** — Pipeline triggers on changes to `migrations/pg/**` and `scripts/migrate.sh`

## [2.13.1] - 2026-02-07

### Fixed
- **Backend crash-loop due to PostgreSQL port mismatch** - PostgreSQL container was listening on default port 5432 while backend expected port 5439 (`NEW_DB_PORT=5439`). Added `PGPORT=5439` environment variable to `newdb` service and updated healthcheck to use `-p 5439`. This caused ALL APIs (legacy + new) to fail since the backend couldn't start.
- **StatsCard dark theme on legacy page** - Reverted `StatsCard.tsx` from dark theme colors (`bg-zinc-900`, `text-zinc-100`) back to light theme (`bg-white`, `text-gray-900`). The component is only used by the legacy light-themed dashboard and was accidentally changed during v2.12.0 dark theme redesign.

### Added
- **StatsCard regression test** (`__tests__/components/StatsCard.test.tsx`) - 8 tests verifying rendering, light theme colors, and subtitle behavior
- **Legacy Dashboard regression test** (`__tests__/components/LegacyDashboard.test.tsx`) - 9 tests covering loading state, stats cards, room grid, occupancy chart, recent activity, empty states, and API error handling
- **Playwright E2E test setup** - End-to-end testing framework for the legacy dashboard
  - `playwright.config.ts` - Chromium-only config targeting localhost:3003
  - `e2e/legacy-dashboard.spec.ts` - 4 E2E tests: page load, stats cards, room grid, navigation
  - `test:e2e` script in package.json

## [2.13.0] - 2026-02-07

### Changed
- **Migrate HotelNew database from SQL Server to PostgreSQL** - Major infrastructure change
  - Replaced SQL Server 2022 container (~2GB RAM, 1.6GB image) with PostgreSQL 17 Alpine (~50-100MB RAM, ~100MB image)
  - Backend now uses `sqlx` crate for PostgreSQL queries (replacing tiberius/bb8 for HotelNew DB)
  - Legacy database (<legacy-mssql-host>) unchanged - still uses tiberius/bb8 for read-only access
  - Converted all 14 route files from T-SQL to PostgreSQL syntax
  - Converted DDL init script (`init-db/init-hotelnew.sql`) to PostgreSQL
  - Stored procedures replaced with PL/pgSQL functions
  - Updated `docker-compose.yml` for PostgreSQL service
  - PostgreSQL auto-initializes from `/docker-entrypoint-initdb.d/` (no manual init needed)
- **Updated CI/CD pipeline for PostgreSQL** - Removed `sqlcmd` database initialization step (PostgreSQL auto-initializes)
- **Updated documentation for PostgreSQL migration** - `.env.example`, `hotel-backend/README.md`, `migrations/README.md`
- **Bumped Rust Docker image from 1.83 to 1.85** - Required by `base64ct` crate needing Rust edition 2024

## [2.12.0] - 2026-02-07

### Changed
- **New system dark theme redesign** - Complete visual overhaul of the new system (`/new/*`) with a professional dark black/red color scheme
  - Dark backgrounds (zinc-950/900/800) replacing white/gray
  - Red accent color (red-600/500/400) replacing emerald/blue/purple
  - DM Sans font replacing Inter for a more professional look
  - Updated all pages: dashboard, bookings, calendar, room-types, rates, housekeeping, maintenance, reports, inventory, billing
  - Updated all shared components: DataTable, StatsCard, NewNavbar, ModeToggle
  - Updated all form components: BookingForm, RoomTypeForm, RateForm, CustomerForm, InventoryItemForm
  - Updated all modal components: QuickCheckInModal, CheckOutModal, GuestRegistryModal, StockAdjustmentModal, PaymentModal, MaintenanceRequestModal
  - Updated picker/misc components: CustomerPicker, RoomPicker, MaintenanceCard, RoomInventoryChecklist, RateCalendar, StayTimeline, BookingDetailDrawer, PrintButton
  - Dark scrollbar and date picker CSS overrides via `.new-system-layout` class
  - Legacy system appearance unchanged (only "New System" switch button color updated)

## [2.11.2] - 2026-02-06

### Fixed
- **Legacy main page** - Removed check-in/check-out functionality (legacy is read-only)

### Changed
- **CI/CD workflow** - Added cancel-previous job to handle stuck runs

## [2.11.1] - 2026-02-06

### Fixed
- **Legacy customers page not displaying customers** - Removed ModeContext dependency that caused page to use wrong API endpoint when localStorage had 'new' mode saved from previous session

### Added
- **Calendar page for new system** (`/app/new/calendar/page.tsx`) - Moved calendar functionality to new system
  - Uses hybrid calendar endpoint to show both legacy and new bookings/check-ins
  - Added calendar link to NewNavbar navigation

### Changed
- **Separated legacy and new mode dependencies** - Legacy pages now always use legacy APIs, new pages always use new APIs (no more mode context interference)

## [2.11.0] - 2026-02-06

### Added
- **Reports Dashboard (Phase 1)** - Analytics and reporting at `/new/reports`
  - **Reports Page** (`/app/new/reports/page.tsx`) - Revenue and occupancy analytics
    - Date range picker with preset options (last 7, 14, 30 days, last month)
    - Period grouping selector (Day/Week/Month)
    - Stats cards: Total Revenue, Occupancy Rate, ADR, RevPAR, Avg Stay Length
    - Revenue trend chart (bar/line toggle)
    - Room type revenue pie chart breakdown
    - Thai language labels throughout
  - **Chart Components** (`/components/Charts.tsx`)
    - `RevenueChart` - Bar/line chart for revenue trends using Recharts
    - `PieChart` - Room type revenue breakdown with color-coded segments
    - Formatted tooltips showing revenue in Thai Baht and booking counts
  - **Report Types** (`/types/reports.ts`)
    - `RevenueDataPoint`, `RevenueResponse` - Revenue report data structures
    - `OccupancyResponse` - Occupancy metrics with ADR/RevPAR
    - `RoomTypeRevenue`, `RevenueByRoomTypeResponse` - Room type breakdown
    - `MaintenanceRequest`, `MaintenanceCategory` - Maintenance types
    - `Payment`, `PaymentsResponse` - Payment tracking types
- **Maintenance Request System (Phase 3)** - Kanban-style maintenance tracking at `/new/maintenance`
  - **Maintenance Page** (`/app/new/maintenance/page.tsx`) - Main maintenance dashboard
    - Three-column Kanban board: "Open" (red), "In Progress" (yellow), "Completed" (green)
    - Request cards showing title, room, category, priority badge, and time elapsed
    - Priority indicators with color coding (High=red, Medium=yellow, Low=gray)
    - Overdue badge for requests waiting > 2 hours
    - Filters: room, category, priority
    - Add request button opens modal form
    - Auto-refresh every 30 seconds
    - Thai language labels throughout
  - **MaintenanceCard Component** (`/components/maintenance/MaintenanceCard.tsx`)
    - Displays request number, title, room, category, priority
    - Time elapsed since created or since started
    - Assigned technician display
    - Quick action buttons: "Start Repair" (open -> in_progress), "Done" (in_progress -> completed)
    - Edit button to modify request details
    - Resolution and cost display for completed requests
  - **MaintenanceRequestModal Component** (`/components/modals/MaintenanceRequestModal.tsx`)
    - Create mode: room picker, category dropdown, title, description, priority, assignedTo
    - Edit mode: adds resolution, cost fields
    - Priority selection with color-coded buttons
    - Validation for required fields
  - **Backend API** (`/api/new/maintenance/*`)
    - `GET /api/new/maintenance/categories` - List maintenance categories
    - `GET /api/new/maintenance/requests` - List requests with filters (status, room, category, priority)
    - `POST /api/new/maintenance/requests` - Create request (generates MR-YYMM-NNNN format)
    - `GET /api/new/maintenance/requests/:id` - Get single request
    - `PUT /api/new/maintenance/requests/:id` - Update request
    - `PUT /api/new/maintenance/requests/:id/status` - Quick status update
  - **Database Migration** (`migrations/007_maintenance_system.sql`)
    - `HT_Maintenance_Categories` table with default categories (Electrical, Plumbing, AC, Furniture, General)
    - `HT_Maintenance_Requests` table with status, priority, cost, resolution tracking
    - `SQ_Maintenance_No` sequence for request number generation
- **Thai Labels**:
  - "แจ้งซ่อม" (Maintenance Request)
  - "รอดำเนินการ" (Pending) / "กำลังดำเนินการ" (In Progress) / "เสร็จสิ้น" (Completed)
  - "ความเร่งด่วน" (Priority): "สูง" (High), "ปานกลาง" (Medium), "ต่ำ" (Low)
  - "เริ่มซ่อม" (Start Repair) / "ซ่อมเสร็จ" (Done)
  - "ผลการซ่อม" (Resolution) / "ค่าใช้จ่าย" (Cost)
- **Test Coverage**
  - `PaymentModal.test.tsx` - Payment modal component tests (form inputs, API submission, validation)
  - `MaintenanceRequestModal.test.tsx` - Maintenance modal component tests (create/edit modes)
  - `MaintenanceCard.test.tsx` - Maintenance card component tests (status changes, priority display)
  - `Charts.test.tsx` - Chart components tests (OccupancyChart, RevenueChart, PieChart)

## [2.10.0] - 2026-02-06

### Added
- **Billing Module** - Invoice viewing and printing functionality at `/new/billing`
  - **Billing List Page** (`/app/new/billing/page.tsx`) - Check-in list with invoice actions
    - Search by guest name or check-in number
    - Filter by status (all, active, checked out)
    - Date range filter for check-in/check-out dates
    - Table showing: Check-in number, Room, Guest Name, Check-in Date, Checkout Date, Total Amount, Status
    - "View Invoice" button linking to invoice detail page
    - Pagination with page navigation
    - Thai language labels throughout
  - **Invoice Detail Page** (`/app/new/billing/[id]/page.tsx`) - Individual invoice view with print
    - Fetches invoice data from `/api/new/checkins/:id/invoice`
    - Displays InvoiceTemplate component with hotel and guest information
    - Print button with PDF save option
    - Back button to return to billing list
    - Loading and error state handling
    - Hotel info: The HF Hotel with Thai address and tax ID
- **Payment Tracking System (Phase 2)** - Multiple payments per check-in support
  - **Database Schema** (`migrations/006_payment_tracking.sql`)
    - `HT_Payments` table for tracking multiple payments per check-in
    - Supports payment methods: cash, credit, transfer, QR code
    - Soft delete (void) capability for payment corrections
    - Reference field for card/transfer numbers
    - Automatic balance calculation (total - paid)
  - **Backend API** (`/api/new/checkins/:id/payments`)
    - `GET /api/new/checkins/:id/payments` - List payments with balance summary
    - `POST /api/new/checkins/:id/payments` - Record a new payment
    - `DELETE /api/new/payments/:id` - Void a payment (soft delete)
  - **PaymentModal Component** (`/components/modals/PaymentModal.tsx`)
    - Amount input with auto-fill remaining balance option
    - Payment method selection buttons (Cash, Credit Card, Transfer, QR)
    - Optional reference field for card/transfer numbers
    - Notes field for additional information
    - Balance summary display (total, paid, remaining)
    - Thai language labels throughout

## [2.9.0] - 2026-02-06

### Fixed
- **Legacy Database Read-Only Enforcement** - Fixed booking notes writing to legacy database
  - Moved `HT_Booking_Notes` table from legacy database to HotelNew database
  - Updated booking routes to use dual-pool architecture:
    - `GET /api/bookings/:id` - Uses legacy DB for booking data, HotelNew for notes
    - `GET /api/bookings/:id/notes` - Uses HotelNew DB (read)
    - `POST /api/bookings/:id/notes` - Uses HotelNew DB (write)
    - `DELETE /api/bookings/:id/notes` - Uses HotelNew DB (write)
  - Legacy database (<legacy-mssql-host>) is now truly read-only

### Changed
- **Backend Architecture** - `bookings.rs` now uses `AppState` instead of `DbPool` for booking detail and notes routes
- **Route Configuration** - Booking notes routes moved from `legacy_routes` to `new_routes` in main.rs
- **Optional Legacy Database** - The app can now run without a legacy database connection
  - When `SYSTEM_MODE=new`, the app starts even if legacy database is unavailable
  - Legacy routes (`/api/rooms`, `/api/bookings`, `/api/checkins`, etc.) return 404 when legacy DB is unavailable
  - HotelNew database is required in New mode
  - Scheduler (checkout reminders) only runs when legacy database is available

### Added
- Migration `005_move_booking_notes_to_hotelnew.sql` - Creates HT_Booking_Notes table in HotelNew database
- `HT_Booking_Notes` table definition added to `init-db/init-hotelnew.sql`
- `create_new_pool` function exported from db module for standalone HotelNew connections

## [2.8.0] - 2026-02-05

### Added
- **Self-Hosted Database for HotelNew** - Dedicated SQL Server Docker container for the new hotel management system
  - **Docker Infrastructure**
    - New `newdb` service in `docker-compose.yml` running `mcr.microsoft.com/mssql/server:2022-latest`
    - SQL Server Express edition (free, suitable for hotel workloads)
    - Data persistence via Docker volume `newdb_data`
    - Health check with automatic service dependency management
    - Internal Docker network (`hotel-network`) - database not exposed to host
  - **Database Initialization**
    - `init-db/init-hotelnew.sql` - Complete database bootstrap script
    - Combines all migrations (002, 003, 004) into single idempotent script
    - Creates HotelNew database with all tables, indexes, sequences, and stored procedures
  - **Environment Configuration**
    - `NEW_DB_SERVER=newdb` - Backend connects to Docker container via service name
    - `NEW_DB_PASSWORD=<see GH secret>` - Strong password for SA account
    - `SYSTEM_MODE=new` - Default to New Mode for fresh deployments
  - **Documentation**
    - Updated `CLAUDE.md` with dual-database architecture diagram
    - Updated `.env.example` with new database configuration pattern
    - First-time setup instructions for database initialization
  - **CI/CD Pipeline Updates** (`.github/workflows/docker-build.yml`)
    - Pipeline now copies `init-db/` folder to production server
    - Automatic database health check with 2-minute timeout
    - Automatic database initialization after container is healthy
    - Idempotent deployment - safe to run from scratch or on existing setup
    - Detailed logging for deployment troubleshooting

### Changed
- **Backend database connection** - `NEW_DB_SERVER` now points to Docker container (`newdb`) instead of external server (<legacy-mssql-host>)
- **System mode default** - Changed from `legacy` to `new` in docker-compose.yml for new deployments

### Security
- SQL Server container only accessible within Docker network (not exposed to host)
- Strong SA password enforced (was using a weak literal for legacy connection)

## [2.7.1] - 2026-02-05

### Added
- **Comprehensive Test Suite** - 509 new tests for New Mode components (555 total)
  - **Test Utilities** (`/__tests__/utils/`)
    - `mockFactories.ts` - Mock data factories for customers, rooms, bookings, check-ins, inventory, invoices
    - `commonMocks.ts` - Lucide icon mocks, fetch mocks, date mocks, browser mocks
    - `testUtils.tsx` - Custom render functions, Thai language assertions
    - `asyncUtils.ts` - Async testing helpers for loading, modals, debounce
  - **Tier 1 Critical Tests**
    - `QuickCheckInModal.test.tsx` - 29 tests for walk-in check-in
    - `CheckOutModal.test.tsx` - 39 tests for checkout process
    - `InvoiceTemplate.test.tsx` - 49 tests for invoice rendering
  - **Picker Component Tests**
    - `CustomerPicker.test.tsx` - 43 tests including keyboard navigation
    - `RoomPicker.test.tsx` - 43 tests for multi-select room selection
  - **Form Component Tests**
    - `CustomerForm.test.tsx` - 35 tests for customer CRUD
    - `RoomTypeForm.test.tsx` - 34 tests for room type configuration
    - `InventoryItemForm.test.tsx` - 37 tests for inventory items
    - `RateForm.test.tsx` - 46 tests for special rates
  - **Operations Component Tests**
    - `HousekeepingStats.test.tsx` - 13 tests for stats display
    - `RoomCleaningCard.test.tsx` - 33 tests for cleaning workflow
    - `StockAdjustmentModal.test.tsx` - 36 tests for stock management
    - `RoomInventoryChecklist.test.tsx` - 34 tests for room inventory

## [2.7.0] - 2026-02-05

### Added
- **Inventory Management System** - Phase 4 inventory tracking module at `/new/inventory`
  - **Inventory Dashboard** (`/app/new/inventory/page.tsx`) - Main inventory overview
    - Summary cards: Total items count, Low stock alerts count, Categories count
    - Quick action buttons: Add Item, Stock Adjustment, View Transactions
    - Low stock alerts section showing items below minimum threshold
    - Recent transactions list with type indicators (IN/OUT/ADJUST/MOVE)
    - Click-through navigation to detailed pages
  - **Item Management** (`/app/new/inventory/items/page.tsx`) - Full CRUD for inventory items
    - Table view with columns: Code, Name, Category, Unit, Min Stock, Current Stock, Status, Actions
    - Search by item name or code
    - Filter by category (Minibar, Amenities, Linens, Equipment)
    - Low stock filter toggle
    - Sortable columns (Code, Name, Stock level)
    - Stock level indicators with color coding (green=good, yellow=low, orange=critical, red=out)
    - Inline stock adjustment and edit actions
  - **Room Inventory** (`/app/new/inventory/rooms/page.tsx`) - Per-room inventory view
    - Grid of room cards showing assigned inventory items
    - Room status indicators (checked today, has missing items, not checked)
    - Click room to open checklist modal
    - Filter by status (all, missing items, checked today)
    - Search by room number
    - Legend explaining status colors
  - **Transaction History** (`/app/new/inventory/transactions/page.tsx`) - Audit trail
    - Full transaction log with Date, Type, Item, Quantity, Room, Notes, By columns
    - Filter by transaction type (IN, OUT, ADJUST, MOVE)
    - Date range filter (from/to)
    - Search by item name/code
    - Print view functionality for reports
    - Stock change display (previous -> new)

- **Inventory Components**
  - `InventoryItemForm` (`/components/forms/InventoryItemForm.tsx`) - Modal for add/edit items
    - Fields: Item Code, Name, Category, Unit, Min Stock, Current Stock, Cost per Unit
    - Category dropdown with Thai labels
    - Unit dropdown with common units (pieces, bottles, boxes, sets, etc.)
    - Validation: unique code, non-negative stock values
    - Delete functionality with confirmation
  - `StockAdjustmentModal` (`/components/modals/StockAdjustmentModal.tsx`) - Quick stock changes
    - Item search with autocomplete
    - Three adjustment types: Add stock, Remove stock, Set stock (absolute)
    - Real-time preview of new stock level
    - Notes field for audit trail
    - Color-coded adjustment type buttons
  - `RoomInventoryChecklist` (`/components/inventory/RoomInventoryChecklist.tsx`) - Room verification
    - Checklist of items assigned to room
    - Checkbox and quantity input for each item
    - Items grouped by category
    - Missing items highlighted in orange
    - "Replenish" button to auto-create transactions for missing items
    - Notes field for housekeeper comments

- **Type Definitions** (`/types/inventory.ts`)
  - `InventoryItem` - Item data structure
  - `InventoryTransaction` - Transaction record structure
  - `RoomInventory` - Room inventory assignment
  - `InventoryCategory` - Enum: Minibar, Amenities, Linens, Equipment
  - `TransactionType` - Enum: IN, OUT, ADJUST, MOVE
  - Stock status helpers: `getStockStatus()`, `getStockStatusColor()`, `getStockStatusLabel()`

- **Thai Labels**:
  - "สินค้าคงคลัง" (Inventory)
  - "หมวดหมู่" (Category)
  - "จำนวนคงเหลือ" (Current Stock)
  - "ขั้นต่ำ" (Minimum)
  - "ปรับสต็อก" (Adjust Stock)
  - "รับเข้า" (Stock In)
  - "เบิกออก" (Stock Out)
  - "โอนย้าย" (Transfer)
  - "ปกติ/ใกล้หมด/วิกฤต/หมด" (Good/Low/Critical/Out stock status)

- **Categories**:
  - "Minibar" - เครื่องดื่ม/ของว่าง
  - "Amenities" - อุปกรณ์อำนวยความสะดวก
  - "Linens" - ผ้าและเครื่องนอน
  - "Equipment" - อุปกรณ์ในห้อง

- **Inventory Backend APIs** (Rust/Axum)
  - `GET/POST /api/new/inventory/categories` - Category management
  - `GET/POST /api/new/inventory/items` - Item CRUD with filters (category, low_stock, search)
  - `GET/PUT/DELETE /api/new/inventory/items/:id` - Item management
  - `GET/PUT /api/new/inventory/rooms/:room_id` - Room inventory assignment
  - `GET/POST /api/new/inventory/transactions` - Transaction log with stock updates
  - `GET /api/new/inventory/stats` - Dashboard statistics
  - `GET /api/new/inventory/low-stock` - Low stock alert items

- **Database Migration** (`migrations/004_create_inventory_tables.sql`)
  - `HT_Inventory_Categories` - Category definitions
  - `HT_Inventory_Items` - Item master with stock tracking
  - `HT_Room_Inventory` - Room-item assignments
  - `HT_Inventory_Transactions` - Stock movement audit log

## [2.6.0] - 2026-02-05

### Added
- **Housekeeping Module** - Kanban-style housekeeping board for room cleaning management at `/new/housekeeping`
  - **Housekeeping Page** (`/app/new/housekeeping/page.tsx`) - Main housekeeping dashboard
    - Three-column Kanban board: "Dirty" (red), "Cleaning" (yellow), "Ready" (green)
    - Room cards display room number, type, floor, and time in current status
    - Priority indicator for rooms that have been dirty > 2 hours
    - Floor filter dropdown to focus on specific floors
    - Auto-refresh every 30 seconds for real-time updates
    - Thai language labels throughout
  - **HousekeepingStats Component** (`/components/housekeeping/HousekeepingStats.tsx`) - Summary statistics
    - Total rooms needing cleaning count
    - Rooms currently being cleaned count
    - Rooms cleaned today count
    - Average cleaning time display (when available)
    - Color-coded stat cards matching Kanban columns
  - **RoomCleaningCard Component** (`/components/housekeeping/RoomCleaningCard.tsx`) - Individual room cards
    - Large room number display with room type and floor
    - Priority badge for urgent rooms (> 2 hours since checkout)
    - Time tracking: checkout time, time in current status
    - Housekeeper assignment display (when available)
    - Expandable notes field for housekeeper comments
    - Action buttons: "Start Cleaning", "Done", "Mark as Dirty"
    - Visual status indicators with color coding
- **Thai Labels**:
  - "Dirty Room" - "Waiting for Cleaning"
  - "Cleaning" - "In Progress"
  - "Ready" - "Clean Room"
  - "Start Cleaning" / "Done" action buttons

## [2.5.0] - 2026-02-05

### Added
- **Phase 3 Financial Backend APIs** - Rust/Axum backend endpoints for rate management and financial reports
  - **Rate Management API** (`/api/new/rates`) - Full CRUD for room rates
    - `GET /api/new/rates` - List all rates with optional `room_type_id` and `active` filters
    - `POST /api/new/rates` - Create a new rate (multiplier or fixed type)
    - `GET /api/new/rates/:id` - Get single rate details
    - `PUT /api/new/rates/:id` - Update rate configuration
    - `DELETE /api/new/rates/:id` - Delete a rate
    - Supports rate fields: name, room type, rate type (multiplier/fixed), value, valid date range, days of week, active status
  - **Financial Reports API** (`/api/new/reports`) - Revenue and occupancy analytics
    - `GET /api/new/reports/revenue?from=&to=&group_by=day|week|month` - Revenue report with period grouping
      - Returns: `{ data: [{ period, revenue, bookings }] }`
      - Revenue calculated from completed check-ins (rate per night x nights stayed)
    - `GET /api/new/reports/occupancy?from=&to=` - Occupancy statistics
      - Returns: occupancy_rate, total_rooms, occupied_nights, available_nights, ADR, RevPAR, avg_stay_length
      - Occupancy = (Occupied room-nights / Total available room-nights) x 100
    - `GET /api/new/reports/revenue-by-room-type?from=&to=` - Revenue breakdown by room type
      - Returns: `[{ room_type, revenue, percentage }]`
  - **Invoice Data API** (`/api/new/checkins/:id/invoice`) - Complete invoice data retrieval
    - Returns guest details, room assignment, rate calculations, totals
    - Includes all data needed for invoice/receipt generation

- **Database Migration** (`migrations/003_alter_ht_rates_table.sql`)
  - Alters HT_Rates table to support multiplier/fixed rate types
  - Adds Rate_Type (varchar), Rate_Value (decimal) columns
  - Renames date columns for API consistency
  - Adds Rate_Updated timestamp column

## [2.4.0] - 2026-02-05

### Added
- **Invoice and Receipt Generation** - Phase 3 financial features for hotel billing
  - `InvoiceTemplate` component (`/components/documents/InvoiceTemplate.tsx`) - Printable invoice layout
    - Hotel information header with logo, name, address, tax ID
    - Guest details section (name, ID card, contact)
    - Room charges table (room number, type, dates, nights, rate, subtotal)
    - Summary section: subtotal, discount, VAT (optional), grand total
    - Thai Buddhist Era dates (Gregorian + 543)
    - Thai/English bilingual labels
    - Print-optimized CSS with @media print rules for A4 paper
  - `ReceiptTemplate` component (`/components/documents/ReceiptTemplate.tsx`) - Payment confirmation document
    - Similar layout to invoice with payment details
    - Payment method and amount display
    - Receipt number field
    - "Paid in Full" indicator (ชำระเงินครบถ้วน / PAID IN FULL)
    - Signature lines for cashier and guest
  - `PrintButton` component (`/components/ui/PrintButton.tsx`) - Print/PDF action button
    - Triggers window.print() for browser printing
    - Dropdown option for "Save as PDF" via browser print dialog
    - Thai labels: "พิมพ์" (Print), "บันทึก PDF" (Save PDF)
    - Size variants (sm, md, lg)
    - Loading state during print operation
  - Type definitions (`/types/invoice.ts`)
    - `InvoiceData` - Invoice data structure
    - `InvoiceRoom` - Room charge line item
    - `HotelInfo` - Hotel information
    - `ReceiptData` - Receipt data extending InvoiceData

## [2.3.0] - 2026-02-05

### Added
- **Room Type Management for New Mode** - Full CRUD room type management at `/new/room-types`
  - `RoomTypeForm` component (`/components/forms/RoomTypeForm.tsx`) - Modal form for create/edit room types
    - Fields: Type Code, Type Name, Base Price, Max Guests, Bed Type, Room Size
    - Thai language labels
    - Validation for required fields
  - New Mode Room Types Page (`/app/new/room-types/page.tsx`)
    - Grid view of room types with cards
    - Each card shows type info, price, and amenities
    - Add/Edit/Delete functionality
  - Backend API: `/api/new/room-types` - Full CRUD with validation
    - Unique type code enforcement
    - Protection against deleting types in use

- **Guest Registry for New Mode** - TM.30 compliance guest tracking
  - `GuestRegistryModal` component (`/components/modals/GuestRegistryModal.tsx`)
    - Add additional guests to a check-in record
    - Guest fields: Name, ID Number, Nationality, Contact
    - Nationality dropdown with common countries
    - View and remove registered guests
  - Backend API: `/api/new/checkins/:id/guests` - Guest registry endpoints
    - GET - List all guests for a check-in
    - POST - Add a guest to a check-in
    - DELETE - Remove a guest from a check-in
    - Validates check-in is active before allowing changes

- **Booking Management UI for New Mode** - Full CRUD booking management at `/new/bookings`
  - `BookingForm` component (`/components/forms/BookingForm.tsx`) - Modal form for create/edit bookings
    - Thai language labels throughout
    - Buddhist Era (B.E.) date display with automatic night calculation
    - Customer picker integration with "Add New Customer" option
    - Multi-room selection via RoomPicker
    - Booking source dropdown (Walk-in, Phone, Online, OTA)
    - Deposit amount field
    - Combined notes field for special requests and internal notes
    - Cancel booking functionality with confirmation
  - `RoomPicker` component (`/components/pickers/RoomPicker.tsx`) - Visual room selector
    - Card-based room display with number, type, and price
    - Multi-select capability with selected room badges
    - Filter by room type
    - Rooms grouped by floor
    - Visual status indicators (available, occupied, maintenance, cleaning)
  - New Mode Bookings Page (`/app/new/bookings/page.tsx`)
    - Table with columns: booking number, date, status, customer, check-in, check-out, rooms
    - Search by booking number or customer name
    - Filter by status and date range
    - Add booking button
    - Click row to edit
    - Pagination with page navigation
    - Status badges with color coding

## [2.2.0] - 2026-02-05

### Added
- **Quick Check-In/Check-Out Modals for New Mode** - Dashboard room cards now support quick actions
  - `QuickCheckInModal` - Walk-in guest check-in form with customer search, expected checkout date picker, rate per night input
  - `CheckOutModal` - Checkout confirmation with stay summary, total calculation (nights x rate), payment method selection
  - Thai Buddhist Era (B.E.) date display support
  - Payment methods: Cash, Credit Card, Transfer
- **RoomGrid Enhanced for New Mode** - Room cards show action buttons when in New Mode
  - Available rooms: "Quick Check-In" button
  - Occupied/Checkout rooms: "Check-Out" button
  - Visual indicator for New Mode active

### Changed
- Dashboard page now detects system mode via `useMode()` hook
- Room grid displays appropriate actions based on room status and system mode

## [2.1.0] - 2026-02-05

### Added
- **Dual-Database Architecture** - Support for both legacy and new HotelNew database
  - New database `HotelNew` with application-owned tables (HT_Customers, HT_Rooms_New, HT_Bookings, HT_CheckIns, etc.)
  - Migration file `migrations/002_create_new_hotel_database.sql` with complete schema
  - Backend supports dual connection pools (legacy + new_hotel)
  - System mode toggle: Legacy (view-only) vs New (full CRUD)
- **Mode Toggle UI** - Navbar button to switch between Legacy and New modes
  - Mode persisted in localStorage
  - Visual indicators: amber for Legacy, green for New
  - Calendar page shows data source indicator
- **Hybrid Calendar Endpoint** - `/api/calendar` fetches from both databases in New mode
  - Color-coded entries by data source (legacy vs new)
  - Combined view of bookings and check-ins from both systems
- **New Database CRUD Routes** (Rust backend)
  - `/api/new/customers` - Full CRUD for HT_Customers
  - `/api/new/rooms` - Full CRUD for HT_Rooms_New
  - `/api/new/bookings` - Full CRUD for HT_Bookings with room assignments
  - `/api/new/checkins` - Check-in/check-out management
  - `/api/mode` - Get current system mode

### Changed
- Calendar page now uses mode context to fetch from appropriate endpoint
- Frontend wrapped with ModeProvider for global mode state

## [2.0.0] - 2026-02-05

### Changed
- **BREAKING: Backend Migration Complete** - All API endpoints now served by Rust/Axum backend
  - Frontend proxies API requests via Next.js rewrites to `http://backend:3003`
  - Removed all Next.js API routes (except `/api/changelog` which reads local CHANGELOG.md)
  - Removed `lib/db.ts`, `lib/scheduler.ts`, `lib/slack.ts`, and `instrumentation.ts`
  - Removed `mssql` and `node-cron` dependencies
  - Frontend is now purely a React UI layer

### Removed
- Next.js API routes: `/api/rooms/*`, `/api/bookings/*`, `/api/checkins`, `/api/customers/*`, `/api/stats`, `/api/occupancy`
- Database-related tests (`__tests__/api/`, `__tests__/integration/`)
- Test scripts: `test:db`, `test:api`, `test:slack`

## [1.19.1] - 2026-02-05

### Fixed
- **Rust backend build issues** - Fixed CI/CD pipeline failures:
  - Updated Dockerfile to use Rust 1.83 (yoke dependency requires rustc 1.82+)
  - Fixed bb8_tiberius error type conversion in error.rs
- **API proxy configuration** - Added Next.js rewrites to forward `/api/*` requests to Rust backend
- **Docker Compose configuration** - Added `BACKEND_URL` environment variable for frontend-to-backend communication

## [1.19.0] - 2026-02-05

### Added
- **Rust Backend Implementation** - Complete Rust/Axum backend in `hotel-backend/` directory
  - All 15 API endpoints ported from Next.js API routes to Rust
  - tiberius for SQL Server connection with bb8 connection pooling
  - tokio-cron-scheduler for background jobs (hourly reports, polling)
  - Slack notification integration with retry logic
  - Thai Buddhist date formatting utilities
  - Docker support with multi-stage build
  - Full API compatibility with existing React frontend

## [1.18.0] - 2026-01-30

### Added
- **Clickable bar chart segments** - Calendar stacked bar chart segments are now clickable
  - Click on any colored segment (continuing stays, new check-ins, or bookings) to view details
  - Detail panel shows list of stays with check-in date, check-out date, and number of nights
  - Bookings also display booking date in the detail view
  - Visual hover feedback on bar segments

## [1.17.0] - 2026-01-30

### Added
- **Stay Timeline Calendar** - New Gantt-style visualization for stays at `/calendar`
  - Horizontal bars showing stay duration (check-in to check-out)
  - Shows both check-ins (actual guests) AND bookings (reservations)
  - **Aggregates stays with same dates** - Groups identical check-in/check-out patterns with count
  - Daily occupancy heat bar showing room density per day
  - Color-coded by stay length (1 night = light blue, 7+ nights = purple)
  - Stats summary: total stays, check-ins, bookings, nights, and average stay
  - Stay length distribution breakdown (1 night, 2-3 nights, 4-7 nights, 7+ nights)
  - Month navigation with Thai Buddhist Era dates

### Changed
- **Calendar page completely redesigned** - Replaced grid calendar with timeline view
- **Simplified stay display** - Focus on stay patterns, not individual customer details

## [1.16.2] - 2026-01-29

### Changed
- **Dashboard stats cards redesign** - Clean white card design with improved spacing
  - Removed colored backgrounds and icons for cleaner look
  - Consistent white color scheme matching other dashboard cards
  - Responsive grid layout (2 columns mobile, 3 tablet, 4 desktop)

## [1.16.1] - 2026-01-29

### Fixed
- **Performance: Removed sluggish animations** - Eliminated `transition-all` and unnecessary `transition-colors` classes that caused layout thrashing and janky interactions:

### Added
- **Database migrations folder** (`/migrations/`) - SQL migration files for tracking database schema changes
  - `001_create_booking_notes_table.sql` - Documents the HT_Booking_Notes table created in v1.16.0
  - `README.md` - Migration guidelines, shared database warnings, and table ownership documentation
- **CLAUDE.md database migration instructions** - Mandatory process for creating migration files when modifying database schema

### Changed
- **Upgraded pnpm to version 10 in Dockerfile** - Matches CI workflow pnpm version, eliminates version mismatch warnings
  - `/app/bookings/page.tsx`: Removed `transition-all duration-300` from main content container, removed `transition-colors` from table rows
  - `/components/RoomGrid.tsx`: Removed `transition-all duration-200` from room cards (kept `hover:shadow-md`)
  - `/app/rooms/page.tsx`: Removed `transition-all` from filter cards and `transition-colors` from list rows
  - `/app/page.tsx`: Removed `transition-colors` from activity list items

### Changed
- **Moved react-datepicker CSS to root layout** - CSS now loads once in `/app/layout.tsx` instead of per-component in `/app/bookings/page.tsx`, reducing redundant CSS parsing

## [1.16.0] - 2026-01-29

### Added
- **Bookings Admin Console Overhaul** - Complete rewrite of the bookings page with improved UX:
  - Bookings now grouped by booking number (multi-room bookings show as single row)
  - Click any booking row to open detail drawer with full info
  - New booking notes feature - add, view, and delete notes per booking
  - Shows all rooms in a booking with room types
  - Customer details section
  - Enhanced search: search by booking number OR customer name

### Changed
- **API: /api/bookings** - Now returns grouped bookings instead of individual room records
- **New API: /api/bookings/[id]** - Get single booking detail with notes
- **New API: /api/bookings/[id]/notes** - CRUD operations for booking notes
- **New Component: BookingDetailDrawer** - Side drawer for comprehensive booking view
- **Database: HT_Booking_Notes table** - Auto-created on first note addition

## [1.15.9] - 2026-01-29

### Changed
- **Fixed Navbar to exactly 2 breakpoints** - Removed container class, use single lg: breakpoint (1024px). Desktop shows full text, mobile shows icons only. All menus always visible at top.

## [1.15.8] - 2026-01-29

### Changed
- **Improved Navbar responsive scaling** - Simplified to 2 states: desktop (full text) and mobile (icons only). Title now hides on mobile, added whitespace-nowrap to prevent text wrapping

## [1.15.7] - 2026-01-29

### Fixed
- **Fixed Docker build excluding CHANGELOG.md** - Added exception in .dockerignore to include CHANGELOG.md in build context

## [1.15.6] - 2026-01-29

### Fixed
- **Fixed CHANGELOG.md missing in Docker container** - Added CHANGELOG.md to Dockerfile runner stage so the /api/changelog endpoint can read it at runtime

## [1.15.5] - 2026-01-29

### Fixed
- **Fixed /api/changelog endpoint returning 500 error** - Changed from fetching GitHub releases API (which returned 404 for private/non-existent repo) to parsing local CHANGELOG.md file directly. This is more reliable and doesn't require external API calls or authentication.

## [1.15.4] - 2026-01-29

### Changed
- **Refactored CustomTooltip in Charts.tsx** - Moved outside OccupancyChart component to prevent unnecessary recreation on each render
- **Added ESLint 9 flat config** - Configured with Next.js and core-web-vitals rules

## [1.15.3] - 2026-01-29

### Changed
- **Added caching to middleware CI workflow** - Significantly speeds up Windows and macOS builds
  - Added Rust dependency caching using `Swatinem/rust-cache@v2` (~55% faster builds after initial run)
  - Added npm caching via `actions/setup-node@v4` cache option
  - Expected improvement: ~13 min → ~5-6 min (Windows), ~11 min → ~4-5 min (macOS)

## [1.15.2] - 2026-01-29

### Fixed
- **Fixed middleware CI workflow** - Corrected Rust toolchain action from non-existent `dtolnay/rust-action` to `dtolnay/rust-toolchain`, and fixed invalid `universal-apple-darwin` target by installing the correct `aarch64-apple-darwin` and `x86_64-apple-darwin` targets separately

## [1.15.1] - 2026-01-29

### Changed
- **Migrated package manager to pnpm** - Full migration from npm to pnpm for consistent tooling
  - Dockerfile now uses `corepack enable && corepack prepare pnpm@9` with `pnpm install --frozen-lockfile`
  - CI/CD workflow updated to use pnpm version 9 (was 8)
  - Removed `package-lock.json`, now using `pnpm-lock.yaml`
  - Benefits: faster installs, better disk efficiency, consistent with CI environment

## [1.15.0] - 2026-01-29

### Security
- **Upgraded Next.js from 15.5.11 to 16.1.6** - Resolves 1 Dependabot security alert:
  - MODERATE: Unbounded Memory Consumption via PPR Resume Endpoint (GHSA-5f7q-jpqc-wp7h) - fixed in >= 15.6.0
- **Upgraded ESLint from 8.57.1 to 9.39.2** - Required by eslint-config-next 16.x
- Upgraded eslint-config-next from 15.5.11 to 16.1.6

### Changed
- **ESLint configuration migrated to flat config format** (ESLint 9 requirement)
  - Added `eslint.config.mjs` with Next.js flat config
  - Added `@eslint/eslintrc` dependency for flat config support
  - Updated lint script to use `eslint .` (Next.js 16 removed `next lint` command)

### Fixed
- Moved `CustomTooltip` component outside of render function in `Charts.tsx` to fix React Hooks lint error

## [1.14.1] - 2026-01-29

### Changed
- **Middleware build pipeline migrated from Electron to Tauri** - `middleware-build.yml` now builds the Tauri-based Thai ID Middleware instead of the Electron version
  - Triggers on changes to `thai-id-middleware-tauri/` instead of `thai-id-middleware/`
  - Uses Rust toolchain with `dtolnay/rust-action` for cross-platform builds
  - Builds macOS Universal binary (Apple Silicon + Intel) and Windows x64
  - Produces smaller artifacts (~10MB vs ~150MB Electron)

### Removed
- `tauri-build.yml` workflow - consolidated into `middleware-build.yml`

## [1.14.0] - 2026-01-29

### Security
- **Upgraded Next.js from 14.2.35 to 15.5.11** - Resolves 4 Dependabot security alerts:
  - HIGH: HTTP request deserialization DoS via React Server Components (GHSA-qpjv-v59x-3qc4) - fixed in >= 15.0.8
  - HIGH: HTTP request deserialization DoS via React Server Components (duplicate alert) - fixed in >= 15.0.8
  - MEDIUM: Image Optimizer remotePatterns DoS (GHSA-qfcj-68r8-w26x) - fixed in >= 15.5.10
  - MEDIUM: Image Optimizer remotePatterns DoS (duplicate alert) - fixed in >= 15.5.10
- **Upgraded React from 18.3.1 to 19.1.0** - Required by Next.js 15
- Upgraded eslint-config-next from 14.2.35 to 15.5.11

### Changed
- **Breaking Change Migration (Next.js 15)**:
  - API route params are now async (Promise-based) - updated all dynamic routes
  - JSX namespace changed from `JSX.Element` to `React.JSX.Element`
  - Removed deprecated `experimental.instrumentationHook` from next.config.js (now enabled by default)

### Note
- **glib vulnerability (RUSTSEC-2024-0429)** - Unsoundness in `VariantStrIter` iterator
  - Status: **Cannot be fixed** - glib 0.18.5 is constrained by Tauri's GTK3 stack (gtk 0.18.x)
  - Impact: **Low** - Linux builds only, vulnerable API not used by application
  - The gtk-rs GTK3 bindings are unmaintained and pinned to glib 0.18.x
  - Fix will come when Tauri migrates to GTK4 or updates dependencies
  - Tracked upstream: waiting for Tauri ecosystem update

## [1.13.3] - 2026-01-29

### Added
- **Photo display in Tauri GUI** - Cardholder's photo now displays in the Tauri frontend when reading cards
  - Photo appears at the top of the debug output when clicking "Test Read"
  - Styled with rounded corners and blue border to match the app theme
- **Debug mode toggle** (🔧 button) in Tauri GUI header
  - When debug=off: Only shows status indicators (HTTP Server, Card Reader, Card) - 400×340px
  - When debug=on: Shows full UI with endpoints, debug tools, and footer - 400×760px
  - Fixed window sizes (non-resizable) that adjust when toggling debug mode
  - Starts in compact mode (debug=off) by default
- **System tray icon** restored - click to show/focus the main window
- **Photo reading support** in Thai ID Middleware Tauri - Read cardholder's photo from Thai ID card
  - New `?photo=true` query parameter for `GET /read` endpoint
  - Photo returned as base64-encoded JPEG in `data.photo` field
  - Photo reading adds ~2 seconds (20 APDU commands for 5KB JPEG data)
  - Example: `curl "http://localhost:9898/read?photo=true" | jq '.data.photo'`
- **Enhanced debug endpoint** (`GET /debug`) now returns comprehensive card information:
  - ATR (Answer To Reset) - card identification bytes
  - Protocol (T=0 or T=1) - smart card communication protocol
  - Reader name
  - AID test results - tests 4 known Thai ID card application IDs with status words
  - Raw read result - shows actual APDU response for CID read command
  - Human-readable status word descriptions (6A82 = File not found, etc.)

### Changed
- Thai ID Middleware Tauri version bumped to 1.1.0
- `read_card` Tauri command now accepts optional `include_photo` parameter
- CardData struct now includes optional `photo` field (base64 string)

## [1.13.2] - 2026-01-29

### Fixed
- **Tauri app crash on macOS** - Fixed SIGABRT crash during app launch
  - Root cause: PNG icon files had 16-bit color depth instead of 8-bit RGBA
  - Converted all icons (32x32.png, 128x128.png, 128x128@2x.png, icon.png) to 8-bit RGBA format
  - Simplified HTTP server lifecycle management to prevent premature shutdown
- **Card reader connection issues** - Fixed "smart card not responding to reset" error
  - Changed from `Protocols::ANY` to explicit `Protocols::T0` for Thai ID cards
  - Added fallback to T1 and ANY protocols if T0 fails
  - Thai ID cards use T=0 protocol which is now tried first for better compatibility

### Added
- **Debug mode for card reader** - Verbose logging can be enabled for troubleshooting
  - HTTP endpoints: `GET /debug`, `GET /debug/enable`, `GET /debug/disable`
  - Tauri commands: `set_debug(enabled)`, `get_debug()`
  - When enabled, logs APDU commands, responses, and connection details to stderr

### Removed
- System tray functionality (temporarily) - Removed to simplify debugging; will be re-added in future version

## [1.13.1] - 2026-01-29

### Fixed
- Memory leak in card reader page causing PC freezing - health check useEffect was using `[checkHealth]` dependency which could cause interval accumulation during re-renders or React Strict Mode double-mounting; changed to `[]` to ensure interval is created exactly once on mount

## [1.13.0] - 2026-01-29

### Added
- **Thai ID Middleware Tauri application** - Complete migration from Electron to Tauri for better macOS Gatekeeper support
  - New `thai-id-middleware-tauri/` directory with full Tauri 2.0 implementation
  - **Rust PC/SC card reader** (`card_reader.rs`) - Native implementation using `pcsc` crate
    - All APDU commands for Thai National ID cards (CID, names, DOB, gender, address, dates)
    - TIS-620 to UTF-8 Thai text encoding conversion
    - Retry logic for cold-inserted cards (5 retries, 1000ms delay)
    - Proper SW1=0x61 response handling with GET RESPONSE
  - **Axum HTTP server** (`server.rs`) - Rust HTTP API on port 9898
    - `GET /health` - Server and reader status
    - `GET /status` - Alias for /health
    - `GET /read` - Read Thai ID card data
    - CORS enabled for localhost web apps
  - **Tauri IPC commands** (`commands.rs`) - Frontend integration
    - `get_status`, `get_version`, `read_card`, `debug_card` commands
  - Frontend ported from Electron with Tauri API integration
- Benefits over Electron:
  - Smaller binary size (~10MB vs ~150MB)
  - Better macOS code signing and Gatekeeper compatibility
  - Lower memory usage
  - Native Rust performance for card operations

## [1.12.5] - 2026-01-29

### Added
- Comprehensive diagnostic logging for Thai ID card reader to help diagnose connection issues
  - Operation counter (`[op:N]`) to correlate connect/disconnect pairs across functions
  - Detailed logging in `resetCard()` showing connect success/failure and disconnect results
  - Detailed logging in `connectWithRetry()` showing each attempt, errors, and retry decisions
  - Entry/exit logging in `readCard()` and `debugCard()` with success/failure status
  - Protocol name logging (T=0/T=1) for successful connections
  - Now logs exact PC/SC error messages to help diagnose silent failures

### Changed
- `connectWithRetry()` now retries on any error, not just "unresponsive" errors
- Middleware version bumped to 1.1.5

## [1.12.4] - 2026-01-29

### Fixed
- Thai ID card reader reliability issues: cards becoming unreadable after ~30 seconds and cold-inserted cards failing
  - Root cause 1: `SCARD_LEAVE_CARD` disconnect mode leaves card in corrupted state after repeated use
  - Root cause 2: Insufficient retry time (1.5s) for cold-inserted cards needing full power cycle
  - Changed all `SCARD_LEAVE_CARD` to `SCARD_RESET_CARD` - performs warm reset clearing card state
  - Added `resetCard()` function to reset cards in unknown state before connecting
  - Increased retry parameters from 3×500ms to 5×1000ms (5 seconds total)
  - On first connection failure, attempts card reset before retrying
- Middleware version bumped to 1.1.4

## [1.12.3] - 2026-01-29

### Fixed
- Thai ID card reader failing with "Card is unresponsive" error when app starts with card already inserted
  - Root cause: Race condition between card detection and card readiness during power-up sequence
  - Added `connectWithRetry()` helper that retries connection up to 3 times with 500ms delay
  - Both `readCard()` and `debugCard()` now use retry logic to handle cards still initializing
- Middleware version bumped to 1.1.3

## [1.12.2] - 2026-01-29

### Fixed
- Thai ID card reader returning empty data for all fields (CID, names, dates, etc.) despite successful card communication
  - Root cause: `readCard()` used plain `transmit()` instead of `transmitWithGetResponse()` for READ commands
  - When card returns SW1=61 (more data available), `transmitWithGetResponse()` sends GET RESPONSE to retrieve data
  - Also changed from parallel `Promise.all()` to sequential reads (smart cards are sequential devices)
- Middleware version bumped to 1.1.2

## [1.12.1] - 2026-01-29

### Fixed
- Thai ID card reader failing with "Failed to select Thai ID applet" on real Thai National ID cards
  - Root cause: Cards return SW 61XX (more data available) instead of 9000 for SELECT commands
  - SW1=61 is valid ISO 7816-4 success response meaning XX bytes of data are pending
  - Added `transmitWithGetResponse()` to automatically send GET RESPONSE (00 C0 00 00 XX) when needed
  - Updated `debugCard()` to recognize SW1=61 as success indicator
- Middleware version bumped to 1.1.1

## [1.12.0] - 2026-01-29

### Added
- Thai ID Middleware debug mode for diagnosing card reading issues
  - "Test Read" button to attempt card read and display results in the app
  - "Debug Info" button to show card ATR and test multiple Application IDs (AIDs)
  - Dark-themed output panel displaying diagnostic information
  - Tests multiple known Thai ID card AIDs (Standard, Alternate, MOI, EMV)
  - Shows APDU status words with human-readable descriptions
- ATR (Answer To Reset) capture when card is inserted for identification
- Window is now resizable to accommodate debug panel

## [1.11.1] - 2026-01-29

### Fixed
- Middleware build workflow failing with "flate: corrupt input before offset 79" on both Windows and macOS
- Root cause: icon.png was only 64x64 pixels, but electron-builder requires at least 512x512 for macOS and 256x256 for Windows
- Solution: Updated icon.svg to 512x512 dimensions and added SVG-to-PNG conversion step in workflow
  - macOS: Uses `librsvg` (`rsvg-convert`)
  - Windows: Uses `Inkscape` CLI

### Changed
- Icon is now generated from SVG during CI/CD build instead of being committed as PNG
- Updated `generate-icon.js` script with new 512x512 SVG design

## [1.11.0] - 2026-01-29

### Changed
- Thai ID Middleware distribution changed from source code (zip) to pre-built executables
- Card reader download page now offers platform-specific downloads (Windows .exe, macOS .dmg)
- Simplified installation: download and run, no npm required
- Added macOS Gatekeeper bypass instructions for unsigned app (right-click → Open or System Settings → Privacy & Security)

### Added
- GitHub Actions workflow (`middleware-build.yml`) for automated cross-platform builds
  - Builds Windows portable executable on `windows-latest`
  - Builds macOS disk image on `macos-latest`
  - Creates GitHub Release when manually triggered with version

### Removed
- `public/downloads/thai-id-middleware.zip` - replaced by GitHub Releases

## [1.10.0] - 2026-01-29

### Security
- Fixed npm vulnerabilities in thai-id-middleware: updated electron (^40.0.0) and electron-builder (^26.6.0)

### Added
- Middleware download available from card reader page (`/card-reader`) - users can download the zip file directly from the web app
- Thai ID Middleware Electron desktop app (`thai-id-middleware/`) for cross-platform Thai ID card reading
  - GUI status display: HTTP server, reader connection, card insertion status
  - HTTP server on localhost:9898 with `/health` and `/read` endpoints
  - System tray support for background operation
  - Cross-platform builds: Windows portable .exe, macOS .dmg, Linux .AppImage
  - PC/SC smart card communication using @pcsclite/client
  - Full Thai National ID card data reading: CID, names (Thai/English), DOB, gender, address, issue/expiry dates

## [1.9.0] - 2026-01-29

### Changed
- Reverted card reader from WebUSB to middleware approach (WebUSB blocked for CCID devices)

### Removed
- WebUSB card reader module (browser security prevents access to smart card readers)

## [1.8.0] - 2026-01-29

### Changed
- Card reader middleware URL is now configurable via `NEXT_PUBLIC_CARD_READER_URL` environment variable (build-time)

### Added
- Thai ID Card Reader POC page (`/card-reader`) for reading guest information from Thai national ID cards
- Connects to local middleware service on `localhost:9898` for PC/SC card reader communication
- Displays all card data: citizen ID, Thai/English names, birth date, gender, address, issue/expiry dates, and photo
- Connection status indicator with automatic health checks
- Setup instructions displayed when middleware is not running
- "ใช้ข้อมูลนี้" button for future check-in integration
- New "อ่านบัตร" navigation link in navbar

## [1.7.3] - 2026-01-29

### Changed
- Rooms page detail panel now displays Room_Group, Room_Book_Name (ผู้จอง), and all price tiers (A, B, C)
- Updated GuestInfo interface to match API response (`checkIn`/`checkOut` instead of `checkInDate`/`checkOutDate`)
- fetchRoomDetail now correctly handles new `/api/rooms/[id]` response structure

## [1.7.2] - 2026-01-29

### Added
- Room detail endpoint `/api/rooms/[id]` returning room details with current guest information from check-in records
- Additional room fields in `/api/rooms` API: `Room_PriceA`, `Room_PriceB`, `Room_PriceC`, `Room_Group`, `Room_Book_Name`

### Changed
- Room interface updated to use actual database column names (`Room_PriceA` instead of `Room_Price`, `Room_Group` instead of `Room_Floor`)

## [1.7.1] - 2026-01-28

### Fixed
- Customer API failing with "Invalid column name 'Book_Cust_No'" when `includeLastVisit=true` - changed to correct column name `Book_Cust_ID`
- Customer bookings API (`/api/customers/[id]/bookings`) using wrong column `Book_Cust_No` - changed to `Book_Cust_ID`
- Customer stats API (`/api/customers/[id]/stats`) using wrong columns - changed `Book_Cust_No` to `Book_Cust_ID` and `Cin_Cust_No` to `Cin_cust_no` (case-sensitive)
- Parameter types changed from `sql.Int` to `sql.NVarChar` since customer IDs are strings like "C0001"

## [1.7.0] - 2026-01-28

### Added
- Server-side sorting for customers table - sorts the entire dataset, not just the visible page
- "Last Visit" column in customers table showing each customer's most recent checkout date
- DataTable component now supports controlled server-side sorting via `onSort`, `sortColumn`, and `sortDirection` props

### Changed
- Customers API now accepts `sortBy` and `sortOrder` query parameters for server-side sorting
- Customers API supports optional `includeLastVisit=true` parameter to include last visit dates

## [1.6.9] - 2026-01-28

### Fixed
- Slack notification times displaying 7 hours ahead - now uses UTC for database dates (which store local Thai time) and Asia/Bangkok for current timestamps

## [1.6.8] - 2026-01-28

### Added
- Customer statistics cards in detail modal showing: total bookings, total stays, first/last visit dates, favorite room type, and average stay duration
- New API endpoint `/api/customers/[id]/stats` for customer statistics
- New API endpoint `/api/customers/[id]/bookings` for customer booking history

## [1.6.7] - 2026-01-28

### Changed
- Customer search now supports multiple fields: name, phone number, ID card (13-digit), and customer ID

## [1.6.6] - 2026-01-28

### Added
- Changelog page (`/changelog`) displaying GitHub release history
- New API endpoint `/api/changelog` fetching releases from GitHub API with 5-minute caching
- "ประวัติ" navigation link in navbar

## [1.6.5] - 2026-01-28

### Fixed
- `/customers` page not working due to API response structure mismatch - transformed SQL column names (`Cust_no`, `Cust_name`, etc.) to frontend-expected format (`id`, `name`, etc.) and flattened pagination response

## [1.6.4] - 2026-01-28

### Added
- `pnpm test:slack` script to run Slack integration tests with verbose output

## [1.6.3] - 2026-01-28

### Fixed
- Slack notifications not working in production Docker container - added `SLACK_WEBHOOK_URL` and `SLACK_NOTIFICATIONS_ENABLED` environment variables to docker-compose.yml
- Updated GitHub Actions workflow to pass Slack webhook secret during deployment

## [1.6.2] - 2026-01-28

### Changed
- RoomGrid mobile view now displays rooms as a sorted list instead of floor plan grid
- Desktop view retains the original floor plan grid layout
- Mobile list shows room number, type, and details with status indicator

## [1.6.1] - 2026-01-28

### Changed
- RoomGrid now mobile responsive with horizontal scroll, preserving floor plan layout
- Responsive cell sizes (60px mobile, 70px desktop), text sizes, and legend
- Fixed React key warning in RoomGrid row fragments

## [1.6.0] - 2026-01-28

### Added
- Checkout notifications: Real-time Slack alerts when guests check out (polled every 2 minutes via `Cin_Room_Out` field)
- New booking notifications: Real-time Slack alerts when new bookings are created (polled every 2 minutes via `Book_Date` field)
- New functions in `lib/slack.ts`: `buildCheckOutAlertMessage`, `buildNewBookingAlertMessage`
- New polling functions in `lib/scheduler.ts`: `pollCheckouts`, `pollNewBookings`

## [1.5.7] - 2026-01-28

### Fixed
- Added missing rooms V.201, A2-1, A2-3 to RoomGrid display on the 4th floor row

## [1.5.6] - 2026-01-28

### Changed
- Switched from cloudflared SSH to self-hosted GitHub Actions runner for deployment - simpler and more reliable

## [1.5.5] - 2026-01-28

### Added
- Automated deployment via SSH in CI/CD pipeline - after build, automatically deploys to production server via Cloudflare tunnel

## [1.5.4] - 2026-01-28

### Fixed
- Fixed stats API counts not matching room grid display - checkout queries now filter to only the most recent check-in record per room using MAX(Cin_Room_In) subquery, preventing historical records from incorrectly counting as today's checkouts

## [1.5.3] - 2026-01-26

### Fixed
- Fixed checkout-today query returning old records from guests who already checked out - now only returns rooms where guest is still checked in (`Room_Use = 'yes'`)

## [1.5.2] - 2026-01-26

### Fixed
- Fixed occupied room count mismatch between stats card and room grid - stats now excludes checkout rooms (after 6 AM) from occupied count
- Occupied rooms count now matches the number of red squares on the grid

### Added
- New "ห้องรอเช็คเอาท์" stat card (blue) showing rooms waiting for checkout today

### Changed
- API integration tests now spin up their own Next.js dev server on port 30031, making tests self-contained and independent of manually running dev server

## [1.5.1] - 2026-01-26

### Fixed
- Fixed "รอเช็คเอาท์" (waiting for checkout) rooms not showing on grid - now uses `View_CheckIn_Ds.Cin_Room_Out` date matching (same method as stats API) instead of unreliable `View_Room_status.room_status` filtering
- Added new `/api/rooms/checkouts-today` endpoint for reliable checkout room detection

## [1.5.0] - 2026-01-26

### Added
- New "booked" (จองแล้ว) room status with yellow color to distinguish rooms with reservations from rooms with checked-in guests
- New "ห้องที่จองแล้ว" stat card on dashboard showing count of booked-but-not-checked-in rooms
- Separate tracking: "ห้องที่มีผู้เข้าพัก" now only counts checked-in guests, "ห้องที่จองแล้ว" counts pending arrivals

## [1.4.2] - 2026-01-26

### Fixed
- Fixed occupied room count mismatch in stats API - now correctly counts rooms with any non-empty `Room_Book` value, matching the RoomGrid display logic (was only counting `Room_Book = 'yes'`)

## [1.4.1] - 2026-01-26

### Added
- Slack integration test (`__tests__/integration/slack.test.ts`) that sends actual test messages to verify webhook connectivity
- Added `dotenv` dev dependency for loading environment variables in tests

## [1.4.0] - 2026-01-24

### Added
- Slack notifications for hotel operations
  - Hourly report: Occupied rooms count and today's new bookings (sent every hour at minute 0)
  - Check-in alerts: Real-time notifications when guests check in (polled every 2 minutes)
- New files: `lib/slack.ts`, `lib/scheduler.ts`, `instrumentation.ts`
- Environment variables: `SLACK_WEBHOOK_URL`, `SLACK_NOTIFICATIONS_ENABLED`
- `.env.example` file with all available configuration options

### Technical
- Uses `node-cron` for scheduling background tasks
- Leverages Next.js instrumentation hook for server-side startup
- Includes retry logic (3 attempts with exponential backoff) for Slack API calls
- Thai language message formatting with Buddhist Era dates

## [1.3.4] - 2026-01-24

### Changed
- Pinned pnpm version to 9.x in Dockerfile (was `pnpm@latest`) to prevent cache invalidation on pnpm updates
- Expanded .dockerignore to exclude .husky, .swc, .claude, .github, .vscode, and *.log files from build context

## [1.3.3] - 2026-01-24

### Security
- Added pnpm override to force glob >=10.5.0 to resolve CVE-2025-64756 (command injection in CLI)
  - Vulnerability is in `@next/eslint-plugin-next` dependency chain
  - Note: Only affects CLI usage; ESLint uses glob as a library, so actual risk is minimal

## [1.3.2] - 2026-01-24

### Security
- Upgraded Next.js from 14.2.21 to 14.2.35 to resolve 17 Dependabot security alerts:
  - Authorization Bypass in Middleware (High)
  - Race Condition to Cache Poisoning (High)
  - SSRF via Improper Middleware Redirect (High)
  - DoS with Server Components (Medium)
  - Content Injection in Image Optimization (Medium)
  - Cache Key Confusion in Image Optimization (Medium)
  - Information exposure in dev server (Medium)
- Upgraded eslint-config-next from 14.2.21 to 14.2.35 (resolves transitive glob vulnerability)

## [1.3.1] - 2026-01-24

### Security
- Fixed SQL injection vulnerability in `/api/checkins/route.ts` - now uses parameterized queries for startDate and endDate filters

### Added
- Component tests for RoomGrid (rendering, status colors, modal interaction)
- Component tests for DataTable (sorting, pagination, empty/loading states)
- Component tests for Calendar (month navigation, date selection, booking/checkin indicators)
- CI/CD pipeline now runs component tests before building Docker image
- Pre-push git hook to run component tests before pushing (via husky)

### Fixed
- Removed hardcoded database values from API tests to prevent false failures when data changes

## [1.3.0] - 2026-01-24

### Changed
- Room status indicator bar changed from vertical (left side) to horizontal (bottom)
- Legend moved from top to bottom of room grid

## [1.2.0] - 2026-01-24

### Added
- Docker containerization with multi-stage Dockerfile for optimized image size
- GitHub Actions CI/CD workflow for automated builds to ghcr.io
- docker-compose.yml for easy local deployment from container registry
- Environment variable support for database configuration (DB_SERVER, DB_NAME, DB_USER, DB_PASSWORD)

### Changed
- Next.js output mode set to 'standalone' for container deployment
- Database credentials now configurable via environment variables with backward-compatible defaults

## [1.1.1] - 2026-01-24

### Changed
- Migrated package manager from npm to pnpm for faster installs and better disk efficiency

## [1.1.0] - 2026-01-23

### Added
- "Waiting for Checkout" room status with light blue color
- Rooms with checkout date = today display as "รอเช็คเอาท์" after 6 AM

## [1.0.2] - 2026-01-23

### Fixed
- Fixed timezone display in Recent Activity: database stores local Thai time but mssql marks it as UTC, so using `timeZone: 'UTC'` displays the stored values correctly without adding 7 hours

## [1.0.1] - 2026-01-23

### Fixed
- Recent Activity section now shows check-in and check-out dates with times (e.g., "23 ม.ค. 04:32 - 24 ม.ค. 12:00")
- Fixed timezone display issue where UTC dates appeared as same-day ranges after conversion to Thai timezone

## [1.0.0] - 2026-01-23

### Added
- Initial release of Hotel Management Visualization Web App
- Dashboard with room status grid and occupancy statistics
- Room grid with custom layout matching physical hotel floor plan
- Real-time room status display (available, occupied, maintenance)
- Room details from database (Room_Type, Room_Details)
- Occupancy chart showing actual daily room counts (check-ins + stay-overs)
- Calendar page for viewing bookings and check-ins by date
- Customers page with search and pagination
- Bookings page with filtering options
- Rooms page with status management

### API Endpoints
- `/api/stats` - Dashboard statistics
- `/api/rooms` - Room listing with status
- `/api/bookings` - Booking management with pagination
- `/api/customers` - Customer search and listing
- `/api/checkins` - Check-in records
- `/api/occupancy` - Daily occupancy data for charts

### Technical
- Next.js 14 with App Router
- SQL Server database connection (mssql)
- Tailwind CSS for styling
- Recharts for data visualization
- Jest for testing (22 tests passing)
