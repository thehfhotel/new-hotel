# syntax=docker/dockerfile:1.7
#
# Build pipeline (cargo-chef edition, restored 2026-04-26):
#
#   chef    — base layer with rust toolchain + cargo-chef pinned.
#   planner — runs `cargo chef prepare` to derive recipe.json (a hash of
#             the full dependency graph). recipe.json changes ONLY when
#             Cargo.toml / Cargo.lock change.
#   builder — runs `cargo chef cook --release` first (compiles the entire
#             dep closure once), then COPYs the source tree and runs
#             `cargo build` for the production binaries.
#   runtime — minimal debian image with just the binaries.
#
# ## Why cargo-chef came back
#
# Previous iteration relied solely on `--mount=type=cache` for target/.
# That works WITHIN a build, but the cache mount lives at the BuildKit
# layer — sibling builds (build-frontend vs build-backend) share it
# only when they run on the same runner with the same gha cache scope.
# In CI today every job pulls its own gha-scoped cache, and a CI
# `cache-to: type=gha,mode=max` upload of a 4GB target/ directory takes
# longer than the savings it produces on the next run.
#
# cargo-chef gives us proper LAYER caching: the `cargo chef cook` RUN
# is keyed on recipe.json (deps-only). When recipe.json is unchanged
# (the common case for source-only changes), Docker's image-layer cache
# turns the cook step into a hit and we skip the ~800-crate dep
# recompile. Source-only changes drop from ~5min to ~1min.
#
# ## Layer caching — the actual cargo-chef win (fixed 2026-06-27)
#
# CRITICAL: target/ and the crate registry are NOT `--mount=type=cache`. They
# live in the cook IMAGE LAYER, which is exported/restored via
# `cache-to/from: type=gha,mode=max` (build-backend job). So a cache hit on the
# cook layer carries the COMPILED ~800-crate dep closure with it, and the build
# step only recompiles workspace code (~4.5min → ~1min on source-only changes).
#
# This is the whole point of cargo-chef — but it was previously DEFEATED by
# mounting target/ as a cache mount: BuildKit excludes mount contents from image
# layers and `cache-to: gha` does not export them, so the cook-layer cache hit
# restored an EMPTY target/ and the build recompiled every crate on EVERY run.
# (Build-log proof + root-cause in commit that removed the mounts.)
#
# Only /sccache stays a cache mount (compiler cache; excluded from the layer so
# it can't bloat it). The build step `cp`s the binaries to /out/ so the runtime
# stage can COPY them without dragging in the whole target/ tree.
#
# mold linker shaves a large chunk off link time vs the default GNU ld.

# ---- chef: base image with cargo-chef installed ---------------------------
# Pin to digest (Batch C of post-Phase-5.5 audit). Source: Docker Hub API
# `hub.docker.com/v2/repositories/library/rust/tags/1.89-bookworm` on 2026-04-26.
FROM rust:1.96-bookworm@sha256:a339861ae23e9abb272cea45dfafde21760d2ce6577a70f8a926153677902663 AS chef
LABEL org.opencontainers.image.source=https://github.com/thehfhotel/new-hotel
LABEL org.opencontainers.image.licenses=Proprietary
# Crate-download resilience (set EARLY so it covers cargo-chef install + cook +
# build). The crates.io CDN intermittently resets an HTTP/2 connection mid-
# transfer, which cargo surfaces as an opaque "curl failed" — and with
# multiplexing a single reset can kill several concurrent downloads at once.
# One connection per download (multiplexing off) + more retries lets those
# transient blips self-heal. Complemented by the `cargo fetch` retry loops
# below (which isolate the network step from the offline compile). Recurring
# CI flake root-caused 2026-06-27.
ENV CARGO_NET_RETRY=10
ENV CARGO_HTTP_MULTIPLEXING=false
RUN apt-get update \
    && apt-get install -y --no-install-recommends mold clang \
    && rm -rf /var/lib/apt/lists/*
# Pin cargo-chef to a specific version (security + reproducibility).
# 0.1.71 = latest stable as of 2026-04-26. Bumps surface as ordinary
# Dockerfile edits — there is no Dependabot ecosystem for this binary.
RUN --mount=type=cache,target=/usr/local/cargo/registry,id=cargo-registry \
    set -eu; \
    n=0; until cargo install cargo-chef --locked --version 0.1.71; do \
        n=$((n+1)); \
        if [ "$n" -ge 4 ]; then echo "cargo-chef install failed after $n attempts" >&2; exit 1; fi; \
        echo "cargo-chef install attempt $n failed; retrying in $((n*10))s" >&2; sleep "$((n*10))"; \
    done
# sccache (compiler cache): wraps rustc and caches build outputs keyed
# on the exact crate inputs, so unchanged crates are linked from cache
# instead of recompiled. Local-disk mode here (no GHA backend reachable
# inside Docker BuildKit); the SCCACHE_DIR cache mount below persists
# the cache across image rebuilds via BuildKit's gha cache scope.
#
# Install via pre-built binary tarball from mozilla/sccache releases —
# `cargo install sccache` compiles from source and costs ~5min on a cold
# layer cache (regression observed on PR #68's first measurement). The
# curl+tar path completes in seconds. Mirrors the host-side install used
# by mozilla-actions/sccache-action in the test-backend job.
#
# Pinned v0.8.2 (latest stable 2026-05-13). Bump version + verify
# `sccache --version` if upstream cuts a new release.
ARG SCCACHE_VERSION=v0.8.2
ARG SCCACHE_ARCH=x86_64-unknown-linux-musl
RUN --mount=type=cache,target=/tmp/sccache-dl,id=sccache-tarball \
    set -eux; \
    curl -fsSL --retry 5 --retry-delay 2 --retry-all-errors -o /tmp/sccache-dl/sccache.tar.gz \
      "https://github.com/mozilla/sccache/releases/download/${SCCACHE_VERSION}/sccache-${SCCACHE_VERSION}-${SCCACHE_ARCH}.tar.gz"; \
    tar -xzf /tmp/sccache-dl/sccache.tar.gz -C /tmp/sccache-dl --strip-components=1; \
    install -m 755 /tmp/sccache-dl/sccache /usr/local/bin/sccache; \
    sccache --version
WORKDIR /usr/src/app

ENV SQLX_OFFLINE=true
# Force cargo to use the sparse registry protocol (faster than git clone of
# crates.io-index, which sccache/rustup-1.86+ defaults to anyway, but be
# explicit so future Rust upgrades don't change behavior under us).
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
# Wire sccache as the rustc wrapper. SCCACHE_DIR points to a BuildKit
# cache mount path used by the cook + build RUNs below; SCCACHE_CACHE_SIZE
# caps the on-disk cache so it can't grow without bound across CI runs.
ENV RUSTC_WRAPPER=sccache
ENV SCCACHE_DIR=/sccache
ENV SCCACHE_CACHE_SIZE=2G
# Match the workflow-level setting: skip cargo incremental in Docker too.
# Incremental artifacts inflate the target/ cache mount without buying
# anything in a CI build (every run starts from a known dep set).
ENV CARGO_INCREMENTAL=0

# ---- planner: derive recipe.json (deps fingerprint) -----------------------
# This stage's only job is to produce recipe.json. It re-runs whenever any
# file in the build context changes, but the OUTPUT (recipe.json) is
# stable when only source files change — so the downstream COPY of
# recipe.json into the builder stage hits cache.
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# ---- builder: cook deps once, then build the binaries --------------------
FROM chef AS builder

# Step 1: cook the dependency closure. This RUN is keyed on recipe.json
# alone — when deps don't change, the cook LAYER is a gha cache hit and the
# command never executes, and (because target/ + the registry live in the
# layer now, not a mount) that hit carries the compiled deps with it. That's
# the cargo-chef value prop, finally working.
COPY --from=planner /usr/src/app/recipe.json recipe.json
# Cook fetches + compiles the whole dep closure; retry it through transient
# crate-download blips (a retry re-pulls the crate that failed and relinks).
RUN --mount=type=cache,target=/sccache,id=sccache-backend \
    set -eu; \
    n=0; until cargo chef cook --release --recipe-path recipe.json; do \
        n=$((n+1)); \
        if [ "$n" -ge 4 ]; then echo "cargo chef cook failed after $n attempts" >&2; exit 1; fi; \
        echo "cargo chef cook attempt $n failed; retrying in $((n*15))s" >&2; sleep "$((n*15))"; \
    done; \
    sccache --show-stats

# Step 2: COPY the full source. The deps are already compiled in target/ — it
# came in with the cook layer (target/ is dockerignored, so this COPY can't
# clobber it) — so this build step only compiles workspace code.
# Build ONLY the production binaries. The `inspect_*` debug bins are
# explicitly excluded from the runtime image (Batch C of audit) — they
# remain available via `cargo run --bin inspect_*` locally for ops use,
# but never ship to production.
COPY . .
# On a warm cook layer the registry is already present, so `cargo fetch` is a
# no-op and the compile is fully OFFLINE. On a COLD layer (recipe.json changed)
# fetch downloads the closure — retried with linear backoff so a transient
# "curl failed" self-heals, and the compile stays OFFLINE so a CDN blip can
# never fail the build itself.
RUN --mount=type=cache,target=/sccache,id=sccache-backend \
    set -eu; \
    n=0; until cargo fetch --locked; do \
        n=$((n+1)); \
        if [ "$n" -ge 5 ]; then echo "cargo fetch failed after $n attempts" >&2; exit 1; fi; \
        echo "cargo fetch attempt $n failed; retrying in $((n*10))s" >&2; sleep "$((n*10))"; \
    done; \
    cargo build --release --locked --offline \
        --bin hotel-backend \
        --bin writeback \
        --bin sync \
        --bin backfill_rooms \
        --bin backfill_checkin_rooms \
        --bin backfill_legacy_checkins \
        --bin backfill_legacy_bookings \
        --bin backfill_payment_ledger \
        --bin backfill_receipt_payments \
        --bin backfill_room_calendar \
        --bin migrate_legacy \
        --bin create_user; \
    sccache --show-stats; \
    mkdir -p /out; \
    cp target/release/hotel-backend /out/; \
    cp target/release/writeback /out/; \
    cp target/release/sync /out/; \
    cp target/release/backfill_rooms /out/; \
    cp target/release/backfill_checkin_rooms /out/; \
    cp target/release/backfill_legacy_checkins /out/; \
    cp target/release/backfill_legacy_bookings /out/; \
    cp target/release/backfill_payment_ledger /out/; \
    cp target/release/backfill_receipt_payments /out/; \
    cp target/release/backfill_room_calendar /out/; \
    cp target/release/migrate_legacy /out/; \
    cp target/release/create_user /out/; \
    rm -rf target
# `rm -rf target` keeps THIS layer tiny (just /out, ~70MB). target/ — the full
# compiled closure (GBs) — already lives in the cached COOK layer below us; the
# binaries are in /out. Without the rm, the build layer would carry the whole
# target/ into the gha cache export on EVERY commit: slow (~40s extra) and, worse,
# it bloats the cache toward the 10GB limit and can evict the cook layer →
# random cold builds. The cook layer (deps) is unaffected; only this build layer
# shrinks. (Perf pass 2026-06-27.)

# ---- Runtime stage --------------------------------------------------------
# Pin to digest. Source: Docker Hub API `library/debian/tags/bookworm-slim`
# on 2026-04-26.
FROM debian:bookworm-slim@sha256:0104b334637a5f19aa9c983a91b54c89887c0984081f2068983107a6f6c21eeb
LABEL org.opencontainers.image.source=https://github.com/thehfhotel/new-hotel
LABEL org.opencontainers.image.licenses=Proprietary

# fonts-tlwg-loma-ttf: Debian DFSG TLWG "Loma" Thai TrueType face, kept as the
# FALLBACK for the reconstructed Thai national-ID card (render/thai_id_card.rs).
# Installs /usr/share/fonts/truetype/tlwg/Loma-Bold.ttf. Pure font data, no
# runtime library to link.
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    fonts-tlwg-loma-ttf \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Thai national-ID card assets, bundled in the image so the render endpoint is
# self-contained (no host bind-mount required):
#   - DilleniaUPC Bold — the EXACT face KPThaiNationalIDCard.exe draws with; the
#     default THAI_ID_FONT_PATH (see config.rs). Sourced from the licensed Windows
#     UPC font set (upcdb.ttf); metrics drive the baseline math in thai_id_card.rs.
#   - thai_id_template.png — the authoritative blank card (504×322) extracted from
#     the reference's `Untitled_1__Copy_` resource; the default THAI_ID_TEMPLATE_PATH.
COPY assets/fonts/DilleniaUPC-Bold.ttf /usr/share/fonts/truetype/dillenia/DilleniaUPC-Bold.ttf
COPY assets/thai_id_template.png /app/assets/thai_id_template.png

COPY --from=builder /out/hotel-backend /app/hotel-backend
COPY --from=builder /out/writeback /app/writeback
COPY --from=builder /out/sync /app/sync
COPY --from=builder /out/backfill_rooms /app/backfill_rooms
COPY --from=builder /out/backfill_checkin_rooms /app/backfill_checkin_rooms
COPY --from=builder /out/backfill_legacy_checkins /app/backfill_legacy_checkins
COPY --from=builder /out/backfill_legacy_bookings /app/backfill_legacy_bookings
COPY --from=builder /out/backfill_payment_ledger /app/backfill_payment_ledger
COPY --from=builder /out/backfill_receipt_payments /app/backfill_receipt_payments
COPY --from=builder /out/backfill_room_calendar /app/backfill_room_calendar
COPY --from=builder /out/migrate_legacy /app/migrate_legacy
COPY --from=builder /out/create_user /app/create_user

RUN useradd -m -u 1000 appuser
USER appuser

EXPOSE 3003

ENV HOST=0.0.0.0
ENV PORT=3003
ENV RUST_LOG=hotel_backend=info,tower_http=info

# Container-level healthcheck. Same endpoint the docker-compose healthcheck
# uses — present here so `docker run` users + non-compose orchestrators
# also get the signal.
HEALTHCHECK --interval=30s --timeout=5s --start-period=30s --retries=3 \
    CMD curl -fsS --max-time 3 http://localhost:3003/health || exit 1

CMD ["./hotel-backend"]
