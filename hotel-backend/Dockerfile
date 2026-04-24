# Build stage
FROM rust:1.85-bookworm AS builder

WORKDIR /usr/src/app

# Enable sqlx offline mode (uses .sqlx/ cache instead of live DB)
ENV SQLX_OFFLINE=true

# Copy the Cargo files first to cache dependencies
COPY Cargo.toml Cargo.lock* ./

# Create dummy sources matching Cargo.toml bin targets to cache dependencies
RUN mkdir -p src/bin && echo "fn main() {}" > src/main.rs && echo "fn main() {}" > src/bin/migrate_legacy.rs && echo "fn main() {}" > src/bin/ville_sync.rs
RUN cargo build --release
RUN rm -rf src

# Copy the sqlx offline cache and actual source code
COPY .sqlx .sqlx
COPY src ./src

# Build the application
RUN find src -name "*.rs" -exec touch {} + && cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /usr/src/app/target/release/hotel-backend /app/hotel-backend

# Create a non-root user
RUN useradd -m -u 1000 appuser
USER appuser

# Expose the port
EXPOSE 3003

# Set environment variables
ENV HOST=0.0.0.0
ENV PORT=3003
ENV RUST_LOG=hotel_backend=info,tower_http=info

# Run the application
CMD ["./hotel-backend"]
