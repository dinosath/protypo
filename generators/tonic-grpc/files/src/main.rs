mod services;
use std::sync::LazyLock;
use tonic::transport::Server;
use tonic_health::server::HealthReporter;
use prost_reflect::DescriptorPool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reflection_service = tonic_reflection::server::Builder::configure()
        .include_reflection_service(true)
        .build_v1()
        .unwrap();

    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();

    let addr = "[::1]:50051".parse().unwrap();
    println!("Server listening on {}", addr);
    Server::builder()
        .add_service(reflection_service)
        .add_service(health_service)
        .serve(addr)
        .await?;

    Ok(())
}
