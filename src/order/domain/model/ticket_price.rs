use std::ops::{Mul, Sub};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct TicketPrice(i32);
impl From<i32> for TicketPrice {
    fn from(i: i32) -> Self {
        Self(i)
    }
}
impl Sub<TicketPrice> for TicketPrice {
    type Output = Self;

    fn sub(self, rhs: TicketPrice) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}
impl Mul<f64> for TicketPrice {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self((self.0 as f64 * rhs).round() as i32)
    }
}
impl Mul<TicketCount> for TicketPrice {
    type Output = Self;

    fn mul(self, rhs: TicketCount) -> Self::Output {
        Self(self.0 * rhs.0 as i32)
    }
}

pub struct TicketCount(u32);
impl From<u32> for TicketCount {
    fn from(u: u32) -> Self {
        Self(u)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum CustomerType {
    Silver,
    Adult,
    Child,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct CustomerTypeDiscount {
    customer_type: CustomerType,
    discount: u32,
}

impl CustomerTypeDiscount {
    pub fn new(customer_type: CustomerType, discount: u32) -> Self {
        Self {
            customer_type,
            discount,
        }
    }
}

impl CustomerTypeDiscount {
    pub fn discount_price(&self, price: TicketPrice) -> TicketPrice {
        price * (self.discount as f64 / 100.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::order::domain::model::ticket_price::*;

    #[test]
    fn test_discount_price() {
        let silver = CustomerTypeDiscount::new(CustomerType::Silver, 20);
        assert_eq!(silver.discount_price(TicketPrice(97)), TicketPrice(19));
        assert_eq!(silver.discount_price(TicketPrice(98)), TicketPrice(20));
    }
}
