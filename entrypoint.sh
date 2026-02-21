#!/bin/sh
set -e

echo "🔄 Running migrations..."

# Ждём пока БД будет готова
until pg_isready -h db -U "$DB_USER" > /dev/null 2>&1; do
  echo "⏳ Waiting for database..."
  sleep 2
done

export PGPASSWORD="$DB_PASSWORD"

# Применяем миграции через psql
for migration in migrations/*/up.sql; do
  if [ -f "$migration" ]; then
    echo "📄 Applying $(basename $migration)..."
    psql -h db -U "$DB_USER" -d "$DB_NAME" -f "$migration" || true
  fi
done

echo "✅ Migrations complete!"

# Запускаем приложение
echo "🚀 Starting application..."
exec /app/server
