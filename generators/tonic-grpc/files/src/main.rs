use myapp::services::healthcheck::MyHealthCheck;
use myapp::services::healthcheck::healthcheck::health_check_server::HealthCheckServer;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();

    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(HealthCheckServer::new(MyHealthCheck::default()))
        .serve(addr)
        .await?;

    Ok(())
}
