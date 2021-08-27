use crate::order::domain::model::order::{Order, OrderDetail, OrderId};
use crate::order::domain::model::ticket_price::CustomerType;
use crate::order::domain::repository::order::OrderRepository;
use crate::order::infra::db::establish_connection;
use crate::order::infra::db::model::{OrderDTO, OrderDetailDTO};
use crate::order::infra::db::repository::order::dto::{NewOrder, NewOrderDetail};
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

    fn save(&self, order: Order) -> Result<(), ()> {
        let conn = establish_connection();
        let new_order = NewOrder {
            id: Into::<u32>::into(order.id()) as i32,
            movie_id: Into::<u32>::into(order.movie_id()) as i32,
            start_at: order.start_at().naive_local().date(),
        };
        let new_order_details: Vec<_> = order
            .details()
            .iter()
            .enumerate()
            .map(|(i, x)| NewOrderDetail {
                order_id: Into::<u32>::into(order.id()) as i32,
                no: (i + 1) as i32,
                customer_type: Into::<i32>::into(x.customer_type() as i32),
                ticket_count: Into::<i32>::into(x.count()),
                ticket_price: Into::<i32>::into(x.price()),
            })
            .collect();

        diesel::insert_into(orders)
            .values(&new_order)
            .execute(&conn)
            .expect("error");
        diesel::insert_into(order_details)
            .values(&new_order_details)
            .execute(&conn)
            .expect("error");

        Result::Ok(())
    }
}

mod dto {
    use crate::order::infra::db::schema::order_details;
    use crate::order::infra::db::schema::orders;
    use chrono::NaiveDate;

    #[derive(Insertable)]
    #[table_name = "orders"]
    pub struct NewOrder {
        pub id: i32,
        pub movie_id: i32,
        pub start_at: NaiveDate,
    }

    #[derive(Insertable)]
    #[table_name = "order_details"]
    pub struct NewOrderDetail {
        pub order_id: i32,
        pub no: i32,
        pub customer_type: i32,
        pub ticket_count: i32,
        pub ticket_price: i32,
    }
}
