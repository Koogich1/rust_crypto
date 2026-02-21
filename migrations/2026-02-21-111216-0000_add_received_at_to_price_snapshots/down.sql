-- Remove received_at column from price_snapshots
ALTER TABLE price_snapshots
DROP COLUMN received_at;
