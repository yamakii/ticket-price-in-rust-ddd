use crate::domain::model::order::Order;
use crate::domain::model::ticket_price::{CustomerType, TicketCount};
use crate::domain::repository::order::{HaveOrderRepository, OrderRepository};
use crate::domain::service::ticket_price::HaveTicketPriceService;
use anyhow::Result;
use chrono::{DateTime, Local};
use std::collections::HashMap;

pub trait OrderRegistrationUsecase {
    fn action(
        &self,
        id: u32,
        movie_id: u32,
        start_at: DateTime<Local>,
        customer_types: HashMap<CustomerType, TicketCount>,
    ) -> Result<()>;
}

pub trait IsOrderRegistrationUsecase: HaveOrderRepository + HaveTicketPriceService {}

impl<O: IsOrderRegistrationUsecase> OrderRegistrationUsecase for O {
    fn action(
        &self,
        order_id: u32,
        movie_id: u32,
        start_at: DateTime<Local>,
        customer_types: HashMap<CustomerType, TicketCount>,
    ) -> Result<()> {
        let order = Order::create(
            order_id,
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

pub trait OrderShowUsecase {
    fn action(&self, order_id: u32) -> Result<Order>;
}

pub trait IsOrderShowUsecase: HaveOrderRepository {}

impl<O: IsOrderShowUsecase> OrderShowUsecase for O {
    fn action(&self, order_id: u32) -> Result<Order> {
        self.order_repository().find(order_id.into())
    }
}

pub fn register_order<T>(
    context: &T,
    order_id: u32,
    movie_id: u32,
    start_at: DateTime<Local>,
    customer_types: HashMap<CustomerType, TicketCount>,
) -> Result<()>
where
    T: HaveOrderRepository + HaveTicketPriceService,
{
    let order = Order::create(
        order_id,
        movie_id,
        start_at,
        customer_types,
        context.ticket_price_service(),
    );
    println!("{:?}", order);
    println!("{:?}", order.price());
    context.order_repository().save(order)
}

pub fn show_order<T>(context: &T, order_id: u32) -> Result<Order>
where
    T: HaveOrderRepository,
{
    context.order_repository().find(order_id.into())
}
