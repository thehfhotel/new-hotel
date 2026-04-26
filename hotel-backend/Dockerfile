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

FROM rust:1.89-bookworm AS builder
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

RUN --mount=type=cache,target=/usr/local/cargo/registry,id=cargo-registry \
    --mount=type=cache,target=/usr/local/cargo/git,id=cargo-git \
    --mount=type=cache,target=/usr/src/app/target,id=hotel-backend-target \
    cargo build --release \
        --bin hotel-backend \
        --bin writeback \
        --bin sync \
        --bin backfill_rooms \
        --bin inspect_booking \
        --bin inspect_schema \
    && mkdir -p /out \
    && cp target/release/hotel-backend /out/ \
    && cp target/release/writeback /out/ \
    && cp target/release/sync /out/ \
    && cp target/release/backfill_rooms /out/ \
    && cp target/release/inspect_booking /out/ \
    && cp target/release/inspect_schema /out/

# ---- Runtime stage --------------------------------------------------------
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /out/hotel-backend /app/hotel-backend
COPY --from=builder /out/writeback /app/writeback
COPY --from=builder /out/sync /app/sync
COPY --from=builder /out/backfill_rooms /app/backfill_rooms
COPY --from=builder /out/inspect_booking /app/inspect_booking
COPY --from=builder /out/inspect_schema /app/inspect_schema

RUN useradd -m -u 1000 appuser
USER appuser

EXPOSE 3003

ENV HOST=0.0.0.0
ENV PORT=3003
ENV RUST_LOG=hotel_backend=info,tower_http=info

CMD ["./hotel-backend"]
