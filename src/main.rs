mod schema;
mod models;
mod db;
mod routers;
mod app;
mod pool;
mod services;
mod middleware;
mod bybit;

use log::info;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio;
use std::env;
use tracing_subscriber::EnvFilter;
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use crate::app::create_app;
use crate::pool::create_pool;
use crate::bybit::{create_bybit_pool, BybitHistoricalService};
use std::sync::Arc;
use crate::services::price_aggregator::{start_price_aggregator, init_coins};

pub const MAIN_MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");
pub const BYBIT_MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations_bybit");

#[cfg(test)]
mod tests;

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    content: String,
}

fn run_migrations(database_url: &str, migrations: EmbeddedMigrations, db_name: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut conn = PgConnection::establish(database_url)
        .map_err(|e| format!("Failed to connect to {} database: {}", db_name, e))?;
    
    info!("🔄 Running {} migrations...", db_name);
    conn.run_pending_migrations(migrations)
        .map_err(|e| format!("Failed to run {} migrations: {}", db_name, e))?;
    info!("✅ {} migrations completed!", db_name);
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("error,tower_http=warn,crypto-aggregator=info"))
                .unwrap(),
        )
        .init();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let bybit_db_url = env::var("BYBIT_DATABASE_URL")
        .expect("BYBIT_DATABASE_URL must be set");

    // Запускаем миграции при старте
    if let Err(e) = run_migrations(&database_url, MAIN_MIGRATIONS, "main") {
        tracing::error!("❌ {}", e);
        std::process::exit(1);
    }

    if let Err(e) = run_migrations(&bybit_db_url, BYBIT_MIGRATIONS, "Bybit") {
        tracing::error!("❌ {}", e);
        std::process::exit(1);
    }

    let pool = create_pool(&database_url);
    let pool_arc = Arc::new(pool);

    let bybit_pool = create_bybit_pool(&bybit_db_url);

    // Инициализируем монеты (создаём, если нет)
    if let Err(e) = init_coins(&pool_arc) {
        tracing::error!("❌ Failed to initialize coins: {}", e);
    }

    start_price_aggregator(pool_arc.clone());

    let bybit_service = BybitHistoricalService::new(bybit_pool);
    tokio::spawn(async move {
        if let Err(e) = bybit_service.backfill_symbol("BTCUSDT", "1", 1).await {
            tracing::error!("❌ Bybit backfill failed: {}", e);
        }
    });

    let app = create_app(pool_arc.clone());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Starting server on {}", addr);

    let listener = match tokio::net::TcpListener::bind("0.0.0.0:3000").await {
        Ok(l) => l,
        Err(e) => {
            tracing::error!("❌ Failed to bind to port 3000: {}", e);
            return;
        }
    };
    axum::serve(listener, app).await.unwrap();
}