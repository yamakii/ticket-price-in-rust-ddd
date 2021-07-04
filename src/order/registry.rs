use crate::order::usecase::order::OrderRegistrationUsecase;
use crate::order::domain::repository::order::OrderRepository;
use crate::order::infra::db::repository::order::DbOrderRepository;

pub fn order_registration_usecase() -> OrderRegistrationUsecase<impl OrderRepository> {
    OrderRegistrationUsecase::new(DbOrderRepository {})
}
