use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};

use crate::bybit::schema::bybit_klines;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = bybit_klines)]
#[diesel(primary_key(id))]
pub struct BybitKline {
    pub id: i64,
    pub symbol: String,
    pub interval: String,
    pub start_time: DateTime<Utc>,
    pub open_time: DateTime<Utc>,
    pub open: BigDecimal,
    pub high: BigDecimal,
    pub low: BigDecimal,
    pub close: BigDecimal,
    pub volume: BigDecimal,
    pub turnover: Option<BigDecimal>,
    pub confirm: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = bybit_klines)]
pub struct NewBybitKline {
    pub symbol: String,
    pub interval: String,
    pub start_time: DateTime<Utc>,
    pub open_time: DateTime<Utc>,
    pub open: BigDecimal,
    pub high: BigDecimal,
    pub low: BigDecimal,
    pub close: BigDecimal,
    pub volume: BigDecimal,
    pub turnover: Option<BigDecimal>,
    pub confirm: Option<bool>,
}
