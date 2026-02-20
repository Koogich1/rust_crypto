use axum_test::TestServer;
use diesel::prelude::*;
use std::env;
use std::sync::Arc;

use crate::models::coin::{Coin, NewCoin};
use crate::schema::coins;
use crate::pool::create_pool;
use crate::app::create_app;

#[tokio::test]
async fn test_get_coins_returns_list() {
    dotenvy::dotenv_override().ok();

    let test_database_url = env::var("TEST_DATABASE_URL")
        .expect("TEST_DATABASE_URL must be set");
    let pool = Arc::new(create_pool(&test_database_url));

    {
        let mut conn = pool.get().expect("Failed to get DB connection");
        diesel::delete(coins::table)
            .execute(&mut conn)
            .expect("Failed to clear coins table");
    }

    {
        let mut conn = pool.get().expect("Failed to get DB connection");
        diesel::insert_into(coins::table)
            .values(vec![
                NewCoin {
                    symbol: "BTC",
                    name: "Bitcoin",
                    decimals: 8,
                },
                NewCoin {
                    symbol: "ETH",
                    name: "Ethereum",
                    decimals: 18,
                },
            ])
            .execute(&mut conn)
            .expect("Failed to insert test coins");
    }

    let app = create_app(pool.clone());
    let server = TestServer::new(app).expect("Failed to start test server");

    let response = server.get("/coins").await;

    response.assert_status_ok();

    let body: Vec<Coin> = response.json();
    assert_eq!(body.len(), 2);
    assert_eq!(body[0].symbol, "BTC");
    assert_eq!(body[1].symbol, "ETH");
}
