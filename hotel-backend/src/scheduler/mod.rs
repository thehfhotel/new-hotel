//! Scheduler module for background jobs

mod jobs;
pub mod mirror;
pub mod mirror_probe;
pub mod notification_state;
pub mod payment_ledger_probe;
pub mod sync;

pub use jobs::init_scheduler;

#[cfg(test)]
mod tests {
    //! Issue #275: every scheduler file that reads from the legacy MSSQL
    //! pool must route through `simple_query_with_timeout_pooled` (or a
    //! sibling helper in `db::mssql_timeout`) rather than calling
    //! `conn.simple_query(...)` directly. A raw call gets neither the
    //! per-op timeout nor `PoisonableConnection::mark_poisoned()` on
    //! timeout (issue #274) — a stalled query hangs unbounded and, if the
    //! future is ever dropped, can hand a TDS-desynced connection back to
    //! the pool. On HF Ville that pool is the SAME one the CT watcher
    //! draws from (`reconcile_mssql = mssql.clone()`), so the gap can
    //! reintroduce the #274 cascade through a door #274's own fix doesn't
    //! cover.
    //!
    //! Source-scan pin, same `include_str!` idiom already used by
    //! `writeback::dispatcher`'s and `bin::sync`'s tests: fails loudly if
    //! a future call site reintroduces `conn.simple_query(`, instead of
    //! silently regressing the poison-aware wrapper's coverage.
    //!
    //! Deliberately excludes `bin/*` — the one-shot admin/backfill/bootstrap
    //! tools are operator-invoked with no scheduled exposure (tracked as
    //! remaining in issue #275) and `bin/sync.rs` still has one raw call in
    //! its `--bootstrap --dry-run` preview path for exactly that reason.
    //!
    //! Issue #279 (sibling of #275): the same legacy-pool files can also
    //! reach MSSQL through a different tiberius API shape —
    //! `tiberius::Query::new(sql)` + optional `.bind(..)` + `.query(&mut
    //! conn)` — used wherever a statement needs a bound parameter
    //! (`Client::simple_query` has no bind support). That shape had no
    //! wrapper at all until `db::mssql_timeout::query_with_timeout_pooled`
    //! (plus its `&mut LegacyConn` / execute-shape siblings
    //! `query_with_timeout` / `query_execute_with_timeout`), so it gets the
    //! same pin below: a direct `.query(&mut` call bypasses the timeout and
    //! poisoning exactly like a direct `.simple_query(` call does.

    /// Every scheduler file that owns a `legacy_pool: &DbPool` connection.
    /// Add new files here as they start touching the legacy pool.
    const SCHEDULER_SOURCES: &[(&str, &str)] = &[
        ("scheduler/sync.rs", include_str!("sync.rs")),
        ("scheduler/mirror_probe.rs", include_str!("mirror_probe.rs")),
        (
            "scheduler/payment_ledger_probe.rs",
            include_str!("payment_ledger_probe.rs"),
        ),
        ("scheduler/mirror.rs", include_str!("mirror.rs")),
        ("scheduler/jobs.rs", include_str!("jobs.rs")),
    ];

    #[test]
    fn scheduler_files_contain_no_raw_simple_query_calls() {
        for (name, src) in SCHEDULER_SOURCES {
            assert!(
                !src.contains(".simple_query("),
                "{name} calls `.simple_query(` directly — route it through \
                 `simple_query_with_timeout_pooled` (db::mssql_timeout) so the \
                 call gets a per-op timeout and poisons the connection on \
                 timeout instead of hanging unbounded and desyncing the pool \
                 (issue #275 / #274)"
            );
        }
    }

    /// Companion: every file listed above must actually import the
    /// helper. Catches the trivial way the pin above could be gamed —
    /// removing the import while leaving stale dead code — though in
    /// practice the compiler's unused-import lint already covers this;
    /// pinned anyway so the invariant reads in one place.
    #[test]
    fn scheduler_files_import_the_timeout_wrapper() {
        for (name, src) in SCHEDULER_SOURCES {
            assert!(
                src.contains("simple_query_with_timeout_pooled"),
                "{name} no longer uses simple_query_with_timeout_pooled — if it \
                 stopped touching the legacy MSSQL pool entirely, remove it from \
                 SCHEDULER_SOURCES instead of leaving a stale entry"
            );
        }
    }

    /// Issue #279 sibling of the pin above: bans a direct `.query(&mut`
    /// call on a `tiberius::Query` — the bound-parameter API shape
    /// `Client::simple_query` can't express. Not paired with a "must
    /// import `query_with_timeout_pooled`" companion test the way
    /// `scheduler_files_import_the_timeout_wrapper` pairs with the
    /// `.simple_query(` pin above: unlike the `simple_query` shape, not
    /// every file in `SCHEDULER_SOURCES` has a bound-parameter statement
    /// to make (`mirror.rs` and `jobs.rs` currently don't), so requiring
    /// the import universally would be a false-positive trap rather than
    /// a real invariant.
    #[test]
    fn scheduler_files_contain_no_raw_query_bind_calls() {
        for (name, src) in SCHEDULER_SOURCES {
            assert!(
                !src.contains(".query(&mut"),
                "{name} calls `.query(&mut` directly on a tiberius::Query — route \
                 it through `query_with_timeout_pooled` (or the `&mut LegacyConn` \
                 / execute-shape siblings `query_with_timeout` / \
                 `query_execute_with_timeout`) in db::mssql_timeout so the \
                 bound-parameter call gets a per-op timeout and poisons the \
                 connection on timeout instead of hanging unbounded and desyncing \
                 the pool (issue #279, sibling of #275 / #274)"
            );
        }
    }
}
