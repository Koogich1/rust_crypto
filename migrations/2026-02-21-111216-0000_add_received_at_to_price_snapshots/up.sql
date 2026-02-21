-- Add received_at column to track API response latency
ALTER TABLE price_snapshots
ADD COLUMN received_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
