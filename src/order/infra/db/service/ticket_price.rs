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
    fn get(&self, customer_type: CustomerType) -> CustomerTypeDiscount {
        let discount = match customer_type {
            CustomerType::Silver => 20,
            CustomerType::Child => 50,
            _ => 0,
        };
        CustomerTypeDiscount::new(customer_type, discount)
    }
}
