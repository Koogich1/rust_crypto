FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef

WORKDIR /app

ENV CARGO_BUILD_JOBS=1

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./

RUN cargo chef prepare --recipe-path recipe.json

FROM lukemathwalker/cargo-chef:latest-rust-1 AS planner

WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY migrations ./migrations
COPY diesel.toml ./

RUN cargo chef cook --release --recipe-path recipe.json

FROM lukemathwalker/cargo-chef:latest-rust-1 AS builder

WORKDIR /app

ENV CARGO_BUILD_JOBS=1

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY migrations ./migrations
COPY diesel.toml ./
COPY --from=planner /app/target ./target

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/* \
    && useradd -r -u 1000 appuser

COPY --from=builder --chown=appuser:appuser /app/target/release/crypto-aggregator /app/server
COPY --chown=appuser:appuser migrations ./migrations

USER appuser

EXPOSE 3000

ENTRYPOINT ["/app/server"]