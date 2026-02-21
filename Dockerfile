# Runtime образ - только для запуска готового бинарника
FROM debian:bookworm-slim

WORKDIR /app

# Устанавливаем runtime зависимости
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/* \
    && useradd -r -u 1000 appuser

# Копируем готовый бинарник (собирается локально)
COPY --chown=appuser:appuser target/release/axum-tracing-example /app/server

# Копируем миграции
COPY --chown=appuser:appuser migrations ./migrations

USER appuser

EXPOSE 3000

ENTRYPOINT ["/app/server"]
