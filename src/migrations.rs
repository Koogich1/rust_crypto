use diesel::prelude::*;
use log::info;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MAIN_MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");
pub const BYBIT_MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations_bybit");

pub fn setup_migrations(database_url: &str, migrations: EmbeddedMigrations, db_name: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut conn = PgConnection::establish(database_url)
        .map_err(|e| format!("Failed to connect to {} database: {}", db_name, e))?;
    
    info!("🔄 Running {} migrations...", db_name);
    conn.run_pending_migrations(migrations)
        .map_err(|e| format!("Failed to run {} migrations: {}", db_name, e))?;
    info!("✅ {} migrations completed!", db_name);
    Ok(())
}

pub fn run_migrations(database_url: &str, bybit_db_url: &str){
    if let Err(e) = setup_migrations(&database_url, MAIN_MIGRATIONS, "main") {
        tracing::error!("❌ {}", e);
        std::process::exit(1);
    }

    if let Err(e) = setup_migrations(&bybit_db_url, BYBIT_MIGRATIONS, "Bybit") {
        tracing::error!("❌ {}", e);
        std::process::exit(1);
    }
}