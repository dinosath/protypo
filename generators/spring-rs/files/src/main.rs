use spring::{auto_config, App};
use spring_sea_orm::{SeaOrmPlugin,};
use spring_web::{WebConfigurator, WebPlugin};
mod models;
mod controllers;

#[auto_config(WebConfigurator)]
#[tokio::main]
async fn main() {
    App::new()
        .add_plugin(SeaOrmPlugin)
        .add_plugin(WebPlugin)
        .run()
        .await
}