#!/usr/bin/env bash
#
# scripts/liveness-check.sh — external container liveness probe for evergreen.
#
# Issue #262: `sync` / `writeback` run under `restart: on-failure:5`. Once those
# 5 attempts are exhausted the container STOPS and stays stopped, silently — the
# in-process watchdog (CT-lag pages, stall detection, boot-refusal alerts) dies
# with the host process, so nothing left running can page about the worker being
# dead. This script is the external observer that notices.
#
# ---------------------------------------------------------------------------
# DEPLOYMENT TARGET: install at /srv/liveness-check.sh, owned root:root, mode 755.
# ---------------------------------------------------------------------------
#
# It is NOT shipped by the deploy tarball. `scripts/deploy/run-deploy.sh` only
# self-updates itself; adding a second self-updating file to that path would mean
# editing the most dangerous file in the pipeline for a monitoring nicety. Install
# this one by hand (it changes about never) and bump SCRIPT_VERSION below when you
# do — `.github/workflows/liveness.yml` compares the reported version against the
# value it expects and warns on drift, so a stale host copy is visible.
#
# Invoked via a SECOND SSH forced-command entry in /home/deploy/.ssh/authorized_keys:
#
#   command="/srv/liveness-check.sh",restrict ssh-ed25519 AAAA...  liveness@github-actions
#
# (`restrict` = no pty, no agent/port/X11 forwarding, no user-rc.) The existing
# deploy key's forced command stays exactly as it is — the two entries are
# independent, and the liveness key can ONLY reach this script. Arguments arrive
# in $SSH_ORIGINAL_COMMAND and are matched against a strict allowlist below, so
# the key still cannot run arbitrary commands on the host.
#
# Runs as the `deploy` user, which is already in the docker group (it runs
# `docker compose` during deploys) and can read /home/deploy/secrets/* (mode 0400,
# owner deploy:docker). No new privileges are required.
#
# ---------------------------------------------------------------------------
# ALERTING CONTRACT
# ---------------------------------------------------------------------------
# Slack is posted FROM THIS HOST, reading the webhook from
# /home/deploy/secrets/slack_webhook_url. The webhook is deliberately NOT
# imported into GitHub Secrets — the runner never sees it, so this monitor does
# not widen secret distribution.
#
# State-change only, or it becomes the next alert-fatigue item: a dead container
# at */15 would otherwise be 96 pages/day.
#   * first observation of a dead container  -> write .liveness-pending-<name>
#   * still dead after GRACE_SECS            -> page + write .liveness-alerted-<name>
#   * already has .liveness-alerted-<name>   -> stay silent
#   * back to running with an alerted marker -> post all-clear, clear both markers
#   * back to running with only a pending marker -> clear it silently (never paged)
#
# The two-observation rule is what makes this safe to run during a deploy:
# `docker compose up -d` recreates containers, so a probe can catch one absent
# for a few seconds. Two consecutive samples 15 minutes apart both landing in
# that window is not a thing.
#
# Classification (why exit code matters):
#   running       -> ALIVE
#   restarting    -> TRANSIENT. on-failure:5 is still doing its job; report only.
#                    If it exhausts, the next tick sees `exited` and pages.
#   exited, rc=0  -> STOPPED-CLEAN, NOT paged. The worker binaries exit Ok(0)
#                    when their feature flag is off — see the restart-policy
#                    comments in docker-compose.yml (`WRITEBACK_ENABLED=false` /
#                    `LEGACY_SYNC_ENABLED=false`). Paging on this would fire every
#                    tick on any site whose profile is intentionally inert.
#   exited rc!=0 / dead / created / paused / missing -> DEAD, pageable.
#
# ---------------------------------------------------------------------------
# USAGE
# ---------------------------------------------------------------------------
#   /srv/liveness-check.sh --check      # normal run: classify, page/all-clear, mutate markers
#   /srv/liveness-check.sh --dry-run    # classify + print only. No Slack, no marker writes.
#
# Exit codes (the workflow maps these to run status):
#   0   all watched containers alive (or intentionally stopped-clean)
#   10  at least one container DEAD — page sent, or already deduped. NOT a monitor
#       failure, so the workflow stays green and annotates; Slack carries the page.
#   2   rejected/unknown argument
#   3   monitor failure (docker/jq/curl missing, webhook unreadable, Slack POST
#       failed). The workflow turns RED on this — the monitor itself is broken.
#
set -euo pipefail

SCRIPT_VERSION=1

# Non-interactive SSH sessions do not necessarily get /snap/bin on PATH, and
# docker on evergreen is snap-confined (see the DEPLOY_DIR comment in
# scripts/deploy/run-deploy.sh). Without this, `docker` is simply not found.
# LIVENESS_PATH is a test seam only — it lets a harness point `docker`/`curl` at
# stubs. Production never sets it.
PATH="${LIVENESS_PATH:-/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/snap/bin}"
export PATH

# ─── Config ──────────────────────────────────────────────────────────────────
# Default watch list. Names are the compose project (`new-hotel-production`, from
# DEPLOY_DIR=/home/deploy/new-hotel-production) plus service, except `new-hotel-db`
# and the two -hfville workers which pin container_name explicitly in
# docker-compose.yml.
DEFAULT_CONTAINERS=(
  new-hotel-production-backend-1
  new-hotel-production-web-1
  new-hotel-production-sync-1
  new-hotel-production-writeback-1
  new-hotel-production-sync-hfville-1
  new-hotel-production-writeback-hfville-1
  new-hotel-db
)

# Optional host-side override so the operator can pare the list down (e.g. a
# profile deliberately left down) without a repo change + manual reinstall.
# One container name per line; blank lines and `#` comments ignored.
CONTAINERS_FILE="${LIVENESS_CONTAINERS_FILE:-/home/deploy/.liveness-containers}"

MARKER_DIR="${LIVENESS_MARKER_DIR:-/home/deploy}"
WEBHOOK_FILE="${LIVENESS_WEBHOOK_FILE:-/home/deploy/secrets/slack_webhook_url}"
LOCK_FILE="$MARKER_DIR/.liveness.lock"

# How long a container must be continuously dead before we page. 300s at a */15
# cadence means "seen dead on two consecutive ticks".
GRACE_SECS="${LIVENESS_GRACE_SECS:-300}"

HOSTNAME_SHORT=$(hostname -s 2>/dev/null || echo evergreen)

# ─── Argument handling (forced-command allowlist) ────────────────────────────
# Under a forced command the client's requested command lands in
# SSH_ORIGINAL_COMMAND rather than $@. Accept ONLY these two exact strings —
# anything else is rejected without being executed, so the key stays as narrow
# as the deploy key's.
RAW_ARG="${SSH_ORIGINAL_COMMAND:-${1:---check}}"
DRY_RUN=false
case "$RAW_ARG" in
  --check|"") DRY_RUN=false ;;
  --dry-run)  DRY_RUN=true ;;
  *)
    echo "::error::liveness: rejected argument '${RAW_ARG}' (allowed: --check, --dry-run)"
    exit 2
    ;;
esac

echo "LIVENESS_SCRIPT_VERSION: $SCRIPT_VERSION"
echo "[liveness] host=$HOSTNAME_SHORT start=$(date -Iseconds) mode=$([ "$DRY_RUN" = true ] && echo dry-run || echo check)"

# ─── Preflight ───────────────────────────────────────────────────────────────
for tool in docker jq curl; do
  command -v "$tool" >/dev/null 2>&1 || {
    echo "::error::liveness: '$tool' not found on PATH ($PATH)"
    exit 3
  }
done

if ! docker info >/dev/null 2>&1; then
  echo "::error::liveness: cannot talk to the docker daemon as $(id -un) — is the user still in the docker group?"
  exit 3
fi

# Serialise against a manual run / an overlapping tick. Contention is benign:
# the other run is doing exactly this work, so skip rather than double-page.
exec 9>"$LOCK_FILE"
if ! flock -n 9; then
  echo "[liveness] another liveness check is already running (lock: $LOCK_FILE) — skipping"
  echo "LIVENESS_RESULT: skipped=1"
  exit 0
fi

# ─── Watch list ──────────────────────────────────────────────────────────────
CONTAINERS=()
if [ -r "$CONTAINERS_FILE" ]; then
  while IFS= read -r line; do
    line="${line%%#*}"
    line="$(echo "$line" | tr -d '[:space:]')"
    [ -n "$line" ] && CONTAINERS+=("$line")
  done < "$CONTAINERS_FILE"
  echo "[liveness] watch list overridden by $CONTAINERS_FILE (${#CONTAINERS[@]} entries)"
fi
if [ "${#CONTAINERS[@]}" -eq 0 ]; then
  CONTAINERS=("${DEFAULT_CONTAINERS[@]}")
fi

# ─── Slack ───────────────────────────────────────────────────────────────────
# Returns non-zero on failure; callers MUST NOT write the alerted marker unless
# this succeeded, so a transient Slack outage retries on the next tick instead of
# silently swallowing the page.
post_slack() {
  local text="$1"
  if [ "$DRY_RUN" = true ]; then
    echo "[liveness] [dry-run] would post to Slack: ${text}"
    return 0
  fi
  if [ ! -r "$WEBHOOK_FILE" ]; then
    echo "::error::liveness: webhook file $WEBHOOK_FILE is missing or unreadable by $(id -un)"
    return 1
  fi
  local url
  url=$(tr -d '\r\n' < "$WEBHOOK_FILE")
  if [ -z "$url" ]; then
    echo "::error::liveness: webhook file $WEBHOOK_FILE is empty"
    return 1
  fi
  # --data @- keeps the payload off the process table; the URL is never echoed.
  if ! jq -n --arg t "$text" '{text:$t}' \
      | curl -fsS --retry 2 --retry-delay 3 --max-time 20 \
             -X POST -H 'Content-type: application/json' \
             --data @- "$url" >/dev/null; then
    echo "::error::liveness: Slack POST failed"
    return 1
  fi
  echo "[liveness] slack: posted"
  return 0
}

marker_path() {
  # Container names are [A-Za-z0-9_.-] under compose; sanitise anyway so a
  # hand-edited watch list can never escape MARKER_DIR.
  local kind="$1" name="$2"
  printf '%s/.liveness-%s-%s' "$MARKER_DIR" "$kind" "$(printf '%s' "$name" | tr -c 'A-Za-z0-9_.-' '_')"
}

# ─── Inspect loop ────────────────────────────────────────────────────────────
n_alive=0; n_dead=0; n_clean=0; n_restarting=0
monitor_failed=false
declare -a dead_names=()

now_epoch=$(date +%s)

for c in "${CONTAINERS[@]}"; do
  raw=$(docker inspect -f '{{.State.Status}}|{{.State.ExitCode}}|{{.State.FinishedAt}}|{{.RestartCount}}' "$c" 2>/dev/null) || raw=""

  if [ -z "$raw" ]; then
    status=missing; exit_code="-"; finished="-"; restarts="-"
  else
    IFS='|' read -r status exit_code finished restarts <<< "$raw"
  fi

  case "$status" in
    running)
      verdict=alive ;;
    restarting)
      verdict=restarting ;;
    exited)
      if [ "$exit_code" = "0" ]; then verdict=stopped_clean; else verdict=dead; fi ;;
    *)
      verdict=dead ;;
  esac

  pending_marker=$(marker_path pending "$c")
  alerted_marker=$(marker_path alerted "$c")

  case "$verdict" in
    alive|restarting|stopped_clean)
      case "$verdict" in
        alive)         n_alive=$((n_alive + 1)) ;;
        restarting)    n_restarting=$((n_restarting + 1)) ;;
        stopped_clean) n_clean=$((n_clean + 1)) ;;
      esac

      echo "[liveness] $c: $verdict (status=$status exit=$exit_code restarts=$restarts)"

      # Recovery transition. A stopped_clean container is NOT a recovery — it is
      # still not running — but we also never paged for it, so only an alerted
      # marker (which can only exist from a genuine DEAD page) triggers all-clear,
      # and only when it is genuinely back up.
      if [ "$verdict" = alive ] && [ -f "$alerted_marker" ]; then
        if [ "$DRY_RUN" = true ]; then
          echo "[liveness] [dry-run] would post all-clear + clear markers for $c"
        elif post_slack ":white_check_mark: RECOVERED — \`$c\` is running again on ${HOSTNAME_SHORT} (restarts=${restarts}). Liveness monitor clearing its alert marker."; then
          rm -f "$alerted_marker" "$pending_marker"
          echo "[liveness] $c: all-clear posted, markers cleared"
        else
          monitor_failed=true
        fi
      elif [ "$verdict" = alive ] && [ -f "$pending_marker" ]; then
        if [ "$DRY_RUN" = true ]; then
          echo "[liveness] [dry-run] would clear stale pending marker for $c"
        else
          rm -f "$pending_marker"
          echo "[liveness] $c: recovered before the grace window elapsed — pending marker cleared, no page sent"
        fi
      fi
      ;;

    dead)
      n_dead=$((n_dead + 1))
      dead_names+=("$c")
      echo "[liveness] $c: DEAD (status=$status exit=$exit_code finished=$finished restarts=$restarts)"

      if [ -f "$alerted_marker" ]; then
        echo "[liveness] $c: already alerted — suppressed (state-change-only paging)"
        continue
      fi

      if [ "$DRY_RUN" = true ]; then
        echo "[liveness] [dry-run] would arm/escalate pending marker for $c"
        continue
      fi

      if [ ! -f "$pending_marker" ]; then
        printf '%s\n' "$now_epoch" > "$pending_marker"
        echo "[liveness] $c: first dead observation recorded; will page if still dead in ${GRACE_SECS}s"
        continue
      fi

      first_seen=$(head -n1 "$pending_marker" 2>/dev/null | tr -cd '0-9')
      [ -n "$first_seen" ] || first_seen="$now_epoch"
      down_for=$((now_epoch - first_seen))
      if [ "$down_for" -lt "$GRACE_SECS" ]; then
        echo "[liveness] $c: dead for ${down_for}s, under the ${GRACE_SECS}s grace window — not paging yet"
        continue
      fi

      # The cause line differs by status: an exhausted `on-failure:5` leaves an
      # exited container with logs to read; a container that is simply GONE was
      # never recreated (profile not passed to `docker compose up`, or removed),
      # and `docker logs` has nothing to show.
      if [ "$status" = "missing" ]; then
        cause="No such container on this host — it was never recreated (was \`docker compose up -d\` run without \`--profile legacy --profile hfville\`?) or it was removed."
        triage="Triage: \`ssh evergreen 'cd /home/deploy/new-hotel-production && docker compose --profile legacy --profile hfville up -d'\`"
      else
        cause="\`restart: on-failure:5\` has stopped retrying; nothing will bring this back on its own."
        triage="Triage: \`ssh evergreen 'docker logs --tail 100 ${c}'\` then \`cd /home/deploy/new-hotel-production && docker compose up -d\` (add \`--profile legacy --profile hfville\` for the workers)."
      fi

      msg=":rotating_light: CONTAINER DOWN — \`$c\` on ${HOSTNAME_SHORT}
status=\`${status}\` exit_code=\`${exit_code}\` restart_count=\`${restarts}\` finished_at=\`${finished}\`
Down for ~$((down_for / 60))m. ${cause}
${triage}
This alert is state-change-only — you get ONE page and one all-clear, not one per 15 minutes."

      if post_slack "$msg"; then
        : > "$alerted_marker"
        echo "[liveness] $c: paged, alerted marker written"
      else
        monitor_failed=true
        echo "[liveness] $c: page FAILED — alerted marker deliberately not written, will retry next tick"
      fi
      ;;
  esac
done

echo "LIVENESS_RESULT: alive=${n_alive} dead=${n_dead} stopped_clean=${n_clean} restarting=${n_restarting} watched=${#CONTAINERS[@]}"
if [ "${#dead_names[@]}" -gt 0 ]; then
  echo "LIVENESS_DEAD: ${dead_names[*]}"
fi
echo "[liveness] done $(date -Iseconds)"

if [ "$monitor_failed" = true ]; then
  exit 3
fi
if [ "$n_dead" -gt 0 ]; then
  exit 10
fi
exit 0
