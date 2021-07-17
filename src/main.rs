use crate::order::registry::usecase::order::HubOrderRegistrationUsecase;
use crate::order::usecase::order::OrderRegistrationUsecase;

mod order;

fn main() {
    let result = HubOrderRegistrationUsecase::new().action();
    println!("result {:?}", result);
}
