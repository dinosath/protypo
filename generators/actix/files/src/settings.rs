use actix_settings::BasicSettings;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub(crate) struct SeaOrmSettings {
    pub(crate) uri: String,
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
pub(crate) struct CacheSettings {
    pub(crate) enabled: bool,
    pub(crate) url: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub(crate) struct OtelSettings {
    pub(crate) enabled: bool,
    pub(crate) url: String,
}


#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub(crate) struct MigrationSettings {
    pub(crate) enabled: bool,
}


#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub(crate) struct AppSettings {
    #[serde(rename = "sea-orm")]
    pub(crate) sea_orm: SeaOrmSettings,
    pub(crate) migration: MigrationSettings,
    pub(crate) cache: CacheSettings,
    pub(crate) otel: OtelSettings,
}

pub(crate) type CustomSettings = BasicSettings<AppSettings>;