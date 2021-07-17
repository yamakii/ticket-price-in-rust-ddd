use crate::order::domain::model::order::MovieId;
use crate::order::domain::model::ticket_price::{CustomerType, CustomerTypeDiscount, TicketPrice};
use crate::order::domain::service::ticket_price::{BasicPriceService, CustomerTypeDiscountService};

#[derive(Debug)]
pub struct DbBasicPriceService {}
impl BasicPriceService for DbBasicPriceService {
    fn basic_price(&self, _id: MovieId) -> TicketPrice {
        100.into()
    }
}

#[derive(Debug)]
pub struct DbCustomerTypeDiscountService {}
impl CustomerTypeDiscountService for DbCustomerTypeDiscountService {
    fn get(&self, _type: CustomerType) -> CustomerTypeDiscount {
        CustomerTypeDiscount::new(CustomerType::Silver, 20)
    }
}
