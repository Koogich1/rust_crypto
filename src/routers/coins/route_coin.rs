use axum::routing::get;
use axum::{Json, Router};
use diesel::prelude::*;
use crate::schema::coins;
use crate::models::coin::Coin;
use axum::extract::Extension;
use crate::db::DbPool;

pub fn routes() -> Router {
    Router::new()
        .route("/coins", get(get_coins))
}

async fn get_coins(Extension(pool): Extension<DbPool>) -> Json<Vec<Coin>> {
		let mut conn = pool.get().expect("Failed to get a connection from the pool");

    let coinst_list = coins::table.load::<Coin>(&mut conn).expect("load coins");

		axum::Json(coinst_list)
}