use crate::order::domain::repository::order::OrderRepository;
use crate::order::domain::model::order::{OrderId, Order};

#[derive(Debug)]
pub struct DbOrderRepository {}

impl OrderRepository for DbOrderRepository {
    fn find(&self, _id: OrderId) -> Result<Order, ()> {
        Result::Ok(Order::new(1))
    }
    fn save(&self, _order: Order) -> Result<(), ()> {
        Result::Ok(())
    }
}