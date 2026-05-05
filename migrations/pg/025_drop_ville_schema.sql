-- Migration: 025_drop_ville_schema
-- Version: 2.55.1
-- Date: 2026-04-30
-- Description: Retire the obsolete `ville` schema in `hotelnew`.
--
-- Migration 010 (v2.22.0) created `hotelnew.ville.*` as a local cache
-- written by the FreeTDS-based `ville_sync` worker on the HF Ville
-- jumpbox. Phase 5 Ville cutover (#76, 2026-04-30) repointed the
-- backend's `ville_pool` away from this cache to the new `hotelville`
-- PG database (sibling of `hotelnew` in the same `newdb` cluster), fed
-- by the central `sync-hfville` Change-Tracking watcher.
--
-- After a 1-week soak window of clean cutover operation, the `ville`
-- schema in `hotelnew` is no longer read by any backend code path and
-- has been stale since `ville_sync` was stopped pre-cutover. Task #77
-- retires the worker (deletes `bin/ville_sync.rs`,
-- `Dockerfile.ville-sync`, `deploy/hfville/`, the `build-ville-sync` +
-- `deploy-hfville` workflow jobs, and the `<wg-self>:5441` host-port
-- mapping that exposed `newdb` for the now-defunct push). This
-- migration drops the orphaned schema so the canonical `hotelnew`
-- database matches its post-retirement reality.
--
-- Note: migration 010's file stays in the repo for archaeology; this
-- migration IS its rollback companion.

-- UP MIGRATION

DROP SCHEMA IF EXISTS ville CASCADE;

-- DOWN MIGRATION (commented; use migration 010's body to re-create the
-- schema if a rollback is ever needed — but be aware the data flowing
-- into it depended on the retired ville_sync worker, which would also
-- need to be re-introduced for the schema to be useful again).
-- See migrations/pg/010_ville_cache_schema.sql for the original DDL.
-- DELETE FROM schema_migrations WHERE version = '025';
