use crate::order::domain::model::order::{Order, OrderId};

pub trait OrderRepository {
    fn find(&self, id: OrderId) -> Result<Order, ()>;
    fn save(&self, order: Order) -> Result<(), ()>;
}

pub trait HaveOrderRepository {
    type OrderRepository: OrderRepository;
    fn order_repository(&self) -> &Self::OrderRepository;
}
