use axum::{Extension, Router};
use tower_http::trace::TraceLayer;
use crate::db::DbPool;
use crate::routers::coins::route_coin;
use std::sync::Arc;

pub fn create_app(pool: Arc<DbPool>) -> Router {
    Router::new()
        .merge(route_coin::routes())
        .layer(Extension(pool))
        .layer(TraceLayer::new_for_http())
}