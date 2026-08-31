-- Migration: 089_create_ht_hk_room_signals
-- Version: vNext
-- Date: 2026-09-01
-- Room signals between reception and maids (ADR 0008 —
-- `docs/adr/0008-room-signals-not-chat.md`, vocabulary in CONTEXT.md
-- §Housekeeping).
--
-- ## Background
--
-- The domain object is the ROOM SIGNAL, not the message: a canned, room-scoped
-- notice broadcast to the other role at that room's branch. There is
-- deliberately NO free-text column anywhere in this table — "canned-only" is
-- the decision ADR 0008 records, and a note column is the thin end of the chat
-- wedge the ADR rejects. Anything unanticipated stays a phone call.
--
-- Desk→maid types: room_check (ขอเช็คห้อง), priority_clean (ทำห้องนี้ก่อน),
-- deliver_linen (แขกขอผ้าเพิ่ม), skip_room (งดทำห้องนี้), checked_out
-- (แขกเช็คเอาท์แล้ว). Maid→desk types: guest_in_room (ลูกค้ายังอยู่ในห้อง),
-- found_belongings (พบของลืมในห้อง), item_missing (มีของหาย), item_damaged
-- (มีของเสียหาย).
--
-- - PG-CANONICAL ONLY, exactly as narrowly as 088's `ht_hk_linen_reports`.
--   iHOTEL has no counterpart to a room signal at all, so there is NO sync
--   mapper, NO writeback recipe and NO `WritebackIntent` — coexistence
--   invariant #6 is upheld by there being nothing legacy-coupled to gate.
--   Unlike `ht_hk_cleaning_events` (whose `done` phase grew a `MarkRoomClean`
--   writeback on 2026-08-11) this table has no path to legacy by design.
-- - It DOES publish domain events (`RoomSignalRaised` / `RoomSignalAcked` /
--   `RoomSignalCompleted` / `RoomSignalCancelled`) — but purely as UI event
--   plumbing over the existing `event_log` + `pg_notify('domain_events')`
--   fan-out that `routes::events` already serves to reception's board and now
--   also to the maid's `/hk` page (`GET /api/hk/events`). An event is not a
--   legacy write.
-- - The row IS the audit record behind guest charges: มีของหาย / มีของเสียหาย
--   stand as guest-accountability signals the desk resolves before settling
--   (ADR 0008 §Consequences). Rows are therefore never deleted by the app;
--   `cancelled` and `done` are terminal STATUSES, not deletions.
--
-- ## Lifecycle
--
-- open → acked (by name) → done (by name); the creator's SIDE may cancel while
-- still open. A signal is visible until done, whatever the day (the boards list
-- `sig_status IN ('open','acked')` — hence the partial index below).
--
-- Two completions are not a bare tap:
--
-- * a maid's เสร็จแล้ว cleaning report auto-completes that room's open/acked
--   `priority_clean` + `checked_out` signals IN THE SAME TRANSACTION as the
--   cleaning event (`service::housekeeping::report_cleaning_progress`),
--   stamped `sig_done_source = 'clean_report'`;
-- * `room_check` may NOT be completed by a tap at all — its completion is an
--   ANSWER (`POST /api/hk/signals/{id}/answer`): `clear`, or `problems` with
--   one or both of item_missing / item_damaged, which in the SAME transaction
--   also insert one standing maid→desk child signal per problem pointing back
--   at the check via `sig_parent_id`. Stamped `'room_check_answer'`.
--
-- Site scoping is connection-level (both `hotelnew` and `hotelville` logical
-- DBs get this table; each site's pool holds its own signals) — same model as
-- `ht_hk_cleaning_events` / `ht_hk_linen_reports`. `scripts/migrate.sh --site
-- hfville` runs this same file against `hotelville`, and
-- `init-db/01-create-hotelville-database.sh` replays `init-hotelnew.sql` there
-- on a fresh cluster, so no per-site file is needed.

-- UP MIGRATION

CREATE TABLE IF NOT EXISTS ht_hk_room_signals (
    sig_id            BIGSERIAL   PRIMARY KEY,
    -- The ONE room this signal is about (site-local canonical room). FK +
    -- CASCADE mirrors `ht_hk_linen_reports.hklr_room_id` — the route 404s an
    -- unknown or inactive room before the insert, so this is a backstop, not
    -- the gate.
    sig_room_id       INTEGER     NOT NULL REFERENCES ht_rooms_new(room_id) ON DELETE CASCADE,
    -- Which way the signal travels. CHECKED, unlike `sig_type`: the direction
    -- is STRUCTURAL — every role rule ("nobody acts on their own direction's
    -- signals except cancel-own-while-open") is expressed over exactly these
    -- two values, and a third would not mean "a new product option", it would
    -- mean the role model no longer holds.
    sig_direction     TEXT        NOT NULL CHECK (sig_direction IN ('desk_to_maid', 'maid_to_desk')),
    -- Which canned signal. DELIBERATELY plain TEXT with NO CHECK enumerating
    -- the types — the same decision, for the same reason, as
    -- `ht_hk_linen_reports.hklr_kind` (migration 088). The allowlist lives ONLY
    -- in the app (`domain::hk_signal::DESK_TO_MAID_TYPES` /
    -- `MAID_TO_DESK_TYPES`, mirroring `app/hk/signal-vocab.ts`), which is what
    -- the 400 for an unknown type is served from. ADR 0008 names extending the
    -- canned vocabulary as "the sanctioned cheap change" — so it must stay a
    -- one-line constant edit shipping with the frontend, with no ALTER on a
    -- live table at two sites and no window where the deployed binary and the
    -- deployed CHECK disagree about the valid set (exactly the coupling
    -- migration 087 had to unpick for `hkev_status`). The cost is that the DB
    -- would accept a type the app refuses; the app is the only writer, so that
    -- cost is not paid.
    sig_type          TEXT        NOT NULL,
    -- Lifecycle position. CHECKED, because statuses ARE structural: every
    -- transition rule, the boards' visibility predicate, the escalation
    -- eligibility scan and the auto-complete sweep are all written over
    -- exactly this closed set. A new status is a redesign, not a vocabulary
    -- addition — the opposite of `sig_type`.
    sig_status        TEXT        NOT NULL DEFAULT 'open'
                                  CHECK (sig_status IN ('open', 'acked', 'done', 'cancelled')),
    -- The ขอเช็คห้อง answer: 'clear' (เคลียร์) or 'problems'. NULL for every
    -- other type and for a room_check that is not answered yet. No CHECK, for
    -- `sig_type`'s reason at one remove: the outcome vocabulary belongs to the
    -- room-check product decision, which the same app constant owns.
    sig_outcome       TEXT        NULL,
    -- Set on a child signal spawned by a `problems` answer: points at the
    -- room_check that produced it, so the desk can see WHY a มีของหาย is
    -- standing against this room. Self-referencing FK, NULL for every signal a
    -- person raised directly. Deliberately NO cascade: the parent is never
    -- deleted (see the audit note above), so a cascade rule would only
    -- describe a situation that cannot arise.
    sig_parent_id     BIGINT      NULL REFERENCES ht_hk_room_signals(sig_id),
    -- HF ID badge of whoever raised it (verified identity, never client-typed)
    -- plus the display-name snapshot — same convention as
    -- `ht_hk_cleaning_events.hkev_badge` / `hkev_name` and
    -- `ht_hk_linen_reports.hklr_badge` / `hklr_name`, and deliberately NO FK to
    -- `ht_users`: maids are CF Access + HF ID identities, not PMS accounts.
    -- The desk surface stamps its own operator label the same way.
    sig_created_badge TEXT        NOT NULL,
    sig_created_name  TEXT,
    sig_created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    -- Who took it (the ack answers "who's on it"), and who finished it. Both
    -- NULL until the corresponding transition happens.
    sig_acked_badge   TEXT,
    sig_acked_name    TEXT,
    sig_acked_at      TIMESTAMPTZ,
    sig_done_badge    TEXT,
    sig_done_name     TEXT,
    sig_done_at       TIMESTAMPTZ,
    -- HOW it was completed: 'tap' (someone pressed done), 'clean_report' (a
    -- maid's เสร็จแล้ว auto-completed it) or 'room_check_answer' (the ขอเช็คห้อง
    -- answer completed the check). NULL until done. No CHECK — same app-owned
    -- vocabulary argument as `sig_type`; the set is written in
    -- `domain::hk_signal::SignalDoneSource`.
    sig_done_source   TEXT        NULL,
    -- Stamped ONCE when the 2-minute unacked-room_check escalation was
    -- delivered to HF ID (any 2xx, INCLUDING `{sent:false, reason:...}` —
    -- "nobody on duty" is a definitive answer, not a failure to retry). NULL
    -- means "not escalated": either not eligible yet, or the POST failed and
    -- the next 30s tick will try again. It is also the monthly-quota ledger —
    -- `COUNT(sig_escalated_at)` in the current Bangkok month is what the
    -- `HK_ESCALATION_MONTHLY_CAP` hard stop reads, which is why the counter
    -- lives on the row rather than in a separate table (ADR 0008 §Decision 3:
    -- "a monthly push counter with a hard stop makes silent quota burn
    -- impossible").
    sig_escalated_at  TIMESTAMPTZ NULL
);

-- The boards' hot path: both surfaces list `sig_status IN ('open','acked')`
-- for a branch and group by room. PARTIAL on exactly that predicate — done and
-- cancelled rows are history that only the audit reads, and they are the ones
-- that grow without bound, so keeping them out of the index keeps it the size
-- of the live board forever.
CREATE INDEX IF NOT EXISTS ix_ht_hk_room_signals_live
    ON ht_hk_room_signals (sig_status, sig_room_id)
    WHERE sig_status IN ('open', 'acked');

-- Per-room recent-first listing (room detail / audit follow-up), mirroring
-- `ix_ht_hk_linen_reports_room_created`.
CREATE INDEX IF NOT EXISTS ix_ht_hk_room_signals_room_created
    ON ht_hk_room_signals (sig_room_id, sig_created_at DESC);

COMMENT ON TABLE ht_hk_room_signals IS
    'Canned room signals between reception and maids (ADR 0008), migration 089. '
    'One room per signal, broadcast to the other role at that room''s branch; '
    'NO free-text column anywhere, by decision. Lifecycle open→acked→done, '
    'creator''s side may cancel while open. room_check completes only via its '
    'answer endpoint, which also spawns one child signal per problem '
    '(sig_parent_id). PG-CANONICAL ONLY: no legacy counterpart, no sync mapper, '
    'no writeback — the domain events it publishes are UI plumbing over the '
    'existing SSE fan-out. sig_type / sig_outcome / sig_done_source are TEXT '
    'with NO CHECK on purpose (app-owned vocabulary, 088 rationale); '
    'sig_direction and sig_status ARE checked because they are structural. '
    'sig_escalated_at is both the once-only escalation stamp and the monthly '
    'LINE-push quota ledger. Per-site (connection-level scoping).';

-- DOWN MIGRATION (commented — destructive)
-- DROP INDEX IF EXISTS ix_ht_hk_room_signals_room_created;
-- DROP INDEX IF EXISTS ix_ht_hk_room_signals_live;
-- DROP TABLE IF EXISTS ht_hk_room_signals;
-- DELETE FROM schema_migrations WHERE version = '089';
