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

    /// Optional `Tb_Save_Image.tmp_no` for the temporary photo upload
    /// associated with this check-in. The legacy app fires
    /// `update Tb_Save_Image set cin_no=…, cust_no=…, tmp_no=''
    ///  where tmp_no=<photo_tmp_no>` on every save (UPDATE matches 0
    /// rows when no photo was uploaded). Mirroring it keeps the
    /// legacy photo-attachment flow intact. None ⇒ skip the UPDATE.
    #[serde(default)]
    pub photo_tmp_no: Option<String>,
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
            WritebackIntent::MarkRoomClean { .. } => "mark_room_clean",
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
            | WritebackIntent::RecordPayment { check_in_id, .. } => *check_in_id,
            WritebackIntent::MarkRoomClean { room_id, .. } => *room_id,
        }
    }
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
