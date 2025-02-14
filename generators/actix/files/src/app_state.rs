use deadpool_redis::Pool;
use sea_orm::DatabaseConnection;

#[derive(Debug, Clone)]
pub struct AppState {
    pub(crate) db: DatabaseConnection,
    pub(crate) redis_pool: Option<Pool>,
}