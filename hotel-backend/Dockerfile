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
# layer — sibling builds (build-backend vs build-ville-sync) share it
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
# ## Cache mounts (kept)
#
# `--mount=type=cache,target=...` mounts persist across runs via the
# gha cache backend on docker/build-push-action. Used here for:
#   - /usr/local/cargo/registry — sparse-protocol crate index + .crate downloads
#   - /usr/local/cargo/git      — git deps (none today, future-proofing)
#   - /usr/src/app/target       — incremental compile artifacts
# These complement the layer cache; cargo-chef + mounts are additive
# (chef makes the COOK layer cacheable; mounts make incremental compiles
# inside any RUN faster).
#
# Cache-mount contents are NOT included in the image layer, so the build
# step `cp`s the binaries to /out/ before they get unmounted.
#
# mold linker shaves a large chunk off link time vs the default GNU ld.

# ---- chef: base image with cargo-chef installed ---------------------------
# Pin to digest (Batch C of post-Phase-5.5 audit). Source: Docker Hub API
# `hub.docker.com/v2/repositories/library/rust/tags/1.89-bookworm` on 2026-04-26.
FROM rust:1.89-bookworm@sha256:948f9b08a66e7fe01b03a98ef1c7568292e07ec2e4fe90d88c07bb14563c84ff AS chef
LABEL org.opencontainers.image.source=https://github.com/thehfhotel/new-hotel
LABEL org.opencontainers.image.licenses=Proprietary
RUN apt-get update \
    && apt-get install -y --no-install-recommends mold clang \
    && rm -rf /var/lib/apt/lists/*
# Pin cargo-chef to a specific version (security + reproducibility).
# 0.1.71 = latest stable as of 2026-04-26. Bumps surface as ordinary
# Dockerfile edits — there is no Dependabot ecosystem for this binary.
RUN --mount=type=cache,target=/usr/local/cargo/registry,id=cargo-registry \
    cargo install cargo-chef --locked --version 0.1.71
WORKDIR /usr/src/app

ENV SQLX_OFFLINE=true
# Force cargo to use the sparse registry protocol (faster than git clone of
# crates.io-index, which sccache/rustup-1.86+ defaults to anyway, but be
# explicit so future Rust upgrades don't change behavior under us).
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

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
# alone — when deps don't change, Docker reuses the prior layer and
# the cook command never executes. That's the cargo-chef value prop.
COPY --from=planner /usr/src/app/recipe.json recipe.json
RUN --mount=type=cache,target=/usr/local/cargo/registry,id=cargo-registry \
    --mount=type=cache,target=/usr/local/cargo/git,id=cargo-git \
    --mount=type=cache,target=/usr/src/app/target,id=hotel-backend-target \
    cargo chef cook --release --recipe-path recipe.json

# Step 2: COPY the full source. Now the deps are pre-compiled in target/
# (via the cache mount), so this build step only compiles workspace code.
# Build ONLY the production binaries. The `inspect_*` debug bins are
# explicitly excluded from the runtime image (Batch C of audit) — they
# remain available via `cargo run --bin inspect_*` locally for ops use,
# but never ship to production.
COPY . .
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

# ---- test: compile + run the integration suite ---------------------------
#
# This stage exists for CI use — `docker buildx build --target test --network host`
# from the test-backend job. It is NOT part of the runtime image lineage
# (the runtime FROM scratch starts a new chain below, so this stage is
# only built when a target=test is explicitly requested).
#
# ## Why it lives here, not in a separate Dockerfile
#
# It SHARES the `hotel-backend-target` cache mount with the builder stage.
# When test-backend runs in CI, the gha-cache backend supplies cooked-deps
# layers from the prior build-backend run (same `cache-from: gha,scope=backend`),
# so the test compile only does the test-specific code on top — no
# 800-crate dep recompile. Pre-cargo-chef this was a separate ~2.5-minute
# cargo compile path; with chef + shared cache it's expected to drop
# substantially.
#
# ## Network requirement
#
# Integration tests connect to PostgreSQL via DATABASE_URL. The test
# stage's RUN inherits the build host's network when buildx is invoked
# with `network: host`. The workflow's `services: postgres` block
# exposes PG on localhost:5439 of the runner; the test container reaches
# it through the host network namespace.
#
# DATABASE_URL is supplied as a build-arg (passed by docker/build-push-action
# from a GH secret + literal port). It is NOT secret in the conventional
# sense — the value is the CI-only DSN with the throwaway TEST_POSTGRES_PASSWORD
# — but we still keep it as a build-arg rather than baking it into the
# image because this stage is never pushed to a registry.
FROM builder AS test
ARG DATABASE_URL
ENV DATABASE_URL=${DATABASE_URL}
# SQLX_OFFLINE=true is inherited from the `chef` stage. The lib crate's
# sqlx::query!() invocations are validated against the .sqlx/ cache file
# (committed to repo) at COMPILE time. The test code itself uses
# sqlx::query() (string-based, runtime-validated against DATABASE_URL)
# so it needs PG reachable at RUN time but NOT at compile time.
# Run cargo test directly inside the build. Result of the RUN = result
# of the test suite; failing tests fail the build (and therefore the CI
# job). Same `--locked` and `--test-threads=1` knobs as the runner-mode
# invocation — see the `Run backend tests` step's commit body for why
# threads=1 is mandatory (shared-DB race; flake fix `1e61bd6`).
RUN --mount=type=cache,target=/usr/local/cargo/registry,id=cargo-registry \
    --mount=type=cache,target=/usr/local/cargo/git,id=cargo-git \
    --mount=type=cache,target=/usr/src/app/target,id=hotel-backend-target \
    cargo test --release --locked -- --test-threads=1
# Sentinel marker — buildx would otherwise return zero output for a
# pure-RUN stage; this gives the image a writable layer above `builder`.
RUN touch /tests-passed

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
