use controller::order::movie_ticket::movie_ticket_api_server::MovieTicketApiServer;
use controller::order::MovieTicketApiController;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use infra::db::make_connection_pool;
use registry::repository::DbRepositoryRegistry;
use registry::service::{DbServiceRegistry, DomainServiceRegistry};
use registry::usecase::UsecaseRegistry;
use std::sync::Arc;
use tonic::transport::Server;

#[macro_use]
extern crate diesel;
extern crate dotenv;

#[macro_use]
extern crate lazy_static;

lazy_static! {
// registryの初期化
static ref POOL: Arc<Pool<ConnectionManager<PgConnection>>> = make_connection_pool();
static ref DB_SERVICE_REGISTRY: DbServiceRegistry = DbServiceRegistry::new(POOL.clone());
static ref DB_REPOSITORY_REGISTRY: DbRepositoryRegistry = DbRepositoryRegistry::new(POOL.clone());
static ref DOMAIN_SERVICE_REGISTRY: DomainServiceRegistry<'static> =
    DomainServiceRegistry::new(&DB_SERVICE_REGISTRY);
static ref USECASE_REGISTRY: UsecaseRegistry<'static> =
    UsecaseRegistry::new(&DB_REPOSITORY_REGISTRY, &DOMAIN_SERVICE_REGISTRY);
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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

pub mod controller;
pub mod domain;
pub mod infra;
pub mod registry;
pub mod usecase;
