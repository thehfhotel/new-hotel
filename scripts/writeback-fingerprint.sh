#!/usr/bin/env bash
# Regenerate the writeback worker's expected schema fingerprint.
#
# Per `docs/architecture.md` §"4e. Schema fingerprint guard" "refuse to write and alert"
# (this used to cite section 4e by number alone, which is ambiguous — architecture.md
# has two headings numbered 4e) + hotel-backend/src/writeback/fingerprint.rs:
# the writeback worker refuses to start if MSSQL's columns drift from the
# captured baseline. When a vendor change legitimately alters the schema:
#
#   1. Re-capture the schema (one of):
#        a. Re-run the spike monitor, or
#        b. Manually pull `INFORMATION_SCHEMA.COLUMNS` for the 10 tables.
#   2. Update `EXPECTED_SCHEMA_BASELINE` in fingerprint.rs with the new tuples.
#   3. Run this script to compute the new SHA-256.
#   4. Paste the new value into `EXPECTED_FINGERPRINT` (same file).
#   5. Run `cargo test fingerprint_constant_matches_baseline` to verify.
#   6. Commit fingerprint.rs in the same change as the schema baseline doc.
#
# This script just runs the dedicated test — failures print the new value
# you should paste into the constant.
#
# Usage:  ./scripts/writeback-fingerprint.sh

set -euo pipefail

cd "$(dirname "$0")/.."

echo "Computing fingerprint of EXPECTED_SCHEMA_BASELINE…"
echo "If this test fails, the assertion message includes the new fingerprint"
echo "value to paste into EXPECTED_FINGERPRINT."
echo

cd hotel-backend
SQLX_OFFLINE=true cargo test --lib \
  writeback::fingerprint::tests::fingerprint_constant_matches_baseline \
  -- --nocapture
