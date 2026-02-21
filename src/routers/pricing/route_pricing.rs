use axum::routing::get;
use axum::{Extension, Json, Router};
use axum::extract::Query;
use serde::Deserialize;
use diesel::prelude::*;
use std::sync::Arc;
use uuid::Uuid;

use crate::db::DbPool;
use crate::models::price_snapshot::PriceSnapshot;
use crate::schema::{coins, price_snapshots};

#[derive(Deserialize, Debug)]
pub struct PriceQuery {
    pub coins: String,
}

pub fn route_pricing() -> Router {
    Router::new()
        .route("/prices", get(get_prices))
        .route("/prices/history", get(get_price_history))
}

async fn get_prices(
    Extension(pool): Extension<Arc<DbPool>>,
    Query(params): Query<PriceQuery>,
) -> Json<Vec<PriceSnapshot>> {
    let mut conn = pool.get().expect("Failed to get DB connection");
    let symbols: Vec<String> = params.coins.split(',').map(|s| s.to_uppercase()).collect();

    let coin_ids: Vec<Uuid> = coins::table
        .filter(coins::symbol.eq_any(&symbols))
        .select(coins::id)
        .load::<Uuid>(&mut conn)
        .expect("Failed to load coin IDs");

    if coin_ids.is_empty() {
        return Json(vec![]);
    }

    let mut results = Vec::new();
    for coin_id in coin_ids {
        if let Some(snapshot) = price_snapshots::table
            .filter(price_snapshots::coin_id.eq(coin_id))
            .order(price_snapshots::created_at.desc())
            .first::<PriceSnapshot>(&mut conn)
            .ok()
        {
            results.push(snapshot);
        }
    }

    Json(results)
}

async fn get_price_history(
    Extension(pool): Extension<Arc<DbPool>>,
    Query(params): Query<PriceQuery>,
) -> Json<Vec<PriceSnapshot>> {
    let mut conn = pool.get().expect("Failed to get DB connection");
    let symbols: Vec<String> = params.coins.split(',').map(|s| s.to_uppercase()).collect();

    let coin_ids: Vec<Uuid> = coins::table
        .filter(coins::symbol.eq_any(&symbols))
        .select(coins::id)
        .load::<Uuid>(&mut conn)
        .expect("Failed to load coin IDs");

    if coin_ids.is_empty() {
        return Json(vec![]);
    }

    let results = price_snapshots::table
        .filter(price_snapshots::coin_id.eq_any(coin_ids))
        .order(price_snapshots::created_at.desc())
        .limit(1000)
        .load::<PriceSnapshot>(&mut conn)
        .expect("Failed to load price history");

    Json(results)
}