# =============================================================================
# Stage 1: Chef - готовим рецепт зависимостей
# =============================================================================
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef

WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo chef prepare --recipe-path recipe.json

# =============================================================================
# Stage 2: Planner - готовим зависимости (кешируется между сборками)
# =============================================================================
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

# Копируем рецепт из chef
COPY --from=chef /app/recipe.json ./recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

# =============================================================================
# Stage 3: Builder - собираем приложение
# =============================================================================
FROM lukemathwalker/cargo-chef:latest-rust-1 AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY migrations ./migrations
COPY diesel.toml ./

# Копируем скомпилированные зависимости из planner
COPY --from=planner /app/target ./target

# Собираем только код приложения (быстро!)
RUN cargo build --release

# =============================================================================
# Stage 4: Runtime - минимальный образ для запуска
# =============================================================================
FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    libpq5 \
    wget \
    && rm -rf /var/lib/apt/lists/* \
    && useradd -r -u 1000 appuser

COPY --from=builder --chown=appuser:appuser /app/target/release/crypto-aggregator /app/server
COPY --chown=appuser:appuser migrations ./migrations

USER appuser

EXPOSE 3000

HEALTHCHECK --interval=30s --timeout=10s --start-period=40s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost:3000/health || exit 1

ENTRYPOINT ["/app/server"]
