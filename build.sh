#!/bin/bash
# Скрипт для сборки релизного бинарника и деплоя

set -e

echo "🔨 Building release binary..."
cargo build --release

echo "✅ Build complete!"
echo "📦 Binary size: $(du -h target/release/axum-tracing-example | cut -f1)"

echo ""
echo "📝 Next steps:"
echo "1. git add target/release/axum-tracing-example"
echo "2. git commit -m 'release: new build'"
echo "3. git push"
echo ""
echo "On server:"
echo "1. git pull"
echo "2. docker compose -f docker-compose.prod.yml down"
echo "3. docker compose -f docker-compose.prod.yml up -d"
