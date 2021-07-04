use crate::order::domain::model::order::MovieId;
use crate::order::domain::model::ticket_price::{
    CustomerType, CustomerTypeDiscount, TicketCount, TicketPrice,
};

#[derive(Debug)]
pub struct TicketPriceService<B, C>
where
    B: BasicPriceService,
    C: CustomerTypeDiscountService,
{
    basic_price_service: B,
    customer_type_discount_service: C,
}

impl<B, C> TicketPriceService<B, C>
where
    B: BasicPriceService,
    C: CustomerTypeDiscountService,
{
    pub fn calculate(
        &self,
        id: MovieId,
        customer_type: CustomerType,
        count: TicketCount,
    ) -> TicketPrice {
        let basic = self.basic_price_service.basic_price(id);
        let discount = self.customer_type_discount_service.get(customer_type);
        (basic - discount.discount_price(basic)) * count
    }
}

pub trait BasicPriceService {
    fn basic_price(&self, id: MovieId) -> TicketPrice;
}

pub trait CustomerTypeDiscountService {
    fn get(&self, customer_type: CustomerType) -> CustomerTypeDiscount;
}

#[cfg(test)]
mod tests {
    use crate::order::domain::service::ticket_price::*;

    #[test]
    fn test_ticket_price_calculate() {
        let service = TicketPriceService {
            basic_price_service: MockBasicPriceService {},
            customer_type_discount_service: MockCustomerTypeDiscountService {},
        };
        assert_eq!(
            service.calculate(1.into(), CustomerType::Silver, 2.into()),
            160.into()
        );
    }

    #[derive(Debug)]
    pub struct MockBasicPriceService {}

    impl BasicPriceService for MockBasicPriceService {
        fn basic_price(&self, _id: MovieId) -> TicketPrice {
            100.into()
        }
    }
    #[derive(Debug)]
    pub struct MockCustomerTypeDiscountService {}

    impl CustomerTypeDiscountService for MockCustomerTypeDiscountService {
        fn get(&self, _type: CustomerType) -> CustomerTypeDiscount {
            CustomerTypeDiscount::new(CustomerType::Silver, 20)
        }
    }
}
