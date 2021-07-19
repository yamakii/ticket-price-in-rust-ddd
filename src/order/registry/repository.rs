use crate::order::infra::db::repository::order::DbOrderRepository;

pub struct DbRepositoryRegistry {
    order: DbOrderRepository,
}

impl DbRepositoryRegistry {
    pub fn new() -> Self {
        DbRepositoryRegistry {
            order: DbOrderRepository {},
        }
    }
    pub fn order(&self) -> &DbOrderRepository {
        &self.order
    }
}
