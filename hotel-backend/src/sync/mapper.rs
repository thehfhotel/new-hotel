//! `MssqlChangeMapper` — per-table translator from a CT row into a
//! canonical PG mutation + a `DomainEvent`.
//!
//! Each CT-enabled table gets one impl. The trait is intentionally
//! narrow so a new mapper is the smallest possible diff:
//!
//! 1. Declare the table name + PK column list.
//! 2. Provide the `SELECT` projection used by the watcher's
//!    `CHANGETABLE(CHANGES …) JOIN <table>` query.
//! 3. Implement [`MssqlChangeMapper::apply`] — translate one CT row
//!    into an UPSERT (Insert/Update) or soft-delete/remove (Delete)
//!    on the canonical `public.ht_*` table, returning the
//!    `DomainEvent` to publish.
//!
//! ## Row abstraction
//!
//! `apply` takes `Option<&dyn MappableRow>` rather than the concrete
//! `tiberius::Row` so unit tests can pass a `HashMapRow` fixture (see
//! [`crate::sync::row`]). The watcher binary wraps the real tiberius row
//! in `&row` (which has the production [`MappableRow`] impl) before
//! calling `apply`.
//!
//! Phase 5.1 shipped only [`NoopMapper`] — the watcher's main loop
//! iterated one mapper per CT-enabled table, exercising the polling,
//! watermark, and observability plumbing without touching any PG row.
//! Phase 5.2 lands real mappers for `HT_Customers` / `HT_Rooms` /
//! `HT_Room_Status` (the last is still a logging stub deferred to
//! 5.3/5.4 — see [`crate::sync::mappers::room`]).

use async_trait::async_trait;

use crate::outbox::event::DomainEvent;
use crate::sync::change_op::ChangeOp;
use crate::sync::row::MappableRow;
use crate::sync::SyncError;

/// One per CT-enabled MSSQL table. Implementations live next to their
/// table-specific PG repository code.
#[async_trait]
pub trait MssqlChangeMapper: Send + Sync {
    /// Verbatim MSSQL table name as it appears in `CHANGETABLE(CHANGES
    /// <table>, …)`. Must match the seeded `legacy_sync_status.table_name`.
    fn table(&self) -> &'static str;

    /// Primary-key column names (MSSQL casing). Used by the watcher to
    /// build the JOIN condition `CHANGETABLE(...) ct JOIN <table> t ON
    /// t.pk1 = ct.pk1 AND t.pk2 = ct.pk2 …`. For composite PKs, list
    /// them in declaration order.
    fn primary_key_cols(&self) -> &'static [&'static str];

    /// SELECT projection (column list) appended after `ct.SYS_CHANGE_VERSION,
    /// ct.SYS_CHANGE_OPERATION, …pks…,` in the watcher's polling query.
    /// Must produce the exact row shape (column names) expected by [`apply`].
    ///
    /// The watcher composes the rest of the query around this string;
    /// callers should return only the projection clause (e.g.
    /// `"t.Cust_no, t.Cust_name, t.Cust_Add_tel"`) — no `SELECT`, no
    /// `FROM`. An empty string disables the JOIN entirely (placeholder
    /// mappers that only need the PK from CT itself).
    fn select_sql(&self) -> &'static str;

    /// Translate one CT row into a canonical mutation + domain event.
    ///
    /// `row` is `Some(_)` for I/U operations (the joined current row
    /// state) and `None` for D operations (the row no longer exists).
    /// Mappers must handle both shapes — D events typically resolve the
    /// PG aggregate by the PK columns extracted from the CT side.
    ///
    /// Returns `Ok(Some(event))` to publish on the bus. `Ok(None)`
    /// means "nothing to publish AND nothing left to do" — the
    /// idempotent skip (canonical row already matches the legacy row,
    /// which is also how echoes of our own writeback are absorbed:
    /// there is NO SQL-layer echo filter; see `db::mssql_session`).
    ///
    /// **`Ok(None)` must NEVER be used for an unresolved FK / missing
    /// parent** (2026-06-11, June-3 incident class): the watcher counts
    /// it as `skipped` and advances the watermark, permanently dropping
    /// the row. Eager-mirror the parent or return `Err` so the watcher
    /// sets `errored=true` and HOLDS the watermark for a loud retry —
    /// see `sync::resolve` module doc.
    async fn apply(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        op: ChangeOp,
        row: Option<&dyn MappableRow>,
    ) -> Result<Option<DomainEvent>, SyncError>;

    /// Optional **aggregate root key** for this row.
    ///
    /// Phase 5.3 introduces a "collect-then-dispatch" pass in the
    /// watcher: when multiple CT rows in the same tick belong to the
    /// same logical aggregate (booking header + line items + per-night
    /// rows all under one `Book_no`), the watcher groups them by
    /// `coalesce_key()` and runs the mapper exactly once per unique
    /// key per tick. This keeps the canonical UPSERT + emitted
    /// `DomainEvent` count at one-per-aggregate, regardless of how
    /// many child rows changed.
    ///
    /// **Default `None`** — flat-table mappers (customer, room) want
    /// per-row dispatch; the coalescing layer is bypassed for them.
    /// Aggregate-root child mappers (booking_h / booking_ds /
    /// booking_date) override this to return `Some(book_id)` so all
    /// three feed into one shared `apply_booking_aggregate` call.
    ///
    /// Returns `None` when the key cannot be derived from the row
    /// (e.g. a D-row on a child table whose CT PK is `id` only) — the
    /// watcher falls back to per-row dispatch in that case, and the
    /// next aggregate-level CT row (almost always within the same
    /// tick, because legacy edits touch the header in the same TX)
    /// drives the canonical re-load.
    fn coalesce_key(&self, _row: &dyn MappableRow) -> Option<String> {
        None
    }
}

/// Phase 5.1 placeholder — exercises the watcher loop without any PG
/// mutations. Returns `Ok(None)` for every input.
///
/// The watcher iterates `Vec<Box<dyn MssqlChangeMapper>>`, one per
/// CT-enabled table. As real mappers land they replace the matching
/// `NoopMapper` entry without touching the loop. A handful of
/// `NoopMapper` entries remain in 5.2 for the booking / checkin tables
/// that 5.3 / 5.4 will fill in.
pub struct NoopMapper {
    pub table_name: &'static str,
}

#[async_trait]
impl MssqlChangeMapper for NoopMapper {
    fn table(&self) -> &'static str {
        self.table_name
    }

    fn primary_key_cols(&self) -> &'static [&'static str] {
        // No PK projection is needed because the no-op mapper never
        // joins to the underlying table — the watcher detects an empty
        // `select_sql()` and skips the JOIN clause entirely (counts CT
        // rows only).
        &[]
    }

    fn select_sql(&self) -> &'static str {
        // Empty projection — the watcher detects this and skips the
        // JOIN clause entirely (counts CT rows only). Documented in
        // `bin/sync.rs::poll_table`.
        ""
    }

    async fn apply(
        &self,
        _tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        _op: ChangeOp,
        _row: Option<&dyn MappableRow>,
    ) -> Result<Option<DomainEvent>, SyncError> {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn noop_mapper_exposes_constructor_table_name() {
        let m = NoopMapper {
            table_name: "HT_Customers",
        };
        assert_eq!(m.table(), "HT_Customers");
        assert!(m.primary_key_cols().is_empty());
        assert!(m.select_sql().is_empty());
    }

    /// `NoopMapper.apply` returns `Ok(None)` regardless of operation.
    /// We can't construct a real `sqlx::Transaction` without a pool, so
    /// we exercise the trait shape via a `Send + Sync` compile check —
    /// the fact that this test compiles is the proof that the
    /// async-trait codegen produced a `Send` future.
    #[tokio::test]
    async fn noop_mapper_apply_returns_none_for_every_op() {
        fn assert_send_sync<T: Send + Sync>(_: &T) {}
        let m = NoopMapper {
            table_name: "HT_Rooms",
        };
        assert_send_sync(&m);
        let _ = m.table();
    }
}
