//! `WritebackIntent` — the type-only contract for the legacy MSSQL adapter.
//!
//! Each variant maps 1:1 to a recipe in `docs/legacy-spike/findings.md` §3a–k.
//! When the writeback worker (Phase 3b / 4b — `bin/writeback.rs`) dequeues a
//! `writeback_jobs` row, it deserializes the JSON payload into one of these and
//! dispatches to the matching `writeback/<flow>.rs` recipe.
//!
//! **Type definitions only** — no SQL, no tiberius, no I/O. The exact MSSQL
//! field shapes are derived inside the writeback recipes during Wave 2.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::{
    booking::BookingState, payment::PaymentMethod, shared::DateRange, shared::Money,
};

/// A single durable command queued in the `writeback_jobs` outbox.
///
/// Variants intentionally cover only the 11 spike-validated flows. New legacy
/// writebacks (e.g. minibar charges, refunds — gaps noted in `findings.md` §7)
/// must extend both this enum and add a recipe in `writeback/`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "intent", content = "payload", rename_all = "snake_case")]
pub enum WritebackIntent {
    /// Spike §3b — `INSERT HT_Customers (if new)` + `INSERT HT_Book_H` +
    /// `INSERT HT_Book_Ds` + `INSERT HT_Book_Date` (×nights).
    CreateBooking {
        booking_id: Uuid,
        payload: CreateBookingPayload,
    },

    /// Spike §3c — targeted UPDATEs against `HT_Book_H` / `HT_Book_Ds` and
    /// add/remove `HT_Book_Date` rows. **Skip the legacy app's destructive
    /// DELETE-then-REINSERT pattern** (per spike §3c).
    ModifyBooking {
        booking_id: Uuid,
        changes: BookingChanges,
    },

    /// Spike §3g-bis — clean cancel: 4 UPDATEs + 1 DELETE on `HT_Book_Date`.
    CancelBooking { booking_id: Uuid },

    /// Spike §3a (walk-in) or §3d (linked to existing booking).
    /// 7 INSERTs + 3 UPDATEs across 7 tables.
    CreateCheckIn {
        check_in_id: Uuid,
        payload: CreateCheckInPayload,
    },

    /// Spike §3i — clean targeted DELETEs + audit row in `HT_Rooms_Cancel`.
    /// Subtracts this room's price from `HT_CheckIn_H` totals (multi-room safe).
    ///
    /// `room_price` is the cancelled room's `Cin_Room_Price` (in baht satang) —
    /// the recipe subtracts it from `HT_CheckIn_H.Total_Price_Room` /
    /// `Total_Price_Net` / `Total_Price_Balance`. `pay_to_subtract` is the
    /// portion of `Total_Price_Pay` to subtract — usually 0 unless a deposit
    /// was paid against only the cancelled room.
    CancelCheckIn {
        check_in_id: Uuid,
        reason: Option<String>,
        /// `Cin_Room_Price` of the cancelled room, in satang. The recipe
        /// converts to baht for the legacy `Total_Price_*` UPDATE.
        #[serde(default)]
        room_price: Money,
        /// Pay amount to subtract from `Total_Price_Pay`, in satang. 0 in the
        /// spike capture; populate only when a per-room deposit was applied.
        #[serde(default)]
        pay_to_subtract: Money,
    },

    /// Spike §3f — recompute totals + replace `HT_Room_Status` rows for the
    /// changed date range. Targeted; skip the destructive Phase B.
    ///
    /// The recipe needs the original `stay_start` (to enumerate the full
    /// `[stay_start, new_end)` calendar range correctly), the guest label
    /// (for `HT_Room_Status.room_Details`), and the recomputed totals so it
    /// doesn't have to re-derive any of them from MSSQL.
    ExtendStay {
        check_in_id: Uuid,
        new_end: DateTime<Utc>,
        /// Original `Cin_Room_In` from the canonical PG state. Used by the
        /// recipe to enumerate calendar nights in `[stay_start, new_end)`.
        #[serde(default = "default_extend_stay_start")]
        stay_start: DateTime<Utc>,
        /// Customer-name label for `HT_Room_Status.room_Details`.
        #[serde(default)]
        guest_label: String,
        /// Recomputed totals to push into `HT_CheckIn_H` (in satang).
        #[serde(default)]
        new_room_price_total: Money,
        #[serde(default)]
        new_net_total: Money,
        #[serde(default)]
        new_pay_total: Money,
        #[serde(default)]
        new_balance_total: Money,
    },

    /// Spike §3e Phase 2 — 5 UPDATEs across `HT_POWER_LOG`, `HT_CheckIn_Ds`,
    /// `HT_Rooms`, `HT_Room_Status`, `HT_CheckIn_H`. **Skip Phase 1** (destructive).
    ///
    /// Audit H1 (Wave 2): the totals fields below propagate the real stay
    /// revenue, nights, and pay/balance from the canonical PG state into
    /// the MSSQL UPDATEs. Prior to the fix the recipe hardcoded
    /// `nights=1, room_price_total=0, ...` regardless of actual stay — every
    /// checkout wiped the legacy revenue with zeros.
    ///
    /// All totals are stored in baht (not satang) because the legacy
    /// `Total_Price_*` columns are MONEY/decimal in baht. They are
    /// `Option<f64>` (not bare `f64`) for back-compat with queued events
    /// emitted before the H1 fix: in-flight rows lack the fields, and the
    /// dispatcher falls back to the prior all-zeros behavior with a WARN.
    CheckOut {
        check_in_id: Uuid,
        /// Nights actually stayed (>=1). The legacy `Cin_Room_night` /
        /// `Room_Use_Count` columns expect this as a whole number;
        /// fractional values are floored.
        #[serde(default)]
        nights: Option<f64>,
        /// Total room revenue in baht. Lands in
        /// `HT_CheckIn_Ds.Cin_Room_PriceTotal` and
        /// `HT_CheckIn_H.Total_Price_Room`.
        #[serde(default)]
        room_price_total: Option<f64>,
        /// Total minibar / extras revenue in baht. Lands in
        /// `HT_CheckIn_H.Total_Price_Product`.
        #[serde(default)]
        product_total: Option<f64>,
        /// Net (room + product) revenue in baht. Lands in
        /// `HT_CheckIn_H.Total_Price_Net`.
        #[serde(default)]
        net_total: Option<f64>,
        /// Total payments tendered in baht. Lands in
        /// `HT_CheckIn_Ds.Cin_Room_Pay_Total` and
        /// `HT_CheckIn_H.Total_Price_Pay`.
        #[serde(default)]
        pay_total: Option<f64>,
        /// Outstanding balance in baht (`net - pay`). Lands in
        /// `HT_CheckIn_H.Total_Price_Balance`.
        #[serde(default)]
        balance: Option<f64>,
    },

    /// Spike §3h — `INSERT HT_CheckIn_Pay` + `UPDATE HT_CheckIn_H` totals.
    /// Receipt INSERTs (`HT_Receipt_H` / `HT_Receipt_Ds`) follow in a separate
    /// intent when the user prints.
    RecordPayment {
        check_in_id: Uuid,
        amount: Money,
        method: PaymentMethod,
        /// Customer + room metadata for the receipt header (`HT_Receipt_H`).
        /// Populated by the route from `ht_customers` + `ht_rooms_new` so the
        /// recipe doesn't have to re-query MSSQL/PG. Defaulted to empty
        /// strings if the route lookup fails (preserves the prior behavior
        /// of issuing a no-detail receipt rather than failing the payment).
        receipt: RecordPaymentReceipt,
        /// Specific `HT_CheckIn_Ds.id` the payment is being apportioned to —
        /// per spike §3h capture line 3, the legacy app fires
        /// `UPDATE HT_CheckIn_Ds SET Cin_Room_Pay_Total=<amt>, Cin_note='' WHERE id=<ds_id>`
        /// just before inserting the `HT_CheckIn_Pay` row. `None` when the
        /// route hasn't resolved a specific room (multi-room check-ins where
        /// the payment is allocated across all rooms). When `None` the recipe
        /// skips the per-room UPDATE — totals on `HT_CheckIn_H` still settle.
        #[serde(default)]
        checkin_ds_id: Option<i32>,
        /// Wave 5a item 2 — canonical per-night rate (`ht_checkins.cin_rate_per_night`)
        /// in baht. Lands verbatim in `HT_CheckIn_Pay.Cin_Pay_Ds_PriceOne` so
        /// the printed receipt line shows the real per-night price. `None`
        /// makes the recipe fall back to `amount/nights` (the prior behavior;
        /// kept defensive for queue rows enqueued before this field landed).
        #[serde(default)]
        price_per_night_baht: Option<f64>,
        /// Wave 5a item 2 — nights covered by the payment (>=1). Lands in
        /// `HT_CheckIn_Pay.Cin_Pay_Ds_Num`. `None` defaults to 1 in the
        /// recipe.
        #[serde(default)]
        nights: Option<i32>,
        /// Wave 5a item 3 — `ht_payments.aggregate_id` of the canonical row
        /// the writeback worker should back-populate with the newly-allocated
        /// `legacy_pay_no` / `legacy_receipt_no`. `None` skips the
        /// back-population step (older queued intents won't carry it).
        #[serde(default)]
        payment_aggregate_id: Option<Uuid>,
        /// Wave 5c — VAT percentage to apply on `HT_Receipt_H`. Read by
        /// the service layer from `ht_settings.vat_percent` so the hotel
        /// can flip between 0%/7% without a code change. `None` makes the
        /// recipe fall back to the legacy hardcoded constant (defensive
        /// for queue rows enqueued before this field landed).
        #[serde(default)]
        vat_percent: Option<i32>,
    },

    /// Spike §3j — `UPDATE HT_Rooms` (by `id`, not `room_no`!) +
    /// `INSERT HT_Housewife` with `h_cin/h_cin_name` looked up from the prior
    /// non-cancelled check-in for this room.
    MarkRoomClean { room_id: Uuid, by: String },

    /// Coexistence audit 2026-06-11 P2 — standalone mark-dirty writeback
    /// (`docs/legacy-app/COMPAT_CHEATSHEET.md` §3.13, ClickClean.cs:493-540).
    ///
    /// Mirrors [`Self::MarkRoomClean`]: `UPDATE HT_Rooms` (by numeric `id`,
    /// not `room_no` — same critical finding as spike §3j) + an
    /// `INSERT HT_Housewife` "start cleaning" audit row referencing the
    /// prior checked-out occupant. Before this intent existed,
    /// `service::housekeeping::mark_dirty` was PG-only and iHOTEL's room
    /// grid kept showing the room clean.
    MarkRoomDirty { room_id: Uuid, by: String },

    /// Coexistence audit 2026-06-11 P2 — room maintenance flag writeback
    /// (`docs/legacy-app/COMPAT_CHEATSHEET.md` §3.15/§3.16).
    ///
    /// `UPDATE HT_Rooms SET Room_Manternace='yes'|'no' WHERE id=<id>` —
    /// the column name is the verbatim legacy typo (`Room_Manternace`,
    /// `hotel-backend/schema-baseline.txt:510`), values are the lowercase
    /// `'yes'` / `'no'` vocabulary the CT room mapper reads back via
    /// `legacy_yesno_to_bool`. Before this intent existed a room taken
    /// out of service in the new app still looked rentable in iHOTEL
    /// (its grid requires `Room_Manternace='no'` for the rentable state).
    ///
    /// Emitted by `routes::new_rooms::update_room` only when the
    /// canonical `room_maintenance` flag actually CHANGES in the edit.
    SetRoomMaintenance {
        /// Canonical aggregate UUID derived from `room_id` via
        /// [`crate::service::ids::aggregate_uuid`]. The writeback worker
        /// resolves `HT_Rooms.id` (numeric internal PK) from
        /// `ht_rooms_new.legacy_room_id_int` keyed on this — same
        /// resolution as [`Self::MarkRoomClean`].
        room_id: Uuid,
        /// `true` ⇒ `Room_Manternace='yes'` (under repair, not rentable);
        /// `false` ⇒ `'no'`.
        maintenance: bool,
    },

    /// Coexistence audit 2026-06-11 P2 — standalone customer-edit
    /// writeback. Before this intent existed, edits made through
    /// `PUT /api/new/customers/:id` never reached `HT_Customers` (only
    /// the `CustomerResave` folded into [`Self::ModifyBooking`] did) and
    /// iHOTEL showed stale phone / address.
    ///
    /// The payload is the full [`CustomerResave`] field set — the same
    /// 31-column `UPDATE HT_Customers ... where Cust_no=...` an iHOTEL
    /// re-save fires (spike §3c capture line 28) — hydrated by
    /// `service::customer::update` from the canonical `ht_customers` row
    /// AFTER the PG UPDATE commits its values. Columns the new-app edit
    /// doesn't touch are hydrated from the CT-mirrored columns
    /// (`cust_add_moo`, `cust_work_*`, …) so the re-save never blanks
    /// iHOTEL-entered values.
    ///
    /// **Customer DELETEs are deliberately NOT written back** — see the
    /// `recipes/update_customer.rs` module doc (iHOTEL's delete is a
    /// destructive hard-DELETE + `'C0000'` cascade, cheatsheet §3.24;
    /// our canonical soft-delete is reversible).
    UpdateCustomer {
        /// Canonical aggregate UUID derived from `cust_id` via
        /// [`crate::service::ids::aggregate_uuid`]. Persisted into
        /// `writeback_jobs.aggregate_id` so jobs for the same customer
        /// group together in the index.
        customer_id: Uuid,
        /// Full re-save payload. `resave.legacy_cust_no` carries the
        /// `HT_Customers.Cust_no` business key directly (resolved by the
        /// service before enqueue — the intent is only emitted for
        /// customers that already exist on the legacy side), so the
        /// writeback resolver no-ops, mirroring [`Self::UpdateRoom`].
        resave: CustomerResave,
    },

    /// Admin / staff edit of room master data (price, type, notes).
    /// Mirrors a PG `ht_rooms_new` UPDATE → legacy `HT_Rooms` UPDATE
    /// keyed by `Room_no`. PG fields that have no MSSQL counterpart
    /// (`room_features`, `room_floor`, `room_view`) are NOT propagated —
    /// they are canonical-only. The legacy `Room_Use` column is also
    /// NOT propagated: it is the legacy *occupancy* flag (`'yes'`/`'no'`)
    /// driven by check-in / check-out flows, not by admin edits.
    /// Keeping the admin-edit / occupancy split keeps housekeeping +
    /// walk-in writebacks authoritative over `Room_Use`.
    ///
    /// Emitted by `routes::new_rooms::update_room`. The payload is the
    /// resolved PG row AFTER the UPDATE; the writeback recipe replays
    /// the full field set on MSSQL. `None` fields are skipped (preserves
    /// any iHOTEL-side value the operator did not touch — narrow update).
    UpdateRoom {
        /// Canonical aggregate UUID derived from `room_id` via
        /// [`crate::service::ids::aggregate_uuid`]. Persisted into
        /// `writeback_jobs.aggregate_id` so jobs for the same room
        /// group together in the index.
        room_id: Uuid,
        payload: UpdateRoomPayload,
    },

    /// Track G2 — `audit-2026-05-13.md` T4 CRIT-1. Refund / negative
    /// payment. The recipe inserts a `HT_CheckIn_Pay` row with a
    /// negative tender amount (per `docs/legacy-app/COMPAT_CHEATSHEET.md:513`
    /// — "Cin_Pay_Cash/Credit ... can be negative (refunds use
    /// negation)") and then re-aggregates `HT_CheckIn_H.Total_Price_Pay`
    /// / `Total_Price_Balance` / `Total_Price_vat` from `HT_CheckIn_Pay`
    /// rows under UPDLOCK+HOLDLOCK (Track C pattern — never additive).
    ///
    /// Receipt-side rows (`HT_Receipt_H` / `HT_Receipt_Ds`) are NOT
    /// emitted for a refund — those are append-only invoice records on
    /// the legacy side, and reprinting a refund receipt is a Track G
    /// follow-on. The refund is solely a tender adjustment against the
    /// existing folio.
    RefundPayment {
        /// Aggregate id of the parent check-in this refund is recorded
        /// against. Required so the writeback worker resolves the legacy
        /// `Cin_no` / `room_no` / `cust_no` triple identically to
        /// [`Self::RecordPayment`].
        check_in_id: Uuid,
        /// Aggregate id of the original `ht_payments` row this refund
        /// offsets. Persisted into the legacy `Cin_Pay_Note` column
        /// (e.g. `"REFUND of Pay_no=R2604-0250"`) when the worker has
        /// back-populated `ht_payments.legacy_pay_no`. Optional so older
        /// queue rows or partial-resolution scenarios still write a
        /// generic refund note.
        original_payment_aggregate_id: Option<Uuid>,
        /// Refund amount as a POSITIVE money value. The recipe negates
        /// it before interpolating into the SQL — keeping the carrier
        /// positive lets the same helpers (`money_2dp`) emit a
        /// `-801.00` string identically across all tender columns.
        amount: Money,
        /// Tender method to negate. Must match the original payment's
        /// method so the refund debits the same tender column.
        method: PaymentMethod,
        /// Operator-supplied free text captured by the route. Lands in
        /// `HT_CheckIn_Pay.Cin_Pay_Note` (prefixed with the legacy
        /// "REFUND of Pay_no=…" reference when available). Empty string
        /// when the operator skipped the field.
        #[serde(default)]
        refund_reason: String,
        /// Wave 5a item 3 — `ht_payments.aggregate_id` of the NEW refund
        /// row (the one we just inserted into PG). The writeback
        /// worker's `back_populate_legacy_ids` step uses this to stamp
        /// the freshly-allocated `legacy_pay_no` onto the canonical
        /// refund row. `None` skips back-population (defensive — older
        /// queued intents).
        #[serde(default)]
        payment_aggregate_id: Option<Uuid>,
    },

    /// Track G4 / T4 HIGH-3 (`audit-2026-05-13.md`,
    /// `docs/legacy-app/COMPAT_CHEATSHEET.md` §`HT_Changed_Room`, 3.17).
    ///
    /// Mid-stay room move. The service inserts the canonical
    /// `ht_room_changes` row and updates the junction inside ONE PG
    /// transaction before enqueuing this intent. The recipe then
    /// loads the row by `rc_id`, INSERTs the corresponding
    /// `HT_Changed_Room` row in MSSQL, and back-populates the legacy
    /// id onto the canonical row. Carries `rc_id` directly because
    /// `HT_Changed_Room` is keyed on a server-allocated IDENTITY —
    /// the canonical row IS the authoritative source for every
    /// column the legacy INSERT needs (no further PG joins required
    /// once the recipe loads it).
    RoomChange {
        /// Aggregate id of the parent check-in this room change is
        /// recorded against. Required so the writeback worker
        /// resolves the legacy `Cin_no` identically to
        /// [`Self::ExtendStay`]. Persisted into
        /// `writeback_jobs.aggregate_id` for index grouping.
        check_in_id: Uuid,
        /// `ht_room_changes.rc_id` of the canonical audit row the
        /// service just inserted. The recipe loads its remaining
        /// columns (`rc_from_room_id` / `rc_to_room_id` numbers,
        /// `rc_reason`, `rc_changed_at`, `rc_changed_by`,
        /// `rc_room_before_price`) from PG before issuing the
        /// legacy INSERT.
        rc_id: i64,
    },

    /// Track F3 — `audit-2026-05-13.md` T1 CRIT-3
    /// (`docs/legacy-app/COMPAT_CHEATSHEET.md:560-564`).
    ///
    /// `UPDATE HT_Products SET Pro_Amt = Pro_Amt + <delta> WHERE Pro_no=<no>`
    /// — closes the stock invariant from our app's writes. The legacy
    /// side still maintains `Pro_Amt` for iHOTEL's own sales; both
    /// updates are additive so concurrent writes never clobber each
    /// other.
    ///
    /// `prod_legacy_no` is the legacy `Pro_no` business key (the
    /// recipe targets MSSQL directly so we don't carry a UUID for the
    /// product — the canonical `ht_products.aggregate_id` is what
    /// drives event subscribers; the writeback only needs the legacy
    /// key). `reason` is captured by the route layer in
    /// `ht_inventory_transactions.trans_notes` before the intent is
    /// enqueued and is NOT propagated to legacy (no audit column on
    /// `HT_Products`).
    AdjustProductStock {
        /// `HT_Products.Pro_no` — business key of the product whose
        /// stock counter to adjust.
        prod_legacy_no: String,
        /// Signed delta. Positive increments stock (e.g. restock
        /// receipt); negative decrements (e.g. consumption recorded
        /// by our app). `f64` so fractional units (litres, kg) are
        /// preserved at the wire boundary.
        delta: f64,
        /// Human-readable reason — captured canonically, NOT written
        /// to legacy. Routes pass-through from the request body.
        #[serde(default)]
        reason: Option<String>,
        /// Canonical `ht_products.aggregate_id` of the row this
        /// intent mutates. Lets subscribers correlate the writeback
        /// completion back to the canonical product row.
        #[serde(default)]
        product_aggregate_id: Option<Uuid>,
    },

    /// Track G5 — issue a new food/breakfast/promo coupon. The service
    /// inserts the canonical `ht_coupons` row inside a PG transaction
    /// before enqueuing this intent. The recipe later allocates a
    /// legacy `HT_Cupon.cupon_no` under TABLOCKX+HOLDLOCK and INSERTs
    /// the row; the writeback worker back-populates `legacy_cupon_no`
    /// onto the canonical row.
    ///
    /// `coupon_id` carries the canonical BIGSERIAL key so the recipe
    /// loads its remaining fields (`cupon_cin_no`, `cupon_cin_room`,
    /// `cupon_date`, `cupon_by`) from PG before issuing the legacy
    /// INSERT — same pattern as Track G4 `RoomChange`.
    IssueCoupon {
        /// Aggregate id of the canonical coupon row (`ht_coupons.aggregate_id`).
        /// Used by the worker's `back_populate_legacy_ids` step to
        /// stamp the freshly-allocated `cupon_no` onto the canonical
        /// row after the legacy INSERT lands.
        coupon_aggregate_id: Uuid,
        /// Canonical BIGSERIAL key. The recipe loads the rest of the
        /// row from PG keyed on this column.
        coupon_id: i64,
    },

    /// Track G5 — redeem (mark printed) an existing coupon. Mirrors
    /// the legacy `UPDATE HT_Cupon SET cupon_print=1 WHERE
    /// cupon_no=...` pattern fired by `Print_Report.cs:2552`.
    ///
    /// The service flips canonical `coupon_status='redeemed'` and
    /// stamps `coupon_redeemed_at` / `coupon_redeemed_cin_id` before
    /// enqueuing this intent. The recipe issues the legacy UPDATE if
    /// `legacy_cupon_no` is resolved; otherwise the worker defers the
    /// flip until the parent `IssueCoupon` writeback completes (the
    /// canonical row already carries the `redeemed` status — legacy
    /// just lags).
    RedeemCoupon {
        /// Aggregate id of the canonical coupon row being redeemed.
        coupon_aggregate_id: Uuid,
        /// Canonical BIGSERIAL key. The recipe loads the resolved
        /// `legacy_cupon_no` from PG keyed on this column.
        coupon_id: i64,
    },

    /// Track G6 — POS / sales-to-room (migration 052).
    ///
    /// One line item rung up against a check-in folio. The service has
    /// already inserted the canonical `ht_pos_sales` row, decremented
    /// `ht_products.prod_current_stock`, and is now asking the writeback
    /// worker to mirror the sale into legacy MSSQL:
    ///
    /// 1. INSERT INTO `HT_CheckIn_Product` (12 cols) OUTPUT INSERTED.id
    /// 2. UPDATE `HT_Products` SET `Pro_Amt = Pro_Amt + (-qty)`
    ///    WHERE `Pro_no = <legacy_no>` (additive — never absolute SET —
    ///    so concurrent iHOTEL sales never get clobbered)
    ///
    /// Carries `sale_id` directly (canonical PK) so the recipe loads the
    /// row's remaining columns from PG via the resolver before issuing
    /// the legacy INSERT. The PK back-link lands on `sale_legacy_id`
    /// after the OUTPUT clause returns the IDENTITY.
    ///
    /// Stock-adjust is bundled into the SAME recipe rather than
    /// emitting a separate `AdjustProductStock` intent: the legacy
    /// app's POS form fires both writes inside one transaction
    /// (`docs/legacy-app/COMPAT_CHEATSHEET.md:560-564`); splitting the
    /// pair into two jobs would let one succeed while the other
    /// failed, breaking the stock invariant the legacy app expects.
    RecordPosSale {
        /// Aggregate id of the parent check-in this sale is recorded
        /// against. Drives the writeback worker's `legacy_cin_no`
        /// + `legacy_room_no` resolution (same resolver branch as
        /// `RecordPayment`).
        check_in_id: Uuid,
        /// `ht_pos_sales.sale_id` of the canonical row the service
        /// just inserted. The recipe loads `sale_qty`, `sale_unit_price`,
        /// `sale_note`, the joined `prod_legacy_no` + `prod_name` +
        /// `prod_unit` from PG before issuing the legacy INSERT.
        sale_id: i64,
    },

    /// Track J6 (round-bill coexistence step 2) — our app **opens** a
    /// cashier round, mirroring iHOTEL's `FrmDueBill.cs:1653`
    /// (`COMPAT_CHEATSHEET.md` §946 / §3.20):
    /// `INSERT HT_Round_Bill (id, round_start, round_price, round_by)`
    /// with `round_end` NULL. The legacy `id` is **explicit** in the
    /// payload (allocated by `ShiftService::open_shift` as
    /// `MAX(shift_no)+1`, the same value mirrored into
    /// `ht_shifts.shift_legacy_round_id`) — NOT re-derived in the recipe.
    /// This keeps the canonical row and the legacy row on the same id and
    /// stays consistent with the read-only `sync_round_bills` mirror
    /// (which keys `ht_shifts` on `shift_no = legacy id`). The legacy PK on
    /// `HT_Round_Bill.id` is the collision backstop against iHOTEL's own
    /// `MAX+1` allocation — a same-instant double-open hard-fails one
    /// INSERT instead of minting a duplicate round.
    OpenRound {
        /// Deterministic per-round aggregate id (see [`round_aggregate`]).
        round_aggregate_id: Uuid,
        payload: OpenRoundPayload,
    },

    /// Track J6 (round-bill coexistence step 2) — our app **closes** the
    /// open cashier round, mirroring iHOTEL's `FrmDueBill.cs:1670`:
    /// `UPDATE HT_Round_Bill SET round_end=<now>, round_by=<emp> WHERE
    /// round_end IS NULL`. We additionally pin `id=<legacy_round_id>` (we
    /// know it) so the UPDATE is precise and naturally idempotent — a
    /// second apply (or one racing iHOTEL's own close) matches 0 rows and
    /// is a no-op.
    CloseRound {
        round_aggregate_id: Uuid,
        payload: CloseRoundPayload,
    },

    /// Task #49 — deposit refund (คืนเงินมัดจำ). Mirrors iHOTEL
    /// `FormShowDEPBack.cs:536` (`docs/legacy-app/COMPAT_CHEATSHEET.md`
    /// §`HT_CheckIn_Ds` "Refund deposit", lines 466-467):
    ///
    /// ```text
    /// update HT_CheckIn_Ds
    ///    set Cin_Dep_return_date=getdate(),
    ///        Cin_Dep_Status='คืนเงินแล้ว',
    ///        Cin_Dep_return_by='<emp>'
    ///  where id=<HT_CheckIn_Ds.id>
    /// ```
    ///
    /// The service flips the canonical `ht_checkin_rooms.cr_dep_status` to the
    /// same `'คืนเงินแล้ว'` literal (+ `cr_dep_returned_at` / `cr_dep_returned_by`)
    /// inside one PG transaction before enqueuing this intent. The recipe keys
    /// the legacy UPDATE on the room's `cr_legacy_ds_id` (== `HT_CheckIn_Ds.id`),
    /// which the writeback worker back-populated when the check-in's own
    /// `CreateCheckIn` writeback landed. The dispatcher resolves it from PG by
    /// `cr_id`; when still absent (the check-in's writeback hasn't completed)
    /// the recipe no-ops with a WARN — the canonical refund already committed
    /// and the next CT round-trip reconciles iHOTEL.
    RefundDeposit {
        /// Aggregate id of the parent check-in (== `aggregate_uuid(CheckIn, …)`).
        /// Drives the `writeback_jobs.aggregate_id` index grouping and the
        /// resolver's self-heal path (same as the other check-in intents).
        check_in_id: Uuid,
        /// `ht_checkin_rooms.cr_id` of the room whose deposit is refunded. The
        /// dispatcher loads `cr_legacy_ds_id` from PG keyed on this column.
        cr_id: i64,
        /// Operator who issued the refund — lands verbatim in legacy
        /// `HT_CheckIn_Ds.Cin_Dep_return_by` (varchar(50)).
        by: String,
    },
}

/// Payload for [`WritebackIntent::OpenRound`].
///
/// Self-contained — the recipe needs no PG re-query. `legacy_round_id` is the
/// explicit `HT_Round_Bill.id` (allocated by `ShiftService::open_shift`).
/// `round_start` is the canonical `ht_shifts.shift_opened_at` (UTC); the recipe
/// converts it to a Bangkok wall-clock naive literal (the legacy column is
/// tz-naive Thai local — see CLAUDE.md "Timezone Handling").
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenRoundPayload {
    /// Site this round belongs to (`hfhotel` / `hfville`). Diagnostic only —
    /// the writeback worker already targets the correct MSSQL server per its
    /// `SITE_ID`; this makes the queued row self-describing.
    pub site_id: String,
    /// Explicit `HT_Round_Bill.id` to INSERT (== `ht_shifts.shift_legacy_round_id`).
    pub legacy_round_id: i32,
    /// `HT_Round_Bill.round_price` (float) — the opening float. iHOTEL always
    /// writes `3000`; our cashier may supply a different value.
    pub round_price: f64,
    /// `HT_Round_Bill.round_by` — the cashier opening the round.
    pub round_by: String,
    /// Canonical `shift_opened_at` (UTC). Recipe → Bangkok naive `round_start`.
    pub round_start: DateTime<Utc>,
}

/// Payload for [`WritebackIntent::CloseRound`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloseRoundPayload {
    pub site_id: String,
    /// Which `HT_Round_Bill.id` to close — pins the UPDATE to the exact row.
    pub legacy_round_id: i32,
    /// `HT_Round_Bill.round_by` re-stamped on close (mirrors iHOTEL, which
    /// overwrites `round_by` with the closing cashier).
    pub round_by: String,
    /// Canonical `shift_closed_at` (UTC). Recipe → Bangkok naive `round_end`.
    pub round_end: DateTime<Utc>,
}

/// Deterministic per-round aggregate id for [`WritebackIntent::OpenRound`] /
/// [`CloseRound`]. Derived as a v5 UUID over `"{site}:{legacy_round_id}"` so
/// that (a) open and close on the *same* round share an aggregate id (their
/// distinct `intent_name`s still yield distinct idempotency keys), and (b) a
/// different round gets a different aggregate. `ht_shifts.shift_id` is a
/// PG-only `BIGSERIAL` and not portable across the two logical DBs, so it is
/// unsuitable as the cross-site aggregate key — the `(site, legacy id)` pair
/// is the stable identity iHOTEL and our app agree on.
pub fn round_aggregate(site_id: &str, legacy_round_id: i32) -> Uuid {
    let namespace = Uuid::new_v5(&Uuid::NAMESPACE_OID, b"new-hotel.aggregate.round_bill");
    Uuid::new_v5(&namespace, format!("{site_id}:{legacy_round_id}").as_bytes())
}

/// Payload for [`WritebackIntent::CreateBooking`].
///
/// Carries everything the recipe needs to build the 4 INSERTs without
/// re-querying PG. Customer fields are denormalized (the booking writeback may
/// be the customer's first appearance in MSSQL — recipe inserts both atomically).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBookingPayload {
    pub customer_id: Uuid,
    pub legacy_cust_no: Option<String>,
    pub customer_name: String,
    pub customer_phone: Option<String>,

    pub stay: DateRange,
    pub room_no: String,
    pub room_type: String,
    pub price: Money,
    pub nights: i32,
    /// Deposit (`เงินมัดจำ` baht). Lands in `HT_Book_H.Book_Price_Pay`.
    /// Defaults to zero. Serde default keeps older queue rows
    /// (pre-2.23.0) deserializable — they'll appear with zero deposit.
    #[serde(default)]
    pub deposit: Money,

    pub created_by: String,
    pub notes: Option<String>,
}

/// Payload for [`WritebackIntent::UpdateRoom`].
///
/// Carries the writeback-relevant slice of `ht_rooms_new` AFTER the PG
/// UPDATE has committed. Every field except `room_no` is `Option<_>` —
/// `None` means "the operator did not touch this column, leave the
/// iHOTEL-side value alone" (narrow UPDATE). `room_no` is the legacy
/// `HT_Rooms.Room_no` business key used to locate the row; it is the
/// canonical `ht_rooms_new.room_no` (which we mirror 1:1).
///
/// Field → legacy column mapping (`docs/legacy-app/SCHEMA.sql` +
/// `bin/backfill_rooms.rs`):
/// * `room_type_name` → `HT_Rooms.Room_Type` (Thai literal, e.g.
///   `'ห้องแฟมิลี่ 2'`). Routes resolve this from `ht_room_types.type_name`
///   keyed on the FK `room_type_id` before enqueuing.
/// * `price_weekday`  → `Room_PriceA`
/// * `price_weekend`  → `Room_PriceB`
/// * `price_special`  → `Room_PriceC`
/// * `notes`          → `Room_Details`
///
/// Columns intentionally NOT mirrored:
/// * `Room_Use` — occupancy state, owned by `walkin` / `checkin_to_booking`
///   / `checkin_cancel` / `checkout` recipes.
/// * `Room_Clean` / `Room_Manternace` — owned by `mark_clean` (housekeeping
///   flow) and the canonical maintenance request flow respectively.
/// * `Room_Group`, `room_floor`, `room_features`, `room_view` — canonical-only
///   or vendor-managed metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRoomPayload {
    /// `HT_Rooms.Room_no` — business key of the row to UPDATE.
    /// Required; the recipe's `WHERE` clause targets this column.
    pub room_no: String,
    /// Thai literal for `HT_Rooms.Room_Type`. `None` ⇒ recipe skips the
    /// `Room_Type=` fragment (keeps the iHOTEL-side value).
    #[serde(default)]
    pub room_type_name: Option<String>,
    /// → `HT_Rooms.Room_PriceA`. `None` ⇒ skip.
    #[serde(default)]
    pub price_weekday: Option<f64>,
    /// → `HT_Rooms.Room_PriceB`. `None` ⇒ skip.
    #[serde(default)]
    pub price_weekend: Option<f64>,
    /// → `HT_Rooms.Room_PriceC`. `None` ⇒ skip.
    #[serde(default)]
    pub price_special: Option<f64>,
    /// → `HT_Rooms.Room_Details`. `None` ⇒ skip. Empty string is
    /// preferred over NULL for legacy `varchar` (the .NET WinForms
    /// controls misbehave on NULL — see spike §3k).
    #[serde(default)]
    pub notes: Option<String>,
}

/// Payload for [`WritebackIntent::CreateCheckIn`].
///
/// Discriminates between walk-in and linked-to-booking via `linked_booking_id`
/// (per spike §3a vs §3d).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCheckInPayload {
    pub customer_id: Uuid,
    pub legacy_cust_no: Option<String>,

    /// `Some(_)` for "check-in to existing booking" (§3d).
    /// `None` for walk-in (§3a) — recipe also inserts a new customer.
    pub linked_booking_id: Option<Uuid>,
    pub linked_legacy_book_id: Option<String>,

    pub room_no: String,
    pub room_type: String,
    pub stay: DateRange,

    pub price_per_night: Money,
    pub nights: i32,
    pub price_total: Money,

    pub created_by: String,
    pub guest_name_for_registry: String,
    pub guest_country: String,

    /// Customer phone number for the booking-linked check-in's
    /// `UPDATE HT_Customers SET Cust_Add_tel=…` step (Wave 5a item 1).
    /// `None` is treated as "phone unknown" and renders as the empty
    /// string in the legacy column (mirrors the prior behavior — the
    /// legacy schema rejects NULL on Cust_Add_tel so empty string is
    /// the preferred sentinel). Routes look this up from
    /// `ht_customers.cust_phone` when assembling the payload so the
    /// booking-time phone isn't wiped on every check-in.
    #[serde(default)]
    pub customer_phone: Option<String>,

    /// Room deposit (`เงินมัดจำ`, baht) collected at check-in. Lands in
    /// `HT_CheckIn_Ds.Cin_Room_Dep` and flips `Cin_dep_status` to
    /// "collected" (`'ยังไม่คืนค่ามัดจำ'`) when `> 0`; zero keeps the legacy
    /// no-deposit byte-shape (`0` + `'ไม่เก็บค่ามัดจำ'`). Defaults to zero;
    /// `serde(default)` keeps older queue rows (pre-deposit) deserializable.
    #[serde(default)]
    pub deposit: Money,

    /// Optional `Tb_Save_Image.tmp_no` for the temporary photo upload
    /// associated with this check-in. The legacy app fires
    /// `update Tb_Save_Image set cin_no=…, cust_no=…, tmp_no=''
    ///  where tmp_no=<photo_tmp_no>` on every save (UPDATE matches 0
    /// rows when no photo was uploaded). Mirroring it keeps the
    /// legacy photo-attachment flow intact. None ⇒ skip the UPDATE.
    #[serde(default)]
    pub photo_tmp_no: Option<String>,

    /// Track B4 / T2 CRIT-1 — per-room slice from the canonical
    /// `ht_checkin_rooms` junction. When non-empty, the recipe emits
    /// **one `HT_CheckIn_Ds` INSERT per `RoomLine`** (plus a single
    /// `HT_CheckIn_H` header carrying every room number in
    /// `Cin_Room_ALL`) — closing the multi-room writeback blind spot
    /// captured in `docs/coexistence/audit-2026-05-13.md` T2 CRIT-1.
    ///
    /// Empty `Vec` ⇒ fall back to the prior single-room behavior using
    /// the top-level `room_no` / `room_type` / `price_per_night` /
    /// `nights` / `price_total` fields. Routes / services that haven't
    /// migrated to the junction yet leave this empty; back-compat is
    /// preserved both at the recipe layer and at the wire (serde
    /// default).
    #[serde(default)]
    pub room_lines: Vec<RoomLine>,
}

/// Track B4 — one room slice for a multi-room check-in folio. Mirrors
/// the canonical `ht_checkin_rooms` row that the sync mapper / dashboard
/// readers already walk (Track B2 + B3). When the service layer enqueues
/// a `CreateCheckIn` intent it reads the junction and packs the rows here
/// so the writeback recipe emits one `HT_CheckIn_Ds` row per junction row.
///
/// `legacy_ds_id` is `Some(_)` for an existing `HT_CheckIn_Ds.id` (edit
/// path — UPDATE that row) and `None` for new junction rows the recipe
/// must INSERT under a freshly-allocated id. The recipe writes the
/// newly-allocated id back via `LegacyIds.checkin_ds_ids_by_room`, and
/// the worker stamps it into `ht_checkin_rooms.cr_legacy_ds_id`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomLine {
    pub room_no: String,
    pub room_type: String,
    /// Per-room rate in baht — lands in `HT_CheckIn_Ds.Cin_Room_Price`.
    pub price_per_night: f64,
    /// Nights for THIS room (may differ from the folio total when a
    /// guest extends only one of N rooms).
    pub nights: i32,
    /// Per-room total in baht — lands in `HT_CheckIn_Ds.Cin_Room_PriceToTal`.
    pub room_total: f64,
    /// Thai status literal preserved verbatim — passed straight into
    /// `Cin_Room_Status` (e.g. `'เข้าพัก'`, `'จอง'`). Empty string ⇒
    /// recipe uses the recipe-level default (`'เข้าพัก'` for active stay).
    #[serde(default)]
    pub room_status: String,
    /// Existing `HT_CheckIn_Ds.id` for the edit path — `Some(_)` ⇒ recipe
    /// emits an UPDATE; `None` ⇒ recipe INSERTs a new row under an
    /// allocator-minted id.
    #[serde(default)]
    pub legacy_ds_id: Option<i32>,
}

impl WritebackIntent {
    /// Stable string identifier for this variant — matches the `intent` discriminant
    /// produced by `serde(tag = "intent", rename_all = "snake_case")`.
    ///
    /// Persisted into `writeback_jobs.intent` so the worker can dispatch to the
    /// matching recipe without having to deserialize the whole payload.
    pub fn intent_name(&self) -> &'static str {
        match self {
            WritebackIntent::CreateBooking { .. } => "create_booking",
            WritebackIntent::ModifyBooking { .. } => "modify_booking",
            WritebackIntent::CancelBooking { .. } => "cancel_booking",
            WritebackIntent::CreateCheckIn { .. } => "create_check_in",
            WritebackIntent::CancelCheckIn { .. } => "cancel_check_in",
            WritebackIntent::ExtendStay { .. } => "extend_stay",
            WritebackIntent::CheckOut { .. } => "check_out",
            WritebackIntent::RecordPayment { .. } => "record_payment",
            WritebackIntent::RefundPayment { .. } => "refund_payment",
            WritebackIntent::RoomChange { .. } => "room_change",
            WritebackIntent::MarkRoomClean { .. } => "mark_room_clean",
            WritebackIntent::MarkRoomDirty { .. } => "mark_room_dirty",
            WritebackIntent::SetRoomMaintenance { .. } => "set_room_maintenance",
            WritebackIntent::UpdateRoom { .. } => "update_room",
            WritebackIntent::UpdateCustomer { .. } => "update_customer",
            WritebackIntent::AdjustProductStock { .. } => "adjust_product_stock",
            WritebackIntent::IssueCoupon { .. } => "issue_coupon",
            WritebackIntent::RedeemCoupon { .. } => "redeem_coupon",
            WritebackIntent::RecordPosSale { .. } => "record_pos_sale",
            WritebackIntent::OpenRound { .. } => "open_round",
            WritebackIntent::CloseRound { .. } => "close_round",
            WritebackIntent::RefundDeposit { .. } => "refund_deposit",
        }
    }

    /// The aggregate root id this intent mutates.
    ///
    /// Persisted into `writeback_jobs.aggregate_id` so jobs for the same entity
    /// can be located by the index `ix_writeback_jobs_aggregate`.
    pub fn aggregate_id(&self) -> Uuid {
        match self {
            WritebackIntent::CreateBooking { booking_id, .. }
            | WritebackIntent::ModifyBooking { booking_id, .. }
            | WritebackIntent::CancelBooking { booking_id } => *booking_id,
            WritebackIntent::CreateCheckIn { check_in_id, .. }
            | WritebackIntent::CancelCheckIn { check_in_id, .. }
            | WritebackIntent::ExtendStay { check_in_id, .. }
            | WritebackIntent::CheckOut { check_in_id, .. }
            | WritebackIntent::RecordPayment { check_in_id, .. }
            | WritebackIntent::RefundPayment { check_in_id, .. }
            | WritebackIntent::RoomChange { check_in_id, .. }
            | WritebackIntent::RecordPosSale { check_in_id, .. }
            | WritebackIntent::RefundDeposit { check_in_id, .. } => *check_in_id,
            WritebackIntent::MarkRoomClean { room_id, .. }
            | WritebackIntent::MarkRoomDirty { room_id, .. }
            | WritebackIntent::SetRoomMaintenance { room_id, .. }
            | WritebackIntent::UpdateRoom { room_id, .. } => *room_id,
            WritebackIntent::UpdateCustomer { customer_id, .. } => *customer_id,
            // Track F3 — fall back to a deterministic v5 UUID derived
            // from the legacy `Pro_no` so the
            // `writeback_jobs.aggregate_id` index always groups
            // stock-adjust intents for the same product, even if the
            // route layer hasn't yet resolved a canonical
            // `ht_products.aggregate_id`.
            WritebackIntent::AdjustProductStock {
                prod_legacy_no,
                product_aggregate_id,
                ..
            } => product_aggregate_id
                .unwrap_or_else(|| product_aggregate_fallback(prod_legacy_no)),
            WritebackIntent::IssueCoupon { coupon_aggregate_id, .. }
            | WritebackIntent::RedeemCoupon { coupon_aggregate_id, .. } => *coupon_aggregate_id,
            WritebackIntent::OpenRound { round_aggregate_id, .. }
            | WritebackIntent::CloseRound { round_aggregate_id, .. } => *round_aggregate_id,
        }
    }
}

/// Deterministic aggregate-id fallback for [`WritebackIntent::AdjustProductStock`]
/// when the route layer hasn't supplied a canonical
/// `ht_products.aggregate_id`. Uses a v5 UUID under a stable namespace
/// so repeated calls return the same id and the `writeback_jobs`
/// `aggregate_id` index groups stock-adjust intents for the same
/// product. `service::ids::aggregate_uuid(AggregateKind::Product, …)`
/// is the preferred path; this fallback is defensive for payloads
/// enqueued before the route resolves the canonical id.
fn product_aggregate_fallback(prod_legacy_no: &str) -> Uuid {
    let namespace = Uuid::new_v5(
        &Uuid::NAMESPACE_OID,
        b"new-hotel.aggregate.product.legacy_no",
    );
    Uuid::new_v5(&namespace, prod_legacy_no.as_bytes())
}

/// Default `stay_start` used when an [`WritebackIntent::ExtendStay`] payload
/// from a previous schema version (pre-fix-#16) is deserialized — falls back
/// to `Utc::now()` so the recipe degrades gracefully (single-night re-insert
/// rather than a panic). New emitters always populate the field explicitly.
fn default_extend_stay_start() -> DateTime<Utc> {
    Utc::now()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Audit H1 back-compat: in-flight CheckOut events queued before the
    /// Wave 2 deploy carry only `check_in_id` — the new totals fields must
    /// deserialize as `None` so the queue drains without manual
    /// intervention. The dispatcher then logs a WARN and falls back to the
    /// prior all-zeros behavior.
    #[test]
    fn check_out_intent_deserializes_pre_wave2_payload_without_totals() {
        let legacy_json = r#"{"intent":"check_out","payload":{"check_in_id":"550e8400-e29b-41d4-a716-446655440000"}}"#;
        let intent: WritebackIntent = serde_json::from_str(legacy_json)
            .expect("pre-Wave-2 CheckOut payload must still deserialize");
        match intent {
            WritebackIntent::CheckOut {
                check_in_id,
                nights,
                room_price_total,
                product_total,
                net_total,
                pay_total,
                balance,
            } => {
                assert_eq!(
                    check_in_id,
                    Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap()
                );
                assert!(nights.is_none(), "nights must default to None");
                assert!(room_price_total.is_none());
                assert!(product_total.is_none());
                assert!(net_total.is_none());
                assert!(pay_total.is_none());
                assert!(balance.is_none());
            }
            other => panic!("expected CheckOut variant, got {other:?}"),
        }
    }

    /// Track B4 — pre-B4 outbox events carry no `room_lines` field. The
    /// `#[serde(default)]` annotation must keep them deserializable so the
    /// queue drains without manual replay after deploy; the recipe then
    /// falls back to the legacy single-room shape via the
    /// `effective_room_lines` synthesis.
    #[test]
    fn create_check_in_payload_deserializes_pre_b4_without_room_lines() {
        let legacy_json = r#"{
            "customer_id":"550e8400-e29b-41d4-a716-446655440000",
            "legacy_cust_no":"C21607",
            "linked_booking_id":null,
            "linked_legacy_book_id":null,
            "room_no":"402",
            "room_type":"Standard",
            "stay":{"start":"2026-04-24T09:56:20Z","end":"2026-04-25T11:59:59Z"},
            "price_per_night":89000,
            "nights":1,
            "price_total":89000,
            "created_by":"Admin",
            "guest_name_for_registry":"SPIKE TEST",
            "guest_country":""
        }"#;
        let payload: CreateCheckInPayload = serde_json::from_str(legacy_json)
            .expect("pre-B4 CreateCheckIn payload must still deserialize");
        assert!(
            payload.room_lines.is_empty(),
            "missing room_lines must default to empty vec"
        );
        assert_eq!(payload.room_no, "402");
    }

    /// Track B4 — round-trip a multi-room payload. The `room_lines` field
    /// serializes/deserializes as a top-level JSON array of structured
    /// objects (mirrors `ht_checkin_rooms`).
    #[test]
    fn create_check_in_payload_roundtrips_multi_room_slice() {
        use chrono::TimeZone;
        let payload = CreateCheckInPayload {
            customer_id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap(),
            legacy_cust_no: Some("C21607".into()),
            linked_booking_id: None,
            linked_legacy_book_id: None,
            room_no: "508".into(),
            room_type: "Standard".into(),
            stay: crate::domain::shared::DateRange {
                start: chrono::Utc.with_ymd_and_hms(2026, 4, 24, 3, 1, 11).unwrap(),
                end: chrono::Utc.with_ymd_and_hms(2026, 4, 25, 4, 59, 59).unwrap(),
            },
            price_per_night: Money::from_satang(89000),
            nights: 1,
            price_total: Money::from_satang(178000),
            created_by: "Admin".into(),
            guest_name_for_registry: "MULTI ROOM GUEST".into(),
            guest_country: "".into(),
            customer_phone: None,
            deposit: Money::ZERO,
            photo_tmp_no: None,
            room_lines: vec![
                RoomLine {
                    room_no: "508".into(),
                    room_type: "Standard".into(),
                    price_per_night: 890.0,
                    nights: 1,
                    room_total: 890.0,
                    room_status: String::new(),
                    legacy_ds_id: None,
                },
                RoomLine {
                    room_no: "509".into(),
                    room_type: "Standard".into(),
                    price_per_night: 890.0,
                    nights: 1,
                    room_total: 890.0,
                    room_status: String::new(),
                    legacy_ds_id: Some(25101),
                },
            ],
        };
        let json = serde_json::to_string(&payload).expect("must serialize");
        let parsed: CreateCheckInPayload =
            serde_json::from_str(&json).expect("must round-trip");
        assert_eq!(parsed.room_lines.len(), 2);
        assert_eq!(parsed.room_lines[0].room_no, "508");
        assert_eq!(parsed.room_lines[1].room_no, "509");
        assert_eq!(parsed.room_lines[1].legacy_ds_id, Some(25101));
    }

    /// Wave-2 emissions carry the full totals payload so the recipe writes
    /// real numbers to MSSQL.
    #[test]
    fn check_out_intent_roundtrips_with_totals_payload() {
        let intent = WritebackIntent::CheckOut {
            check_in_id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap(),
            nights: Some(3.0),
            room_price_total: Some(2670.0),
            product_total: Some(0.0),
            net_total: Some(2670.0),
            pay_total: Some(2670.0),
            balance: Some(0.0),
        };
        let json = serde_json::to_string(&intent).expect("must serialize");
        let parsed: WritebackIntent =
            serde_json::from_str(&json).expect("must round-trip");
        match parsed {
            WritebackIntent::CheckOut {
                nights,
                room_price_total,
                pay_total,
                ..
            } => {
                assert_eq!(nights, Some(3.0));
                assert_eq!(room_price_total, Some(2670.0));
                assert_eq!(pay_total, Some(2670.0));
            }
            other => panic!("expected CheckOut variant, got {other:?}"),
        }
    }

    /// `UpdateRoom` must expose the snake_case discriminant
    /// `"update_room"` and report the carried room aggregate id so the
    /// writeback worker indexes / back-populates against the same row.
    #[test]
    fn update_room_intent_name_and_aggregate_id() {
        let room_id = Uuid::parse_str("11111111-2222-3333-4444-555555555555").unwrap();
        let intent = WritebackIntent::UpdateRoom {
            room_id,
            payload: UpdateRoomPayload {
                room_no: "A2-1".into(),
                room_type_name: Some("ห้องแฟมิลี่ 2".into()),
                price_weekday: Some(890.0),
                price_weekend: None,
                price_special: None,
                notes: None,
            },
        };
        assert_eq!(intent.intent_name(), "update_room");
        assert_eq!(intent.aggregate_id(), room_id);
    }

    /// Idempotency key is derived from `(intent_name, aggregate_id)` only —
    /// two `UpdateRoom` intents for the same room collapse to the same key
    /// regardless of which fields the operator changed, which is exactly
    /// what we want: the writeback worker should never queue two parallel
    /// `HT_Rooms` UPDATEs against the same row. The route's PG UPDATE +
    /// outbox enqueue commit atomically, so the "latest commit wins"
    /// invariant is preserved.
    #[test]
    fn update_room_idempotency_key_is_deterministic_for_same_room() {
        use crate::outbox::generate_idempotency_key;
        let room_id = Uuid::parse_str("11111111-2222-3333-4444-555555555555").unwrap();
        let intent_a = WritebackIntent::UpdateRoom {
            room_id,
            payload: UpdateRoomPayload {
                room_no: "A2-1".into(),
                room_type_name: Some("ห้องแฟมิลี่ 2".into()),
                price_weekday: Some(890.0),
                price_weekend: None,
                price_special: None,
                notes: None,
            },
        };
        let intent_b = WritebackIntent::UpdateRoom {
            room_id,
            // Different payload (operator-touched fields differ) — but the
            // key is payload-independent so it must still match.
            payload: UpdateRoomPayload {
                room_no: "A2-1".into(),
                room_type_name: None,
                price_weekday: None,
                price_weekend: Some(1090.0),
                price_special: None,
                notes: Some("renovated 2026-05".into()),
            },
        };
        assert_eq!(
            generate_idempotency_key(&intent_a, room_id),
            generate_idempotency_key(&intent_b, room_id),
        );
    }

    /// `MarkRoomDirty` must expose the snake_case discriminant
    /// `"mark_room_dirty"` and report the carried room aggregate id —
    /// mirrors `MarkRoomClean` exactly.
    #[test]
    fn mark_room_dirty_intent_name_and_aggregate_id() {
        let room_id = Uuid::parse_str("11111111-2222-3333-4444-555555555555").unwrap();
        let intent = WritebackIntent::MarkRoomDirty {
            room_id,
            by: "Admin".into(),
        };
        assert_eq!(intent.intent_name(), "mark_room_dirty");
        assert_eq!(intent.aggregate_id(), room_id);
    }

    /// `SetRoomMaintenance` must expose the snake_case discriminant
    /// `"set_room_maintenance"`, report the room aggregate id, and
    /// round-trip the boolean through serde in both directions.
    #[test]
    fn set_room_maintenance_intent_roundtrips_both_directions() {
        let room_id = Uuid::parse_str("11111111-2222-3333-4444-555555555555").unwrap();
        for maintenance in [true, false] {
            let intent = WritebackIntent::SetRoomMaintenance { room_id, maintenance };
            assert_eq!(intent.intent_name(), "set_room_maintenance");
            assert_eq!(intent.aggregate_id(), room_id);
            let json = serde_json::to_string(&intent).expect("must serialize");
            let parsed: WritebackIntent =
                serde_json::from_str(&json).expect("must round-trip");
            match parsed {
                WritebackIntent::SetRoomMaintenance { room_id: r, maintenance: m } => {
                    assert_eq!(r, room_id);
                    assert_eq!(m, maintenance);
                }
                other => panic!("expected SetRoomMaintenance, got {other:?}"),
            }
        }
    }

    /// `UpdateCustomer` must expose the snake_case discriminant
    /// `"update_customer"`, report the customer aggregate id, and carry
    /// the full `CustomerResave` payload through serde unchanged.
    #[test]
    fn update_customer_intent_roundtrips_resave_payload() {
        let customer_id = Uuid::parse_str("66666666-7777-8888-9999-aaaaaaaaaaaa").unwrap();
        let intent = WritebackIntent::UpdateCustomer {
            customer_id,
            resave: CustomerResave {
                legacy_cust_no: "C21624".into(),
                cust_name: "Alberto Calvo Alvarez".into(),
                cust_add_tel: "0900000088".into(),
                cust_idcard: "1234567890123".into(),
                ..CustomerResave::default()
            },
        };
        assert_eq!(intent.intent_name(), "update_customer");
        assert_eq!(intent.aggregate_id(), customer_id);
        let json = serde_json::to_string(&intent).expect("must serialize");
        let parsed: WritebackIntent =
            serde_json::from_str(&json).expect("must round-trip");
        match parsed {
            WritebackIntent::UpdateCustomer { customer_id: c, resave } => {
                assert_eq!(c, customer_id);
                assert_eq!(resave.legacy_cust_no, "C21624");
                assert_eq!(resave.cust_name, "Alberto Calvo Alvarez");
                assert_eq!(resave.cust_add_tel, "0900000088");
                assert_eq!(resave.cust_idcard, "1234567890123");
                // Untouched fields stay empty strings (legacy '' convention).
                assert_eq!(resave.cust_work_name, "");
            }
            other => panic!("expected UpdateCustomer, got {other:?}"),
        }
    }

    /// Round-trip an `UpdateRoom` payload through serde — partial
    /// payloads (only price_weekday set) must serialize / deserialize
    /// without the `#[serde(default)]` annotations dropping the explicit
    /// `Some(_)` value.
    #[test]
    fn update_room_payload_roundtrips_partial_update() {
        let payload = UpdateRoomPayload {
            room_no: "A2-1".into(),
            room_type_name: None,
            price_weekday: Some(890.0),
            price_weekend: None,
            price_special: None,
            notes: None,
        };
        let json = serde_json::to_string(&payload).expect("must serialize");
        let parsed: UpdateRoomPayload =
            serde_json::from_str(&json).expect("must round-trip");
        assert_eq!(parsed.room_no, "A2-1");
        assert_eq!(parsed.price_weekday, Some(890.0));
        assert!(parsed.room_type_name.is_none());
        assert!(parsed.price_weekend.is_none());
        assert!(parsed.notes.is_none());
    }

    /// Task #49 — `RefundDeposit` must expose the snake_case discriminant
    /// `"refund_deposit"`, report the parent check-in aggregate id (so the
    /// resolver's check-in self-heal path applies), and round-trip its
    /// `cr_id` / `by` payload through serde unchanged.
    #[test]
    fn refund_deposit_intent_name_aggregate_id_and_roundtrip() {
        let check_in_id = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let intent = WritebackIntent::RefundDeposit {
            check_in_id,
            cr_id: 4231,
            by: "Front Desk".into(),
        };
        assert_eq!(intent.intent_name(), "refund_deposit");
        assert_eq!(intent.aggregate_id(), check_in_id);
        let json = serde_json::to_string(&intent).expect("must serialize");
        let parsed: WritebackIntent =
            serde_json::from_str(&json).expect("must round-trip");
        match parsed {
            WritebackIntent::RefundDeposit { check_in_id: c, cr_id, by } => {
                assert_eq!(c, check_in_id);
                assert_eq!(cr_id, 4231);
                assert_eq!(by, "Front Desk");
            }
            other => panic!("expected RefundDeposit, got {other:?}"),
        }
    }
}

/// Receipt-header fields carried by [`WritebackIntent::RecordPayment`].
///
/// The `payment` recipe (spike §3h) copies these straight into `HT_Receipt_H`:
/// `Receipt_Name`, `Receipt_Address`, `Receipt_Tel`. Empty strings are
/// preferred over `NULL` per the legacy schema (NULL crashes WinForms
/// downstream — see the same convention in `booking_create` recipe).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RecordPaymentReceipt {
    pub customer_name: String,
    pub customer_address: String,
    pub customer_tel: String,
}

/// Diff payload for [`WritebackIntent::ModifyBooking`].
///
/// Only the fields a user actually edits round-trip through here — fields
/// retained as `None` are left untouched. The writeback recipe applies these
/// as targeted UPDATEs (not the legacy app's DELETE-then-REINSERT).
///
/// `customer_resave` mirrors spike §3c lines 5/16/28: the .NET app re-saves
/// the customer record on every booking modify. Phone/address edits never
/// propagate to the customer master without it. When `Some(_)` the recipe
/// emits an `UPDATE HT_Customers SET …` with the full field set.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookingChanges {
    pub new_stay: Option<DateRange>,
    pub new_room_no: Option<String>,
    pub new_room_type: Option<String>,
    pub new_price: Option<Money>,
    pub new_state: Option<BookingState>,
    pub new_notes: Option<String>,
    pub new_customer_phone: Option<String>,
    /// Customer display name — used by the recipe to re-write the
    /// `HT_Rooms.Room_Book_Name` / `room_book_ds` display caption when a
    /// modify changes any caption-relevant field (date / room / notes).
    /// Not the same as `customer_resave.cust_name`; populated separately so
    /// the caption rewrite still works even when the route doesn't have the
    /// full re-save payload.
    #[serde(default)]
    pub new_customer_name: Option<String>,
    /// Full customer profile re-save (spike §3c). `None` skips the UPDATE.
    /// Set on every modify so the legacy app's "phone edit triggers customer
    /// master sync" expectation holds.
    #[serde(default)]
    pub customer_resave: Option<CustomerResave>,
}

/// Full customer-record re-save payload for [`BookingChanges::customer_resave`].
///
/// Fields mirror the .NET app's UPDATE in spike §3c capture line 28 — the
/// recipe writes them all so the legacy customer record reflects the latest
/// values from PG. Empty strings are preferred over NULL (NULL crashes the
/// .NET WinForms downstream — see `booking_create` recipe).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CustomerResave {
    /// `HT_Customers.Cust_no` of the row to update.
    pub legacy_cust_no: String,
    pub cust_name: String,
    pub cust_name2: String,
    pub cust_type: String,
    pub cust_type_main: String,
    pub cust_email: String,
    pub cust_add_no: String,
    pub cust_add_moo: String,
    pub cust_add_soi: String,
    pub cust_add_road: String,
    pub cust_add_tambon: String,
    pub cust_add_ampore: String,
    pub cust_add_province: String,
    pub cust_add_code: String,
    pub cust_add_tel: String,
    pub cust_add_fax: String,
    pub cust_work_name: String,
    pub cust_work_no: String,
    pub cust_work_moo: String,
    pub cust_work_soi: String,
    pub cust_work_road: String,
    pub cust_work_tambon: String,
    pub cust_work_ampore: String,
    pub cust_work_province: String,
    pub cust_work_code: String,
    pub cust_work_tel: String,
    pub cust_work_fax: String,
    /// `Cust_Work_tax` — work tax ID. Default empty string.
    #[serde(default)]
    pub cust_work_tax: String,
    /// `Cust_perfix` — personal title (Mr., Mrs., นาย, น.ส., etc.).
    /// Default empty string.
    #[serde(default)]
    pub cust_perfix: String,
    /// `Cust_sex` — Thai gender literal (e.g. `'ชาย'` for male).
    /// Default empty string.
    #[serde(default)]
    pub cust_sex: String,
    /// `Cust_IDcard` — national ID / passport number. Default empty string.
    #[serde(default)]
    pub cust_idcard: String,
    /// `Cust_Contry` — country (note legacy spelling: "Contry").
    /// Default empty string.
    #[serde(default)]
    pub cust_contry: String,
}
