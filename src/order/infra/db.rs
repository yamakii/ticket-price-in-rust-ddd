pub mod model;
pub mod repository;
pub mod schema;
pub mod service;

// DB接続
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use diesel::r2d2::{ConnectionManager, Pool};
use std::sync::Arc;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn make_connection_pool() -> Arc<Pool<ConnectionManager<PgConnection>>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Arc::new(Pool::builder().max_size(2).build(manager).unwrap())
}
