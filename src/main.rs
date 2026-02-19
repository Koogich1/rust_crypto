use axum::{Router};
use log::info;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;
#[derive(Debug, Serialize, Deserialize)]
struct Message {
    content: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("axum_tracing_example=error,tower_http=warn"))
                .unwrap(),
        )
        .init();

    let app = Router::new()
        // .merge(routes::health::router())
        // .merge(routes::convert::router())
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}