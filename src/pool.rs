use tracing_subscriber::EnvFilter;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use crate::db::DbPool;

pub fn create_pool(database_url: &str) -> DbPool {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("axum_tracing_example=error,tower_http=warn"))
                .unwrap(),
        )
        .init();

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .max_size(10) 
        .build(manager)
        .expect("Failed to create pool");

		pool
}