use std::collections::HashMap;

use chrono::{DateTime, Local};

use crate::order::domain::model::ticket_price::{CustomerType, TicketCount, TicketPrice};
use crate::order::domain::service::ticket_price::TicketPriceService;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct OrderId(u32);

impl From<u32> for OrderId {
    fn from(u: u32) -> Self {
        Self(u)
    }
}
impl Into<u32> for OrderId {
    fn into(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Debug)]
pub struct Order {
    id: OrderId,
    movie_id: MovieId,
    start_at: DateTime<Local>,
    details: Vec<OrderDetail>,
}

impl PartialEq for Order {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Order {
    pub fn new(
        id: u32,
        movie_id: u32,
        start_at: DateTime<Local>,
        details: Vec<OrderDetail>,
    ) -> Order {
        Order {
            id: id.into(),
            movie_id: movie_id.into(),
            start_at,
            details,
        }
    }
    pub fn create<T: TicketPriceService>(
        id: u32,
        movie_id: u32,
        start_at: DateTime<Local>,
        customer_types: HashMap<CustomerType, TicketCount>,
        service: &T,
    ) -> Order {
        let details = customer_types
            .iter()
            .map(|(key, value)| OrderDetail {
                customer_type: *key,
                count: *value,
                price: service.calculate(movie_id.into(), *key, *value),
            })
            .collect();
        Order {
            id: OrderId(id),
            movie_id: movie_id.into(),
            start_at,
            details,
        }
    }
    pub fn price(&self) -> TicketPrice {
        self.details.iter().fold(0.into(), |acc, x| acc + x.price)
    }
}

#[derive(Clone, Debug)]
pub struct OrderDetail {
    customer_type: CustomerType,
    count: TicketCount,
    price: TicketPrice,
}

impl OrderDetail {
    pub(crate) fn new(customer_type: CustomerType, count: TicketCount, price: TicketPrice) -> Self {
        Self {
            customer_type,
            count,
            price,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct MovieId(u32);

impl From<u32> for MovieId {
    fn from(u: u32) -> Self {
        Self(u)
    }
}
