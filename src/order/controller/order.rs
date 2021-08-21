use crate::order::controller::order::movie_ticket::movie_ticket_api_server::MovieTicketApi;
use crate::order::registry::usecase::order::HubOrderShowUsecase;
use crate::order::usecase::order::OrderShowUsecase;
use tonic::Response;

pub mod movie_ticket {
    tonic::include_proto!("movie_ticket");
}

#[derive(Debug)]
pub struct MyMovieTicketApi {
    order_show_usecase: &'static HubOrderShowUsecase<'static>,
}

impl MyMovieTicketApi {
    pub fn new(order_show_usecase: &'static HubOrderShowUsecase<'static>) -> Self {
        MyMovieTicketApi { order_show_usecase }
    }
}

#[tonic::async_trait]
impl MovieTicketApi for MyMovieTicketApi {
    async fn show(
        &self,
        request: tonic::Request<movie_ticket::ShowRequest>,
    ) -> Result<tonic::Response<movie_ticket::ShowResponse>, tonic::Status> {
        println!("Got a request: {:?}", request);

        let result = self
            .order_show_usecase
            .action(request.into_inner().order_id);
        let response = movie_ticket::ShowResponse {
            message: format!("result {:?}", result).into(),
        };

        Ok(Response::new(response))
    }
}
