use crate::order::domain::model::order::MovieId;
use crate::order::domain::model::ticket_price::{CustomerType, CustomerTypeDiscount, TicketPrice};
use crate::order::domain::service::ticket_price::{BasicPriceService, CustomerTypeDiscountService};
use crate::order::infra::db::establish_connection;
use crate::order::infra::db::model::{CustomerTypeDiscountDTO, MovieDTO};
use crate::order::infra::db::schema::customer_type_discounts::dsl::customer_type_discounts;
use crate::order::infra::db::schema::movies::dsl::movies;
use diesel::prelude::*;

#[derive(Debug)]
pub struct DbBasicPriceService {}
impl BasicPriceService for DbBasicPriceService {
    fn basic_price(&self, key: MovieId) -> TicketPrice {
        let key: u32 = key.into();
        let conn = establish_connection();

        let movie_dto: MovieDTO = movies
            .find(key as i32)
            .first(&conn)
            .expect("Error loading movies");

        movie_dto.basic_price.into()
    }
}

#[derive(Debug)]
pub struct DbCustomerTypeDiscountService {}
impl CustomerTypeDiscountService for DbCustomerTypeDiscountService {
    fn get(&self, customer_type: CustomerType) -> CustomerTypeDiscount {
        let key = customer_type as i32;
        let conn = establish_connection();

        let customer_type_discount_dto: CustomerTypeDiscountDTO = customer_type_discounts
            .find(key as i32)
            .first(&conn)
            .expect("Error loading customer_type_discounts");

        CustomerTypeDiscount::new(customer_type, customer_type_discount_dto.discount as u32)
    }
}
