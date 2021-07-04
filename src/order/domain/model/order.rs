#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct OrderId(u32);
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct MovieId(u32);
impl From<u32> for MovieId {
    fn from(u: u32) -> Self {
        Self(u)
    }
}

#[derive(Clone, Debug)]
pub struct Order {
    id: OrderId,
}

impl Order {
    pub fn new(id: u32) -> Order {
        Order { id: OrderId(id) }
    }
}

impl PartialEq for Order {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
