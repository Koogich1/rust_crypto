// @generated automatically by Diesel CLI.

diesel::table! {
    bybit_klines (id) {
        id -> Int8,
        #[max_length = 20]
        symbol -> Varchar,
        #[max_length = 10]
        interval -> Varchar,
        start_time -> Timestamptz,
        open_time -> Timestamptz,
        open -> Numeric,
        high -> Numeric,
        low -> Numeric,
        close -> Numeric,
        volume -> Numeric,
        turnover -> Nullable<Numeric>,
        confirm -> Nullable<Bool>,
    }
}
