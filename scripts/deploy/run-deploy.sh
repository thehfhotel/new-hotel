#!/bin/bash
#
# /srv/run-deploy.sh — production deploy script for evergreen
#
# Deployment target: install at /srv/run-deploy.sh, owned root:root, mode 755.
# Triggered via SSH forced-command in /home/deploy/.ssh/authorized_keys —
# the workflow runs `ssh deploy@evergreen.tailnet < payload.json` and authorized_keys
# pins this script as the only thing that key can do. The deploy user can't run
# anything else; the script can't be modified by the deploy user (root-owned).
#
# Reads JSON from stdin:
# {
#   "commit_sha": "abc123...",
#   "deploy_payload_b64": "<base64 tarball: docker-compose.yml + init-db/ + migrations/pg/ + scripts/migrate.sh + scripts/deploy/run-deploy.sh>",
#   "env": { "DB_SERVER": "...", "DB_PASSWORD": "...", ... }
# }
#
# Self-update: the tarball ships `scripts/deploy/run-deploy.sh` so the
# script can replace itself in /srv/run-deploy.sh on each deploy — see
# Track J5. The repo copy at scripts/deploy/run-deploy.sh is now the
# source of truth; edit it there, not on the host.
#
# Logs to /var/log/deploy/deploy-<timestamp>.log so operator can post-mortem.

set -euo pipefail
# umask is intentionally NOT set globally — only applied around .env write
# below. Setting it globally would make `mkdir -p` create $DEPLOY_DIR with
# mode 0700, which (a) breaks postgres bind-mounting init-db/ on first deploy
# (postgres container UID can't read 0700 owned by deploy uid), and
# (b) blocks `nut` from inspecting via the /home/nut/new-hotel-production
# symlink during incidents.

DEPLOY_DIR=/home/deploy/new-hotel-production
LOG_DIR=/var/log/deploy
LOCK_FILE="$LOG_DIR/.run-deploy.lock"   # under LOG_DIR so we know deploy user owns it
# DEPLOY_DIR lives under /home/ rather than /srv/ because the snap-confined
# docker on this host can only see /home/, /media/, /mnt/ via the snap
# `home` interface — putting docker-compose.yml in /srv/ would result in
# `docker compose pull` returning "no configuration file provided: not found"
# even though the file exists with correct perms. /home/deploy/ is the
# minimal-blast-radius location: owned by the deploy user, snap-readable.

mkdir -p "$LOG_DIR" "$DEPLOY_DIR"
LOG_FILE="$LOG_DIR/deploy-$(date +%Y%m%d-%H%M%S).log"
exec > >(tee -a "$LOG_FILE") 2>&1

# Mutex via flock — prevent two concurrent deploys from racing on $DEPLOY_DIR
# extracts and `docker compose up`. The workflow's `concurrency:` block already
# serialises GH-side, but a manual `ssh deploy@evergreen` while a workflow is
# in flight would otherwise interleave.
exec 9>"$LOCK_FILE"
flock -n 9 || { echo "::error::another deploy is already running (lock: $LOCK_FILE)"; exit 1; }

echo "[deploy] start $(date -Iseconds) caller=${SSH_CONNECTION:-?}"

# --- read + validate JSON payload -------------------------------------------
# Cap stdin at 16 MB to bound memory in case of malicious / runaway payload.
# Real payloads for this repo are <5 MB even with the tarball; 16 MB leaves
# headroom while preventing OOM via unbounded `cat`.
MAX_BYTES=$((16 * 1024 * 1024))
PAYLOAD=$(head -c $((MAX_BYTES + 1)))
if [ "${#PAYLOAD}" -gt "$MAX_BYTES" ]; then
  echo "::error::payload exceeds ${MAX_BYTES} bytes — refusing"
  exit 1
fi

echo "$PAYLOAD" | jq -e 'has("commit_sha") and has("deploy_payload_b64") and has("env") and has("ghcr")' >/dev/null \
  || { echo "::error::malformed payload (missing fields)"; exit 1; }

COMMIT_SHA=$(echo "$PAYLOAD" | jq -r '.commit_sha')
echo "[deploy] commit: $COMMIT_SHA"

# --- ghcr.io login --------------------------------------------------------
# The GH-hosted runner's GITHUB_TOKEN has `packages: read` scope on this
# repo's private images. Pass it here so the local docker daemon can pull;
# without this `docker compose pull` returns "denied: denied" on the
# private :latest images. Token expires when the workflow run ends, so each
# deploy gets a fresh one — no long-lived credential lives on evergreen.
GHCR_USER=$(echo "$PAYLOAD" | jq -r '.ghcr.user')
GHCR_TOKEN=$(echo "$PAYLOAD" | jq -r '.ghcr.token')
[[ -n "$GHCR_USER" && -n "$GHCR_TOKEN" ]] \
  || { echo "::error::missing ghcr.user / ghcr.token in payload"; exit 1; }
echo "$GHCR_TOKEN" | docker login ghcr.io -u "$GHCR_USER" --password-stdin >/dev/null \
  || { echo "::error::docker login ghcr.io failed"; exit 1; }
echo "[deploy] ghcr authenticated as $GHCR_USER"

# --- extract deploy artifacts -----------------------------------------------
# The tar contains: docker-compose.yml, init-db/, migrations/pg/,
# scripts/migrate.sh, scripts/deploy/run-deploy.sh (self-update payload).
# Extract into DEPLOY_DIR, replacing prior versions.
echo "$PAYLOAD" | jq -r '.deploy_payload_b64' | base64 -d | tar -xz -C "$DEPLOY_DIR"
[[ -f "$DEPLOY_DIR/docker-compose.yml" && -x "$DEPLOY_DIR/scripts/migrate.sh" ]] \
  || { echo "::error::tar extract incomplete"; exit 1; }

cd "$DEPLOY_DIR"

# --- self-update /srv/run-deploy.sh from repo (Track J5) --------------------
# The tarball now ships `scripts/deploy/run-deploy.sh` (the repo's
# version-controlled copy). Compare it against the live /srv/run-deploy.sh
# and replace if different.
#
# Self-update timing: the new version takes effect on the NEXT deploy — the
# currently-running script (this one) finishes with its own code. That's
# intentional. Mid-flight `exec` of a freshly-overwritten script breaks the
# tail of this run (logging FD, lock FD, $PAYLOAD vars all dropped). One
# deploy of lag is the right trade.
#
# Sudo NOPASSWD for `install` on /srv/run-deploy.sh is required (see
# /etc/sudoers.d/deploy-run-deploy on evergreen). Without it, this block
# logs a warning and skips — the deploy continues with the existing script.
SELF_UPDATE_SRC="$DEPLOY_DIR/scripts/deploy/run-deploy.sh"
SELF_UPDATE_DST=/srv/run-deploy.sh
if [ -f "$SELF_UPDATE_SRC" ]; then
  if ! cmp -s "$SELF_UPDATE_SRC" "$SELF_UPDATE_DST"; then
    if sudo -n install -m 0755 -o root -g root "$SELF_UPDATE_SRC" "$SELF_UPDATE_DST" 2>/dev/null; then
      echo "[deploy] self-updated $SELF_UPDATE_DST from repo (effective next deploy)"
    else
      echo "::warning::self-update of $SELF_UPDATE_DST skipped — sudo NOPASSWD missing or install failed"
    fi
  else
    echo "[deploy] $SELF_UPDATE_DST already matches repo copy"
  fi
else
  echo "::warning::no scripts/deploy/run-deploy.sh in payload — self-update skipped"
fi

# --- materialize .env from payload ------------------------------------------
# Subshell-scoped umask so .env is born 600 with no world-readable window,
# without affecting directory perms set above.
# jq's @sh applies shell-safe single-quote escaping (handles passwords with !"' etc.)
( umask 077 && echo "$PAYLOAD" | jq -r '.env | to_entries[] | "\(.key)=\(.value | @sh)"' > .env )
chmod 600 .env

# Required-env validation — fail loud if any expected key is empty.
# LEGACY_SYNC_* are included because compose's ${VAR:-default} fallback would
# silently mask a missing secret; explicit validation matches the prior workflow's
# behaviour (line 519) and prevents per-push reverts of operator-set values.
required=(
  DB_SERVER DB_NAME DB_USER DB_PASSWORD
  POSTGRES_DB POSTGRES_USER POSTGRES_PASSWORD
  LEGACY_SYNC_ENABLED LEGACY_SYNC_SHADOW_MODE
)
for var in "${required[@]}"; do
  grep -q "^${var}='..*'$" .env \
    || { echo "::error::env var $var missing or empty"; exit 1; }
done
echo "[deploy] artifacts staged at $DEPLOY_DIR, .env mode $(stat -c '%a' .env)"

# --- materialize Docker secret files from payload ---------------------------
# PR #89 (security audit 2026-05-14) moved DB_PASSWORD / POSTGRES_PASSWORD /
# VILLE_DB_PASSWORD / SLACK_WEBHOOK_URL out of compose `environment:` blocks
# into top-level `secrets:` that bind-mount /home/deploy/secrets/<name> at
# /run/secrets/<name>. The Rust hydrator (hotel-backend/src/secrets.rs) reads
# them at process start so downstream env::var(...) callers keep working.
#
# The workflow now ALSO ships a `.secrets` JSON object alongside `.env`
# (`{ db_password, postgres_password, ville_db_password, slack_webhook_url }`).
# Defensive `// {}` fallback: payloads from an OLDER workflow that lack the
# secrets block are tolerated — the loop is a no-op and the env-var fallback
# inside the Rust hydrator handles those (additive-migration window).
#
# Writes are atomic: `install` creates the destination + sets mode + owner
# in one syscall, so a concurrent `docker compose up` can never see a
# half-written file. Mode 0400 (read-only, owner-only) matches the same
# tight perms `docker secret create` would use in swarm mode.
SECRETS_DIR_HOST=/home/deploy/secrets
mkdir -p "$SECRETS_DIR_HOST"
chmod 0755 "$SECRETS_DIR_HOST"
chown deploy:docker "$SECRETS_DIR_HOST"

secrets_written=0
while IFS=$'\t' read -r secret_name secret_value; do
  [ -z "$secret_name" ] && continue
  # `printf %s` (no trailing newline) — the Rust hydrator strips one trailing
  # `\n` if present, but writing without one keeps `wc -c` matching the raw
  # value length, which is the operator's quickest sanity check.
  printf '%s' "$secret_value" \
    | install -m 0444 -o deploy -g docker /dev/stdin "$SECRETS_DIR_HOST/$secret_name"
  secrets_written=$((secrets_written + 1))
done < <(echo "$PAYLOAD" | jq -r '.secrets // {} | to_entries[] | "\(.key)\t\(.value)"')

if [ "$secrets_written" -eq 0 ]; then
  echo "::warning::no .secrets block in payload — falling back to env-var path"
else
  echo "[deploy] wrote $secrets_written secret file(s) to $SECRETS_DIR_HOST"
  # Print sizes only (no values) for operator confirmation.
  wc -c "$SECRETS_DIR_HOST"/* | sed 's|^|[deploy] |'
fi

# --- helpers ----------------------------------------------------------------

# Wait for a container to reach healthy (or running if no healthcheck).
wait_healthy() {
  local container=$1
  local timeout=${2:-60}
  local elapsed=0 health
  while [ $elapsed -lt $timeout ]; do
    health=$(docker inspect --format='{{if .State.Health}}{{.State.Health.Status}}{{else}}{{.State.Status}}{{end}}' "$container" 2>/dev/null || echo "not_found")
    case "$health" in
      healthy|running) return 0 ;;
    esac
    sleep 2
    elapsed=$((elapsed + 2))
  done
  echo "::error::$container unhealthy after ${timeout}s (last state: ${health:-unknown})"
  docker logs "$container" --tail 50 2>&1 || true
  return 1
}

# snap-docker on this host periodically returns ENETUNREACH on outbound TCP
# during container churn (well-documented snap mount-namespace flap). Wrap
# every ghcr.io op in 5 attempts. Remove this whole helper after the apt
# Docker CE migration lands and we've verified one full week without flap.
retry_compose() {
  local attempt sleep_s
  for attempt in 1 2 3 4 5; do
    if docker compose "$@"; then
      return 0
    fi
    sleep_s=$((attempt * 5))
    echo "::warning::compose $* attempt $attempt failed, retrying in ${sleep_s}s"
    sleep $sleep_s
  done
  echo "::error::compose $* failed after 5 attempts"
  return 1
}

# --- deploy -----------------------------------------------------------------
# Order: pull → up newdb → migrate → up rest → force-recreate workers
# Migrations run BEFORE new backend image starts so the schema is in place.

retry_compose pull
docker compose up -d newdb
wait_healthy new-hotel-db 120

./scripts/migrate.sh
./scripts/migrate.sh --site hfville

docker compose up -d
wait_healthy new-hotel-production-backend-1 90
wait_healthy new-hotel-production-web-1 60

# legacy profile: writeback worker (LISTEN'er → MSSQL via tiberius)
retry_compose --profile legacy pull writeback
docker compose --profile legacy up -d --force-recreate writeback

# legacy profile: CT-watcher (default-disabled; exits Ok(0) when LEGACY_SYNC_ENABLED=false)
retry_compose --profile legacy pull sync
docker compose --profile legacy up -d --force-recreate sync

# hfville profile: Ville's worker pair (LEGACY_SYNC_ENABLED can be true post-cutover)
retry_compose --profile hfville pull sync-hfville writeback-hfville
docker compose --profile hfville up -d --force-recreate sync-hfville writeback-hfville

echo "[deploy] complete"
docker compose ps

# --- post-deploy verification -----------------------------------------------

# Hard-fail if backend isn't healthy (covers crash-loop where State.Status=running but app is down)
if ! wait_healthy new-hotel-production-backend-1 30; then
  echo "::error::backend failed post-deploy healthcheck"
  docker logs new-hotel-production-backend-1 --tail 50
  exit 1
fi
echo "[deploy] backend healthy"

# Worker status warnings (don't fail the deploy — outbox jobs queue if writeback is down,
# recoverable when worker comes back, but operator needs to know).
WRITEBACK_STATUS=$(docker inspect --format='{{.State.Status}}' new-hotel-production-writeback-1 2>/dev/null || echo "not_found")
if [ "$WRITEBACK_STATUS" = "running" ]; then
  echo "[deploy] writeback running"
else
  echo "::warning::writeback not running ($WRITEBACK_STATUS) — outbox jobs will queue"
  docker logs new-hotel-production-writeback-1 --tail 50 2>&1 || true
fi

# CT-watcher: "running" or "exited" are both expected (controlled by LEGACY_SYNC_ENABLED).
SYNC_STATUS=$(docker inspect --format='{{.State.Status}}' new-hotel-production-sync-1 2>/dev/null || echo "not_found")
case "$SYNC_STATUS" in
  running) echo "[deploy] CT-watcher running (LEGACY_SYNC_ENABLED=true)" ;;
  exited)  echo "[deploy] CT-watcher exited cleanly (LEGACY_SYNC_ENABLED=false — expected default)" ;;
  not_found) echo "::warning::CT-watcher container missing — verify docker-compose.yml sync service block" ;;
  *) echo "::warning::CT-watcher unexpected state: $SYNC_STATUS"
     docker logs new-hotel-production-sync-1 --tail 50 2>&1 || true ;;
esac

echo "[deploy] done $(date -Iseconds) commit=$COMMIT_SHA log=$LOG_FILE"

# Log rotation — keep 90 days of deploy logs, drop older ones.
# Run as a final step so failures earlier don't skip cleanup if not strictly needed
# (this script's logs are small, ~100 KB each; bound is ample).
find "$LOG_DIR" -name 'deploy-*.log' -type f -mtime +90 -delete 2>/dev/null || true
