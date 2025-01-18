use std::fmt::format;
use sea_orm::{ConnectionTrait, Statement};
use sea_orm::JsonValue::String;
use spring::{tracing::{debug,info},auto_config, App};
use spring_sea_orm::{SeaOrmPlugin,DbConn,};
use spring_opentelemetry::{KeyValue, OpenTelemetryPlugin, ResourceConfigurator, SERVICE_NAME, SERVICE_VERSION, };
use spring_web::{
    axum::{
        body,
        middleware::{self, Next},
        response::{IntoResponse, Response},
    },
    error::Result,
    extractor::{Component,Path,Request},
    Router, WebConfigurator, WebPlugin,
};
use spring_web::error::KnownWebError;

mod models;
mod controllers;

#[tokio::main]
async fn main() {
    App::new()
        .opentelemetry_attrs([
            KeyValue::new(SERVICE_NAME, env!("CARGO_PKG_NAME")),
            KeyValue::new(SERVICE_VERSION, env!("CARGO_PKG_VERSION")),
        ])
        .add_plugin(SeaOrmPlugin)
        .add_plugin(WebPlugin)
        .add_plugin(OpenTelemetryPlugin)
        .add_router(router())
        .run()
        .await
}

fn router() -> Router {
    Router::new()
        .layer(middleware::from_fn(insert_tenant_middleware))
        .merge(spring_web::handler::auto_router())
}

async fn insert_tenant_middleware(
    Component(db): Component<DbConn>,
    request: Request,
    next: Next,
) -> Response {
    info!("middleware: insert_tenant_middleware");
    info!("path:{:?}",request.uri().path());
    if !request.uri().path().ends_with(r"/companies(/(\d+))?$") {

        if let Some(Path((company_id,))) = request.extensions().get::<Path<(i32,)>>() {
            let query_raw = format!("SET app.current_tenant = '{}';", company_id);
            let query = Statement::from_string(sea_orm::DatabaseBackend::Postgres, query_raw);

            if let Err(e) = db.execute(query).await {
                KnownWebError::bad_request(format!("cannot set app.current_tenant due to: {:?}", e));
            }
        }
    }

    next.run(request).await
}
