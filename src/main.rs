use crate::order::controller::order::movie_ticket::movie_ticket_api_server::MovieTicketApiServer;
use crate::order::controller::order::MovieTicketApiController;
use crate::order::registry::repository::DbRepositoryRegistry;
use crate::order::registry::service::{DbServiceRegistry, DomainServiceRegistry};
use crate::order::registry::usecase::UsecaseRegistry;
use tonic::transport::Server;

#[macro_use]
extern crate diesel;
extern crate dotenv;

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
    let movie_ticket = MovieTicketApiController::new(
        USECASE_REGISTRY.order_registration(),
        USECASE_REGISTRY.order_show(),
    );

    Server::builder()
        .add_service(MovieTicketApiServer::new(movie_ticket))
        .serve(addr)
        .await?;

    Ok(())
}

mod order;
