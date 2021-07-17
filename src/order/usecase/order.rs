use crate::order::domain::repository::order::HaveOrderRepository;
use crate::order::domain::service::ticket_price::HaveTicketPriceService;

pub trait OrderRegistrationUsecase {
    fn action(&self) -> Result<(), ()>;
}

pub trait IsOrderRegistrationUsecase: HaveOrderRepository + HaveTicketPriceService {}

impl<O: IsOrderRegistrationUsecase> OrderRegistrationUsecase for O {
    fn action(&self) -> Result<(), ()> {
        // let order = Order::new(1);
        // self.order_repository().save(order);
        Result::Ok(())
    }
}
