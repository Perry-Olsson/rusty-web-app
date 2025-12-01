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

# Run tests - if this fails, the build fails
RUN cargo nextest run --release

# Create a marker file to indicate tests passed
RUN echo "tests passed" > /tmp/tests-passed

# Runtime stage
FROM debian:bookworm-slim AS runtime

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the test marker to ensure tests ran (creates dependency on tester stage)
COPY --from=tester /tmp/tests-passed /tmp/tests-passed

# Copy the binary from builder stage
COPY --from=builder /app/target/release/app /app/app

# Expose the port (adjust if needed)
EXPOSE 8080

# Run the application
CMD ["/app/app"]
