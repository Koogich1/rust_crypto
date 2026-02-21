use diesel::prelude::*;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use bigdecimal::BigDecimal;
use crate::schema::price_snapshots;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Identifiable, Debug, Serialize, Deserialize)]
#[diesel(table_name = price_snapshots)]
#[diesel(primary_key(id))]
pub struct PriceSnapshot {
    pub id: Uuid,
    pub coin_id: Uuid,
    pub price_usd: BigDecimal,
    pub price_change_24h: BigDecimal,
    pub volume_24h: BigDecimal,
    pub market_cap_usd: BigDecimal,
    pub source: String,
    pub timestamp: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = price_snapshots)]
pub struct NewPriceSnapshot<'a> {
    pub coin_id: Uuid,
    pub price_usd: BigDecimal,
    pub price_change_24h: BigDecimal,
    pub volume_24h: BigDecimal,
    pub market_cap_usd: BigDecimal,
    pub source: &'a str,
    pub timestamp: DateTime<Utc>,
}