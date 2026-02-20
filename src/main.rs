mod schema;
mod models;
mod db;
mod routers;
mod app;
mod pool;

use log::info;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio;
use std::env;

use crate::app::create_app;
use crate::pool::create_pool;
use std::sync::Arc;

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

    let pool = create_pool(&database_url);
    
    let app = create_app(Arc::new(pool));  

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}