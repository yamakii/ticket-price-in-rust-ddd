use crate::order::domain::model::order::MovieId;
use crate::order::domain::model::ticket_price::{CustomerType, CustomerTypeDiscount, TicketPrice};
use crate::order::domain::service::ticket_price::{BasicPriceService, CustomerTypeDiscountService};
use crate::order::infra::db::model::{CustomerTypeDiscountDTO, MovieDTO};
use crate::order::infra::db::schema::customer_type_discounts::dsl::customer_type_discounts;
use crate::order::infra::db::schema::movies::dsl::movies;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use std::sync::Arc;

pub struct DbBasicPriceService {
    pool: Arc<Pool<ConnectionManager<PgConnection>>>,
}

impl DbBasicPriceService {
    pub fn new(pool: Arc<Pool<ConnectionManager<PgConnection>>>) -> Self {
        DbBasicPriceService { pool }
    }
}

impl BasicPriceService for DbBasicPriceService {
    fn basic_price(&self, key: MovieId) -> TicketPrice {
        let key: u32 = key.into();
        let conn = self.pool.get().unwrap();

        let movie_dto: MovieDTO = movies
            .find(key as i32)
            .first(&conn)
            .expect("Error loading movies");

        movie_dto.basic_price.into()
    }
}

pub struct DbCustomerTypeDiscountService {
    pool: Arc<Pool<ConnectionManager<PgConnection>>>,
}

impl DbCustomerTypeDiscountService {
    pub fn new(pool: Arc<Pool<ConnectionManager<PgConnection>>>) -> Self {
        DbCustomerTypeDiscountService { pool }
    }
}

impl CustomerTypeDiscountService for DbCustomerTypeDiscountService {
    fn get(&self, customer_type: CustomerType) -> CustomerTypeDiscount {
        let key = customer_type as i32;
        let conn = self.pool.get().unwrap();

        let customer_type_discount_dto: CustomerTypeDiscountDTO = customer_type_discounts
            .find(key as i32)
            .first(&conn)
            .expect("Error loading customer_type_discounts");

        CustomerTypeDiscount::new(customer_type, customer_type_discount_dto.discount as u32)
    }
}
