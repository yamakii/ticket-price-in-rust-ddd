use crate::order::domain::repository::order::OrderRepository;
use crate::order::domain::model::order::Order;

#[derive(Debug)]
pub struct OrderRegistrationUsecase<T: OrderRepository> {
    repo: T,
}

impl<T: OrderRepository> OrderRegistrationUsecase<T> {
    pub fn new(repo: T) -> OrderRegistrationUsecase<T> {
        OrderRegistrationUsecase { repo }
    }
}

impl<T: OrderRepository> OrderRegistrationUsecase<T> {
    pub fn action(&self) -> Result<(), ()> {
        let order = Order::new(1);
        self.repo.save(order)
    }
}
