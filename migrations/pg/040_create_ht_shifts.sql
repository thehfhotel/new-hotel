-- Migration: 040_create_ht_shifts
-- Version: 2.63.16
-- Date: 2026-05-13
-- Description: Track F2 / T1 HIGH-5 — create the `ht_shifts` canonical table
--              so the receptionist app can gate payments behind an open
--              cashier shift, mirroring iHOTEL's `HT_Round_Bill` discipline.
--
-- Why: iHOTEL refuses to accept any payment unless a shift is open. Our
-- app today accepts payments unconditionally, which makes cash-drawer
-- reconciliation impossible (T1 HIGH-5 in
-- `docs/coexistence/audit-2026-05-13.md`).
--
-- F2 lands the PG canonical + a service-layer gate in
-- `service::payment::record_payment`. The legacy `HT_Round_Bill` mirror
-- writeback + CT-side sync are deferred to a follow-up (Track G, the
-- "round-bill shift discipline" feature) — F2 is the operational win
-- that stops the cash-drawer bleeding while the legacy bridge is built.
--
-- Cardinality with legacy `HT_Round_Bill` is `1:1` per
-- `docs/coexistence/CARDINALITY_MAP.md`. PG is the source of truth from
-- day one; the legacy mirror exists for iHOTEL's report compatibility
-- only.
--
-- Invariant: at most one open shift per site. Enforced by the partial
-- unique index `ht_shifts_one_open_per_site` (PG can index a predicate,
-- which is exactly what we need for "WHERE closed_at IS NULL").
--
-- Idempotent: every CREATE uses `IF NOT EXISTS`.

-- UP MIGRATION

CREATE TABLE IF NOT EXISTS ht_shifts (
    shift_id              BIGSERIAL PRIMARY KEY,
    -- Site this shift belongs to. Matches `SiteConfig::id` literals
    -- (`hfhotel` / `hfville` today). VARCHAR(50) leaves headroom for
    -- future sites without another migration.
    shift_site_id         VARCHAR(50) NOT NULL,
    -- Sequential round number within the site. Service layer assigns
    -- `max(shift_no) + 1` per (shift_site_id) on open. Mirrors
    -- iHOTEL's `HT_Round_Bill.Round_No` semantics.
    shift_no              INTEGER NOT NULL,
    -- Opening cash float (baht). NUMERIC(12,2) matches the rest of
    -- the canonical money columns.
    shift_opening_float   NUMERIC(12,2) NOT NULL DEFAULT 0,
    -- Cashier username/initials. Routes populate from auth context.
    shift_opened_by       VARCHAR(50) NOT NULL,
    shift_opened_at       TIMESTAMPTZ NOT NULL,
    -- NULL until the shift is closed. The partial index below relies
    -- on this column to enforce one-open-per-site.
    shift_closed_at       TIMESTAMPTZ,
    shift_closed_by       VARCHAR(50),
    -- Legacy `HT_Round_Bill.id` once the writeback mapper lands. Kept
    -- nullable so PG-canonical-only operation works today (deferred to
    -- Track G follow-up).
    shift_legacy_round_id INTEGER,
    shift_notes           TEXT,
    shift_created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (shift_site_id, shift_no)
);

-- One-open-shift-per-site invariant. PG partial unique indexes give us
-- exactly the right primitive: the constraint is enforced only on rows
-- where shift_closed_at IS NULL, so the same site can have many closed
-- historical shifts but only one currently-open round.
CREATE UNIQUE INDEX IF NOT EXISTS ht_shifts_one_open_per_site
    ON ht_shifts (shift_site_id)
    WHERE shift_closed_at IS NULL;

-- Listing recent shifts is the dominant read pattern (cash-drawer
-- review). Index `(site, opened_at DESC)` covers the
-- "give me the last N shifts for site X" query.
CREATE INDEX IF NOT EXISTS ix_ht_shifts_site_opened_at
    ON ht_shifts (shift_site_id, shift_opened_at DESC);

-- DOWN MIGRATION (commented — deletion would lose audit trail for any
-- shift activity that has already accumulated):
-- DROP INDEX IF EXISTS ix_ht_shifts_site_opened_at;
-- DROP INDEX IF EXISTS ht_shifts_one_open_per_site;
-- DROP TABLE IF EXISTS ht_shifts;
