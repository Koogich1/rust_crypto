use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::env;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations_bybit");

fn run_migrations(database_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = PgConnection::establish(database_url)
        .expect("Failed to connect to Bybit database");
    
    println!("🔄 Running Bybit migrations...");

    conn.run_pending_migrations(MIGRATIONS)
        .map_err(|e| e as Box<dyn std::error::Error>)?;

    println!("✅ Bybit migrations completed!");
    Ok(())
}

fn main() {
    dotenvy::dotenv().ok();
    
    let bybit_db_url = env::var("BYBIT_DATABASE_URL")
        .expect("BYBIT_DATABASE_URL must be set for migrations");
    
    if let Err(e) = run_migrations(&bybit_db_url) {
        eprintln!("❌ Migration failed: {}", e);
        std::process::exit(1);
    }
}