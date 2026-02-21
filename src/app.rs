use axum::{Extension, Router};
use tower_http::trace::TraceLayer;
use crate::db::DbPool;
use crate::routers::coins::route_coin::{coin_routes};
use crate::routers::pricing::route_pricing::{route_pricing};
use crate::middleware::auth_middleware;
use std::sync::Arc;

pub fn create_app(pool: Arc<DbPool>) -> Router {
    Router::new()
        .merge(coin_routes())
        .merge(route_pricing())
        .layer(axum::middleware::from_fn(auth_middleware))
        .layer(Extension(pool))
        .layer(TraceLayer::new_for_http())
}