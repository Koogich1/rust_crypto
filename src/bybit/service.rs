use serde::Deserialize;
use reqwest;
use std::time::Duration;
use std::env;
use std::cmp;
use chrono::{DateTime, Utc, Duration as ChronoDuration};
use diesel::prelude::*;
use tracing::{info, warn};

use crate::bybit::db::BybitDbPool;
use crate::bybit::models::NewBybitKline;
use crate::bybit::schema::bybit_klines;

#[derive(Deserialize, Debug, Clone)]
pub struct BybitApiKline {
    #[serde(rename = "startTime")]
    pub start_time: i64,
    #[serde(rename = "openPrice")]
    pub open: String,
    pub high: String,
    pub low: String,
    #[serde(rename = "closePrice")]
    pub close: String,
    pub volume: String,
    pub turnover: String,
}

pub struct BybitConfig {
    pub api_key: String,
    pub secret: String,
    pub base_url: String,
    pub rate_limit_ms: u64,
}

impl BybitConfig {
    pub fn from_env() -> Self {
        Self {
            api_key: env::var("BYBIT_API_KEY").unwrap_or_default(),
            secret: env::var("BYBIT_SECRET").unwrap_or_default(),
            base_url: env::var("BYBIT_API_URL")
                .unwrap_or_else(|_| "https://api.bybit.com".to_string()),
            rate_limit_ms: 500,
        }
    }
}

pub struct BybitHistoricalService {
    client: reqwest::Client,
    config: BybitConfig,
    pool: BybitDbPool,
}

impl BybitHistoricalService {
    pub fn new(pool: BybitDbPool) -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(Duration::from_secs(30))
                .user_agent("crypto-ml-bot/0.1")
                .build()
                .expect("Failed to create HTTP client"),
            config: BybitConfig::from_env(),
            pool,
        }
    }

    pub async fn fetch_history(
        &self,
        symbol: &str,
        interval: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<BybitApiKline>, Box<dyn std::error::Error + Send + Sync>> {
        let mut all_klines = Vec::new();
        let mut current_start = start;

        while current_start < end {
            let url = format!(
                "{}/v5/market/kline?category=spot&symbol={}&interval={}&start={}&end={}&limit=200",
                self.config.base_url,
                symbol,
                interval,
                current_start.timestamp_millis(),
                end.timestamp_millis()
            );

            info!("📡 Fetching: {}", url);

            let response = self.client
                .get(&url)
                .header("X-BAPI-API-KEY", &self.config.api_key)
                .send()
                .await?;

            if !response.status().is_success() {
                warn!("⚠️  API error: {}", response.status());
                tokio::time::sleep(Duration::from_secs(2)).await;
                continue;
            }

            let data: serde_json::Value = response.json().await?;

            let list_size = data["result"]["list"].as_array().map(|a| a.len()).unwrap_or(0);
            info!("📥 Received {} klines", list_size);
            
            if let Some(list) = data["result"]["list"].as_array() {
                for item in list {
                    if let Some(kline) = self.parse_kline_array(item, symbol, interval) {
                        all_klines.push(kline);
                    }
                }
            }

            current_start += ChronoDuration::minutes(200);
            
            tokio::time::sleep(Duration::from_millis(self.config.rate_limit_ms)).await;
        }

        Ok(all_klines)
    }

    fn parse_kline_array(
        &self,
        arr: &serde_json::Value,
        _symbol: &str,
        _interval: &str,
    ) -> Option<BybitApiKline> {
        let items = arr.as_array()?;
        if items.len() < 7 { return None; }

        Some(BybitApiKline {
            start_time: items[0].as_str()?.parse().ok()?,
            open: items[1].as_str()?.to_string(),
            high: items[2].as_str()?.to_string(),
            low: items[3].as_str()?.to_string(),
            close: items[4].as_str()?.to_string(),
            volume: items[5].as_str()?.to_string(),
            turnover: items[6].as_str()?.to_string(),
        })
    }

    pub fn save_klines(&self, symbol: &str, interval: &str, klines: &[BybitApiKline]) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
        let mut conn = self.pool.get()?;
        let mut count = 0;

        for kline in klines {
            let open_time = DateTime::from_timestamp_millis(kline.start_time)
                .unwrap_or_else(|| Utc::now());

            let new_kline = NewBybitKline {
                symbol: symbol.to_string(),
                interval: interval.to_string(),
                start_time: open_time,
                open_time: open_time,
                open: kline.open.parse()?,
                high: kline.high.parse()?,
                low: kline.low.parse()?,
                close: kline.close.parse()?,
                volume: kline.volume.parse()?,
                turnover: Some(kline.turnover.parse()?),
                confirm: Some(true),
            };

            diesel::insert_into(bybit_klines::table)
                .values(&new_kline)
                .on_conflict((bybit_klines::symbol, bybit_klines::interval, bybit_klines::start_time))
                .do_update()
                .set((
                    bybit_klines::open.eq(&new_kline.open),
                    bybit_klines::high.eq(&new_kline.high),
                    bybit_klines::low.eq(&new_kline.low),
                    bybit_klines::close.eq(&new_kline.close),
                    bybit_klines::volume.eq(&new_kline.volume),
                    bybit_klines::turnover.eq(&new_kline.turnover),
                    bybit_klines::confirm.eq(&new_kline.confirm),
                ))
                .execute(&mut conn)?;
            
            count += 1;
        }

        Ok(count)
    }

    pub async fn backfill_symbol(
        &self,
        symbol: &str,
        interval: &str,
        years: i32,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let end = Utc::now();
        let start = end - ChronoDuration::days(365 * years as i64);

        info!("🔄 Starting backfill: {} {} for {} years", symbol, interval, years);

        let mut current_start = start;
        let mut total_saved = 0;

        while current_start < end {
            let batch_end = cmp::min(
                current_start + ChronoDuration::minutes(200),
                end
            );

            let klines = self.fetch_history(symbol, interval, current_start, batch_end).await?;
            info!("💾 Fetched {} klines, saving...", klines.len());

            if !klines.is_empty() {
                let saved = self.save_klines(symbol, interval, &klines)?;
                total_saved += saved;
                info!("✅ Saved {} klines (total: {})", saved, total_saved);
            }

            current_start = batch_end;
        }

        info!("✅ Backfill complete: {} total klines saved for {}", total_saved, symbol);
        Ok(())
    }
}
