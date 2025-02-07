use spring::{auto_config, App};
use spring_sea_orm::{SeaOrmPlugin,};
use spring_web::{WebConfigurator, WebPlugin};
use spring_opentelemetry::{KeyValue, OpenTelemetryPlugin, ResourceConfigurator, SERVICE_NAME, SERVICE_VERSION, };

mod models;
mod controllers;

#[actix_web::main]
async fn main() {
    App::new()
        .opentelemetry_attrs([
            KeyValue::new(SERVICE_NAME, env!("CARGO_PKG_NAME")),
            KeyValue::new(SERVICE_VERSION, env!("CARGO_PKG_VERSION")),
        ])
        .add_plugin(SeaOrmPlugin)
        .add_plugin(WebPlugin)
        .add_plugin(OpenTelemetryPlugin)
        .run()
        .await
}