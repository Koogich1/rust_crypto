mod schema;
mod models;
mod db;
mod routers;
mod app;
mod pool;
mod services;
mod middleware;

use log::info;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio;
use std::env;

use crate::app::create_app;
use crate::pool::create_pool;
use std::sync::Arc;
use crate::services::price_aggregator::{start_price_aggregator, init_coins};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use diesel::prelude::*;

#[cfg(test)]
mod tests;

#[derive(Debug, Serialize, Deserialize)]

struct Message {
    content: String,
}

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    // Запускаем миграции
    let mut conn = PgConnection::establish(&database_url)
        .expect("Failed to connect to database");
    MIGRATIONS
        .run_pending(&mut conn)
        .expect("Failed to run migrations");
    tracing::info!("✅ Migrations applied successfully");

    let pool = create_pool(&database_url);
    let pool_arc = Arc::new(pool);

    // Инициализируем монеты (создаём, если нет)
    if let Err(e) = init_coins(&pool_arc) {
        tracing::error!("❌ Failed to initialize coins: {}", e);
    }

    // Запускаем агрегатор цен (собирает цены каждые 10 секунд)
    start_price_aggregator(pool_arc.clone());

    let app = create_app(pool_arc.clone());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}