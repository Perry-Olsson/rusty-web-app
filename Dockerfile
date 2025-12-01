# Build stage
FROM rust:1.91.1 AS builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./
COPY app ./app
COPY runner ./runner

# Build the application in release mode
RUN cargo build --release

# Test stage
FROM builder AS tester

# Install cargo-nextest
RUN cargo install cargo-nextest --locked

# Tests will be run separately in CI via docker run

# Runtime stage
FROM debian:bookworm-slim AS runtime

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/app /app/app

# Expose the port (adjust if needed)
EXPOSE 8080

# Run the application
CMD ["/app/app"]
