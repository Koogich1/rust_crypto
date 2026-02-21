use std::sync::Arc;
use tokio::time::{self, Duration};
use diesel::prelude::*;
use uuid::Uuid;
use bigdecimal::{BigDecimal, FromPrimitive};
use chrono::Utc;

use crate::db::DbPool;
use crate::models::price_snapshot::NewPriceSnapshot;
use crate::schema::{coins, price_snapshots};
use super::coingecko::{fetch_prices, symbol_to_coingecko_id};

const DEFAULT_COINS: &[(&str, &str, i32)] = &[
    ("BTC", "Bitcoin", 8),
    ("ETH", "Ethereum", 18),
    ("SOL", "Solana", 9),
    ("USDT", "Tether", 6),
    ("USDC", "USD Coin", 6),
    ("BNB", "Binance Coin", 18),
    ("XRP", "Ripple", 6),
    ("ADA", "Cardano", 6),
    ("DOGE", "Dogecoin", 8),
];

pub fn init_coins(pool: &Arc<DbPool>) -> Result<(), diesel::result::Error> {
    let mut conn = pool.get().map_err(|e| {
        diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::Unknown,
            Box::new(e.to_string()),
        )
    })?;

    for (symbol, name, decimals) in DEFAULT_COINS {
        let exists = coins::table
            .filter(coins::symbol.eq(symbol))
            .select(coins::id)
            .first::<Uuid>(&mut conn)
            .optional()?;

        if exists.is_none() {
            diesel::insert_into(coins::table)
                .values((
                    coins::symbol.eq(symbol),
                    coins::name.eq(name),
                    coins::decimals.eq(decimals),
                ))
                .execute(&mut conn)?;
            
            tracing::info!("💾 Added coin: {} ({})", name, symbol);
        }
    }

    Ok(())
}

pub fn start_price_aggregator(pool: Arc<DbPool>) {
    tokio::spawn(async move {
        if let Err(e) = init_coins(&pool) {
            tracing::error!("❌ Failed to initialize coins: {}", e);
            return;
        }

        loop {
            tracing::info!("🔄 Starting price aggregation cycle...");
            
            let coins_list: Vec<(Uuid, String)> = {
                let mut conn = match pool.get() {
                    Ok(conn) => conn,
                    Err(e) => {
                        tracing::error!("❌ Failed to get DB connection: {}", e);
                        time::sleep(Duration::from_secs(10)).await;
                        continue;
                    }
                };
                
                match coins::table
                    .select((coins::columns::id, coins::columns::symbol))
                    .load::<(Uuid, String)>(&mut conn)
                {
                    Ok(coins) => coins,
                    Err(e) => {
                        tracing::error!("❌ Failed to load coins from DB: {}", e);
                        time::sleep(Duration::from_secs(10)).await;
                        continue;
                    }
                }
            };

            if coins_list.is_empty() {
                tracing::warn!("⚠️ No coins found in database");
                time::sleep(Duration::from_secs(30)).await;
                continue;
            }

            let symbols: Vec<&str> = coins_list.iter().map(|(_, s)| s.as_str()).collect();
            tracing::debug!("Coins to fetch: {:?}", symbols);

            match fetch_prices(&symbols).await {
                Ok(prices) => {
                    tracing::info!("✅ Fetched prices for {} coins", prices.len());

                    let mut conn = match pool.get() {
                        Ok(conn) => conn,
                        Err(e) => {
                            tracing::error!("❌ Failed to get DB connection for saving: {}", e);
                            time::sleep(Duration::from_secs(5)).await;
                            continue;
                        }
                    };

                    let now = Utc::now();
                    let received_at = Utc::now();
                    let mut saved_count = 0;

                    for (coin_id, symbol) in coins_list.iter() {
                        if let Some(gecko_id) = symbol_to_coingecko_id(symbol) {
                            if let Some(coin_data) = prices.get(gecko_id) {
                                let new_snapshot = NewPriceSnapshot {
                                    coin_id: *coin_id,
                                    price_usd: BigDecimal::from_f64(coin_data.usd).unwrap_or_default(),
                                    price_change_24h: BigDecimal::from_f64(coin_data.change_24h).unwrap_or_default(),
                                    volume_24h: BigDecimal::from_f64(coin_data.volume_24h).unwrap_or_default(),
                                    market_cap_usd: BigDecimal::from_f64(coin_data.market_cap).unwrap_or_default(),
                                    source: "coingecko",
                                    timestamp: now,
                                    received_at,
                                };

                                if let Err(e) = diesel::insert_into(price_snapshots::table)
                                    .values(&new_snapshot)
                                    .execute(&mut conn)
                                {
                                    tracing::error!("❌ Failed to insert snapshot for {}: {}", symbol, e);
                                } else {
                                    saved_count += 1;
                                }
                            }
                        }
                    }

                    tracing::info!("💾 Saved {} price snapshots to DB", saved_count);
                }
                Err(e) => {
                    tracing::error!("❌ Failed to fetch prices from CoinGecko: {}", e);
                }
            }

            tracing::info!("⏳ Sleeping for 60 seconds before next cycle...");
            time::sleep(Duration::from_secs(60)).await;
        }
    });
}