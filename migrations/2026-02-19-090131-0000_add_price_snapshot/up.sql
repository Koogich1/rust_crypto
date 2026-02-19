-- Your SQL goes here
CREATE TABLE price_snapshots (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	coin_id UUID NOT NULL REFERENCES coins(id) ON DELETE CASCADE,
	price_usd NUMERIC(20, 10) NOT NULL,
	price_change_24h NUMERIC(10, 2) NOT NULL,
	volume_24h NUMERIC(20, 2) NOT NULL,
	market_cap_usd NUMERIC(20, 2) NOT NULL,
	source VARCHAR(255) NOT NULL,
	timestamp TIMESTAMPTZ NOT NULL,
	created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);