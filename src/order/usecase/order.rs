use crate::order::domain::model::order::Order;
use crate::order::domain::model::ticket_price::{CustomerType, TicketCount};
use crate::order::domain::repository::order::{HaveOrderRepository, OrderRepository};
use crate::order::domain::service::ticket_price::HaveTicketPriceService;
use chrono::{DateTime, Local};
use std::collections::HashMap;

pub trait OrderRegistrationUsecase {
    fn action(
        &self,
        id: u32,
        movie_id: u32,
        start_at: DateTime<Local>,
        customer_types: HashMap<CustomerType, TicketCount>,
    ) -> Result<(), ()>;
}

pub trait IsOrderRegistrationUsecase: HaveOrderRepository + HaveTicketPriceService {}

impl<O: IsOrderRegistrationUsecase> OrderRegistrationUsecase for O {
    fn action(
        &self,
        id: u32,
        movie_id: u32,
        start_at: DateTime<Local>,
        customer_types: HashMap<CustomerType, TicketCount>,
    ) -> Result<(), ()> {
        let order = Order::create(
            id,
            movie_id,
            start_at,
            customer_types,
            self.ticket_price_service(),
        );
        println!("{:?}", order);
        println!("{:?}", order.price());
        self.order_repository().save(order)
    }
}
