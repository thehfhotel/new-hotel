# syntax=docker/dockerfile:1.7
#
# Build pipeline:
#   chef    — base image with mold linker + cargo-chef installed
#   planner — produces recipe.json describing the dependency graph
#   builder — cooks deps from the recipe (cached), then compiles the real source
#   runtime — minimal debian image with just the binary
#
# mold linker shaves a large chunk off link time vs the default GNU ld.
# cargo-chef gives us deterministic dependency-only caching that survives any
# source-file change (the old "dummy main" trick re-cooked deps whenever
# Cargo.toml's [[bin]] list changed and was fragile around lockfile updates).

FROM rust:1.89-bookworm AS chef
RUN apt-get update \
    && apt-get install -y --no-install-recommends mold clang \
    && rm -rf /var/lib/apt/lists/*
RUN cargo install cargo-chef --locked
WORKDIR /usr/src/app

# ---- Plan stage: hash the dependency graph into recipe.json ---------------
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# ---- Build stage: cook deps (cached layer) then compile real source -------
FROM chef AS builder
ENV SQLX_OFFLINE=true

# Cook the recipe — this layer is cached as long as recipe.json is unchanged,
# i.e. as long as Cargo.toml/Cargo.lock haven't moved. Builds all bin targets'
# dependency closures via cargo-chef's synthesised dummy mains.
COPY --from=planner /usr/src/app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Now copy the real source and build the actual binary. Deps are already
# compiled in target/, so this only recompiles first-party code.
COPY . .
RUN cargo build --release --bin hotel-backend

# ---- Runtime stage --------------------------------------------------------
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /usr/src/app/target/release/hotel-backend /app/hotel-backend

RUN useradd -m -u 1000 appuser
USER appuser

EXPOSE 3003

ENV HOST=0.0.0.0
ENV PORT=3003
ENV RUST_LOG=hotel_backend=info,tower_http=info

CMD ["./hotel-backend"]
