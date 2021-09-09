use crate::order::infra::db::repository::order::DbOrderRepository;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use std::sync::Arc;

pub struct DbRepositoryRegistry {
    order: DbOrderRepository,
}

impl DbRepositoryRegistry {
    pub fn new(pool: Arc<Pool<ConnectionManager<PgConnection>>>) -> Self {
        DbRepositoryRegistry {
            order: DbOrderRepository::new(pool),
        }
    }
    pub fn order(&self) -> &DbOrderRepository {
        &self.order
    }
}
