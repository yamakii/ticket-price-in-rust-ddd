use crate::order::domain::model::order::{OrderId, Order};

pub trait OrderRepository {
    fn find(&self, id: OrderId) -> Result<Order, ()>;
    fn save(&self, order: Order) -> Result<(), ()>;
}