use crate::repository_registry::DbRepositoryRegistry;
use crate::service_registry::DomainServiceRegistry;
use crate::usecase_registry::order::{HubOrderRegistrationUsecase, HubOrderShowUsecase};

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
    pub fn order_show(&self) -> &HubOrderShowUsecase {
        &self.order_show
    }
}
