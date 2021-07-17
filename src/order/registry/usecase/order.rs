use crate::order::domain::repository::order::HaveOrderRepository;
use crate::order::domain::service::ticket_price::HaveTicketPriceService;
use crate::order::infra::db::repository::order::DbOrderRepository;
use crate::order::registry::service::ticket_price::HubTicketPriceService;
use crate::order::usecase::order::IsOrderRegistrationUsecase;

pub struct HubOrderRegistrationUsecase {
    repo: DbOrderRepository,
    service: HubTicketPriceService,
}

impl HubOrderRegistrationUsecase {
    pub fn new() -> Self {
        HubOrderRegistrationUsecase {
            repo: DbOrderRepository {},
            service: HubTicketPriceService::new(),
        }
    }
}

impl IsOrderRegistrationUsecase for HubOrderRegistrationUsecase {}

impl HaveOrderRepository for HubOrderRegistrationUsecase {
    type OrderRepository = DbOrderRepository;

    fn order_repository(&self) -> &Self::OrderRepository {
        &self.repo
    }
}

impl HaveTicketPriceService for HubOrderRegistrationUsecase {
    type TicketPriceService = HubTicketPriceService;

    fn ticket_price_service(&self) -> &Self::TicketPriceService {
        &self.service
    }
}
