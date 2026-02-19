-- Your SQL goes here
CREATE TABLE coins (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	symbol VARCHAR(10) NOT NULL,
	name VARCHAR(100) NOT NULL,
	decimals INTEGER NOT NULL,
	created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()  
);

SELECT diesel_manage_updated_at('coins');