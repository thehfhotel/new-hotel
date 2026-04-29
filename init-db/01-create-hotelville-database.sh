#!/bin/bash
# =============================================================================
# Create the `hotelville` database on first PostgreSQL container start.
# =============================================================================
# Per ADR 0001 (Phase 5 Ville multi-site): per-DB topology means HF Hotel and
# HF Ville share the same PG cluster but live in separate logical databases:
#
#   hotelnew    — HF Hotel canonical state (existing)
#   hotelville  — HF Ville canonical state (new in Phase 5 Ville)
#
# Each DB gets its own legacy_ct_state row, its own legacy_sync_status, its
# own ht_reconcile_log etc. The single-row CHECK constraints (e.g.
# `legacy_ct_state.id BIGINT PRIMARY KEY DEFAULT 1 CHECK (id = 1)`) stay
# valid because each DB has its own copy.
#
# This script runs ONLY on first init of an empty data dir
# (Postgres docker-entrypoint-initdb.d convention). For existing clusters
# (evergreen production), the equivalent CREATE DATABASE was applied as a
# one-shot via scripts/create-hotelville-onetime.sh — see that file for the
# rationale and audit trail.
#
# Numbered `01-` so it runs AFTER the main init-hotelnew.sql (lexicographic
# order in /docker-entrypoint-initdb.d). The init for hotelnew is a .sql
# file; this is .sh so it can use psql against the just-created cluster.
# =============================================================================

set -euo pipefail

echo "==> Creating hotelville database alongside hotelnew..."

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" <<-EOSQL
    CREATE DATABASE hotelville;
EOSQL

echo "==> Applying init-hotelnew.sql to hotelville (same schema baseline)..."

# Re-use the same baseline script. The schema is identical between the two
# DBs at init time — divergence (if any) would only come from migrations
# ordered differently per site, which we don't do.
psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname hotelville \
    --file /docker-entrypoint-initdb.d/init-hotelnew.sql

echo "==> hotelville database created and initialised."
