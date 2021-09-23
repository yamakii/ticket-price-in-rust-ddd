use crate::domain::model::ticket_price::{CustomerType, TicketCount, TicketPrice};
use crate::domain::service::ticket_price::TicketPriceService;
use chrono::{DateTime, Local};
use std::collections::HashMap;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct OrderId(u32);

impl From<u32> for OrderId {
    fn from(u: u32) -> Self {
        Self(u)
    }
}

impl From<OrderId> for u32 {
    fn from(order_id: OrderId) -> Self {
        order_id.0
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
    pub fn id(&self) -> OrderId {
        self.id
    }
    pub fn movie_id(&self) -> MovieId {
        self.movie_id
    }
    pub fn start_at(&self) -> DateTime<Local> {
        self.start_at
    }
    pub fn details(&self) -> &Vec<OrderDetail> {
        &self.details
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
    pub fn new(customer_type: CustomerType, count: TicketCount, price: TicketPrice) -> Self {
        Self {
            customer_type,
            count,
            price,
        }
    }
    pub fn customer_type(&self) -> CustomerType {
        self.customer_type
    }
    pub fn count(&self) -> TicketCount {
        self.count
    }
    pub fn price(&self) -> TicketPrice {
        self.price
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct MovieId(u32);

impl From<u32> for MovieId {
    fn from(u: u32) -> Self {
        Self(u)
    }
}

impl From<MovieId> for u32 {
    fn from(movie_id: MovieId) -> Self {
        movie_id.0
    }
}
