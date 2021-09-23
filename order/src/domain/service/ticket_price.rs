use crate::domain::model::order::MovieId;
use crate::domain::model::ticket_price::{
    CustomerType, CustomerTypeDiscount, TicketCount, TicketPrice,
};

pub trait TicketPriceService {
    fn calculate(
        &self,
        id: MovieId,
        customer_type: CustomerType,
        count: TicketCount,
    ) -> TicketPrice;
}

pub trait IsTicketPriceService: HaveBasicPriceService + HaveCustomerTypeDiscountService {}

impl<T: IsTicketPriceService> TicketPriceService for T {
    fn calculate(
        &self,
        id: MovieId,
        customer_type: CustomerType,
        count: TicketCount,
    ) -> TicketPrice {
        let basic = self.basic_price_service().basic_price(id);
        let discount = self.customer_type_discount_service().get(customer_type);
        (basic - discount.discount_price(basic)) * count
    }
}

pub trait HaveTicketPriceService {
    type TicketPriceService: TicketPriceService;
    fn ticket_price_service(&self) -> &Self::TicketPriceService;
}

pub trait BasicPriceService {
    fn basic_price(&self, id: MovieId) -> TicketPrice;
}

pub trait HaveBasicPriceService {
    type BasicPriceService: BasicPriceService;
    fn basic_price_service(&self) -> &Self::BasicPriceService;
}

pub trait CustomerTypeDiscountService {
    fn get(&self, customer_type: CustomerType) -> CustomerTypeDiscount;
}

pub trait HaveCustomerTypeDiscountService {
    type CustomerTypeDiscountService: CustomerTypeDiscountService;
    fn customer_type_discount_service(&self) -> &Self::CustomerTypeDiscountService;
}

#[cfg(test)]
mod tests {
    use crate::domain::service::ticket_price::*;

    #[test]
    fn test_ticket_price_calculate() {
        let service = MockTicketPriceService {}.ticket_price_service();
        assert_eq!(
            service.calculate(1.into(), CustomerType::Silver, 2.into()),
            160.into()
        );
    }

    pub trait MockBasicPriceService {}
    impl<B: MockBasicPriceService> BasicPriceService for B {
        fn basic_price(&self, _id: MovieId) -> TicketPrice {
            100.into()
        }
    }
    pub trait MockCustomerTypeDiscountService {}
    impl<C: MockCustomerTypeDiscountService> CustomerTypeDiscountService for C {
        fn get(&self, _type: CustomerType) -> CustomerTypeDiscount {
            CustomerTypeDiscount::new(CustomerType::Silver, 20)
        }
    }

    #[derive(Debug)]
    pub struct MockTicketPriceService {}
    impl MockBasicPriceService for MockTicketPriceService {}
    impl MockCustomerTypeDiscountService for MockTicketPriceService {}
    impl IsTicketPriceService for MockTicketPriceService {}
    impl HaveBasicPriceService for MockTicketPriceService {
        type BasicPriceService = Self;

        fn basic_price_service(&self) -> &Self::BasicPriceService {
            &self
        }
    }
    impl HaveCustomerTypeDiscountService for MockTicketPriceService {
        type CustomerTypeDiscountService = Self;

        fn customer_type_discount_service(&self) -> &Self::CustomerTypeDiscountService {
            &self
        }
    }
    impl HaveTicketPriceService for MockTicketPriceService {
        type TicketPriceService = Self;

        fn ticket_price_service(&self) -> &Self::TicketPriceService {
            &self
        }
    }
}
