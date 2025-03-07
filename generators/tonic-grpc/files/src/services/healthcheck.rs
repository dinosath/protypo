use tonic::{codegen::tokio_stream::wrappers::ReceiverStream, Request, Response, Status};
use healthcheck::health_check_server::HealthCheck;
use healthcheck::{HealthCheckResponse};
pub mod healthcheck {
    tonic::include_proto!("healthcheck");
}

#[derive(Debug, Default)]
pub struct MyHealthCheck {}

#[tonic::async_trait]
impl HealthCheck for MyHealthCheck {
    async fn check(&self, request: Request<()>, ) -> Result<Response<HealthCheckResponse>, Status> {
        println!("Got a request: {:?}", request);

        let response = HealthCheckResponse {
            status: 0,
        };

        Ok(Response::new(response))
    }

    type WatchStream = ReceiverStream<Result<HealthCheckResponse, Status>>;

    async fn watch(&self, request: Request<()>) -> Result<Response<Self::WatchStream>, Status> {
        todo!()
    }
}