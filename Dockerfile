# Multi-stage build - собираем на сервере
FROM rust:1.88-slim-bookworm AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -rf src

COPY src ./src
COPY migrations ./migrations
COPY diesel.toml ./

RUN cargo build --release

# Ставим diesel CLI
RUN cargo install diesel_cli --no-default-features --features postgres

FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    libpq5 \
    postgresql-client \
    && rm -rf /var/lib/apt/lists/* \
    && useradd -r -u 1000 appuser

COPY --from=builder --chown=appuser:appuser /app/target/release/axum-tracing-example /app/server
COPY --from=builder --chown=appuser:appuser /usr/local/cargo/bin/diesel /usr/local/bin/diesel
COPY --chown=appuser:appuser migrations ./migrations
COPY --chown=appuser:appuser entrypoint.sh /entrypoint.sh

RUN chmod +x /entrypoint.sh

USER appuser

EXPOSE 3000

ENTRYPOINT ["/entrypoint.sh"]
