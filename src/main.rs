use crate::order::registry;

mod order;

fn main() {
    let result = registry::order_registration_usecase().action();
    println!("result {:?}", result);
}
