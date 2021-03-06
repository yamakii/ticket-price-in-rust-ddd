pub mod model;
pub mod repository;
pub mod schema;
pub mod service;

// DB接続
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use std::env;
use std::sync::Arc;

pub fn make_connection_pool() -> Arc<Pool<ConnectionManager<PgConnection>>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Arc::new(Pool::builder().max_size(2).build(manager).unwrap())
}
