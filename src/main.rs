mod schema;
mod models;
mod db;
mod migrations;
mod routers;
mod app;
mod pool;
mod services;
mod middleware;
mod bybit;
mod tracing;

use log::{info, error};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio;
use std::env;

use crate::app::create_app;
use crate::pool::create_pool;
use crate::bybit::{create_bybit_pool, BybitHistoricalService};
use std::sync::Arc;
// use crate::services::price_aggregator::{start_price_aggregator};

#[cfg(test)]
mod tests;

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    content: String,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
     let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let bybit_db_url = env::var("BYBIT_DATABASE_URL")
        .expect("BYBIT_DATABASE_URL must be set");

    migrations::run_migrations(&database_url, &bybit_db_url);

    tracing::setup_tracing();

    let pool = create_pool(&database_url);
    let pool_arc = Arc::new(pool);

    let bybit_pool = create_bybit_pool(&bybit_db_url);

    //start_price_aggregator(pool_arc.clone());

    let bybit_service = BybitHistoricalService::new(bybit_pool);
		
	// tokio::spawn(async move {
    //     if let Err(e) = bybit_service.backfill_symbol("BTCUSDT", "1", 1).await {
    //         error!("❌ Bybit backfill failed: {}", e);
    //     }
    // });

    let app = create_app(pool_arc.clone());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Starting server on {}", addr);

    let listener = match tokio::net::TcpListener::bind("0.0.0.0:3000").await {
        Ok(l) => l,
        Err(e) => {
            error!("❌ Failed to bind to port 3000: {}", e);
            return;
        }
    };
    axum::serve(listener, app).await.unwrap();
}