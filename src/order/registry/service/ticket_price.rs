use crate::order::domain::service::ticket_price::{
    HaveBasicPriceService, HaveCustomerTypeDiscountService, HaveTicketPriceService,
    IsTicketPriceService,
};
use crate::order::infra::db::service::ticket_price::{
    DbBasicPriceService, DbCustomerTypeDiscountService,
};
use crate::order::registry::service::DbServiceRegistry;

pub struct HubTicketPriceService<'a> {
    basic: &'a DbBasicPriceService,
    customer_type_discount: &'a DbCustomerTypeDiscountService,
}

impl<'a> HubTicketPriceService<'a> {
    pub fn new(registry: &'a DbServiceRegistry) -> Self {
        HubTicketPriceService {
            basic: registry.basic_price(),
            customer_type_discount: registry.customer_type_discount(),
        }
    }
}

impl<'a> IsTicketPriceService for HubTicketPriceService<'a> {}

impl<'a> HaveBasicPriceService for HubTicketPriceService<'a> {
    type BasicPriceService = DbBasicPriceService;

    fn basic_price_service(&self) -> &Self::BasicPriceService {
        &self.basic
    }
}

impl<'a> HaveCustomerTypeDiscountService for HubTicketPriceService<'a> {
    type CustomerTypeDiscountService = DbCustomerTypeDiscountService;

    fn customer_type_discount_service(&self) -> &Self::CustomerTypeDiscountService {
        &self.customer_type_discount
    }
}

impl<'a> HaveTicketPriceService for HubTicketPriceService<'a> {
    type TicketPriceService = Self;

    fn ticket_price_service(&self) -> &Self::TicketPriceService {
        &self
    }
}
