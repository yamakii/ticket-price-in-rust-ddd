use crate::order::controller::order::movie_ticket::movie_ticket_api_server::MovieTicketApiServer;
use crate::order::controller::order::MyMovieTicketApi;
use crate::order::domain::model::ticket_price::{CustomerType, TicketCount};
use crate::order::registry::repository::DbRepositoryRegistry;
use crate::order::registry::service::{DbServiceRegistry, DomainServiceRegistry};
use crate::order::registry::usecase::UsecaseRegistry;
use crate::order::usecase::order::OrderRegistrationUsecase;
use chrono::Local;
use std::collections::HashMap;
use tonic::transport::Server;

mod order;

#[macro_use]
extern crate lazy_static;

lazy_static! {
// registryの初期化
static ref DB_SERVICE_REGISTRY: DbServiceRegistry = DbServiceRegistry::new();
static ref DB_REPOSITORY_REGISTRY: DbRepositoryRegistry = DbRepositoryRegistry::new();
static ref DOMAIN_SERVICE_REGISTRY: DomainServiceRegistry<'static> =
    DomainServiceRegistry::new(&DB_SERVICE_REGISTRY);
static ref USECASE_REGISTRY: UsecaseRegistry<'static> =
    UsecaseRegistry::new(&DB_REPOSITORY_REGISTRY, &DOMAIN_SERVICE_REGISTRY);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let movie_ticket = MyMovieTicketApi::new(USECASE_REGISTRY.order_show());

    Server::builder()
        .add_service(MovieTicketApiServer::new(movie_ticket))
        .serve(addr)
        .await?;

    // ユースケース
    // 映画チケット購入
    let ticket_types: HashMap<_, _> = vec![
        (CustomerType::Adult, TicketCount::from(2)),
        (CustomerType::Child, TicketCount::from(3)),
        (CustomerType::Silver, TicketCount::from(1)),
    ]
    .into_iter()
    .collect();
    let result = USECASE_REGISTRY
        .order_registration()
        .action(1, 1, Local::now(), ticket_types);
    println!("result {:?}", result);

    Ok(())
}
