use std::io::Error;
use actix_settings::{ApplySettings, BasicSettings, Mode, Settings};
use actix_web::middleware::{Compress, Condition, Logger};
use actix_web::{web, App, HttpServer};
use actix_files::Files as Fs;
use sea_orm::{Database, DatabaseConnection};
use serde::Deserialize;
use settings::CustomSettings;
mod models;
mod controllers;
mod settings;

#[derive(Debug, Clone)]
pub struct AppState {
    db: DatabaseConnection,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut settings = CustomSettings::parse_toml("./config.toml").expect("Failed to parse `Settings` from config.toml");
    init_logger(&settings);
    Settings::override_field_with_env_var(&mut settings.application.sea_orm.uri, "SEA_ORM_URI")?;
    Settings::override_field_with_env_var(&mut settings.application.migration.enabled, "MIGRATION_ENABLED")?;
    Settings::override_field_with_env_var(&mut settings.application.cache.enabled, "CACHE_ENABLED")?;
    Settings::override_field_with_env_var(&mut settings.application.cache.url, "CACHE_URL")?;

    if(settings.application.cache.enabled){
        let redis_url = settings.application.cache.url.clone();
        let cfg = deadpool_redis::Config::from_url(redis_url.clone());
        let redis_pool = cfg.create_pool(Some(deadpool_redis::Runtime::Tokio1))
            .expect(format!("Cannot create deadpool redis from url:{}",redis_url).as_str());
        let redis_pool_data = actix_web::web::Data::new(redis_pool);

        let secret_key = actix_web::cookie::Key::from(settings.secret.hmac_secret.as_bytes());
        let redis_store = actix_session::storage::RedisSessionStore::new(redis_url.clone())
            .await.expect("Cannot unwrap redis session.");
    }

    let db = Database::connect(&settings.application.sea_orm.uri).await.expect("Failed to initialize database");
    let state = AppState { db:db.clone() };

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
                .wrap(Logger::default())
                .configure(init)
        }
    })
        .try_apply_settings(&settings)?
        .run()
        .await
}

fn init(cfg: &mut web::ServiceConfig) {
}


fn init_logger(settings: &CustomSettings) {
    if !settings.actix.enable_log {
        return;
    }
    std::env::set_var(
        "RUST_LOG",
        match settings.actix.mode {
            Mode::Development => "actix_web=debug",
            Mode::Production => "actix_web=info",
        },
    );
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
}

async fn init_db(settings: &CustomSettings) -> Result<DatabaseConnection, sea_orm::DbErr> {
    let conn = Database::connect(&settings.application.sea_orm.uri).await?;
    Ok(conn)
}