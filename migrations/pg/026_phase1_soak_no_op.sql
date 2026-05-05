-- Migration: 026_phase1_soak_no_op
-- Version: 2.57.2
-- Date: 2026-05-05
--
-- Pure no-op migration. Exists to exercise the Phase 1 deploy pipeline's
-- drift-check + migrate.sh paths without touching any prod schema. The
-- backend filter trigger fires because this file lives in `migrations/pg/`,
-- which causes `build-backend` to run (cargo doesn't actually change, so the
-- new image is functionally identical, just a different SHA — exactly the
-- scenario we want to verify recreates the container correctly).
--
-- Safe to leave in the schema permanently (the `SELECT 1 WHERE FALSE`
-- returns no rows and modifies nothing) or remove later — migrate.sh will
-- just see an extra schema_migrations row, which is benign.

-- UP MIGRATION
SELECT 1 WHERE FALSE;

-- DOWN MIGRATION (commented)
-- (no-op, nothing to roll back)
