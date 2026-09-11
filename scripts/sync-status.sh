#!/usr/bin/env bash
# Phase 5 sync worker observability dashboard.
#
# Prints a sectioned, color-coded report of the CT watcher's state so you can
# decide whether to flip from shadow to live. Designed to be run from anywhere
# on your laptop — it SSHes to evergreen and queries PG directly.
#
# Usage:
#   scripts/sync-status.sh                  # one-shot report (HF Hotel)
#   scripts/sync-status.sh --watch          # refresh every 30s (Ctrl-C to exit)
#   scripts/sync-status.sh --json           # machine-readable, single shot
#   scripts/sync-status.sh --readiness      # cutover readiness check (exit 0 = green)
#   scripts/sync-status.sh --site hfville   # query the Ville canonical PG database
#
# Flags:
#   --site <hfhotel|hfville>  Pick which site's PG database to query.
#                             hfhotel (default) → database `hotelnew`.
#                             hfville            → database `hotelville`.
#                             Both live in the same `new-hotel-db` container;
#                             only the database name changes. Container
#                             selection (sync/writeback) is NOT switched here —
#                             that's task #78's out-of-scope item.
#
# Sections:
#   1. Container status — sync, writeback, backend, newdb (running? healthy?)
#   2. Watermark + lag — how far behind MSSQL the watcher is (skipped if --no-mssql)
#   3. Per-table activity — rows_ingested, rows_skipped, errors, freshness
#   4. Reconcile drift — unresolved ht_reconcile_log entries by table
#   5. Cutover readiness — green/yellow/red checklist
#
# Watermark modes:
#   The CT watcher holds its resume point in one of two shapes, selected by
#   the `SYNC_PER_TABLE_WATERMARK` env var on the sync container:
#     off (default) — single-row `legacy_ct_state`. Sections 2 + 5 read it,
#                     exactly as they always have. Output is unchanged.
#     on            — one row per CT-enabled table in
#                     `legacy_ct_state_per_table`; the global row STOPS
#                     advancing and is no longer a health signal. Section 2
#                     then also reports the lowest watermark and the stalest
#                     poll (with the offending table name), and section 5's
#                     bootstrap + freshness gates are computed from those
#                     rows instead of the frozen global one.
#   The mode is detected the same way shadow mode is — `docker inspect` of
#   the sync container's env. A failed inspect / absent var reads as `off`,
#   matching the binary's own default.
#
#   `--json` reports the per-table rows in BOTH modes (new keys, prefixed
#   `per_table_`), so you can confirm migration 078's reseed has landed and
#   `per_table_lag_behind_global` is ~0 BEFORE flipping the flag.
#
#   Caveat inherited from #78: `--site hfville` switches the PG database but
#   NOT the container, so mode detection still inspects HF Hotel's sync
#   container. For an accurate Ville reading, also pass
#   `SYNC_STATUS_SYNC_CONTAINER=new-hotel-production-sync-hfville-1`.
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
# PG_DATABASE is set after argument parsing — it depends on `--site`. The
# default is `hotelnew` (HF Hotel), preserving pre-#78 behavior.
PG_DATABASE=""
SYNC_CONTAINER="${SYNC_STATUS_SYNC_CONTAINER:-new-hotel-production-sync-1}"
WATCH_INTERVAL_SECS="${SYNC_STATUS_WATCH_INTERVAL_SECS:-30}"

# ─── Mode detection ───────────────────────────────────────────────────────────
# Reads LEGACY_SYNC_SHADOW_MODE from the running sync container's env via
# `docker inspect`. Cached once per script invocation. Three states:
#   shadow  — TX rolls back every tick; watermark + counters never advance.
#             "freshness" and "rows_ingested" checks DON'T apply.
#   live    — TX commits; watermark advances; counters move.
#   unknown — container missing OR env var absent. Treat as live (strictest gate).
detect_sync_mode() {
    local env_dump
    env_dump=$(ssh -o BatchMode=yes "$SSH_HOST" \
        "docker inspect $SYNC_CONTAINER --format '{{range .Config.Env}}{{println .}}{{end}}'" 2>/dev/null) || {
        echo "unknown"; return
    }
    local val
    val=$(echo "$env_dump" | grep -E '^LEGACY_SYNC_SHADOW_MODE=' | head -1 | cut -d= -f2- | tr -d '[:space:]')
    case "$val" in
        true|TRUE|True|1) echo "shadow" ;;
        false|FALSE|False|0) echo "live" ;;
        *) echo "unknown" ;;
    esac
}
SYNC_MODE=""  # populated lazily by sections that need it

# ─── Watermark-mode detection ─────────────────────────────────────────────────
# Reads SYNC_PER_TABLE_WATERMARK from the running sync container's env (same
# `docker inspect` mechanism as detect_sync_mode). Two states:
#   on   — the watcher resumes from `legacy_ct_state_per_table`; the single-row
#          `legacy_ct_state` is frozen at the moment of the flip and is NOT a
#          health signal any more.
#   off  — the single-row `legacy_ct_state` is authoritative (shipped default).
# Anything else — absent var, missing container, failed ssh — reads as `off`,
# which both matches the binary's own default and keeps this script's output
# byte-identical to its pre-per-table behaviour.
detect_per_table_mode() {
    local env_dump
    env_dump=$(ssh -o BatchMode=yes "$SSH_HOST" \
        "docker inspect $SYNC_CONTAINER --format '{{range .Config.Env}}{{println .}}{{end}}'" 2>/dev/null) || {
        echo "off"; return
    }
    local val
    # `|| true` on the pipeline: unlike LEGACY_SYNC_SHADOW_MODE, this var is
    # absent on any container built before the flag was plumbed, and a bare
    # failing grep under `set -e` would abort the script.
    val=$(printf '%s\n' "$env_dump" | grep -E '^SYNC_PER_TABLE_WATERMARK=' | head -1 | cut -d= -f2- | tr -d '[:space:]' || true)
    case "$val" in
        true|TRUE|True|1) echo "on" ;;
        *) echo "off" ;;
    esac
}
PER_TABLE_MODE=""  # populated lazily by sections that need it

# Single-row aggregate over `legacy_ct_state_per_table`, tab-separated:
#   rows  min_version  max_version  lowest_table  stalest_poll_secs  stalest_table  errored_rows
# Emitted as one row so the whole per-table picture costs one round trip.
per_table_stats() {
    pq "SELECT (SELECT COUNT(*) FROM legacy_ct_state_per_table),
               COALESCE((SELECT MIN(last_seen_version) FROM legacy_ct_state_per_table), 0),
               COALESCE((SELECT MAX(last_seen_version) FROM legacy_ct_state_per_table), 0),
               COALESCE((SELECT table_name FROM legacy_ct_state_per_table
                          ORDER BY last_seen_version ASC, table_name ASC LIMIT 1), '-'),
               COALESCE((SELECT EXTRACT(EPOCH FROM (now() - MIN(last_polled_at)))::int
                           FROM legacy_ct_state_per_table), 0),
               COALESCE((SELECT table_name FROM legacy_ct_state_per_table
                          ORDER BY last_polled_at ASC, table_name ASC LIMIT 1), '-'),
               (SELECT COUNT(*) FROM legacy_ct_state_per_table WHERE last_error IS NOT NULL)"
}

# ─── Args ─────────────────────────────────────────────────────────────────────
# `--site` is parsed in a `while`-shift loop because it takes a value. The
# other flags stay boolean. Default site is `hfhotel`, which maps to the
# original `hotelnew` database name (back-compat — pre-#78 callers see no
# change).
WATCH=0; JSON=0; READINESS=0
SITE="hfhotel"
while [[ $# -gt 0 ]]; do
    case "$1" in
        -w|--watch) WATCH=1; shift ;;
        --json) JSON=1; shift ;;
        --readiness) READINESS=1; shift ;;
        --site)
            [[ $# -lt 2 ]] && { echo "--site requires a value (hfhotel|hfville)" >&2; exit 2; }
            SITE="$2"; shift 2 ;;
        --site=*)
            SITE="${1#--site=}"; shift ;;
        -h|--help) grep '^#' "$0" | sed 's/^# \?//'; exit 0 ;;
        *) echo "unknown flag: $1" >&2; exit 2 ;;
    esac
done

# Derive the PG database name from --site. Override is still honored via
# SYNC_STATUS_PG_DATABASE for one-off operator sessions (e.g. pointing at
# a staging DB), but --site always wins when it's explicit.
case "$SITE" in
    hfhotel) PG_DATABASE="${SYNC_STATUS_PG_DATABASE:-hotelnew}" ;;
    hfville) PG_DATABASE="${SYNC_STATUS_PG_DATABASE_HFVILLE:-hotelville}" ;;
    *) echo "unknown site: $SITE (expected hfhotel or hfville)" >&2; exit 2 ;;
esac

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

    # Per-table mode changes WHICH rows are authoritative. Nothing is printed
    # (and nothing above is re-interpreted) while the flag is off, so the
    # default-mode report stays byte-identical.
    if [[ -z "$PER_TABLE_MODE" ]]; then PER_TABLE_MODE=$(detect_per_table_mode); fi
    if [[ "$PER_TABLE_MODE" == "on" ]]; then
        section_watermark_per_table "$last_seen"
        return $?
    fi

    if (( last_seen == 0 )); then
        printf "  ${Y}⚠ watermark is 0 — bootstrap not run yet (./sync --bootstrap)${N}\n"
        return 1
    fi
    return 0
}

# ─── Section 2b: per-table watermarks (only when SYNC_PER_TABLE_WATERMARK) ────
# $1 = the global watermark already printed above, used to show how far the
# slowest table sits behind the (now frozen) global row.
section_watermark_per_table() {
    local global_watermark="$1"
    local stats
    stats=$(per_table_stats) || {
        echo "  ${R}✗ failed to query legacy_ct_state_per_table${N}"; return 2; }
    [[ -z "$stats" ]] && {
        echo "  ${R}✗ failed to query legacy_ct_state_per_table${N}"; return 2; }

    local n_rows min_ver max_ver min_table oldest_secs oldest_table n_errors
    IFS=$'\t' read -r n_rows min_ver max_ver min_table oldest_secs oldest_table n_errors <<< "$stats" || true

    printf "  ${D}SYNC_PER_TABLE_WATERMARK=true — legacy_ct_state above is FROZEN at the\n"
    printf "  flip point; the legacy_ct_state_per_table rows below are authoritative.${N}\n"

    if [[ "${n_rows:-0}" == "0" ]]; then
        printf "  ${R}✗ legacy_ct_state_per_table is EMPTY — per-table mode has no resume\n"
        printf "    point. Apply migration 078 (reseed) before flipping the flag.${N}\n"
        return 2
    fi

    local rows_color="$G"
    # 19 CT_ENABLED_TABLES as of 2026-07-27 (bin/sync.rs). Fewer rows than
    # tables means a mapper shipped without its seed — that table would
    # resume from 0 and trip the retention-overflow page.
    (( n_rows < 19 )) && rows_color="$Y"
    printf "  Per-table rows:   %s%s${N}  (legacy_ct_state_per_table; expect 19)\n" "$rows_color" "$n_rows"

    printf "  Watermarks:       min ${G}%s${N} (%s) … max ${G}%s${N}\n" "$min_ver" "$min_table" "$max_ver"

    # Lag vs the frozen global row is the migration-078 reseed signal: > 0
    # means some table never caught up to where global mode left off, and the
    # gap is what `check_retention` will measure against CT's 2-day horizon.
    local lag=$(( global_watermark - min_ver ))
    local lag_color="$G"; local lag_note="0 — in step with the frozen global row"
    if (( min_ver == 0 )); then
        lag_color="$R"; lag_note="UNSEEDED — apply migration 078 / run ./sync --bootstrap"
    elif (( lag > 0 )); then
        lag_color="$Y"; lag_note="${lag} behind the frozen global row (${min_table})"
    fi
    printf "  Lag vs global:    %s%s${N}\n" "$lag_color" "$lag_note"

    local poll_color="$G"; local poll_label="fresh"
    if (( oldest_secs > 60 )); then poll_color="$R"; poll_label="STALE — that table may be wedged"; fi
    if (( oldest_secs > 10 && oldest_secs <= 60 )); then poll_color="$Y"; poll_label="slow"; fi
    printf "  Stalest poll:     %s%ds ago${N}  (%s — %s)\n" "$poll_color" "$oldest_secs" "$oldest_table" "$poll_label"

    if [[ "${n_errors:-0}" != "0" ]]; then
        printf "  Rows with error:  ${R}%s${N}  (legacy_ct_state_per_table.last_error)\n" "$n_errors"
        pq "SELECT table_name, LEFT(last_error, 60) FROM legacy_ct_state_per_table
            WHERE last_error IS NOT NULL ORDER BY last_error_at DESC LIMIT 3" | \
        while IFS=$'\t' read -r tbl err; do
            printf "    %-24s %s\n" "$tbl" "$err"
        done
        return 1
    fi
    printf "  Rows with error:  ${G}0${N}\n"

    if (( min_ver == 0 )); then
        printf "  ${Y}⚠ a per-table watermark is 0 — bootstrap not run yet (./sync --bootstrap)${N}\n"
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
# Asks a different question depending on the worker's mode:
#   shadow → "is it safe to flip to LIVE?" — verifies polling activity, no
#            mapper errors, no NEW reconcile drift since cutover (the existing
#            pre-cutover backlog of ht_reconcile_log entries is ignored).
#   live   → "is it healthy?" — verifies watermark advances, rows_ingested>0,
#            no per-table failures, no growing drift.
#   unknown→ falls back to the live-mode gate (strictest).
section_readiness() {
    [[ -z "$SYNC_MODE" ]] && SYNC_MODE=$(detect_sync_mode)
    if [[ -z "$PER_TABLE_MODE" ]]; then PER_TABLE_MODE=$(detect_per_table_mode); fi
    # Suffix only appears in per-table mode, so the default header is unchanged.
    local pt_suffix=""
    [[ "$PER_TABLE_MODE" == "on" ]] && pt_suffix=", watermark: per-table"
    echo "${B}── 5. Cutover readiness checklist (mode: ${SYNC_MODE}${pt_suffix}) ───────────${N}"
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

    # ── Universal checks (apply in any mode) ──────────────────────────────
    # Container state
    local sync_state
    sync_state=$(ssh -o BatchMode=yes "$SSH_HOST" "docker inspect $SYNC_CONTAINER --format '{{.State.Status}}'" 2>/dev/null || echo "missing")
    if [[ "$sync_state" == "running" ]]; then chk "sync container running" 0 ""; else chk "sync container running" 1 "state=$sync_state (set LEGACY_SYNC_ENABLED=true to start)"; fi

    # Bootstrap (watermark > 0). In per-table mode the global row is frozen and
    # proves nothing — gate on the LOWEST per-table watermark instead, so a
    # single unseeded table (which would resume from 0 and replay from
    # time-zero) fails the check.
    local last_seen
    if [[ "$PER_TABLE_MODE" == "on" ]]; then
        last_seen=$(pq "SELECT COALESCE(MIN(last_seen_version), 0) FROM legacy_ct_state_per_table")
        if [[ "${last_seen:-0}" -gt 0 ]]; then chk "bootstrap completed (min per-table watermark > 0)" 0 ""; else chk "bootstrap completed" 1 "a legacy_ct_state_per_table row is at 0 — apply migration 078 / run ./sync --bootstrap"; fi
    else
        last_seen=$(pq "SELECT last_seen_version FROM legacy_ct_state WHERE id=1")
        if [[ "${last_seen:-0}" -gt 0 ]]; then chk "bootstrap completed (watermark > 0)" 0 ""; else chk "bootstrap completed" 1 "run ./sync --bootstrap on evergreen"; fi
    fi

    # Per-table failures
    local fail_count
    fail_count=$(pq "SELECT COUNT(*) FROM legacy_sync_status WHERE consecutive_failures > 0")
    if [[ "$fail_count" == "0" ]]; then chk "no per-table consecutive failures" 0 ""; else chk "no per-table consecutive failures" 1 "$fail_count tables failing"; fi

    # R3-native error signal. `legacy_sync_status.consecutive_failures` above
    # is the watcher's observability counter; `legacy_ct_state_per_table
    # .last_error` is written by record_per_table_error on its own small TX and
    # is the only place a per-table failure is durable in shadow mode. Only
    # checked in per-table mode — the column is inert otherwise.
    if [[ "$PER_TABLE_MODE" == "on" ]]; then
        local pt_err_count
        pt_err_count=$(pq "SELECT COUNT(*) FROM legacy_ct_state_per_table WHERE last_error IS NOT NULL")
        if [[ "${pt_err_count:-0}" == "0" ]]; then chk "no per-table watermark errors" 0 ""; else chk "no per-table watermark errors" 1 "${pt_err_count} rows carry legacy_ct_state_per_table.last_error"; fi
    fi

    # ── Mode-specific checks ──────────────────────────────────────────────
    if [[ "$SYNC_MODE" == "shadow" ]]; then
        # Shadow mode design constraint: the per-table dispatch updates
        # `legacy_sync_status` counters INSIDE the polling TX, which rolls back.
        # That means we CAN'T observe ingestion freshness or rows_skipped from
        # PG in shadow mode — only error-path writes (consecutive_failures /
        # last_error) survive because they use a separate small TX.
        # Reconcile drift growth post-bootstrap is also EXPECTED (the watcher
        # rolls back its writes; the 15-min reconcile keeps detecting the same
        # divergence). So the only meaningful signal in shadow mode is:
        # container alive, bootstrap done, no error counters ticking.
        # Soak duration is informational — recommend 24h+ but don't gate on it.
        local soak_secs soak_h soak_m
        soak_secs=$(pq "SELECT EXTRACT(EPOCH FROM (now() - last_polled_at))::int FROM legacy_ct_state WHERE id=1")
        soak_h=$(( ${soak_secs:-0} / 3600 ))
        soak_m=$(( (${soak_secs:-0} % 3600) / 60 ))
        if [[ "${soak_secs:-0}" -ge 86400 ]]; then
            chk "soaked >= 24h (recommended)" 0 ""
        else
            # Print but don't fail — render as a yellow info line, not a red ✗.
            checks_total=$((checks_total + 1))
            checks_pass=$((checks_pass + 1))  # treat as pass for verdict
            printf "  ${Y}…${N} soak duration: ${soak_h}h ${soak_m}m ${D}(recommend 24h+ before flipping live; not blocking)${N}\n"
        fi
    else
        # Live mode (or unknown — treated as live for the strictest gate).
        # Watermark advances continuously; rows_ingested ticks up.
        # Freshness. In per-table mode `legacy_ct_state.last_polled_at` stops
        # moving at the flip, so reading it would fail this gate forever (and
        # hide a genuinely wedged table). Gate on the OLDEST per-table poll and
        # name the offending table in the hint.
        local secs_ago
        if [[ "$PER_TABLE_MODE" == "on" ]]; then
            local stalest_row stalest_table
            stalest_row=$(pq "SELECT COALESCE(EXTRACT(EPOCH FROM (now() - MIN(last_polled_at)))::int, 9999),
                                     COALESCE((SELECT table_name FROM legacy_ct_state_per_table
                                                ORDER BY last_polled_at ASC, table_name ASC LIMIT 1), '-')
                              FROM legacy_ct_state_per_table")
            IFS=$'\t' read -r secs_ago stalest_table <<< "$stalest_row" || true
            if [[ "${secs_ago:-9999}" -lt 30 ]]; then chk "per-table watermarks fresh (<30s, stalest ${stalest_table:--})" 0 ""; else chk "per-table watermarks fresh" 1 "${stalest_table:--} last polled ${secs_ago:-9999}s ago — that table may be wedged"; fi
        else
            secs_ago=$(pq "SELECT EXTRACT(EPOCH FROM (now() - last_polled_at))::int FROM legacy_ct_state WHERE id=1")
            if [[ "${secs_ago:-9999}" -lt 30 ]]; then chk "watermark fresh (<30s)" 0 ""; else chk "watermark fresh" 1 "${secs_ago}s since last poll — worker may be stuck"; fi
        fi

        local skipped_tables
        skipped_tables=$(pq "SELECT COUNT(*) FROM legacy_sync_status WHERE rows_skipped > 0")
        if [[ "${skipped_tables:-0}" -gt 0 ]]; then chk "CONTEXT_INFO filter active (rows_skipped>0)" 0 ""; else chk "CONTEXT_INFO filter active" 1 "no skipped rows yet — writeback may be idle (informational)"; fi

        local recent_drift
        recent_drift=$(pq "SELECT COUNT(*) FROM ht_reconcile_log WHERE resolved_at IS NULL AND detected_at > now() - interval '1 hour'")
        if [[ "${recent_drift:-0}" == "0" ]]; then chk "no new reconcile drift in last 1h" 0 ""; else chk "no new reconcile drift" 1 "$recent_drift unresolved entries from past hour"; fi
    fi

    # Verdict (compare the explicit any_failed flag, not color strings — colors
    # collapse to "" in non-TTY mode and would always match $G).
    echo
    if [[ "$any_failed" == "0" ]]; then
        if [[ "$SYNC_MODE" == "shadow" ]]; then
            printf "  ${G}★ READY TO FLIP LIVE — %d/%d checks pass${N}\n" "$checks_pass" "$checks_total"
            printf "  ${D}To flip: gh secret set LEGACY_SYNC_SHADOW_MODE -b 'false' && gh workflow run docker-build.yml${N}\n"
        else
            printf "  ${G}★ HEALTHY (live mode) — %d/%d checks pass${N}\n" "$checks_pass" "$checks_total"
        fi
        return 0
    else
        printf "  ${R}NOT READY — %d/%d checks pass${N}\n" "$checks_pass" "$checks_total"
        printf "  ${D}Resolve red items above. Run again to re-check.${N}\n"
        return 1
    fi
}

# ─── JSON mode (machine-readable, single shot) ────────────────────────────────
emit_json() {
    [[ -z "$SYNC_MODE" ]] && SYNC_MODE=$(detect_sync_mode)
    if [[ -z "$PER_TABLE_MODE" ]]; then PER_TABLE_MODE=$(detect_per_table_mode); fi
    local ws lp
    IFS=$'\t' read -r ws lp <<< "$(pq 'SELECT last_seen_version, EXTRACT(EPOCH FROM (now() - last_polled_at))::int FROM legacy_ct_state WHERE id=1')"
    # Per-table rows are reported in BOTH modes — while the flag is still off
    # they are the pre-flip readiness signal (is migration 078's reseed in, is
    # per_table_lag_behind_global ~0?). Existing keys are untouched; every new
    # key is prefixed `per_table_` so monitors keying on the old schema keep
    # working.
    local pt_rows pt_min pt_max pt_min_table pt_secs pt_stalest_table pt_errored
    IFS=$'\t' read -r pt_rows pt_min pt_max pt_min_table pt_secs pt_stalest_table pt_errored <<< "$(per_table_stats)" || true
    local fail_count
    fail_count=$(pq "SELECT COUNT(*) FROM legacy_sync_status WHERE consecutive_failures > 0")
    local drift_total
    drift_total=$(pq "SELECT COUNT(*) FROM ht_reconcile_log WHERE resolved_at IS NULL")
    local recent_drift
    recent_drift=$(pq "SELECT COUNT(*) FROM ht_reconcile_log WHERE resolved_at IS NULL AND detected_at > now() - interval '1 hour'")
    local new_drift_since_bootstrap
    new_drift_since_bootstrap=$(pq "SELECT COUNT(*) FROM ht_reconcile_log WHERE resolved_at IS NULL AND detected_at > (SELECT last_polled_at FROM legacy_ct_state WHERE id=1)")
    jq -n \
        --arg mode "$SYNC_MODE" \
        --arg watermark "$ws" \
        --arg seconds_since_poll "$lp" \
        --arg failing_tables "$fail_count" \
        --arg drift_total "$drift_total" \
        --arg drift_last_hour "$recent_drift" \
        --arg drift_since_bootstrap "$new_drift_since_bootstrap" \
        --arg per_table_mode "$PER_TABLE_MODE" \
        --arg per_table_rows "${pt_rows:-0}" \
        --arg per_table_min_watermark "${pt_min:-0}" \
        --arg per_table_max_watermark "${pt_max:-0}" \
        --arg per_table_min_watermark_table "${pt_min_table:--}" \
        --arg per_table_seconds_since_poll "${pt_secs:-0}" \
        --arg per_table_stalest_table "${pt_stalest_table:--}" \
        --arg per_table_errored_rows "${pt_errored:-0}" \
        '{mode: $mode,
          watermark: ($watermark|tonumber),
          seconds_since_poll: ($seconds_since_poll|tonumber),
          failing_tables: ($failing_tables|tonumber),
          drift_total: ($drift_total|tonumber),
          drift_last_hour: ($drift_last_hour|tonumber),
          drift_since_bootstrap: ($drift_since_bootstrap|tonumber),
          # Per-table watermark block (SYNC_PER_TABLE_WATERMARK, migration 050
          # + the 078 reseed). Reported in BOTH modes: while the flag is off
          # these are the PRE-FLIP readiness signal — `per_table_rows` should
          # equal the CT_ENABLED_TABLES count (19) and
          # `per_table_lag_behind_global` should be ~0 before flipping, else
          # every table trips the CT retention-overflow page on the first tick.
          per_table_mode: $per_table_mode,
          per_table_rows: ($per_table_rows|tonumber),
          per_table_min_watermark: ($per_table_min_watermark|tonumber),
          per_table_max_watermark: ($per_table_max_watermark|tonumber),
          per_table_min_watermark_table: $per_table_min_watermark_table,
          per_table_lag_behind_global:
            (($watermark|tonumber) - ($per_table_min_watermark|tonumber)),
          per_table_seconds_since_poll: ($per_table_seconds_since_poll|tonumber),
          per_table_stalest_table: $per_table_stalest_table,
          per_table_errored_rows: ($per_table_errored_rows|tonumber),
          ready_to_flip: (
            # Mode-aware ready-to-flip-live verdict.
            # - shadow: bootstrap done + no per-table failures (the only PG-observable
            #           signals; counters/freshness are inside the rolled-back TX).
            # - live:   watermark fresh + no per-table failures + no recent drift growth.
            # - unknown: never ready (require explicit operator inspection).
            #
            # Under per-table mode the global row is FROZEN at the flip point,
            # so the bootstrap + freshness terms switch to the per-table
            # aggregates. The off-mode expression is unchanged.
            (if $per_table_mode == "on"
               then ($per_table_min_watermark|tonumber) > 0
               else ($watermark|tonumber) > 0 end) and
            ($failing_tables|tonumber) == 0 and (
              ($mode == "shadow")
              or
              ($mode == "live"
               and (if $per_table_mode == "on"
                      then ($per_table_seconds_since_poll|tonumber) < 30
                      else ($seconds_since_poll|tonumber) < 30 end)
               and ($drift_last_hour|tonumber) == 0)
            )
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
