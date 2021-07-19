use crate::order::domain::model::ticket_price::{CustomerType, TicketCount};
use crate::order::registry::repository::DbRepositoryRegistry;
use crate::order::registry::service::{DbServiceRegistry, DomainServiceRegistry};
use crate::order::registry::usecase::UsecaseRegistry;
use crate::order::usecase::order::OrderRegistrationUsecase;
use chrono::Local;
use std::collections::HashMap;

mod order;

fn main() {
    // registryの初期化
    let db_service_registry = DbServiceRegistry::new();
    let db_repository_registry = DbRepositoryRegistry::new();
    let domain_service_registry = DomainServiceRegistry::new(&db_service_registry);
    let usecase_registry = UsecaseRegistry::new(&db_repository_registry, &domain_service_registry);

    // ユースケース
    // 映画チケット購入
    let ticket_types: HashMap<_, _> = vec![
        (CustomerType::Adult, TicketCount::from(2)),
        (CustomerType::Child, TicketCount::from(3)),
        (CustomerType::Silver, TicketCount::from(1)),
    ]
    .into_iter()
    .collect();
    let result = usecase_registry
        .order_registration()
        .action(1, 1, Local::now(), ticket_types);
    println!("result {:?}", result);
}
