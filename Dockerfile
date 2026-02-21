FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef

WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -rf src

COPY src ./src
COPY migrations ./migrations
COPY diesel.toml ./

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/* \
    && useradd -r -u 1000 appuser

COPY --from=builder --chown=appuser:appuser /app/target/release/axum-tracing-example /app/server
COPY --chown=appuser:appuser migrations ./migrations

USER appuser

EXPOSE 3000

ENTRYPOINT ["/app/server"]
