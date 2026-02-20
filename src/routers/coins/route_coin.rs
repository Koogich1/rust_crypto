use axum::routing::get;
use axum::{Json, Router};
use diesel::prelude::*;
use crate::schema::coins;
use crate::models::coin::Coin;
use axum::extract::Extension;
use crate::db::DbPool;
use std::sync::Arc;

pub fn routes() -> Router {
    Router::new()
        .route("/coins", get(get_coins))
}

/// Получает список монет из БД
/// Extension берёт Arc<DbPool> из состояния приложения
async fn get_coins(Extension(pool): Extension<Arc<DbPool>>) -> Json<Vec<Coin>> {
    let mut conn = pool.get().expect("Failed to get a connection from the pool");

    let coins_list = coins::table.load::<Coin>(&mut conn).expect("load coins");

    Json(coins_list)
}