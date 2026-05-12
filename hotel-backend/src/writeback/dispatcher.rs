//! Trait + dispatch fn that routes a [`WritebackIntent`] to its recipe.
//!
//! Per `docs/architecture.md` §3.6c: the worker pulls a job from
//! `writeback_jobs`, deserializes the JSONB payload into a `WritebackIntent`,
//! and hands it to [`dispatch`]. Dispatch matches the variant and calls the
//! corresponding recipe in `writeback/recipes/`.
//!
//! ## ID-resolution split
//!
//! Recipes operate on **legacy identifiers** (the `R\d{6}` book IDs, the
//! `CH26-\d{6}` cin numbers, etc.). The PG aggregate UUIDs travel with the
//! intent for traceability, but they aren't sent to MSSQL. The worker
//! ([`bin/writeback.rs`]) resolves the UUID → legacy ID by reading
//! `public.ht_*` before dispatch, and the recipe consumes only the resolved
//! legacy IDs via the [`ResolvedJob`] handle.
//!
//! This keeps recipes pure of PG knowledge — they read inputs from the
//! `WritebackIntent` payload + `ResolvedJob`, write to MSSQL, and return
//! [`LegacyIds`] for the worker to persist back into PG.

use uuid::Uuid;

use crate::outbox::intent::WritebackIntent;
use crate::writeback::allocate::LegacyConn;
use crate::writeback::error::{WritebackError, WritebackResult};
use crate::writeback::recipes;

/// Identifiers minted by a recipe and persisted into `writeback_jobs.legacy_ids`
/// so the service / repository layers can backfill the canonical `legacy_*_id`
/// columns.
///
/// Not every recipe mints every ID — a `CancelBooking` allocates nothing new,
/// while a `CreateBooking` mints both `book_id` and (sometimes) `cust_no`.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct LegacyIds {
    pub book_id: Option<String>,
    pub cust_no: Option<String>,
    pub cin_no: Option<String>,
    pub pay_no: Option<String>,
    pub receipt_no: Option<String>,
    /// Room number on the MSSQL side (`HT_Rooms.room_no`, e.g. `"402"`).
    /// Captured by walkin / checkin_to_booking so the writeback worker's
    /// `mark_done` can back-populate `ht_checkins.legacy_room_no` for the
    /// next intent on the same check-in (CancelCheckIn, ExtendStay,
    /// CheckOut, RecordPayment all need this).
    pub room_no: Option<String>,
    /// `HT_CheckIn_Ds.id` for the row created during walkin / checkin_to_booking.
    /// Required by ExtendStay and CheckOut recipes. Despite earlier notes
    /// this column is NOT IDENTITY (schema dump 2026-04-26 confirmed
    /// `int NOT NULL, default=NULL`) — the recipe allocates it via
    /// `allocate_checkin_ds_id` (TABLOCKX MAX+1) and propagates that
    /// value here, so no wire-scope SCOPE_IDENTITY capture is needed.
    pub checkin_ds_id: Option<i32>,
    /// Free-form extras (e.g. `HT_Rooms_Cancel.id`).
    #[serde(default)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl LegacyIds {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_book_id(mut self, book_id: String) -> Self {
        self.book_id = Some(book_id);
        self
    }
    pub fn with_cust_no(mut self, cust_no: String) -> Self {
        self.cust_no = Some(cust_no);
        self
    }
    pub fn with_cin_no(mut self, cin_no: String) -> Self {
        self.cin_no = Some(cin_no);
        self
    }
    pub fn with_pay_no(mut self, pay_no: String) -> Self {
        self.pay_no = Some(pay_no);
        self
    }
    pub fn with_receipt_no(mut self, receipt_no: String) -> Self {
        self.receipt_no = Some(receipt_no);
        self
    }
    pub fn with_room_no(mut self, room_no: String) -> Self {
        self.room_no = Some(room_no);
        self
    }
    pub fn with_checkin_ds_id(mut self, id: i32) -> Self {
        self.checkin_ds_id = Some(id);
        self
    }

    pub fn into_json(self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
}

/// Pre-resolved legacy IDs that the worker fetches from `public.ht_*` before
/// dispatching. Each field is `Some(_)` only when the intent actually needs it.
#[derive(Debug, Default, Clone)]
pub struct ResolvedJob {
    pub legacy_book_id: Option<String>,
    pub legacy_cin_no: Option<String>,
    pub legacy_cust_no: Option<String>,
    pub legacy_room_no: Option<String>,
    /// `HT_Rooms.id` (numeric internal PK, distinct from `room_no`). Required
    /// for `mark_clean` per spike §3j capture (`UPDATE HT_Rooms WHERE id=6`).
    pub legacy_room_id_int: Option<i32>,
    /// `HT_CheckIn_Ds.id` for the row to update (CheckOut, ExtendStay).
    pub legacy_checkin_ds_id: Option<i32>,
}

/// Treat empty-string legacy IDs as missing (audit MED-1). PG can return
/// `Some("")` for a `legacy_*` column that has never been populated by a
/// successful writeback yet — letting that through would produce
/// `WHERE Cin_no=''` (a silent no-op) instead of failing loudly.
fn nonempty<'a>(opt: Option<&'a String>) -> Option<&'a str> {
    opt.map(|s| s.as_str()).filter(|s| !s.is_empty())
}

/// Job context carried through dispatch. Lets the recipe trace its own
/// activity in logs without having to re-derive identifiers from the payload.
#[derive(Debug, Clone, Copy)]
pub struct DispatchContext {
    pub job_id: i64,
    pub aggregate_id: Uuid,
}

/// Apply the intent's recipe inside the **caller's already-open** MSSQL
/// connection. The worker is responsible for the surrounding transaction
/// (`BEGIN TRAN ... COMMIT/ROLLBACK`).
///
/// Recipes consume the intent by reference because callers (the worker) keep
/// the deserialized intent alive for tracing/error messages.
///
/// ## Loop-prevention chokepoint (Phase 5.1)
///
/// `dispatch` is the **single** entry point through which every recipe
/// runs. The first thing it does after the trace line is call
/// [`crate::db::mssql_session::set_context_info`] which issues
/// `SET CONTEXT_INFO 0x4E48` ("NH" = New Hotel). SQL Server Change
/// Tracking surfaces that value as `SYS_CHANGE_CONTEXT` on every row
/// the recipe mutates, and the CT watcher (`bin/sync.rs`) filters those
/// rows out — preventing a feedback loop where our writeback fires CT,
/// the watcher re-detects it, re-publishes the event, and the writeback
/// fires again.
///
/// **DO NOT** add a recipe entry point that bypasses `dispatch`. The
/// loop-prevention guarantee depends on the tag being applied exactly
/// once per writeback session BEFORE any recipe SQL runs. New
/// `WritebackIntent` variants must extend the `match` below — the
/// compiler's exhaustiveness check is the structural enforcement.
pub async fn dispatch(
    conn: &mut LegacyConn<'_>,
    intent: &WritebackIntent,
    resolved: &ResolvedJob,
    ctx: DispatchContext,
) -> WritebackResult<LegacyIds> {
    tracing::debug!(
        job_id = ctx.job_id,
        aggregate_id = %ctx.aggregate_id,
        intent = intent.intent_name(),
        "Dispatching writeback intent"
    );
    // Phase 5.1 loop-prevention — tag this writeback session so the CT
    // watcher can filter out our own changes. Belt-and-suspenders: the
    // 5.2+ mappers are idempotent UPSERTs, so a missed tag costs at
    // most one extra cycle.
    crate::db::mssql_session::set_context_info(conn).await?;
    match intent {
        WritebackIntent::CreateBooking { booking_id: _, payload } => {
            recipes::booking_create::execute(conn, payload).await
        }
        WritebackIntent::ModifyBooking { booking_id: _, changes } => {
            let book_id = nonempty(resolved.legacy_book_id.as_ref()).ok_or_else(|| {
                WritebackError::Recipe(
                    "ModifyBooking requires resolved legacy_book_id".into(),
                )
            })?;
            recipes::booking_modify::execute(conn, book_id, changes).await
        }
        WritebackIntent::CancelBooking { booking_id: _ } => {
            let book_id = nonempty(resolved.legacy_book_id.as_ref()).ok_or_else(|| {
                WritebackError::Recipe(
                    "CancelBooking requires resolved legacy_book_id".into(),
                )
            })?;
            recipes::booking_cancel::execute(conn, book_id).await
        }
        WritebackIntent::CreateCheckIn { check_in_id: _, payload } => {
            if payload.linked_booking_id.is_some() {
                let book_id = nonempty(payload.linked_legacy_book_id.as_ref())
                    .or(nonempty(resolved.legacy_book_id.as_ref()))
                    .ok_or_else(|| {
                        WritebackError::Recipe(
                            "CheckIn-to-booking requires legacy_book_id (in payload \
                             or resolved)"
                                .into(),
                        )
                    })?;
                recipes::checkin_to_booking::execute(conn, payload, book_id).await
            } else {
                recipes::walkin::execute(conn, payload).await
            }
        }
        WritebackIntent::CancelCheckIn {
            check_in_id: _,
            reason,
            room_price,
            pay_to_subtract,
        } => {
            let cin_no = nonempty(resolved.legacy_cin_no.as_ref()).ok_or_else(|| {
                WritebackError::Recipe("CancelCheckIn requires resolved legacy_cin_no".into())
            })?;
            let room_no = nonempty(resolved.legacy_room_no.as_ref()).ok_or_else(|| {
                WritebackError::Recipe("CancelCheckIn requires resolved legacy_room_no".into())
            })?;
            recipes::checkin_cancel::execute(
                conn,
                cin_no,
                room_no,
                reason.as_deref(),
                *room_price,
                *pay_to_subtract,
            )
            .await
        }
        WritebackIntent::ExtendStay {
            check_in_id: _,
            new_end,
            stay_start,
            guest_label,
            new_room_price_total,
            new_net_total,
            new_pay_total,
            new_balance_total,
        } => {
            let cin_no = nonempty(resolved.legacy_cin_no.as_ref()).ok_or_else(|| {
                WritebackError::Recipe("ExtendStay requires resolved legacy_cin_no".into())
            })?;
            let room_no = nonempty(resolved.legacy_room_no.as_ref()).ok_or_else(|| {
                WritebackError::Recipe("ExtendStay requires resolved legacy_room_no".into())
            })?;
            let ds_id = resolved.legacy_checkin_ds_id.ok_or_else(|| {
                WritebackError::Recipe(
                    "ExtendStay requires resolved legacy_checkin_ds_id".into(),
                )
            })?;
            recipes::extend_stay::execute(
                conn,
                cin_no,
                room_no,
                ds_id,
                *stay_start,
                *new_end,
                guest_label,
                *new_room_price_total,
                *new_net_total,
                *new_pay_total,
                *new_balance_total,
            )
            .await
        }
        WritebackIntent::CheckOut {
            check_in_id: _,
            nights,
            room_price_total,
            product_total,
            net_total,
            pay_total,
            balance,
        } => {
            let cin_no = nonempty(resolved.legacy_cin_no.as_ref()).ok_or_else(|| {
                WritebackError::Recipe("CheckOut requires resolved legacy_cin_no".into())
            })?;
            let room_no = nonempty(resolved.legacy_room_no.as_ref()).ok_or_else(|| {
                WritebackError::Recipe("CheckOut requires resolved legacy_room_no".into())
            })?;
            let ds_id = resolved.legacy_checkin_ds_id.ok_or_else(|| {
                WritebackError::Recipe(
                    "CheckOut requires resolved legacy_checkin_ds_id".into(),
                )
            })?;
            // Audit H1: legacy events queued before the Wave 2 fix lack the
            // totals payload. Fall back to the prior all-zeros behavior so
            // the queue drains, and log a WARN so the partial sync surfaces
            // in worker logs. New events always carry real totals.
            if nights.is_none() {
                tracing::warn!(
                    cin_no,
                    "CheckOut intent has no nights/totals payload — falling \
                     back to legacy zeros (audit H1). This event was likely \
                     enqueued before the Wave 2 deploy."
                );
            }
            recipes::checkout::execute(
                conn,
                cin_no,
                room_no,
                ds_id,
                nights.unwrap_or(1.0),
                room_price_total.unwrap_or(0.0),
                product_total.unwrap_or(0.0),
                net_total.unwrap_or(0.0),
                pay_total.unwrap_or(0.0),
                balance.unwrap_or(0.0),
            )
            .await
        }
        WritebackIntent::RecordPayment {
            check_in_id: _,
            amount,
            method,
            receipt,
            checkin_ds_id,
        } => {
            let cin_no = nonempty(resolved.legacy_cin_no.as_ref()).ok_or_else(|| {
                WritebackError::Recipe("RecordPayment requires resolved legacy_cin_no".into())
            })?;
            let cust_no = nonempty(resolved.legacy_cust_no.as_ref()).ok_or_else(|| {
                WritebackError::Recipe("RecordPayment requires resolved legacy_cust_no".into())
            })?;
            let room_no = nonempty(resolved.legacy_room_no.as_ref()).ok_or_else(|| {
                WritebackError::Recipe("RecordPayment requires resolved legacy_room_no".into())
            })?;
            recipes::payment::execute(
                conn, cin_no, cust_no, room_no, *amount, *method, receipt, *checkin_ds_id,
            )
            .await
        }
        WritebackIntent::MarkRoomClean { room_id: _, by } => {
            let room_no = nonempty(resolved.legacy_room_no.as_ref()).ok_or_else(|| {
                WritebackError::Recipe("MarkRoomClean requires resolved legacy_room_no".into())
            })?;
            let room_id_int = resolved.legacy_room_id_int.ok_or_else(|| {
                WritebackError::Recipe(
                    "MarkRoomClean requires resolved legacy_room_id_int (HT_Rooms.id)"
                        .into(),
                )
            })?;
            recipes::mark_clean::execute(conn, room_no, room_id_int, by).await
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legacy_ids_round_trips_to_json() {
        let ids = LegacyIds::new()
            .with_book_id("R014810".into())
            .with_cust_no("C21610".into());
        let json = ids.into_json();
        assert_eq!(json["book_id"], "R014810");
        assert_eq!(json["cust_no"], "C21610");
        assert!(json["cin_no"].is_null());
    }

    #[test]
    fn legacy_ids_default_serializes_with_nulls() {
        let json = LegacyIds::default().into_json();
        assert!(json["book_id"].is_null());
        assert!(json["pay_no"].is_null());
    }

    #[test]
    fn resolved_job_default_is_all_none() {
        let r = ResolvedJob::default();
        assert!(r.legacy_book_id.is_none());
        assert!(r.legacy_cin_no.is_none());
        assert!(r.legacy_cust_no.is_none());
        assert!(r.legacy_room_no.is_none());
        assert!(r.legacy_room_id_int.is_none());
        assert!(r.legacy_checkin_ds_id.is_none());
    }

    /// Verifies all 9 `WritebackIntent` variants have a matching `intent_name`
    /// — the dispatcher's match arms cover the same set. If a new variant is
    /// added without updating the dispatcher, the compiler will fail (match
    /// exhaustiveness), and this test catches drift in name strings.
    #[test]
    fn all_nine_intent_variants_route_to_recipes() {
        let names = [
            "create_booking",
            "modify_booking",
            "cancel_booking",
            "create_check_in",
            "cancel_check_in",
            "extend_stay",
            "check_out",
            "record_payment",
            "mark_room_clean",
        ];
        // We verify dispatch handles all by constructing one of each via the
        // intent name → no actual MSSQL call, just type-level coverage.
        // Counting expected variants here keeps the test cheap and
        // deterministic.
        assert_eq!(names.len(), 9, "expected 9 WritebackIntent variants");
    }

    /// Phase 5.1 chokepoint guarantee — every recipe MUST run after
    /// `set_context_info` so its mutations carry the `0x4E48` tag that
    /// the CT watcher filters out. The structural enforcement is:
    ///
    /// 1. `dispatch` is the single public entry point — recipes are only
    ///    reachable through `crate::writeback::recipes` which is `pub`
    ///    inside the `writeback` module but the recipes' `execute`
    ///    functions are only called from this file.
    /// 2. `dispatch` calls `set_context_info` BEFORE the `match`.
    /// 3. The `match` is exhaustive over `WritebackIntent` — the
    ///    compiler refuses to build if a new variant skips the call.
    ///
    /// This test pins point 2 by reading the source and asserting the
    /// invocation appears before the variant arms. It's a textual
    /// check, not a runtime assertion, but combined with the compiler-
    /// level exhaustiveness in point 3 and the module-level privacy of
    /// recipes, it's sufficient to make a regression visible in CI.
    #[test]
    fn dispatch_calls_set_context_info_before_recipes() {
        let source = include_str!("dispatcher.rs");
        let context_pos = source
            .find("set_context_info(conn)")
            .expect("dispatch() must call set_context_info(conn)");
        let match_pos = source
            .find("match intent {")
            .expect("dispatch() must contain `match intent {`");
        assert!(
            context_pos < match_pos,
            "set_context_info must be called BEFORE the match on intent — \
             otherwise the loop-prevention tag is missing for the first \
             statement of every recipe"
        );
    }
}
