# Build stage
FROM rust:1.91.1 AS builder

ARG APP_NAME
FROM builder AS dev

WORKDIR /${APP_NAME}

CMD ["scripts/run.sh"]

FROM builder as runtime-builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY app ./app
COPY runner ./runner

RUN cargo build --release

FROM runtime-builder AS tester

RUN cargo install cargo-nextest --locked

FROM debian:bookworm-slim AS runtime

RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=runtime-builder /app/target/release/app /app/app

EXPOSE 8080

# Run the application
CMD ["/app/app"]
