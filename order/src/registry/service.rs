use crate::infra::db::service::ticket_price::{DbBasicPriceService, DbCustomerTypeDiscountService};
use crate::registry::service::ticket_price::HubTicketPriceService;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use std::sync::Arc;

pub mod ticket_price;

pub struct DomainServiceRegistry<'a> {
    ticket_price: HubTicketPriceService<'a>,
}

impl<'a> DomainServiceRegistry<'a> {
    pub fn new(registry: &'a DbServiceRegistry) -> Self {
        DomainServiceRegistry {
            ticket_price: HubTicketPriceService::new(registry),
        }
    }
    pub fn ticket_price(&self) -> &HubTicketPriceService {
        &self.ticket_price
    }
}

pub struct DbServiceRegistry {
    basic_price: DbBasicPriceService,
    customer_type_discount: DbCustomerTypeDiscountService,
}

impl DbServiceRegistry {
    pub fn new(pool: Arc<Pool<ConnectionManager<PgConnection>>>) -> Self {
        DbServiceRegistry {
            basic_price: DbBasicPriceService::new(pool.clone()),
            customer_type_discount: DbCustomerTypeDiscountService::new(pool),
        }
    }
    pub fn basic_price(&self) -> &DbBasicPriceService {
        &self.basic_price
    }
    pub fn customer_type_discount(&self) -> &DbCustomerTypeDiscountService {
        &self.customer_type_discount
    }
}
