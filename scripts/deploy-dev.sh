#!/usr/bin/env bash
# Manual dev-mode deploy: build on evergreen, restart in place.
#
# Bypasses CI/CD entirely. Steps:
#   1. rsync the hotel-backend source to evergreen
#   2. ssh: `docker build` directly on evergreen (linux/amd64 native)
#   3. ssh: re-tag :dev-local → :latest, force-recreate writeback + backend
#
# Why on-evergreen build:
#   - No cross-compile (target arch matches host)
#   - No tarball scp (image stays in evergreen's daemon)
#   - No local Docker Desktop required
#   - Buildx layer cache on evergreen persists across runs (warm builds <1min)
#
# Usage:
#   ./scripts/deploy-dev.sh                  # rsync + build + restart both
#   ./scripts/deploy-dev.sh --skip-build     # rsync + restart only
#   ./scripts/deploy-dev.sh --skip-rsync     # build with whatever's on evergreen
#   ./scripts/deploy-dev.sh --backend-only   # only restart backend
#   ./scripts/deploy-dev.sh --writeback-only # only restart writeback
#   ./scripts/deploy-dev.sh --backfill       # also run backfill_rooms after restart

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SSH_HOST="evergreen"
REMOTE_BUILD_DIR="/home/nut/new-hotel-dev"
IMAGE="ghcr.io/thehfhotel/new-hotel-backend:dev-local"

SKIP_BUILD=0
SKIP_RSYNC=0
RESTART_BACKEND=1
RESTART_WRITEBACK=1
RUN_BACKFILL=0
for arg in "$@"; do
    case "$arg" in
        --skip-build) SKIP_BUILD=1 ;;
        --skip-rsync) SKIP_RSYNC=1 ;;
        --backend-only) RESTART_WRITEBACK=0 ;;
        --writeback-only) RESTART_BACKEND=0 ;;
        --backfill) RUN_BACKFILL=1 ;;
        *) echo "unknown flag: $arg" >&2; exit 2 ;;
    esac
done

if [[ "$SKIP_RSYNC" == "0" ]]; then
    echo "==> Rsyncing source to evergreen ($REMOTE_BUILD_DIR)..."
    ssh "$SSH_HOST" "mkdir -p $REMOTE_BUILD_DIR"
    rsync -az --delete \
        --exclude target/ \
        --exclude node_modules/ \
        --exclude .next/ \
        --exclude .git/ \
        --exclude .claude/worktrees/ \
        --exclude '*.log' \
        "$REPO_ROOT/hotel-backend/" "$SSH_HOST:$REMOTE_BUILD_DIR/hotel-backend/"
fi

if [[ "$SKIP_BUILD" == "0" ]]; then
    echo "==> Building $IMAGE on evergreen..."
    ssh "$SSH_HOST" "cd $REMOTE_BUILD_DIR/hotel-backend && docker build --tag $IMAGE ."
fi

echo "==> Re-tagging + restarting on evergreen..."
ssh "$SSH_HOST" bash -s <<REMOTE
set -euo pipefail
cd ~/new-hotel-production
docker tag $IMAGE ghcr.io/thehfhotel/new-hotel-backend:latest

if [[ "$RESTART_BACKEND" == "1" ]]; then
    docker compose up -d --force-recreate backend
fi
if [[ "$RESTART_WRITEBACK" == "1" ]]; then
    docker compose --profile legacy up -d --force-recreate writeback
fi
sleep 3

if [[ "$RUN_BACKFILL" == "1" ]]; then
    echo "==> Running backfill_rooms..."
    docker compose --profile backfill run --rm backfill-rooms
fi

docker compose ps backend writeback 2>/dev/null || docker compose --profile legacy ps writeback
REMOTE

echo "==> Deploy complete."
