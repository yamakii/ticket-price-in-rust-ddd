use crate::order::registry::repository::DbRepositoryRegistry;
use crate::order::registry::service::DomainServiceRegistry;
use crate::order::registry::usecase::order::{HubOrderRegistrationUsecase, HubOrderShowUsecase};

pub mod order;

pub struct UsecaseRegistry<'a> {
    order_registration: HubOrderRegistrationUsecase<'a>,
    order_show: HubOrderShowUsecase<'a>,
}

impl<'a> UsecaseRegistry<'a> {
    pub fn new(repository: &'a DbRepositoryRegistry, service: &'a DomainServiceRegistry) -> Self {
        UsecaseRegistry {
            order_registration: HubOrderRegistrationUsecase::new(repository, service),
            order_show: HubOrderShowUsecase::new(repository),
        }
    }
    pub fn order_registration(&self) -> &HubOrderRegistrationUsecase {
        &self.order_registration
    }
    pub(crate) fn order_show(&self) -> &HubOrderShowUsecase {
        &self.order_show
    }
}
