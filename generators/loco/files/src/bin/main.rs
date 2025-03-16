use dotenvy::dotenv;
use loco_rs::cli;
use myapp::app::App;
use migration::Migrator;

#[tokio::main]
async fn main() -> loco_rs::Result<()> {
    dotenv().ok();
    cli::main::<App, Migrator>().await
}