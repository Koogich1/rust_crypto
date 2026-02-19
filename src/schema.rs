// @generated automatically by Diesel CLI.

diesel::table! {
    aggregated_prices (id) {
        id -> Uuid,
        coin_id -> Uuid,
        average_price_usd -> Numeric,
        min_price_usd -> Numeric,
        max_price_usd -> Numeric,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    coins (id) {
        id -> Uuid,
        #[max_length = 10]
        symbol -> Varchar,
        #[max_length = 100]
        name -> Varchar,
        decimals -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    metrics (id) {
        id -> Uuid,
        coin_id -> Uuid,
        #[max_length = 255]
        metric_name -> Varchar,
        metric_value -> Numeric,
        timestamp -> Timestamptz,
        calculated_at -> Timestamptz,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    price_snapshots (id) {
        id -> Uuid,
        coin_id -> Uuid,
        price_usd -> Numeric,
        price_change_24h -> Numeric,
        volume_24h -> Numeric,
        market_cap_usd -> Numeric,
        #[max_length = 255]
        source -> Varchar,
        timestamp -> Timestamptz,
        created_at -> Timestamptz,
    }
}

diesel::joinable!(aggregated_prices -> coins (coin_id));
diesel::joinable!(metrics -> coins (coin_id));
diesel::joinable!(price_snapshots -> coins (coin_id));

diesel::allow_tables_to_appear_in_same_query!(aggregated_prices, coins, metrics, price_snapshots,);
