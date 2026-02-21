#!/bin/sh
set -e

echo "🔄 Running migrations..."

# Ждём пока БД будет готова
until pg_isready -h db -U "$DB_USER" > /dev/null 2>&1; do
  echo "⏳ Waiting for database..."
  sleep 2
done

# Применяем миграции через diesel
export PGPASSWORD="$DB_PASSWORD"
diesel migration run --database-url="postgres://$DB_USER:$DB_PASSWORD@db:5432/$DB_NAME"

echo "✅ Migrations complete!"

# Запускаем приложение
echo "🚀 Starting application..."
exec /app/server
