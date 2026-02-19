-- Your SQL goes here
CREATE TABLE aggregated_prices (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	coin_id UUID NOT NULL REFERENCES coins(id) ON DELETE CASCADE,
	average_price_usd NUMERIC(20, 10) NOT NULL,
	min_price_usd NUMERIC(20, 10) NOT NULL,
	max_price_usd NUMERIC(20, 10) NOT NULL,
	updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);