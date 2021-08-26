use crate::order::domain::model::order::{Order, OrderId};
use crate::order::domain::repository::order::OrderRepository;
use crate::order::infra::db::establish_connection;
use chrono::Local;
use diesel::RunQueryDsl;

#[derive(Debug)]
pub struct DbOrderRepository {}

impl OrderRepository for DbOrderRepository {
    fn find(&self, _id: OrderId) -> Result<Order, ()> {
        use crate::order::infra::db::schema::movies::dsl::*;

        let connection = establish_connection();
        let _result = movies.load::<Movie>(&connection).unwrap();
        Result::Ok(Order::new(1, 1, Local::now(), Vec::new()))
    }
    fn save(&self, _order: Order) -> Result<(), ()> {
        Result::Ok(())
    }
}

#[derive(Queryable)]
pub struct Movie {
    pub id: i32,
    pub title: String,
    pub basic_price: i32,
}
