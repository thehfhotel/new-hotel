# Build stage
FROM rust:1.75-bookworm as builder

WORKDIR /usr/src/app

# Copy the Cargo files first to cache dependencies
COPY Cargo.toml Cargo.lock* ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy the actual source code
COPY src ./src

# Build the application
RUN touch src/main.rs && cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
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
