#!/usr/bin/env bash
# Manual dev-mode deploy to evergreen. Bypasses CI/CD entirely.
#
# Builds the backend image LOCALLY (linux/amd64), tarballs it, scp's to
# evergreen, loads, force-recreates the writeback + backend containers.
# Total time: ~3 min on warm cache vs ~7 min via CI/CD.
#
# Usage:
#   ./scripts/deploy-dev.sh                  # build + deploy + restart
#   ./scripts/deploy-dev.sh --skip-build     # use existing local image
#   ./scripts/deploy-dev.sh --backend-only   # only backend, skip writeback
#   ./scripts/deploy-dev.sh --writeback-only # only writeback, skip backend

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
IMAGE="ghcr.io/thehfhotel/new-hotel-backend:dev-local"
TAR="/tmp/new-hotel-backend-dev.tar.gz"
SSH_HOST="evergreen"

SKIP_BUILD=0
RESTART_BACKEND=1
RESTART_WRITEBACK=1
for arg in "$@"; do
    case "$arg" in
        --skip-build) SKIP_BUILD=1 ;;
        --backend-only) RESTART_WRITEBACK=0 ;;
        --writeback-only) RESTART_BACKEND=0 ;;
        *) echo "unknown flag: $arg" >&2; exit 2 ;;
    esac
done

if [[ "$SKIP_BUILD" == "0" ]]; then
    echo "==> Building $IMAGE locally for linux/amd64..."
    cd "$REPO_ROOT/hotel-backend"
    docker buildx build \
        --platform linux/amd64 \
        --tag "$IMAGE" \
        --load \
        .
    echo "==> Build complete."
fi

echo "==> Saving + compressing image..."
docker save "$IMAGE" | gzip -2 > "$TAR"
echo "    $(du -h "$TAR" | cut -f1)"

echo "==> Transferring to evergreen..."
scp "$TAR" "$SSH_HOST:/tmp/new-hotel-backend-dev.tar.gz"

echo "==> Loading + restarting on evergreen..."
ssh "$SSH_HOST" bash -s <<REMOTE
set -euo pipefail
cd ~/new-hotel-production
docker load -i /tmp/new-hotel-backend-dev.tar.gz
# Re-tag :dev-local → :latest so docker compose picks it up via the
# existing image: line in docker-compose.yml.
docker tag $IMAGE ghcr.io/thehfhotel/new-hotel-backend:latest
rm /tmp/new-hotel-backend-dev.tar.gz

if [[ "$RESTART_BACKEND" == "1" ]]; then
    docker compose up -d --force-recreate backend
fi
if [[ "$RESTART_WRITEBACK" == "1" ]]; then
    docker compose --profile legacy up -d --force-recreate writeback
fi

sleep 3
docker compose ps backend writeback 2>/dev/null || docker compose --profile legacy ps writeback
REMOTE

rm "$TAR"
echo "==> Deploy complete."
