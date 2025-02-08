use actix_settings::{ApplySettings, BasicSettings, Mode};
use actix_web::middleware::{Compress, Condition, Logger};
use actix_web::{web, App, HttpServer};
use sea_orm::{Database, DatabaseConnection};
use serde::Deserialize;

mod models;
mod controllers;

#[derive(Debug, Clone)]
pub struct AppState {
    conn: DatabaseConnection,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
struct SeaOrmSettings {
    uri: String,
    min_connections: u32,
    max_connections: u32,
    acquire_timeout: u64,
    idle_timeout: u64,
    connect_timeout: u64,
    enable_logging: bool,
}

impl Default for SeaOrmSettings {
    fn default() -> Self {
        Self {
            uri: "postgres://root:123456@localhost:5432/pg_db".to_string(),
            min_connections: 1,
            max_connections: 10,
            acquire_timeout: 30_000,
            idle_timeout: 600_000,
            connect_timeout: 1_800_000,
            enable_logging: true,
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
struct AppSettings {
    #[serde(rename = "nested-field")]
    sea_orm_config: SeaOrmSettings,
}

type CustomSettings = BasicSettings<AppSettings>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut settings = CustomSettings::parse_toml("./config.toml").expect("Failed to parse `Settings` from config.toml");
    init_logger(&settings);
    let conn = init_db(&settings).await.expect("Failed to initialize database");
    let state = AppState { conn };

    println!("🚀 Server started successfully");
    HttpServer::new({
        let settings = settings.clone();

        move || {
            App::new()
                .wrap(Condition::new(
                    settings.actix.enable_compression,
                    Compress::default(),
                ))
                .wrap(Logger::default())
                .app_data(web::Data::new(state.clone()))
        }
    })
        .try_apply_settings(&settings)?
        .run()
        .await
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
    let conn = Database::connect(&settings.application.sea_orm_config.uri).await?;
    Ok(conn)
}