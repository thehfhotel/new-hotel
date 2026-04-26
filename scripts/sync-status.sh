#!/usr/bin/env bash
# Phase 5 sync worker observability dashboard.
#
# Prints a sectioned, color-coded report of the CT watcher's state so you can
# decide whether to flip from shadow to live. Designed to be run from anywhere
# on your laptop — it SSHes to evergreen and queries PG directly.
#
# Usage:
#   scripts/sync-status.sh                # one-shot report
#   scripts/sync-status.sh --watch        # refresh every 30s (Ctrl-C to exit)
#   scripts/sync-status.sh --json         # machine-readable, single shot
#   scripts/sync-status.sh --readiness    # cutover readiness check (exit 0 = green)
#
# Sections:
#   1. Container status — sync, writeback, backend, newdb (running? healthy?)
#   2. Watermark + lag — how far behind MSSQL the watcher is (skipped if --no-mssql)
#   3. Per-table activity — rows_ingested, rows_skipped, errors, freshness
#   4. Reconcile drift — unresolved ht_reconcile_log entries by table
#   5. Cutover readiness — green/yellow/red checklist
#
# Exit codes:
#   0  — sync looks healthy (or --readiness check passed)
#   1  — degraded (warnings present, but not critical)
#   2  — broken (errors / not running / not enabled)

set -euo pipefail

# ─── Config ───────────────────────────────────────────────────────────────────
SSH_HOST="${SYNC_STATUS_SSH_HOST:-evergreen}"
PG_CONTAINER="${SYNC_STATUS_PG_CONTAINER:-new-hotel-db}"
PG_USER="${SYNC_STATUS_PG_USER:-postgres}"
PG_DATABASE="${SYNC_STATUS_PG_DATABASE:-hotelnew}"
SYNC_CONTAINER="${SYNC_STATUS_SYNC_CONTAINER:-new-hotel-production-sync-1}"
WATCH_INTERVAL_SECS="${SYNC_STATUS_WATCH_INTERVAL_SECS:-30}"

# ─── Args ─────────────────────────────────────────────────────────────────────
WATCH=0; JSON=0; READINESS=0
for arg in "$@"; do
    case "$arg" in
        -w|--watch) WATCH=1 ;;
        --json) JSON=1 ;;
        --readiness) READINESS=1 ;;
        -h|--help) grep '^#' "$0" | sed 's/^# \?//'; exit 0 ;;
        *) echo "unknown flag: $arg" >&2; exit 2 ;;
    esac
done

# ─── Color helpers (no color when piped or --json) ────────────────────────────
if [[ -t 1 && "$JSON" == "0" ]]; then
    R=$'\033[0;31m'; G=$'\033[0;32m'; Y=$'\033[0;33m'; B=$'\033[0;34m'; N=$'\033[0m'; D=$'\033[2m'
else
    R=""; G=""; Y=""; B=""; N=""; D=""
fi

# ─── PG query helper ──────────────────────────────────────────────────────────
pq() {
    # $1 = SQL; outputs tab-separated rows, no header, no footer
    ssh -o BatchMode=yes -o ConnectTimeout=10 "$SSH_HOST" \
        "docker exec $PG_CONTAINER psql -U $PG_USER -d $PG_DATABASE -At -F$'\t' -c \"$1\"" 2>/dev/null
}

# ─── Section 1: Container status ──────────────────────────────────────────────
section_containers() {
    echo "${B}── 1. Container status ───────────────────────────────────────────${N}"
    local services="backend sync writeback newdb"
    local out
    out=$(ssh -o BatchMode=yes "$SSH_HOST" \
        "cd ~/new-hotel-production && docker compose ps --format json $services 2>/dev/null" 2>/dev/null) || {
        echo "  ${R}✗ ssh to $SSH_HOST failed${N}"
        return 2
    }
    [[ -z "$out" ]] && { echo "  ${R}✗ no compose output (deploy dir wrong?)${N}"; return 2; }

    local rc=0
    echo "$out" | jq -rc '. | "\(.Service)\t\(.State)\t\(.Health // "n/a")\t\(.Status // "")"' 2>/dev/null | \
    while IFS=$'\t' read -r svc state health status; do
        local color="$G"; local icon="✓"
        [[ "$state" != "running" ]] && { color="$R"; icon="✗"; rc=2; }
        [[ "$health" == "unhealthy" ]] && { color="$R"; icon="✗"; rc=2; }
        [[ "$health" == "starting" ]] && { color="$Y"; icon="…"; }
        printf "  %s%s %-12s %-10s %-12s%s %s\n" "$color" "$icon" "$svc" "$state" "$health" "$N" "$D$status$N"
    done
    return $rc
}

# ─── Section 2: Watermark + activity freshness ────────────────────────────────
section_watermark() {
    echo "${B}── 2. CT watermark + freshness ───────────────────────────────────${N}"
    local row
    row=$(pq "SELECT last_seen_version, EXTRACT(EPOCH FROM (now() - last_polled_at))::int FROM legacy_ct_state WHERE id=1") || {
        echo "  ${R}✗ failed to query legacy_ct_state${N}"; return 2; }
    [[ -z "$row" ]] && { echo "  ${R}✗ legacy_ct_state row missing${N}"; return 2; }

    local last_seen secs_ago
    IFS=$'\t' read -r last_seen secs_ago <<< "$row"
    local ago_color="$G"; local ago_label="fresh"
    if (( secs_ago > 60 )); then ago_color="$R"; ago_label="STALE — worker not polling"; fi
    if (( secs_ago > 10 && secs_ago <= 60 )); then ago_color="$Y"; ago_label="slow"; fi

    printf "  Watermark:        ${G}%s${N}  (legacy_ct_state.last_seen_version)\n" "$last_seen"
    printf "  Last polled:      %s%ds ago${N}  (%s)\n" "$ago_color" "$secs_ago" "$ago_label"
    if (( last_seen == 0 )); then
        printf "  ${Y}⚠ watermark is 0 — bootstrap not run yet (./sync --bootstrap)${N}\n"
        return 1
    fi
    return 0
}

# ─── Section 3: Per-table activity ────────────────────────────────────────────
section_per_table() {
    echo "${B}── 3. Per-table activity (legacy_sync_status) ────────────────────${N}"
    printf "  %s%-22s %12s %12s %14s %10s %s%s\n" "$D" "table" "ingested" "skipped" "last_polled" "fails" "last_error" "$N"
    local table ing skip last fails err
    pq "SELECT table_name, rows_ingested, rows_skipped,
              COALESCE(EXTRACT(EPOCH FROM (now() - last_processed_at))::int::text || 's ago', '-'),
              consecutive_failures,
              COALESCE(LEFT(last_error, 50), '-')
        FROM legacy_sync_status ORDER BY table_name" | \
    while IFS=$'\t' read -r table ing skip last fails err; do
        local color="$G"
        if (( fails > 0 )); then color="$R"; fi
        if [[ "$err" != "-" && "$err" != "" ]]; then color="$R"; fi
        if [[ "$ing" == "0" && "$skip" == "0" ]]; then color="$Y"; fi  # idle = warn (no activity yet)
        printf "  %s%-22s %12s %12s %14s %10s %s%s\n" \
            "$color" "$table" "$ing" "$skip" "$last" "$fails" "$err" "$N"
    done
}

# ─── Section 4: Reconcile drift ───────────────────────────────────────────────
section_reconcile_drift() {
    echo "${B}── 4. Reconcile drift (ht_reconcile_log unresolved) ──────────────${N}"
    local total
    total=$(pq "SELECT COUNT(*) FROM ht_reconcile_log WHERE resolved_at IS NULL")
    if [[ "$total" == "0" ]]; then
        printf "  ${G}✓ 0 unresolved drift entries — shadow agrees with reconcile${N}\n"
        return 0
    fi
    printf "  ${Y}⚠ %s unresolved drift entries — shadow vs reconcile diverged${N}\n" "$total"
    pq "SELECT table_name, COUNT(*) FROM ht_reconcile_log WHERE resolved_at IS NULL
        GROUP BY table_name ORDER BY 2 DESC" | \
    while IFS=$'\t' read -r tbl cnt; do
        printf "    %-22s %s\n" "$tbl" "$cnt"
    done
    echo "  ${D}Sample of 3 most recent unresolved entries:${N}"
    pq "SELECT detected_at::timestamp(0), table_name, legacy_pk
        FROM ht_reconcile_log WHERE resolved_at IS NULL
        ORDER BY detected_at DESC LIMIT 3" | \
    while IFS=$'\t' read -r ts tbl pk; do
        printf "    %s  %-22s pk=%s\n" "$ts" "$tbl" "$pk"
    done
    return 1
}

# ─── Section 5: Cutover readiness ─────────────────────────────────────────────
section_readiness() {
    echo "${B}── 5. Cutover readiness checklist ────────────────────────────────${N}"
    local checks_pass=0 checks_total=0 any_failed=0

    chk() { # $1=label  $2=condition_result_code (0=pass)  $3=hint
        # Using $((x+1)) instead of ((x++)) — the latter returns nonzero
        # when the pre-increment value is 0, which short-circuits the caller's
        # `&& chk 0 || chk 1` pattern and causes double-firing.
        checks_total=$((checks_total + 1))
        if [[ "$2" == "0" ]]; then
            printf "  ${G}✓${N} %s\n" "$1"
            checks_pass=$((checks_pass + 1))
        else
            printf "  ${R}✗${N} %s ${D}— %s${N}\n" "$1" "$3"
            any_failed=1
        fi
    }

    # Check: sync container running
    local sync_state
    sync_state=$(ssh -o BatchMode=yes "$SSH_HOST" "docker inspect $SYNC_CONTAINER --format '{{.State.Status}}'" 2>/dev/null || echo "missing")
    if [[ "$sync_state" == "running" ]]; then chk "sync container running" 0 ""; else chk "sync container running" 1 "state=$sync_state (set LEGACY_SYNC_ENABLED=true to start)"; fi

    # Check: watermark > 0 (bootstrap was run)
    local last_seen
    last_seen=$(pq "SELECT last_seen_version FROM legacy_ct_state WHERE id=1")
    if [[ "${last_seen:-0}" -gt 0 ]]; then chk "bootstrap completed (watermark > 0)" 0 ""; else chk "bootstrap completed" 1 "run ./sync --bootstrap on evergreen"; fi

    # Check: poll freshness < 30s
    local secs_ago
    secs_ago=$(pq "SELECT EXTRACT(EPOCH FROM (now() - last_polled_at))::int FROM legacy_ct_state WHERE id=1")
    if [[ "${secs_ago:-9999}" -lt 30 ]]; then chk "watermark fresh (<30s)" 0 ""; else chk "watermark fresh" 1 "${secs_ago}s since last poll"; fi

    # Check: zero consecutive_failures across tables
    local fail_count
    fail_count=$(pq "SELECT COUNT(*) FROM legacy_sync_status WHERE consecutive_failures > 0")
    if [[ "$fail_count" == "0" ]]; then chk "no per-table consecutive failures" 0 ""; else chk "no per-table consecutive failures" 1 "$fail_count tables failing"; fi

    # Check: rows_skipped > 0 on at least one table (proves CONTEXT_INFO loop-prevention is working)
    local skipped_tables
    skipped_tables=$(pq "SELECT COUNT(*) FROM legacy_sync_status WHERE rows_skipped > 0")
    if [[ "${skipped_tables:-0}" -gt 0 ]]; then chk "CONTEXT_INFO filter active (rows_skipped>0 somewhere)" 0 ""; else chk "CONTEXT_INFO filter active" 1 "no skipped rows yet — writeback may be inactive (informational; wait for writeback traffic)"; fi

    # Check: drift log empty or stable (no NEW entries in last 1 hour)
    local recent_drift
    recent_drift=$(pq "SELECT COUNT(*) FROM ht_reconcile_log WHERE resolved_at IS NULL AND detected_at > now() - interval '1 hour'")
    if [[ "$recent_drift" == "0" ]]; then chk "no new reconcile drift in last 1h" 0 ""; else chk "no new reconcile drift" 1 "$recent_drift unresolved entries from past hour"; fi

    # Verdict (compare the explicit any_failed flag, not color strings — colors
    # collapse to "" in non-TTY mode and would always match $G).
    echo
    if [[ "$any_failed" == "0" ]]; then
        printf "  ${G}★ READY TO FLIP LIVE — %d/%d checks pass${N}\n" "$checks_pass" "$checks_total"
        printf "  ${D}To flip: gh secret set LEGACY_SYNC_SHADOW_MODE -b 'false' && trigger redeploy${N}\n"
        return 0
    else
        printf "  ${R}NOT READY — %d/%d checks pass${N}\n" "$checks_pass" "$checks_total"
        printf "  ${D}Resolve red items above. Run again to re-check.${N}\n"
        return 1
    fi
}

# ─── JSON mode (machine-readable, single shot) ────────────────────────────────
emit_json() {
    local ws lp
    IFS=$'\t' read -r ws lp <<< "$(pq 'SELECT last_seen_version, EXTRACT(EPOCH FROM (now() - last_polled_at))::int FROM legacy_ct_state WHERE id=1')"
    local fail_count
    fail_count=$(pq "SELECT COUNT(*) FROM legacy_sync_status WHERE consecutive_failures > 0")
    local drift_total
    drift_total=$(pq "SELECT COUNT(*) FROM ht_reconcile_log WHERE resolved_at IS NULL")
    local recent_drift
    recent_drift=$(pq "SELECT COUNT(*) FROM ht_reconcile_log WHERE resolved_at IS NULL AND detected_at > now() - interval '1 hour'")
    jq -n \
        --arg watermark "$ws" \
        --arg seconds_since_poll "$lp" \
        --arg failing_tables "$fail_count" \
        --arg drift_total "$drift_total" \
        --arg drift_last_hour "$recent_drift" \
        '{watermark: ($watermark|tonumber),
          seconds_since_poll: ($seconds_since_poll|tonumber),
          failing_tables: ($failing_tables|tonumber),
          drift_total: ($drift_total|tonumber),
          drift_last_hour: ($drift_last_hour|tonumber),
          ready_to_flip: (
            ($seconds_since_poll|tonumber) < 30 and
            ($failing_tables|tonumber) == 0 and
            ($drift_last_hour|tonumber) == 0 and
            ($watermark|tonumber) > 0
          )}'
}

# ─── Readiness mode (focused, exit-code-driven) ───────────────────────────────
emit_readiness() {
    section_readiness
}

# ─── Full report (one shot) ───────────────────────────────────────────────────
emit_full() {
    local now; now=$(date -u +%Y-%m-%dT%H:%M:%SZ)
    echo "${B}══ Phase 5 sync status @ $now ($SSH_HOST) ══${N}"
    echo
    local r1=0 r2=0 r3=0 r4=0 r5=0
    section_containers || r1=$?
    echo
    section_watermark || r2=$?
    echo
    section_per_table || r3=$?
    echo
    section_reconcile_drift || r4=$?
    echo
    section_readiness || r5=$?
    return $(( r1 > r2 ? r1 : (r2 > r3 ? r2 : (r3 > r4 ? r3 : (r4 > r5 ? r4 : r5))) ))
}

# ─── Main ─────────────────────────────────────────────────────────────────────
if [[ "$JSON" == "1" ]]; then emit_json; exit 0; fi
if [[ "$READINESS" == "1" ]]; then emit_readiness; exit $?; fi
if [[ "$WATCH" == "1" ]]; then
    while true; do
        clear
        emit_full || true
        echo
        echo "${D}refresh in ${WATCH_INTERVAL_SECS}s — Ctrl-C to exit${N}"
        sleep "$WATCH_INTERVAL_SECS"
    done
fi
emit_full
