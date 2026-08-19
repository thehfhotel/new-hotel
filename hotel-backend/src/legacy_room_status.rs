//! iHOTEL room-status READ for the maid surface (CR-1, owner decision
//! locked 2026-08-15).
//!
//! ## Two facts, one read
//!
//! One `SELECT` answers BOTH questions the maid's list asks about a room:
//!
//! | legacy column | fact                              | polarity          |
//! |---------------|-----------------------------------|-------------------|
//! | `Room_Clean`  | [`LegacyRoomFlags::is_clean`]     | INVERTED (see below) |
//! | `Room_Use`    | [`LegacyRoomFlags::occupied`]     | NOT inverted (`'yes'` = occupied) |
//!
//! They are carried per-fact-optional ([`LegacyRoomFlags`]) rather than as one
//! all-or-nothing answer: a room whose `Room_Clean` is junk but whose
//! `Room_Use` reads cleanly must still get iHOTEL's OCCUPANCY, and vice versa.
//! Widening the read costs nothing — same row, same round-trip, same 34-58 row
//! scan.
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
//! `Room_Use` is the OCCUPANCY flag and is **not** inverted: `'yes'` = a guest
//! is in the room, `'no'` = vacant. Same two-literal alphabet, same
//! `None`-is-unknown rule ([`legacy_use_to_occupied`] mirrors
//! [`legacy_clean_to_is_clean`] exactly apart from the inversion). Verified
//! against BOTH production sites 2026-08-19: `SELECT Room_Use, COUNT(*) FROM
//! HT_Rooms GROUP BY Room_Use` returns only `'no'` / `'yes'`, untrimmed-clean
//! and lowercase, at HF Hotel (47/11 of 58) and HF Ville (18/16 of 34). There
//! is no third literal to widen the match for.
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
//! literally: `routes::hk` depends on the [`RoomFlagsSource`] TRAIT, never on
//! tiberius, and the decommission story is unchanged — drop the reader, the
//! surface falls back to canonical PG on its own.
//!
//! ## Fail-soft contract (the opposite of `hfid_location`)
//!
//! [`hfid_location`](crate::hfid_location) fails CLOSED because answering
//! wrongly there sends a maid to the wrong hotel. This one fails SOFT
//! ([`RoomFlagsOutcome::Unavailable`] ⇒ show the PG mirror plus a visible Thai
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
/// bound is still authoritative — see [`MssqlRoomFlagsSource::room_flags`].
const QUERY_TIMEOUT: Duration = Duration::from_millis(2_500);

/// The one statement this module issues. READ-ONLY.
pub const ROOM_STATUS_SQL: &str = "SELECT Room_no, Room_Clean, Room_Use FROM HT_Rooms";

/// Per-room iHOTEL flags. Per-fact `None` = unrecognised literal — that FACT
/// keeps its canonical value; the other fact still wins per CR-1.
///
/// Per-fact rather than per-room on purpose: the two columns are independent
/// literals written by different iHOTEL code paths, and one of them being junk
/// is no reason to hand the maid a stale answer for the other.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LegacyRoomFlags {
    /// From `Room_Clean`, INVERTED (`'no'` = clean). Unchanged semantics.
    pub is_clean: Option<bool>,
    /// From `Room_Use`, NOT inverted (`'yes'` = occupied).
    pub occupied: Option<bool>,
}

/// What iHOTEL says about a room's cleanliness AND occupancy right now.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoomFlagsOutcome {
    /// iHOTEL answered. Map is `Room_no` (trimmed) → [`LegacyRoomFlags`], with
    /// `is_clean` already in CANONICAL polarity (inverted; `true` = IS clean).
    /// A room enters the map when AT LEAST ONE fact parsed; a room whose
    /// `Room_Clean` AND `Room_Use` were both NULL / blank / unrecognised is
    /// ABSENT rather than guessed — and an absent room keeps its PG values.
    Available(HashMap<String, LegacyRoomFlags>),
    /// No answer was obtained: no reader configured for this branch, pool
    /// acquire failed, the query errored, or the budget elapsed. The caller
    /// MUST fall back to the canonical PG values and say so on screen.
    Unavailable,
}

/// iHOTEL room-status read, behind a trait so `routes::hk` can be tested
/// against scripted answers with no MSSQL anywhere near CI.
///
/// `Debug` is required so [`crate::routes::hk::HkPolicy`] keeps its derive.
#[async_trait]
pub trait RoomFlagsSource: Send + Sync + fmt::Debug {
    /// Read every room's current legacy cleanliness + occupancy. Never errors
    /// — an unreachable legacy is [`RoomFlagsOutcome::Unavailable`], which is a
    /// first-class, displayable state rather than an exception.
    async fn room_flags(&self) -> RoomFlagsOutcome;
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

/// Legacy `Room_Use` literal → `occupied`. PURE — unit-tested.
///
/// NOT inverted (contrast [`legacy_clean_to_is_clean`]): `'yes'` means a guest
/// is in the room. Otherwise this mirrors `legacy_clean_to_is_clean` /
/// `sync::mappers::room::legacy_yesno_to_bool` exactly — trimmed, matched
/// against the two LOWERCASE literals only, everything else `None` = unknown.
///
/// Case-sensitive on purpose, for the same reason as its sibling: the sync
/// mapper refuses `'Yes'`, and a surface that interpreted a literal the mapper
/// would refuse would disagree with the mirror about the very row the
/// divergence log is meant to flag.
pub fn legacy_use_to_occupied(raw: Option<&str>) -> Option<bool> {
    match raw.map(str::trim) {
        Some("yes") => Some(true), // in use ⇒ occupied
        Some("no") => Some(false), // not in use ⇒ vacant
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
pub struct RoomFlagsReaders(BTreeMap<&'static str, Arc<dyn RoomFlagsSource>>);

impl RoomFlagsReaders {
    pub fn new(readers: BTreeMap<&'static str, Arc<dyn RoomFlagsSource>>) -> Self {
        Self(readers)
    }

    /// Branch ids that have a live reader — for the startup log.
    pub fn branches(&self) -> Vec<&'static str> {
        self.0.keys().copied().collect()
    }

    /// Ask a branch's reader, or report [`RoomFlagsOutcome::Unavailable`] when
    /// that branch has none.
    ///
    /// "No reader configured" and "reader failed" collapse to the SAME answer,
    /// deliberately and identically to `routes::hk`'s
    /// `resolve_legacy_room_flags`: both mean the caller is about to render the
    /// canonical PG mirror, and the screen must say so either way. Two surfaces
    /// disagreeing about what "stale" means would be worse than either being
    /// wrong.
    pub async fn read(&self, branch_id: &str) -> RoomFlagsOutcome {
        match self.0.get(branch_id) {
            Some(source) => source.room_flags().await,
            None => {
                tracing::debug!(
                    branch = branch_id,
                    "no iHOTEL room-status reader for this branch — serving the \
                     canonical PG mirror with the stale flag"
                );
                RoomFlagsOutcome::Unavailable
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
pub struct MssqlRoomFlagsSource {
    pool: DbPool,
    /// `hfhotel` / `hfville` — log field only, so an operator can tell the two
    /// readers apart in one grep.
    site: &'static str,
}

impl fmt::Debug for MssqlRoomFlagsSource {
    /// Hand-written: `bb8::Pool`'s own `Debug` is not part of our contract and
    /// nothing about a connection pool belongs in a startup log line.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MssqlRoomFlagsSource")
            .field("site", &self.site)
            .finish()
    }
}

impl MssqlRoomFlagsSource {
    pub fn new(pool: DbPool, site: &'static str) -> Self {
        Self { pool, site }
    }
}

#[async_trait]
impl RoomFlagsSource for MssqlRoomFlagsSource {
    async fn room_flags(&self) -> RoomFlagsOutcome {
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
                    return RoomFlagsOutcome::Unavailable;
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
                    return RoomFlagsOutcome::Unavailable;
                }
            };

            let mut map = HashMap::with_capacity(rows.len());
            let mut unknown_clean = 0usize;
            let mut unknown_use = 0usize;
            for row in &rows {
                let Some(room_no) = row
                    .get::<&str, _>("Room_no")
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                else {
                    continue;
                };
                let is_clean = legacy_clean_to_is_clean(row.get::<&str, _>("Room_Clean"));
                let occupied = legacy_use_to_occupied(row.get::<&str, _>("Room_Use"));
                if is_clean.is_none() {
                    unknown_clean += 1;
                }
                if occupied.is_none() {
                    unknown_use += 1;
                }
                // A room enters the map when AT LEAST ONE fact parsed — a junk
                // `Room_Clean` must not cost the maid iHOTEL's occupancy, nor
                // the reverse. Both unrecognised ⇒ absent, so the merge keeps
                // BOTH canonical values.
                if is_clean.is_some() || occupied.is_some() {
                    map.insert(room_no.to_string(), LegacyRoomFlags { is_clean, occupied });
                }
            }
            if unknown_clean > 0 || unknown_use > 0 {
                // ONE line, two counters. Counted rather than per-row logged —
                // a schema drift would otherwise emit 58 lines per room-list
                // load, and the two facts drift independently.
                tracing::warn!(
                    site = self.site,
                    unknown_clean,
                    unknown_use,
                    "iHOTEL room-status read: rooms with an unrecognised \
                     Room_Clean / Room_Use literal kept their canonical PG value \
                     for that fact"
                );
            }
            RoomFlagsOutcome::Available(map)
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
                RoomFlagsOutcome::Unavailable
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

    /// `Room_Use` is NOT inverted, both poles. The mirror image of
    /// `legacy_no_is_clean_and_yes_is_dirty`, and pinned separately BECAUSE
    /// the two columns disagree about what `'yes'` means: `Room_Clean='yes'`
    /// is dirty, `Room_Use='yes'` is occupied. Copy-pasting the inversion into
    /// this one shows every occupied room as vacant — a maid walking in on a
    /// guest.
    #[test]
    fn legacy_use_yes_is_occupied_and_no_is_vacant() {
        assert_eq!(legacy_use_to_occupied(Some("yes")), Some(true));
        assert_eq!(legacy_use_to_occupied(Some("no")), Some(false));
    }

    /// Trimmed, exactly like its sibling.
    #[test]
    fn use_literals_are_trimmed() {
        assert_eq!(legacy_use_to_occupied(Some("  yes  ")), Some(true));
        assert_eq!(legacy_use_to_occupied(Some("no ")), Some(false));
    }

    /// Same unknown set as `legacy_clean_to_is_clean`, matched literal for
    /// literal. `'Yes'` / `'YES'` are UNKNOWN, not occupied: this function is
    /// specified as a mirror of `legacy_yesno_to_bool`, which is case-SENSITIVE
    /// — and a surface that read a literal the sync mapper refuses would
    /// disagree with the mirror about the very row it is meant to reconcile.
    ///
    /// Production says there is nothing to widen for: `Room_Use` holds only
    /// `'no'` / `'yes'` at both sites (verified 2026-08-19).
    #[test]
    fn unrecognised_use_literals_are_unknown_not_guessed() {
        for raw in [None, Some(""), Some("Yes"), Some("YES"), Some("y"), Some("1")] {
            assert_eq!(
                legacy_use_to_occupied(raw),
                None,
                "{raw:?} must be UNKNOWN — occupancy then keeps its canonical value"
            );
        }
    }

    /// The two mappings share an alphabet but NOT a polarity. Pinned as one
    /// assertion so a refactor that "unifies" them fails here rather than in
    /// production.
    #[test]
    fn the_two_facts_read_the_same_literal_oppositely() {
        assert_eq!(legacy_clean_to_is_clean(Some("yes")), Some(false));
        assert_eq!(legacy_use_to_occupied(Some("yes")), Some(true));
    }

    /// The exact statement, byte for byte. The widening from one column to two
    /// is the change this module shipped; pinning the literal means a future
    /// edit that drops `Room_Use` (silently reverting every maid's occupancy to
    /// the canonical fallback) fails a test instead of shipping quiet.
    #[test]
    fn the_statement_reads_both_facts() {
        assert_eq!(
            ROOM_STATUS_SQL,
            "SELECT Room_no, Room_Clean, Room_Use FROM HT_Rooms"
        );
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
