#!/usr/bin/env bash
#
# smoke-ville.sh — HF Ville connectivity smoke check.
#
# Actively probes the backend's /api/health/ville endpoint (which runs a live
# `SELECT 1` against the HF Ville pool) and reports whether HF Ville is
# reachable RIGHT NOW — not just whether the pool existed at startup.
#
# Why this exists: ville_pool is built fail-soft at boot (hotel-backend/src/
# main.rs) and never re-checked, so a Ville tunnel/DB that was down at the last
# backend restart leaves HF Ville silently disabled. Run this on the host (or
# wire it into monitoring / the post-deploy step) to catch that.
#
# Usage:
#   scripts/smoke-ville.sh                  # checks http://localhost:3003
#   BASE_URL=http://backend:3003 scripts/smoke-ville.sh
#
# Exit codes: 0 = connected, 1 = not connected / unreachable, 2 = bad response.
set -euo pipefail

BASE_URL="${BASE_URL:-http://localhost:3003}"
URL="${BASE_URL%/}/api/health/ville"

echo "[smoke-ville] GET ${URL}"
body="$(curl -fsS --max-time 10 "${URL}" 2>/dev/null || true)"

if [ -z "${body}" ]; then
  echo "[smoke-ville] FAIL: no response from ${URL} (backend down or route missing?)" >&2
  exit 2
fi
echo "[smoke-ville] response: ${body}"

# Prefer jq; fall back to a grep heuristic so the script works on bare hosts.
if command -v jq >/dev/null 2>&1; then
  connected="$(printf '%s' "${body}" | jq -r '.connected // false')"
else
  if printf '%s' "${body}" | grep -q '"connected":[[:space:]]*true'; then
    connected="true"
  else
    connected="false"
  fi
fi

if [ "${connected}" = "true" ]; then
  echo "[smoke-ville] OK: HF Ville is connected."
  exit 0
fi

echo "[smoke-ville] FAIL: HF Ville is NOT connected." >&2
echo "[smoke-ville] Likely causes: VILLE_DB_ENABLED not 'true', the ville tunnel/DB is down," >&2
echo "[smoke-ville] or the pool failed at the last backend start (a restart re-attempts the connect)." >&2
exit 1
