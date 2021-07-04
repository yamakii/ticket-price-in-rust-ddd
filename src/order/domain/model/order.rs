#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct OrderId(i32);

#[derive(Clone, Debug)]
pub struct Order {
    id: OrderId,
}

impl Order {
    pub(crate) fn new(id: i32) -> Order {
        Order { id: OrderId(id) }
    }
}

impl PartialEq for Order {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}