use crate::order::domain::service::ticket_price::{
    HaveBasicPriceService, HaveCustomerTypeDiscountService, HaveTicketPriceService,
    IsTicketPriceService,
};
use crate::order::infra::db::service::ticket_price::{
    DbBasicPriceService, DbCustomerTypeDiscountService,
};

#[derive(Debug)]
pub struct HubTicketPriceService {
    basic: DbBasicPriceService,
    customer_type_discount: DbCustomerTypeDiscountService,
}

impl HubTicketPriceService {
    pub fn new() -> Self {
        HubTicketPriceService {
            basic: DbBasicPriceService {},
            customer_type_discount: DbCustomerTypeDiscountService {},
        }
    }
}

impl IsTicketPriceService for HubTicketPriceService {}

impl HaveBasicPriceService for HubTicketPriceService {
    type BasicPriceService = DbBasicPriceService;

    fn basic_price_service(&self) -> &Self::BasicPriceService {
        &self.basic
    }
}

impl HaveCustomerTypeDiscountService for HubTicketPriceService {
    type CustomerTypeDiscountService = DbCustomerTypeDiscountService;

    fn customer_type_discount_service(&self) -> &Self::CustomerTypeDiscountService {
        &self.customer_type_discount
    }
}

impl HaveTicketPriceService for HubTicketPriceService {
    type TicketPriceService = Self;

    fn ticket_price_service(&self) -> &Self::TicketPriceService {
        &self
    }
}
