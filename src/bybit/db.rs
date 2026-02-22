use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;

pub type BybitDbPool = Pool<ConnectionManager<PgConnection>>;

pub fn create_bybit_pool(database_url: &str) -> BybitDbPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .max_size(10)
        .build(manager)
        .expect("Failed to create Bybit pool")
}
