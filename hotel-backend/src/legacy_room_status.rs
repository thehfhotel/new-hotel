//! iHOTEL room-cleanliness READ for the maid surface (CR-1, owner decision
//! locked 2026-08-15).
//!
//! ## Why this exists
//!
//! `/hk` used to show the maid canonical `ht_rooms_new.room_clean`, which is a
//! MIRROR of legacy `HT_Rooms.Room_Clean` maintained by the CT watcher. When
//! the mirror lags (or, at HF Ville, points at the wrong room — see
//! `bin/repair_room_legacy_keys`), the maid and the receptionist read the same
//! room off two different screens and disagree. The owner's decision is that
//! **iHOTEL wins on this surface**: what reception sees on the iHOTEL board is
//! what the maid must see on her phone, with PG as the mirror rather than the
//! source.
//!
//! ## Polarity (do not "simplify" this)
//!
//! Legacy `HT_Rooms.Room_Clean` is a NEEDS-CLEANING flag, so it is INVERTED
//! relative to our canonical column:
//!
//! | legacy `Room_Clean` | meaning          | canonical `room_clean` |
//! |---------------------|------------------|------------------------|
//! | `'no'`              | no cleaning needed | `true` (IS clean)    |
//! | `'yes'`             | needs cleaning     | `false` (dirty)      |
//!
//! Same mapping as `sync::mappers::room` (`legacy_yesno_to_bool(..).map(|needs| !needs)`)
//! and the `mark_clean` / `mark_dirty` writeback recipes. Anything else —
//! `NULL`, `''`, `'Yes'`, junk — is `None`: UNKNOWN, never a guess. An unknown
//! room keeps its PG value rather than being flipped by a literal we do not
//! recognise.
//!
//! ## Keyed on `Room_no`, NOT on `legacy_room_id_int`
//!
//! `bin/repair_room_legacy_keys` settled which pointer is truthful against live
//! data: the room-NUMBER path is correct, the ID path was corrupt at HF Ville
//! (30 of 34 rooms carried a `legacy_room_id_int` pointing at a different
//! legacy room — NOT `legacy_room_no`, which was the truthful side; the bin's
//! own correctness proof was that it reported 0 changes at HF Hotel).
//! Repaired and verified 2026-08-16: `hotelville.ht_rooms_new` reports 0/34
//! mismatched pointers (58/0 at HF Hotel as the control).
//!
//! Reading by `Room_no` is therefore both the correct join AND the one that
//! makes the maid's screen agree with the iHOTEL board she is being reconciled
//! against — it is what surfaces the Ville 104/203-class divergence instead of
//! reproducing it. Keying on `Room_no` stays correct after the repair: it is
//! the column iHOTEL's own board is keyed on, so it cannot reintroduce a
//! pointer class of bug even if the id column drifts again.
//!
//! ## Read-only, and outside the write boundary
//!
//! This module issues exactly ONE statement, a `SELECT`. It performs no legacy
//! write, opens no transaction, and takes no lock hint (default `READ
//! COMMITTED`, matching every other legacy read in this repo — a `NOLOCK`
//! dirty read of a mid-transaction iHOTEL value would be worse than the
//! documented fallback). Invariant #6 (new legacy WRITES ship dark) is
//! therefore not engaged.
//!
//! ## Placement
//!
//! A crate-root module next to [`crate::hfid_location`], NOT under
//! `repository/` (which is PG-only by `docs/architecture.md` §2) and not under
//! `service/` (it holds no business rule and no transaction). It is an
//! outbound READ adapter that turns "which site" into "what iHOTEL says". The
//! policy built on the answer — iHOTEL-wins merge, the stale note, the
//! divergence log — lives in `routes::hk`, the one place that knows what a maid
//! should see.
//!
//! `docs/architecture.md`'s "Routes never know about MSSQL" still holds
//! literally: `routes::hk` depends on the [`RoomCleanSource`] TRAIT, never on
//! tiberius, and the decommission story is unchanged — drop the reader, the
//! surface falls back to canonical PG on its own.
//!
//! ## Fail-soft contract (the opposite of `hfid_location`)
//!
//! [`hfid_location`](crate::hfid_location) fails CLOSED because answering
//! wrongly there sends a maid to the wrong hotel. This one fails SOFT
//! ([`RoomCleanOutcome::Unavailable`] ⇒ show the PG mirror plus a visible Thai
//! note) because answering nothing here means a maid on a stairwell stares at
//! an error page instead of her room list. Stale-but-shown beats dead screen —
//! the owner's call, and the reason the timeout is short.

use std::collections::{BTreeMap, HashMap};
use std::fmt;
use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;

use crate::db::mssql_timeout::simple_query_with_explicit_timeout;
use crate::db::DbPool;

/// Whole-budget cap on one iHOTEL room-status read — pool acquire AND the
/// query, together.
///
/// Deliberately shorter than [`crate::db::mssql_timeout`]'s 10s read default:
/// this call sits in the request path of a maid's room list on a phone, and a
/// legacy row lock must degrade to the fallback note in seconds, not stall the
/// screen. 3s comfortably clears the WG-tunnelled round-trip (p99 ~400ms
/// handshake, and this is a 34-58 row table scan).
pub const ROOM_STATUS_TIMEOUT: Duration = Duration::from_secs(3);

/// Budget handed to the inner query, leaving headroom inside
/// [`ROOM_STATUS_TIMEOUT`] for the bb8 acquire that precedes it. The OUTER
/// bound is still authoritative — see [`MssqlRoomCleanSource::room_clean`].
const QUERY_TIMEOUT: Duration = Duration::from_millis(2_500);

/// The one statement this module issues. READ-ONLY.
pub const ROOM_STATUS_SQL: &str = "SELECT Room_no, Room_Clean FROM HT_Rooms";

/// What iHOTEL says about room cleanliness right now.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoomCleanOutcome {
    /// iHOTEL answered. Map is `Room_no` (trimmed) → `is_clean` in CANONICAL
    /// polarity (already inverted; `true` = IS clean). Rooms whose
    /// `Room_Clean` literal was NULL / blank / unrecognised are ABSENT from the
    /// map rather than guessed — an absent room keeps its PG value.
    Available(HashMap<String, bool>),
    /// No answer was obtained: no reader configured for this branch, pool
    /// acquire failed, the query errored, or the budget elapsed. The caller
    /// MUST fall back to the canonical PG value and say so on screen.
    Unavailable,
}

/// iHOTEL room-status read, behind a trait so `routes::hk` can be tested
/// against scripted answers with no MSSQL anywhere near CI.
///
/// `Debug` is required so [`crate::routes::hk::HkPolicy`] keeps its derive.
#[async_trait]
pub trait RoomCleanSource: Send + Sync + fmt::Debug {
    /// Read every room's current legacy cleanliness. Never errors — an
    /// unreachable legacy is [`RoomCleanOutcome::Unavailable`], which is a
    /// first-class, displayable state rather than an exception.
    async fn room_clean(&self) -> RoomCleanOutcome;
}

/// Legacy `Room_Clean` literal → canonical `is_clean`. PURE — unit-tested.
///
/// INVERTED on purpose (see the module table). Unrecognised ⇒ `None` = unknown.
/// Trimmed but NOT case-folded: `sync::mappers::room::legacy_yesno_to_bool`
/// matches the two lowercase literals exactly, and this surface must not
/// interpret a value the sync mapper would refuse — the two would then
/// disagree about the same row and the divergence log would cry wolf.
pub fn legacy_clean_to_is_clean(raw: Option<&str>) -> Option<bool> {
    match raw.map(str::trim) {
        Some("no") => Some(true),   // no cleaning needed ⇒ IS clean
        Some("yes") => Some(false), // needs cleaning ⇒ dirty
        _ => None,
    }
}

/// The per-branch iHOTEL readers, in a shape that can be shared with surfaces
/// OUTSIDE `routes::hk`.
///
/// `HkPolicy` owns the authoritative map (it is the thing `main.rs` attaches
/// readers to at startup). Reception's board needs the SAME readers — the whole
/// point of the wave-5 change is that the maid and the receptionist read one
/// truth — but it lives on the main router, which has no `HkPolicy`. Rather
/// than build a second set of readers (a second legacy connection per site, and
/// two things to keep in step), `main.rs` clones the map into this newtype and
/// layers it as an `Extension` on the main router.
///
/// A newtype rather than a bare `BTreeMap` so the axum `Extension` is keyed on
/// a type that means something: a second `BTreeMap<&str, Arc<…>>` extension
/// would silently collide with this one.
///
/// Keys are the wire branch ids (`hfhotel` / `hfville`) — the `?branch=`
/// spelling, so the reader and the PG pool are chosen off the same token.
#[derive(Clone, Debug, Default)]
pub struct RoomCleanReaders(BTreeMap<&'static str, Arc<dyn RoomCleanSource>>);

impl RoomCleanReaders {
    pub fn new(readers: BTreeMap<&'static str, Arc<dyn RoomCleanSource>>) -> Self {
        Self(readers)
    }

    /// Branch ids that have a live reader — for the startup log.
    pub fn branches(&self) -> Vec<&'static str> {
        self.0.keys().copied().collect()
    }

    /// Ask a branch's reader, or report [`RoomCleanOutcome::Unavailable`] when
    /// that branch has none.
    ///
    /// "No reader configured" and "reader failed" collapse to the SAME answer,
    /// deliberately and identically to `routes::hk`'s
    /// `resolve_legacy_room_clean`: both mean the caller is about to render the
    /// canonical PG mirror, and the screen must say so either way. Two surfaces
    /// disagreeing about what "stale" means would be worse than either being
    /// wrong.
    pub async fn read(&self, branch_id: &str) -> RoomCleanOutcome {
        match self.0.get(branch_id) {
            Some(source) => source.room_clean().await,
            None => {
                tracing::debug!(
                    branch = branch_id,
                    "no iHOTEL room-status reader for this branch — serving the \
                     canonical PG mirror with the stale flag"
                );
                RoomCleanOutcome::Unavailable
            }
        }
    }
}

/// The live reader: one `SELECT` against a site's legacy `HT_Rooms`.
///
/// Holds a [`DbPool`] it does NOT own — at HF Hotel it is the very pool
/// `main.rs` already builds for the scheduler's reconcile backstop, so the
/// shipping branch adds NO new connection plumbing (reuse over new plumbing).
/// The pool's `PoisonAwareManager` means a timed-out read marks its connection
/// broken instead of returning a desynced stream to the shared pool.
pub struct MssqlRoomCleanSource {
    pool: DbPool,
    /// `hfhotel` / `hfville` — log field only, so an operator can tell the two
    /// readers apart in one grep.
    site: &'static str,
}

impl fmt::Debug for MssqlRoomCleanSource {
    /// Hand-written: `bb8::Pool`'s own `Debug` is not part of our contract and
    /// nothing about a connection pool belongs in a startup log line.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MssqlRoomCleanSource")
            .field("site", &self.site)
            .finish()
    }
}

impl MssqlRoomCleanSource {
    pub fn new(pool: DbPool, site: &'static str) -> Self {
        Self { pool, site }
    }
}

#[async_trait]
impl RoomCleanSource for MssqlRoomCleanSource {
    async fn room_clean(&self) -> RoomCleanOutcome {
        // ONE outer bound over acquire + query. The pool's own
        // `connection_timeout` is 5s — longer than this whole call is allowed
        // to take — so bounding only the query would let a wedged legacy hold
        // the maid's list for 5s before the query budget even started.
        let work = async {
            let mut conn = match self.pool.get().await {
                Ok(conn) => conn,
                Err(err) => {
                    tracing::warn!(
                        site = self.site,
                        error = %err,
                        "iHOTEL room-status read: pool acquire failed — \
                         falling back to the canonical PG value"
                    );
                    return RoomCleanOutcome::Unavailable;
                }
            };
            let rows = match simple_query_with_explicit_timeout(
                &mut conn,
                ROOM_STATUS_SQL,
                QUERY_TIMEOUT,
            )
            .await
            {
                Ok(rows) => rows,
                Err(err) => {
                    tracing::warn!(
                        site = self.site,
                        error = %err,
                        "iHOTEL room-status read failed — falling back to the \
                         canonical PG value"
                    );
                    return RoomCleanOutcome::Unavailable;
                }
            };

            let mut map = HashMap::with_capacity(rows.len());
            let mut unknown = 0usize;
            for row in &rows {
                let Some(room_no) = row
                    .get::<&str, _>("Room_no")
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                else {
                    continue;
                };
                match legacy_clean_to_is_clean(row.get::<&str, _>("Room_Clean")) {
                    Some(is_clean) => {
                        map.insert(room_no.to_string(), is_clean);
                    }
                    // Unrecognised literal: leave the room OUT of the map so
                    // the merge keeps its PG value. Counted, not per-row
                    // logged — a schema drift would otherwise emit 58 lines.
                    None => unknown += 1,
                }
            }
            if unknown > 0 {
                tracing::warn!(
                    site = self.site,
                    unknown,
                    "iHOTEL room-status read: rooms with an unrecognised \
                     Room_Clean literal kept their canonical PG value"
                );
            }
            RoomCleanOutcome::Available(map)
        };

        match tokio::time::timeout(ROOM_STATUS_TIMEOUT, work).await {
            Ok(outcome) => outcome,
            Err(_elapsed) => {
                tracing::warn!(
                    site = self.site,
                    budget_ms = ROOM_STATUS_TIMEOUT.as_millis() as u64,
                    "iHOTEL room-status read exceeded its budget — falling back \
                     to the canonical PG value"
                );
                RoomCleanOutcome::Unavailable
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The inversion, both poles. This is the single most breakable fact in
    /// the module: getting it backwards shows every clean room as dirty.
    #[test]
    fn legacy_no_is_clean_and_yes_is_dirty() {
        assert_eq!(legacy_clean_to_is_clean(Some("no")), Some(true));
        assert_eq!(legacy_clean_to_is_clean(Some("yes")), Some(false));
    }

    /// Whitespace is trimmed — legacy `varchar` columns are padded in places.
    #[test]
    fn literals_are_trimmed() {
        assert_eq!(legacy_clean_to_is_clean(Some("  no  ")), Some(true));
        assert_eq!(legacy_clean_to_is_clean(Some("yes ")), Some(false));
    }

    /// Everything else is UNKNOWN, never a guess. Case included: the sync
    /// mapper matches lowercase exactly, and this surface must not disagree
    /// with it about the same row.
    #[test]
    fn unrecognised_literals_are_unknown_not_guessed() {
        for raw in [None, Some(""), Some("   "), Some("Yes"), Some("NO"), Some("y"), Some("1")] {
            assert_eq!(
                legacy_clean_to_is_clean(raw),
                None,
                "{raw:?} must be UNKNOWN — an unknown room keeps its PG value"
            );
        }
    }

    /// The SQL is a bare SELECT. Pinned so a future edit cannot quietly turn
    /// the maid's read path into a legacy WRITE (invariant #6) or add a lock
    /// hint on the shared production server.
    #[test]
    fn the_only_statement_is_a_read() {
        let sql = ROOM_STATUS_SQL.to_ascii_uppercase();
        assert!(sql.starts_with("SELECT"), "{ROOM_STATUS_SQL}");
        for forbidden in [
            "UPDATE", "INSERT", "DELETE", "MERGE", "BEGIN", "COMMIT", "NOLOCK", "TABLOCKX",
        ] {
            assert!(
                !sql.contains(forbidden),
                "{ROOM_STATUS_SQL} must not contain {forbidden}"
            );
        }
    }

    /// The query budget must sit strictly inside the whole-call budget, or the
    /// outer timeout is the only one that ever fires and the acquire gets no
    /// headroom.
    #[test]
    fn query_budget_is_inside_the_call_budget() {
        assert!(QUERY_TIMEOUT < ROOM_STATUS_TIMEOUT);
    }
}
