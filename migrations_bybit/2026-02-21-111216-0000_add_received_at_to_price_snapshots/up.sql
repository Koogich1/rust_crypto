-- Add received_at column to track API response latency
CREATE TABLE IF NOT EXISTS bybit_klines (
    id BIGSERIAL PRIMARY KEY,
    symbol VARCHAR(20) NOT NULL,
    interval VARCHAR(10) NOT NULL,
    start_time TIMESTAMPTZ NOT NULL,
    open_time TIMESTAMPTZ NOT NULL,
    open NUMERIC(20,8) NOT NULL,
    high NUMERIC(20,8) NOT NULL,
    low NUMERIC(20,8) NOT NULL,
    close NUMERIC(20,8) NOT NULL,
    volume NUMERIC(30,8) NOT NULL,
    turnover NUMERIC(30,8),
    confirm BOOLEAN DEFAULT false,

    UNIQUE(symbol, interval, start_time)
);

CREATE INDEX IF NOT EXISTS idx_bybit_symbol_time ON bybit_klines (symbol, start_time DESC);
CREATE INDEX IF NOT EXISTS idx_bybit_interval ON bybit_klines (interval);