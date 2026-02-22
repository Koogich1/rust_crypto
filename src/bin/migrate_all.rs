use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::env;

pub const MAIN_MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");
pub const BYBIT_MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations_bybit");

fn run_main_migrations(database_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = PgConnection::establish(database_url)
        .expect("Failed to connect to main database");

    println!("🔄 Running main migrations...");
    conn.run_pending_migrations(MAIN_MIGRATIONS)
        .map_err(|e| e as Box<dyn std::error::Error>)?;
    println!("✅ Main migrations completed!");
    Ok(())
}

fn run_bybit_migrations(database_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = PgConnection::establish(database_url)
        .expect("Failed to connect to Bybit database");

    println!("🔄 Running Bybit migrations...");
    conn.run_pending_migrations(BYBIT_MIGRATIONS)
        .map_err(|e| e as Box<dyn std::error::Error>)?;
    println!("✅ Bybit migrations completed!");
    Ok(())
}

fn main() {
    dotenvy::dotenv().ok();

    let main_db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    let bybit_db_url = env::var("BYBIT_DATABASE_URL")
        .expect("BYBIT_DATABASE_URL must be set");

    if let Err(e) = run_main_migrations(&main_db_url) {
        eprintln!("❌ Main migration failed: {}", e);
        std::process::exit(1);
    }

    if let Err(e) = run_bybit_migrations(&bybit_db_url) {
        eprintln!("❌ Bybit migration failed: {}", e);
        std::process::exit(1);
    }

    println!("✅ All migrations completed successfully!");
}
