-- Your SQL goes here
CREATE TABLE metrics (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	coin_id UUID NOT NULL REFERENCES coins(id) ON DELETE CASCADE,
	metric_name VARCHAR(255) NOT NULL,
	metric_value NUMERIC(20, 10) NOT NULL,
	timestamp TIMESTAMPTZ NOT NULL,
	calculated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
)