# Build stage
FROM rust:1.88-slim-bookworm AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy manifests first for dependency caching
COPY Cargo.toml Cargo.lock ./
COPY crates/core/Cargo.toml crates/core/
COPY crates/api/Cargo.toml crates/api/
COPY crates/adapters/Cargo.toml crates/adapters/
COPY crates/server/Cargo.toml crates/server/

# Create dummy source files to build dependencies
RUN mkdir -p crates/core/src crates/api/src crates/adapters/src crates/server/src && \
    echo "pub fn dummy() {}" > crates/core/src/lib.rs && \
    echo "pub fn dummy() {}" > crates/api/src/lib.rs && \
    echo "pub fn dummy() {}" > crates/adapters/src/lib.rs && \
    echo "fn main() {}" > crates/server/src/main.rs

# Build dependencies only
RUN cargo build --release --bin sats-escrow 2>/dev/null || true

# Copy real source code
COPY crates/core/src crates/core/src
COPY crates/api/src crates/api/src
COPY crates/adapters/src crates/adapters/src
COPY crates/server/src crates/server/src

# Copy test files if they exist (for api crate)
COPY crates/api/tests crates/api/tests

# Touch files to invalidate cache and rebuild
RUN touch crates/core/src/lib.rs \
    crates/api/src/lib.rs \
    crates/adapters/src/lib.rs \
    crates/server/src/main.rs

# Build the actual application
RUN cargo build --release --bin sats-escrow

# Runtime stage
FROM debian:bookworm-slim AS runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder
COPY --from=builder /app/target/release/sats-escrow /app/sats-escrow

# Create non-root user
RUN useradd -r -s /bin/false appuser && \
    chown -R appuser:appuser /app

USER appuser

# Expose port
EXPOSE 8080

# Set default environment variables
ENV PORT=8080
ENV RUST_LOG=info

# Run the binary
CMD ["/app/sats-escrow"]
