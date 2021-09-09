use crate::order::domain::repository::order::HaveOrderRepository;
use crate::order::domain::service::ticket_price::HaveTicketPriceService;
use crate::order::infra::db::repository::order::DbOrderRepository;
use crate::order::registry::repository::DbRepositoryRegistry;
use crate::order::registry::service::ticket_price::HubTicketPriceService;
use crate::order::registry::service::DomainServiceRegistry;
use crate::order::usecase::order::{IsOrderRegistrationUsecase, IsOrderShowUsecase};

pub struct HubOrderRegistrationUsecase<'a> {
    repo: &'a DbOrderRepository,
    service: &'a HubTicketPriceService<'a>,
}

impl<'a> HubOrderRegistrationUsecase<'a> {
    pub fn new(repository: &'a DbRepositoryRegistry, service: &'a DomainServiceRegistry) -> Self {
        HubOrderRegistrationUsecase {
            repo: repository.order(),
            service: service.ticket_price(),
        }
    }
}

impl<'a> IsOrderRegistrationUsecase for HubOrderRegistrationUsecase<'a> {}

impl<'a> HaveOrderRepository for HubOrderRegistrationUsecase<'a> {
    type OrderRepository = DbOrderRepository;

    fn order_repository(&self) -> &Self::OrderRepository {
        self.repo
    }
}

impl<'a> HaveTicketPriceService for HubOrderRegistrationUsecase<'a> {
    type TicketPriceService = HubTicketPriceService<'a>;

    fn ticket_price_service(&self) -> &Self::TicketPriceService {
        self.service
    }
}

pub struct HubOrderShowUsecase<'a> {
    repo: &'a DbOrderRepository,
}

impl<'a> HubOrderShowUsecase<'a> {
    pub fn new(repository: &'a DbRepositoryRegistry) -> Self {
        HubOrderShowUsecase {
            repo: repository.order(),
        }
    }
}

impl<'a> IsOrderShowUsecase for HubOrderShowUsecase<'a> {}

impl<'a> HaveOrderRepository for HubOrderShowUsecase<'a> {
    type OrderRepository = DbOrderRepository;

    fn order_repository(&self) -> &Self::OrderRepository {
        self.repo
    }
}
