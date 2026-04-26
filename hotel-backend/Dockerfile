# syntax=docker/dockerfile:1.7
#
# Build pipeline:
#   builder — single stage that compiles all 3 binaries with BuildKit cache
#             mounts persisted across CI runs via `cache-to: type=gha,mode=max`
#             on the docker/build-push-action step.
#   runtime — minimal debian image with just the binaries.
#
# ## Caching strategy
#
# Three cache mounts persist across builds (the GHA Cache backend stores
# them between CI runs):
#
# - `/usr/local/cargo/registry` — cargo's downloaded crate registry. Saves
#   the registry-clone time (~5-15s) on every build.
# - `/usr/local/cargo/git`      — git dependencies (none today, kept for
#   future-proofing).
# - `/usr/src/app/target`       — cargo's incremental compile artifacts.
#   This is the BIG win: unchanged code never recompiles. A source-only
#   change rebuilds only the affected crate, not the dependency closure.
#
# We dropped cargo-chef (was: chef → planner → builder, ~5 layers) — its
# dep-layer-pinning value is fully subsumed by `--mount=type=cache` on
# target/, and the simpler Dockerfile is easier to reason about.
#
# Cache-mount contents are NOT included in the image layer (the mount is
# overlaid only during the RUN). That's why the build step `cp`s the
# binaries out to `/out/` first — the COPY in the runtime stage needs
# them in a layer-persisted path.
#
# mold linker shaves a large chunk off link time vs the default GNU ld.

# Pin to digest (Batch C of post-Phase-5.5 audit). Source: Docker Hub API
# `hub.docker.com/v2/repositories/library/rust/tags/1.89-bookworm` on 2026-04-26.
FROM rust:1.95-bookworm@sha256:adab7941580c74513aa3347f2d2a1f975498280743d29ec62978ba12e3540d3a AS builder
LABEL org.opencontainers.image.source=https://github.com/thehfhotel/new-hotel
LABEL org.opencontainers.image.licenses=Proprietary
RUN apt-get update \
    && apt-get install -y --no-install-recommends mold clang \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /usr/src/app

ENV SQLX_OFFLINE=true
# Force cargo to use the sparse registry protocol (faster than git clone of
# crates.io-index, which sccache/rustup-1.86+ defaults to anyway, but be
# explicit so future Rust upgrades don't change behavior under us).
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

COPY . .

# Build ONLY the production binaries. The `inspect_*` debug bins are
# explicitly excluded from the runtime image (Batch C of audit) — they
# remain available via `cargo run --bin inspect_*` locally for ops use,
# but never ship to production.
RUN --mount=type=cache,target=/usr/local/cargo/registry,id=cargo-registry \
    --mount=type=cache,target=/usr/local/cargo/git,id=cargo-git \
    --mount=type=cache,target=/usr/src/app/target,id=hotel-backend-target \
    cargo build --release --locked \
        --bin hotel-backend \
        --bin writeback \
        --bin sync \
        --bin backfill_rooms \
    && mkdir -p /out \
    && cp target/release/hotel-backend /out/ \
    && cp target/release/writeback /out/ \
    && cp target/release/sync /out/ \
    && cp target/release/backfill_rooms /out/

# ---- Runtime stage --------------------------------------------------------
# Pin to digest. Source: Docker Hub API `library/debian/tags/bookworm-slim`
# on 2026-04-26.
FROM debian:bookworm-slim@sha256:f9c6a2fd2ddbc23e336b6257a5245e31f996953ef06cd13a59fa0a1df2d5c252
LABEL org.opencontainers.image.source=https://github.com/thehfhotel/new-hotel
LABEL org.opencontainers.image.licenses=Proprietary

RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /out/hotel-backend /app/hotel-backend
COPY --from=builder /out/writeback /app/writeback
COPY --from=builder /out/sync /app/sync
COPY --from=builder /out/backfill_rooms /app/backfill_rooms

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
    CMD curl -fsS --max-time 3 http://localhost:3003/api/mode || exit 1

CMD ["./hotel-backend"]
