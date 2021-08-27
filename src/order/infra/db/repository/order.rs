use crate::order::domain::model::order::{Order, OrderDetail, OrderId};
use crate::order::domain::model::ticket_price::CustomerType;
use crate::order::domain::repository::order::OrderRepository;
use crate::order::infra::db::establish_connection;
use crate::order::infra::db::model::{OrderDTO, OrderDetailDTO};
use crate::order::infra::db::schema::order_details::dsl::{order_details, order_id};
use crate::order::infra::db::schema::orders::dsl::orders;
use chrono::{Local, TimeZone};
use diesel::prelude::*;

#[derive(Debug)]
pub struct DbOrderRepository {}

impl OrderRepository for DbOrderRepository {
    fn find(&self, key: OrderId) -> Result<Order, ()> {
        let key: u32 = key.into();
        let conn = establish_connection();

        let details_dto = order_details
            .filter(order_id.eq(key as i32))
            .load::<OrderDetailDTO>(&conn)
            .expect("Error loading order_details");
        let order_dto: OrderDTO = orders
            .find(key as i32)
            .first(&conn)
            .expect("Error loading order_details");

        let details: Vec<_> = details_dto
            .iter()
            .map(|x| {
                OrderDetail::new(
                    CustomerType::new(x.customer_type),
                    x.ticket_count.into(),
                    x.ticket_price.into(),
                )
            })
            .collect();

        Result::Ok(Order::new(
            order_dto.id as u32,
            order_dto.movie_id as u32,
            Local
                .from_local_datetime(&order_dto.start_at.and_hms(0, 0, 0))
                .unwrap(),
            details,
        ))
    }

    fn save(&self, _order: Order) -> Result<(), ()> {
        Result::Ok(())
    }
}
