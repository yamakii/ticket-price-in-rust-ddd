use crate::order::domain::model::order::{Order, OrderId};
use crate::order::domain::repository::order::OrderRepository;
use chrono::Local;

#[derive(Debug)]
pub struct DbOrderRepository {}

impl OrderRepository for DbOrderRepository {
    fn find(&self, _id: OrderId) -> Result<Order, ()> {
        Result::Ok(Order::new(1, 1, Local::now(), Vec::new()))
    }
    fn save(&self, _order: Order) -> Result<(), ()> {
        Result::Ok(())
    }
}
