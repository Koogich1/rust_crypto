use diesel::prelude::*;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::schema::coins;
use serde::Serialize;

#[derive(Queryable, Identifiable, Debug, Serialize)]
#[diesel(table_name = coins)]
#[diesel(primary_key(id))]
pub struct Coin {
    pub id: Uuid,
    pub symbol: String,
    pub name: String,
    pub decimals: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = coins)]
pub struct NewCoin<'a> {
    pub symbol: &'a str,
    pub name: &'a str,
    pub decimals: i32,
}
