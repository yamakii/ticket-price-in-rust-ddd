use crate::order::registry::repository::DbRepositoryRegistry;
use crate::order::registry::service::DomainServiceRegistry;
use crate::order::registry::usecase::order::HubOrderRegistrationUsecase;

pub mod order;

pub struct UsecaseRegistry<'a> {
    order_registration: HubOrderRegistrationUsecase<'a>,
}

impl<'a> UsecaseRegistry<'a> {
    pub fn new(repository: &'a DbRepositoryRegistry, service: &'a DomainServiceRegistry) -> Self {
        UsecaseRegistry {
            order_registration: HubOrderRegistrationUsecase::new(repository, service),
        }
    }
    pub fn order_registration(&self) -> &HubOrderRegistrationUsecase {
        &self.order_registration
    }
}
