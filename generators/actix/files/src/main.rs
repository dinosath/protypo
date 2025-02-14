use actix_files::Files as Fs;
use actix_settings::{ApplySettings, BasicSettings, Mode, Settings};
use actix_web::middleware::{Compress, Condition, Logger};
use actix_web::{web, App, HttpServer};
use deadpool_redis::{Runtime};
use opentelemetry::trace::TracerProvider;
use opentelemetry::{global, KeyValue};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{
    propagation::TraceContextPropagator, runtime::TokioCurrentThread, trace::Config, Resource,
};
use opentelemetry_semantic_conventions::resource;
use sea_orm::{Database, DatabaseConnection};
use serde::Deserialize;
use settings::CustomSettings;
use std::io::Error;
use std::sync::LazyLock;
use tracing_actix_web::TracingLogger;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use crate::app_state::AppState;

mod controllers;
mod models;
mod settings;
mod app_state;

const APP_NAME: &str = "app_name";
static RESOURCE: LazyLock<Resource> = LazyLock::new(|| Resource::new(vec![KeyValue::new(resource::SERVICE_NAME, APP_NAME)]));

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut settings = CustomSettings::parse_toml("./config.toml")
        .expect("Failed to parse `Settings` from config.toml");
    init_telemetry(&settings);
    Settings::override_field_with_env_var(&mut settings.application.sea_orm.uri, "SEA_ORM_URI")?;
    Settings::override_field_with_env_var(
        &mut settings.application.migration.enabled,
        "MIGRATION_ENABLED",
    )?;
    Settings::override_field_with_env_var(
        &mut settings.application.cache.enabled,
        "CACHE_ENABLED",
    )?;
    Settings::override_field_with_env_var(&mut settings.application.cache.url, "CACHE_URL")?;

    let redis_pool = if settings.application.cache.enabled {
        let redis_url = settings.application.cache.url.clone();
        let redis_cfg = deadpool_redis::Config::from_url(redis_url.clone());
        Some(
            redis_cfg
                .create_pool(Some(Runtime::Tokio1))
                .expect(format!("Cannot create deadpool redis from url:{}", redis_url).as_str()),
        )
    } else {
        None
    };

    let db = Database::connect(&settings.application.sea_orm.uri)
        .await
        .expect("Failed to initialize database");
    let state = AppState { db:db.clone(),redis_pool };

    println!("🚀 Server started successfully");
    HttpServer::new({
        let settings = settings.clone();

        move || {
            App::new()
                .wrap(Condition::new(
                    settings.actix.enable_compression,
                    Compress::default(),
                ))
                .service(Fs::new("/static", "./static"))
                .app_data(web::Data::new(state.clone()))
                .wrap(TracingLogger::default())
                .configure(init)
        }
    })
    .try_apply_settings(&settings)?
    .run()
    .await
}

fn init(cfg: &mut web::ServiceConfig) {

}

async fn init_db(settings: &CustomSettings) -> Result<DatabaseConnection, sea_orm::DbErr> {
    let conn = Database::connect(&settings.application.sea_orm.uri).await?;
    Ok(conn)
}

fn init_telemetry(settings: &CustomSettings) {
    if settings.application.otel.enabled {
        global::set_text_map_propagator(TraceContextPropagator::new());
        let otlp_exporter = opentelemetry_otlp::SpanExporter::builder()
            .with_tonic()
            .with_endpoint(settings.clone().application.otel.url)
            .build()
            .expect("Failed to build the span exporter");
        let provider = opentelemetry_sdk::trace::TracerProvider::builder()
            .with_batch_exporter(otlp_exporter, TokioCurrentThread)
            .with_config(Config::default().with_resource(RESOURCE.clone()))
            .build();
        let tracer = provider.tracer(APP_NAME);

        let env_filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("info"));
        let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
        let formatting_layer = BunyanFormattingLayer::new(APP_NAME.into(), std::io::stdout);
        let subscriber = Registry::default()
            .with(env_filter)
            .with(telemetry)
            .with(JsonStorageLayer)
            .with(formatting_layer);
        tracing::subscriber::set_global_default(subscriber)
            .expect("Failed to install `tracing` subscriber.")
    }
}