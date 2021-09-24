use crate::order::movie_ticket::movie_ticket_api_server::MovieTicketApi;
use chrono::Local;
use domain::model::ticket_price::{CustomerType, TicketCount};
use registry::usecase_registry::order::{HubOrderRegistrationUsecase, HubOrderShowUsecase};
use std::collections::HashMap;
use tonic::Response;
use usecase::order::{OrderRegistrationUsecase, OrderShowUsecase};

pub mod movie_ticket {
    tonic::include_proto!("movie_ticket");
}

pub struct MovieTicketApiController {
    order_registration_usecase: &'static HubOrderRegistrationUsecase<'static>,
    order_show_usecase: &'static HubOrderShowUsecase<'static>,
}

impl MovieTicketApiController {
    pub fn new(
        order_registration_usecase: &'static HubOrderRegistrationUsecase<'static>,
        order_show_usecase: &'static HubOrderShowUsecase<'static>,
    ) -> Self {
        MovieTicketApiController {
            order_registration_usecase,
            order_show_usecase,
        }
    }
}

#[tonic::async_trait]
impl MovieTicketApi for MovieTicketApiController {
    async fn order(
        &self,
        request: tonic::Request<movie_ticket::OrderRequest>,
    ) -> Result<tonic::Response<movie_ticket::OrderResponse>, tonic::Status> {
        let request = request.into_inner();

        let ticket_types: HashMap<_, _> = vec![
            (CustomerType::Adult, TicketCount::from(request.count_adult)),
            (CustomerType::Child, TicketCount::from(request.count_child)),
            (
                CustomerType::Silver,
                TicketCount::from(request.count_silver),
            ),
        ]
        .into_iter()
        .collect();
        let result = self.order_registration_usecase.action(
            request.order_id,
            request.movie_id,
            Local::now(),
            ticket_types,
        );

        let response = movie_ticket::OrderResponse {
            message: format!("result {:?}", result),
        };
        Ok(Response::new(response))
    }
    async fn show(
        &self,
        request: tonic::Request<movie_ticket::ShowRequest>,
    ) -> Result<tonic::Response<movie_ticket::ShowResponse>, tonic::Status> {
        println!("Got a request: {:?}", request);

        let result = self
            .order_show_usecase
            .action(request.into_inner().order_id);

        let response = movie_ticket::ShowResponse {
            message: format!("result {:?}", result),
        };
        Ok(Response::new(response))
    }
}
