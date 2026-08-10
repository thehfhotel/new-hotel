//! `legacy_stale` — ephemeral "iHOTEL's room grid is now stale" hint.
//!
//! Per [`docs/adr/0006-legacy-stale-notification.md`] and the decompile
//! findings in `docs/legacy-app/ROOM_GRID_REFRESH.md`: when one of our
//! writeback jobs commits a row into the shared legacy MSSQL, iHOTEL's
//! `FormRoomMain` does not know to re-read it. Its own 60.56 s refresh timer is
//! gated on `!MSSQL.CodeErr`, which goes `true` the moment the form loses focus
//! — so while reception is working in OUR app (exactly when a writeback fires)
//! iHOTEL's grid never auto-refreshes at all, for any duration. This channel is
//! how reception is told, so she can refresh on her own terms.
//!
//! ## Two names this deliberately does NOT use
//!
//! **Not [`crate::outbox::bus::EventBus`] / `event_log`.** That path writes a
//! durable, indexed, 30-day-retained row per publish, and it exists to record
//! *domain* facts. This signal is neither: it is worthless the instant it is
//! stale or nobody is listening (nothing replays a "your other app's screen was
//! out of date 40 minutes ago" hint), and "a row now exists in legacy MSSQL" is
//! an **adapter-level** fact produced by `bin/writeback.rs` — outside the
//! decommission boundary. Putting it in `event_log` would mislabel it as a
//! domain event for every future reader of that table. A plain `pg_notify` with
//! no durable row is the right shape, and it already has a precedent here:
//! `migrations/pg/016_writeback_notify_trigger.sql`.
//!
//! **Not named `refresh`.** `routes/events.rs` already defines
//! `RESYNC_EVENT = "refresh"`, meaning "refetch OUR OWN UI's data". It fires on
//! SSE listener reconnects and on lagged subscribers — moments that have
//! nothing to do with a writeback landing in legacy. Reusing the string would
//! make two structurally different signals indistinguishable on the wire, and
//! would fabricate a "iHOTEL is stale" toast every time a browser tab
//! reconnected.
//!
//! ## Payload budget (hard constraint)
//!
//! PostgreSQL caps a `NOTIFY` payload at just under 8000 bytes and raises an
//! error past it — which, on this path, would be an error raised AFTER the
//! MSSQL transaction already committed. So the clamp is applied at
//! CONSTRUCTION, not at send time: [`LegacyStaleSignal::new`] truncates
//! `items` to [`MAX_ITEMS`] entries of [`MAX_ITEM_CHARS`] characters each and
//! `summary` to [`MAX_SUMMARY_CHARS`], char-wise (never byte-wise — every
//! label here is Thai). The signal's fields are private so nothing WE
//! construct can bypass the clamp — every path we build a signal through goes
//! via `new()` or [`coalesce`] (which itself calls `new()`). This does NOT
//! cover `#[derive(Deserialize)]` (below): `routes/events.rs` deserializes an
//! inbound `legacy_stale` notification on every delivery, and that path is
//! not clamped — it forwards the already-published (and therefore
//! already-clamped) payload verbatim without re-serializing, so no unclamped
//! signal is ever published, but the clamp itself is not a type-level
//! invariant.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use super::intent::{NoteTargetKind, WritebackIntent};

/// PG `NOTIFY` channel this signal travels on. Shared by the publisher
/// (`bin/writeback.rs`) and the subscriber (`routes/events.rs`) so the two
/// can never drift apart.
pub const LEGACY_STALE_CHANNEL: &str = "legacy_stale";

/// SSE `event:` name the signal is forwarded to browsers under. Same string
/// as the channel on purpose — one concept, one name, greppable end-to-end.
///
/// Deliberately NOT a member of `routes/events.rs`'s `RESYNC_FANOUT_EVENTS`:
/// a lagged subscriber or a listener reconnect must never fabricate a
/// "iHOTEL is stale" toast out of nothing.
pub const LEGACY_STALE_EVENT: &str = "legacy_stale";

/// Maximum enumerated items in one signal. Anything beyond is represented by
/// `count` alone — the toast has room for a headline, not a list.
pub const MAX_ITEMS: usize = 3;

/// Per-item character cap (chars, not bytes — Thai is 3 bytes/char in UTF-8).
pub const MAX_ITEM_CHARS: usize = 80;

/// Summary character cap.
pub const MAX_SUMMARY_CHARS: usize = 200;

/// Above this many notes in one drain tick, stop enumerating and emit a
/// generic count summary. A burst that big is a backlog drain, not something
/// a receptionist can read item-by-item.
pub const MAX_ITEMISED_NOTES: usize = 20;

/// PostgreSQL's `NOTIFY` payload ceiling. The real limit is 8000 bytes; we
/// treat it as a hard budget and never get anywhere near it (see the module
/// doc's clamp).
pub const MAX_NOTIFY_PAYLOAD_BYTES: usize = 8000;

/// One writeback that landed in legacy MSSQL, reduced to what reception needs
/// to be told. Built in `bin/writeback.rs` right after a successful
/// `mark_done`, accumulated for the duration of one drain tick, then folded
/// into a single [`LegacyStaleSignal`] by [`coalesce`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StaleNote {
    intent: &'static str,
    label: String,
}

impl StaleNote {
    /// Build a note from the intent that just landed plus whatever room
    /// number the worker already had in hand (see [`stale_label`]).
    pub fn for_intent(intent: &WritebackIntent, room_no: Option<&str>) -> Self {
        Self {
            intent: intent.intent_name(),
            label: stale_label(intent, room_no),
        }
    }

    /// `WritebackIntent::intent_name` of the job this note came from —
    /// diagnostic only (log lines), never shown to reception.
    pub fn intent(&self) -> &'static str {
        self.intent
    }

    /// The Thai, iHOTEL-anchored one-liner.
    pub fn label(&self) -> &str {
        &self.label
    }
}

/// The wire payload of a `legacy_stale` notification.
///
/// Fields are private — see the module doc's payload-budget section. Construct
/// with [`LegacyStaleSignal::new`] (which clamps) or [`coalesce`]; read with
/// the accessors below.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LegacyStaleSignal {
    /// Per-signal id so a middleware receiving the same signal twice (browser
    /// reconnect replay, two tabs open) can deduplicate.
    id: Uuid,
    /// Which site's legacy DB went stale (`hfhotel` / `hfville`). Diagnostic:
    /// the subscriber tags the broadcast from the database the notification
    /// arrived on, not from this field.
    site: String,
    /// How many writebacks landed in this drain tick. May exceed
    /// `items.len()` — that is the normal case for a burst.
    count: u32,
    /// One-line headline for the toast.
    summary: String,
    /// Up to [`MAX_ITEMS`] deduplicated labels, in the order they landed.
    items: Vec<String>,
    emitted_at: DateTime<Utc>,
}

impl LegacyStaleSignal {
    /// Construct a signal, clamping `summary` and `items` to the documented
    /// payload budget. This is the ONLY constructor — the clamp cannot be
    /// bypassed from outside this module.
    pub fn new(
        site: impl Into<String>,
        count: u32,
        summary: impl Into<String>,
        items: Vec<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            site: site.into(),
            count,
            summary: clamp_chars(summary.into(), MAX_SUMMARY_CHARS),
            items: items
                .into_iter()
                .take(MAX_ITEMS)
                .map(|item| clamp_chars(item, MAX_ITEM_CHARS))
                .collect(),
            emitted_at: Utc::now(),
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
    pub fn site(&self) -> &str {
        &self.site
    }
    pub fn count(&self) -> u32 {
        self.count
    }
    pub fn summary(&self) -> &str {
        &self.summary
    }
    pub fn items(&self) -> &[String] {
        &self.items
    }
    pub fn emitted_at(&self) -> DateTime<Utc> {
        self.emitted_at
    }
}

/// Fold one drain tick's worth of notes into a single signal.
///
/// **Why one signal per drain tick and not one per job.** The writeback
/// worker's inner loop already drains until `claim_next_job` returns
/// `Ok(None)`, so the tick boundary IS the burst boundary — no timer, no
/// buffering window, no extra state. A booking that emits 3 intents enqueues
/// all 3 rows in one PG transaction, the notify trigger wakes the worker once,
/// and all 3 drain in the same tick ⇒ one signal with `count: 3`. A slow
/// trickle of unrelated edits yields one signal each, which is correct — those
/// ARE separate events, and the middleware's latch (ADR 0006 §5) is what stops
/// them becoming separate toasts.
///
/// `count` is the number of jobs that landed; `items` is the DEDUPLICATED
/// label list capped at [`MAX_ITEMS`]. Three payments against room 302 are
/// three landings but one thing worth reading.
pub fn coalesce(site: &str, notes: &[StaleNote]) -> LegacyStaleSignal {
    debug_assert!(
        !notes.is_empty(),
        "coalesce called with no notes — the caller must skip publishing instead"
    );

    let count = notes.len();

    // Dedupe by label, preserving first-seen order. `seen` gives O(1)
    // membership checks — `notes` is one drain tick, but a post-outage
    // backlog can be thousands of jobs, and a `Vec::contains` scan there is
    // O(notes * distinct_labels). `labels` stays the insertion-ordered list
    // the summary and `items` are built from.
    let mut seen: std::collections::HashSet<&str> = std::collections::HashSet::new();
    let mut labels: Vec<&str> = Vec::new();
    for note in notes {
        if seen.insert(note.label()) {
            labels.push(note.label());
        }
    }

    let items: Vec<String> = labels
        .iter()
        .take(MAX_ITEMS)
        .map(|label| (*label).to_string())
        .collect();

    let summary = if count > MAX_ITEMISED_NOTES {
        format!("มีการเปลี่ยนแปลง {count} รายการ")
    } else {
        let mut text = items.join(", ");
        let hidden = labels.len().saturating_sub(items.len());
        if hidden > 0 {
            text.push_str(&format!(" และอีก {hidden} รายการ"));
        }
        text
    };

    LegacyStaleSignal::new(site, count as u32, summary, items)
}

/// Emit one `legacy_stale` notification.
///
/// **Autocommit on purpose.** Everywhere else in `outbox/` a publish joins the
/// caller's transaction so the notification and the canonical write commit
/// together. Here there is nothing to be atomic with: the MSSQL COMMIT this
/// signal describes already happened, on a different server, in a transaction
/// PostgreSQL knows nothing about. Wrapping this in a PG transaction would buy
/// nothing and could only add a failure mode.
///
/// Errors are returned, never panicked — the caller's contract is to log and
/// carry on. A job whose legacy write committed must never be failed or
/// retried because a best-effort hint could not be delivered.
pub async fn publish(pg: &PgPool, signal: &LegacyStaleSignal) -> Result<(), sqlx::Error> {
    let payload = serde_json::to_string(signal).map_err(|e| {
        sqlx::Error::Encode(format!("LegacyStaleSignal serialization failed: {e}").into())
    })?;

    // Belt-and-braces: the clamp in `LegacyStaleSignal::new` already keeps us
    // an order of magnitude under this. If it is ever breached, dropping the
    // hint is strictly better than letting PG raise on a signal nobody can
    // act on anyway.
    if payload.len() >= MAX_NOTIFY_PAYLOAD_BYTES {
        tracing::warn!(
            bytes = payload.len(),
            limit = MAX_NOTIFY_PAYLOAD_BYTES,
            "legacy_stale payload exceeds the pg_notify budget; dropping the hint"
        );
        return Ok(());
    }

    // Dynamic `sqlx::query()` (not `query!`) — same rationale as `queue.rs`:
    // keeps the crate buildable without a live PG at compile time.
    sqlx::query("SELECT pg_notify($1, $2)")
        .bind(LEGACY_STALE_CHANNEL)
        .bind(&payload)
        .execute(pg)
        .await?;

    Ok(())
}

/// What reception is told about one landed writeback, in Thai.
///
/// **Exhaustive match, no `_` arm — on purpose.** Adding a
/// [`WritebackIntent`] variant should not silently inherit some generic
/// "something changed" string: it should fail to compile until somebody
/// decides what the receptionist is told. That decision is cheap here and
/// expensive to notice in production.
///
/// **No new DB query.** `room_no` is whatever the worker already had in hand
/// at `mark_done` time — `LegacyIds::room_no` (minted by the walk-in /
/// checkin-to-booking recipes) falling back to `ResolvedJob::legacy_room_no`
/// (resolved before dispatch for every room-scoped intent). Intents whose
/// payload carries the room directly read it from there instead. Anything
/// genuinely room-less degrades to a bare verb.
///
/// Vocabulary is iHOTEL's own wherever iHOTEL has one (ADR 0003's
/// iHOTEL-anchoring). `MarkRoomDirty`'s label reuses the literal tile text
/// from `docs/legacy-app/ROOM_STATUS_PALETTE.md:12` ("รอ ทำความสะอาด") so the
/// toast and the board a receptionist looks at next say the same words.
/// `MarkRoomClean`'s label ("ทำความสะอาดเรียบร้อย") is NOT that file's
/// vacant-clean tile text (which is "ว่าง") — it's the phrase iHOTEL itself
/// logs in its cleaning-done button handler, `docs/legacy-app/
/// COMPAT_CHEATSHEET.md:1391`. `SetRoomMaintenance`'s two labels
/// ("แจ้งซ่อม" / "ยกเลิกแจ้งซ่อม") have no `docs/legacy-app/` hit at all
/// (iHOTEL's maintenance tile just says "ซ่อม") — those are our own wording,
/// not a decompile citation.
pub fn stale_label(intent: &WritebackIntent, room_no: Option<&str>) -> String {
    use WritebackIntent::*;

    // Normalize ONCE at the door: `ResolvedJob::legacy_room_no` is read
    // straight out of PG and is `Some("")` for a room whose legacy back-link
    // was never populated. Without this, every room-scoped arm below would
    // emit a dangling `"เช็คเอาต์ ห้อง "`.
    let room_no = room_or(room_no);

    match intent {
        // --- Booking ---
        CreateBooking { payload, .. } => with_room("จองห้องพัก", room_or(Some(&payload.room_no))),
        ModifyBooking { changes, .. } => with_room(
            "แก้ไขการจอง",
            room_or(changes.new_room_no.as_deref()).or(room_no),
        ),
        CancelBooking { .. } => "ยกเลิกการจอง".to_string(),

        // --- Stay lifecycle ---
        CreateCheckIn { payload, .. } => {
            with_room("เช็คอิน", room_or(Some(&payload.room_no)).or(room_no))
        }
        CancelCheckIn { .. } => with_room("ยกเลิกเช็คอิน", room_no),
        ExtendStay { .. } => with_room("ต่อเวลาเข้าพัก", room_no),
        CheckOut { .. } => with_room("เช็คเอาต์", room_no),
        RoomChange { .. } => with_room("ย้ายห้องพัก", room_no),

        // --- Money ---
        RecordPayment { .. } => with_room("รับชำระเงิน", room_no),
        RefundPayment { .. } => with_room("คืนเงิน", room_no),
        RefundDeposit { .. } => with_room("คืนเงินมัดจำ", room_no),
        OpenRound { .. } => "เปิดรอบบิล".to_string(),
        CloseRound { .. } => "ปิดรอบบิล".to_string(),

        // --- Housekeeping / room master data ---
        // MarkRoomDirty reuses iHOTEL's own tile text (ROOM_STATUS_PALETTE.md:12)
        // so the toast matches the board reception looks at next. MarkRoomClean
        // is iHOTEL's cleaning-done button-log phrase (COMPAT_CHEATSHEET.md:1391),
        // not the palette's vacant-clean tile text (which is "ว่าง").
        MarkRoomClean { .. } => room_prefixed(room_no, "ทำความสะอาดเรียบร้อย"),
        MarkRoomDirty { .. } => room_prefixed(room_no, "รอ ทำความสะอาด"),
        SetRoomMaintenance { maintenance, .. } => room_prefixed(
            room_no,
            if *maintenance {
                "แจ้งซ่อม"
            } else {
                "ยกเลิกแจ้งซ่อม"
            },
        ),
        UpdateRoom { payload, .. } => {
            with_room("แก้ไขข้อมูลห้องพัก", room_or(Some(&payload.room_no)))
        }
        MoveRoomTiles { .. } => "จัดผังห้องพัก".to_string(),
        UpsertRatePrice { .. } => "แก้ไขราคาห้องพัก".to_string(),

        // --- Customer / guest registry ---
        UpdateCustomer { .. } => "แก้ไขข้อมูลลูกค้า".to_string(),
        MirrorGuestImage { .. } => "บันทึกรูปเอกสารผู้เข้าพัก".to_string(),
        MirrorCompanion { .. } | CompanionAdd { .. } => "เพิ่มผู้ติดตาม".to_string(),
        CompanionDelete { .. } => "ลบผู้ติดตาม".to_string(),
        MirrorCompanionList { .. } => "ปรับปรุงรายชื่อผู้ติดตาม".to_string(),

        // --- Products / POS / coupons ---
        AdjustProductStock { .. } => "ปรับสต๊อกสินค้า".to_string(),
        RecordPosSale { .. } => with_room("ขายสินค้า", room_no),
        VoidPosSale { .. } => with_room("ยกเลิกรายการขาย", room_no),
        RecordReceipt { .. } => "ขายสินค้าหน้าร้าน".to_string(),
        IssueCoupon { .. } => "ออกคูปองอาหาร".to_string(),
        RedeemCoupon { .. } => "ใช้คูปองอาหาร".to_string(),

        // --- Sticky notes ---
        CreateNote {
            target_kind,
            target_key,
            ..
        } => match target_kind {
            NoteTargetKind::Room => room_prefixed(room_or(Some(target_key)), "มีข้อความใหม่"),
            NoteTargetKind::Staff => "ข้อความถึงพนักงาน".to_string(),
        },
        MarkNoteRead { .. } => "อ่านข้อความแล้ว".to_string(),

        // --- Petty cash ---
        // Issue #202. "รายรับ-รายจ่าย" is the feature's own Thai gloss
        // (migration 059's doc comment / the `/v2/cash` page title) — no
        // room to attach (TB_Pay_History is a standalone ledger, not a
        // folio line).
        CreateCashEntry { .. } => "บันทึกรายรับ-รายจ่าย".to_string(),
    }
}

/// `"<verb> ห้อง <no>"`, or a bare verb when the room is unknown.
fn with_room(verb: &str, room_no: Option<&str>) -> String {
    match room_no {
        Some(room) => format!("{verb} ห้อง {room}"),
        None => verb.to_string(),
    }
}

/// `"ห้อง <no> <tail>"`, or a bare tail when the room is unknown. The
/// room-first word order the housekeeping states read naturally in.
fn room_prefixed(room_no: Option<&str>, tail: &str) -> String {
    match room_no {
        Some(room) => format!("ห้อง {room} {tail}"),
        None => tail.to_string(),
    }
}

/// Normalize a candidate room number: trimmed, and `None` rather than an
/// empty string (several payload fields are `String`, not `Option<String>`,
/// and carry `""` when the room was never resolved).
fn room_or(candidate: Option<&str>) -> Option<&str> {
    candidate.map(str::trim).filter(|room| !room.is_empty())
}

/// Truncate to `max` CHARACTERS, never bytes — every label here is Thai, and
/// a byte-wise cut would panic on a char boundary.
fn clamp_chars(text: String, max: usize) -> String {
    if text.chars().count() <= max {
        return text;
    }
    text.chars().take(max).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::payment::PaymentMethod;
    use crate::domain::shared::{DateRange, Money};
    use crate::outbox::intent::{
        BookingChanges, CreateBookingPayload, CreateCashEntryPayload, CreateCheckInPayload,
        RecordPaymentReceipt, UpdateRoomPayload,
    };
    use chrono::TimeZone;

    fn agg() -> Uuid {
        Uuid::nil()
    }

    fn stay() -> DateRange {
        DateRange {
            start: Utc.with_ymd_and_hms(2026, 8, 4, 0, 0, 0).unwrap(),
            end: Utc.with_ymd_and_hms(2026, 8, 5, 0, 0, 0).unwrap(),
        }
    }

    /// Every variant of [`WritebackIntent`], once. The point of the list is
    /// the compile error a new variant causes in [`stale_label`]'s exhaustive
    /// match — this test is what makes sure the label chosen there is also
    /// LOOKED at, not just written.
    fn every_intent() -> Vec<WritebackIntent> {
        use WritebackIntent::*;
        vec![
            CreateBooking {
                booking_id: agg(),
                payload: CreateBookingPayload {
                    customer_id: agg(),
                    legacy_cust_no: None,
                    customer_name: "TEST".into(),
                    customer_phone: None,
                    stay: stay(),
                    room_no: "302".into(),
                    room_type: "STD".into(),
                    price: Money::ZERO,
                    nights: 1,
                    deposit: Money::ZERO,
                    created_by: "test".into(),
                    notes: None,
                },
            },
            ModifyBooking {
                booking_id: agg(),
                changes: BookingChanges {
                    new_stay: None,
                    new_room_no: Some("415".into()),
                    new_room_type: None,
                    new_price: None,
                    new_state: None,
                    new_notes: None,
                    new_customer_phone: None,
                    new_customer_name: None,
                    customer_resave: None,
                },
            },
            CancelBooking { booking_id: agg() },
            CreateCheckIn {
                check_in_id: agg(),
                payload: CreateCheckInPayload {
                    room_no: "302".into(),
                    ..sample_checkin_payload()
                },
            },
            CancelCheckIn {
                check_in_id: agg(),
                reason: None,
                room_price: Money::ZERO,
                pay_to_subtract: Money::ZERO,
            },
            ExtendStay {
                check_in_id: agg(),
                new_end: stay().end,
                stay_start: stay().start,
                guest_label: String::new(),
                new_room_price_total: Money::ZERO,
                new_net_total: Money::ZERO,
                new_pay_total: Money::ZERO,
                new_balance_total: Money::ZERO,
            },
            CheckOut {
                check_in_id: agg(),
                nights: None,
                room_price_total: None,
                product_total: None,
                net_total: None,
                pay_total: None,
                balance: None,
                cr_id: None,
                room_ds_price_total: None,
                room_ds_nights: None,
                room_ds_pay_total: None,
            },
            RecordPayment {
                check_in_id: agg(),
                amount: Money::ZERO,
                method: PaymentMethod::Cash,
                receipt: RecordPaymentReceipt::default(),
                checkin_ds_id: None,
                price_per_night_baht: None,
                nights: None,
                payment_aggregate_id: None,
                vat_percent: None,
            },
            RefundPayment {
                check_in_id: agg(),
                original_payment_aggregate_id: None,
                amount: Money::ZERO,
                method: PaymentMethod::Cash,
                refund_reason: String::new(),
                payment_aggregate_id: None,
            },
            RoomChange {
                check_in_id: agg(),
                rc_id: 1,
            },
            MarkRoomClean {
                room_id: agg(),
                by: "test".into(),
            },
            MarkRoomDirty {
                room_id: agg(),
                by: "test".into(),
            },
            SetRoomMaintenance {
                room_id: agg(),
                maintenance: true,
            },
            SetRoomMaintenance {
                room_id: agg(),
                maintenance: false,
            },
            UpdateRoom {
                room_id: agg(),
                payload: UpdateRoomPayload {
                    room_no: "302".into(),
                    room_type_name: None,
                    price_weekday: None,
                    price_weekend: None,
                    price_special: None,
                    notes: None,
                },
            },
            MoveRoomTiles {
                room_id: agg(),
                moves: vec![],
            },
            UpdateCustomer {
                customer_id: agg(),
                resave: Default::default(),
            },
            AdjustProductStock {
                prod_legacy_no: "P1".into(),
                delta: -1.0,
                reason: None,
                product_aggregate_id: None,
            },
            IssueCoupon {
                coupon_aggregate_id: agg(),
                coupon_id: 1,
            },
            RedeemCoupon {
                coupon_aggregate_id: agg(),
                coupon_id: 1,
            },
            RecordPosSale {
                check_in_id: agg(),
                sale_id: 1,
            },
            RecordReceipt {
                receipt_aggregate_id: agg(),
                receipt_id: 1,
            },
            VoidPosSale {
                check_in_id: agg(),
                sale_id: 1,
            },
            OpenRound {
                round_aggregate_id: agg(),
                payload: sample_open_round(),
            },
            CloseRound {
                round_aggregate_id: agg(),
                payload: sample_close_round(),
            },
            RefundDeposit {
                check_in_id: agg(),
                cr_id: 1,
                by: "test".into(),
            },
            CreateNote {
                note_aggregate_id: agg(),
                target_kind: NoteTargetKind::Room,
                target_key: "302".into(),
                body: "hello".into(),
                created_by: "test".into(),
            },
            CreateNote {
                note_aggregate_id: agg(),
                target_kind: NoteTargetKind::Staff,
                target_key: "somchai".into(),
                body: "hello".into(),
                created_by: "test".into(),
            },
            MarkNoteRead {
                note_aggregate_id: agg(),
                target_kind: NoteTargetKind::Room,
            },
            UpsertRatePrice {
                rate_aggregate_id: agg(),
                payload: sample_rate_price(),
            },
            MirrorGuestImage {
                doc_id: 1,
                doc_type: "passport".into(),
                cust_legacy_no: None,
                cin_legacy_no: None,
                tmp_no: "T1".into(),
            },
            MirrorCompanion {
                cin_legacy_no: "CH26-000001".into(),
                name: "TEST".into(),
                country: "THAILAND".into(),
            },
            MirrorCompanionList {
                cin_legacy_no: "CH26-000001".into(),
                companions: vec![],
            },
            CompanionAdd {
                cin_legacy_no: "CH26-000001".into(),
                guest_id: 1,
                name: "TEST".into(),
                country: "THAILAND".into(),
            },
            CompanionDelete {
                cin_legacy_no: "CH26-000001".into(),
                legacy_id: 1,
            },
            CreateCashEntry {
                cash_aggregate_id: agg(),
                payload: CreateCashEntryPayload {
                    site_id: "hfhotel".into(),
                    entry_date: stay().start,
                    program_date: None,
                    amount: 100.0,
                    legacy_pay_type: "รายจ่าย".into(),
                    bill_no: None,
                    payee: None,
                    note: None,
                    group: None,
                    account: None,
                },
            },
        ]
    }

    fn sample_checkin_payload() -> CreateCheckInPayload {
        serde_json::from_value(serde_json::json!({
            "customer_id": agg(),
            "room_no": "302",
            "room_type": "STD",
            "stay": { "start": stay().start, "end": stay().end },
            "price_per_night": 0,
            "nights": 1,
            "price_total": 0,
            "created_by": "test",
            "guest_name_for_registry": "TEST",
            "guest_country": "THAILAND",
        }))
        .expect("CreateCheckInPayload fixture must deserialize")
    }

    fn sample_open_round() -> crate::outbox::intent::OpenRoundPayload {
        serde_json::from_value(serde_json::json!({
            "site_id": "hfhotel",
            "legacy_round_id": 1,
            "round_price": 0.0,
            "round_by": "test",
            "round_start": stay().start,
        }))
        .expect("OpenRoundPayload fixture must deserialize")
    }

    fn sample_close_round() -> crate::outbox::intent::CloseRoundPayload {
        serde_json::from_value(serde_json::json!({
            "site_id": "hfhotel",
            "legacy_round_id": 1,
            "round_by": "test",
            "round_end": stay().end,
        }))
        .expect("CloseRoundPayload fixture must deserialize")
    }

    fn sample_rate_price() -> crate::outbox::intent::RatePricePayload {
        serde_json::from_value(serde_json::json!({
            "site_id": "hfhotel",
            "room_type": "STD",
            "cust_type": "ทั่วไป",
            "price": 800.0,
        }))
        .expect("RatePricePayload fixture must deserialize")
    }

    /// Every intent produces a non-empty label, both with a resolved room and
    /// without one. The `None` half is the degrade-to-bare-verb contract.
    #[test]
    fn every_intent_has_a_label_with_and_without_a_room() {
        for intent in every_intent() {
            let with = stale_label(&intent, Some("302"));
            let without = stale_label(&intent, None);
            assert!(
                !with.trim().is_empty(),
                "{} produced an empty label with a room",
                intent.intent_name()
            );
            assert!(
                !without.trim().is_empty(),
                "{} produced an empty label without a room",
                intent.intent_name()
            );
            assert!(
                with.chars().count() <= MAX_ITEM_CHARS,
                "{} label is longer than one toast line: {with}",
                intent.intent_name()
            );
        }
    }

    /// The labels reception actually sees on the common paths, pinned.
    #[test]
    fn common_labels_read_the_way_reception_expects() {
        let by_name = |name: &str, room: Option<&str>| {
            every_intent()
                .iter()
                .find(|i| i.intent_name() == name)
                .map(|i| stale_label(i, room))
                .unwrap_or_else(|| panic!("no fixture for {name}"))
        };

        assert_eq!(by_name("create_check_in", Some("302")), "เช็คอิน ห้อง 302");
        assert_eq!(by_name("check_out", Some("415")), "เช็คเอาต์ ห้อง 415");
        assert_eq!(by_name("record_payment", Some("302")), "รับชำระเงิน ห้อง 302");
        // iHOTEL's own tile text — see ROOM_STATUS_PALETTE.md:12.
        assert_eq!(by_name("mark_room_dirty", Some("302")), "ห้อง 302 รอ ทำความสะอาด");
        // iHOTEL's cleaning-done button-log phrase, not the palette's
        // vacant-clean tile text ("ว่าง") — see COMPAT_CHEATSHEET.md:1391.
        assert_eq!(
            by_name("mark_room_clean", Some("302")),
            "ห้อง 302 ทำความสะอาดเรียบร้อย"
        );
        // Room-less intents degrade to a bare verb rather than "ห้อง ".
        assert_eq!(by_name("check_out", None), "เช็คเอาต์");
        assert_eq!(by_name("open_round", None), "เปิดรอบบิล");
        // Payload-carried room wins even with no resolved room_no.
        assert_eq!(by_name("create_check_in", None), "เช็คอิน ห้อง 302");
    }

    /// A blank/whitespace room number must not produce `"เช็คเอาต์ ห้อง "`.
    #[test]
    fn blank_room_degrades_to_bare_verb() {
        let intent = WritebackIntent::CancelCheckIn {
            check_in_id: agg(),
            reason: None,
            room_price: Money::ZERO,
            pay_to_subtract: Money::ZERO,
        };
        assert_eq!(stale_label(&intent, Some("   ")), "ยกเลิกเช็คอิน");
        assert_eq!(stale_label(&intent, Some("")), "ยกเลิกเช็คอิน");
        assert_eq!(stale_label(&intent, Some(" 302 ")), "ยกเลิกเช็คอิน ห้อง 302");
    }

    fn note(label: &str) -> StaleNote {
        StaleNote {
            intent: "test_intent",
            label: label.to_string(),
        }
    }

    /// The 3-intent booking case: one signal, `count: 3`, all three enumerated.
    #[test]
    fn coalesce_reports_every_job_in_the_tick() {
        let notes = vec![
            note("จองห้องพัก ห้อง 302"),
            note("รับชำระเงิน ห้อง 302"),
            note("เช็คอิน ห้อง 302"),
        ];
        let signal = coalesce("hfhotel", &notes);

        assert_eq!(signal.count(), 3);
        assert_eq!(signal.items().len(), 3);
        assert_eq!(signal.site(), "hfhotel");
        assert!(signal.summary().contains("เช็คอิน ห้อง 302"));
    }

    /// A single-job tick reads as the label itself — no counting noise.
    #[test]
    fn coalesce_of_one_note_is_just_its_label() {
        let signal = coalesce("hfville", &[note("เช็คเอาต์ ห้อง 415")]);
        assert_eq!(signal.count(), 1);
        assert_eq!(signal.summary(), "เช็คเอาต์ ห้อง 415");
        assert_eq!(signal.items(), ["เช็คเอาต์ ห้อง 415"]);
    }

    /// Items cap at MAX_ITEMS; the remainder is acknowledged, not dropped
    /// silently.
    #[test]
    fn coalesce_truncates_items_and_says_how_many_were_hidden() {
        let notes: Vec<StaleNote> = (1..=6).map(|n| note(&format!("เช็คเอาต์ ห้อง {n}"))).collect();
        let signal = coalesce("hfhotel", &notes);

        assert_eq!(signal.count(), 6);
        assert_eq!(signal.items().len(), MAX_ITEMS);
        assert!(
            signal.summary().contains("และอีก 3 รายการ"),
            "summary should acknowledge the hidden remainder, got: {}",
            signal.summary()
        );
    }

    /// Identical labels (three payments on one room) collapse in `items`
    /// while `count` still reports every landed job.
    #[test]
    fn coalesce_dedupes_identical_labels_but_not_the_count() {
        let notes = vec![
            note("รับชำระเงิน ห้อง 302"),
            note("รับชำระเงิน ห้อง 302"),
            note("รับชำระเงิน ห้อง 302"),
        ];
        let signal = coalesce("hfhotel", &notes);

        assert_eq!(signal.count(), 3, "every landed job is still counted");
        assert_eq!(signal.items(), ["รับชำระเงิน ห้อง 302"]);
        assert_eq!(signal.summary(), "รับชำระเงิน ห้อง 302");
    }

    /// Past MAX_ITEMISED_NOTES the summary stops enumerating — a backlog
    /// drain is not something a receptionist reads line by line.
    #[test]
    fn coalesce_uses_a_generic_summary_for_a_big_burst() {
        let notes: Vec<StaleNote> = (1..=47).map(|n| note(&format!("เช็คเอาต์ ห้อง {n}"))).collect();
        let signal = coalesce("hfhotel", &notes);

        assert_eq!(signal.count(), 47);
        assert_eq!(signal.summary(), "มีการเปลี่ยนแปลง 47 รายการ");
        assert_eq!(
            signal.items().len(),
            MAX_ITEMS,
            "items stay capped even when the summary goes generic"
        );
    }

    /// A 500-job burst through `coalesce` — the realistic post-outage-backlog
    /// shape. Not the hard-constraint proof (see the structural test below):
    /// `coalesce` collapses anything past `MAX_ITEMISED_NOTES` (20) to the
    /// ~26-char generic summary and caps `items` at `MAX_ITEMS` (3) on its
    /// own, so this can't exercise the clamp constants directly — it just
    /// pins that the everyday coalesce path stays well under budget too.
    #[test]
    fn a_500_job_batch_via_coalesce_serializes_well_under_the_notify_limit() {
        let notes: Vec<StaleNote> = (1..=500)
            .map(|n| note(&format!("ยกเลิกรายการขายสินค้าและคืนเงินมัดจำ ห้อง {n}")))
            .collect();
        let signal = coalesce("hfhotel", &notes);
        let payload = serde_json::to_string(&signal).expect("serialize");

        assert_eq!(signal.count(), 500);
        assert!(
            payload.len() < MAX_NOTIFY_PAYLOAD_BYTES,
            "payload was {} bytes, budget is {MAX_NOTIFY_PAYLOAD_BYTES}",
            payload.len()
        );
    }

    /// The hard constraint, proven structurally instead of through
    /// `coalesce`. The test above can never fail for any input reachable
    /// through `coalesce`, because `coalesce` itself already enforces
    /// `MAX_ITEMISED_NOTES`/`MAX_ITEMS` before a signal is even built. This
    /// one calls `LegacyStaleSignal::new` directly with the worst case the
    /// clamp is supposed to bound — a summary and items each longer than
    /// their respective caps — so it fails the instant `MAX_SUMMARY_CHARS`
    /// or `MAX_ITEMS` is raised without re-checking the payload budget. Thai
    /// is 3 bytes/char in UTF-8, which is what makes the char caps worth
    /// checking in bytes rather than assuming char-count alone is safe.
    #[test]
    fn worst_case_signal_serializes_under_the_notify_limit() {
        let signal = LegacyStaleSignal::new(
            "hfhotel",
            u32::MAX,
            "ก".repeat(1000),
            vec!["ข".repeat(1000); 10],
        );
        let payload = serde_json::to_string(&signal).expect("serialize");

        assert!(
            payload.len() < MAX_NOTIFY_PAYLOAD_BYTES,
            "payload was {} bytes, budget is {MAX_NOTIFY_PAYLOAD_BYTES}",
            payload.len()
        );
    }

    /// Clamping is per-character, not per-byte — a byte-wise truncate would
    /// panic mid-codepoint on Thai.
    #[test]
    fn construction_clamps_summary_and_items_by_characters() {
        let long_summary: String = "ก".repeat(MAX_SUMMARY_CHARS * 3);
        let long_item: String = "ข".repeat(MAX_ITEM_CHARS * 3);
        let signal = LegacyStaleSignal::new(
            "hfhotel",
            9,
            long_summary,
            vec![long_item.clone(), long_item.clone(), long_item.clone(), long_item],
        );

        assert_eq!(signal.summary().chars().count(), MAX_SUMMARY_CHARS);
        assert_eq!(signal.items().len(), MAX_ITEMS, "items list is capped");
        for item in signal.items() {
            assert_eq!(item.chars().count(), MAX_ITEM_CHARS);
        }
    }

    /// The channel and the SSE event name are one concept; and neither may
    /// collide with `routes/events.rs`'s `RESYNC_EVENT` ("refresh"), which
    /// means something structurally different.
    #[test]
    fn channel_and_event_names_are_stable_and_not_refresh() {
        assert_eq!(LEGACY_STALE_CHANNEL, "legacy_stale");
        assert_eq!(LEGACY_STALE_EVENT, "legacy_stale");
        assert_ne!(LEGACY_STALE_EVENT, crate::routes::events::RESYNC_EVENT);
    }

    /// The wire shape is a contract with the Tauri middleware — a rename of
    /// any of these fields breaks it silently.
    #[test]
    fn wire_shape_is_stable() {
        let signal = coalesce("hfhotel", &[note("เช็คอิน ห้อง 302")]);
        let json: serde_json::Value =
            serde_json::from_str(&serde_json::to_string(&signal).unwrap()).unwrap();

        for key in ["id", "site", "count", "summary", "items", "emitted_at"] {
            assert!(json.get(key).is_some(), "missing wire field `{key}`");
        }
        assert_eq!(json["count"], 1);
        assert_eq!(json["items"][0], "เช็คอิน ห้อง 302");

        // …and it round-trips, which is what the SSE listener's validation does.
        let parsed: LegacyStaleSignal = serde_json::from_value(json).expect("round-trip");
        assert_eq!(parsed, signal);
    }
}
